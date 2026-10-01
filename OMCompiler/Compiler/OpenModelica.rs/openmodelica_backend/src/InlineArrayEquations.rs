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

use crate::BackendDAEUtil;
use crate::BackendEquation;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// inline arrayeqns stuff
//
// public functions:
//   - inlineArrayEqn
//   - getScalarArrayEqns
// =============================================================================
pub(crate) fn inlineArrayEqn(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &fnptr!(
            inlineArrayEqn1,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            bool
        ),
        false,
    )?;
    Ok(outDAE)
}

fn inlineArrayEqn1(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inOptimized: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
) {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outOptimized: bool;
    (outEqSystem, outOptimized) = 'mc: {
        let __mc_input = &*inEqSystem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut orderedEqs = (*orderedEqs).clone();
                    eqnLst = BackendEquation::equationList(orderedEqs.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(getScalarArrayEqns(&eqnLst)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqnLst = metamodelica::Own::own(__pa0);
                    orderedEqs = BackendEquation::listEquation(&eqnLst)?;
                    Ok((BackendDAEUtil::clearEqSyst(&(BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), orderedEqs.clone()))), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEqSystem.clone(), inOptimized))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEqSystem, outShared, outOptimized)
}

pub(crate) fn getScalarArrayEqns(
    mut inEqnLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool) {
    let mut outEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outFound: bool;
    (outEqnLst, outFound) = getScalarArrayEqns0(inEqnLst, metamodelica::nil(), false);
    (outEqnLst, outFound)
}

fn getScalarArrayEqns0<'__b>(
    mut inEqnLst: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inAccEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inFound: bool,
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match inEqnLst {
            Deref @ metamodelica::ListNode::Nil => {
                return (inAccEqnLst.reverse(), inFound)
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns } => {
                let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut b: bool;
                (eqns1, b) = getScalarArrayEqns1(eqn.clone(), inAccEqnLst);
                { (inEqnLst, inAccEqnLst, inFound) = (eqns, eqns1, b || inFound); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getScalarArrayEqns1(
    mut inEqn: metamodelica::Ref<BackendDAE::Equation>,
    mut inAccEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool) {
    let mut outEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outFound: bool;
    (outEqnLst, outFound) = 'mc: {
        let __mc_input = &*inEqn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: lhs, right: rhs, source, attr, .. } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut ea1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ea2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    if Expression::isArray(metamodelica::AsArg::as_arg(&lhs)) || Expression::isMatrix(metamodelica::AsArg::as_arg(&lhs)) {
                        ea1 = Expression::flattenArrayExpToList(lhs.clone());
                    } else {
                        (e1, _) = Expression::extendArrExp(lhs.clone(), false);
                        (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                        let true = (Expression::isArray(&e1) || Expression::isMatrix(&e1)) else { return Err("pattern mismatch") };
                        ea1 = Expression::flattenArrayExpToList(e1.clone());
                    }
                    if Expression::isArray(metamodelica::AsArg::as_arg(&rhs)) || Expression::isMatrix(metamodelica::AsArg::as_arg(&rhs)) {
                        ea2 = Expression::flattenArrayExpToList(rhs.clone());
                    } else {
                        (e2, _) = Expression::extendArrExp(rhs.clone(), false);
                        (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                        let true = (Expression::isArray(&e2) || Expression::isMatrix(&e2)) else { return Err("pattern mismatch") };
                        ea2 = Expression::flattenArrayExpToList(e2.clone());
                    }
                    (_, eqns) = List::threadFold3(&ea1, ea2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>, __a2: metamodelica::Ref<DAE::ElementSource>, __a3: BackendDAE::EquationAttributes, __a4: metamodelica::Ref<DAE::EquationExp>, __a5: (i32, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>)| generateScalarArrayEqns2(__a0, __a1, __a2, __a3, __a4, &__a5), source.clone(), attr.clone(), metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: lhs.clone(), rhs: rhs.clone() }), (1, inAccEqnLst.clone()))?;
                    Ok((eqns.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: lhs, right: rhs, source, attr, .. } => {
                    let mut ea1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ea2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    ea1 = Expression::splitRecord(metamodelica::AsArg::as_arg(&lhs), &(Expression::r#typeof(lhs.clone())?))?;
                    ea2 = Expression::splitRecord(metamodelica::AsArg::as_arg(&rhs), &(Expression::r#typeof(rhs.clone())?))?;
                    (_, eqns) = List::threadFold3(&ea1, ea2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>, __a2: metamodelica::Ref<DAE::ElementSource>, __a3: BackendDAE::EquationAttributes, __a4: metamodelica::Ref<DAE::EquationExp>, __a5: (i32, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>)| generateScalarArrayEqns2(__a0, __a1, __a2, __a3, __a4, &__a5), source.clone(), attr.clone(), metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: lhs.clone(), rhs: rhs.clone() }), (1, inAccEqnLst.clone()))?;
                    Ok((eqns.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::cons(inEqn.clone(), inAccEqnLst.clone()), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEqnLst, outFound)
}

fn generateScalarArrayEqns2(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut eqAttr: BackendDAE::EquationAttributes,
    mut eqExp: metamodelica::Ref<DAE::EquationExp>,
    mut iEqns: &(i32, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>),
) -> Result<(i32, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>)> {
    let mut oEqns: (i32, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>);
    oEqns = 'mc: {
        let __mc_input = iEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, eqns) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut size: i32;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeComplex(&tp)) else { return Err("pattern mismatch") };
                    size = Expression::sizeOf(&tp);
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_SCALARIZE { before: eqExp.clone(), index: i.clone(), after: metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: inExp1.clone(), rhs: inExp2.clone() }) }))?;
                    Ok((i.clone() + 1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: eqAttr }), eqns.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, eqns) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut recordSize: Option<i32>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut ds: metamodelica::List<i32>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeArray(&tp)) else { return Err("pattern mismatch") };
                    dims = Expression::arrayDimension(&tp);
                    tp = DAEUtil::expTypeElementType(&tp);
                    if DAEUtil::expTypeComplex(&tp) {
                        recordSize = Some(Expression::sizeOf(&tp));
                    } else {
                        recordSize = None;
                    }
                    ds = Expression::dimensionsSizes(dims.clone())?;
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_SCALARIZE { before: eqExp.clone(), index: i.clone(), after: metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: inExp1.clone(), rhs: inExp2.clone() }) }))?;
                    Ok((i.clone() + 1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: ds.clone(), left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: eqAttr, recordSize: recordSize.clone() }), eqns.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, eqns) => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    b1 = DAEUtil::expTypeComplex(&tp);
                    b2 = DAEUtil::expTypeArray(&tp);
                    let false = (b1 || b2) else { return Err("pattern mismatch") };
                    source = ElementSource::addSymbolicTransformation(inSource.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_SCALARIZE { before: eqExp.clone(), index: i.clone(), after: metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: inExp1.clone(), rhs: inExp2.clone() }) }))?;
                    Ok((i.clone() + 1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: inExp1.clone(), scalar: inExp2.clone(), source: source.clone(), attr: eqAttr }), eqns.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InlineArrayEquations.generateScalarArrayEqns2 failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oEqns)
}
