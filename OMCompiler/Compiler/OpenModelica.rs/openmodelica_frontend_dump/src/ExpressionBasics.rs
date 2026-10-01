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

use crate::AbsynUtil;
use crate::ComponentReferenceBasics;
use crate::ExpressionDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
pub fn printExpStr(mut e: metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = Tpl::tplString2(
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
    Ok(s)
}

pub fn dimensionString(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**dim {
        DAE::Dimension::DIM_UNKNOWN { .. } => {
            literal!(":")
        }
        DAE::Dimension::DIM_ENUM { enumTypeName: p, .. } => {
            let mut s: ArcStr;
            s = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
            s
        }
        DAE::Dimension::DIM_BOOLEAN { .. } => {
            literal!("Boolean")
        }
        DAE::Dimension::DIM_INTEGER { integer: x } => {
            let mut s: ArcStr;
            s = intString(x.clone());
            s
        }
        DAE::Dimension::DIM_EXP { exp: e } => {
            let mut s: ArcStr;
            s = printExpStr(e.clone())?;
            s
        }
    });
    Ok(r#str)
}

pub fn dimensionsString(mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        List::map(dims, &move |__a0: metamodelica::Ref<DAE::Dimension>| {
            dimensionString(&__a0)
        })?,
        literal!(","),
    );
    Ok(r#str)
}

pub(crate) fn shouldParenthesize(
    mut inOperand: &metamodelica::Ref<DAE::Exp>,
    mut inOperator: &metamodelica::Ref<DAE::Exp>,
    mut inLhs: bool,
) -> Result<bool> {
    let mut outShouldParenthesize: bool;
    outShouldParenthesize = (match &**inOperand {
        DAE::Exp::UNARY { .. } => true,
        _ => {
            let mut diff: i32;
            diff = Util::intCompare(priority(inOperand, inLhs)?, priority(inOperator, inLhs)?);
            shouldParenthesize2(diff, inOperand, inLhs)
        }
    });
    Ok(outShouldParenthesize)
}

fn shouldParenthesize2(mut inPrioDiff: i32, mut inOperand: &metamodelica::Ref<DAE::Exp>, mut inLhs: bool) -> bool {
    let mut outShouldParenthesize: bool;
    outShouldParenthesize = (match inPrioDiff {
        1 => true,
        0 => {
            if (inLhs) {
                isNonAssociativeExp(inOperand)
            } else {
                !(isAssociativeExp(inOperand))
            }
        }
        _ => false,
    });
    outShouldParenthesize
}

fn isAssociativeExp(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsAssociative: bool;
    outIsAssociative = (match &**inExp {
        DAE::Exp::BINARY { operator: op, .. } => isAssociativeOp(op),
        DAE::Exp::LBINARY { .. } => true,
        _ => false,
    });
    outIsAssociative
}

fn isAssociativeOp(mut inOperator: &DAE::Operator) -> bool {
    let mut outIsAssociative: bool;
    outIsAssociative = (match inOperator.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::ADD_ARR { .. } => true,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => true,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    outIsAssociative
}

fn isNonAssociativeExp(mut exp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (match &**exp {
        DAE::Exp::BINARY {
            operator: __exp_operator,
            ..
        } => isNonAssociativeOp(metamodelica::AsArg::as_arg(&__exp_operator)),
        _ => false,
    });
    isNonAssociative
}

fn isNonAssociativeOp(mut inOperator: &DAE::Operator) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (match inOperator.clone() {
        DAE::Operator::POW { .. } => true,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => true,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => true,
        DAE::Operator::POW_ARR { .. } => true,
        DAE::Operator::POW_ARR2 { .. } => true,
        _ => false,
    });
    isNonAssociative
}

pub fn priority(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inLhs: bool) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (::match_deref::match_deref! { match &((&**inExp, inLhs)) {
        (Deref @ DAE::Exp::BINARY { operator: op, .. }, false) => {
            priorityBinopRhs(metamodelica::AsArg::as_arg(&op))?
        },
        (Deref @ DAE::Exp::BINARY { operator: op, .. }, true) => {
            priorityBinopLhs(metamodelica::AsArg::as_arg(&op))?
        },
        (Deref @ DAE::Exp::RCONST { .. }, _) if (var_field!((**inExp).real, DAE::Exp::RCONST).clone() < metamodelica::OrderedFloat(0.0_f64)) => {
            4
        },
        (Deref @ DAE::Exp::UNARY { .. }, _) => {
            4
        },
        (Deref @ DAE::Exp::LBINARY { operator: op, .. }, _) => {
            priorityLBinop(metamodelica::AsArg::as_arg(&op))?
        },
        (Deref @ DAE::Exp::LUNARY { .. }, _) => {
            7
        },
        (Deref @ DAE::Exp::RELATION { .. }, _) => {
            6
        },
        (Deref @ DAE::Exp::RANGE { .. }, _) => {
            10
        },
        (Deref @ DAE::Exp::IFEXP { .. }, _) => {
            11
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPriority)
}

fn priorityBinopLhs(mut inOp: &DAE::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp.clone() {
        DAE::Operator::ADD { .. } => 5,
        DAE::Operator::SUB { .. } => 5,
        DAE::Operator::MUL { .. } => 2,
        DAE::Operator::DIV { .. } => 2,
        DAE::Operator::POW { .. } => 1,
        DAE::Operator::ADD_ARR { .. } => 5,
        DAE::Operator::SUB_ARR { .. } => 5,
        DAE::Operator::MUL_ARR { .. } => 2,
        DAE::Operator::DIV_ARR { .. } => 2,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => 2,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => 5,
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => 5,
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => 2,
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => 2,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => 2,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => 2,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => 1,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => 1,
        DAE::Operator::POW_ARR { .. } => 1,
        DAE::Operator::POW_ARR2 { .. } => 1,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityBinopRhs(mut inOp: &DAE::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp.clone() {
        DAE::Operator::ADD { .. } => 6,
        DAE::Operator::SUB { .. } => 5,
        DAE::Operator::MUL { .. } => 3,
        DAE::Operator::DIV { .. } => 2,
        DAE::Operator::POW { .. } => 1,
        DAE::Operator::ADD_ARR { .. } => 6,
        DAE::Operator::SUB_ARR { .. } => 5,
        DAE::Operator::MUL_ARR { .. } => 3,
        DAE::Operator::DIV_ARR { .. } => 2,
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => 3,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => 6,
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => 5,
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => 3,
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => 3,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => 2,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => 2,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => 1,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => 1,
        DAE::Operator::POW_ARR { .. } => 1,
        DAE::Operator::POW_ARR2 { .. } => 1,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityLBinop(mut inOp: &DAE::Operator) -> Result<i32> {
    let mut outPriority: i32;
    outPriority = (match inOp.clone() {
        DAE::Operator::AND { .. } => 8,
        DAE::Operator::OR { .. } => 9,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPriority)
}

pub fn evalCat<Exp: Clone + 'static + metamodelica::gc::MMTrace>(
    mut dim: i32,
    mut exps: metamodelica::List<Exp>,
    mut getArrayContents: &dyn ::std::ops::Fn(Exp) -> Result<metamodelica::List<Exp>>,
    mut toString: &dyn ::std::ops::Fn(Exp) -> Result<ArcStr>,
) -> Result<(metamodelica::List<Exp>, metamodelica::List<i32>)> {
    pub type GetArrayContents<Exp: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<metamodelica::List<Exp>> + 'static>;

    pub type MakeArrayFromList<Exp: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<Exp>) -> Result<Exp> + 'static>;

    pub type ToString<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<ArcStr> + 'static>;

    let mut outExps: metamodelica::List<Exp>;
    let mut outDims: metamodelica::List<i32>;
    let mut arr: metamodelica::List<Exp>;
    let mut arrs: metamodelica::List<metamodelica::List<Exp>> = metamodelica::nil();
    let mut dims: metamodelica::List<i32>;
    let mut firstDims: metamodelica::List<i32> = metamodelica::nil();
    let mut lastDims: metamodelica::List<i32>;
    let mut reverseDims: metamodelica::List<i32>;
    let mut dimsLst: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut j: i32;
    let mut k: i32;
    let mut l: i32;
    let mut thisDim: i32;
    let mut lastDim: i32;
    let mut expArr: metamodelica::Array<Exp>;
    let true = (dim >= 1) else {
        return Err("pattern mismatch");
    };
    let false = ((exps).is_empty()) else {
        return Err("pattern mismatch");
    };
    if 1 == dim {
        outExps = ({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut e in (exps.reverse()).into_iter().cloned() {
                let __x = getArrayContents(e.clone())?;
                __acc = __x.append(&__acc);
            }
            __acc
        });
        outDims = list![((outExps).len() as i32)];
        return Ok((outExps, outDims));
    }
    for mut e in &*exps.clone().reverse() {
        (arr, dims) = evalCatGetFlatArray(e.clone(), dim, getArrayContents, toString)?;
        arrs = metamodelica::cons(arr, arrs);
        dimsLst = metamodelica::cons(dims, dimsLst);
    }
    for mut i in 1..=dim - 1 {
        j = ({
            let mut __acc: Option<i32> = None;
            for mut d in (dimsLst.clone()).into_iter().cloned() {
                let __x = (d).head().cloned()?;
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
            __acc.unwrap_or(i32::MAX)
        });
        if j != ({
            let mut __acc: Option<i32> = None;
            for mut d in (dimsLst.clone()).into_iter().cloned() {
                let __x = (d).head().cloned()?;
                __acc = Some(match __acc {
                    None => __x,
                    Some(__cur) => {
                        if __x > __cur {
                            __x
                        } else {
                            __cur
                        }
                    }
                });
            }
            __acc.unwrap_or((-i32::MAX))
        }) {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ExpressionBasics.evalCat"));
                    __mm_s.push_str(&*literal!(": cat got uneven dimensions for dim="));
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{}", i)));
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut e in (exps.clone()).into_iter().cloned() {
                                let __x = toString(e.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(", "),
                    ));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("FrontEnd/ExpressionBasics.mo")),
            )?;
        }
        firstDims = metamodelica::cons(j, firstDims);
        dimsLst = ({
            let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
            for mut d in (dimsLst).into_iter().cloned() {
                let __x = (d).rest()?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    reverseDims = firstDims.clone();
    firstDims = firstDims.reverse();
    lastDims = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut d in (dimsLst).into_iter().cloned() {
            let __x = (d).head().cloned()?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    lastDim = ({
        let mut __acc: i32 = 0;
        for mut d in (lastDims.clone()).into_iter().cloned() {
            let __x = d.clone();
            __acc += __x;
        }
        __acc
    });
    reverseDims = metamodelica::cons(lastDim, reverseDims);
    expArr = metamodelica::arrayCreate(
        lastDim
            * ({
                let mut __acc: i32 = 1;
                for mut d in (firstDims).into_iter().cloned() {
                    let __x = d.clone();
                    __acc *= __x;
                }
                __acc
            }),
        (exps).head().cloned()?,
    );
    k = 1;
    for mut exps in &*arrs {
        let mut exps = exps.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lastDims) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        thisDim = metamodelica::Own::own(__pa0);
        lastDims = metamodelica::Own::own(__pa1);
        l = 0;
        for mut e in &*exps {
            unsafe {
                metamodelica::Dangerous::arrayInitSlotChecked(
                    expArr.clone(),
                    k + intMod(l, thisDim) + lastDim * intDiv(l, thisDim),
                    e.clone(),
                )
            }?;
            l = l + 1;
        }
        k = k + thisDim;
    }
    outExps = expArr
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    outDims = reverseDims.reverse();
    Ok((outExps, outDims))
}

fn evalCatGetFlatArray<Exp: Clone + 'static + metamodelica::gc::MMTrace>(
    mut e: Exp,
    mut dim: i32,
    mut getArrayContents: &dyn ::std::ops::Fn(Exp) -> Result<metamodelica::List<Exp>>,
    mut toString: &dyn ::std::ops::Fn(Exp) -> Result<ArcStr>,
) -> Result<(metamodelica::List<Exp>, metamodelica::List<i32>)> {
    pub type GetArrayContents<Exp: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<metamodelica::List<Exp>> + 'static>;

    pub type ToString<Exp: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp) -> Result<ArcStr> + 'static>;

    let mut outExps: metamodelica::List<Exp> = metamodelica::nil();
    let mut outDims: metamodelica::List<i32> = metamodelica::nil();
    let mut arr: metamodelica::List<Exp>;
    let mut dims: metamodelica::List<i32>;
    let mut i: i32;
    if dim == 1 {
        outExps = getArrayContents(e)?;
        outDims = list![((outExps).len() as i32)];
        return Ok((outExps, outDims));
    }
    i = 0;
    for mut exp in &*getArrayContents(e.clone())?.reverse() {
        (arr, dims) = evalCatGetFlatArray(exp.clone(), dim - 1, getArrayContents, toString)?;
        if (outDims).is_empty() {
            outDims = dims;
        } else if !(dims.clone() == outDims.clone()) {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ExpressionBasics.evalCatGetFlatArray"));
                    __mm_s.push_str(&*literal!(": Got unbalanced array from "));
                    __mm_s.push_str(&*toString(e.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("FrontEnd/ExpressionBasics.mo")),
            )?;
        }
        outExps = listAppend(arr, outExps);
        i = i + 1;
    }
    outDims = metamodelica::cons(i, outDims);
    Ok((outExps, outDims))
}

pub fn expEqual(mut inExp1: &metamodelica::Ref<DAE::Exp>, mut inExp2: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outEqual: bool;
    outEqual = 0 == compare(inExp1, inExp2)?;
    Ok(outEqual)
}

pub fn compare<'__b>(
    mut inExp1: &'__b metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<i32> {
    let mut comp: i32;
    if referenceEq(&*(&**inExp1), &*(&*inExp2)) {
        comp = 0;
        return Ok(comp);
    }
    comp = Util::intCompare(
        metamodelica::valueConstructor((&*&**inExp1))?,
        metamodelica::valueConstructor((&*&*inExp2))?,
    );
    if comp != 0 {
        return Ok(comp);
    }
    comp = (match &**inExp1 {
        DAE::Exp::ICONST { .. } => {
            let mut i: i32;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            i = metamodelica::Own::own(__pa0);
            Util::intCompare(var_field!((**inExp1).integer, DAE::Exp::ICONST).clone(), i)
        }
        DAE::Exp::RCONST { .. } => {
            let mut r: metamodelica::Real;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            Util::realCompare(var_field!((**inExp1).real, DAE::Exp::RCONST).clone(), r)
        }
        DAE::Exp::SCONST { .. } => {
            let mut s: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::SCONST { string: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            s = metamodelica::Own::own(__pa0);
            stringCompare(&var_field!((**inExp1).string, DAE::Exp::SCONST), &s)
        }
        DAE::Exp::BCONST { .. } => {
            let mut b: bool;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            b = metamodelica::Own::own(__pa0);
            Util::boolCompare(var_field!((**inExp1).bool, DAE::Exp::BCONST).clone(), b)
        }
        DAE::Exp::ENUM_LITERAL { .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ENUM_LITERAL { name: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            AbsynUtil::pathCompare(var_field!((**inExp1).name, DAE::Exp::ENUM_LITERAL), &p)?
        }
        DAE::Exp::CREF { .. } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            ComponentReferenceBasics::crefCompareGeneric(var_field!((**inExp1).componentRef, DAE::Exp::CREF), &cr)?
        }
        DAE::Exp::ARRAY { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ARRAY { ty: __pa0, array: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = valueCompare(var_field!((**inExp1).ty, DAE::Exp::ARRAY).clone(), ty);
            if (0 == comp) {
                compareList(var_field!((**inExp1).array, DAE::Exp::ARRAY), expl)?
            } else {
                comp
            }
        }
        DAE::Exp::MATRIX { .. } => {
            let mut mexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::MATRIX { ty: __pa0, matrix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            mexpl = metamodelica::Own::own(__pa1);
            comp = valueCompare(var_field!((**inExp1).ty, DAE::Exp::MATRIX).clone(), ty);
            if (0 == comp) {
                compareListList(var_field!((**inExp1).matrix, DAE::Exp::MATRIX), mexpl)?
            } else {
                comp
            }
        }
        DAE::Exp::BINARY { .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = operatorCompare(var_field!((**inExp1).operator, DAE::Exp::BINARY), &op)?;
            comp = if (0 == comp) {
                compare(var_field!((**inExp1).exp1, DAE::Exp::BINARY), e1)?
            } else {
                comp
            };
            if (0 == comp) {
                compare(var_field!((**inExp1).exp2, DAE::Exp::BINARY), e2)?
            } else {
                comp
            }
        }
        DAE::Exp::LBINARY { .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = operatorCompare(var_field!((**inExp1).operator, DAE::Exp::LBINARY), &op)?;
            comp = if (0 == comp) {
                compare(var_field!((**inExp1).exp1, DAE::Exp::LBINARY), e1)?
            } else {
                comp
            };
            if (0 == comp) {
                compare(var_field!((**inExp1).exp2, DAE::Exp::LBINARY), e2)?
            } else {
                comp
            }
        }
        DAE::Exp::UNARY { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::UNARY { exp: __pa0, operator: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            comp = operatorCompare(var_field!((**inExp1).operator, DAE::Exp::UNARY), &op)?;
            if (0 == comp) {
                compare(var_field!((**inExp1).exp, DAE::Exp::UNARY), e)?
            } else {
                comp
            }
        }
        DAE::Exp::LUNARY { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::LUNARY { exp: __pa0, operator: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            comp = operatorCompare(var_field!((**inExp1).operator, DAE::Exp::LUNARY), &op)?;
            if (0 == comp) {
                compare(var_field!((**inExp1).exp, DAE::Exp::LUNARY), e)?
            } else {
                comp
            }
        }
        DAE::Exp::RELATION { .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = operatorCompare(var_field!((**inExp1).operator, DAE::Exp::RELATION), &op)?;
            comp = if (0 == comp) {
                compare(var_field!((**inExp1).exp1, DAE::Exp::RELATION), e1)?
            } else {
                comp
            };
            if (0 == comp) {
                compare(var_field!((**inExp1).exp2, DAE::Exp::RELATION), e2)?
            } else {
                comp
            }
        }
        DAE::Exp::IFEXP { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::IFEXP { expCond: __pa0, expThen: __pa1, expElse: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            e1 = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = compare(var_field!((**inExp1).expCond, DAE::Exp::IFEXP), e)?;
            comp = if (0 == comp) {
                compare(var_field!((**inExp1).expThen, DAE::Exp::IFEXP), e1)?
            } else {
                comp
            };
            if (0 == comp) {
                compare(var_field!((**inExp1).expElse, DAE::Exp::IFEXP), e2)?
            } else {
                comp
            }
        }
        DAE::Exp::CALL { .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::CALL { path: __pa0, expLst: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = AbsynUtil::pathCompare(var_field!((**inExp1).path, DAE::Exp::CALL), &p)?;
            if (0 == comp) {
                compareList(var_field!((**inExp1).expLst, DAE::Exp::CALL), expl)?
            } else {
                comp
            }
        }
        DAE::Exp::RECORD { .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::RECORD { path: __pa0, exps: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = AbsynUtil::pathCompare(var_field!((**inExp1).path, DAE::Exp::RECORD), &p)?;
            if (0 == comp) {
                compareList(var_field!((**inExp1).exps, DAE::Exp::RECORD), expl)?
            } else {
                comp
            }
        }
        DAE::Exp::PARTEVALFUNCTION { .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::PARTEVALFUNCTION { path: __pa0, expList: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = AbsynUtil::pathCompare(var_field!((**inExp1).path, DAE::Exp::PARTEVALFUNCTION), &p)?;
            if (0 == comp) {
                compareList(var_field!((**inExp1).expList, DAE::Exp::PARTEVALFUNCTION), expl)?
            } else {
                comp
            }
        }
        DAE::Exp::RANGE { .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut oe: Option<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            oe = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = compare(var_field!((**inExp1).start, DAE::Exp::RANGE), e1)?;
            comp = if (0 == comp) {
                compare(var_field!((**inExp1).stop, DAE::Exp::RANGE), e2)?
            } else {
                comp
            };
            if (0 == comp) {
                compareOpt(var_field!((**inExp1).step, DAE::Exp::RANGE).clone(), oe)?
            } else {
                comp
            }
        }
        DAE::Exp::TUPLE { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            compareList(var_field!((**inExp1).PR, DAE::Exp::TUPLE), expl)?
        }
        DAE::Exp::CAST { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::CAST { ty: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            e = metamodelica::Own::own(__pa1);
            comp = valueCompare(var_field!((**inExp1).ty, DAE::Exp::CAST).clone(), ty);
            if (0 == comp) {
                compare(var_field!((**inExp1).exp, DAE::Exp::CAST), e)?
            } else {
                comp
            }
        }
        DAE::Exp::ASUB { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ASUB { exp: __pa0, sub: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            comp = compare(var_field!((**inExp1).exp, DAE::Exp::ASUB), e)?;
            if (comp == 0) {
                compareSubscriptList(var_field!((**inExp1).sub, DAE::Exp::ASUB), subs)?
            } else {
                comp
            }
        }
        DAE::Exp::RSUB { .. } => {
            let mut i: i32;
            let mut s: ArcStr;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::RSUB { exp: __pa0, ix: __pa1, fieldName: __pa2, ty: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            s = metamodelica::Own::own(__pa2);
            ty = metamodelica::Own::own(__pa3);
            comp = Util::intCompare(var_field!((**inExp1).ix, DAE::Exp::RSUB).clone(), i);
            comp = if (comp == 0) {
                valueCompare(var_field!((**inExp1).ty, DAE::Exp::RSUB).clone(), ty)
            } else {
                comp
            };
            comp = if (comp == 0) {
                stringCompare(&var_field!((**inExp1).fieldName, DAE::Exp::RSUB), &s)
            } else {
                comp
            };
            if (comp == 0) {
                compare(var_field!((**inExp1).exp, DAE::Exp::RSUB), e)?
            } else {
                comp
            }
        }
        DAE::Exp::TSUB { .. } => {
            let mut i: i32;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::TSUB { exp: __pa0, ix: __pa1, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            ty = metamodelica::Own::own(__pa2);
            comp = Util::intCompare(var_field!((**inExp1).ix, DAE::Exp::TSUB).clone(), i);
            comp = if (0 == comp) {
                valueCompare(var_field!((**inExp1).ty, DAE::Exp::TSUB).clone(), ty)
            } else {
                comp
            };
            if (0 == comp) {
                compare(var_field!((**inExp1).exp, DAE::Exp::TSUB), e)?
            } else {
                comp
            }
        }
        DAE::Exp::SIZE { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut oe: Option<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::SIZE { exp: __pa0, sz: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            oe = metamodelica::Own::own(__pa1);
            comp = compare(var_field!((**inExp1).exp, DAE::Exp::SIZE), e)?;
            if (comp == 0) {
                compareOpt(var_field!((**inExp1).sz, DAE::Exp::SIZE).clone(), oe)?
            } else {
                comp
            }
        }
        DAE::Exp::REDUCTION { .. } => valueCompare(inExp1.clone(), inExp2),
        DAE::Exp::LIST { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::LIST { valList: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            compareList(var_field!((**inExp1).valList, DAE::Exp::LIST), expl)?
        }
        DAE::Exp::CONS { .. } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::CONS { car: __pa0, cdr: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            e2 = metamodelica::Own::own(__pa1);
            comp = compare(var_field!((**inExp1).car, DAE::Exp::CONS), e1)?;
            if (0 == comp) {
                compare(var_field!((**inExp1).cdr, DAE::Exp::CONS), e2)?
            } else {
                comp
            }
        }
        DAE::Exp::META_TUPLE { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::META_TUPLE { listExp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            compareList(var_field!((**inExp1).listExp, DAE::Exp::META_TUPLE), expl)?
        }
        DAE::Exp::META_OPTION { .. } => {
            let mut oe: Option<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::META_OPTION { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            oe = metamodelica::Own::own(__pa0);
            compareOpt(var_field!((**inExp1).exp, DAE::Exp::META_OPTION).clone(), oe)?
        }
        DAE::Exp::METARECORDCALL { .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::METARECORDCALL { path: __pa0, args: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = AbsynUtil::pathCompare(var_field!((**inExp1).path, DAE::Exp::METARECORDCALL), &p)?;
            if (comp == 0) {
                compareList(var_field!((**inExp1).args, DAE::Exp::METARECORDCALL), expl)?
            } else {
                comp
            }
        }
        DAE::Exp::MATCHEXPRESSION { .. } => valueCompare(inExp1.clone(), inExp2),
        DAE::Exp::BOX { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::BOX { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            compare(var_field!((**inExp1).exp, DAE::Exp::BOX), e)?
        }
        DAE::Exp::UNBOX { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            compare(var_field!((**inExp1).exp, DAE::Exp::UNBOX), e)?
        }
        DAE::Exp::SHARED_LITERAL { .. } => {
            let mut i: i32;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::SHARED_LITERAL { index: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            i = metamodelica::Own::own(__pa0);
            Util::intCompare(var_field!((**inExp1).index, DAE::Exp::SHARED_LITERAL).clone(), i)
        }
        DAE::Exp::EMPTY { .. } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::EMPTY { name: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            ComponentReferenceBasics::crefCompareGeneric(var_field!((**inExp1).name, DAE::Exp::EMPTY), &cr)?
        }
        DAE::Exp::CODE { .. } => valueCompare(inExp1.clone(), inExp2),
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ExpressionBasics.compare failed: ctor:"));
                    __mm_s.push_str(&*ArcStr::from(::std::format!(
                        "{:?}",
                        metamodelica::valueConstructor((&*&**inExp1))?
                    )));
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*printExpStr(inExp1.clone())?);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*printExpStr(inExp2)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/ExpressionBasics.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(comp)
}

fn compareList(
    mut inExpl1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExpl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<i32> {
    let mut comp: i32;
    let mut len1: i32;
    let mut len2: i32;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut rest_expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpl2.clone();
    len1 = ((inExpl1).len() as i32);
    len2 = ((inExpl2).len() as i32);
    comp = Util::intCompare(len1, len2);
    if comp != 0 {
        return Ok(comp);
    }
    for mut e1 in &**inExpl1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_expl2 = metamodelica::Own::own(__pa1);
        comp = compare(metamodelica::AsArg::as_arg(&e1), e2)?;
        if 0 != comp {
            return Ok(comp);
        }
    }
    comp = 0;
    Ok(comp)
}

fn compareListList(
    mut inExpl1: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inExpl2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<i32> {
    let mut comp: i32;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rest_expl2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = inExpl2.clone();
    let mut len1: i32;
    let mut len2: i32;
    len1 = ((inExpl1).len() as i32);
    len2 = ((inExpl2).len() as i32);
    comp = Util::intCompare(len1, len2);
    if comp != 0 {
        return Ok(comp);
    }
    for mut expl1 in &**inExpl1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        expl2 = metamodelica::Own::own(__pa0);
        rest_expl2 = metamodelica::Own::own(__pa1);
        comp = compareList(metamodelica::AsArg::as_arg(&expl1), expl2)?;
        if 0 != comp {
            return Ok(comp);
        }
    }
    comp = 0;
    Ok(comp)
}

fn compareOpt(
    mut inExp1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inExp2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<i32> {
    let mut comp: i32;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    comp = (::match_deref::match_deref! { match &((inExp1, inExp2)) {
        (None, None) => 0,
        (None, _) => -1,
        (_, None) => 1,
        (Some(__esc_e1), Some(__esc_e2)) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            compare(metamodelica::AsArg::as_arg(&e1), e2.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(comp)
}

pub fn operatorCompare(mut inOperator1: &DAE::Operator, mut inOperator2: &DAE::Operator) -> Result<i32> {
    let mut comp: i32;
    comp = (match (inOperator1.clone(), inOperator2.clone()) {
        (DAE::Operator::USERDEFINED { fqName: ref p1 }, DAE::Operator::USERDEFINED { fqName: ref p2 }) => {
            AbsynUtil::pathCompare(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))?
        }
        _ => Util::intCompare(
            metamodelica::valueConstructor((&inOperator1.clone()))?,
            metamodelica::valueConstructor((&inOperator2.clone()))?,
        ),
    });
    Ok(comp)
}

pub(crate) fn compareSubscripts(
    mut sub1: &metamodelica::Ref<DAE::Subscript>,
    mut sub2: &metamodelica::Ref<DAE::Subscript>,
) -> Result<i32> {
    let mut res: i32;
    if referenceEq(&*(&**sub1), &*(&**sub2)) {
        res = 0;
    } else {
        res = (::match_deref::match_deref! { match (sub1, sub2) {
            (Deref @ DAE::Subscript::WHOLEDIM { .. }, Deref @ DAE::Subscript::WHOLEDIM { .. }) => 0,
            (Deref @ DAE::Subscript::SLICE { .. }, Deref @ DAE::Subscript::SLICE { .. }) => compare(var_field!((**sub1).exp, DAE::Subscript::SLICE), var_field!((**sub2).exp, DAE::Subscript::SLICE).clone())?,
            (Deref @ DAE::Subscript::INDEX { .. }, Deref @ DAE::Subscript::INDEX { .. }) => compare(var_field!((**sub1).exp, DAE::Subscript::INDEX), var_field!((**sub2).exp, DAE::Subscript::INDEX).clone())?,
            (Deref @ DAE::Subscript::WHOLE_NONEXP { .. }, Deref @ DAE::Subscript::WHOLE_NONEXP { .. }) => compare(var_field!((**sub1).exp, DAE::Subscript::WHOLE_NONEXP), var_field!((**sub2).exp, DAE::Subscript::WHOLE_NONEXP).clone())?,
            _ => Util::intCompare(metamodelica::valueConstructor((&*&**sub1))?, metamodelica::valueConstructor((&*&**sub2))?),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(res)
}

fn compareSubscriptList(
    mut subs1: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut subs2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<i32> {
    let mut comp: i32;
    let mut len1: i32;
    let mut len2: i32;
    let mut s2: metamodelica::Ref<DAE::Subscript>;
    let mut rest_subs2: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = subs2.clone();
    len1 = ((subs1).len() as i32);
    len2 = ((subs2).len() as i32);
    comp = Util::intCompare(len1, len2);
    if comp != 0 {
        return Ok(comp);
    }
    for mut s1 in &**subs1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_subs2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        s2 = metamodelica::Own::own(__pa0);
        rest_subs2 = metamodelica::Own::own(__pa1);
        comp = compareSubscripts(metamodelica::AsArg::as_arg(&s1), &s2)?;
        if 0 != comp {
            return Ok(comp);
        }
    }
    comp = 0;
    Ok(comp)
}

pub(crate) fn subscriptInt(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<i32> {
    let mut outInteger: i32 = expArrayIndex(&(subscriptIndexExp(inSubscript)?))?;
    Ok(outInteger)
}

pub(crate) fn subscriptsInt(
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<i32>> {
    let mut outIntegers: metamodelica::List<i32>;
    outIntegers = List::map(inSubscripts, &move |__a0: metamodelica::Ref<DAE::Subscript>| {
        subscriptInt(&__a0)
    })?;
    Ok(outIntegers)
}

pub fn expArrayIndex(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut outIndex: i32;
    outIndex = (match &**inExp {
        DAE::Exp::ICONST {
            integer: __inExp_integer,
        } => __inExp_integer.clone(),
        DAE::Exp::ENUM_LITERAL {
            index: __inExp_index, ..
        } => __inExp_index.clone(),
        DAE::Exp::BCONST { bool: __inExp_bool } => {
            if (__inExp_bool.clone()) {
                2
            } else {
                1
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outIndex)
}

pub fn subscriptIndexExp(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &((*inSubscript)) {
        Deref @ DAE::Subscript::INDEX { exp: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outExp = metamodelica::Own::own(__pa0);
    Ok(outExp)
}

pub fn subscriptEqual<'__b>(
    mut inSubscriptLst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inSubscriptLst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inSubscriptLst1, inSubscriptLst2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(true)
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: xs1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: xs2 }) => {
                { (inSubscriptLst1, inSubscriptLst2) = (xs1, xs2); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e1 }, tail: xs1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e2 }, tail: xs2 }) => {
                if (expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) {{ (inSubscriptLst1, inSubscriptLst2) = (xs1, xs2); continue '__tco; }} else {return Ok(false)}
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e1 }, tail: xs1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e2 }, tail: xs2 }) => {
                if (expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) {{ (inSubscriptLst1, inSubscriptLst2) = (xs1, xs2); continue '__tco; }} else {return Ok(false)}
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e1 }, tail: xs1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e2 }, tail: xs2 }) => {
                if (expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) {{ (inSubscriptLst1, inSubscriptLst2) = (xs1, xs2); continue '__tco; }} else {return Ok(false)}
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn printListStr<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTypeALst: metamodelica::List<Type_a>,
    mut inFuncTypeTypeAToString: &dyn ::std::ops::Fn(Type_a) -> Result<ArcStr>,
    mut inString: ArcStr,
) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = stringDelimitList(List::map(inTypeALst, inFuncTypeTypeAToString)?, inString);
    Ok(outString)
}

pub fn printSubscriptStr(mut sub: &metamodelica::Ref<DAE::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**sub {
        DAE::Subscript::WHOLEDIM { .. } => literal!(":"),
        DAE::Subscript::INDEX { exp: __sub_exp } => printExpStr(__sub_exp.clone())?,
        DAE::Subscript::SLICE { exp: __sub_exp } => printExpStr(__sub_exp.clone())?,
        DAE::Subscript::WHOLE_NONEXP { exp: __sub_exp } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("1:"));
            __mm_s.push_str(&*printExpStr(__sub_exp.clone())?);
            ArcStr::from(__mm_s)
        }
    });
    Ok(outString)
}

pub fn hashExp(mut e: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut hash: i32;
    hash = 'mc: {
        let __mc_input = &**e;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: i } => {
                    Ok(stringHashDjb2(&(intString(i.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: r } => {
                    Ok(stringHashDjb2(&(realString(r.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: b } => {
                    Ok(stringHashDjb2(&(boolString(b.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    Ok(stringHashDjb2(&s))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: path, .. } => {
                    Ok(stringHashDjb2(&(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    Ok(ComponentReferenceBasics::hashComponentRef(metamodelica::AsArg::as_arg(&cr))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    Ok(1 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashOp(metamodelica::AsArg::as_arg(&op))? + hashExp(metamodelica::AsArg::as_arg(&e2))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
                    Ok(2 + hashOp(metamodelica::AsArg::as_arg(&op))? + hashExp(metamodelica::AsArg::as_arg(&e1))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    Ok(3 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashOp(metamodelica::AsArg::as_arg(&op))? + hashExp(metamodelica::AsArg::as_arg(&e2))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
                    Ok(4 + hashOp(metamodelica::AsArg::as_arg(&op))? + hashExp(metamodelica::AsArg::as_arg(&e1))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: _, optionExpisASUB: _ } => {
                    Ok(5 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashOp(metamodelica::AsArg::as_arg(&op))? + hashExp(metamodelica::AsArg::as_arg(&e2))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
                    Ok(6 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashExp(metamodelica::AsArg::as_arg(&e2))? + hashExp(metamodelica::AsArg::as_arg(&e3))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: expl, .. } => {
                    Ok(7 + stringHashDjb2(&(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?)) + List::reduce(&(List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| hashExp(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { path, exps: expl, .. } => {
                    Ok(8 + stringHashDjb2(&(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?)) + List::reduce(&(List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| hashExp(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, .. } => {
                    Ok(9 + stringHashDjb2(&(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?)) + List::reduce(&(List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| hashExp(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    Ok(10 + List::reduce(&(List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| hashExp(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } => {
                    Ok(11 + List::reduce(&(List::map(List::flatten(mexpl.clone())?, &move |__a0: metamodelica::Ref<DAE::Exp>| hashExp(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start: e1, step: Some(e2), stop: e3 } => {
                    Ok(12 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashExp(metamodelica::AsArg::as_arg(&e2))? + hashExp(metamodelica::AsArg::as_arg(&e3))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start: e1, step: None, stop: e3 } => {
                    Ok(13 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashExp(metamodelica::AsArg::as_arg(&e3))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expl } => {
                    Ok(14 + List::reduce(&(List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| hashExp(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: _, exp: e1 } => {
                    Ok(15 + hashExp(metamodelica::AsArg::as_arg(&e1))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
                            Ok(16 + hashExp(metamodelica::AsArg::as_arg(&e1))? + List::reduce(&(({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = hashExp(&(getSubscriptExp(&(sub.clone()))?))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), &fnptr!(intAdd, i32, i32))?)
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: _ } => {
                    Ok(17 + hashExp(metamodelica::AsArg::as_arg(&e1))? + stringHashDjb2(&(intString(i.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
                    Ok(18 + hashExp(metamodelica::AsArg::as_arg(&e1))? + hashExp(metamodelica::AsArg::as_arg(&e2))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e1, sz: None } => {
                    Ok(19 + hashExp(metamodelica::AsArg::as_arg(&e1))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: info, expr: e1, iterators: iters } => {
                    Ok(22 + hashReductionInfo(metamodelica::AsArg::as_arg(&info))? + hashExp(metamodelica::AsArg::as_arg(&e1))? + List::reduce(&(List::map(iters.clone(), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>| hashReductionIter(&__a0))?), &fnptr!(intAdd, i32, i32))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(stringHashDjb2(&(printExpStr(e.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(hash)
}

fn hashReductionInfo(mut info: &metamodelica::Ref<DAE::ReductionInfo>) -> Result<i32> {
    let mut hash: i32;
    hash = (match &**info {
        DAE::ReductionInfo { path, .. } => {
            22 + stringHashDjb2(&(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?))
        }
    });
    Ok(hash)
}

fn hashReductionIter(mut iter: &metamodelica::Ref<DAE::ReductionIterator>) -> Result<i32> {
    let mut hash: i32;
    hash = (::match_deref::match_deref! { match iter {
        Deref @ DAE::ReductionIterator { id, exp: e1, guardExp: Some(e2), ty: _ } => {
            23 + stringHashDjb2(&id) + hashExp(e1)? + hashExp(metamodelica::AsArg::as_arg(&e2))?
        },
        Deref @ DAE::ReductionIterator { id, exp: e1, guardExp: None, ty: _ } => {
            24 + stringHashDjb2(&id) + hashExp(e1)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(hash)
}

fn hashOp(mut op: &DAE::Operator) -> Result<i32> {
    let mut hash: i32;
    hash = (match op.clone() {
        DAE::Operator::ADD { ty: _ } => 25,
        DAE::Operator::SUB { ty: _ } => 26,
        DAE::Operator::MUL { ty: _ } => 27,
        DAE::Operator::DIV { ty: _ } => 28,
        DAE::Operator::POW { ty: _ } => 29,
        DAE::Operator::UMINUS { ty: _ } => 30,
        DAE::Operator::UMINUS_ARR { ty: _ } => 31,
        DAE::Operator::ADD_ARR { ty: _ } => 32,
        DAE::Operator::SUB_ARR { ty: _ } => 33,
        DAE::Operator::MUL_ARR { ty: _ } => 34,
        DAE::Operator::DIV_ARR { ty: _ } => 35,
        DAE::Operator::MUL_ARRAY_SCALAR { ty: _ } => 36,
        DAE::Operator::ADD_ARRAY_SCALAR { ty: _ } => 37,
        DAE::Operator::SUB_SCALAR_ARRAY { ty: _ } => 38,
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ } => 39,
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ } => 40,
        DAE::Operator::DIV_ARRAY_SCALAR { ty: _ } => 41,
        DAE::Operator::DIV_SCALAR_ARRAY { ty: _ } => 42,
        DAE::Operator::POW_ARRAY_SCALAR { ty: _ } => 43,
        DAE::Operator::POW_SCALAR_ARRAY { ty: _ } => 44,
        DAE::Operator::POW_ARR { ty: _ } => 45,
        DAE::Operator::POW_ARR2 { ty: _ } => 46,
        DAE::Operator::AND { ty: _ } => 47,
        DAE::Operator::OR { ty: _ } => 48,
        DAE::Operator::NOT { ty: _ } => 49,
        DAE::Operator::LESS { ty: _ } => 50,
        DAE::Operator::LESSEQ { ty: _ } => 51,
        DAE::Operator::GREATER { ty: _ } => 52,
        DAE::Operator::GREATEREQ { ty: _ } => 53,
        DAE::Operator::EQUAL { ty: _ } => 54,
        DAE::Operator::NEQUAL { ty: _ } => 55,
        DAE::Operator::USERDEFINED { fqName: ref path } => {
            56 + stringHashDjb2(&(AbsynUtil::pathString(path.clone(), literal!("."), true, false)?))
        }
    });
    Ok(hash)
}

fn getSubscriptExp(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inSubscript {
        DAE::Subscript::SLICE { exp: e } => e.clone(),
        DAE::Subscript::INDEX { exp: e } => e.clone(),
        DAE::Subscript::WHOLE_NONEXP { exp: e } => e.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}
