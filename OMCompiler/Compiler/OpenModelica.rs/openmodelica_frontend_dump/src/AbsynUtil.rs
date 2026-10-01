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

use crate::Dump;
use openmodelica_ast::Absyn;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn traverseExp<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outArg: Arg;
    (outExp, outArg) = traverseExpBidir(
        inExp,
        std::sync::Arc::new(fnptr!(dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)),
        inFunc.clone(),
        inArg,
    )?;
    Ok((outExp, outArg))
}

pub fn traverseExpTopDown<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outArg: Arg;
    (outExp, outArg) = traverseExpBidir(
        inExp,
        inFunc.clone(),
        std::sync::Arc::new(fnptr!(dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)),
        inArg,
    )?;
    Ok((outExp, outArg))
}

pub(crate) fn traverseExpList<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExpList: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outExpList: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut outArg: Arg;
    (outExpList, outArg) = traverseExpListBidir(
        inExpList,
        std::sync::Arc::new(fnptr!(dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)),
        inFunc.clone(),
        inArg,
    )?;
    Ok((outExpList, outArg))
}

pub(crate) fn traverseExpListBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outExpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut outArg: Arg;
    (outExpl, outArg) =
        List::map2FoldCheckReferenceEq(inExpl, &traverseExpBidir, enterFunc.clone(), exitFunc.clone(), inArg)?;
    Ok((outExpl, outArg))
}

pub fn traverseExpBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut arg: Arg;
    (e, arg) = enterFunc(inExp, inArg)?;
    (e, arg) = traverseExpBidirSubExps(e, enterFunc.clone(), exitFunc.clone(), arg)?;
    (e, arg) = exitFunc(e, arg)?;
    Ok((e, arg))
}

pub(crate) fn traverseExpOptBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(Option<metamodelica::Ref<Absyn::Exp>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut arg: Arg;
    (outExp, arg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(e1) => {
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            (e2, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e2))) {inExp} else {Some(e2)}, arg)
        },
        _ => {
            (inExp, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

fn traverseExpBidirSubExps<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut arg: Arg = arg;
    (exp, arg) = (match &*exp {
        Absyn::Exp::INTEGER { .. } => (exp, arg),
        Absyn::Exp::REAL { .. } => (exp, arg),
        Absyn::Exp::STRING { .. } => (exp, arg),
        Absyn::Exp::BOOL { .. } => (exp, arg),
        Absyn::Exp::CREF { componentRef: cref } => {
            let mut crefm: metamodelica::Ref<Absyn::ComponentRef>;
            (crefm, arg) = traverseExpBidirCref(cref.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(cref.clone()), &*(&*crefm))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: crefm })
                },
                arg,
            )
        }
        Absyn::Exp::BINARY {
            exp1: e1,
            exp2: e2,
            op: __exp_op,
        } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            let mut e2m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m)) && referenceEq(&*(e2.clone()), &*(&*e2m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::BINARY {
                        exp1: e1m,
                        op: __exp_op.clone(),
                        exp2: e2m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::UNARY { exp: e1, op: __exp_op } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::UNARY {
                        op: __exp_op.clone(),
                        exp: e1m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::LBINARY {
            exp1: e1,
            exp2: e2,
            op: __exp_op,
        } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            let mut e2m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m)) && referenceEq(&*(e2.clone()), &*(&*e2m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::LBINARY {
                        exp1: e1m,
                        op: __exp_op.clone(),
                        exp2: e2m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::LUNARY { exp: e1, op: __exp_op } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::LUNARY {
                        op: __exp_op.clone(),
                        exp: e1m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::RELATION {
            exp1: e1,
            exp2: e2,
            op: __exp_op,
        } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            let mut e2m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m)) && referenceEq(&*(e2.clone()), &*(&*e2m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::RELATION {
                        exp1: e1m,
                        op: __exp_op.clone(),
                        exp2: e2m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::IFEXP {
            ifExp: e1,
            trueBranch: e2,
            elseBranch: e3,
            elseIfBranch: else_ifs1,
        } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            let mut e2m: metamodelica::Ref<Absyn::Exp>;
            let mut e3m: metamodelica::Ref<Absyn::Exp>;
            let mut else_ifs2: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e3m, arg) = traverseExpBidir(e3.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (else_ifs2, arg) = List::map2FoldCheckReferenceEq(
                else_ifs1.clone(),
                &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>),
                       __a1: _,
                       __a2: _,
                       __a3: _| traverseExpBidirElseIf(&__a0, __a1, __a2, __a3),
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m))
                    && referenceEq(&*(e2.clone()), &*(&*e2m))
                    && referenceEq(&*(e3.clone()), &*(&*e3m))
                    && metamodelica::ReferenceEq::reference_eq(&(else_ifs1.clone()), &(else_ifs2)))
                {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::IFEXP {
                        ifExp: e1m,
                        trueBranch: e2m,
                        elseBranch: e3m,
                        elseIfBranch: else_ifs2,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::CALL {
            function_: cref,
            functionArgs: fargs1,
            typeVars: __exp_typeVars,
        } => {
            let mut fargs2: metamodelica::Ref<Absyn::FunctionArgs>;
            (fargs2, arg) = traverseExpBidirFunctionArgs(fargs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(fargs1.clone()), &*(&*fargs2))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::CALL {
                        function_: cref.clone(),
                        functionArgs: fargs2,
                        typeVars: __exp_typeVars.clone(),
                    })
                },
                arg,
            )
        }
        Absyn::Exp::PARTEVALFUNCTION {
            function_: cref,
            functionArgs: fargs1,
        } => {
            let mut fargs2: metamodelica::Ref<Absyn::FunctionArgs>;
            (fargs2, arg) = traverseExpBidirFunctionArgs(fargs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(fargs1.clone()), &*(&*fargs2))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::PARTEVALFUNCTION {
                        function_: cref.clone(),
                        functionArgs: fargs2,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::ARRAY { arrayExp: expl1 } => {
            let mut expl2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(expl1.clone()), &(expl2))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: expl2 })
                },
                arg,
            )
        }
        Absyn::Exp::MATRIX { matrix: mat_expl } => {
            let mut mat_expl = (*mat_expl).clone();
            (mat_expl, arg) = List::map2FoldCheckReferenceEq(
                mat_expl.clone(),
                &traverseExpListBidir,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                metamodelica::Ref::new(Absyn::Exp::MATRIX {
                    matrix: mat_expl.clone(),
                }),
                arg,
            )
        }
        Absyn::Exp::RANGE {
            start: e1,
            step: oe1,
            stop: e2,
        } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            let mut e2m: metamodelica::Ref<Absyn::Exp>;
            let mut oe1m: Option<metamodelica::Ref<Absyn::Exp>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (oe1m, arg) = traverseExpOptBidir(oe1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m))
                    && referenceEq(&*(e2.clone()), &*(&*e2m))
                    && (match (&(oe1), &(oe1m)) {
                        (None, None) => true,
                        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                        _ => false,
                    }))
                {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::RANGE {
                        start: e1m,
                        step: oe1m,
                        stop: e2m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::END { .. } => (exp, arg),
        Absyn::Exp::TUPLE { expressions: expl1 } => {
            let mut expl2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(expl1.clone()), &(expl2))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: expl2 })
                },
                arg,
            )
        }
        Absyn::Exp::AS { id, exp: e1 } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::AS {
                        id: id.clone(),
                        exp: e1m,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::CONS { head: e1, rest: e2 } => {
            let mut e1m: metamodelica::Ref<Absyn::Exp>;
            let mut e2m: metamodelica::Ref<Absyn::Exp>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e1m)) && referenceEq(&*(e2.clone()), &*(&*e2m))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::CONS { head: e1m, rest: e2m })
                },
                arg,
            )
        }
        Absyn::Exp::MATCHEXP {
            inputExp: e1,
            cases: match_cases,
            comment: __exp_comment,
            localDecls: __exp_localDecls,
            matchTy: __exp_matchTy,
        } => {
            let mut e1 = (*e1).clone();
            let mut match_cases = (*match_cases).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (match_cases, arg) = List::map2FoldCheckReferenceEq(
                match_cases.clone(),
                &traverseMatchCase,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                metamodelica::Ref::new(Absyn::Exp::MATCHEXP {
                    matchTy: __exp_matchTy.clone(),
                    inputExp: e1.clone(),
                    localDecls: __exp_localDecls.clone(),
                    cases: match_cases.clone(),
                    comment: __exp_comment.clone(),
                }),
                arg,
            )
        }
        Absyn::Exp::LIST { exps: expl1 } => {
            let mut expl2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(expl1.clone()), &(expl2))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::LIST { exps: expl2 })
                },
                arg,
            )
        }
        Absyn::Exp::CODE { .. } => (exp, arg),
        Absyn::Exp::DOT {
            exp: __exp_exp,
            index: __exp_index,
        } => {
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            (e1, arg) = traverseExpBidir(__exp_exp.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2, arg) = traverseExpBidir(__exp_index.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(__exp_exp.clone()), &*(&*e1)) && referenceEq(&*(__exp_index.clone()), &*(&*e2))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::DOT { exp: e1, index: e2 })
                },
                arg,
            )
        }
        Absyn::Exp::EXPRESSIONCOMMENT {
            commentsAfter: __exp_commentsAfter,
            commentsBefore: __exp_commentsBefore,
            exp: __exp_exp,
        } => {
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            (e1, arg) = traverseExpBidir(__exp_exp.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))) {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::EXPRESSIONCOMMENT {
                        commentsBefore: __exp_commentsBefore.clone(),
                        exp: e1,
                        commentsAfter: __exp_commentsAfter.clone(),
                    })
                },
                arg,
            )
        }
        Absyn::Exp::SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
        } => {
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            (e1, arg) = traverseExpBidir(__exp_exp.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (subs, arg) = traverseExpBidirSubs(__exp_subscripts.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))
                    && metamodelica::ReferenceEq::reference_eq(&(__exp_subscripts.clone()), &(subs)))
                {
                    exp
                } else {
                    metamodelica::Ref::new(Absyn::Exp::SUBSCRIPTED_EXP {
                        exp: e1,
                        subscripts: subs,
                    })
                },
                arg,
            )
        }
        Absyn::Exp::BREAK { .. } => (exp, arg),
        _ => {
            let mut error_msg: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            (_, _, enterName) = System::dladdr(&enterFunc.clone());
            (_, _, exitName) = System::dladdr(&exitFunc.clone());
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("in traverseExpBidirSubExps("));
                __mm_s.push_str(&*enterName);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*exitName);
                __mm_s.push_str(&*literal!(") - Unknown expression: "));
                ArcStr::from(__mm_s)
            };
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*error_msg);
                __mm_s.push_str(&*Dump::printExpStr(exp)?);
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    });
    Ok((exp, arg))
}

pub(crate) fn traverseExpBidirCref<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::ComponentRef>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut arg: Arg = arg;
    (cref, arg) = (match &*cref {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr1 } => {
            let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
            (cr2, arg) = traverseExpBidirCref(cr1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(cr1.clone()), &*(&*cr2))) {
                    cref
                } else {
                    crefMakeFullyQualified(cr2)
                },
                arg,
            )
        }
        Absyn::ComponentRef::CREF_QUAL {
            name,
            subscripts: subs1,
            componentRef: cr1,
        } => {
            let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
            let mut subs2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            (subs2, arg) = traverseExpBidirSubs(subs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (cr2, arg) = traverseExpBidirCref(cr1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(cr1.clone()), &*(&*cr2))
                    && metamodelica::ReferenceEq::reference_eq(&(subs1.clone()), &(subs2)))
                {
                    cref
                } else {
                    metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                        name: name.clone(),
                        subscripts: subs2,
                        componentRef: cr2,
                    })
                },
                arg,
            )
        }
        Absyn::ComponentRef::CREF_IDENT {
            name,
            subscripts: subs1,
        } => {
            let mut subs2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            (subs2, arg) = traverseExpBidirSubs(subs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(subs1.clone()), &(subs2))) {
                    cref
                } else {
                    metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                        name: name.clone(),
                        subscripts: subs2,
                    })
                },
                arg,
            )
        }
        Absyn::ComponentRef::ALLWILD { .. } => (cref, arg),
        Absyn::ComponentRef::WILD { .. } => (cref, arg),
    });
    Ok((cref, arg))
}

pub(crate) fn traverseExpBidirSubs<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut subscripts: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::Subscript>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut subscripts: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = subscripts;
    let mut arg: Arg = arg;
    (subscripts, arg) = List::map2FoldCheckReferenceEq(
        subscripts,
        &traverseExpBidirSub,
        enterFunc.clone(),
        exitFunc.clone(),
        arg,
    )?;
    Ok((subscripts, arg))
}

pub(crate) fn traverseExpBidirSub<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut subscript: metamodelica::Ref<Absyn::Subscript>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Subscript>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut subscript: metamodelica::Ref<Absyn::Subscript> = subscript;
    let mut arg: Arg = arg;
    (subscript, arg) = (match &*subscript {
        Absyn::Subscript::SUBSCRIPT { subscript: e1 } => {
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            (e2, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                    subscript
                } else {
                    metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: e2 })
                },
                arg,
            )
        }
        Absyn::Subscript::NOSUB { .. } => (subscript, arg),
    });
    Ok((subscript, arg))
}

pub(crate) fn traverseExpBidirElseIf<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElseIf: &(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>),
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<((metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>), Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outElseIf: (metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>);
    let mut arg: Arg;
    let mut e1: metamodelica::Ref<Absyn::Exp>;
    let mut e2: metamodelica::Ref<Absyn::Exp>;
    (e1, e2) = inElseIf.clone();
    (e1, arg) = traverseExpBidir(e1, enterFunc.clone(), exitFunc.clone(), inArg)?;
    (e2, arg) = traverseExpBidir(e2, enterFunc.clone(), exitFunc.clone(), arg)?;
    outElseIf = (e1, e2);
    Ok((outElseIf, arg))
}

pub fn traverseExpBidirFunctionArgs<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut args: metamodelica::Ref<Absyn::FunctionArgs>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::FunctionArgs>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut args: metamodelica::Ref<Absyn::FunctionArgs> = args;
    let mut arg: Arg = arg;
    (args, arg) = (match &*args {
        Absyn::FunctionArgs::FUNCTIONARGS {
            args: expl1,
            argNames: named_args1,
        } => {
            let mut expl2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut named_args2: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (named_args2, arg) = List::map2FoldCheckReferenceEq(
                named_args1.clone(),
                &traverseExpBidirNamedArg,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(expl1.clone()), &(expl2))
                    && metamodelica::ReferenceEq::reference_eq(&(named_args1.clone()), &(named_args2)))
                {
                    args
                } else {
                    metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                        args: expl2,
                        argNames: named_args2,
                    })
                },
                arg,
            )
        }
        Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: e1,
            iterType,
            iterators: iters1,
        } => {
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            let mut iters2: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
            (e2, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (iters2, arg) = List::map2FoldCheckReferenceEq(
                iters1.clone(),
                &traverseExpBidirIterator,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                if (referenceEq(&*(e1.clone()), &*(&*e2))
                    && metamodelica::ReferenceEq::reference_eq(&(iters1.clone()), &(iters2)))
                {
                    args
                } else {
                    metamodelica::Ref::new(Absyn::FunctionArgs::FOR_ITER_FARG {
                        exp: e2,
                        iterType: iterType.clone(),
                        iterators: iters2,
                    })
                },
                arg,
            )
        }
    });
    Ok((args, arg))
}

pub(crate) fn traverseExpBidirNamedArg<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArg: metamodelica::Ref<Absyn::NamedArg>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inExtra: Arg,
) -> Result<(metamodelica::Ref<Absyn::NamedArg>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outArg: metamodelica::Ref<Absyn::NamedArg>;
    let mut outExtra: Arg;
    let mut name: ArcStr;
    let mut value1: metamodelica::Ref<Absyn::Exp>;
    let mut value2: metamodelica::Ref<Absyn::Exp>;
    let __arc2 = inArg.clone();
    let Absyn::NAMEDARG {
        argName: __pa0,
        argValue: __pa1,
    } = &*__arc2;
    name = metamodelica::Own::own(__pa0);
    value1 = metamodelica::Own::own(__pa1);
    (value2, outExtra) = traverseExpBidir(value1.clone(), enterFunc.clone(), exitFunc.clone(), inExtra)?;
    outArg = if (referenceEq(&*(value1), &*(&*value2))) {
        inArg
    } else {
        metamodelica::Ref::new(Absyn::NamedArg {
            argName: name,
            argValue: value2,
        })
    };
    Ok((outArg, outExtra))
}

pub(crate) fn traverseExpBidirIterator<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIterator: metamodelica::Ref<Absyn::ForIterator>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::Ref<Absyn::ForIterator>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outIterator: metamodelica::Ref<Absyn::ForIterator>;
    let mut outArg: Arg;
    let mut name: ArcStr;
    let mut guardExp1: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut guardExp2: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut range1: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut range2: Option<metamodelica::Ref<Absyn::Exp>>;
    let __arc3 = inIterator.clone();
    let Absyn::ITERATOR {
        name: __pa0,
        guardExp: __pa1,
        range: __pa2,
    } = &*__arc3;
    name = metamodelica::Own::own(__pa0);
    guardExp1 = metamodelica::Own::own(__pa1);
    range1 = metamodelica::Own::own(__pa2);
    (guardExp2, outArg) = traverseExpOptBidir(guardExp1.clone(), enterFunc.clone(), exitFunc.clone(), inArg)?;
    (range2, outArg) = traverseExpOptBidir(range1.clone(), enterFunc.clone(), exitFunc.clone(), outArg)?;
    outIterator = if ((match (&(guardExp1), &(guardExp2)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    }) && (match (&(range1), &(range2)) {
        (None, None) => true,
        (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
        _ => false,
    })) {
        inIterator
    } else {
        metamodelica::Ref::new(Absyn::ForIterator {
            name: name,
            guardExp: guardExp2,
            range: range2,
        })
    };
    Ok((outIterator, outArg))
}

pub(crate) fn traverseMatchCase<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut matchCase: metamodelica::Ref<Absyn::Case>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Case>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut matchCase: metamodelica::Ref<Absyn::Case> = matchCase;
    let mut arg: Arg = arg;
    (matchCase, arg) = (match &*matchCase {
        Absyn::Case::CASE {
            pattern,
            patternGuard,
            patternInfo: pinfo,
            localDecls: ldecls,
            classPart: cp,
            result,
            resultInfo,
            comment: cmt,
            info,
        } => {
            let mut pattern = (*pattern).clone();
            let mut patternGuard = (*patternGuard).clone();
            let mut cp = (*cp).clone();
            let mut result = (*result).clone();
            (pattern, arg) = traverseExpBidir(pattern.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (patternGuard, arg) = traverseExpOptBidir(patternGuard.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (cp, arg) = traverseClassPartBidir(cp.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (result, arg) = traverseExpBidir(result.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                metamodelica::Ref::new(Absyn::Case::CASE {
                    pattern: pattern.clone(),
                    patternGuard: patternGuard.clone(),
                    patternInfo: pinfo.clone(),
                    localDecls: ldecls.clone(),
                    classPart: cp.clone(),
                    result: result.clone(),
                    resultInfo: resultInfo.clone(),
                    comment: cmt.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        Absyn::Case::ELSE {
            localDecls: ldecls,
            classPart: cp,
            result,
            resultInfo,
            comment: cmt,
            info,
        } => {
            let mut cp = (*cp).clone();
            let mut result = (*result).clone();
            (cp, arg) = traverseClassPartBidir(cp.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (result, arg) = traverseExpBidir(result.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (
                metamodelica::Ref::new(Absyn::Case::ELSE {
                    localDecls: ldecls.clone(),
                    classPart: cp.clone(),
                    result: result.clone(),
                    resultInfo: resultInfo.clone(),
                    comment: cmt.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
    });
    Ok((matchCase, arg))
}

fn traverseClassPartBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cp: metamodelica::Ref<Absyn::ClassPart>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::ClassPart>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut cp: metamodelica::Ref<Absyn::ClassPart> = cp;
    let mut arg: Arg = arg;
    (cp, arg) = (match &*cp {
        Absyn::ClassPart::ALGORITHMS { contents: algs } => {
            let mut algs = (*algs).clone();
            (algs, arg) = List::map2FoldCheckReferenceEq(
                algs.clone(),
                &traverseAlgorithmItemBidir,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                metamodelica::Ref::new(Absyn::ClassPart::ALGORITHMS { contents: algs.clone() }),
                arg,
            )
        }
        Absyn::ClassPart::EQUATIONS { contents: eqs } => {
            let mut eqs = (*eqs).clone();
            (eqs, arg) = List::map2FoldCheckReferenceEq(
                eqs.clone(),
                &traverseEquationItemBidir,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (
                metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: eqs.clone() }),
                arg,
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((cp, arg))
}

pub fn traverseEquationItemListBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquationItems: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outEquationItems: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut outArg: Arg;
    (outEquationItems, outArg) = List::map2FoldCheckReferenceEq(
        inEquationItems,
        &traverseEquationItemBidir,
        enterFunc.clone(),
        exitFunc.clone(),
        inArg,
    )?;
    Ok((outEquationItems, outArg))
}

pub fn traverseAlgorithmItemListBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inAlgs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outAlgs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    let mut outArg: Arg;
    (outAlgs, outArg) = List::map2FoldCheckReferenceEq(
        inAlgs,
        &traverseAlgorithmItemBidir,
        enterFunc.clone(),
        exitFunc.clone(),
        inArg,
    )?;
    Ok((outAlgs, outArg))
}

fn traverseAlgorithmItemBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut algorithmItem: metamodelica::Ref<Absyn::AlgorithmItem>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::AlgorithmItem>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut algorithmItem: metamodelica::Ref<Absyn::AlgorithmItem> = algorithmItem;
    let mut arg: Arg = arg;
    let () = (match &*algorithmItem {
        Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, .. } => {
            let mut alg = (*alg).clone();
            (alg, arg) = traverseAlgorithmBidir(alg.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            assign_variant_field!(algorithmItem => Absyn::AlgorithmItem::ALGORITHMITEM; algorithm_ = alg.clone());
            ()
        }
        Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { .. } => (),
    });
    Ok((algorithmItem, arg))
}

fn traverseEquationItemBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut equationItem: metamodelica::Ref<Absyn::EquationItem>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::EquationItem>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut equationItem: metamodelica::Ref<Absyn::EquationItem> = equationItem;
    let mut arg: Arg = arg;
    let () = (match &*equationItem {
        Absyn::EquationItem::EQUATIONITEM { equation_: eq, .. } => {
            let mut eq = (*eq).clone();
            (eq, arg) = traverseEquationBidir(eq.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            assign_variant_field!(equationItem => Absyn::EquationItem::EQUATIONITEM; equation_ = eq.clone());
            ()
        }
        Absyn::EquationItem::EQUATIONITEMCOMMENT { .. } => (),
    });
    Ok((equationItem, arg))
}

pub(crate) fn traverseEquationBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eq: metamodelica::Ref<Absyn::Equation>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Equation>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut eq: metamodelica::Ref<Absyn::Equation> = eq;
    let mut arg: Arg = arg;
    eq = (match &*eq {
        Absyn::Equation::EQ_IF {
            ifExp: e1,
            equationTrueItems: eqil1,
            elseIfBranches: else_branch,
            equationElseItems: eqil2,
        } => {
            let mut e1 = (*e1).clone();
            let mut eqil1 = (*eqil1).clone();
            let mut else_branch = (*else_branch).clone();
            let mut eqil2 = (*eqil2).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (eqil1, arg) = traverseEquationItemListBidir(eqil1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (else_branch, arg) = List::map2FoldCheckReferenceEq(
                else_branch.clone(),
                &move |__a0: (
                    metamodelica::Ref<Absyn::Exp>,
                    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
                ),
                       __a1: _,
                       __a2: _,
                       __a3: _| traverseEquationBidirElse(&__a0, __a1, __a2, __a3),
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (eqil2, arg) = traverseEquationItemListBidir(eqil2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_IF {
                ifExp: e1.clone(),
                equationTrueItems: eqil1.clone(),
                elseIfBranches: else_branch.clone(),
                equationElseItems: eqil2.clone(),
            })
        }
        Absyn::Equation::EQ_EQUALS {
            leftSide: e1,
            rightSide: e2,
        } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_EQUALS {
                leftSide: e1.clone(),
                rightSide: e2.clone(),
            })
        }
        Absyn::Equation::EQ_PDE {
            leftSide: e1,
            rightSide: e2,
            domain: cref1,
        } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut cref1 = (*cref1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (cref1, _) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            metamodelica::Ref::new(Absyn::Equation::EQ_PDE {
                leftSide: e1.clone(),
                rightSide: e2.clone(),
                domain: cref1.clone(),
            })
        }
        Absyn::Equation::EQ_CONNECT {
            connector1: cref1,
            connector2: cref2,
        } => {
            let mut cref1 = (*cref1).clone();
            let mut cref2 = (*cref2).clone();
            (cref1, arg) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (cref2, arg) = traverseExpBidirCref(cref2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_CONNECT {
                connector1: cref1.clone(),
                connector2: cref2.clone(),
            })
        }
        Absyn::Equation::EQ_FOR {
            iterators: iters,
            forEquations: eqil1,
        } => {
            let mut iters = (*iters).clone();
            let mut eqil1 = (*eqil1).clone();
            (iters, arg) = List::map2FoldCheckReferenceEq(
                iters.clone(),
                &traverseExpBidirIterator,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (eqil1, arg) = traverseEquationItemListBidir(eqil1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_FOR {
                iterators: iters.clone(),
                forEquations: eqil1.clone(),
            })
        }
        Absyn::Equation::EQ_WHEN_E {
            whenExp: e1,
            whenEquations: eqil1,
            elseWhenEquations: else_branch,
        } => {
            let mut e1 = (*e1).clone();
            let mut eqil1 = (*eqil1).clone();
            let mut else_branch = (*else_branch).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (eqil1, arg) = traverseEquationItemListBidir(eqil1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (else_branch, arg) = List::map2FoldCheckReferenceEq(
                else_branch.clone(),
                &move |__a0: (
                    metamodelica::Ref<Absyn::Exp>,
                    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
                ),
                       __a1: _,
                       __a2: _,
                       __a3: _| traverseEquationBidirElse(&__a0, __a1, __a2, __a3),
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            metamodelica::Ref::new(Absyn::Equation::EQ_WHEN_E {
                whenExp: e1.clone(),
                whenEquations: eqil1.clone(),
                elseWhenEquations: else_branch.clone(),
            })
        }
        Absyn::Equation::EQ_NORETCALL {
            functionName: cref1,
            functionArgs: func_args,
        } => {
            let mut cref1 = (*cref1).clone();
            let mut func_args = (*func_args).clone();
            (cref1, arg) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (func_args, arg) =
                traverseExpBidirFunctionArgs(func_args.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_NORETCALL {
                functionName: cref1.clone(),
                functionArgs: func_args.clone(),
            })
        }
        Absyn::Equation::EQ_FAILURE { equ: eq1 } => {
            let mut eq1 = (*eq1).clone();
            (eq1, arg) = traverseEquationItemBidir(eq1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Equation::EQ_FAILURE { equ: eq1.clone() })
        }
    });
    Ok((eq, arg))
}

fn traverseEquationBidirElse<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElse: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    ),
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(
    (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    ),
    Arg,
)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outElse: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    );
    let mut arg: Arg;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut eqil: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    (e, eqil) = inElse.clone();
    (e, arg) = traverseExpBidir(e, enterFunc.clone(), exitFunc.clone(), inArg)?;
    (eqil, arg) = traverseEquationItemListBidir(eqil, enterFunc.clone(), exitFunc.clone(), arg)?;
    outElse = (e, eqil);
    Ok((outElse, arg))
}

fn traverseAlgorithmBidirElse<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElse: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    ),
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut inArg: Arg,
) -> Result<(
    (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    ),
    Arg,
)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut outElse: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    );
    let mut arg: Arg;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut algs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    (e, algs) = inElse.clone();
    (e, arg) = traverseExpBidir(e, enterFunc.clone(), exitFunc.clone(), inArg)?;
    (algs, arg) = traverseAlgorithmItemListBidir(algs, enterFunc.clone(), exitFunc.clone(), arg)?;
    outElse = (e, algs);
    Ok((outElse, arg))
}

fn traverseAlgorithmBidir<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut alg: metamodelica::Ref<Absyn::Algorithm>,
    mut enterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut exitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >,
    mut arg: Arg,
) -> Result<(metamodelica::Ref<Absyn::Algorithm>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, Arg) -> Result<(metamodelica::Ref<Absyn::Exp>, Arg)>
            + 'static,
    >;

    let mut alg: metamodelica::Ref<Absyn::Algorithm> = alg;
    let mut arg: Arg = arg;
    alg = (match &*alg {
        Absyn::Algorithm::ALG_ASSIGN {
            assignComponent: e1,
            value: e2,
        } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN {
                assignComponent: e1.clone(),
                value: e2.clone(),
            })
        }
        Absyn::Algorithm::ALG_IF {
            ifExp: e1,
            trueBranch: algs1,
            elseIfAlgorithmBranch: else_branch,
            elseBranch: algs2,
        } => {
            let mut e1 = (*e1).clone();
            let mut algs1 = (*algs1).clone();
            let mut else_branch = (*else_branch).clone();
            let mut algs2 = (*algs2).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (else_branch, arg) = List::map2FoldCheckReferenceEq(
                else_branch.clone(),
                &move |__a0: (
                    metamodelica::Ref<Absyn::Exp>,
                    metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
                ),
                       __a1: _,
                       __a2: _,
                       __a3: _| traverseAlgorithmBidirElse(&__a0, __a1, __a2, __a3),
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (algs2, arg) = traverseAlgorithmItemListBidir(algs2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_IF {
                ifExp: e1.clone(),
                trueBranch: algs1.clone(),
                elseIfAlgorithmBranch: else_branch.clone(),
                elseBranch: algs2.clone(),
            })
        }
        Absyn::Algorithm::ALG_FOR {
            iterators: iters,
            forBody: algs1,
        } => {
            let mut iters = (*iters).clone();
            let mut algs1 = (*algs1).clone();
            (iters, arg) = List::map2FoldCheckReferenceEq(
                iters.clone(),
                &traverseExpBidirIterator,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_FOR {
                iterators: iters.clone(),
                forBody: algs1.clone(),
            })
        }
        Absyn::Algorithm::ALG_PARFOR {
            iterators: iters,
            parforBody: algs1,
        } => {
            let mut iters = (*iters).clone();
            let mut algs1 = (*algs1).clone();
            (iters, arg) = List::map2FoldCheckReferenceEq(
                iters.clone(),
                &traverseExpBidirIterator,
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_PARFOR {
                iterators: iters.clone(),
                parforBody: algs1.clone(),
            })
        }
        Absyn::Algorithm::ALG_WHILE {
            boolExpr: e1,
            whileBody: algs1,
        } => {
            let mut e1 = (*e1).clone();
            let mut algs1 = (*algs1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_WHILE {
                boolExpr: e1.clone(),
                whileBody: algs1.clone(),
            })
        }
        Absyn::Algorithm::ALG_WHEN_A {
            boolExpr: e1,
            whenBody: algs1,
            elseWhenAlgorithmBranch: else_branch,
        } => {
            let mut e1 = (*e1).clone();
            let mut algs1 = (*algs1).clone();
            let mut else_branch = (*else_branch).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (else_branch, arg) = List::map2FoldCheckReferenceEq(
                else_branch.clone(),
                &move |__a0: (
                    metamodelica::Ref<Absyn::Exp>,
                    metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
                ),
                       __a1: _,
                       __a2: _,
                       __a3: _| traverseAlgorithmBidirElse(&__a0, __a1, __a2, __a3),
                enterFunc.clone(),
                exitFunc.clone(),
                arg,
            )?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_WHEN_A {
                boolExpr: e1.clone(),
                whenBody: algs1.clone(),
                elseWhenAlgorithmBranch: else_branch.clone(),
            })
        }
        Absyn::Algorithm::ALG_NORETCALL {
            functionCall: cref1,
            functionArgs: func_args,
        } => {
            let mut cref1 = (*cref1).clone();
            let mut func_args = (*func_args).clone();
            (cref1, arg) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (func_args, arg) =
                traverseExpBidirFunctionArgs(func_args.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL {
                functionCall: cref1.clone(),
                functionArgs: func_args.clone(),
            })
        }
        Absyn::Algorithm::ALG_RETURN { .. } => alg,
        Absyn::Algorithm::ALG_BREAK { .. } => alg,
        Absyn::Algorithm::ALG_CONTINUE { .. } => alg,
        Absyn::Algorithm::ALG_FAILURE { equ: algs1 } => {
            let mut algs1 = (*algs1).clone();
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_FAILURE { equ: algs1.clone() })
        }
        Absyn::Algorithm::ALG_TRY {
            body: algs1,
            elseBody: algs2,
        } => {
            let mut algs1 = (*algs1).clone();
            let mut algs2 = (*algs2).clone();
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            (algs2, arg) = traverseAlgorithmItemListBidir(algs2.clone(), enterFunc.clone(), exitFunc.clone(), arg)?;
            metamodelica::Ref::new(Absyn::Algorithm::ALG_TRY {
                body: algs1.clone(),
                elseBody: algs2.clone(),
            })
        }
    });
    Ok((alg, arg))
}

pub fn makeIdentPathFromString(mut s: ArcStr) -> metamodelica::Ref<Absyn::Path> {
    let mut p: metamodelica::Ref<Absyn::Path>;
    p = metamodelica::Ref::new(Absyn::Path::IDENT { name: s });
    p
}

pub(crate) fn makeQualifiedPathFromStrings(mut s1: ArcStr, mut s2: ArcStr) -> metamodelica::Ref<Absyn::Path> {
    let mut p: metamodelica::Ref<Absyn::Path>;
    p = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
        name: s1,
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: s2 }),
    });
    p
}

pub fn className(mut cl: &metamodelica::Ref<Absyn::Class>) -> ArcStr {
    let mut name: ArcStr;
    let __arc1 = &(*cl);
    let Absyn::CLASS { name: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    name
}

pub fn isClassNamed(mut inName: &ArcStr, mut inClass: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (match &**inClass {
        Absyn::Class { .. } => metamodelica::stringEq(&inName, &inClass.name),
        _ => false,
    });
    outIsNamed
}

pub fn isComponentItemNamed(mut name: &ArcStr, mut component: &metamodelica::Ref<Absyn::ComponentItem>) -> bool {
    let mut res: bool = isComponentNamed(name, &component.component);
    res
}

pub(crate) fn isComponentNamed(mut name: &ArcStr, mut component: &Absyn::Component) -> bool {
    let mut res: bool = metamodelica::stringEq(&name, &component.name);
    res
}

pub fn elementSpecName(mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = (::match_deref::match_deref! { match inElementSpec {
        Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: n, .. }, .. } => {
            n.clone()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: n, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            n.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outIdent)
}

pub fn elementItemNames(mut item: &metamodelica::Ref<Absyn::ElementItem>) -> Result<metamodelica::List<ArcStr>> {
    let mut names: metamodelica::List<ArcStr>;
    names = (match &**item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => elementNames(metamodelica::AsArg::as_arg(&__item_element))?,
        _ => metamodelica::nil(),
    });
    Ok(names)
}

pub(crate) fn elementNames(mut element: &metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::List<ArcStr>> {
    let mut names: metamodelica::List<ArcStr>;
    names = (match &**element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => elementSpecNames(metamodelica::AsArg::as_arg(&__element_specification))?,
        _ => metamodelica::nil(),
    });
    Ok(names)
}

pub(crate) fn elementSpecNames(mut spec: &metamodelica::Ref<Absyn::ElementSpec>) -> Result<metamodelica::List<ArcStr>> {
    let mut names: metamodelica::List<ArcStr>;
    names = (match &**spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => list![className(metamodelica::AsArg::as_arg(&__spec_class_))],
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            ..
        } => {
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut c in (__spec_components.clone()).into_iter().cloned() {
                    let __x = componentName(&(c.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        _ => metamodelica::nil(),
    });
    Ok(names)
}

pub fn isClassdef(mut inElement: &metamodelica::Ref<Absyn::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn printImportString(mut imp: &Absyn::Import) -> Result<ArcStr> {
    let mut ostring: ArcStr;
    ostring = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone(),
        Absyn::Import::QUAL_IMPORT { .. } => pathString(
            var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(),
            literal!("."),
            true,
            false,
        )?,
        Absyn::Import::UNQUAL_IMPORT { .. } => pathString(
            var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(),
            literal!("."),
            true,
            false,
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(ostring)
}

pub fn expString(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ Absyn::Exp::STRING { value: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    Ok(r#str)
}

pub fn expCref(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ Absyn::Exp::CREF { componentRef: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    Ok(cr)
}

pub fn crefExp(mut cr: metamodelica::Ref<Absyn::ComponentRef>) -> metamodelica::Ref<Absyn::Exp> {
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    exp = metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr });
    exp
}

pub fn pathEqual<'__b>(
    mut path1: &'__b metamodelica::Ref<Absyn::Path>,
    mut path2: &'__b metamodelica::Ref<Absyn::Path>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (path1, path2) {
            (Deref @ Absyn::Path::FULLYQUALIFIED { .. }, _) => { (path1, path2) = (var_field!((**path1).path, Absyn::Path::FULLYQUALIFIED), path2); continue '__tco; },
            (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => { (path1, path2) = (path1, var_field!((**path2).path, Absyn::Path::FULLYQUALIFIED)); continue '__tco; },
            (Deref @ Absyn::Path::IDENT { .. }, Deref @ Absyn::Path::IDENT { .. }) => return stringEq(&var_field!((**path1).name, Absyn::Path::IDENT), &var_field!((**path2).name, Absyn::Path::IDENT)),
            (Deref @ Absyn::Path::QUALIFIED { .. }, Deref @ Absyn::Path::QUALIFIED { .. }) => return stringEq(&var_field!((**path1).name, Absyn::Path::QUALIFIED), &var_field!((**path2).name, Absyn::Path::QUALIFIED)) && pathEqual(var_field!((**path1).path, Absyn::Path::QUALIFIED), var_field!((**path2).path, Absyn::Path::QUALIFIED)),
            _ => return false,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn pathEqualCaseInsensitive<'__b>(
    mut path1: &'__b metamodelica::Ref<Absyn::Path>,
    mut path2: &'__b metamodelica::Ref<Absyn::Path>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (path1, path2) {
            (Deref @ Absyn::Path::FULLYQUALIFIED { .. }, _) => { (path1, path2) = (var_field!((**path1).path, Absyn::Path::FULLYQUALIFIED), path2); continue '__tco; },
            (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => { (path1, path2) = (path1, var_field!((**path2).path, Absyn::Path::FULLYQUALIFIED)); continue '__tco; },
            (Deref @ Absyn::Path::IDENT { .. }, Deref @ Absyn::Path::IDENT { .. }) => return stringEq(&(System::tolower(var_field!((**path1).name, Absyn::Path::IDENT).clone())), &(System::tolower(var_field!((**path2).name, Absyn::Path::IDENT).clone()))),
            (Deref @ Absyn::Path::QUALIFIED { .. }, Deref @ Absyn::Path::QUALIFIED { .. }) => return stringEq(&(System::tolower(var_field!((**path1).name, Absyn::Path::QUALIFIED).clone())), &(System::tolower(var_field!((**path2).name, Absyn::Path::QUALIFIED).clone()))) && pathEqualCaseInsensitive(var_field!((**path1).path, Absyn::Path::QUALIFIED), var_field!((**path2).path, Absyn::Path::QUALIFIED)),
            _ => return false,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn typeSpecEqual(
    mut a: &metamodelica::Ref<Absyn::TypeSpec>,
    mut b: &metamodelica::Ref<Absyn::TypeSpec>,
) -> Result<bool> {
    let mut ob: bool;
    ob = (::match_deref::match_deref! { match (a, b) {
        (Deref @ Absyn::TypeSpec::TPATH { .. }, Deref @ Absyn::TypeSpec::TPATH { .. }) => pathEqual(var_field!((**a).path, Absyn::TypeSpec::TPATH), var_field!((**b).path, Absyn::TypeSpec::TPATH)) && optArrayDimEqual(var_field!((**a).arrayDim, Absyn::TypeSpec::TPATH).clone(), var_field!((**b).arrayDim, Absyn::TypeSpec::TPATH).clone())?,
        (Deref @ Absyn::TypeSpec::TCOMPLEX { .. }, Deref @ Absyn::TypeSpec::TCOMPLEX { .. }) => pathEqual(var_field!((**a).path, Absyn::TypeSpec::TCOMPLEX), var_field!((**b).path, Absyn::TypeSpec::TCOMPLEX)) && List::isEqualOnTrue(var_field!((**a).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone(), var_field!((**b).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone(), &move |__a0: metamodelica::Ref<Absyn::TypeSpec>, __a1: metamodelica::Ref<Absyn::TypeSpec>| typeSpecEqual(&__a0, &__a1))? && optArrayDimEqual(var_field!((**a).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), var_field!((**b).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ob)
}

pub(crate) fn optArrayDimEqual(
    mut oad1: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
    mut oad2: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((oad1, oad2)) {
        (Some(ad1), Some(ad2)) => {
            List::isEqualOnTrue(ad1.clone(), ad2.clone(), &move |__a0: metamodelica::Ref<Absyn::Subscript>, __a1: metamodelica::Ref<Absyn::Subscript>| subscriptEqual(&__a0, &__a1))?
        },
        (None, None) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn typeSpecPathString(mut tp: &metamodelica::Ref<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut s: ArcStr = pathString(typeSpecPath(tp), literal!("."), true, false)?;
    Ok(s)
}

pub fn typeSpecPath(mut tp: &metamodelica::Ref<Absyn::TypeSpec>) -> metamodelica::Ref<Absyn::Path> {
    let mut op: metamodelica::Ref<Absyn::Path>;
    op = (match &**tp {
        Absyn::TypeSpec::TCOMPLEX { path: __tp_path, .. } => __tp_path.clone(),
        Absyn::TypeSpec::TPATH { path: __tp_path, .. } => __tp_path.clone(),
    });
    op
}

pub fn typeSpecDimensions(
    mut inTypeSpec: &metamodelica::Ref<Absyn::TypeSpec>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outDimensions: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outDimensions = (::match_deref::match_deref! { match inTypeSpec {
        Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(dim), .. } => {
            dim.clone()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: Some(dim), .. } => {
            dim.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimensions
}

pub fn pathString(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut delimiter: ArcStr,
    mut usefq: bool,
    mut reverse: bool,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut p1: metamodelica::Ref<Absyn::Path>;
    let mut p2: metamodelica::Ref<Absyn::Path>;
    let mut count: i32 = 0;
    let mut len: i32 = 0;
    let mut dlen: i32 = ((delimiter).len() as i32);
    let mut b: bool;
    p1 = if (usefq) { path } else { makeNotFullyQualified(path) };
    let () = (match &*p1 {
        Absyn::Path::IDENT { name: __p1_name } => {
            s = __p1_name.clone();
            return Ok(s);
            ()
        }
        _ => (),
    });
    p2 = p1.clone();
    b = true;
    while b {
        (p2, len, count, b) = (match &*p2.clone() {
            Absyn::Path::IDENT { name: __p2_name } => (p2, len + 1, count + ((__p2_name).len() as i32), false),
            Absyn::Path::QUALIFIED {
                name: __p2_name,
                path: __p2_path,
            } => (__p2_path.clone(), len + 1, count + ((__p2_name).len() as i32), true),
            Absyn::Path::FULLYQUALIFIED { path: __p2_path } => (__p2_path.clone(), len + 1, count, true),
        });
    }
    s = pathStringWork(p1, (len - 1) * dlen + count, delimiter, dlen, reverse)?;
    Ok(s)
}

fn pathStringWork(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut len: i32,
    mut delimiter: ArcStr,
    mut dlen: i32,
    mut reverse: bool,
) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut p: metamodelica::Ref<Absyn::Path> = inPath;
    let mut b: bool = true;
    let mut count: i32 = 0;
    let mut sb: System::StringAllocator = System::StringAllocator(len)?;
    while b {
        (p, count, b) = (match &*p.clone() {
            Absyn::Path::IDENT { name: __p_name } => {
                System::stringAllocatorStringCopy(
                    sb.clone(),
                    __p_name.clone(),
                    if (reverse) {
                        len - count - ((__p_name).len() as i32)
                    } else {
                        count
                    },
                );
                (p, count + ((__p_name).len() as i32), false)
            }
            Absyn::Path::QUALIFIED {
                name: __p_name,
                path: __p_path,
            } => {
                System::stringAllocatorStringCopy(
                    sb.clone(),
                    __p_name.clone(),
                    if (reverse) {
                        len - count - ((__p_name).len() as i32)
                    } else {
                        count
                    },
                );
                System::stringAllocatorStringCopy(
                    sb.clone(),
                    delimiter.clone(),
                    if (reverse) {
                        len - count - ((__p_name).len() as i32) - dlen
                    } else {
                        count + ((__p_name).len() as i32)
                    },
                );
                (__p_path.clone(), count + ((__p_name).len() as i32) + dlen, true)
            }
            Absyn::Path::FULLYQUALIFIED { path: __p_path } => {
                System::stringAllocatorStringCopy(
                    sb.clone(),
                    delimiter.clone(),
                    if (reverse) { len - count - dlen } else { count },
                );
                (__p_path.clone(), count + dlen, true)
            }
        });
    }
    s = System::stringAllocatorResult(sb, s);
    Ok(s)
}

// Function alias `pathStringNoQual = pathString(usefq=false)`; the default-argument
// overrides are applied where calls to the alias omit those arguments.
pub use pathString as pathStringNoQual;

pub fn pathStringDefault(mut path: metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> {
    let mut s: ArcStr = pathString(path.clone(), literal!("."), true, false)?;
    Ok(s)
}

pub(crate) fn classNameCompare(
    mut c1: &metamodelica::Ref<Absyn::Class>,
    mut c2: &metamodelica::Ref<Absyn::Class>,
) -> i32 {
    let mut o: i32;
    o = stringCompare(&c1.name, &c2.name);
    o
}

pub fn classNameGreater(mut c1: &metamodelica::Ref<Absyn::Class>, mut c2: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut b: bool;
    b = stringCompare(&c1.name, &c2.name) > 0;
    b
}

pub fn pathCompare<'__b>(
    mut ip1: &'__b metamodelica::Ref<Absyn::Path>,
    mut ip2: &'__b metamodelica::Ref<Absyn::Path>,
) -> Result<i32> {
    let mut o: i32;
    o = (::match_deref::match_deref! { match (ip1, ip2) {
        (Deref @ Absyn::Path::FULLYQUALIFIED { path: p1 }, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
            pathCompare(p1, p2)?
        },
        (Deref @ Absyn::Path::FULLYQUALIFIED { .. }, _) => {
            1
        },
        (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => {
            -1
        },
        (Deref @ Absyn::Path::QUALIFIED { name: i1, path: p1 }, Deref @ Absyn::Path::QUALIFIED { name: i2, path: p2 }) => {
            o = stringCompare(&i1, &i2);
            o = if (o == 0) {pathCompare(p1, p2)?} else {o};
            o
        },
        (Deref @ Absyn::Path::QUALIFIED { .. }, _) => {
            1
        },
        (_, Deref @ Absyn::Path::QUALIFIED { .. }) => {
            -1
        },
        (Deref @ Absyn::Path::IDENT { name: i1 }, Deref @ Absyn::Path::IDENT { name: i2 }) => {
            stringCompare(&i1, &i2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(o)
}

pub fn pathCompareNoQual(
    mut ip1: metamodelica::Ref<Absyn::Path>,
    mut ip2: metamodelica::Ref<Absyn::Path>,
) -> Result<i32> {
    let mut o: i32;
    o = (::match_deref::match_deref! { match &((ip1, ip2)) {
        (Deref @ Absyn::Path::FULLYQUALIFIED { path: p1 }, p2) => {
            pathCompareNoQual(p1.clone(), p2.clone())?
        },
        (p1, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
            pathCompareNoQual(p1.clone(), p2.clone())?
        },
        (Deref @ Absyn::Path::QUALIFIED { name: i1, path: p1 }, Deref @ Absyn::Path::QUALIFIED { name: i2, path: p2 }) => {
            o = stringCompare(&i1, &i2);
            o = if (o == 0) {pathCompare(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))?} else {o};
            o
        },
        (Deref @ Absyn::Path::QUALIFIED { .. }, _) => {
            1
        },
        (_, Deref @ Absyn::Path::QUALIFIED { .. }) => {
            -1
        },
        (Deref @ Absyn::Path::IDENT { name: i1 }, Deref @ Absyn::Path::IDENT { name: i2 }) => {
            stringCompare(&i1, &i2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(o)
}

pub fn pathHash(mut path: &metamodelica::Ref<Absyn::Path>) -> i32 {
    let mut hash: i32;
    hash = pathHashContinue(path, Util::HASH_SEED.clone());
    hash
}

pub fn pathHashContinue<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>, mut hash: i32) -> i32 {
    '__tco: loop {
        match &**path {
            Absyn::Path::FULLYQUALIFIED { .. } => {
                hash = stringHashDjb2Continue(&(literal!(".")), hash);
                {
                    (path, hash) = (var_field!((**path).path, Absyn::Path::FULLYQUALIFIED), hash);
                    continue '__tco;
                }
            }
            Absyn::Path::QUALIFIED { .. } => {
                hash = stringHashDjb2Continue(&(literal!(".")), hash);
                hash = stringHashDjb2Continue(&var_field!((**path).name, Absyn::Path::QUALIFIED), hash);
                {
                    (path, hash) = (var_field!((**path).path, Absyn::Path::QUALIFIED), hash);
                    continue '__tco;
                }
            }
            Absyn::Path::IDENT { .. } => {
                hash = stringHashDjb2Continue(&(literal!(".")), hash);
                return stringHashDjb2Continue(&var_field!((**path).name, Absyn::Path::IDENT), hash);
            }
        }
    }
}

pub(crate) fn optPathString(mut inPathOption: Option<metamodelica::Ref<Absyn::Path>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inPathOption) {
        None => {
            literal!("")
        },
        Some(p) => {
            pathString(p.clone(), literal!("."), true, false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn pathStringUnquoteReplaceDot(mut inPath: &metamodelica::Ref<Absyn::Path>, mut repStr: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut strlst: metamodelica::List<ArcStr>;
    let mut rep_rep: ArcStr;
    rep_rep = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*repStr);
        __mm_s.push_str(&*repStr);
        ArcStr::from(__mm_s)
    };
    strlst = pathToStringList(inPath);
    strlst = List::map2(strlst, &System::stringReplace, repStr.clone(), rep_rep)?;
    strlst = List::map(strlst, &fnptr!(System::unquoteIdentifier, ArcStr))?;
    outString = stringDelimitList(strlst, repStr);
    Ok(outString)
}

pub fn stringPath(mut r#str: ArcStr) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut qualifiedPath: metamodelica::Ref<Absyn::Path>;
    let mut paths: metamodelica::List<ArcStr>;
    paths = Util::stringSplitAtChar(r#str, literal!("."))?;
    qualifiedPath = stringListPath(paths)?;
    Ok(qualifiedPath)
}

pub fn stringListPath(mut paths: metamodelica::List<ArcStr>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut qualifiedPath: metamodelica::Ref<Absyn::Path> = stringListPathReversed(&(paths.clone().reverse()))?;
    Ok(qualifiedPath)
}

pub fn stringListPathReversed(mut inStrings: &metamodelica::List<ArcStr>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut id: ArcStr;
    let mut rest_str: metamodelica::List<ArcStr>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inStrings)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    id = metamodelica::Own::own(__pa0);
    rest_str = metamodelica::Own::own(__pa1);
    outPath = metamodelica::Ref::new(Absyn::Path::IDENT { name: id });
    for mut s in &*rest_str {
        outPath = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: s.clone(),
            path: outPath,
        });
    }
    Ok(outPath)
}

pub fn pathLastIdent<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>) -> ArcStr {
    '__tco: loop {
        match &**path {
            Absyn::Path::QUALIFIED { .. } => {
                path = var_field!((**path).path, Absyn::Path::QUALIFIED);
                continue '__tco;
            }
            Absyn::Path::IDENT { .. } => return var_field!((**path).name, Absyn::Path::IDENT).clone(),
            Absyn::Path::FULLYQUALIFIED { .. } => {
                path = var_field!((**path).path, Absyn::Path::FULLYQUALIFIED);
                continue '__tco;
            }
        }
    }
}

pub fn pathSetLastIdent(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut ident: &ArcStr,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &**path {
        Absyn::Path::IDENT { .. } => metamodelica::Ref::new(Absyn::Path::IDENT { name: ident.clone() }),
        Absyn::Path::QUALIFIED {
            name: __path_name,
            path: __path_path,
        } => metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: __path_name.clone(),
            path: pathSetLastIdent(metamodelica::AsArg::as_arg(&__path_path), ident),
        }),
        Absyn::Path::FULLYQUALIFIED { path: __path_path } => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
            path: pathSetLastIdent(metamodelica::AsArg::as_arg(&__path_path), ident),
        }),
    });
    outPath
}

pub fn pathLast(mut path: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::Path> {
    '__tco: loop {
        match &*path {
            Absyn::Path::QUALIFIED { path: __path_path, .. } => {
                path = __path_path.clone();
                continue '__tco;
            }
            Absyn::Path::IDENT { .. } => return path,
            Absyn::Path::FULLYQUALIFIED { path: __path_path } => {
                path = __path_path.clone();
                continue '__tco;
            }
        }
    }
}

pub fn pathFirstIdent<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>) -> ArcStr {
    '__tco: loop {
        match &**path {
            Absyn::Path::FULLYQUALIFIED { .. } => {
                path = var_field!((**path).path, Absyn::Path::FULLYQUALIFIED);
                continue '__tco;
            }
            Absyn::Path::QUALIFIED { .. } => return var_field!((**path).name, Absyn::Path::QUALIFIED).clone(),
            Absyn::Path::IDENT { .. } => return var_field!((**path).name, Absyn::Path::IDENT).clone(),
        }
    }
}

pub(crate) fn pathSetFirstIdent(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut ident: &ArcStr,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &**path {
        Absyn::Path::IDENT { .. } => metamodelica::Ref::new(Absyn::Path::IDENT { name: ident.clone() }),
        Absyn::Path::QUALIFIED { path: __path_path, .. } => metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: ident.clone(),
            path: __path_path.clone(),
        }),
        Absyn::Path::FULLYQUALIFIED { path: __path_path } => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
            path: pathSetFirstIdent(metamodelica::AsArg::as_arg(&__path_path), ident),
        }),
    });
    outPath
}

pub fn pathFirstPath<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::Path> {
    '__tco: loop {
        match &**path {
            Absyn::Path::IDENT { .. } => return path.clone(),
            Absyn::Path::QUALIFIED { .. } => {
                return metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: var_field!((**path).name, Absyn::Path::QUALIFIED).clone(),
                });
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                path = var_field!((**path).path, Absyn::Path::FULLYQUALIFIED);
                continue '__tco;
            }
        }
    }
}

pub fn pathSecondIdent<'__b>(mut inPath: &'__b metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inPath {
            Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: n, .. }, .. } => {
                return Ok(n.clone())
            },
            Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { name: n }, .. } => {
                return Ok(n.clone())
            },
            Deref @ Absyn::Path::FULLYQUALIFIED { path: p } => {
                { inPath = p; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn pathNthIdent(mut path: metamodelica::Ref<Absyn::Path>, mut n: i32) -> Result<ArcStr> {
    let mut ident: ArcStr;
    let mut p: metamodelica::Ref<Absyn::Path> = makeNotFullyQualified(path.clone());
    for mut i in 2..=n {
        let __pa0 = ::match_deref::match_deref! { match &(p) {
            Deref @ Absyn::Path::QUALIFIED { path: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        p = metamodelica::Own::own(__pa0);
    }
    ident = pathFirstIdent(&p);
    Ok(ident)
}

pub fn pathSetNthIdent(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut ident: &ArcStr,
    mut n: i32,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    if n == 1 {
        outPath = pathSetFirstIdent(path, ident);
    } else {
        outPath = (match &**path {
            Absyn::Path::QUALIFIED {
                name: __path_name,
                path: __path_path,
            } => metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: __path_name.clone(),
                path: pathSetNthIdent(metamodelica::AsArg::as_arg(&__path_path), ident, n - 1)?,
            }),
            Absyn::Path::FULLYQUALIFIED { path: __path_path } => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
                path: pathSetNthIdent(metamodelica::AsArg::as_arg(&__path_path), ident, n)?,
            }),
            _ => return Err("match: no arm matched"),
        });
    }
    Ok(outPath)
}

pub fn pathRest(mut inPath: metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &*inPath {
        Absyn::Path::QUALIFIED {
            path: __esc_outPath, ..
        } => {
            outPath = (*__esc_outPath).clone();
            outPath.clone()
        }
        Absyn::Path::FULLYQUALIFIED { path: __esc_outPath } => {
            outPath = (*__esc_outPath).clone();
            pathRest(outPath.clone())?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outPath)
}

pub fn pathStripSamePrefix(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
) -> Result<Option<metamodelica::Ref<Absyn::Path>>> {
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut path1: metamodelica::Ref<Absyn::Path> = makeNotFullyQualified(inPath1.clone());
    let mut path2: metamodelica::Ref<Absyn::Path> = makeNotFullyQualified(inPath2.clone());
    while metamodelica::stringEq(&(pathFirstIdent(&path1)), &(pathFirstIdent(&path2))) {
        if pathIsIdent(&path1) {
            outPath = None;
            return Ok(outPath);
        }
        path1 = pathRest(path1)?;
        if pathIsIdent(&path2) {
            break;
        }
        path2 = pathRest(path2)?;
    }
    outPath = Some(path1);
    Ok(outPath)
}

pub fn pathPrefix<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match path {
            Deref @ Absyn::Path::FULLYQUALIFIED { .. } => { path = var_field!((**path).path, Absyn::Path::FULLYQUALIFIED); continue '__tco; },
            Deref @ Absyn::Path::QUALIFIED { path: Deref @ Absyn::Path::IDENT { .. }, .. } => return Ok(metamodelica::Ref::new(Absyn::Path::IDENT { name: var_field!((**path).name, Absyn::Path::QUALIFIED).clone() })),
            Deref @ Absyn::Path::QUALIFIED { .. } => return Ok(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: var_field!((**path).name, Absyn::Path::QUALIFIED).clone(), path: pathPrefix(var_field!((**path).path, Absyn::Path::QUALIFIED))? })),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn prefixPath(mut prefix: ArcStr, mut path: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
        name: prefix,
        path: path,
    });
    outPath
}

pub fn suffixPath(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inSuffix: &ArcStr,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &**inPath {
        Absyn::Path::IDENT { name } => metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: name.clone(),
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: inSuffix.clone() }),
        }),
        Absyn::Path::QUALIFIED { name, path } => {
            let mut path = (*path).clone();
            path = suffixPath(metamodelica::AsArg::as_arg(&path), inSuffix);
            metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: name.clone(),
                path: path.clone(),
            })
        }
        Absyn::Path::FULLYQUALIFIED { path } => {
            let mut path = (*path).clone();
            path = suffixPath(metamodelica::AsArg::as_arg(&path), inSuffix);
            metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: path.clone() })
        }
    });
    outPath
}

pub fn pathSuffixOf<'__b>(
    mut suffix_path: &'__b metamodelica::Ref<Absyn::Path>,
    mut path: &'__b metamodelica::Ref<Absyn::Path>,
) -> bool {
    '__tco: loop {
        match &**path {
            _ if (pathEqual(suffix_path, path)) => return true,
            Absyn::Path::FULLYQUALIFIED { path: p } => {
                (suffix_path, path) = (suffix_path, p);
                continue '__tco;
            }
            Absyn::Path::QUALIFIED { path: p, .. } => {
                (suffix_path, path) = (suffix_path, p);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn pathSuffixOfr(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut suffix_path: &metamodelica::Ref<Absyn::Path>,
) -> bool {
    let mut res: bool;
    res = pathSuffixOf(suffix_path, path);
    res
}

pub fn pathToStringList(mut path: &metamodelica::Ref<Absyn::Path>) -> metamodelica::List<ArcStr> {
    let mut outPaths: metamodelica::List<ArcStr>;
    outPaths = pathToStringListReverse(path, metamodelica::nil()).reverse();
    outPaths
}

pub fn pathToStringListReverse<'__b>(
    mut path: &'__b metamodelica::Ref<Absyn::Path>,
    mut acc: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        match &**path {
            Absyn::Path::IDENT { .. } => {
                return metamodelica::cons(var_field!((**path).name, Absyn::Path::IDENT).clone(), acc);
            }
            Absyn::Path::QUALIFIED { .. } => {
                (path, acc) = (
                    var_field!((**path).path, Absyn::Path::QUALIFIED),
                    metamodelica::cons(var_field!((**path).name, Absyn::Path::QUALIFIED).clone(), acc),
                );
                continue '__tco;
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (path, acc) = (var_field!((**path).path, Absyn::Path::FULLYQUALIFIED), acc);
                continue '__tco;
            }
        }
    }
}

pub(crate) fn addSubscriptsLast(
    mut icr: &metamodelica::Ref<Absyn::ComponentRef>,
    mut i: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut ocr: metamodelica::Ref<Absyn::ComponentRef>;
    ocr = (match &**icr {
        Absyn::ComponentRef::CREF_IDENT {
            name: id,
            subscripts: subs,
        } => metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
            name: id.clone(),
            subscripts: listAppend(subs.clone(), i.clone()),
        }),
        Absyn::ComponentRef::CREF_QUAL {
            name: id,
            subscripts: subs,
            componentRef: cr,
        } => {
            let mut cr = (*cr).clone();
            cr = addSubscriptsLast(metamodelica::AsArg::as_arg(&cr), i)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: id.clone(),
                subscripts: subs.clone(),
                componentRef: cr.clone(),
            })
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = addSubscriptsLast(metamodelica::AsArg::as_arg(&cr), i)?;
            crefMakeFullyQualified(cr.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(ocr)
}

pub fn crefReplaceFirst(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut replacement: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (match &**cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => replacement.clone(),
        Absyn::ComponentRef::CREF_QUAL { .. } => joinCrefs(replacement, crefStripFirst(cref)?)?,
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: crefReplaceFirst(metamodelica::AsArg::as_arg(&__cref_componentRef), replacement)?,
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn crefReplaceFirstIdent(
    mut icref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut replPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (match &**icref {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = crefReplaceFirstIdent(metamodelica::AsArg::as_arg(&cr), replPath)?;
            crefMakeFullyQualified(cr.clone())
        }
        Absyn::ComponentRef::CREF_QUAL {
            componentRef: cr,
            subscripts: subs,
            ..
        } => {
            let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
            cref = pathToCref(replPath);
            cref = addSubscriptsLast(&cref, subs)?;
            joinCrefs(&cref, cr.clone())?
        }
        Absyn::ComponentRef::CREF_IDENT { subscripts: subs, .. } => {
            let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
            cref = pathToCref(replPath);
            cref = addSubscriptsLast(&cref, subs)?;
            cref
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

pub fn pathPrefixOf(mut prefixPath: metamodelica::Ref<Absyn::Path>, mut path: metamodelica::Ref<Absyn::Path>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((prefixPath, path)) {
            (Deref @ Absyn::Path::FULLYQUALIFIED { path: p }, p2) => {
                { (prefixPath, path) = (p.clone(), p2.clone()); continue '__tco; }
            },
            (p, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
                { (prefixPath, path) = (p.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ Absyn::Path::IDENT { name: id }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                return stringEq(&id, &id2)
            },
            (Deref @ Absyn::Path::IDENT { name: id }, Deref @ Absyn::Path::QUALIFIED { name: id2, .. }) => {
                return stringEq(&id, &id2)
            },
            (Deref @ Absyn::Path::QUALIFIED { name: id, path: p }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: p2 }) => {
                return stringEq(&id, &id2) && pathPrefixOf(p.clone(), p2.clone())
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn removePrefix(
    mut prefix_path: metamodelica::Ref<Absyn::Path>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((prefix_path, path)) {
            (p, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
                { (prefix_path, path) = (p.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ Absyn::Path::QUALIFIED { name: id1, path: p }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: p2 }) => {
                let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                { (prefix_path, path) = (p.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ Absyn::Path::IDENT { name: id1 }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: p2 }) => {
                let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                return Ok(p2.clone())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn removePrefixOpt<'__b>(
    mut prefixPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut path: &'__b metamodelica::Ref<Absyn::Path>,
) -> Option<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (prefixPath, path) {
            (_, Deref @ Absyn::Path::FULLYQUALIFIED { .. }) => { (prefixPath, path) = (prefixPath, var_field!((**path).path, Absyn::Path::FULLYQUALIFIED)); continue '__tco; },
            (Deref @ Absyn::Path::QUALIFIED { .. }, Deref @ Absyn::Path::QUALIFIED { .. }) if (metamodelica::stringEq(&var_field!((**prefixPath).name, Absyn::Path::QUALIFIED), &var_field!((**path).name, Absyn::Path::QUALIFIED))) => { (prefixPath, path) = (var_field!((**prefixPath).path, Absyn::Path::QUALIFIED), var_field!((**path).path, Absyn::Path::QUALIFIED)); continue '__tco; },
            (Deref @ Absyn::Path::IDENT { .. }, Deref @ Absyn::Path::QUALIFIED { .. }) if (metamodelica::stringEq(&var_field!((**prefixPath).name, Absyn::Path::IDENT), &var_field!((**path).name, Absyn::Path::QUALIFIED))) => return Some(var_field!((**path).path, Absyn::Path::QUALIFIED).clone()),
            _ => return None,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn removePartialPrefix(
    mut inPrefix: &metamodelica::Ref<Absyn::Path>,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = &**inPrefix;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(removePrefix(inPrefix.clone(), inPath.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { .. } => {
                    Ok(removePrefix(var_field!((**inPrefix).path, Absyn::Path::QUALIFIED).clone(), inPath.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
                    Ok(removePartialPrefix(var_field!((**inPrefix).path, Absyn::Path::FULLYQUALIFIED), inPath))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPath.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outPath
}

pub fn getCrefsFromSubs<'__b>(
    mut isubs: &'__b metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut includeSubs: bool,
    mut includeFunctions: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    crefs = (::match_deref::match_deref! { match isubs {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: subs } => {
            getCrefsFromSubs(subs, includeSubs, includeFunctions)?
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: exp }, tail: subs } => {
            let mut crefs1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            crefs1 = getCrefsFromSubs(subs, includeSubs, includeFunctions)?;
            crefs = getCrefFromExp(exp.clone(), includeSubs, includeFunctions)?;
            listAppend(crefs, crefs1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(crefs)
}

pub fn getCrefFromExp(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut includeSubs: bool,
    mut includeFunctions: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ Absyn::Exp::INTEGER { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::REAL { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::STRING { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::BOOL { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::ALLWILD { .. } } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::WILD { .. } } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::CREF { componentRef: cr } if (!(includeSubs)) => {
                return Ok(list![cr.clone()])
            },
            Deref @ Absyn::Exp::CREF { componentRef: cr } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                subs = getSubsFromCref(metamodelica::AsArg::as_arg(&cr), includeSubs, includeFunctions)?;
                l1 = getCrefsFromSubs(&subs, includeSubs, includeFunctions)?;
                return Ok(metamodelica::cons(cr.clone(), l1))
            },
            Deref @ Absyn::Exp::BINARY { exp1: e1, exp2: e2, .. } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?;
                l2 = getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?;
                return Ok(listAppend(l1, l2))
            },
            Deref @ Absyn::Exp::UNARY { exp: e1, .. } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                { (inExp, includeSubs, includeFunctions) = (e1.clone(), includeSubs, includeFunctions); continue '__tco; }
            },
            Deref @ Absyn::Exp::LBINARY { exp1: e1, exp2: e2, .. } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?;
                l2 = getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?;
                return Ok(listAppend(l1, l2))
            },
            Deref @ Absyn::Exp::LUNARY { exp: e1, .. } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                { (inExp, includeSubs, includeFunctions) = (e1.clone(), includeSubs, includeFunctions); continue '__tco; }
            },
            Deref @ Absyn::Exp::RELATION { exp1: e1, exp2: e2, .. } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?;
                l2 = getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?;
                return Ok(listAppend(l1, l2))
            },
            Deref @ Absyn::Exp::IFEXP { ifExp: e1, trueBranch: e2, elseBranch: e3, .. } => {
                return Ok(List::flatten(list![getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?, getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?, getCrefFromExp(e3.clone(), includeSubs, includeFunctions)?])?)
            },
            Deref @ Absyn::Exp::CALL { function_: cr, functionArgs: farg, .. } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                res = getCrefFromFarg(metamodelica::AsArg::as_arg(&farg), includeSubs, includeFunctions)?;
                if (includeFunctions) {return Ok(metamodelica::cons(cr.clone(), res))} else {return Ok(res)}
            },
            Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cr, functionArgs: farg } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                res = getCrefFromFarg(metamodelica::AsArg::as_arg(&farg), includeSubs, includeFunctions)?;
                if (includeFunctions) {return Ok(metamodelica::cons(cr.clone(), res))} else {return Ok(res)}
            },
            Deref @ Absyn::Exp::ARRAY { arrayExp: expl } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut lstres1: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
                lstres1 = List::map2(expl.clone(), &getCrefFromExp, includeSubs, includeFunctions)?;
                return Ok(List::flatten(lstres1)?)
            },
            Deref @ Absyn::Exp::MATRIX { matrix: expll } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                return Ok(List::flatten(List::flatten(List::map2List(expll.clone(), &getCrefFromExp, includeSubs, includeFunctions)?)?)?)
            },
            Deref @ Absyn::Exp::RANGE { start: e1, step: Some(e3), stop: e2 } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?;
                l2 = getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?;
                l2 = listAppend(l1, l2);
                l1 = getCrefFromExp(e3.clone(), includeSubs, includeFunctions)?;
                return Ok(listAppend(l1, l2))
            },
            Deref @ Absyn::Exp::RANGE { start: e1, step: None, stop: e2 } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?;
                l2 = getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?;
                return Ok(listAppend(l1, l2))
            },
            Deref @ Absyn::Exp::END { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::TUPLE { expressions: expl } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut crefll: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
                crefll = List::map2(expl.clone(), &getCrefFromExp, includeSubs, includeFunctions)?;
                return Ok(List::flatten(crefll)?)
            },
            Deref @ Absyn::Exp::CODE { .. } => {
                return Ok(metamodelica::nil())
            },
            Deref @ Absyn::Exp::AS { exp: e1, .. } => {
                { (inExp, includeSubs, includeFunctions) = (e1.clone(), includeSubs, includeFunctions); continue '__tco; }
            },
            Deref @ Absyn::Exp::CONS { head: e1, rest: e2 } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(e1.clone(), includeSubs, includeFunctions)?;
                l2 = getCrefFromExp(e2.clone(), includeSubs, includeFunctions)?;
                return Ok(listAppend(l1, l2))
            },
            Deref @ Absyn::Exp::LIST { exps: expl } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut crefll: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
                crefll = List::map2(expl.clone(), &getCrefFromExp, includeSubs, includeFunctions)?;
                return Ok(List::flatten(crefll)?)
            },
            Deref @ Absyn::Exp::MATCHEXP { .. } => {
                return Ok(return Err("fail"))
            },
            Deref @ Absyn::Exp::DOT { exp: __inExp_exp, .. } => {
                { (inExp, includeSubs, includeFunctions) = (__inExp_exp.clone(), includeSubs, includeFunctions); continue '__tco; }
            },
            Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: __inExp_exp, .. } => {
                { (inExp, includeSubs, includeFunctions) = (__inExp_exp.clone(), includeSubs, includeFunctions); continue '__tco; }
            },
            Deref @ Absyn::Exp::SUBSCRIPTED_EXP { exp: __inExp_exp, subscripts: __inExp_subscripts } => {
                let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                l1 = getCrefFromExp(__inExp_exp.clone(), includeSubs, includeFunctions)?;
                if includeSubs {
                    l2 = getCrefsFromSubs(metamodelica::AsArg::as_arg(&__inExp_subscripts), includeSubs, includeFunctions)?;
                    l1 = listAppend(l2, l1);
                }
                return Ok(l1)
            },
            Deref @ Absyn::Exp::BREAK { .. } => {
                return Ok(metamodelica::nil())
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynUtil.getCrefFromExp")); __mm_s.push_str(&*literal!(" failed ")); __mm_s.push_str(&*Dump::printExpStr(inExp)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/AbsynUtil.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getCrefFromFarg(
    mut inFunctionArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut includeSubs: bool,
    mut includeFunctions: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut outComponentRefLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outComponentRefLst = (match &**inFunctionArgs {
        Absyn::FunctionArgs::FUNCTIONARGS {
            args: expl,
            argNames: nargl,
        } => {
            let mut l1: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
            let mut l2: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
            let mut fl1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut fl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            l1 = List::map2(expl.clone(), &getCrefFromExp, includeSubs, includeFunctions)?;
            fl1 = List::flatten(l1)?;
            l2 = List::map2(
                nargl.clone(),
                &move |__a0: metamodelica::Ref<Absyn::NamedArg>, __a1: bool, __a2: bool| {
                    getCrefFromNarg(&__a0, __a1, __a2)
                },
                includeSubs,
                includeFunctions,
            )?;
            fl2 = List::flatten(l2)?;
            res = listAppend(fl1, fl2);
            res
        }
        Absyn::FunctionArgs::FOR_ITER_FARG {
            exp,
            iterType: _,
            iterators,
        } => {
            let mut l1: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
            let mut l2: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>>;
            let mut fl1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut fl2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut fl3: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            l1 = List::map2Option(
                &(List::map(iterators.clone(), &move |__a0: metamodelica::Ref<
                    Absyn::ForIterator,
                >|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(iteratorRange(&__a0))
                })?),
                &getCrefFromExp,
                includeSubs,
                includeFunctions,
            )?;
            l2 = List::map2Option(
                &(List::map(iterators.clone(), &move |__a0: metamodelica::Ref<
                    Absyn::ForIterator,
                >|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(iteratorGuard(&__a0))
                })?),
                &getCrefFromExp,
                includeSubs,
                includeFunctions,
            )?;
            fl1 = List::flatten(l1)?;
            fl2 = List::flatten(l2)?;
            fl3 = getCrefFromExp(exp.clone(), includeSubs, includeFunctions)?;
            res = listAppend(fl1, listAppend(fl2, fl3));
            res
        }
    });
    Ok(outComponentRefLst)
}

pub fn iteratorName(mut iterator: &metamodelica::Ref<Absyn::ForIterator>) -> ArcStr {
    let mut name: ArcStr;
    let __arc1 = &(*iterator);
    let Absyn::ITERATOR { name: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    name
}

pub(crate) fn iteratorRange(
    mut iterator: &metamodelica::Ref<Absyn::ForIterator>,
) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut range: Option<metamodelica::Ref<Absyn::Exp>>;
    let __arc1 = &(*iterator);
    let Absyn::ITERATOR { range: __pa0, .. } = &**__arc1;
    range = metamodelica::Own::own(__pa0);
    range
}

pub(crate) fn iteratorGuard(
    mut iterator: &metamodelica::Ref<Absyn::ForIterator>,
) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut guardExp: Option<metamodelica::Ref<Absyn::Exp>>;
    let __arc1 = &(*iterator);
    let Absyn::ITERATOR { guardExp: __pa0, .. } = &**__arc1;
    guardExp = metamodelica::Own::own(__pa0);
    guardExp
}

// stefan
pub fn getNamedFuncArgNamesAndValues(
    mut namedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> (
    metamodelica::List<ArcStr>,
    metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) {
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut values: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
    for mut arg in &*namedArgs.reverse() {
        names = metamodelica::cons(arg.argName.clone(), names);
        values = metamodelica::cons(arg.argValue.clone(), values);
    }
    (names, values)
}

fn getCrefFromNarg(
    mut inNamedArg: &metamodelica::Ref<Absyn::NamedArg>,
    mut includeSubs: bool,
    mut includeFunctions: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut outComponentRefLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outComponentRefLst = getCrefFromExp(inNamedArg.argValue.clone(), includeSubs, includeFunctions)?;
    Ok(outComponentRefLst)
}

pub fn joinPaths(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPath1, inPath2)) {
            (Deref @ Absyn::Path::IDENT { name: r#str }, p2) => {
                return Ok(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: r#str.clone(), path: p2.clone() }))
            },
            (Deref @ Absyn::Path::QUALIFIED { name: r#str, path: p }, p2) => {
                let mut p_1: metamodelica::Ref<Absyn::Path>;
                p_1 = joinPaths(p.clone(), p2.clone())?;
                return Ok(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: r#str.clone(), path: p_1 }))
            },
            (Deref @ Absyn::Path::FULLYQUALIFIED { path: p }, p2) => {
                { (inPath1, inPath2) = (p.clone(), p2.clone()); continue '__tco; }
            },
            (p, Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 }) => {
                { (inPath1, inPath2) = (p.clone(), p2.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn joinPathsOpt(
    mut inPath1: Option<metamodelica::Ref<Absyn::Path>>,
    mut inPath2: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath1) {
        None => {
            inPath2
        },
        Some(p) => {
            joinPaths(p.clone(), inPath2)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

pub fn joinPathsOptSuffix(
    mut inPath1: metamodelica::Ref<Absyn::Path>,
    mut inPath2: Option<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath2) {
        Some(p) => {
            joinPaths(inPath1, p.clone())?
        },
        _ => {
            inPath1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

pub fn stripLast(mut inPath: &metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match inPath {
        Deref @ Absyn::Path::QUALIFIED { name: r#str, path: Deref @ Absyn::Path::IDENT { .. } } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: r#str.clone() })
        },
        Deref @ Absyn::Path::QUALIFIED { name: r#str, path: p } => {
            let mut p = (*p).clone();
            p = stripLast(metamodelica::AsArg::as_arg(&p))?;
            metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: r#str.clone(), path: p.clone() })
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path: p } => {
            let mut p = (*p).clone();
            p = stripLast(metamodelica::AsArg::as_arg(&p))?;
            metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: p.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn crefStripLast(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match inCref {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            return Err("fail")
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { name: r#str, subscripts: subs, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { .. } } => {
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: r#str.clone(), subscripts: subs.clone() })
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { name: r#str, subscripts: subs, componentRef: c } => {
            let mut c_1: metamodelica::Ref<Absyn::ComponentRef>;
            c_1 = crefStripLast(c)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: r#str.clone(), subscripts: subs.clone(), componentRef: c_1 })
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut c_1: metamodelica::Ref<Absyn::ComponentRef>;
            c_1 = crefStripLast(c)?;
            crefMakeFullyQualified(c_1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn splitQualAndIdentPath<'__b>(
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
) -> Result<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inPath {
            Deref @ Absyn::Path::QUALIFIED { name: s1, path: Deref @ Absyn::Path::IDENT { name: s2 } } => {
                return Ok((metamodelica::Ref::new(Absyn::Path::IDENT { name: s1.clone() }), metamodelica::Ref::new(Absyn::Path::IDENT { name: s2.clone() })))
            },
            Deref @ Absyn::Path::QUALIFIED { name: s1, path: qPath } => {
                let mut curPath: metamodelica::Ref<Absyn::Path>;
                let mut identPath: metamodelica::Ref<Absyn::Path>;
                (curPath, identPath) = splitQualAndIdentPath(qPath)?;
                return Ok((metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: s1.clone(), path: curPath }), identPath))
            },
            Deref @ Absyn::Path::FULLYQUALIFIED { path: qPath } => {
                let mut curPath: metamodelica::Ref<Absyn::Path>;
                let mut identPath: metamodelica::Ref<Absyn::Path>;
                { inPath = qPath; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn crefToPath(
    mut inComponentRef: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match inComponentRef {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: i, subscripts: Deref @ metamodelica::ListNode::Nil } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: i.clone() })
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: c } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = crefToPath(c)?;
            metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: i.clone(), path: p })
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = crefToPath(c)?;
            metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: p })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn elementSpecToPath(
    mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &((*inElementSpec)) {
        Deref @ Absyn::ElementSpec::EXTENDS { path: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outPath = metamodelica::Own::own(__pa0);
    Ok(outPath)
}

pub fn crefToPathIgnoreSubs(
    mut inComponentRef: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &**inComponentRef {
        Absyn::ComponentRef::CREF_IDENT { name: i, .. } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: i.clone() })
        }
        Absyn::ComponentRef::CREF_QUAL {
            name: i,
            componentRef: c,
            ..
        } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = crefToPathIgnoreSubs(c)?;
            metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: i.clone(),
                path: p,
            })
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            p = crefToPathIgnoreSubs(c)?;
            metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: p })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outPath)
}

pub fn crefToTypeSpec(mut cref: metamodelica::Ref<Absyn::ComponentRef>) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    subs = crefGetLastSubs(&cref)?;
    path = crefToPath(&(crefStripLastSubs(cref)?))?;
    ty = metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
        path: path,
        arrayDim: if ((subs).is_empty()) { None } else { Some(subs) },
    });
    Ok(ty)
}

pub fn pathToCref(mut inPath: &metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<Absyn::ComponentRef>;
    outComponentRef = (match &**inPath {
        Absyn::Path::IDENT { name: i } => metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
            name: i.clone(),
            subscripts: metamodelica::nil(),
        }),
        Absyn::Path::QUALIFIED { name: i, path: p } => {
            let mut c: metamodelica::Ref<Absyn::ComponentRef>;
            c = pathToCref(p);
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: i.clone(),
                subscripts: metamodelica::nil(),
                componentRef: c,
            })
        }
        Absyn::Path::FULLYQUALIFIED { path: p } => {
            let mut c: metamodelica::Ref<Absyn::ComponentRef>;
            c = pathToCref(p);
            crefMakeFullyQualified(c)
        }
    });
    outComponentRef
}

pub fn pathToCrefWithSubs(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inSubs: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<Absyn::ComponentRef>;
    outComponentRef = (match &**inPath {
        Absyn::Path::IDENT { name: i } => metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
            name: i.clone(),
            subscripts: inSubs.clone(),
        }),
        Absyn::Path::QUALIFIED { name: i, path: p } => {
            let mut c: metamodelica::Ref<Absyn::ComponentRef>;
            c = pathToCrefWithSubs(p, inSubs);
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: i.clone(),
                subscripts: metamodelica::nil(),
                componentRef: c,
            })
        }
        Absyn::Path::FULLYQUALIFIED { path: p } => {
            let mut c: metamodelica::Ref<Absyn::ComponentRef>;
            c = pathToCrefWithSubs(p, inSubs);
            crefMakeFullyQualified(c)
        }
    });
    outComponentRef
}

pub(crate) fn crefLastIdent<'__b>(mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(var_field!((**cref).name, Absyn::ComponentRef::CREF_IDENT).clone());
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL);
                continue '__tco;
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub(crate) fn crefFirstIdentNoSubs<'__b>(mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match cref {
            Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::ListNode::Nil, .. } => return Ok(var_field!((**cref).name, Absyn::ComponentRef::CREF_IDENT).clone()),
            Deref @ Absyn::ComponentRef::CREF_QUAL { subscripts: Deref @ metamodelica::ListNode::Nil, .. } => return Ok(var_field!((**cref).name, Absyn::ComponentRef::CREF_QUAL).clone()),
            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => { cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED); continue '__tco; },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn crefIsIdent(mut inComponentRef: &metamodelica::Ref<Absyn::ComponentRef>) -> bool {
    let mut outIsIdent: bool;
    outIsIdent = (match &**inComponentRef {
        Absyn::ComponentRef::CREF_IDENT { .. } => true,
        _ => false,
    });
    outIsIdent
}

pub fn crefIsQual(mut inComponentRef: &metamodelica::Ref<Absyn::ComponentRef>) -> bool {
    let mut outIsQual: bool;
    outIsQual = (match &**inComponentRef {
        Absyn::ComponentRef::CREF_QUAL { .. } => true,
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => true,
        _ => false,
    });
    outIsQual
}

pub fn crefFirstSubs<'__b>(
    mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone());
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                return Ok(var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone());
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefLastSubs<'__b>(
    mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone());
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL);
                continue '__tco;
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub(crate) fn crefSetFirstSubs(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = subscripts.clone());
            ()
        }
        Absyn::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; subscripts = subscripts.clone());
            ()
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefSetFirstSubs(__cref_componentRef.clone(), subscripts)?);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub fn crefSetLastSubs(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = inSubscripts.clone());
            ()
        }
        Absyn::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            ..
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; componentRef = crefSetLastSubs(__cref_componentRef.clone(), inSubscripts)?);
            ()
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefSetLastSubs(__cref_componentRef.clone(), inSubscripts)?);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub fn crefHasSubscripts<'__b>(mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match cref {
            Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => return !((var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT)).is_empty()),
            Deref @ Absyn::ComponentRef::CREF_QUAL { subscripts: Deref @ metamodelica::ListNode::Nil, .. } => { cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL); continue '__tco; },
            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => { cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED); continue '__tco; },
            Deref @ Absyn::ComponentRef::WILD { .. } => return false,
            Deref @ Absyn::ComponentRef::ALLWILD { .. } => return false,
            _ => return true,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getSubsFromCref<'__b>(
    mut cr: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut includeSubs: bool,
    mut includeFunctions: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut subscripts: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    subscripts = (match &**cr {
        Absyn::ComponentRef::CREF_IDENT {
            name: _,
            subscripts: subs2,
        } => subs2.clone(),
        Absyn::ComponentRef::CREF_QUAL {
            name: _,
            subscripts: subs2,
            componentRef: child,
        } => {
            subscripts = getSubsFromCref(child, includeSubs, includeFunctions)?;
            subscripts = List::unionOnTrue(
                &subscripts,
                subs2,
                &move |__a0: metamodelica::Ref<Absyn::Subscript>, __a1: metamodelica::Ref<Absyn::Subscript>| {
                    subscriptEqual(&__a0, &__a1)
                },
            )?;
            subscripts
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: child } => {
            subscripts = getSubsFromCref(child, includeSubs, includeFunctions)?;
            subscripts
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(subscripts)
}

pub fn getString<'__b>(mut exp: &'__b metamodelica::Ref<Absyn::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**exp {
        Absyn::Exp::EXPRESSIONCOMMENT { .. } => getString(var_field!((**exp).exp, Absyn::Exp::EXPRESSIONCOMMENT))?,
        Absyn::Exp::STRING { value: __esc_str } => {
            r#str = (*__esc_str).clone();
            r#str.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub fn stripCommentExpressions(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut onlyComments: bool,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    (exp, _) = traverseExp(
        exp,
        (std::sync::Arc::new(fnptr!(
            stripCommentExpressionsHelper,
            metamodelica::Ref<Absyn::Exp>,
            bool
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, bool) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)>
                    + 'static,
            >),
        onlyComments,
    )?;
    Ok(exp)
}

fn stripCommentExpressionsHelper(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut onlyComments: bool,
) -> (metamodelica::Ref<Absyn::Exp>, bool) {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut onlyComments: bool = onlyComments;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: __esc_e, tail: Deref @ metamodelica::ListNode::Nil } } if (!(onlyComments)) => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: __exp_exp, .. } => __exp_exp.clone(),
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, onlyComments)
}

pub fn crefGetLastIdent<'__b>(mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(var_field!((**cref).name, Absyn::ComponentRef::CREF_IDENT).clone());
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL);
                continue '__tco;
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefGetLastSubs<'__b>(
    mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone());
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL);
                continue '__tco;
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefStripLastSubs(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = metamodelica::nil());
            ()
        }
        Absyn::ComponentRef::CREF_QUAL {
            componentRef: __cref_componentRef,
            ..
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; componentRef = crefStripLastSubs(__cref_componentRef.clone())?);
            ()
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefStripLastSubs(__cref_componentRef.clone())?);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub fn joinCrefs(
    mut inComponentRef1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<Absyn::ComponentRef>;
    outComponentRef = (match &**inComponentRef1 {
        Absyn::ComponentRef::CREF_IDENT {
            name: id,
            subscripts: sub,
        } => {
            let mut cr2 = inComponentRef2;
            if '__try0: {
                ::match_deref::match_deref! { match &(cr2.clone()) {
                    Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => (),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: id.clone(),
                subscripts: sub.clone(),
                componentRef: cr2,
            })
        }
        Absyn::ComponentRef::CREF_QUAL {
            name: id,
            subscripts: sub,
            componentRef: cr,
        } => {
            let mut cr2 = inComponentRef2;
            let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
            cr_1 = joinCrefs(cr, cr2)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: id.clone(),
                subscripts: sub.clone(),
                componentRef: cr_1,
            })
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut cr2 = inComponentRef2;
            let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
            cr_1 = joinCrefs(cr, cr2)?;
            crefMakeFullyQualified(cr_1)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn crefFirstIdent<'__b>(mut inCref: &'__b metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    '__tco: loop {
        match &**inCref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(var_field!((**inCref).name, Absyn::ComponentRef::CREF_IDENT).clone());
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                return Ok(var_field!((**inCref).name, Absyn::ComponentRef::CREF_QUAL).clone());
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                inCref = var_field!((**inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefSetFirstIdent(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut ident: &ArcStr,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = ident.clone());
            ()
        }
        Absyn::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; name = ident.clone());
            ()
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefSetFirstIdent(__cref_componentRef.clone(), ident));
            ()
        }
        _ => (),
    });
    cref
}

pub fn crefSecondIdent<'__b>(mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                return Ok(crefFirstIdent(var_field!(
                    (**cref).componentRef,
                    Absyn::ComponentRef::CREF_QUAL
                ))?);
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                cref = var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefFirstCref<'__b>(
    mut inCref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    '__tco: loop {
        match &**inCref {
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                return metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: var_field!((**inCref).name, Absyn::ComponentRef::CREF_QUAL).clone(),
                    subscripts: var_field!((**inCref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(),
                });
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                inCref = var_field!((**inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED);
                continue '__tco;
            }
            _ => return inCref.clone(),
        }
    }
}

pub fn crefStripFirst<'__b>(
    mut inComponentRef: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    '__tco: loop {
        match &**inComponentRef {
            Absyn::ComponentRef::CREF_QUAL { componentRef: cr, .. } => return Ok(cr.clone()),
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
                inComponentRef = cr;
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefIsFullyQualified(mut inCref: &metamodelica::Ref<Absyn::ComponentRef>) -> bool {
    let mut outIsFullyQualified: bool;
    outIsFullyQualified = (match &**inCref {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => true,
        _ => false,
    });
    outIsFullyQualified
}

pub fn crefMakeFullyQualified(
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<Absyn::ComponentRef>;
    outComponentRef = (match &*inComponentRef {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => inComponentRef,
        _ => metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: inComponentRef,
        }),
    });
    outComponentRef
}

pub fn restrString(mut inRestriction: &Absyn::Restriction) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inRestriction.clone() {
        Absyn::Restriction::R_CLASS { .. } => literal!("CLASS"),
        Absyn::Restriction::R_OPTIMIZATION { .. } => literal!("OPTIMIZATION"),
        Absyn::Restriction::R_MODEL { .. } => literal!("MODEL"),
        Absyn::Restriction::R_RECORD { .. } => literal!("RECORD"),
        Absyn::Restriction::R_BLOCK { .. } => literal!("BLOCK"),
        Absyn::Restriction::R_CONNECTOR { .. } => literal!("CONNECTOR"),
        Absyn::Restriction::R_EXP_CONNECTOR { .. } => literal!("EXPANDABLE CONNECTOR"),
        Absyn::Restriction::R_TYPE { .. } => literal!("TYPE"),
        Absyn::Restriction::R_PACKAGE { .. } => literal!("PACKAGE"),
        Absyn::Restriction::R_FUNCTION {
            functionRestriction:
                Absyn::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: Absyn::FunctionPurity::PURE { .. },
                },
        } => literal!("PURE FUNCTION"),
        Absyn::Restriction::R_FUNCTION {
            functionRestriction:
                Absyn::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: Absyn::FunctionPurity::IMPURE { .. },
                },
        } => literal!("IMPURE FUNCTION"),
        Absyn::Restriction::R_FUNCTION {
            functionRestriction:
                Absyn::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: Absyn::FunctionPurity::NO_PURITY { .. },
                },
        } => literal!("FUNCTION"),
        Absyn::Restriction::R_FUNCTION {
            functionRestriction: Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
        } => literal!("OPERATOR FUNCTION"),
        Absyn::Restriction::R_PREDEFINED_INTEGER { .. } => literal!("PREDEFINED_INT"),
        Absyn::Restriction::R_PREDEFINED_REAL { .. } => literal!("PREDEFINED_REAL"),
        Absyn::Restriction::R_PREDEFINED_STRING { .. } => literal!("PREDEFINED_STRING"),
        Absyn::Restriction::R_PREDEFINED_BOOLEAN { .. } => literal!("PREDEFINED_BOOL"),
        Absyn::Restriction::R_PREDEFINED_CLOCK { .. } => literal!("PREDEFINED_CLOCK"),
        Absyn::Restriction::R_UNIONTYPE { .. } => literal!("UNIONTYPE"),
        _ => literal!("* Unknown restriction *"),
    });
    outString
}

pub fn lastClassname(mut inProgram: Absyn::Program) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut lst: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut id: ArcStr;
    let Absyn::PROGRAM { classes: __pa0, .. } = inProgram;
    lst = metamodelica::Own::own(__pa0);
    let __arc2 = List::last(&lst)?;
    let Absyn::CLASS { name: __pa1, .. } = &*__arc2;
    id = metamodelica::Own::own(__pa1);
    outPath = metamodelica::Ref::new(Absyn::Path::IDENT { name: id });
    Ok(outPath)
}

pub fn classFilename(mut inClass: &metamodelica::Ref<Absyn::Class>) -> Result<ArcStr> {
    let mut outFilename: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*inClass)) {
        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outFilename = metamodelica::Own::own(__pa0);
    Ok(outFilename)
}

pub fn setClassFilename(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut fileName: ArcStr,
) -> metamodelica::Ref<Absyn::Class> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass.clone()) {
        cl @ Deref @ Absyn::Class { info: info @ SourceInfo { fileName: old_filename, .. }, .. } if (!(stringEq(&old_filename, &fileName))) => {
            let mut cl = (*cl).clone();
            let mut info = (*info).clone();
            info.fileName = fileName.clone();
            assign_field!(
                cl.info = info.clone(),
                cl.body = setClassDefFilename(cl.body.clone(), metamodelica::AsArg::as_arg(&old_filename), fileName.clone())
            );
            cl.clone()
        },
        _ => {
            inClass
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClass
}

fn setClassDefFilename(
    mut body: metamodelica::Ref<Absyn::ClassDef>,
    mut oldFilename: &ArcStr,
    mut newFilename: ArcStr,
) -> metamodelica::Ref<Absyn::ClassDef> {
    let mut body: metamodelica::Ref<Absyn::ClassDef> = body;
    let () = (match &*body {
        Absyn::ClassDef::PARTS {
            classParts: __body_classParts,
            ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut part in (__body_classParts.clone()).into_iter().cloned() {
                    let __x = setClassPartFilename(part.clone(), oldFilename, newFilename.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __body_parts, ..
        } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut part in (__body_parts.clone()).into_iter().cloned() {
                    let __x = setClassPartFilename(part.clone(), oldFilename, newFilename.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    body
}

fn setClassPartFilename(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
    mut oldFilename: &ArcStr,
    mut newFilename: ArcStr,
) -> metamodelica::Ref<Absyn::ClassPart> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let () = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut e in (__part_contents.clone()).into_iter().cloned() {
                    let __x = setElementItemFilename(e.clone(), oldFilename, newFilename.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut e in (__part_contents.clone()).into_iter().cloned() {
                    let __x = setElementItemFilename(e.clone(), oldFilename, newFilename.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    part
}

fn setElementItemFilename(
    mut item: metamodelica::Ref<Absyn::ElementItem>,
    mut oldFilename: &ArcStr,
    mut newFilename: ArcStr,
) -> metamodelica::Ref<Absyn::ElementItem> {
    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let () = (match &*item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = setElementFilename(__item_element.clone(), oldFilename, newFilename));
            ()
        }
        _ => (),
    });
    item
}

fn setElementFilename(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut oldFilename: &ArcStr,
    mut newFilename: ArcStr,
) -> metamodelica::Ref<Absyn::Element> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            info: info @ SourceInfo { .. },
            ..
        } if (stringEq(&info.fileName, &oldFilename)) => {
            let mut info = (*info).clone();
            info.fileName = newFilename.clone();
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                info = info.clone(),
                specification = setElementSpecFilename(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), newFilename)
            );
            ()
        }
        Absyn::Element::DEFINEUNIT {
            info: info @ SourceInfo { .. },
            ..
        } if (stringEq(&info.fileName, &oldFilename)) => {
            let mut info = (*info).clone();
            info.fileName = newFilename;
            assign_variant_field!(element => Absyn::Element::DEFINEUNIT; info = info.clone());
            ()
        }
        Absyn::Element::TEXT {
            info: info @ SourceInfo { .. },
            ..
        } if (stringEq(&info.fileName, &oldFilename)) => {
            let mut info = (*info).clone();
            info.fileName = newFilename;
            assign_variant_field!(element => Absyn::Element::TEXT; info = info.clone());
            ()
        }
        _ => (),
    });
    element
}

fn setElementSpecFilename(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut newFilename: ArcStr,
) -> metamodelica::Ref<Absyn::ElementSpec> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = setClassFilename(__spec_class_.clone(), newFilename));
            ()
        }
        Absyn::ElementSpec::IMPORT {
            info: info @ SourceInfo { .. },
            ..
        } => {
            let mut info = (*info).clone();
            info.fileName = newFilename;
            assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; info = info.clone());
            ()
        }
        _ => (),
    });
    spec
}

pub fn setClassName(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut newName: ArcStr,
) -> metamodelica::Ref<Absyn::Class> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass;
    outClass = (match &*outClass {
        Absyn::Class { .. } => {
            assign_field!(outClass.name = newName);
            outClass
        }
    });
    outClass
}

pub fn setClassBody(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inBody: metamodelica::Ref<Absyn::ClassDef>,
) -> metamodelica::Ref<Absyn::Class> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass;
    outClass = (match &*outClass {
        Absyn::Class { .. } => {
            assign_field!(outClass.body = inBody);
            outClass
        }
    });
    outClass
}

pub fn crefEqual<'__b>(
    mut cref1: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut cref2: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (cref1, cref2) {
            (Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, Deref @ Absyn::ComponentRef::CREF_IDENT { .. }) => return Ok(stringEq(&var_field!((**cref1).name, Absyn::ComponentRef::CREF_IDENT), &var_field!((**cref2).name, Absyn::ComponentRef::CREF_IDENT)) && subscriptsEqual(var_field!((**cref1).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), var_field!((**cref2).subscripts, Absyn::ComponentRef::CREF_IDENT).clone())?),
            (Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, Deref @ Absyn::ComponentRef::CREF_QUAL { .. }) => return Ok(stringEq(&var_field!((**cref1).name, Absyn::ComponentRef::CREF_QUAL), &var_field!((**cref2).name, Absyn::ComponentRef::CREF_QUAL)) && subscriptsEqual(var_field!((**cref1).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), var_field!((**cref2).subscripts, Absyn::ComponentRef::CREF_QUAL).clone())? && crefEqual(var_field!((**cref1).componentRef, Absyn::ComponentRef::CREF_QUAL), var_field!((**cref2).componentRef, Absyn::ComponentRef::CREF_QUAL))?),
            (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }) => { (cref1, cref2) = (var_field!((**cref1).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED), var_field!((**cref2).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED)); continue '__tco; },
            _ => return Ok(false),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn crefFirstEqual(
    mut iCr1: &metamodelica::Ref<Absyn::ComponentRef>,
    mut iCr2: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = stringEq(&(crefFirstIdent(iCr1)?), &(crefFirstIdent(iCr2)?));
    Ok(outBoolean)
}

pub fn subscriptEqual(
    mut inSubscript1: &metamodelica::Ref<Absyn::Subscript>,
    mut inSubscript2: &metamodelica::Ref<Absyn::Subscript>,
) -> Result<bool> {
    let mut outIsEqual: bool;
    outIsEqual = (::match_deref::match_deref! { match (inSubscript1, inSubscript2) {
        (Deref @ Absyn::Subscript::NOSUB { .. }, Deref @ Absyn::Subscript::NOSUB { .. }) => {
            true
        },
        (Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }) => {
            expEqual(e1.clone(), e2.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsEqual)
}

pub(crate) fn subscriptsEqual(
    mut inSubList1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inSubList2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<bool> {
    let mut outIsEqual: bool;
    outIsEqual = List::isEqualOnTrue(inSubList1, inSubList2, &move |__a0: metamodelica::Ref<
        Absyn::Subscript,
    >,
                                                                    __a1: metamodelica::Ref<
        Absyn::Subscript,
    >| subscriptEqual(&__a0, &__a1))?;
    Ok(outIsEqual)
}

pub fn crefEqualNoSubs<'__b>(
    mut cr1: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut cr2: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (cr1, cr2) {
            (Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, Deref @ Absyn::ComponentRef::CREF_IDENT { .. }) => return stringEq(&var_field!((**cr1).name, Absyn::ComponentRef::CREF_IDENT), &var_field!((**cr2).name, Absyn::ComponentRef::CREF_IDENT)),
            (Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, Deref @ Absyn::ComponentRef::CREF_QUAL { .. }) => return stringEq(&var_field!((**cr1).name, Absyn::ComponentRef::CREF_QUAL), &var_field!((**cr2).name, Absyn::ComponentRef::CREF_QUAL)) && crefEqualNoSubs(var_field!((**cr1).componentRef, Absyn::ComponentRef::CREF_QUAL), var_field!((**cr2).componentRef, Absyn::ComponentRef::CREF_QUAL)),
            (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }) => { (cr1, cr2) = (var_field!((**cr1).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED), var_field!((**cr2).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED)); continue '__tco; },
            _ => return false,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn crefCompare<'__b>(
    mut cr1: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut cr2: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<i32> {
    let mut comp: i32;
    let mut name: ArcStr;
    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    if referenceEq(&*(&**cr1), &*(&*cr2)) {
        comp = 0;
        return Ok(comp);
    }
    comp = Util::intCompare(
        metamodelica::valueConstructor((&*&**cr1))?,
        metamodelica::valueConstructor((&*&*cr2))?,
    );
    if comp != 0 {
        return Ok(comp);
    }
    comp = (match &**cr1 {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(cr2) {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            crefCompare(
                var_field!((**cr1).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED),
                cr,
            )?
        }
        Absyn::ComponentRef::CREF_QUAL { .. } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(cr2) {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: __pa0, subscripts: __pa1, componentRef: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            cr = metamodelica::Own::own(__pa2);
            comp = stringCompare(&var_field!((**cr1).name, Absyn::ComponentRef::CREF_QUAL), &name);
            if comp == 0 {
                comp = List::compare(
                    var_field!((**cr1).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(),
                    subs,
                    &move |__a0: metamodelica::Ref<Absyn::Subscript>, __a1: metamodelica::Ref<Absyn::Subscript>| {
                        subscriptCompare(&__a0, &__a1)
                    },
                )?;
            }
            if (comp == 0) {
                crefCompare(var_field!((**cr1).componentRef, Absyn::ComponentRef::CREF_QUAL), cr)?
            } else {
                comp
            }
        }
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cr2) {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, subscripts: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            comp = stringCompare(&var_field!((**cr1).name, Absyn::ComponentRef::CREF_IDENT), &name);
            if (comp == 0) {
                List::compare(
                    var_field!((**cr1).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(),
                    subs,
                    &move |__a0: metamodelica::Ref<Absyn::Subscript>, __a1: metamodelica::Ref<Absyn::Subscript>| {
                        subscriptCompare(&__a0, &__a1)
                    },
                )?
            } else {
                comp
            }
        }
        _ => 0,
    });
    Ok(comp)
}

pub(crate) fn subscriptCompare(
    mut sub1: &metamodelica::Ref<Absyn::Subscript>,
    mut sub2: &metamodelica::Ref<Absyn::Subscript>,
) -> Result<i32> {
    let mut comp: i32;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    if referenceEq(&*(&**sub1), &*(&**sub2)) {
        comp = 0;
    }
    comp = Util::intCompare(
        metamodelica::valueConstructor((&*&**sub1))?,
        metamodelica::valueConstructor((&*&**sub2))?,
    );
    if comp != 0 {
        return Ok(comp);
    }
    comp = (match &**sub1 {
        Absyn::Subscript::NOSUB { .. } => 0,
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub1_subscript,
        } => {
            let __pa0 = ::match_deref::match_deref! { match &((*sub2)) {
                Deref @ Absyn::Subscript::SUBSCRIPT { subscript: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            exp = metamodelica::Own::own(__pa0);
            stringCompare(
                &(Dump::printExpStr(__sub1_subscript.clone())?),
                &(Dump::printExpStr(exp)?),
            )
        }
    });
    Ok(comp)
}

pub fn isPackageRestriction(mut inRestriction: &Absyn::Restriction) -> bool {
    let mut outIsPackage: bool;
    outIsPackage = (match inRestriction.clone() {
        Absyn::Restriction::R_PACKAGE { .. } => true,
        _ => false,
    });
    outIsPackage
}

pub(crate) fn isFunctionRestriction(mut inRestriction: &Absyn::Restriction) -> bool {
    let mut outIsFunction: bool;
    outIsFunction = (match inRestriction.clone() {
        Absyn::Restriction::R_FUNCTION { .. } => true,
        _ => false,
    });
    outIsFunction
}

pub fn expEqual(mut exp1: metamodelica::Ref<Absyn::Exp>, mut exp2: metamodelica::Ref<Absyn::Exp>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Absyn::Exp::INTEGER { .. }, Deref @ Absyn::Exp::REAL { .. }) => realEq(intReal(var_field!((*exp1).value, Absyn::Exp::INTEGER).clone()), stringReal(var_field!((*exp2).value, Absyn::Exp::REAL).clone())?),
        (Deref @ Absyn::Exp::REAL { .. }, Deref @ Absyn::Exp::INTEGER { .. }) => realEq(intReal(var_field!((*exp2).value, Absyn::Exp::INTEGER).clone()), stringReal(var_field!((*exp1).value, Absyn::Exp::REAL).clone())?),
        _ => exp1 == exp2,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn getClassName(mut inClass: &metamodelica::Ref<Absyn::Class>) -> ArcStr {
    let mut outName: ArcStr;
    let __arc1 = &(*inClass);
    let Absyn::CLASS { name: __pa0, .. } = &**__arc1;
    outName = metamodelica::Own::own(__pa0);
    outName
}

pub type IteratorIndexedCref = (metamodelica::Ref<Absyn::ComponentRef>, i32);

pub fn findIteratorIndexedCrefs(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inIterator: &ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    (_, outCrefs) = traverseExp(
        inExp,
        (std::sync::Arc::new({
            let __pe_b2 = inIterator.clone();
            move |__pe_a0, __pe_a1| Ok(findIteratorIndexedCrefs_traverser(__pe_a0, __pe_a1, &__pe_b2))
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
                    )> + 'static,
            >),
        metamodelica::nil(),
    )?;
    outCrefs = List::fold(
        &outCrefs,
        &({
            let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(
                move |__a0: (metamodelica::Ref<Absyn::ComponentRef>, i32),
                      __a1: (metamodelica::Ref<Absyn::ComponentRef>, i32)| {
                    iteratorIndexedCrefsEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<Absyn::ComponentRef>, i32),
                            (metamodelica::Ref<Absyn::ComponentRef>, i32),
                        ) -> Result<bool>
                        + 'static,
                >);
            move |__pe_a0, __pe_a1| List::unionEltOnTrue(__pe_a0, __pe_a1, &*__pe_b2)
        }),
        inCrefs,
    )?;
    Ok(outCrefs)
}

fn findIteratorIndexedCrefs_traverser(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
    mut inIterator: &ArcStr,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) {
    let mut outExp: metamodelica::Ref<Absyn::Exp> = inExp.clone();
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = (match &*inExp {
        Absyn::Exp::CREF {
            componentRef: __inExp_componentRef,
        } => getIteratorIndexedCrefs(__inExp_componentRef.clone(), inIterator, inCrefs),
        _ => inCrefs,
    });
    (outExp, outCrefs)
}

fn iteratorIndexedCrefsEqual(mut inCref1: &IteratorIndexedCref, mut inCref2: &IteratorIndexedCref) -> Result<bool> {
    let mut outEqual: bool;
    let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
    let mut idx1: i32;
    let mut idx2: i32;
    (cr1, idx1) = inCref1.clone();
    (cr2, idx2) = inCref2.clone();
    outEqual = idx1 == idx2 && crefEqual(&cr1, &cr2)?;
    Ok(outEqual)
}

fn getIteratorIndexedCrefs<'__b>(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inIterator: &'__b ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)> = inCrefs.clone();
    let mut crefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = (match &*inCref {
        Absyn::ComponentRef::CREF_IDENT {
            name: id,
            subscripts: subs,
        } => {
            let mut idx: i32;
            let mut name: ArcStr;
            idx = 1;
            for mut sub in &*subs.clone() {
                let () = (::match_deref::match_deref! { match &(sub.clone()) {
                    Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, subscripts: Deref @ metamodelica::ListNode::Nil } } } => {
                        name = (*__esc_name).clone();
                        if metamodelica::stringEq(&name, &inIterator) {
                            outCrefs = metamodelica::cons((metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() }), idx), outCrefs);
                        }
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                idx = idx + 1;
            }
            outCrefs
        }
        Absyn::ComponentRef::CREF_QUAL {
            name: id,
            subscripts: subs,
            componentRef: cref,
        } => {
            let mut idx: i32;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), inIterator, metamodelica::nil());
            for mut cr in &*crefs {
                (cref, idx) = cr.clone();
                outCrefs = metamodelica::cons(
                    (
                        metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                            name: id.clone(),
                            subscripts: subs.clone(),
                            componentRef: cref.clone(),
                        }),
                        idx,
                    ),
                    outCrefs,
                );
            }
            getIteratorIndexedCrefs(
                metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: id.clone(),
                    subscripts: subs.clone(),
                }),
                inIterator,
                outCrefs,
            )
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut idx: i32;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), inIterator, metamodelica::nil());
            for mut cr in &*crefs {
                (cref, idx) = cr.clone();
                outCrefs = metamodelica::cons(
                    (
                        metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
                            componentRef: cref.clone(),
                        }),
                        idx,
                    ),
                    outCrefs,
                );
            }
            outCrefs
        }
        _ => inCrefs,
    });
    outCrefs
}

pub(crate) fn getFileNameFromInfo(mut inInfo: SourceInfo) -> Result<ArcStr> {
    let mut inFileName: ArcStr;
    let SourceInfo { fileName: __pa0, .. } = (inInfo) else {
        return Err("pattern mismatch");
    };
    inFileName = metamodelica::Own::own(__pa0);
    Ok(inFileName)
}

pub fn isOuter(mut io: Absyn::InnerOuter) -> bool {
    let mut isItAnOuter: bool;
    isItAnOuter = (match io {
        Absyn::InnerOuter::INNER_OUTER { .. } => true,
        Absyn::InnerOuter::OUTER { .. } => true,
        _ => false,
    });
    isItAnOuter
}

pub fn isInner(mut io: Absyn::InnerOuter) -> bool {
    let mut isItAnInner: bool;
    isItAnInner = (match io {
        Absyn::InnerOuter::INNER_OUTER { .. } => true,
        Absyn::InnerOuter::INNER { .. } => true,
        _ => false,
    });
    isItAnInner
}

pub fn isOnlyInner(mut inIO: Absyn::InnerOuter) -> bool {
    let mut outOnlyInner: bool;
    outOnlyInner = (match inIO {
        Absyn::InnerOuter::INNER { .. } => true,
        _ => false,
    });
    outOnlyInner
}

pub fn isOnlyOuter(mut inIO: Absyn::InnerOuter) -> bool {
    let mut outOnlyOuter: bool;
    outOnlyOuter = (match inIO {
        Absyn::InnerOuter::OUTER { .. } => true,
        _ => false,
    });
    outOnlyOuter
}

pub fn isInnerOuter(mut inIO: Absyn::InnerOuter) -> bool {
    let mut outIsInnerOuter: bool;
    outIsInnerOuter = (match inIO {
        Absyn::InnerOuter::INNER_OUTER { .. } => true,
        _ => false,
    });
    outIsInnerOuter
}

pub fn isNotInnerOuter(mut inIO: Absyn::InnerOuter) -> bool {
    let mut outIsNotInnerOuter: bool;
    outIsNotInnerOuter = (match inIO {
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => true,
        _ => false,
    });
    outIsNotInnerOuter
}

pub(crate) fn innerOuterEqual(mut io1: Absyn::InnerOuter, mut io2: Absyn::InnerOuter) -> bool {
    let mut res: bool;
    res = (match (io1, io2) {
        (Absyn::InnerOuter::INNER { .. }, Absyn::InnerOuter::INNER { .. }) => true,
        (Absyn::InnerOuter::OUTER { .. }, Absyn::InnerOuter::OUTER { .. }) => true,
        (Absyn::InnerOuter::INNER_OUTER { .. }, Absyn::InnerOuter::INNER_OUTER { .. }) => true,
        (Absyn::InnerOuter::NOT_INNER_OUTER { .. }, Absyn::InnerOuter::NOT_INNER_OUTER { .. }) => true,
        _ => false,
    });
    res
}

pub fn makeFullyQualified(mut inPath: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &*inPath {
        Absyn::Path::FULLYQUALIFIED { .. } => inPath,
        _ => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: inPath }),
    });
    outPath
}

pub fn makeNotFullyQualified(mut inPath: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &*inPath {
        Absyn::Path::FULLYQUALIFIED { path } => path.clone(),
        _ => inPath,
    });
    outPath
}

pub(crate) fn importEqual(mut im1: &Absyn::Import, mut im2: &Absyn::Import) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match (im1.clone(), im2.clone()) {
        (Absyn::Import::NAMED_IMPORT { .. }, Absyn::Import::NAMED_IMPORT { .. }) => {
            stringEq(
                &var_field!(im1.name, Absyn::Import::NAMED_IMPORT),
                &var_field!(im2.name, Absyn::Import::NAMED_IMPORT),
            ) && pathEqual(
                var_field!(im1.path, Absyn::Import::NAMED_IMPORT),
                var_field!(im2.path, Absyn::Import::NAMED_IMPORT),
            )
        }
        (Absyn::Import::QUAL_IMPORT { .. }, Absyn::Import::QUAL_IMPORT { .. }) => pathEqual(
            var_field!(im1.path, Absyn::Import::QUAL_IMPORT),
            var_field!(im2.path, Absyn::Import::QUAL_IMPORT),
        ),
        (Absyn::Import::UNQUAL_IMPORT { .. }, Absyn::Import::UNQUAL_IMPORT { .. }) => pathEqual(
            var_field!(im1.path, Absyn::Import::UNQUAL_IMPORT),
            var_field!(im2.path, Absyn::Import::UNQUAL_IMPORT),
        ),
        _ => false,
    });
    outBoolean
}

pub fn canonIfExp(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match inExp {
        Deref @ Absyn::Exp::IFEXP { elseIfBranch: Deref @ metamodelica::ListNode::Nil, .. } => {
            inExp.clone()
        },
        Deref @ Absyn::Exp::IFEXP { ifExp: cond, trueBranch: tb, elseBranch: eb, elseIfBranch: Deref @ metamodelica::ListNode::Cons { head: (ei_cond, ei_tb), tail: eib } } => {
            let mut e: metamodelica::Ref<Absyn::Exp>;
            e = canonIfExp(&(metamodelica::Ref::new(Absyn::Exp::IFEXP { ifExp: ei_cond.clone(), trueBranch: ei_tb.clone(), elseBranch: eb.clone(), elseIfBranch: eib.clone() })))?;
            metamodelica::Ref::new(Absyn::Exp::IFEXP { ifExp: cond.clone(), trueBranch: tb.clone(), elseBranch: e, elseIfBranch: metamodelica::nil() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn onlyLiteralsInAnnotationMod(mut inMod: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>) -> bool {
    let mut onlyLiterals: bool;
    onlyLiterals = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "interaction" }, .. }, tail: rest } => {
                    Ok(onlyLiteralsInAnnotationMod(metamodelica::AsArg::as_arg(&rest)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: dive, eqMod }), .. }, tail: rest } => {
                    Ok(onlyLiteralsInEqMod(metamodelica::AsArg::as_arg(&eqMod))? && onlyLiteralsInAnnotationMod(metamodelica::AsArg::as_arg(&dive)) && onlyLiteralsInAnnotationMod(metamodelica::AsArg::as_arg(&rest)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(onlyLiteralsInAnnotationMod(metamodelica::AsArg::as_arg(&rest)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    onlyLiterals
}

pub fn onlyLiteralsInEqMod(mut eqMod: &metamodelica::Ref<Absyn::EqMod>) -> Result<bool> {
    let mut onlyLiterals: bool;
    onlyLiterals = (match &**eqMod {
        Absyn::EqMod::NOMOD { .. } => true,
        Absyn::EqMod::EQMOD { exp: __eqMod_exp, .. } => onlyLiteralsInExp(__eqMod_exp.clone())?,
    });
    Ok(onlyLiterals)
}

pub(crate) fn onlyLiteralsInExp(mut exp: metamodelica::Ref<Absyn::Exp>) -> Result<bool> {
    let mut onlyLiterals: bool;
    let mut lst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &(traverseExpBidir(exp, (std::sync::Arc::new(fnptr!(onlyLiteralsInExpEnter, metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>)> + 'static>), (std::sync::Arc::new(fnptr!(onlyLiteralsInExpExit, metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>)> + 'static>), metamodelica::cons(metamodelica::nil(), metamodelica::nil()))?) {
        (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    lst = metamodelica::Own::own(__pa0);
    onlyLiterals = (lst).is_empty();
    Ok(onlyLiterals)
}

fn onlyLiteralsInExpEnter(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    (outExp, outLst) = (::match_deref::match_deref! { match &((inExp.clone(), inLst.clone())) {
        (e @ Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { name, .. } }, Deref @ metamodelica::ListNode::Cons { head: lst, tail: rest }) => {
            let mut b: bool;
            let mut lst = (*lst).clone();
            b = listMember(name.clone(), list![literal!("LinePattern"), literal!("Arrow"), literal!("FillPattern"), literal!("BorderPattern"), literal!("TextStyle"), literal!("Smooth"), literal!("TextAlignment")]);
            lst = List::consOnTrue(!(b), e.clone(), lst.clone());
            (inExp, metamodelica::cons(lst.clone(), rest.clone()))
        },
        (Deref @ Absyn::Exp::CREF { .. }, Deref @ metamodelica::ListNode::Cons { head: lst, tail: rest }) => {
            (inExp.clone(), metamodelica::cons(metamodelica::cons(inExp, lst.clone()), rest.clone()))
        },
        _ => {
            (inExp, inLst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outLst)
}

fn onlyLiteralsInExpExit(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    (outExp, outLst) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "DynamicSelect", .. }, .. } => {
            let mut lst = inLst.clone();
            (inExp, lst)
        },
        _ => {
            (inExp, inLst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outLst)
}

pub fn makeCons(
    mut e1: metamodelica::Ref<Absyn::Exp>,
    mut e2: metamodelica::Ref<Absyn::Exp>,
) -> metamodelica::Ref<Absyn::Exp> {
    let mut e: metamodelica::Ref<Absyn::Exp>;
    e = metamodelica::Ref::new(Absyn::Exp::CONS { head: e1, rest: e2 });
    e
}

pub fn crefIdent(mut cr: &metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*cr)) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, subscripts: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    Ok(r#str)
}

pub fn unqotePathIdents(mut inPath: &metamodelica::Ref<Absyn::Path>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = stringListPath(List::map(
        pathToStringList(inPath),
        &fnptr!(System::unquoteIdentifier, ArcStr),
    )?)?;
    Ok(path)
}

pub fn unqualifyCref(mut inCref: metamodelica::Ref<Absyn::ComponentRef>) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (match &*inCref {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __inCref_componentRef,
        } => __inCref_componentRef.clone(),
        _ => inCref,
    });
    outCref
}

pub fn pathIsFullyQualified(mut inPath: &metamodelica::Ref<Absyn::Path>) -> bool {
    let mut outIsQualified: bool;
    outIsQualified = (match &**inPath {
        Absyn::Path::FULLYQUALIFIED { .. } => true,
        _ => false,
    });
    outIsQualified
}

pub fn pathIsIdent(mut inPath: &metamodelica::Ref<Absyn::Path>) -> bool {
    let mut outIsIdent: bool;
    outIsIdent = (match &**inPath {
        Absyn::Path::IDENT { .. } => true,
        _ => false,
    });
    outIsIdent
}

pub fn pathIsQual(mut inPath: &metamodelica::Ref<Absyn::Path>) -> bool {
    let mut outIsQual: bool;
    outIsQual = (match &**inPath {
        Absyn::Path::QUALIFIED { .. } => true,
        _ => false,
    });
    outIsQual
}

pub fn withinEqual(mut within1: &Absyn::Within, mut within2: &Absyn::Within) -> bool {
    let mut b: bool;
    b = (match (within1.clone(), within2.clone()) {
        (Absyn::Within::TOP { .. }, Absyn::Within::TOP { .. }) => true,
        (Absyn::Within::WITHIN { .. }, Absyn::Within::WITHIN { .. }) => pathEqual(
            var_field!(within1.path, Absyn::Within::WITHIN),
            var_field!(within2.path, Absyn::Within::WITHIN),
        ),
        _ => false,
    });
    b
}

pub fn withinEqualCaseInsensitive(mut within1: &Absyn::Within, mut within2: &Absyn::Within) -> bool {
    let mut b: bool;
    b = (match (within1.clone(), within2.clone()) {
        (Absyn::Within::TOP { .. }, Absyn::Within::TOP { .. }) => true,
        (Absyn::Within::WITHIN { .. }, Absyn::Within::WITHIN { .. }) => pathEqualCaseInsensitive(
            var_field!(within1.path, Absyn::Within::WITHIN),
            var_field!(within2.path, Absyn::Within::WITHIN),
        ),
        _ => false,
    });
    b
}

pub fn withinString(mut w1: &Absyn::Within) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match w1.clone() {
        Absyn::Within::TOP { .. } => literal!("within ;"),
        Absyn::Within::WITHIN { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("within "));
            __mm_s.push_str(&*pathString(
                var_field!(w1.path, Absyn::Within::WITHIN).clone(),
                literal!("."),
                true,
                false,
            )?);
            __mm_s.push_str(&*literal!(";"));
            ArcStr::from(__mm_s)
        }
    });
    Ok(r#str)
}

pub fn joinWithinPath(
    mut within_: &Absyn::Within,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match within_.clone() {
        Absyn::Within::TOP { .. } => path,
        Absyn::Within::WITHIN { .. } => joinPaths(var_field!(within_.path, Absyn::Within::WITHIN).clone(), path)?,
    });
    Ok(outPath)
}

pub fn innerOuterStr(mut io: Absyn::InnerOuter) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match io {
        Absyn::InnerOuter::INNER_OUTER { .. } => literal!("inner outer"),
        Absyn::InnerOuter::INNER { .. } => literal!("inner"),
        Absyn::InnerOuter::OUTER { .. } => literal!("outer"),
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => literal!(""),
    });
    r#str
}

pub(crate) fn subscriptExpOpt(
    mut inSub: &metamodelica::Ref<Absyn::Subscript>,
) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut outExpOpt: Option<metamodelica::Ref<Absyn::Exp>>;
    outExpOpt = (match &**inSub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __inSub_subscript,
        } => Some(__inSub_subscript.clone()),
        Absyn::Subscript::NOSUB { .. } => None,
    });
    outExpOpt
}

pub fn crefInsertSubscriptLstLst(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>;
    (outExp, outLst) = 'mc: {
        let __mc_input = (&*inExp, inLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cref }, subs) => {
                    let mut cref2: metamodelica::Ref<Absyn::ComponentRef>;
                    cref2 = crefInsertSubscriptLstLst2(cref.clone(), subs.clone())?;
                    Ok((metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cref2.clone() }), subs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outLst)
}

pub(crate) fn crefInsertSubscriptLstLst2(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inSubs: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (inCref, inSubs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_IDENT { name: n, .. }, Deref @ metamodelica::ListNode::Cons { head: s, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: n.clone(), subscripts: s.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_QUAL { name: n, componentRef: cref, .. }, Deref @ metamodelica::ListNode::Cons { head: s, tail: subs }) => {
                    let mut cref2: metamodelica::Ref<Absyn::ComponentRef>;
                    cref2 = crefInsertSubscriptLstLst2(cref.clone(), subs.clone())?;
                    Ok(metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: n.clone(), subscripts: s.clone(), componentRef: cref2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref }, subs) => {
                    let mut cref2: metamodelica::Ref<Absyn::ComponentRef>;
                    cref2 = crefInsertSubscriptLstLst2(cref.clone(), subs.clone())?;
                    Ok(crefMakeFullyQualified(cref2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCref)
}

pub fn isCref(mut exp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (match &**exp {
        Absyn::Exp::CREF { .. } => true,
        _ => false,
    });
    b
}

pub fn isTuple(mut exp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (match &**exp {
        Absyn::Exp::TUPLE { expressions: _ } => true,
        _ => false,
    });
    b
}

pub(crate) fn allFieldsAreCrefs(mut expLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>) -> Result<bool> {
    let mut b: bool;
    b = List::all(expLst, &move |__a0: metamodelica::Ref<Absyn::Exp>| complexIsCref(&__a0))?;
    Ok(b)
}

pub fn complexIsCref(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> Result<bool> {
    let mut b: bool;
    b = (match &**inExp {
        Absyn::Exp::TUPLE {
            expressions: __inExp_expressions,
        } => allFieldsAreCrefs(metamodelica::AsArg::as_arg(&__inExp_expressions))?,
        Absyn::Exp::CONS {
            head: __inExp_head,
            rest: __inExp_rest,
        } => {
            complexIsCref(metamodelica::AsArg::as_arg(&__inExp_head))?
                && complexIsCref(metamodelica::AsArg::as_arg(&__inExp_rest))?
        }
        _ => isCref(inExp),
    });
    Ok(b)
}

pub(crate) fn isDerCref(mut exp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "der", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDerCrefFail(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<()> {
    ::match_deref::match_deref! { match &((*exp)) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "der", subscripts: Deref @ metamodelica::ListNode::Nil }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. } => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn getExpsFromArrayDim(
    mut inAd: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<(bool, metamodelica::List<metamodelica::Ref<Absyn::Exp>>)> {
    let mut hasUnknownDimensions: bool;
    let mut outExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    (hasUnknownDimensions, outExps) = getExpsFromArrayDim_tail(inAd, metamodelica::nil())?;
    Ok((hasUnknownDimensions, outExps))
}

pub fn getExpsFromArrayDimOpt(
    mut inAdO: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> Result<(bool, metamodelica::List<metamodelica::Ref<Absyn::Exp>>)> {
    let mut hasUnknownDimensions: bool;
    let mut outExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    (hasUnknownDimensions, outExps) = (::match_deref::match_deref! { match &(inAdO) {
        None => {
            (false, metamodelica::nil())
        },
        Some(ad) => {
            getExpsFromArrayDim_tail(ad.clone(), metamodelica::nil())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((hasUnknownDimensions, outExps))
}

pub(crate) fn getExpsFromArrayDim_tail(
    mut inAd: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inAccumulator: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<(bool, metamodelica::List<metamodelica::Ref<Absyn::Exp>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAd, inAccumulator)) {
            (Deref @ metamodelica::ListNode::Nil, acc) => {
                return Ok((false, acc.clone().reverse()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: rest }, acc) => {
                let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                let mut b: bool;
                { (inAd, inAccumulator) = (rest.clone(), metamodelica::cons(e.clone(), acc.clone())); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: rest }, acc) => {
                let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                (_, exps) = getExpsFromArrayDim_tail(rest.clone(), acc.clone())?;
                return Ok((true, exps))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isInputOrOutput(mut direction: Absyn::Direction) -> bool {
    let mut isIorO: bool;
    isIorO = (match direction {
        Absyn::Direction::INPUT { .. } => true,
        Absyn::Direction::OUTPUT { .. } => true,
        Absyn::Direction::INPUT_OUTPUT { .. } => true,
        Absyn::Direction::BIDIR { .. } => false,
    });
    isIorO
}

pub fn isInput(mut inDirection: Absyn::Direction) -> bool {
    let mut outIsInput: bool;
    outIsInput = (match inDirection {
        Absyn::Direction::INPUT { .. } => true,
        Absyn::Direction::INPUT_OUTPUT { .. } => true,
        _ => false,
    });
    outIsInput
}

pub fn isOutput(mut inDirection: Absyn::Direction) -> bool {
    let mut outIsOutput: bool;
    outIsOutput = (match inDirection {
        Absyn::Direction::OUTPUT { .. } => true,
        Absyn::Direction::INPUT_OUTPUT { .. } => true,
        _ => false,
    });
    outIsOutput
}

pub fn directionEqual(mut inDirection1: Absyn::Direction, mut inDirection2: Absyn::Direction) -> bool {
    let mut outEqual: bool;
    outEqual = (match (inDirection1, inDirection2) {
        (Absyn::Direction::BIDIR { .. }, Absyn::Direction::BIDIR { .. }) => true,
        (Absyn::Direction::INPUT { .. }, Absyn::Direction::INPUT { .. }) => true,
        (Absyn::Direction::OUTPUT { .. }, Absyn::Direction::OUTPUT { .. }) => true,
        (Absyn::Direction::INPUT_OUTPUT { .. }, Absyn::Direction::INPUT_OUTPUT { .. }) => true,
        _ => false,
    });
    outEqual
}

pub(crate) fn isFieldEqual(mut isField1: Absyn::IsField, mut isField2: Absyn::IsField) -> bool {
    let mut outEqual: bool;
    outEqual = (match (isField1, isField2) {
        (Absyn::IsField::NONFIELD { .. }, Absyn::IsField::NONFIELD { .. }) => true,
        (Absyn::IsField::FIELD { .. }, Absyn::IsField::FIELD { .. }) => true,
        _ => false,
    });
    outEqual
}

pub(crate) fn pathLt(
    mut path1: metamodelica::Ref<Absyn::Path>,
    mut path2: metamodelica::Ref<Absyn::Path>,
) -> Result<bool> {
    let mut lt: bool;
    lt = stringCompare(
        &(pathString(path1, literal!("."), true, false)?),
        &(pathString(path2, literal!("."), true, false)?),
    ) < 0;
    Ok(lt)
}

pub fn pathGe(mut path1: metamodelica::Ref<Absyn::Path>, mut path2: metamodelica::Ref<Absyn::Path>) -> Result<bool> {
    let mut ge: bool;
    ge = !(pathLt(path1, path2)?);
    Ok(ge)
}

pub fn getShortClass(mut cl: metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cl: metamodelica::Ref<Absyn::Class> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { .. }, .. } => return Err("fail"),
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => return Err("fail"),
        Deref @ Absyn::Class { .. } => {
            assign_field!(cl.body = stripClassDefComment(cl.body.clone()));
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cl)
}

fn stripClassDefComment(mut cl: metamodelica::Ref<Absyn::ClassDef>) -> metamodelica::Ref<Absyn::ClassDef> {
    let mut cl: metamodelica::Ref<Absyn::ClassDef> = cl;
    let () = (match &*cl {
        Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::PARTS; comment = None);
            ()
        }
        Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::DERIVED; comment = None);
            ()
        }
        Absyn::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::ENUMERATION; comment = None);
            ()
        }
        Absyn::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::OVERLOAD; comment = None);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::CLASS_EXTENDS; comment = None);
            ()
        }
        Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::PDER; comment = None);
            ()
        }
        _ => (),
    });
    cl
}

pub fn getFunctionInterface(mut cl: metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cl: metamodelica::Ref<Absyn::Class> = cl;
    let mut def: metamodelica::Ref<Absyn::ClassDef>;
    let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { restriction: Absyn::Restriction::R_FUNCTION { .. }, body: __esc_def @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            def = (*__esc_def).clone();
            let __pa0 = ::match_deref::match_deref! { match &(List::fold(&(var_field!((*def).classParts, Absyn::ClassDef::PARTS).clone().reverse()), &move |__a0: metamodelica::Ref<Absyn::ClassPart>, __a1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>| getFunctionInterfaceParts(&__a0, __a1), metamodelica::nil())?) {
                __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elts = metamodelica::Own::own(__pa0);
            assign_field!(
                cl.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: var_field!((*def).typeVars, Absyn::ClassDef::PARTS).clone(), classAttrs: var_field!((*def).classAttrs, Absyn::ClassDef::PARTS).clone(), classParts: list![metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elts })], ann: metamodelica::nil(), comment: None }),
                cl.commentsBeforeEnd = metamodelica::nil(),
                cl.commentsAfterEnd = metamodelica::nil()
            );
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cl)
}

fn getFunctionInterfaceParts(
    mut part: &metamodelica::Ref<Absyn::ClassPart>,
    mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut oelts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    oelts = (match &**part {
        Absyn::ClassPart::PUBLIC { contents: elts1 } => {
            let mut elts2 = elts.clone();
            let mut elts1 = (*elts1).clone();
            elts1 = List::filterOnTrue(
                elts1.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Absyn::ElementItem>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(filterAnnotationItem(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementItem>) -> Result<bool> + 'static,
                    >),
            )?;
            listAppend(elts1.clone(), elts2)
        }
        _ => elts,
    });
    Ok(oelts)
}

fn filterAnnotationItem(mut elt: &metamodelica::Ref<Absyn::ElementItem>) -> bool {
    let mut outB: bool;
    outB = (match &**elt {
        Absyn::ElementItem::ELEMENTITEM { .. } => true,
        _ => false,
    });
    outB
}

pub fn filterNestedClasses(mut cl: metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cl: metamodelica::Ref<Absyn::Class> = cl;
    let mut def: metamodelica::Ref<Absyn::ClassDef>;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { body: __esc_def @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            def = (*__esc_def).clone();
            assign_variant_field!(def => Absyn::ClassDef::PARTS; classParts = List::fold(&(var_field!((*def).classParts, Absyn::ClassDef::PARTS).clone().reverse()), &filterNestedClassesParts, metamodelica::nil())?);
            assign_field!(cl.body = def.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cl)
}

fn filterNestedClassesParts(
    mut classPart: metamodelica::Ref<Absyn::ClassPart>,
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassPart: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outClassPart = (match &*classPart {
        Absyn::ClassPart::PUBLIC { contents: elts } => {
            let mut classParts = inClassParts.clone();
            assign_variant_field!(classPart => Absyn::ClassPart::PUBLIC; contents = List::filterOnFalse(elts.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementItem>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isElementItemClass(&__a0)) })?);
            metamodelica::cons(classPart, classParts)
        }
        Absyn::ClassPart::PROTECTED { contents: elts } => {
            let mut classParts = inClassParts.clone();
            assign_variant_field!(classPart => Absyn::ClassPart::PROTECTED; contents = List::filterOnFalse(elts.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementItem>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isElementItemClass(&__a0)) })?);
            metamodelica::cons(classPart, classParts)
        }
        _ => metamodelica::cons(classPart, inClassParts),
    });
    Ok(outClassPart)
}

pub fn getExternalDecl(mut inCls: &metamodelica::Ref<Absyn::Class>) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut outExternal: metamodelica::Ref<Absyn::ClassPart>;
    let mut class_parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inCls)) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    class_parts = metamodelica::Own::own(__pa0);
    outExternal = List::find(
        &class_parts,
        &move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isExternalPart(&__a0))
        },
    )?;
    Ok(outExternal)
}

pub fn isExternalPart(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> bool {
    let mut outFound: bool;
    outFound = (match &**inClassPart {
        Absyn::ClassPart::EXTERNAL { .. } => true,
        _ => false,
    });
    outFound
}

pub fn isParts(mut cl: &metamodelica::Ref<Absyn::ClassDef>) -> bool {
    let mut b: bool;
    b = (match &**cl {
        Absyn::ClassDef::PARTS { .. } => true,
        _ => false,
    });
    b
}

pub fn makeClassElement(mut cl: metamodelica::Ref<Absyn::Class>) -> metamodelica::Ref<Absyn::ElementItem> {
    let mut el: metamodelica::Ref<Absyn::ElementItem>;
    let mut info: SourceInfo;
    let mut fp: bool;
    let __arc2 = cl.clone();
    let Absyn::CLASS {
        finalPrefix: __pa0,
        info: __pa1,
        ..
    } = &*__arc2;
    fp = metamodelica::Own::own(__pa0);
    info = metamodelica::Own::own(__pa1);
    el = metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM {
        element: metamodelica::Ref::new(Absyn::Element::ELEMENT {
            finalPrefix: fp,
            redeclareKeywords: None,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                replaceable_: false,
                class_: cl,
            }),
            info: info,
            constrainClass: None,
        }),
    });
    el
}

pub fn componentName(mut c: &metamodelica::Ref<Absyn::ComponentItem>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __arc1 = &(*c);
    let Absyn::COMPONENTITEM {
        component: Absyn::COMPONENT { name: __pa0, .. },
        ..
    } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    Ok(name)
}

pub fn expContainsInitial(mut inExp: metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut hasInitial: bool;
    hasInitial = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool;
                    (_, b) = traverseExp(inExp.clone(), (std::sync::Arc::new(fnptr!(isInitialTraverseHelper, metamodelica::Ref<Absyn::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, bool) -> Result<(metamodelica::Ref<Absyn::Exp>, bool)> + 'static>), false)?;
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    hasInitial
}

fn isInitialTraverseHelper(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBool: bool,
) -> (metamodelica::Ref<Absyn::Exp>, bool) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outBool: bool;
    (outExp, outBool) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::NOT { .. }, exp: _ } => {
            (inExp, inBool)
        },
        e => {
            let mut b: bool;
            b = isInitial(metamodelica::AsArg::as_arg(&e));
            (e.clone(), b)
        },
        _ => {
            (inExp, inBool)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outBool)
}

pub fn isInitial(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match inExp {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initial", subscripts: _ }, .. } => true,
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initial", subscripts: _ } }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

pub fn importPath(mut inImport: &Absyn::Import) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { path: mut path, .. } => path.clone(),
        Absyn::Import::QUAL_IMPORT { path: mut path } => path.clone(),
        Absyn::Import::UNQUAL_IMPORT { path: mut path } => path.clone(),
        Absyn::Import::GROUP_IMPORT { prefix: ref path, .. } => path.clone(),
    });
    outPath
}

pub fn setImportPath(mut imp: Absyn::Import, mut path: metamodelica::Ref<Absyn::Path>) -> Absyn::Import {
    let mut imp: Absyn::Import = imp;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            let __owned_variant_path_0 = path;
            if let Absyn::Import::NAMED_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::NAMED_IMPORT");
            }
            ()
        }
        Absyn::Import::QUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = path;
            if let Absyn::Import::QUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::QUAL_IMPORT");
            }
            ()
        }
        Absyn::Import::UNQUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = path;
            if let Absyn::Import::UNQUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::UNQUAL_IMPORT");
            }
            ()
        }
        Absyn::Import::GROUP_IMPORT { .. } => {
            let __owned_variant_prefix_0 = path;
            if let Absyn::Import::GROUP_IMPORT { prefix, .. } = &mut imp {
                *prefix = __owned_variant_prefix_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::GROUP_IMPORT");
            }
            ()
        }
    });
    imp
}

pub fn importName(mut inImport: &Absyn::Import) -> Result<ArcStr> {
    let mut outName: ArcStr;
    outName = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => var_field!(inImport.name, Absyn::Import::NAMED_IMPORT).clone(),
        Absyn::Import::QUAL_IMPORT { .. } => pathLastIdent(var_field!(inImport.path, Absyn::Import::QUAL_IMPORT)),
        _ => return Err("match: no arm matched"),
    });
    Ok(outName)
}

pub fn mergeAnnotationsList(
    mut oldAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut newAnnotations: &metamodelica::List<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut outAnnotation: metamodelica::Ref<Absyn::Annotation> = oldAnnotation;
    for mut ann in &**newAnnotations {
        outAnnotation = mergeAnnotations(ann.clone(), outAnnotation, false, false)?;
    }
    Ok(outAnnotation)
}

pub fn mergeAnnotations(
    mut oldAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut newAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut mergeSubMods: bool,
    mut mergeEqMods: bool,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut outAnnotation: metamodelica::Ref<Absyn::Annotation>;
    outAnnotation = (::match_deref::match_deref! { match &((oldAnnotation.clone(), newAnnotation.clone())) {
        (Deref @ Absyn::Annotation { elementArgs: Deref @ metamodelica::ListNode::Nil }, _) => newAnnotation,
        (_, Deref @ Absyn::Annotation { elementArgs: Deref @ metamodelica::ListNode::Nil }) => oldAnnotation,
        _ => metamodelica::Ref::new(Absyn::Annotation { elementArgs: mergeAnnotations2(oldAnnotation.elementArgs.clone(), &newAnnotation.elementArgs, mergeSubMods, mergeEqMods)? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

fn mergeAnnotations2(
    mut oldArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut newArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut mergeSubMods: bool,
    mut mergeEqMods: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = oldArgs;
    let mut found: bool;
    let mut new_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    for mut arg in &**newArgs {
        (outArgs, found) = List::findAndMap(
            outArgs,
            &({
                let __pe_b1 = elementArgName(metamodelica::AsArg::as_arg(&arg))?;
                move |__pe_a0| Ok(isModificationOfPath(&__pe_a0, &__pe_b1))
            }),
            &*(if (mergeSubMods) {
                (std::sync::Arc::new({
                    let __pe_b1 = arg.clone();
                    let __pe_b2 = mergeEqMods;
                    move |__pe_a0| mergeAnnotations3(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::ElementArg>,
                            ) -> Result<metamodelica::Ref<Absyn::ElementArg>>
                            + 'static,
                    >)
            } else {
                (std::sync::Arc::new({
                    let __pe_b1 = arg.clone();
                    move |__pe_a0| subModsInSameOrder(&__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::ElementArg>,
                            ) -> Result<metamodelica::Ref<Absyn::ElementArg>>
                            + 'static,
                    >)
            }),
        )?;
        if !(found) {
            new_args = metamodelica::cons(arg.clone(), new_args);
        }
    }
    outArgs = listAppend(outArgs, metamodelica::Dangerous::listReverseInPlace(new_args));
    Ok(outArgs)
}

fn mergeAnnotations3(
    mut oldArg: metamodelica::Ref<Absyn::ElementArg>,
    mut newArg: metamodelica::Ref<Absyn::ElementArg>,
    mut mergeEqMods: bool,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outArg: metamodelica::Ref<Absyn::ElementArg>;
    let mut old_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut new_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut old_eq: metamodelica::Ref<Absyn::EqMod>;
    let mut new_eq: metamodelica::Ref<Absyn::EqMod>;
    let mut cmt: Option<ArcStr>;
    outArg = (::match_deref::match_deref! { match &((oldArg.clone(), newArg.clone())) {
        (Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. }, _) => newArg,
        (_, Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. }) => oldArg,
        (Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: __esc_old_args, eqMod: __esc_old_eq }), .. }, Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: __esc_new_args, eqMod: __esc_new_eq }), .. }) => {
            old_args = (*__esc_old_args).clone();
            old_eq = (*__esc_old_eq).clone();
            new_args = (*__esc_new_args).clone();
            new_eq = (*__esc_new_eq).clone();
            new_eq = mergeAnnotationEqMods(old_eq.clone(), new_eq.clone(), mergeEqMods);
            new_args = mergeAnnotations2(old_args.clone(), metamodelica::AsArg::as_arg(&new_args), true, mergeEqMods)?;
            cmt = if ((var_field!((*newArg).comment, Absyn::ElementArg::MODIFICATION)).is_some()) {var_field!((*newArg).comment, Absyn::ElementArg::MODIFICATION).clone()} else {var_field!((*oldArg).comment, Absyn::ElementArg::MODIFICATION).clone()};
            metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: var_field!((*oldArg).path, Absyn::ElementArg::MODIFICATION).clone(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: new_args.clone(), eqMod: new_eq.clone() })), comment: cmt, info: var_field!((*oldArg).info, Absyn::ElementArg::MODIFICATION).clone() })
        },
        _ => newArg,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

fn mergeAnnotationEqMods(
    mut oldEq: metamodelica::Ref<Absyn::EqMod>,
    mut newEq: metamodelica::Ref<Absyn::EqMod>,
    mut mergeExpressions: bool,
) -> metamodelica::Ref<Absyn::EqMod> {
    let mut outEq: metamodelica::Ref<Absyn::EqMod>;
    let mut new_exp: metamodelica::Ref<Absyn::Exp>;
    let mut old_exp: metamodelica::Ref<Absyn::Exp>;
    outEq = (::match_deref::match_deref! { match &((oldEq.clone(), newEq.clone())) {
        (Deref @ Absyn::EqMod::NOMOD { .. }, _) => newEq,
        (_, Deref @ Absyn::EqMod::NOMOD { .. }) => oldEq,
        (Deref @ Absyn::EqMod::EQMOD { exp: __esc_old_exp, .. }, Deref @ Absyn::EqMod::EQMOD { exp: __esc_new_exp, .. }) if (mergeExpressions) => {
            old_exp = (*__esc_old_exp).clone();
            new_exp = (*__esc_new_exp).clone();
            new_exp = (::match_deref::match_deref! { match &((old_exp.clone(), new_exp.clone())) {
        (Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CALL { .. }, tail: _ } }, Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CALL { .. }, tail: _ } }) => metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: listAppend(var_field!((*old_exp).arrayExp, Absyn::Exp::ARRAY).clone(), var_field!((*new_exp).arrayExp, Absyn::Exp::ARRAY).clone()) }),
        _ => new_exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: new_exp.clone(), info: var_field!((*newEq).info, Absyn::EqMod::EQMOD).clone() })
        },
        _ => newEq,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEq
}

pub fn mergeCommentAnnotation(
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
    mut inComment: Option<metamodelica::Ref<Absyn::Comment>>,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut outComment: Option<metamodelica::Ref<Absyn::Comment>>;
    outComment = (::match_deref::match_deref! { match &(inComment) {
        None => {
            Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(inAnnotation), comment: None }))
        },
        Some(Deref @ Absyn::Comment { annotation_: None, comment: cmt }) => {
            Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(inAnnotation), comment: cmt.clone() }))
        },
        Some(Deref @ Absyn::Comment { annotation_: Some(ann), comment: cmt }) => {
            Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(mergeAnnotations(ann.clone(), inAnnotation, false, false)?), comment: cmt.clone() }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComment)
}

pub fn mergeModifiers(
    mut outerMod: &metamodelica::Ref<Absyn::Modification>,
    mut innerMod: &metamodelica::Ref<Absyn::Modification>,
) -> Result<metamodelica::Ref<Absyn::Modification>> {
    let mut outMod: metamodelica::Ref<Absyn::Modification>;
    outMod = metamodelica::Ref::new(Absyn::Modification {
        elementArgLst: mergeAnnotations2(innerMod.elementArgLst.clone(), &outerMod.elementArgLst, false, false)?,
        eqMod: mergeEqMods(outerMod.eqMod.clone(), innerMod.eqMod.clone()),
    });
    Ok(outMod)
}

pub(crate) fn mergeEqMods(
    mut outerEqMod: metamodelica::Ref<Absyn::EqMod>,
    mut innerEqMod: metamodelica::Ref<Absyn::EqMod>,
) -> metamodelica::Ref<Absyn::EqMod> {
    let mut outEqMod: metamodelica::Ref<Absyn::EqMod>;
    outEqMod = (match &*outerEqMod {
        Absyn::EqMod::EQMOD { .. } => outerEqMod,
        _ => innerEqMod,
    });
    outEqMod
}

pub(crate) fn isModificationOfPath(
    mut r#mod: &metamodelica::Ref<Absyn::ElementArg>,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> bool {
    let mut yes: bool;
    yes = (::match_deref::match_deref! { match (r#mod, path) {
        (Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: id1 }, .. }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
            metamodelica::stringEq(&id1, &id2)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

pub(crate) fn subModsInSameOrder(
    mut oldmod: &metamodelica::Ref<Absyn::ElementArg>,
    mut newmod: metamodelica::Ref<Absyn::ElementArg>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut r#mod: metamodelica::Ref<Absyn::ElementArg>;
    r#mod = (::match_deref::match_deref! { match &((oldmod.clone(), newmod.clone())) {
        (_, Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. }) => {
            newmod
        },
        (Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. }, _) => {
            newmod
        },
        (Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args1, eqMod: _ }), .. }, arg2 @ Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args2, eqMod: eq2 }), .. }) => {
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut arg2 = (*arg2).clone();
            res = metamodelica::nil();
            for mut arg1 in &*args1.clone() {
                let __pa0 = ::match_deref::match_deref! { match &(arg1.clone()) {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                p = metamodelica::Own::own(__pa0);
                if List::any(metamodelica::AsArg::as_arg(&args2), &({ let __pe_b1 = p; move |__pe_a0| Ok(isModificationOfPath(&__pe_a0, &__pe_b1)) }))? {
                    res = metamodelica::cons(arg1.clone(), res);
                }
            }
            res = res.reverse();
            res = mergeAnnotations2(res, metamodelica::AsArg::as_arg(&args2), false, false)?;
            assign_variant_field!(arg2 => Absyn::ElementArg::MODIFICATION; modification = Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: res, eqMod: eq2.clone() })));
            arg2.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#mod)
}

pub fn annotationToElementArgs(
    mut ann: &metamodelica::Ref<Absyn::Annotation>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let __arc1 = &(*ann);
    let Absyn::ANNOTATION { elementArgs: __pa0 } = &**__arc1;
    args = metamodelica::Own::own(__pa0);
    args
}

pub(crate) fn pathToTypeSpec(mut inPath: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Absyn::TypeSpec> {
    let mut outTypeSpec: metamodelica::Ref<Absyn::TypeSpec>;
    outTypeSpec = metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
        path: inPath,
        arrayDim: None,
    });
    outTypeSpec
}

pub(crate) fn typeSpecString(mut inTs: metamodelica::Ref<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = Dump::unparseTypeSpec(inTs)?;
    Ok(outStr)
}

pub fn crefString(mut inCr: &metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = Dump::printComponentRefStr(inCr)?;
    Ok(outStr)
}

pub(crate) fn typeSpecStringNoQualNoDims(mut inTs: &metamodelica::Ref<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (match &**inTs {
        Absyn::TypeSpec::TPATH { path, .. } => {
            pathString(makeNotFullyQualified(path.clone()), literal!("."), true, false)?
        }
        Absyn::TypeSpec::TCOMPLEX {
            path,
            typeSpecs: typeSpecLst,
            ..
        } => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = pathString(makeNotFullyQualified(path.clone()), literal!("."), true, false)?;
            str2 = typeSpecStringNoQualNoDimsLst(typeSpecLst.clone())?;
            stringAppendList(list![str1, literal!("<"), str2, literal!(">")])
        }
    });
    Ok(outStr)
}

pub(crate) fn typeSpecStringNoQualNoDimsLst(
    mut inTypeSpecLst: metamodelica::List<metamodelica::Ref<Absyn::TypeSpec>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = List::toStringCustom(
        inTypeSpecLst,
        &move |__a0: metamodelica::Ref<Absyn::TypeSpec>| typeSpecStringNoQualNoDims(&__a0),
        literal!(""),
        literal!(""),
        literal!(", "),
        literal!(""),
        false,
        0,
    )?;
    Ok(outString)
}

pub(crate) fn crefStringIgnoreSubs(mut inCr: &metamodelica::Ref<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut p: metamodelica::Ref<Absyn::Path>;
    p = crefToPathIgnoreSubs(inCr)?;
    outStr = pathString(makeNotFullyQualified(p), literal!("."), true, false)?;
    Ok(outStr)
}

pub(crate) fn importString(mut inImp: Absyn::Import) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = Dump::unparseImportStr(inImp)?;
    Ok(outStr)
}

pub(crate) fn refString(mut inRef: &Absyn::Ref) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (match inRef.clone() {
        Absyn::Ref::RCR { .. } => crefString(var_field!(inRef.cr, Absyn::Ref::RCR))?,
        Absyn::Ref::RTS { .. } => typeSpecString(var_field!(inRef.ts, Absyn::Ref::RTS).clone())?,
        Absyn::Ref::RIM { .. } => importString(var_field!(inRef.im, Absyn::Ref::RIM).clone())?,
    });
    Ok(outStr)
}

pub(crate) fn refStringBrief(mut inRef: &Absyn::Ref) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (match inRef.clone() {
        Absyn::Ref::RCR { .. } => crefStringIgnoreSubs(var_field!(inRef.cr, Absyn::Ref::RCR))?,
        Absyn::Ref::RTS { .. } => typeSpecStringNoQualNoDims(var_field!(inRef.ts, Absyn::Ref::RTS))?,
        Absyn::Ref::RIM { .. } => importString(var_field!(inRef.im, Absyn::Ref::RIM).clone())?,
    });
    Ok(outStr)
}

pub(crate) fn getArrayDimOptAsList(
    mut inArrayDim: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outArrayDim: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outArrayDim = (::match_deref::match_deref! { match &(inArrayDim) {
        Some(ad) => {
            ad.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArrayDim
}

pub fn removeCrefFromCrefs(
    mut inAbsynComponentRefLst: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>> {
    let mut outAbsynComponentRefLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = (&**inAbsynComponentRefLst, inComponentRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr1, tail: rest }, cr2) => {
                    let mut n1: ArcStr;
                    let mut n2: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(cr1.clone()) {
                        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, subscripts: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    n1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(cr2.clone()) {
                        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa1, subscripts: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    n2 = metamodelica::Own::own(__pa1);
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    Ok(removeCrefFromCrefs(metamodelica::AsArg::as_arg(&rest), cr2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr1, tail: rest }, cr2) => {
                    let mut n1: ArcStr;
                    let mut n2: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(cr1.clone()) {
                        Deref @ Absyn::ComponentRef::CREF_QUAL { name: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    n1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(cr2.clone()) {
                        Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa1, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    n2 = metamodelica::Own::own(__pa1);
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    Ok(removeCrefFromCrefs(metamodelica::AsArg::as_arg(&rest), cr2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr1, tail: rest }, cr2) => {
                    let mut rest = (*rest).clone();
                    rest = removeCrefFromCrefs(metamodelica::AsArg::as_arg(&rest), cr2.clone())?;
                    Ok(metamodelica::cons(cr1.clone(), rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outAbsynComponentRefLst)
}

pub fn lookupClassAnnotation(
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>>;
    outMod = lookupClassDefAnnotation(&cls.body, name)?;
    Ok(outMod)
}

pub(crate) fn lookupClassDefAnnotation(
    mut cdef: &metamodelica::Ref<Absyn::ClassDef>,
    mut name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>> = None;
    let mut ann: metamodelica::Ref<Absyn::Annotation>;
    outMod = (match &**cdef {
        Absyn::ClassDef::PARTS { ann: __cdef_ann, .. } => List::findSome(
            metamodelica::AsArg::as_arg(&__cdef_ann),
            &({
                let __pe_b1 = name.clone();
                move |__pe_a0| Ok(lookupAnnotation(&__pe_a0, &__pe_b1))
            }),
        )?,
        Absyn::ClassDef::CLASS_EXTENDS { ann: __cdef_ann, .. } => List::findSome(
            metamodelica::AsArg::as_arg(&__cdef_ann),
            &({
                let __pe_b1 = name.clone();
                move |__pe_a0| Ok(lookupAnnotation(&__pe_a0, &__pe_b1))
            }),
        )?,
        Absyn::ClassDef::DERIVED {
            comment: __cdef_comment,
            ..
        } => lookupCommentOptAnnotation(__cdef_comment.clone(), name),
        Absyn::ClassDef::ENUMERATION {
            comment: __cdef_comment,
            ..
        } => lookupCommentOptAnnotation(__cdef_comment.clone(), name),
        Absyn::ClassDef::OVERLOAD {
            comment: __cdef_comment,
            ..
        } => lookupCommentOptAnnotation(__cdef_comment.clone(), name),
        Absyn::ClassDef::PDER {
            comment: __cdef_comment,
            ..
        } => lookupCommentOptAnnotation(__cdef_comment.clone(), name),
        _ => None,
    });
    Ok(outMod)
}

pub(crate) fn lookupCommentOptAnnotation(
    mut cmt: Option<metamodelica::Ref<Absyn::Comment>>,
    mut name: &ArcStr,
) -> Option<metamodelica::Ref<Absyn::Modification>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut ann: metamodelica::Ref<Absyn::Annotation>;
    outMod = (::match_deref::match_deref! { match &(cmt) {
        Some(Deref @ Absyn::Comment { annotation_: Some(__esc_ann), .. }) => {
            ann = (*__esc_ann).clone();
            lookupAnnotation(metamodelica::AsArg::as_arg(&ann), name)
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub(crate) fn lookupAnnotation(
    mut ann: &metamodelica::Ref<Absyn::Annotation>,
    mut name: &ArcStr,
) -> Option<metamodelica::Ref<Absyn::Modification>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>> = None;
    for mut m in &*ann.elementArgs.clone() {
        outMod = (match &*m.clone() {
            Absyn::ElementArg::MODIFICATION {
                modification: __m_modification,
                path: __m_path,
                ..
            } if (metamodelica::stringEq(&(pathFirstIdent(metamodelica::AsArg::as_arg(&__m_path))), &name)) => {
                __m_modification.clone()
            }
            _ => outMod,
        });
        if (outMod).is_some() {
            break;
        }
    }
    outMod
}

pub fn getNamedAnnotationInClass<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut id: &metamodelica::Ref<Absyn::Path>,
    mut f: &dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<T>,
) -> Option<T> {
    pub type ModFunc<T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<T> + 'static>;

    let mut outString: Option<T>;
    outString = 'mc: {
        let __mc_input = &**inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann, .. }, .. } => {
                    let mut annlst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    annlst = List::flatten(List::map(ann.clone(), &move |__a0: metamodelica::Ref<Absyn::Annotation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(annotationToElementArgs(&__a0)) })?)?;
                    Ok(getNamedAnnotationStr(&annlst, id, f)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann, .. }, .. } => {
                    let mut annlst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    annlst = List::flatten(List::map(ann.clone(), &move |__a0: metamodelica::Ref<Absyn::Annotation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(annotationToElementArgs(&__a0)) })?)?;
                    Ok(getNamedAnnotationStr(&annlst, id, f)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annlst }), comment: _ }), .. }, .. } => {
                    Ok(getNamedAnnotationStr(metamodelica::AsArg::as_arg(&annlst), id, f)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annlst }), comment: _ }), .. }, .. } => {
                    Ok(getNamedAnnotationStr(metamodelica::AsArg::as_arg(&annlst), id, f)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annlst }), comment: _ }), .. }, .. } => {
                    Ok(getNamedAnnotationStr(metamodelica::AsArg::as_arg(&annlst), id, f)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
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

fn getNamedAnnotationStr<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut id: &metamodelica::Ref<Absyn::Path>,
    mut f: &dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<T>,
) -> Result<Option<T>> {
    pub type ModFunc<T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<T> + 'static>;

    let mut outString: Option<T>;
    outString = 'mc: {
        let __mc_input = (&**inAbsynElementArgLst, &**id);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: id1 }, modification: r#mod, .. }, tail: _ }, Deref @ Absyn::Path::IDENT { name: id2 }) => {
                    let mut r#str: T;
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    r#str = f(r#mod.clone())?;
                    Ok(Some(r#str.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: id1 }, modification: Some(Deref @ Absyn::Modification { elementArgLst: xs, .. }), .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { name: id2, path: rest }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(getNamedAnnotationStr(metamodelica::AsArg::as_arg(&xs), metamodelica::AsArg::as_arg(&rest), f)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, _) => {
                    Ok(getNamedAnnotationStr(metamodelica::AsArg::as_arg(&xs), id, f)?)
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

pub fn transformAnnotationArg(
    mut ann: metamodelica::Ref<Absyn::Annotation>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
            + 'static,
    >,
    mut insert: bool,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
            + 'static,
    >;

    let mut ann: metamodelica::Ref<Absyn::Annotation> = ann;
    assign_field!(ann.elementArgs = transformAnnotationInArgs(ann.elementArgs.clone(), path, func.clone(), insert)?);
    Ok(ann)
}

pub fn transformAnnotationInArgs(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut r#fn: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
            + 'static,
    >,
    mut insert: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    pub type Fn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
            + 'static,
    >;

    fn is_named(mut arg: &metamodelica::Ref<Absyn::ElementArg>, mut name: &ArcStr) -> bool {
        let mut result: bool;
        let mut arg_name: ArcStr;
        result = (::match_deref::match_deref! { match arg {
            Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: __esc_arg_name }, .. } => {
                arg_name = (*__esc_arg_name).clone();
                metamodelica::stringEq(&name, &arg_name)
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        result
    }

    fn apply_fn(
        mut arg: metamodelica::Ref<Absyn::ElementArg>,
        mut path: metamodelica::Ref<Absyn::Path>,
        mut r#fn: Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
                + 'static,
        >,
        mut insert: bool,
    ) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
        let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
        let mut r#mod: metamodelica::Ref<Absyn::Modification>;
        if pathIsIdent(&path) {
            arg = r#fn(arg)?;
        } else {
            let () = (match &*arg {
                Absyn::ElementArg::MODIFICATION {
                    modification: __arg_modification,
                    ..
                } => {
                    if (__arg_modification).is_some() {
                        let __pa0 = ::match_deref::match_deref! { match &(__arg_modification.clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        r#mod = metamodelica::Own::own(__pa0);
                    } else if insert {
                        r#mod = metamodelica::Ref::new(Absyn::Modification {
                            elementArgLst: metamodelica::nil(),
                            eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
                        });
                    } else {
                        return Err("fail");
                    }
                    assign_field!(
                        r#mod.elementArgLst = transformAnnotationInArgs(
                            r#mod.elementArgLst.clone(),
                            pathRest(path)?,
                            r#fn.clone(),
                            insert
                        )?
                    );
                    assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod));
                    ()
                }
                _ => return Err("match: no arm matched"),
            });
        }
        Ok(arg)
    }

    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = args;
    let mut name: ArcStr;
    let mut found: bool;
    let mut arg: metamodelica::Ref<Absyn::ElementArg>;
    name = pathFirstIdent(&path);
    (args, found) = List::findAndMap(
        args,
        &({
            let __pe_b1 = name.clone();
            move |__pe_a0| Ok(is_named(&__pe_a0, &__pe_b1))
        }),
        &({
            let __pe_b1 = path.clone();
            let __pe_b2 = r#fn.clone();
            let __pe_b3 = insert;
            move |__pe_a0| apply_fn(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    if !(found) {
        if insert {
            arg = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
                finalPrefix: false,
                eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name }),
                modification: None,
                comment: None,
                info: Absyn::dummyInfo.clone(),
            });
            arg = apply_fn(arg, path, r#fn.clone(), insert)?;
            args = metamodelica::cons(arg, args);
        } else {
            return Err("fail");
        }
    }
    Ok(args)
}

pub(crate) fn mapCrefParts(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inMapFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<metamodelica::Ref<Absyn::ComponentRef>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentRef>) -> Result<metamodelica::Ref<Absyn::ComponentRef>>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (match &**inCref {
        Absyn::ComponentRef::CREF_QUAL {
            name,
            subscripts: subs,
            componentRef: rest_cref,
        } => {
            let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
            let mut name = (*name).clone();
            let mut subs = (*subs).clone();
            let mut rest_cref = (*rest_cref).clone();
            cref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: name.clone(),
                subscripts: subs.clone(),
            });
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inMapFunc(cref)?) {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, subscripts: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            rest_cref = mapCrefParts(metamodelica::AsArg::as_arg(&rest_cref), inMapFunc)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: name.clone(),
                subscripts: subs.clone(),
                componentRef: rest_cref.clone(),
            })
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut cref = (*cref).clone();
            cref = mapCrefParts(metamodelica::AsArg::as_arg(&cref), inMapFunc)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
                componentRef: cref.clone(),
            })
        }
        _ => inMapFunc(inCref.clone())?,
    });
    Ok(outCref)
}

pub fn opEqual(mut op1: Absyn::Operator, mut op2: Absyn::Operator) -> bool {
    let mut isEqual: bool;
    isEqual = op1 == op2;
    isEqual
}

pub fn opIsElementWise(mut op: Absyn::Operator) -> bool {
    let mut isElementWise: bool;
    isElementWise = (match op {
        Absyn::Operator::ADD_EW { .. } => true,
        Absyn::Operator::SUB_EW { .. } => true,
        Absyn::Operator::MUL_EW { .. } => true,
        Absyn::Operator::DIV_EW { .. } => true,
        Absyn::Operator::POW_EW { .. } => true,
        Absyn::Operator::UPLUS_EW { .. } => true,
        Absyn::Operator::UMINUS_EW { .. } => true,
        _ => false,
    });
    isElementWise
}

pub fn dummyTraverseExp<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inArg: Arg,
) -> (metamodelica::Ref<Absyn::Exp>, Arg) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outArg: Arg;
    outExp = inExp;
    outArg = inArg;
    (outExp, outArg)
}

pub fn getDefineUnitsInElements(
    mut elts: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outElts: metamodelica::List<metamodelica::Ref<Absyn::Element>> = metamodelica::nil();
    for mut i in &**elts {
        outElts = (::match_deref::match_deref! { match &(i.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::DEFINEUNIT { .. } } => metamodelica::cons(var_field!((**i).element, Absyn::ElementItem::ELEMENTITEM).clone(), outElts),
            _ => outElts,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outElts = metamodelica::Dangerous::listReverseInPlace(outElts);
    outElts
}

pub fn getClassPartsInClass(
    mut cls: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cls.body.clone();
    parts = (match &*cdef {
        Absyn::ClassDef::PARTS {
            classParts: __cdef_classParts,
            ..
        } => __cdef_classParts.clone(),
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __cdef_parts, ..
        } => __cdef_parts.clone(),
        _ => metamodelica::nil(),
    });
    parts
}

pub fn setClassPartsInClass(
    mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut cls: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cls.body.clone();
    let () = (match &*cdef {
        Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    assign_field!(cls.body = cdef);
    Ok(cls)
}

pub fn getElementItemsInElement(
    mut element: &metamodelica::Ref<Absyn::Element>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    outElements = (::match_deref::match_deref! { match element {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_cls, .. }, .. } => {
            cls = (*__esc_cls).clone();
            getElementItemsInClass(metamodelica::AsArg::as_arg(&cls))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElements)
}

pub fn getElementItemsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> =
        getElementItemsInClassDef(&inClass.body)?;
    Ok(outElements)
}

pub fn getElementItemsInClassDef(
    mut classDef: &metamodelica::Ref<Absyn::ClassDef>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outElements = (match &**classDef {
        Absyn::ClassDef::PARTS {
            classParts: __classDef_classParts,
            ..
        } => List::mapFlat(
            metamodelica::AsArg::as_arg(&__classDef_classParts),
            &move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(getElementItemsInClassPart(&__a0))
            },
        )?,
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __classDef_parts,
            ..
        } => List::mapFlat(
            metamodelica::AsArg::as_arg(&__classDef_parts),
            &move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(getElementItemsInClassPart(&__a0))
            },
        )?,
        _ => metamodelica::nil(),
    });
    Ok(outElements)
}

pub fn getElementItemsInClassPart(
    mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outElements = (match &**inClassPart {
        Absyn::ClassPart::PUBLIC {
            contents: __inClassPart_contents,
        } => __inClassPart_contents.clone(),
        Absyn::ClassPart::PROTECTED {
            contents: __inClassPart_contents,
        } => __inClassPart_contents.clone(),
        _ => metamodelica::nil(),
    });
    outElements
}

pub fn traverseClassComponents<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::Class>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >;

    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass;
    let mut outArg: ArgT;
    outClass = (match &*outClass {
        Absyn::Class { .. } => {
            let mut body: metamodelica::Ref<Absyn::ClassDef>;
            (body, outArg, _) = traverseClassDef(
                outClass.body.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, _) -> Result<_>
                            + 'static,
                    > = inFunc.clone();
                    move |__pe_a0, __pe_a2| traverseClassPartComponents(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                inArg,
            )?;
            if !(referenceEq(&*(&*body), &*(outClass.body.clone()))) {
                assign_field!(outClass.body = body);
            }
            outClass
        }
    });
    Ok((outClass, outArg))
}

fn traverseListGeneric<
    T: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
    ArgT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T, ArgT) -> Result<(T, ArgT, bool)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::List<T>, ArgT, bool)> {
    pub type FuncType<T: Clone + 'static, ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T, ArgT) -> Result<(T, ArgT, bool)> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut outArg: ArgT = inArg;
    let mut outContinue: bool = true;
    let mut eq: bool;
    let mut changed: bool = false;
    let mut e: T;
    let mut new_e: T;
    let mut rest_e: metamodelica::List<T> = inList.clone();
    while !((rest_e).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest_e = metamodelica::Own::own(__pa1);
        (new_e, outArg, outContinue) = inFunc(e.clone(), outArg)?;
        eq = metamodelica::ReferenceEq::reference_eq(&(new_e.clone()), &(e.clone()));
        outList = metamodelica::cons(if (eq) { e } else { new_e }, outList);
        changed = changed || !(eq);
        if !(outContinue) {
            break;
        }
    }
    if changed {
        outList = List::append_reverse(&outList, rest_e);
    } else {
        outList = inList;
    }
    Ok((outList, outArg, outContinue))
}

fn traverseClassPartComponents<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ClassPart>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >;

    let mut outClassPart: metamodelica::Ref<Absyn::ClassPart> = inClassPart;
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let () = (match &*outClassPart {
        Absyn::ClassPart::PUBLIC {
            contents: __outClassPart_contents,
        } => {
            let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (items, outArg, outContinue) = traverseListGeneric(
                __outClassPart_contents.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, _) -> Result<_>
                            + 'static,
                    > = inFunc.clone();
                    move |__pe_a0, __pe_a2| traverseElementItemComponents(__pe_a0, &*__pe_b1, __pe_a2)
                }),
                inArg,
            )?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = items);
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __outClassPart_contents,
        } => {
            let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (items, outArg, outContinue) = traverseListGeneric(
                __outClassPart_contents.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, _) -> Result<_>
                            + 'static,
                    > = inFunc.clone();
                    move |__pe_a0, __pe_a2| traverseElementItemComponents(__pe_a0, &*__pe_b1, __pe_a2)
                }),
                inArg,
            )?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = items);
            ()
        }
        _ => (),
    });
    Ok((outClassPart, outArg, outContinue))
}

fn traverseElementItemComponents<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inItem: metamodelica::Ref<Absyn::ElementItem>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
        ArgT,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ElementItem>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >;

    let mut outItem: metamodelica::Ref<Absyn::ElementItem>;
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outItem, outArg, outContinue) = (match &*inItem {
        Absyn::ElementItem::ELEMENTITEM {
            element: __inItem_element,
        } => {
            let mut elem: metamodelica::Ref<Absyn::Element>;
            (elem, outArg, outContinue) = traverseElementComponents(__inItem_element.clone(), inFunc, inArg)?;
            outItem = if (referenceEq(&*(&*elem), &*(__inItem_element.clone()))) {
                inItem
            } else {
                metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elem })
            };
            (outItem, outArg, outContinue)
        }
        _ => (inItem, inArg, true),
    });
    Ok((outItem, outArg, outContinue))
}

fn traverseElementComponents<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElement: metamodelica::Ref<Absyn::Element>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
        ArgT,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >;

    let mut outElement: metamodelica::Ref<Absyn::Element> = inElement.clone();
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outElement, outArg, outContinue) = (match &*outElement {
        Absyn::Element::ELEMENT {
            specification: __outElement_specification,
            ..
        } => {
            let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
            (spec, outArg, outContinue) =
                traverseElementSpecComponents(__outElement_specification.clone(), inFunc, inArg)?;
            if !(referenceEq(
                &*(&*spec),
                &*(var_field!((*outElement).specification, Absyn::Element::ELEMENT).clone()),
            )) {
                assign_variant_field!(outElement => Absyn::Element::ELEMENT; specification = spec);
            }
            (outElement, outArg, outContinue)
        }
        _ => (inElement, inArg, true),
    });
    Ok((outElement, outArg, outContinue))
}

fn traverseElementSpecComponents<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inSpec: metamodelica::Ref<Absyn::ElementSpec>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
        ArgT,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ElementSpec>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
                ArgT,
            )
                -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArgT, bool)>
            + 'static,
    >;

    let mut outSpec: metamodelica::Ref<Absyn::ElementSpec> = inSpec.clone();
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outSpec, outArg, outContinue) = (match &*outSpec {
        Absyn::ElementSpec::COMPONENTS {
            components: __outSpec_components,
            ..
        } => {
            let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
            (comps, outArg, outContinue) = inFunc(__outSpec_components.clone(), inArg)?;
            if !(metamodelica::ReferenceEq::reference_eq(
                &(comps),
                &(var_field!((*outSpec).components, Absyn::ElementSpec::COMPONENTS).clone()),
            )) {
                assign_variant_field!(outSpec => Absyn::ElementSpec::COMPONENTS; components = comps);
            }
            (outSpec, outArg, outContinue)
        }
        _ => (inSpec, inArg, true),
    });
    Ok((outSpec, outArg, outContinue))
}

fn traverseClassDef<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClassDef: metamodelica::Ref<Absyn::ClassDef>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Absyn::ClassPart>,
        ArgT,
    ) -> Result<(metamodelica::Ref<Absyn::ClassPart>, ArgT, bool)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ClassDef>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::ClassPart>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::ClassPart>, ArgT, bool)>
            + 'static,
    >;

    let mut outClassDef: metamodelica::Ref<Absyn::ClassDef> = inClassDef;
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let () = (match &*outClassDef {
        Absyn::ClassDef::PARTS {
            classParts: __outClassDef_classParts,
            ..
        } => {
            let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            (parts, outArg, outContinue) = traverseListGeneric(__outClassDef_classParts.clone(), inFunc, inArg)?;
            assign_variant_field!(outClassDef => Absyn::ClassDef::PARTS; classParts = parts);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __outClassDef_parts,
            ..
        } => {
            let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            (parts, outArg, outContinue) = traverseListGeneric(__outClassDef_parts.clone(), inFunc, inArg)?;
            assign_variant_field!(outClassDef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts);
            ()
        }
        _ => (),
    });
    Ok((outClassDef, outArg, outContinue))
}

pub fn isEmptyMod(mut inMod: &metamodelica::Ref<Absyn::Modification>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match inMod {
        Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } } => true,
        Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Nil }, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn isEmptySubMod(mut inSubMod: &metamodelica::Ref<Absyn::ElementArg>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match inSubMod {
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: true, .. } => {
            false
        },
        Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. } => {
            true
        },
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), .. } => {
            isEmptyMod(metamodelica::AsArg::as_arg(&r#mod))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub(crate) fn isEmptyEqMod(mut eqMod: &metamodelica::Ref<Absyn::EqMod>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (match &**eqMod {
        Absyn::EqMod::NOMOD { .. } => true,
        _ => false,
    });
    isEmpty
}

pub fn elementArgName(mut inArg: &metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outName: metamodelica::Ref<Absyn::Path>;
    outName = (match &**inArg {
        Absyn::ElementArg::MODIFICATION {
            path: __esc_outName, ..
        } => {
            outName = (*__esc_outName).clone();
            outName.clone()
        }
        Absyn::ElementArg::REDECLARATION { elementSpec: e, .. } => makeIdentPathFromString(elementSpecName(e)?),
        _ => return Err("match: no arm matched"),
    });
    Ok(outName)
}

pub fn elementArgEqualName(
    mut inArg1: &metamodelica::Ref<Absyn::ElementArg>,
    mut inArg2: &metamodelica::Ref<Absyn::ElementArg>,
) -> Result<bool> {
    let mut outEqual: bool = pathEqual(&(elementArgName(inArg1)?), &(elementArgName(inArg2)?));
    Ok(outEqual)
}

pub fn optMsg(mut inShowMessage: bool, mut inInfo: SourceInfo) -> Absyn::Msg {
    let mut outMsg: Absyn::Msg;
    outMsg = if (inShowMessage) {
        Absyn::Msg::MSG { info: inInfo }
    } else {
        openmodelica_ast::Absyn::Msg::NO_MSG
    };
    outMsg
}

pub fn makeSubscript(mut inExp: metamodelica::Ref<Absyn::Exp>) -> metamodelica::Ref<Absyn::Subscript> {
    let mut outSubscript: metamodelica::Ref<Absyn::Subscript>;
    outSubscript = metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: inExp });
    outSubscript
}

pub fn makeIntegerSubscript(mut n: i32) -> metamodelica::Ref<Absyn::Subscript> {
    let mut sub: metamodelica::Ref<Absyn::Subscript>;
    sub = metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
        subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: n }),
    });
    sub
}

pub fn crefExplode<'__b>(
    mut inCref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut inAccum: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>> {
    '__tco: loop {
        match &**inCref {
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                (inCref, inAccum) = (
                    var_field!((**inCref).componentRef, Absyn::ComponentRef::CREF_QUAL),
                    metamodelica::cons(crefFirstCref(inCref), inAccum),
                );
                continue '__tco;
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                (inCref, inAccum) = (
                    var_field!((**inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED),
                    inAccum,
                );
                continue '__tco;
            }
            _ => return metamodelica::cons(inCref.clone(), inAccum).reverse(),
        }
    }
}

pub fn traverseExpShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inArg: ArgT,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<Absyn::Exp> = inExp;
    let () = (match &*outExp {
        Absyn::Exp::BINARY {
            exp1: __outExp_exp1, ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::BINARY;
                exp1 = inFunc(__outExp_exp1.clone(), inArg.clone())?,
                exp2 = inFunc(var_field!((*outExp).exp2, Absyn::Exp::BINARY).clone(), inArg)?
            );
            ()
        }
        Absyn::Exp::UNARY { exp: __outExp_exp, .. } => {
            assign_variant_field!(outExp => Absyn::Exp::UNARY; exp = inFunc(__outExp_exp.clone(), inArg)?);
            ()
        }
        Absyn::Exp::LBINARY {
            exp1: __outExp_exp1, ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::LBINARY;
                exp1 = inFunc(__outExp_exp1.clone(), inArg.clone())?,
                exp2 = inFunc(var_field!((*outExp).exp2, Absyn::Exp::LBINARY).clone(), inArg)?
            );
            ()
        }
        Absyn::Exp::LUNARY { exp: __outExp_exp, .. } => {
            assign_variant_field!(outExp => Absyn::Exp::LUNARY; exp = inFunc(__outExp_exp.clone(), inArg)?);
            ()
        }
        Absyn::Exp::RELATION {
            exp1: __outExp_exp1, ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::RELATION;
                exp1 = inFunc(__outExp_exp1.clone(), inArg.clone())?,
                exp2 = inFunc(var_field!((*outExp).exp2, Absyn::Exp::RELATION).clone(), inArg)?
            );
            ()
        }
        Absyn::Exp::IFEXP {
            ifExp: __outExp_ifExp, ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::IFEXP;
                        ifExp = inFunc(__outExp_ifExp.clone(), inArg.clone())?,
                        trueBranch = inFunc(var_field!((*outExp).trueBranch, Absyn::Exp::IFEXP).clone(), inArg.clone())?,
                        elseBranch = inFunc(var_field!((*outExp).elseBranch, Absyn::Exp::IFEXP).clone(), inArg.clone())?,
                        elseIfBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<Absyn::Exp>)> = metamodelica::nil();
                for mut e in (var_field!((*outExp).elseIfBranch, Absyn::Exp::IFEXP).clone()).into_iter().cloned() {
                    let __x = (inFunc(Util::tuple21(e.clone()), inArg.clone())?, inFunc(Util::tuple22(e.clone()), inArg.clone())?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::Exp::CALL {
            functionArgs: __outExp_functionArgs,
            ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::CALL; functionArgs = traverseExpShallowFuncArgs(__outExp_functionArgs.clone(), inArg, inFunc)?);
            ()
        }
        Absyn::Exp::PARTEVALFUNCTION {
            functionArgs: __outExp_functionArgs,
            ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::PARTEVALFUNCTION; functionArgs = traverseExpShallowFuncArgs(__outExp_functionArgs.clone(), inArg, inFunc)?);
            ()
        }
        Absyn::Exp::ARRAY {
            arrayExp: __outExp_arrayExp,
        } => {
            assign_variant_field!(outExp => Absyn::Exp::ARRAY; arrayExp = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (__outExp_arrayExp.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone(), inArg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::Exp::MATRIX {
            matrix: __outExp_matrix,
        } => {
            assign_variant_field!(outExp => Absyn::Exp::MATRIX; matrix = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> = metamodelica::nil();
                for mut lst in (__outExp_matrix.clone()).into_iter().cloned() {
                    let __x = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (lst.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone(), inArg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::Exp::RANGE {
            start: __outExp_start, ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::RANGE;
                start = inFunc(__outExp_start.clone(), inArg.clone())?,
                step = Util::applyOption1(var_field!((*outExp).step, Absyn::Exp::RANGE).clone(), inFunc, inArg.clone())?,
                stop = inFunc(var_field!((*outExp).stop, Absyn::Exp::RANGE).clone(), inArg)?
            );
            ()
        }
        Absyn::Exp::TUPLE {
            expressions: __outExp_expressions,
        } => {
            assign_variant_field!(outExp => Absyn::Exp::TUPLE; expressions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (__outExp_expressions.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone(), inArg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::Exp::AS { exp: __outExp_exp, .. } => {
            assign_variant_field!(outExp => Absyn::Exp::AS; exp = inFunc(__outExp_exp.clone(), inArg)?);
            ()
        }
        Absyn::Exp::CONS {
            head: __outExp_head, ..
        } => {
            assign_variant_field!(outExp => Absyn::Exp::CONS;
                head = inFunc(__outExp_head.clone(), inArg.clone())?,
                rest = inFunc(var_field!((*outExp).rest, Absyn::Exp::CONS).clone(), inArg)?
            );
            ()
        }
        Absyn::Exp::LIST { exps: __outExp_exps } => {
            assign_variant_field!(outExp => Absyn::Exp::LIST; exps = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (__outExp_exps.clone()).into_iter().cloned() {
                    let __x = inFunc(e.clone(), inArg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::Exp::DOT { exp: __outExp_exp, .. } => {
            assign_variant_field!(outExp => Absyn::Exp::DOT;
                exp = inFunc(__outExp_exp.clone(), inArg.clone())?,
                index = inFunc(var_field!((*outExp).index, Absyn::Exp::DOT).clone(), inArg)?
            );
            ()
        }
        Absyn::Exp::EXPRESSIONCOMMENT { exp: __outExp_exp, .. } => {
            assign_variant_field!(outExp => Absyn::Exp::EXPRESSIONCOMMENT; exp = inFunc(__outExp_exp.clone(), inArg)?);
            ()
        }
        Absyn::Exp::SUBSCRIPTED_EXP { exp: __outExp_exp, .. } => {
            assign_variant_field!(outExp => Absyn::Exp::SUBSCRIPTED_EXP;
                        exp = inFunc(__outExp_exp.clone(), inArg.clone())?,
                        subscripts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                for mut s in (var_field!((*outExp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
                    let __x = traverseExpShallowSub(s.clone(), inArg.clone(), inFunc)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    Ok(outExp)
}

fn traverseExpShallowFuncArgs<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inArgs: metamodelica::Ref<Absyn::FunctionArgs>,
    mut inArg: ArgT,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut outArgs: metamodelica::Ref<Absyn::FunctionArgs> = inArgs;
    outArgs = (match &*outArgs {
        Absyn::FunctionArgs::FUNCTIONARGS {
            args: __outArgs_args, ..
        } => {
            assign_variant_field!(outArgs => Absyn::FunctionArgs::FUNCTIONARGS; args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut arg in (__outArgs_args.clone()).into_iter().cloned() {
                    let __x = inFunc(arg.clone(), inArg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            outArgs
        }
        Absyn::FunctionArgs::FOR_ITER_FARG { exp: __outArgs_exp, .. } => {
            assign_variant_field!(outArgs => Absyn::FunctionArgs::FOR_ITER_FARG;
                        exp = inFunc(__outArgs_exp.clone(), inArg.clone())?,
                        iterators = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
                for mut it in (var_field!((*outArgs).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone()).into_iter().cloned() {
                    let __x = traverseExpShallowIterator(&(it.clone()), inArg.clone(), inFunc)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            outArgs
        }
    });
    Ok(outArgs)
}

fn traverseExpShallowIterator<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIterator: &metamodelica::Ref<Absyn::ForIterator>,
    mut inArg: ArgT,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::ForIterator>> {
    pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut outIterator: metamodelica::Ref<Absyn::ForIterator>;
    let mut name: ArcStr;
    let mut guard_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut range_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let __arc3 = &(*inIterator);
    let Absyn::ITERATOR {
        name: __pa0,
        guardExp: __pa1,
        range: __pa2,
    } = &**__arc3;
    name = metamodelica::Own::own(__pa0);
    guard_exp = metamodelica::Own::own(__pa1);
    range_exp = metamodelica::Own::own(__pa2);
    guard_exp = Util::applyOption1(guard_exp, inFunc, inArg.clone())?;
    range_exp = Util::applyOption1(range_exp, inFunc, inArg)?;
    outIterator = metamodelica::Ref::new(Absyn::ForIterator {
        name: name,
        guardExp: guard_exp,
        range: range_exp,
    });
    Ok(outIterator)
}

pub(crate) fn traverseExpShallowSub<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut sub: metamodelica::Ref<Absyn::Subscript>,
    mut inArg: ArgT,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    pub type FuncT<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut sub: metamodelica::Ref<Absyn::Subscript> = sub;
    let () = (match &*sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = inFunc(__sub_subscript.clone(), inArg)?);
            ()
        }
        _ => (),
    });
    Ok(sub)
}

pub fn isElementItemClass(mut inElement: &metamodelica::Ref<Absyn::ElementItem>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn isElementItemExtends(mut item: &metamodelica::Ref<Absyn::ElementItem>) -> bool {
    let mut isExtends: bool;
    isExtends = (::match_deref::match_deref! { match item {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { .. }, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtends
}

pub(crate) fn isElementItem(mut inElement: &metamodelica::Ref<Absyn::ElementItem>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (match &**inElement {
        Absyn::ElementItem::ELEMENTITEM { .. } => true,
        _ => false,
    });
    outIsClass
}

pub(crate) fn isAlgorithmItem(mut inAlg: &metamodelica::Ref<Absyn::AlgorithmItem>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (match &**inAlg {
        Absyn::AlgorithmItem::ALGORITHMITEM { .. } => true,
        _ => false,
    });
    outIsClass
}

pub fn isElementItemClassNamed(mut inName: &ArcStr, mut inElement: &metamodelica::Ref<Absyn::ElementItem>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, .. }, .. }, .. } } => {
            metamodelica::stringEq(&name, &inName)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

pub fn isElementItemNamed(mut name: &ArcStr, mut element: &metamodelica::Ref<Absyn::ElementItem>) -> Result<bool> {
    let mut res: bool;
    res = (match &**element {
        Absyn::ElementItem::ELEMENTITEM {
            element: __element_element,
        } => isElementNamed(name, metamodelica::AsArg::as_arg(&__element_element))?,
        _ => false,
    });
    Ok(res)
}

pub fn isElementNamed(mut name: &ArcStr, mut element: &metamodelica::Ref<Absyn::Element>) -> Result<bool> {
    let mut res: bool;
    res = (match &**element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => isElementSpecNamed(name, metamodelica::AsArg::as_arg(&__element_specification))?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn isElementSpecNamed(
    mut name: &ArcStr,
    mut elementSpec: &metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<bool> {
    let mut res: bool;
    res = (match &**elementSpec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __elementSpec_class_,
            ..
        } => isClassNamed(name, metamodelica::AsArg::as_arg(&__elementSpec_class_)),
        Absyn::ElementSpec::COMPONENTS {
            components: __elementSpec_components,
            ..
        } => List::any(
            metamodelica::AsArg::as_arg(&__elementSpec_components),
            &({
                let __pe_b0 = name.clone();
                move |__pe_a1| Ok(isComponentItemNamed(&__pe_b0, &__pe_a1))
            }),
        )?,
        _ => false,
    });
    Ok(res)
}

pub fn isEmptyClassPart(mut inClassPart: &metamodelica::Ref<Absyn::ClassPart>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match inClassPart {
        Deref @ Absyn::ClassPart::PUBLIC { contents: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ Absyn::ClassPart::PROTECTED { contents: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ Absyn::ClassPart::CONSTRAINTS { contents: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ Absyn::ClassPart::EQUATIONS { contents: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ Absyn::ClassPart::ALGORITHMS { contents: Deref @ metamodelica::ListNode::Nil } => true,
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: Deref @ metamodelica::ListNode::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn isInvariantExpNoTraverse(
    mut e: metamodelica::Ref<Absyn::Exp>,
    mut b: bool,
) -> (metamodelica::Ref<Absyn::Exp>, bool) {
    let mut e: metamodelica::Ref<Absyn::Exp> = e;
    let mut b: bool = b;
    if !(b) {
        return (e, b);
    }
    b = (::match_deref::match_deref! { match &(&*e) {
        Deref @ Absyn::Exp::INTEGER { .. } => true,
        Deref @ Absyn::Exp::REAL { .. } => true,
        Deref @ Absyn::Exp::STRING { .. } => true,
        Deref @ Absyn::Exp::BOOL { .. } => true,
        Deref @ Absyn::Exp::BINARY { .. } => true,
        Deref @ Absyn::Exp::UNARY { .. } => true,
        Deref @ Absyn::Exp::LBINARY { .. } => true,
        Deref @ Absyn::Exp::LUNARY { .. } => true,
        Deref @ Absyn::Exp::RELATION { .. } => true,
        Deref @ Absyn::Exp::IFEXP { .. } => true,
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }, .. } => true,
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }, .. } => true,
        Deref @ Absyn::Exp::ARRAY { .. } => true,
        Deref @ Absyn::Exp::MATRIX { .. } => true,
        Deref @ Absyn::Exp::RANGE { .. } => true,
        Deref @ Absyn::Exp::CONS { .. } => true,
        Deref @ Absyn::Exp::LIST { .. } => true,
        Deref @ Absyn::Exp::BREAK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (e, b)
}

pub fn pathPartCount<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>, mut partsAccum: i32) -> i32 {
    '__tco: loop {
        match &**path {
            Absyn::Path::IDENT { .. } => return partsAccum + 1,
            Absyn::Path::QUALIFIED { .. } => {
                (path, partsAccum) = (var_field!((**path).path, Absyn::Path::QUALIFIED), partsAccum + 1);
                continue '__tco;
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (path, partsAccum) = (var_field!((**path).path, Absyn::Path::FULLYQUALIFIED), partsAccum);
                continue '__tco;
            }
        }
    }
}

pub fn getAnnotationsFromConstraintClass(
    mut inCC: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    let mut elementArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    elementArgs = (::match_deref::match_deref! { match &(inCC) {
        Some(Deref @ Absyn::ConstrainClass { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: __esc_elementArgs }), .. }), .. }) => {
            elementArgs = (*__esc_elementArgs).clone();
            elementArgs.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elementArgs
}

pub fn getAnnotationsFromItems(
    mut inComponentItems: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut ccAnnotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> = metamodelica::nil();
    let mut annotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    for mut comp in &*inComponentItems.reverse() {
        annotations = (::match_deref::match_deref! { match &(comp.clone()) {
            Deref @ Absyn::ComponentItem { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: __esc_annotations }), .. }), .. } => {
                annotations = (*__esc_annotations).clone();
                listAppend(annotations.clone(), ccAnnotations.clone())
            },
            _ => ccAnnotations.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outLst = metamodelica::cons(annotations.clone(), outLst);
    }
    outLst
}

pub fn stripGraphicsAndInteractionModification(
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
)> {
    let mut outAbsynElementArgLst1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut outAbsynElementArgLst2: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    (outAbsynElementArgLst1, outAbsynElementArgLst2) = 'mc: {
        let __mc_input = &**inAbsynElementArgLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "interaction" }, .. }, tail: rest } => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&rest))?;
                    Ok((l1.clone(), l2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: None, path: Deref @ Absyn::Path::IDENT { name: Deref @ "graphics" }, .. }, tail: rest } => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&rest))?;
                    Ok((l1.clone(), l2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#mod @ Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(_), path: Deref @ Absyn::Path::IDENT { name: Deref @ "graphics" }, .. }, tail: rest } => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&rest))?;
                    Ok((l1.clone(), metamodelica::cons(r#mod.clone(), l2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#mod @ Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(_), path: Deref @ Absyn::Path::IDENT { name: Deref @ "choice" }, .. }, tail: rest } => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&rest))?;
                    Ok((l1.clone(), metamodelica::cons(r#mod.clone(), l2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: r#mod @ Deref @ Absyn::ElementArg::MODIFICATION { .. }, tail: rest } => {
                    let mut l1: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&rest))?;
                    Ok((metamodelica::cons(r#mod.clone(), l1.clone()), l2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outAbsynElementArgLst1, outAbsynElementArgLst2))
}

pub fn traverseClasses<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inProgram: Absyn::Program,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >,
    mut inArg: Arg,
    mut inVisitProtected: bool,
) -> Result<(Absyn::Program, Option<metamodelica::Ref<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >;

    let mut outTpl: (Absyn::Program, Option<metamodelica::Ref<Absyn::Path>>, Arg);
    outTpl = (match inProgram {
        mut p @ Absyn::Program { .. } => {
            let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
            let mut pa: Option<metamodelica::Ref<Absyn::Path>>;
            let mut arg: Arg;
            (classes, pa, arg) = traverseClasses2(&p.classes, inPath, inFunc.clone(), inArg, inVisitProtected)?;
            p.classes = classes;
            (p.clone(), pa, arg)
        }
    });
    Ok(outTpl)
}

fn traverseClasses2<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClasses: &metamodelica::List<metamodelica::Ref<Absyn::Class>>,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >,
    mut inArg: Arg,
    mut inVisitProtected: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::Class>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Arg,
)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >;

    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<Absyn::Class>>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    );
    outTpl = 'mc: {
        let __mc_input = (&**inClasses, inPath, inFunc.clone(), inArg, inVisitProtected);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, pa, _, args, _) => {
                    Ok((metamodelica::nil(), pa.clone(), args.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: class_, tail: classes }, pa, visitor, args, traverse_prot) => {
                    let mut pa_3: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut args_1: Arg;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_1: metamodelica::Ref<Absyn::Class>;
                    let mut class_2: metamodelica::Ref<Absyn::Class>;
                    let mut classes_1: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                    (class_1, _, args_1) = visitor((class_.clone(), pa.clone(), args.clone()))?;
                    (class_2, _, args_2) = traverseInnerClass(class_1.clone(), pa.clone(), &*(visitor.clone()), args_1.clone(), traverse_prot.clone());
                    (classes_1, pa_3, args_3) = traverseClasses2(metamodelica::AsArg::as_arg(&classes), pa.clone(), visitor.clone(), args_2.clone(), traverse_prot.clone())?;
                    Ok((metamodelica::cons(class_2.clone(), classes_1.clone()), pa_3.clone(), args_3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: class_, tail: classes }, pa, visitor, args, traverse_prot) => {
                    let mut pa_3: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_2: metamodelica::Ref<Absyn::Class>;
                    let mut classes_1: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                    (class_2, _, args_2) = traverseInnerClass(class_.clone(), pa.clone(), &*(visitor.clone()), args.clone(), traverse_prot.clone());
                    let true = (classHasLocalClasses(&class_2)?) else { return Err("pattern mismatch") };
                    (classes_1, pa_3, args_3) = traverseClasses2(metamodelica::AsArg::as_arg(&classes), pa.clone(), visitor.clone(), args_2.clone(), traverse_prot.clone())?;
                    Ok((metamodelica::cons(class_2.clone(), classes_1.clone()), pa_3.clone(), args_3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: classes }, pa, visitor, args, traverse_prot) => {
                    let mut pa_3: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut args_3: Arg;
                    let mut classes_1: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
                    (classes_1, pa_3, args_3) = traverseClasses2(metamodelica::AsArg::as_arg(&classes), pa.clone(), visitor.clone(), args.clone(), traverse_prot.clone())?;
                    Ok((classes_1.clone(), pa_3.clone(), args_3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: class_, tail: _ }, _, _, _, _) => {
                    metamodelica::print(literal!("-traverse_classes2 failed on class:"));
                    metamodelica::print(className(metamodelica::AsArg::as_arg(&class_)));
                    metamodelica::print(literal!("\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTpl)
}

fn classHasLocalClasses(mut cl: &metamodelica::Ref<Absyn::Class>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match cl {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            partsHasLocalClass(metamodelica::AsArg::as_arg(&parts))
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            partsHasLocalClass(metamodelica::AsArg::as_arg(&parts))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

fn partsHasLocalClass<'__b>(mut inParts: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inParts {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: _ } if (eltsHasLocalClass(metamodelica::AsArg::as_arg(&elts))) => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: _ } if (eltsHasLocalClass(metamodelica::AsArg::as_arg(&elts))) => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: parts } => {
                { inParts = parts; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn eltsHasLocalClass<'__b>(mut inElts: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inElts {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: elts } => {
                { inElts = elts; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn traverseInnerClass<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut path: Option<metamodelica::Ref<Absyn::Path>>,
    mut visitor: &dyn ::std::ops::Fn(
        (
            metamodelica::Ref<Absyn::Class>,
            Option<metamodelica::Ref<Absyn::Path>>,
            Arg,
        ),
    ) -> Result<(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    )>,
    mut arg: Arg,
    mut visitProtected: bool,
) -> (
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Arg,
) {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >;

    let mut outTpl: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    );
    let mut cls: metamodelica::Ref<Absyn::Class> = inClass.clone();
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = inClass.body.clone();
    let mut pa: metamodelica::Ref<Absyn::Path>;
    let mut opt_pa: Option<metamodelica::Ref<Absyn::Path>> = None;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut args: Arg;
    (cdef, opt_pa, args) = 'mc: {
        let __mc_input = (cdef.clone(), path.clone());
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::PARTS { .. }, Some(pa)) => {
                    let mut pa = (*pa).clone();
                    let mut args: Arg;
                    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef.clone();
                    let mut opt_pa: Option<metamodelica::Ref<Absyn::Path>> = opt_pa.clone();
                    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts.clone();
                    pa = joinPaths(pa.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: cls.name.clone() }))?;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), Some(pa.clone()), visitor, arg.clone(), visitProtected)?;
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    Ok(((cdef.clone(), opt_pa.clone(), args.clone()), cdef.clone(), opt_pa.clone(), parts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cdef = __wb0;
            opt_pa = __wb1;
            parts = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::PARTS { .. }, None) => {
                    let mut args: Arg;
                    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef.clone();
                    let mut opt_pa: Option<metamodelica::Ref<Absyn::Path>> = opt_pa.clone();
                    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts.clone();
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), Some(metamodelica::Ref::new(Absyn::Path::IDENT { name: cls.name.clone() })), visitor, arg.clone(), visitProtected)?;
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    Ok(((cdef.clone(), opt_pa.clone(), args.clone()), cdef.clone(), opt_pa.clone(), parts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cdef = __wb0;
            opt_pa = __wb1;
            parts = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::PARTS { .. }, opt_pa) => {
                    let mut opt_pa = (*opt_pa).clone();
                    let mut args: Arg;
                    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef.clone();
                    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts.clone();
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), opt_pa.clone(), visitor, arg.clone(), visitProtected)?;
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    Ok(((cdef.clone(), opt_pa.clone(), args.clone()), cdef.clone(), parts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cdef = __wb0;
            parts = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, Some(pa)) => {
                    let mut pa = (*pa).clone();
                    let mut args: Arg;
                    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef.clone();
                    let mut opt_pa: Option<metamodelica::Ref<Absyn::Path>> = opt_pa.clone();
                    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts.clone();
                    pa = joinPaths(pa.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: cls.name.clone() }))?;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Some(pa.clone()), visitor, arg.clone(), visitProtected)?;
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    Ok(((cdef.clone(), opt_pa.clone(), args.clone()), cdef.clone(), opt_pa.clone(), parts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cdef = __wb0;
            opt_pa = __wb1;
            parts = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, None) => {
                    let mut args: Arg;
                    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef.clone();
                    let mut opt_pa: Option<metamodelica::Ref<Absyn::Path>> = opt_pa.clone();
                    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts.clone();
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Some(metamodelica::Ref::new(Absyn::Path::IDENT { name: cls.name.clone() })), visitor, arg.clone(), visitProtected)?;
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    Ok(((cdef.clone(), opt_pa.clone(), args.clone()), cdef.clone(), opt_pa.clone(), parts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cdef = __wb0;
            opt_pa = __wb1;
            parts = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, opt_pa) => {
                    let mut opt_pa = (*opt_pa).clone();
                    let mut args: Arg;
                    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef.clone();
                    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts.clone();
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), opt_pa.clone(), visitor, arg.clone(), visitProtected)?;
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    Ok(((cdef.clone(), opt_pa.clone(), args.clone()), cdef.clone(), parts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cdef = __wb0;
            parts = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cdef.clone(), path.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    assign_field!(cls.body = cdef);
    outTpl = (cls, opt_pa, args);
    outTpl
}

fn traverseInnerClassParts<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut visitor: &dyn ::std::ops::Fn(
        (
            metamodelica::Ref<Absyn::Class>,
            Option<metamodelica::Ref<Absyn::Path>>,
            Arg,
        ),
    ) -> Result<(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    )>,
    mut inArg: Arg,
    mut visitProtected: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Arg,
)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >;

    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    );
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
    let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut arg: Arg = inArg;
    parts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
        for mut p in (inClassParts).into_iter().cloned() {
            let __x = (match &*p.clone() {
                Absyn::ClassPart::PUBLIC { contents: __p_contents } => {
                    (elts, _, arg) = traverseInnerClassElements(
                        metamodelica::AsArg::as_arg(&__p_contents),
                        inPath.clone(),
                        visitor,
                        arg.clone(),
                        visitProtected,
                    )?;
                    metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elts.clone() })
                }
                Absyn::ClassPart::PROTECTED { contents: __p_contents } if (visitProtected) => {
                    (elts, _, arg) = traverseInnerClassElements(
                        metamodelica::AsArg::as_arg(&__p_contents),
                        inPath.clone(),
                        visitor,
                        arg.clone(),
                        true,
                    )?;
                    metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: elts.clone() })
                }
                _ => p.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outTpl = (parts, inPath, arg);
    Ok(outTpl)
}

fn traverseInnerClassElements<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElements: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut visitor: &dyn ::std::ops::Fn(
        (
            metamodelica::Ref<Absyn::Class>,
            Option<metamodelica::Ref<Absyn::Path>>,
            Arg,
        ),
    ) -> Result<(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    )>,
    mut inArg: Arg,
    mut visitProtected: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Arg,
)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >;

    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    );
    let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
    let mut el: metamodelica::Ref<Absyn::Element>;
    let mut arg: Arg = inArg;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut cl: metamodelica::Ref<Absyn::Class>;
    for mut e in &**inElements {
        let mut e = e.clone();
        elts = (::match_deref::match_deref! { match &(e.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: __esc_el @ Deref @ Absyn::Element::ELEMENT { specification: __esc_spec, .. } } => {
                el = (*__esc_el).clone();
                spec = (*__esc_spec).clone();
                (spec, _, arg) = traverseInnerClassElementspec(spec.clone(), inPath.clone(), visitor, arg, visitProtected)?;
                assign_variant_field!(el => Absyn::Element::ELEMENT; specification = spec.clone());
                assign_variant_field!(e => Absyn::ElementItem::ELEMENTITEM; element = el.clone());
                metamodelica::cons(e, elts)
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: __esc_el @ Deref @ Absyn::Element::ELEMENT { specification: __esc_spec @ Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } } => {
                el = (*__esc_el).clone();
                spec = (*__esc_spec).clone();
                (cl, _, arg) = traverseInnerClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), inPath.clone(), visitor, arg, visitProtected);
                assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cl);
                assign_variant_field!(el => Absyn::Element::ELEMENT; specification = spec.clone());
                assign_variant_field!(e => Absyn::ElementItem::ELEMENTITEM; element = el.clone());
                metamodelica::cons(e, elts)
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { .. } } => elts,
            _ => metamodelica::cons(e, elts),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    elts = metamodelica::Dangerous::listReverseInPlace(elts);
    outTpl = (elts, inPath, arg);
    Ok(outTpl)
}

fn traverseInnerClassElementspec<Arg: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElementSpec: metamodelica::Ref<Absyn::ElementSpec>,
    mut inPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut visitor: &dyn ::std::ops::Fn(
        (
            metamodelica::Ref<Absyn::Class>,
            Option<metamodelica::Ref<Absyn::Path>>,
            Arg,
        ),
    ) -> Result<(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    )>,
    mut inArg: Arg,
    mut visitProtected: bool,
) -> Result<(
    metamodelica::Ref<Absyn::ElementSpec>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Arg,
)> {
    pub type FuncType<Arg: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                (
                    metamodelica::Ref<Absyn::Class>,
                    Option<metamodelica::Ref<Absyn::Path>>,
                    Arg,
                ),
            ) -> Result<(
                metamodelica::Ref<Absyn::Class>,
                Option<metamodelica::Ref<Absyn::Path>>,
                Arg,
            )> + 'static,
    >;

    let mut outTpl: (
        metamodelica::Ref<Absyn::ElementSpec>,
        Option<metamodelica::Ref<Absyn::Path>>,
        Arg,
    );
    outTpl = (match &*inElementSpec {
        Absyn::ElementSpec::CLASSDEF {
            replaceable_: repl,
            class_: cl,
        } => {
            let mut pa = inPath;
            let mut args = inArg;
            let mut cl = (*cl).clone();
            (cl, _, args) = visitor((cl.clone(), pa.clone(), args))?;
            (cl, pa, args) = traverseInnerClass(cl.clone(), pa, visitor, args, visitProtected);
            (
                metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                    replaceable_: repl.clone(),
                    class_: cl.clone(),
                }),
                pa,
                args,
            )
        }
        Absyn::ElementSpec::EXTENDS { .. } => {
            let mut pa = inPath;
            let mut args = inArg;
            (inElementSpec, pa, args)
        }
        Absyn::ElementSpec::IMPORT { .. } => {
            let mut pa = inPath;
            let mut args = inArg;
            (inElementSpec, pa, args)
        }
        Absyn::ElementSpec::COMPONENTS { .. } => {
            let mut pa = inPath;
            let mut args = inArg;
            (inElementSpec, pa, args)
        }
    });
    Ok(outTpl)
}

pub fn getTypeSpecFromElementItemOpt(
    mut inElementItem: &metamodelica::Ref<Absyn::ElementItem>,
) -> Option<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut outTypeSpec: Option<metamodelica::Ref<Absyn::TypeSpec>>;
    outTypeSpec = (::match_deref::match_deref! { match inElementItem {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: ty_spec, .. }, .. } } => {
            Some(ty_spec.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTypeSpec
}

pub(crate) fn getElementSpecificationFromElementItemOpt(
    mut inElementItem: &metamodelica::Ref<Absyn::ElementItem>,
) -> Option<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outSpec: Option<metamodelica::Ref<Absyn::ElementSpec>>;
    outSpec = (::match_deref::match_deref! { match inElementItem {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: spec, .. } } => {
            Some(spec.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSpec
}

pub fn getComponentItemsFromElement(
    mut element: &metamodelica::Ref<Absyn::Element>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    items = (::match_deref::match_deref! { match element {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: __esc_items, .. }, .. } => {
            items = (*__esc_items).clone();
            items.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    items
}

pub(crate) fn getComponentItemsFromElementSpec(
    mut elemSpec: &metamodelica::Ref<Absyn::ElementSpec>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut componentItems: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    componentItems = (match &**elemSpec {
        Absyn::ElementSpec::COMPONENTS {
            components: __elemSpec_components,
            ..
        } => __elemSpec_components.clone(),
        _ => metamodelica::nil(),
    });
    componentItems
}

pub fn getComponentItemsFromElementItem(
    mut inElementItem: &metamodelica::Ref<Absyn::ElementItem>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut componentItems: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    componentItems = (::match_deref::match_deref! { match &(getElementSpecificationFromElementItemOpt(inElementItem)) {
        Some(elementSpec) => {
            getComponentItemsFromElementSpec(metamodelica::AsArg::as_arg(&elementSpec))
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    componentItems
}

pub fn getDirection(mut elementItem: &metamodelica::Ref<Absyn::ElementItem>) -> Absyn::Direction {
    let mut oDirection: Absyn::Direction;
    oDirection = (::match_deref::match_deref! { match elementItem {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { direction: __esc_oDirection, .. }, .. }, .. } } => {
            oDirection = (*__esc_oDirection).clone();
            oDirection.clone()
        },
        _ => openmodelica_ast::Absyn::Direction::BIDIR,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oDirection
}

pub fn isNamedPathIdent(mut path: &metamodelica::Ref<Absyn::Path>, mut name: &ArcStr) -> bool {
    let mut res: bool;
    res = (match &**path {
        Absyn::Path::IDENT { name: __path_name } => metamodelica::stringEq(&__path_name, &name),
        _ => false,
    });
    res
}

pub(crate) fn isUniontype(mut cls: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut b: bool;
    b = (match cls.restriction.clone() {
        Absyn::Restriction::R_UNIONTYPE => true,
        _ => false,
    });
    b
}

pub fn traverseClassElements<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::Class>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >;

    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut arg: ArgT = arg;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    (body, arg) = traverseClassDefElements(cls.body.clone(), func.clone(), arg)?;
    if !(referenceEq(&*(&*body), &*(cls.body.clone()))) {
        assign_field!(cls.body = body);
    }
    Ok((cls, arg))
}

pub(crate) fn traverseClassDefElements<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut classDef: metamodelica::Ref<Absyn::ClassDef>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ClassDef>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >;

    let mut classDef: metamodelica::Ref<Absyn::ClassDef> = classDef;
    let mut arg: ArgT = arg;
    (classDef, arg, _) = traverseClassDef(
        classDef,
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>, _) -> Result<_> + 'static> =
                func.clone();
            move |__pe_a0, __pe_a2| traverseClassPartElements(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        arg,
    )?;
    Ok((classDef, arg))
}

fn traverseClassPartElements<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClassPart: metamodelica::Ref<Absyn::ClassPart>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ClassPart>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >;

    let mut outClassPart: metamodelica::Ref<Absyn::ClassPart> = inClassPart;
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let () = (match &*outClassPart {
        Absyn::ClassPart::PUBLIC {
            contents: __outClassPart_contents,
        } => {
            let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (items, outArg, outContinue) = traverseListGeneric(
                __outClassPart_contents.clone(),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>, _) -> Result<_> + 'static> =
                        inFunc.clone();
                    move |__pe_a0, __pe_a2| traverseElementItem(__pe_a0, &*__pe_b1, __pe_a2)
                }),
                inArg,
            )?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = items);
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __outClassPart_contents,
        } => {
            let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            (items, outArg, outContinue) = traverseListGeneric(
                __outClassPart_contents.clone(),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>, _) -> Result<_> + 'static> =
                        inFunc.clone();
                    move |__pe_a0, __pe_a2| traverseElementItem(__pe_a0, &*__pe_b1, __pe_a2)
                }),
                inArg,
            )?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = items);
            ()
        }
        _ => (),
    });
    Ok((outClassPart, outArg, outContinue))
}

fn traverseElementItem<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inItem: metamodelica::Ref<Absyn::ElementItem>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Absyn::Element>,
        ArgT,
    ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ElementItem>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Element>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Absyn::Element>, ArgT, bool)>
            + 'static,
    >;

    let mut outItem: metamodelica::Ref<Absyn::ElementItem>;
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outItem, outArg, outContinue) = (match &*inItem {
        Absyn::ElementItem::ELEMENTITEM {
            element: __inItem_element,
        } => {
            let mut elem: metamodelica::Ref<Absyn::Element>;
            (elem, outArg, outContinue) = inFunc(__inItem_element.clone(), inArg)?;
            outItem = if (referenceEq(&*(&*elem), &*(__inItem_element.clone()))) {
                inItem
            } else {
                metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: elem })
            };
            (outItem, outArg, outContinue)
        }
        _ => (inItem, inArg, true),
    });
    Ok((outItem, outArg, outContinue))
}

pub fn elementSpec(mut el: &metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut elSpec: metamodelica::Ref<Absyn::ElementSpec>;
    let __pa0 = ::match_deref::match_deref! { match &((*el)) {
        Deref @ Absyn::Element::ELEMENT { specification: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    elSpec = metamodelica::Own::own(__pa0);
    Ok(elSpec)
}

pub(crate) fn isClassOrComponentElementSpec(mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>) -> bool {
    let mut yes: bool = false;
    yes = (::match_deref::match_deref! { match inElementSpec {
        Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { .. }, .. } => true,
        Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

pub fn isPartial(mut inClass: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut outBoolean: bool;
    let __arc1 = &(*inClass);
    let Absyn::CLASS {
        partialPrefix: __pa0, ..
    } = &**__arc1;
    outBoolean = metamodelica::Own::own(__pa0);
    outBoolean
}

pub fn isNotPartial(mut inClass: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut outBoolean: bool;
    outBoolean = !(isPartial(inClass));
    outBoolean
}

pub fn crefIsWild(mut cref: &metamodelica::Ref<Absyn::ComponentRef>) -> bool {
    let mut wild: bool;
    wild = (match &**cref {
        Absyn::ComponentRef::WILD { .. } => true,
        Absyn::ComponentRef::ALLWILD { .. } => true,
        _ => false,
    });
    wild
}

pub fn makeCall(
    mut name: metamodelica::Ref<Absyn::ComponentRef>,
    mut posArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut namedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> metamodelica::Ref<Absyn::Exp> {
    let mut callExp: metamodelica::Ref<Absyn::Exp>;
    callExp = metamodelica::Ref::new(Absyn::Exp::CALL {
        function_: name,
        functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
            args: posArgs,
            argNames: namedArgs,
        }),
        typeVars: metamodelica::nil(),
    });
    callExp
}

pub(crate) fn setClassCommentsAfterEnd(
    mut cl: metamodelica::Ref<Absyn::Class>,
    mut comments: metamodelica::List<ArcStr>,
) -> metamodelica::Ref<Absyn::Class> {
    let mut cl: metamodelica::Ref<Absyn::Class> = cl;
    assign_field!(cl.commentsAfterEnd = comments);
    cl
}

pub fn pathReplaceFirst(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut prefix: &metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match &**path {
        Absyn::Path::IDENT { .. } => prefix.clone(),
        Absyn::Path::QUALIFIED { path: __path_path, .. } => joinPaths(prefix.clone(), __path_path.clone())?,
        Absyn::Path::FULLYQUALIFIED { path: __path_path } => metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED {
            path: pathReplaceFirst(metamodelica::AsArg::as_arg(&__path_path), prefix)?,
        }),
    });
    Ok(outPath)
}

pub fn pathContains<'__b>(mut path: &'__b metamodelica::Ref<Absyn::Path>, mut name: &'__b ArcStr) -> bool {
    '__tco: loop {
        match &**path {
            Absyn::Path::IDENT { .. } => {
                return metamodelica::stringEq(&var_field!((**path).name, Absyn::Path::IDENT), &name);
            }
            Absyn::Path::QUALIFIED { .. } => {
                return metamodelica::stringEq(&var_field!((**path).name, Absyn::Path::QUALIFIED), &name)
                    || pathContains(var_field!((**path).path, Absyn::Path::QUALIFIED), name);
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (path, name) = (var_field!((**path).path, Absyn::Path::FULLYQUALIFIED), name);
                continue '__tco;
            }
        }
    }
}

pub fn getClassAnnotation(
    mut cls: &metamodelica::Ref<Absyn::Class>,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    outAnnotation = getClassDefAnnotation(&cls.body)?;
    Ok(outAnnotation)
}

pub(crate) fn getClassDefAnnotation(
    mut def: &metamodelica::Ref<Absyn::ClassDef>,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    outAnnotation = (match &**def {
        Absyn::ClassDef::PARTS { ann: __def_ann, .. } if (!((__def_ann).is_empty())) => {
            Some((__def_ann).head().cloned()?)
        }
        Absyn::ClassDef::DERIVED {
            comment: __def_comment, ..
        } => getCommentOptAnnotation(__def_comment.clone())?,
        Absyn::ClassDef::ENUMERATION {
            comment: __def_comment, ..
        } => getCommentOptAnnotation(__def_comment.clone())?,
        Absyn::ClassDef::OVERLOAD {
            comment: __def_comment, ..
        } => getCommentOptAnnotation(__def_comment.clone())?,
        Absyn::ClassDef::CLASS_EXTENDS { ann: __def_ann, .. } if (!((__def_ann).is_empty())) => {
            Some((__def_ann).head().cloned()?)
        }
        Absyn::ClassDef::PDER {
            comment: __def_comment, ..
        } => getCommentOptAnnotation(__def_comment.clone())?,
        _ => None,
    });
    Ok(outAnnotation)
}

pub fn setClassAnnotation(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut ann: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    assign_field!(cls.body = setClassDefAnnotation(cls.body.clone(), ann)?);
    Ok(cls)
}

pub(crate) fn setClassDefAnnotation(
    mut cdef: metamodelica::Ref<Absyn::ClassDef>,
    mut ann: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef;
    let () = (match &*cdef {
        Absyn::ClassDef::PARTS { .. } => {
            if !((var_field!((*cdef).ann, Absyn::ClassDef::PARTS)).is_empty()) {
                assign_variant_field!(cdef => Absyn::ClassDef::PARTS; ann = (var_field!((*cdef).ann, Absyn::ClassDef::PARTS)).rest()?);
            }
            if (ann).is_some() {
                assign_variant_field!(cdef => Absyn::ClassDef::PARTS; ann = metamodelica::cons(ann.ok_or("pattern mismatch")?, var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone()));
            }
            ()
        }
        Absyn::ClassDef::DERIVED {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; comment = setCommentAnnotation(__cdef_comment.clone(), ann)?);
            ()
        }
        Absyn::ClassDef::ENUMERATION {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::ENUMERATION; comment = setCommentAnnotation(__cdef_comment.clone(), ann)?);
            ()
        }
        Absyn::ClassDef::OVERLOAD {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::OVERLOAD; comment = setCommentAnnotation(__cdef_comment.clone(), ann)?);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            if !((var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS)).is_empty()) {
                assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; ann = (var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS)).rest()?);
            }
            if (ann).is_some() {
                assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; ann = metamodelica::cons(ann.ok_or("pattern mismatch")?, var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()));
            }
            ()
        }
        Absyn::ClassDef::PDER {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PDER; comment = setCommentAnnotation(__cdef_comment.clone(), ann)?);
            ()
        }
        _ => (),
    });
    Ok(cdef)
}

pub fn setCommentString(
    mut comment: Option<metamodelica::Ref<Absyn::Comment>>,
    mut commentString: Option<ArcStr>,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut comment: Option<metamodelica::Ref<Absyn::Comment>> = comment;
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut r#str: Option<ArcStr>;
    if (comment).is_some() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comment) {
            Some(Deref @ Absyn::Comment { annotation_: __pa0, comment: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ann = metamodelica::Own::own(__pa0);
        r#str = metamodelica::Own::own(__pa1);
        comment = if ((ann).is_some() || (r#str).is_some()) {
            Some(metamodelica::Ref::new(Absyn::Comment {
                annotation_: ann,
                comment: commentString,
            }))
        } else {
            None
        };
    } else if (commentString).is_some() {
        comment = Some(metamodelica::Ref::new(Absyn::Comment {
            annotation_: None,
            comment: commentString,
        }));
    }
    Ok(comment)
}

pub(crate) fn setCommentAnnotation(
    mut comment: Option<metamodelica::Ref<Absyn::Comment>>,
    mut ann: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut comment: Option<metamodelica::Ref<Absyn::Comment>> = comment;
    let mut old_ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut r#str: Option<ArcStr>;
    if (comment).is_some() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comment) {
            Some(Deref @ Absyn::Comment { annotation_: __pa0, comment: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        old_ann = metamodelica::Own::own(__pa0);
        r#str = metamodelica::Own::own(__pa1);
        comment = if ((ann).is_some() || (r#str).is_some()) {
            Some(metamodelica::Ref::new(Absyn::Comment {
                annotation_: ann,
                comment: r#str,
            }))
        } else {
            None
        };
    } else if (ann).is_some() {
        comment = Some(metamodelica::Ref::new(Absyn::Comment {
            annotation_: ann,
            comment: None,
        }));
    }
    Ok(comment)
}

pub fn mapAnnotationBinding(
    mut ann: metamodelica::Ref<Absyn::Annotation>,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static>,
) -> Result<(metamodelica::Ref<Absyn::Annotation>, bool)> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut ann: metamodelica::Ref<Absyn::Annotation> = ann;
    let mut found: bool;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = ann.elementArgs.clone();
    (args, found) = List::findMap(
        args,
        &({
            let __pe_b1 = path.clone();
            let __pe_b2: Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
            > = func.clone();
            move |__pe_a0| mapAnnotationBindingInArg(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }),
    )?;
    assign_field!(ann.elementArgs = args);
    Ok((ann, found))
}

pub(crate) fn mapAnnotationBindingInArg(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static>,
) -> Result<(metamodelica::Ref<Absyn::ElementArg>, bool)> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let mut found: bool = false;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    let mut mod_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut mod_eq: metamodelica::Ref<Absyn::EqMod>;
    let mut rest_path: metamodelica::Ref<Absyn::Path>;
    let mut arg_path_len: i32;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__esc_mod), .. } => {
            r#mod = (*__esc_mod).clone();
            if pathPrefixOf(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), path.clone()) {
                arg_path_len = pathPartCount(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION), 0);
                if arg_path_len == pathPartCount(&path, 0) {
                    mod_eq = mapAnnotationBindingInEqMod(r#mod.eqMod.clone(), &*func)?;
                    assign_field!(r#mod.eqMod = mod_eq);
                    found = true;
                } else {
                    rest_path = Util::foldcallN(arg_path_len, &pathRest, path)?;
                    (mod_args, found) = List::findMap(r#mod.elementArgLst.clone(), &({ let __pe_b1 = rest_path; let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static> = func.clone(); move |__pe_a0| mapAnnotationBindingInArg(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
                    assign_field!(r#mod.elementArgLst = mod_args);
                }
                if found {
                    assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod.clone()));
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((arg, found))
}

pub(crate) fn mapAnnotationBindingInEqMod(
    mut eqMod: metamodelica::Ref<Absyn::EqMod>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::EqMod>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut eqMod: metamodelica::Ref<Absyn::EqMod> = eqMod;
    let () = (match &*eqMod {
        Absyn::EqMod::EQMOD { exp: __eqMod_exp, .. } => {
            assign_variant_field!(eqMod => Absyn::EqMod::EQMOD; exp = func(__eqMod_exp.clone())?);
            ()
        }
        _ => (),
    });
    Ok(eqMod)
}

pub fn createChoiceArray(
    mut inChoices: metamodelica::Ref<Absyn::ElementArg>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outChoices: metamodelica::Ref<Absyn::ElementArg> = inChoices.clone();
    let mut choices: metamodelica::Ref<Absyn::ElementArg>;
    let mut choice: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut c: metamodelica::Ref<Absyn::ElementArg>;
    let mut el: metamodelica::Ref<Absyn::ElementArg>;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo = Absyn::dummyInfo.clone();
    let mut cmt1: Option<ArcStr>;
    let mut cmt2: Option<ArcStr> = None;
    let mut fp1: bool;
    let mut fp2: bool = false;
    let mut ep1: Absyn::Each;
    let mut ep2: Absyn::Each = openmodelica_ast::Absyn::Each::NON_EACH;
    let mut choiceArray: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut s: ArcStr = arcstr::literal!("");
    let mut e: metamodelica::Ref<Absyn::Exp>;
    outChoices = (::match_deref::match_deref! { match &(inChoices.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: __esc_fp1, eachPrefix: __esc_ep1, path: Deref @ Absyn::Path::IDENT { name: Deref @ "choices" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: __esc_choice, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), comment: __esc_cmt1, info: __esc_info1 } => {
            fp1 = (*__esc_fp1).clone();
            ep1 = (*__esc_ep1).clone();
            choice = (*__esc_choice).clone();
            cmt1 = (*__esc_cmt1).clone();
            info1 = (*__esc_info1).clone();
            for mut m in &*choice.clone() {
                (choiceArray, acc) = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: __esc_fp2, eachPrefix: __esc_ep2, path: Deref @ Absyn::Path::IDENT { name: Deref @ "choice" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Cons { head: __esc_el, tail: Deref @ metamodelica::ListNode::Nil }, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), comment: __esc_cmt2, info: __esc_info2 } => {
            fp2 = (*__esc_fp2).clone();
            ep2 = (*__esc_ep2).clone();
            el = (*__esc_el).clone();
            cmt2 = (*__esc_cmt2).clone();
            info2 = (*__esc_info2).clone();
            s = Dump::unparseElementArgStr(el.clone())?;
            (metamodelica::cons(s, choiceArray), acc)
        },
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix: __esc_fp2, eachPrefix: __esc_ep2, path: Deref @ Absyn::Path::IDENT { name: Deref @ "choice" }, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __esc_e, .. } }), comment: __esc_cmt2, info: __esc_info2 } => {
            fp2 = (*__esc_fp2).clone();
            ep2 = (*__esc_ep2).clone();
            e = (*__esc_e).clone();
            cmt2 = (*__esc_cmt2).clone();
            info2 = (*__esc_info2).clone();
            s = Dump::printExpStr(e.clone())?;
            (metamodelica::cons(s, choiceArray), acc)
        },
        _ => (choiceArray, metamodelica::cons(m.clone(), acc)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if !((choiceArray).is_empty()) {
                e = metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
        for mut s in (choiceArray.reverse()).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Absyn::Exp::STRING { value: s.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) });
                c = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fp2, eachPrefix: ep2, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("choice") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: e, info: info2.clone() }) })), comment: cmt2, info: info2 });
                args = metamodelica::cons(c, acc).reverse();
            } else {
                args = acc.reverse();
            }
            choices = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fp1.clone(), eachPrefix: ep1.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("choices") }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: cmt1.clone(), info: info1.clone() });
            choices
        },
        _ => inChoices,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outChoices)
}

pub(crate) fn mapCrefExps(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_IDENT {
            subscripts: __cref_subscripts,
            ..
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
                    let __x = mapSubscriptExp(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ComponentRef::CREF_QUAL {
            subscripts: __cref_subscripts,
            ..
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; subscripts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
                    let __x = mapSubscriptExp(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = mapCrefExps(__cref_componentRef.clone(), func)?);
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub(crate) fn mapSubscriptExp(
    mut sub: metamodelica::Ref<Absyn::Subscript>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut sub: metamodelica::Ref<Absyn::Subscript> = sub;
    let () = (match &*sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = func(__sub_subscript.clone())?);
            ()
        }
        _ => (),
    });
    Ok(sub)
}

pub fn getElementConstrainingClass(
    mut element: &metamodelica::Ref<Absyn::Element>,
) -> Option<metamodelica::Ref<Absyn::ConstrainClass>> {
    let mut cc: Option<metamodelica::Ref<Absyn::ConstrainClass>>;
    cc = (match &**element {
        Absyn::Element::ELEMENT {
            constrainClass: __element_constrainClass,
            ..
        } => __element_constrainClass.clone(),
        _ => None,
    });
    cc
}

pub fn isElementReplaceable(mut element: &metamodelica::Ref<Absyn::Element>) -> bool {
    let mut res: bool;
    let mut redecl: Absyn::RedeclareKeywords;
    res = (match &**element {
        Absyn::Element::ELEMENT {
            redeclareKeywords: Some(__esc_redecl),
            ..
        } => {
            redecl = (*__esc_redecl).clone();
            (match redecl.clone() {
                Absyn::RedeclareKeywords::REPLACEABLE { .. } => true,
                Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. } => true,
                _ => false,
            })
        }
        _ => false,
    });
    res
}

pub fn isElementRedeclare(mut element: &metamodelica::Ref<Absyn::Element>) -> bool {
    let mut res: bool;
    let mut redecl: Absyn::RedeclareKeywords;
    res = (match &**element {
        Absyn::Element::ELEMENT {
            redeclareKeywords: Some(__esc_redecl),
            ..
        } => {
            redecl = (*__esc_redecl).clone();
            (match redecl.clone() {
                Absyn::RedeclareKeywords::REDECLARE { .. } => true,
                _ => false,
            })
        }
        _ => false,
    });
    res
}

pub fn isModel(mut cls: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (match &**cls {
        Absyn::Class {
            restriction: Absyn::Restriction::R_MODEL { .. },
            ..
        } => true,
        _ => false,
    });
    res
}

pub fn isBlock(mut cls: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (match &**cls {
        Absyn::Class {
            restriction: Absyn::Restriction::R_BLOCK { .. },
            ..
        } => true,
        _ => false,
    });
    res
}

pub fn isConnector(mut cls: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (match &**cls {
        Absyn::Class {
            restriction: Absyn::Restriction::R_CONNECTOR { .. },
            ..
        } => true,
        _ => false,
    });
    res
}

pub fn isExpandableConnector(mut cls: &metamodelica::Ref<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (match &**cls {
        Absyn::Class {
            restriction: Absyn::Restriction::R_EXP_CONNECTOR { .. },
            ..
        } => true,
        _ => false,
    });
    res
}

pub(crate) fn eachBool(mut eachPrefix: Absyn::Each) -> bool {
    let mut res: bool;
    res = (match eachPrefix {
        Absyn::Each::EACH { .. } => true,
        _ => false,
    });
    res
}

pub fn getElementAnnotation(
    mut element: &metamodelica::Ref<Absyn::Element>,
    mut name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    outAnnotation = (match &**element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => getElementSpecAnnotation(metamodelica::AsArg::as_arg(&__element_specification), name)?,
        _ => None,
    });
    Ok(outAnnotation)
}

pub(crate) fn getElementSpecAnnotation(
    mut spec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    outAnnotation = (match &**spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => getClassAnnotation(metamodelica::AsArg::as_arg(&__spec_class_))?,
        Absyn::ElementSpec::EXTENDS {
            annotationOpt: __spec_annotationOpt,
            ..
        } => __spec_annotationOpt.clone(),
        Absyn::ElementSpec::IMPORT {
            comment: __spec_comment,
            ..
        } => getCommentOptAnnotation(__spec_comment.clone())?,
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            ..
        } => getComponentItemsAnnotation(metamodelica::AsArg::as_arg(&__spec_components), name)?,
        _ => None,
    });
    Ok(outAnnotation)
}

pub(crate) fn getComponentItemsAnnotation(
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut oi: Option<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut i: metamodelica::Ref<Absyn::ComponentItem>;
    oi = List::findOption(
        items,
        &({
            let __pe_b0 = name.clone();
            move |__pe_a1| Ok(isComponentItemNamed(&__pe_b0, &__pe_a1))
        }),
    )?;
    if (oi).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(oi) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        i = metamodelica::Own::own(__pa0);
        outAnnotation = getCommentOptAnnotation(i.comment.clone())?;
    } else {
        outAnnotation = None;
    }
    Ok(outAnnotation)
}

pub fn getCommentOptAnnotation(
    mut commentOpt: Option<metamodelica::Ref<Absyn::Comment>>,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>;
    if (commentOpt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(commentOpt) {
            Some(Deref @ Absyn::Comment { annotation_: __pa0, .. }) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        outAnnotation = metamodelica::Own::own(__pa0);
    } else {
        outAnnotation = None;
    }
    Ok(outAnnotation)
}

pub fn getCommentOptComment(mut commentOpt: Option<metamodelica::Ref<Absyn::Comment>>) -> Result<Option<ArcStr>> {
    let mut outComment: Option<ArcStr>;
    if (commentOpt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(commentOpt) {
            Some(Deref @ Absyn::Comment { comment: __pa0, .. }) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        outComment = metamodelica::Own::own(__pa0);
    } else {
        outComment = None;
    }
    Ok(outComment)
}

pub fn setElementAnnotation(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut name: &ArcStr,
    mut inAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = setElementSpecAnnotation(__element_specification.clone(), name, inAnnotation)?);
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn setElementSpecAnnotation(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut name: &ArcStr,
    mut inAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = setClassAnnotation(__spec_class_.clone(), inAnnotation)?);
            ()
        }
        Absyn::ElementSpec::EXTENDS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS; annotationOpt = inAnnotation);
            ()
        }
        Absyn::ElementSpec::IMPORT {
            comment: __spec_comment,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; comment = setCommentAnnotation(__spec_comment.clone(), inAnnotation)?);
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = List::findAndMap(__spec_components.clone(), &({ let __pe_b0 = name.clone(); move |__pe_a1| Ok(isComponentItemNamed(&__pe_b0, &__pe_a1)) }), &({ let __pe_b1 = inAnnotation; move |__pe_a0| setComponentItemAnnotation(__pe_a0, __pe_b1.clone()) }))?.0);
            ()
        }
        _ => (),
    });
    Ok(spec)
}

pub fn setComponentItemAnnotation(
    mut item: metamodelica::Ref<Absyn::ComponentItem>,
    mut inAnnotation: Option<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut item: metamodelica::Ref<Absyn::ComponentItem> = item;
    assign_field!(item.comment = setCommentAnnotation(item.comment.clone(), inAnnotation)?);
    Ok(item)
}

pub fn isImpure(mut purity: Absyn::FunctionPurity, mut defaultImpure: bool) -> bool {
    let mut isImpure: bool;
    isImpure = (match purity {
        Absyn::FunctionPurity::IMPURE { .. } => true,
        Absyn::FunctionPurity::NO_PURITY { .. } => defaultImpure,
        _ => false,
    });
    isImpure
}

pub(crate) fn purityEqual(
    mut purity1: Absyn::FunctionPurity,
    mut purity2: Absyn::FunctionPurity,
    mut defaultImpure: bool,
) -> bool {
    let mut isEqual: bool;
    if metamodelica::valueConstructor((&purity1)).unwrap() == metamodelica::valueConstructor((&purity2)).unwrap() {
        isEqual = true;
    } else if defaultImpure {
        isEqual = (match (purity1, purity2) {
            (Absyn::FunctionPurity::NO_PURITY { .. }, Absyn::FunctionPurity::IMPURE { .. }) => true,
            (Absyn::FunctionPurity::IMPURE { .. }, Absyn::FunctionPurity::NO_PURITY { .. }) => true,
            _ => false,
        });
    } else {
        isEqual = (match (purity1, purity2) {
            (Absyn::FunctionPurity::NO_PURITY { .. }, Absyn::FunctionPurity::PURE { .. }) => true,
            (Absyn::FunctionPurity::PURE { .. }, Absyn::FunctionPurity::NO_PURITY { .. }) => true,
            _ => false,
        });
    }
    isEqual
}

pub fn isElementSection(mut part: &metamodelica::Ref<Absyn::ClassPart>) -> bool {
    let mut res: bool;
    res = (match &**part {
        Absyn::ClassPart::PUBLIC { .. } => true,
        Absyn::ClassPart::PROTECTED { .. } => true,
        _ => false,
    });
    res
}

pub fn isEquationSection(mut part: &metamodelica::Ref<Absyn::ClassPart>) -> bool {
    let mut res: bool;
    res = (match &**part {
        Absyn::ClassPart::EQUATIONS { .. } => true,
        Absyn::ClassPart::INITIALEQUATIONS { .. } => true,
        _ => false,
    });
    res
}

pub fn isAlgorithmSection(mut part: &metamodelica::Ref<Absyn::ClassPart>) -> bool {
    let mut res: bool;
    res = (match &**part {
        Absyn::ClassPart::ALGORITHMS { .. } => true,
        Absyn::ClassPart::INITIALALGORITHMS { .. } => true,
        _ => false,
    });
    res
}

pub fn getEquationItemsInPart(
    mut part: &metamodelica::Ref<Absyn::ClassPart>,
) -> metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    eqs = (match &**part {
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => __part_contents.clone(),
        Absyn::ClassPart::INITIALEQUATIONS {
            contents: __part_contents,
        } => __part_contents.clone(),
        _ => metamodelica::nil(),
    });
    eqs
}

pub fn setEquationItemsInPart(
    mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut part: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let () = (match &*part {
        Absyn::ClassPart::EQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = eqs);
            ()
        }
        Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = eqs);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(part)
}

pub fn setElementType(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut typeSpec: metamodelica::Ref<Absyn::TypeSpec>,
    mut allowMultipleComponents: bool,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = setElementSpecType(__element_specification.clone(), typeSpec, allowMultipleComponents)?);
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn setElementSpecType(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut typeSpec: metamodelica::Ref<Absyn::TypeSpec>,
    mut allowMultipleComponents: bool,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = setClassType(__spec_class_.clone(), typeSpec)?);
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            ..
        } if (allowMultipleComponents || ((__spec_components).len() as i32) == 1) => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; typeSpec = typeSpec);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(spec)
}

pub(crate) fn setClassType(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut typeSpec: metamodelica::Ref<Absyn::TypeSpec>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    assign_field!(cls.body = setClassDefType(cls.body.clone(), typeSpec)?);
    Ok(cls)
}

pub(crate) fn setClassDefType(
    mut cdef: metamodelica::Ref<Absyn::ClassDef>,
    mut typeSpec: metamodelica::Ref<Absyn::TypeSpec>,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef;
    let () = (match &*cdef {
        Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; typeSpec = typeSpec);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cdef)
}

pub fn isLiteralExp(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<bool> {
    let mut literal: bool;
    literal = (match &**exp {
        Absyn::Exp::INTEGER { .. } => true,
        Absyn::Exp::REAL { .. } => true,
        Absyn::Exp::STRING { .. } => true,
        Absyn::Exp::BOOL { .. } => true,
        Absyn::Exp::ARRAY {
            arrayExp: __exp_arrayExp,
        } => List::all(
            metamodelica::AsArg::as_arg(&__exp_arrayExp),
            &move |__a0: metamodelica::Ref<Absyn::Exp>| isLiteralExp(&__a0),
        )?,
        Absyn::Exp::MATRIX { matrix: __exp_matrix } => {
            literal = true;
            for mut row in &*__exp_matrix.clone() {
                literal = literal
                    && List::all(metamodelica::AsArg::as_arg(&row), &move |__a0: metamodelica::Ref<
                        Absyn::Exp,
                    >| {
                        isLiteralExp(&__a0)
                    })?;
                if !(literal) {
                    break;
                }
            }
            literal
        }
        Absyn::Exp::RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
        } => {
            isLiteralExp(metamodelica::AsArg::as_arg(&__exp_start))?
                && Util::applyOptionOrDefault(
                    __exp_step.clone(),
                    &move |__a0: metamodelica::Ref<Absyn::Exp>| isLiteralExp(&__a0),
                    true,
                )?
                && isLiteralExp(metamodelica::AsArg::as_arg(&__exp_stop))?
        }
        _ => false,
    });
    Ok(literal)
}

pub fn enumLiteralName(mut literal: &metamodelica::Ref<Absyn::EnumLiteral>) -> ArcStr {
    let mut name: ArcStr = literal.literal.clone();
    name
}

pub fn elementItemClass(mut item: &metamodelica::Ref<Absyn::ElementItem>) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let __pa0 = ::match_deref::match_deref! { match &((*item)) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cls = metamodelica::Own::own(__pa0);
    Ok(cls)
}

pub fn classDefStringComment(mut def: &metamodelica::Ref<Absyn::ClassDef>) -> ArcStr {
    let mut comment: ArcStr;
    comment = (::match_deref::match_deref! { match def {
        Deref @ Absyn::ClassDef::PARTS { comment: Some(__esc_comment), .. } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        Deref @ Absyn::ClassDef::DERIVED { comment: Some(Deref @ Absyn::Comment { comment: Some(__esc_comment), .. }), .. } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { comment: Some(Deref @ Absyn::Comment { comment: Some(__esc_comment), .. }), .. } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { comment: Some(Deref @ Absyn::Comment { comment: Some(__esc_comment), .. }), .. } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: Some(__esc_comment), .. } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        Deref @ Absyn::ClassDef::PDER { comment: Some(Deref @ Absyn::Comment { comment: Some(__esc_comment), .. }), .. } => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    comment
}

pub fn appendEquation(
    mut eq: metamodelica::Ref<Absyn::EquationItem>,
    mut isInitial: bool,
    mut cls: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    fn append_eq(
        mut eq: metamodelica::Ref<Absyn::EquationItem>,
        mut isInitial: bool,
        mut part: metamodelica::Ref<Absyn::ClassPart>,
    ) -> (metamodelica::Ref<Absyn::ClassPart>, bool) {
        let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
        let mut found: bool;
        found = (match &*part {
            Absyn::ClassPart::EQUATIONS {
                contents: __part_contents,
            } if (!(isInitial)) => {
                assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = List::appendElt(eq, __part_contents.clone()));
                true
            }
            Absyn::ClassPart::INITIALEQUATIONS {
                contents: __part_contents,
            } if (isInitial) => {
                assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = List::appendElt(eq, __part_contents.clone()));
                true
            }
            _ => false,
        });
        (part, found)
    }

    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut found: bool;
    parts = getClassPartsInClass(&cls).reverse();
    (parts, found) = List::findMap(
        parts,
        &({
            let __pe_b0 = eq.clone();
            let __pe_b1 = isInitial;
            move |__pe_a2| Ok(append_eq(__pe_b0.clone(), __pe_b1.clone(), __pe_a2))
        }),
    )?;
    if !(found) {
        parts = if (isInitial) {
            metamodelica::cons(
                metamodelica::Ref::new(Absyn::ClassPart::INITIALEQUATIONS { contents: list![eq] }),
                parts,
            )
        } else {
            metamodelica::cons(
                metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: list![eq] }),
                parts,
            )
        };
    }
    cls = setClassPartsInClass(parts.reverse(), cls)?;
    Ok(cls)
}

pub(crate) fn forIteratorEqual(
    mut iter1: &metamodelica::Ref<Absyn::ForIterator>,
    mut iter2: &metamodelica::Ref<Absyn::ForIterator>,
) -> Result<bool> {
    let mut equal: bool = metamodelica::stringEq(&iter1.name, &iter2.name)
        && Util::optionEqual(iter1.guardExp.clone(), iter2.guardExp.clone(), &expEqual)?
        && Util::optionEqual(iter1.range.clone(), iter2.range.clone(), &expEqual)?;
    Ok(equal)
}

pub(crate) fn functionArgsEqual(
    mut args1: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut args2: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<bool> {
    fn named_arg_equal(
        mut arg1: &metamodelica::Ref<Absyn::NamedArg>,
        mut arg2: &metamodelica::Ref<Absyn::NamedArg>,
    ) -> Result<bool> {
        let mut equal: bool = metamodelica::stringEq(&arg1.argName, &arg2.argName)
            && expEqual(arg1.argValue.clone(), arg2.argValue.clone())?;
        Ok(equal)
    }

    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (args1, args2) {
        (Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. }, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. }) => List::isEqualOnTrue(var_field!((**args1).args, Absyn::FunctionArgs::FUNCTIONARGS).clone(), var_field!((**args2).args, Absyn::FunctionArgs::FUNCTIONARGS).clone(), &expEqual)? && List::isEqualOnTrue(var_field!((**args1).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), var_field!((**args2).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), &move |__a0: metamodelica::Ref<Absyn::NamedArg>, __a1: metamodelica::Ref<Absyn::NamedArg>| named_arg_equal(&__a0, &__a1))?,
        (Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }, Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }) => expEqual(var_field!((**args1).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), var_field!((**args2).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone())? && var_field!((**args1).iterType, Absyn::FunctionArgs::FOR_ITER_FARG).clone() == var_field!((**args2).iterType, Absyn::FunctionArgs::FOR_ITER_FARG).clone() && List::isEqualOnTrue(var_field!((**args1).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), var_field!((**args2).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), &move |__a0: metamodelica::Ref<Absyn::ForIterator>, __a1: metamodelica::Ref<Absyn::ForIterator>| forIteratorEqual(&__a0, &__a1))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub(crate) fn commentEqual(
    mut cmt1: &metamodelica::Ref<Absyn::Comment>,
    mut cmt2: &metamodelica::Ref<Absyn::Comment>,
) -> Result<bool> {
    let mut equal: bool = Util::optionEqual(
        cmt1.comment.clone(),
        cmt2.comment.clone(),
        &fnptr!(stringEq, ArcStr, ArcStr),
    )? && Util::optionEqual(
        cmt1.annotation_.clone(),
        cmt2.annotation_.clone(),
        &move |__a0: metamodelica::Ref<Absyn::Annotation>, __a1: metamodelica::Ref<Absyn::Annotation>| {
            annotationEqual(&__a0, &__a1)
        },
    )?;
    Ok(equal)
}

pub(crate) fn annotationEqual(
    mut ann1: &metamodelica::Ref<Absyn::Annotation>,
    mut ann2: &metamodelica::Ref<Absyn::Annotation>,
) -> Result<bool> {
    let mut equal: bool = List::isEqualOnTrue(
        ann1.elementArgs.clone(),
        ann2.elementArgs.clone(),
        &move |__a0: metamodelica::Ref<Absyn::ElementArg>, __a1: metamodelica::Ref<Absyn::ElementArg>| {
            elementArgEqual(&__a0, &__a1)
        },
    )?;
    Ok(equal)
}

pub(crate) fn elementArgEqual(
    mut arg1: &metamodelica::Ref<Absyn::ElementArg>,
    mut arg2: &metamodelica::Ref<Absyn::ElementArg>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (arg1, arg2) {
        (Deref @ Absyn::ElementArg::MODIFICATION { .. }, Deref @ Absyn::ElementArg::MODIFICATION { .. }) => var_field!((**arg1).finalPrefix, Absyn::ElementArg::MODIFICATION).clone() == var_field!((**arg2).finalPrefix, Absyn::ElementArg::MODIFICATION).clone() && var_field!((**arg1).eachPrefix, Absyn::ElementArg::MODIFICATION).clone() == var_field!((**arg2).eachPrefix, Absyn::ElementArg::MODIFICATION).clone() && pathEqual(var_field!((**arg1).path, Absyn::ElementArg::MODIFICATION), var_field!((**arg2).path, Absyn::ElementArg::MODIFICATION)) && Util::optionEqual(var_field!((**arg1).modification, Absyn::ElementArg::MODIFICATION).clone(), var_field!((**arg2).modification, Absyn::ElementArg::MODIFICATION).clone(), &move |__a0: metamodelica::Ref<Absyn::Modification>, __a1: metamodelica::Ref<Absyn::Modification>| modEqual(&__a0, &__a1))? && Util::optionEqual(var_field!((**arg1).comment, Absyn::ElementArg::MODIFICATION).clone(), var_field!((**arg2).comment, Absyn::ElementArg::MODIFICATION).clone(), &fnptr!(stringEq, ArcStr, ArcStr))?,
        (Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. }, Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. }) => metamodelica::stringEq(&var_field!((**arg1).comment, Absyn::ElementArg::ELEMENTARGCOMMENT), &var_field!((**arg2).comment, Absyn::ElementArg::ELEMENTARGCOMMENT)),
        (Deref @ Absyn::ElementArg::INHERITANCEBREAK { .. }, Deref @ Absyn::ElementArg::INHERITANCEBREAK { .. }) => equationEqual(var_field!((**arg1).cnct, Absyn::ElementArg::INHERITANCEBREAK), var_field!((**arg2).cnct, Absyn::ElementArg::INHERITANCEBREAK), false, true)?,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynUtil.elementArgEqual")); __mm_s.push_str(&*literal!(" got unknown element.")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("FrontEnd/AbsynUtil.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub(crate) fn modEqual(
    mut mod1: &metamodelica::Ref<Absyn::Modification>,
    mut mod2: &metamodelica::Ref<Absyn::Modification>,
) -> Result<bool> {
    let mut equal: bool = eqModEqual(&mod1.eqMod, &mod2.eqMod)?
        && List::isEqualOnTrue(
            mod1.elementArgLst.clone(),
            mod2.elementArgLst.clone(),
            &move |__a0: metamodelica::Ref<Absyn::ElementArg>, __a1: metamodelica::Ref<Absyn::ElementArg>| {
                elementArgEqual(&__a0, &__a1)
            },
        )?;
    Ok(equal)
}

pub(crate) fn eqModEqual(
    mut eqMod1: &metamodelica::Ref<Absyn::EqMod>,
    mut eqMod2: &metamodelica::Ref<Absyn::EqMod>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (eqMod1, eqMod2) {
        (Deref @ Absyn::EqMod::NOMOD { .. }, Deref @ Absyn::EqMod::NOMOD { .. }) => true,
        (Deref @ Absyn::EqMod::EQMOD { .. }, Deref @ Absyn::EqMod::EQMOD { .. }) => expEqual(var_field!((**eqMod1).exp, Absyn::EqMod::EQMOD).clone(), var_field!((**eqMod2).exp, Absyn::EqMod::EQMOD).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn equationItemEqual(
    mut eq1: &metamodelica::Ref<Absyn::EquationItem>,
    mut eq2: &metamodelica::Ref<Absyn::EquationItem>,
    mut shallow: bool,
    mut ignoreComment: bool,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (eq1, eq2) {
        (Deref @ Absyn::EquationItem::EQUATIONITEM { .. }, Deref @ Absyn::EquationItem::EQUATIONITEM { .. }) => equationEqual(var_field!((**eq1).equation_, Absyn::EquationItem::EQUATIONITEM), var_field!((**eq2).equation_, Absyn::EquationItem::EQUATIONITEM), shallow, true)? && (ignoreComment || Util::optionEqual(var_field!((**eq1).comment, Absyn::EquationItem::EQUATIONITEM).clone(), var_field!((**eq2).comment, Absyn::EquationItem::EQUATIONITEM).clone(), &move |__a0: metamodelica::Ref<Absyn::Comment>, __a1: metamodelica::Ref<Absyn::Comment>| commentEqual(&__a0, &__a1))?),
        (Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { .. }, Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { .. }) => metamodelica::stringEq(&var_field!((**eq1).comment, Absyn::EquationItem::EQUATIONITEMCOMMENT), &var_field!((**eq2).comment, Absyn::EquationItem::EQUATIONITEMCOMMENT)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub(crate) fn equationItemsEqual(
    mut eql1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut eql2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut shallow: bool,
    mut ignoreComment: bool,
) -> Result<bool> {
    let mut equal: bool = List::isEqualOnTrue(
        eql1.clone(),
        eql2.clone(),
        &({
            let __pe_b2 = shallow;
            let __pe_b3 = ignoreComment;
            move |__pe_a0, __pe_a1| equationItemEqual(&__pe_a0, &__pe_a1, __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    Ok(equal)
}

pub(crate) fn equationEqual(
    mut eq1: &metamodelica::Ref<Absyn::Equation>,
    mut eq2: &metamodelica::Ref<Absyn::Equation>,
    mut shallow: bool,
    mut ignoreComment: bool,
) -> Result<bool> {
    fn branch_eq(
        mut branch1: (
            metamodelica::Ref<Absyn::Exp>,
            metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
        ),
        mut branch2: (
            metamodelica::Ref<Absyn::Exp>,
            metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
        ),
        mut shallow: bool,
        mut ignoreComment: bool,
    ) -> Result<bool> {
        let mut equal: bool = expEqual(Util::tuple21(branch1.clone()), Util::tuple21(branch2.clone()))?
            && (shallow
                || equationItemsEqual(
                    Util::tuple22(branch1.clone()),
                    Util::tuple22(branch2.clone()),
                    false,
                    ignoreComment,
                )?);
        Ok(equal)
    }

    let mut equal: bool;
    let mut e1: metamodelica::Ref<Absyn::Exp>;
    let mut e2: metamodelica::Ref<Absyn::Exp>;
    let mut eql1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut eql2: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    )>;
    let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
    let mut iters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
    let mut args: metamodelica::Ref<Absyn::FunctionArgs>;
    let mut eq: metamodelica::Ref<Absyn::EquationItem>;
    if metamodelica::valueConstructor((&*&**eq1))? != metamodelica::valueConstructor((&*&**eq2))? {
        equal = false;
        return Ok(equal);
    }
    equal = (match &**eq1 {
        Absyn::Equation::EQ_IF {
            elseIfBranches: __eq1_elseIfBranches,
            equationElseItems: __eq1_equationElseItems,
            equationTrueItems: __eq1_equationTrueItems,
            ifExp: __eq1_ifExp,
        } => {
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_IF { ifExp: __pa0, equationTrueItems: __pa1, elseIfBranches: __pa2, equationElseItems: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            eql1 = metamodelica::Own::own(__pa1);
            branches = metamodelica::Own::own(__pa2);
            eql2 = metamodelica::Own::own(__pa3);
            expEqual(__eq1_ifExp.clone(), e1)?
                && (shallow || equationItemsEqual(__eq1_equationTrueItems.clone(), eql1, false, true)?)
                && List::isEqualOnTrue(
                    __eq1_elseIfBranches.clone(),
                    branches,
                    &({
                        let __pe_b2 = shallow;
                        let __pe_b3 = ignoreComment;
                        move |__pe_a0, __pe_a1| branch_eq(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                    }),
                )?
                && (shallow || equationItemsEqual(__eq1_equationElseItems.clone(), eql2, false, ignoreComment)?)
        }
        Absyn::Equation::EQ_EQUALS {
            leftSide: __eq1_leftSide,
            rightSide: __eq1_rightSide,
        } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: __pa0, rightSide: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            e2 = metamodelica::Own::own(__pa1);
            expEqual(__eq1_leftSide.clone(), e1)? && expEqual(__eq1_rightSide.clone(), e2)?
        }
        Absyn::Equation::EQ_PDE {
            domain: __eq1_domain,
            leftSide: __eq1_leftSide,
            rightSide: __eq1_rightSide,
        } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_PDE { leftSide: __pa0, rightSide: __pa1, domain: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            e2 = metamodelica::Own::own(__pa1);
            cr1 = metamodelica::Own::own(__pa2);
            expEqual(__eq1_leftSide.clone(), e1)?
                && expEqual(__eq1_rightSide.clone(), e2)?
                && crefEqual(metamodelica::AsArg::as_arg(&__eq1_domain), &cr1)?
        }
        Absyn::Equation::EQ_CONNECT {
            connector1: __eq1_connector1,
            connector2: __eq1_connector2,
        } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_CONNECT { connector1: __pa0, connector2: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr1 = metamodelica::Own::own(__pa0);
            cr2 = metamodelica::Own::own(__pa1);
            crefEqual(metamodelica::AsArg::as_arg(&__eq1_connector1), &cr1)?
                && crefEqual(metamodelica::AsArg::as_arg(&__eq1_connector2), &cr2)?
        }
        Absyn::Equation::EQ_FOR {
            forEquations: __eq1_forEquations,
            iterators: __eq1_iterators,
        } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_FOR { iterators: __pa0, forEquations: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            iters = metamodelica::Own::own(__pa0);
            eql1 = metamodelica::Own::own(__pa1);
            List::isEqualOnTrue(__eq1_iterators.clone(), iters, &move |__a0: metamodelica::Ref<
                Absyn::ForIterator,
            >,
                                                                       __a1: metamodelica::Ref<
                Absyn::ForIterator,
            >| {
                forIteratorEqual(&__a0, &__a1)
            })? && (shallow || equationItemsEqual(__eq1_forEquations.clone(), eql1, false, ignoreComment)?)
        }
        Absyn::Equation::EQ_WHEN_E {
            elseWhenEquations: __eq1_elseWhenEquations,
            whenEquations: __eq1_whenEquations,
            whenExp: __eq1_whenExp,
        } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_WHEN_E { whenExp: __pa0, whenEquations: __pa1, elseWhenEquations: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            eql1 = metamodelica::Own::own(__pa1);
            branches = metamodelica::Own::own(__pa2);
            expEqual(__eq1_whenExp.clone(), e1)?
                && equationItemsEqual(__eq1_whenEquations.clone(), eql1, false, true)?
                && List::isEqualOnTrue(
                    __eq1_elseWhenEquations.clone(),
                    branches,
                    &({
                        let __pe_b2 = shallow;
                        let __pe_b3 = ignoreComment;
                        move |__pe_a0, __pe_a1| branch_eq(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                    }),
                )?
        }
        Absyn::Equation::EQ_NORETCALL {
            functionArgs: __eq1_functionArgs,
            functionName: __eq1_functionName,
        } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_NORETCALL { functionName: __pa0, functionArgs: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr1 = metamodelica::Own::own(__pa0);
            args = metamodelica::Own::own(__pa1);
            crefEqual(metamodelica::AsArg::as_arg(&__eq1_functionName), &cr1)?
                && functionArgsEqual(metamodelica::AsArg::as_arg(&__eq1_functionArgs), &args)?
        }
        Absyn::Equation::EQ_FAILURE { equ: __eq1_equ } => {
            let __pa0 = ::match_deref::match_deref! { match &((*eq2)) {
                Deref @ Absyn::Equation::EQ_FAILURE { equ: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            eq = metamodelica::Own::own(__pa0);
            shallow || equationItemEqual(metamodelica::AsArg::as_arg(&__eq1_equ), &eq, false, ignoreComment)?
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("AbsynUtil.equationEqual"));
                    __mm_s.push_str(&*literal!(" got unknown equation."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("FrontEnd/AbsynUtil.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(equal)
}
