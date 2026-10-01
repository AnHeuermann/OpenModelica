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

use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

//--------------------------------
// collect for-loops
//--------------------------------
pub(crate) fn collectForLoops(
    mut varsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut eqsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut varsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut arrayCrefs: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut arrayVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut forEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut mixEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut nonArrEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (varLst, arrayVars) = List::fold(
        varsIn,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
               __a1: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        )|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(getArrayVars(__a0, &__a1)) },
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    (arrayCrefs, _) = List::fold(
        &arrayVars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
               __a1: (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                i32,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        )|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(getArrayVarCrefs(&__a0, __a1)) },
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    (forEqs, mixEqs, nonArrEqs) = List::fold1(
        eqsIn,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )| dispatchLoopEquations(__a0, __a1, &__a2),
        List::map(arrayCrefs, &fnptr!(Util::tuple31, _))?,
        (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
    )?;
    forEqs = buildBackendDAEForEquations(&forEqs, &(metamodelica::nil()));
    mixEqs = List::fold(
        &mixEqs,
        &fnptr!(
            buildAccumExpInEquations,
            metamodelica::Ref<BackendDAE::Equation>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>
        ),
        metamodelica::nil(),
    )?
    .reverse();
    arrayVars = unexpandArrayVariables(&arrayVars, &(metamodelica::nil()));
    eqsOut = listAppend(forEqs, listAppend(mixEqs, nonArrEqs));
    varsOut = listAppend(arrayVars, varLst);
    Ok((varsOut, eqsOut))
}

fn unexpandArrayVariables(
    mut varsIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut foldIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Var>> {
    let mut foldOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    foldOut = 'mc: {
        let __mc_input = &**varsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(foldIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: var, tail: rest } => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut scalars: metamodelica::List<_>;
                    let mut var = (*var).clone();
                    let mut rest = (*rest).clone();
                    cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&var));
                    let true = (ComponentReference::crefHaveSubs(&cref)) else { return Err("pattern mismatch") };
                    (scalars, rest) = List::split1OnTrue(metamodelica::AsArg::as_arg(&rest), &move |__a0: metamodelica::Ref<BackendDAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(varIsEqualCrefWithoutSubs(&__a0, __a1)) }, cref.clone())?;
                    cref = replaceFirstSubsInCref(&cref, &(list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::RANGE { ty: BackendVariable::varType(metamodelica::AsArg::as_arg(&var)), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: ((scalars).len() as i32) + 1 }) }) })]));
                    var = BackendVariable::copyVarNewName(cref.clone(), var.clone());
                    Ok(unexpandArrayVariables(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(var.clone(), foldIn.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: var, tail: rest } => {
                    Ok(unexpandArrayVariables(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(var.clone(), foldIn.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    foldOut
}

fn varIsEqualCrefWithoutSubs(
    mut varIn: &metamodelica::Ref<BackendDAE::Var>,
    mut crefIn: metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut b: bool;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    cref = BackendVariable::varCref(varIn);
    b = ComponentReferenceBasics::crefEqualWithoutSubs(cref, crefIn);
    b
}

fn buildAccumExpInEquations(
    mut mixEq: metamodelica::Ref<BackendDAE::Equation>,
    mut foldIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> {
    let mut foldOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    foldOut = 'mc: {
        let __mc_input = &*mixEq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: rhs, scalar: lhs, source, attr } => {
                    let mut allTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut minmaxTerms: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)>;
                    let mut rhs = (*rhs).clone();
                    let mut lhs = (*lhs).clone();
                    allTerms = Expression::allTerms(metamodelica::AsArg::as_arg(&lhs));
                    minmaxTerms = List::fold(&allTerms, &fnptr!(buildAccumExpInEquations1, metamodelica::Ref<DAE::Exp>, metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)>), metamodelica::nil())?;
                    let __pa0 = ::match_deref::match_deref! { match &(buildAccumExpInEquations2(&(minmaxTerms.clone().reverse()), &(metamodelica::nil()))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs = metamodelica::Own::own(__pa0);
                    allTerms = Expression::allTerms(metamodelica::AsArg::as_arg(&rhs));
                    minmaxTerms = List::fold(&allTerms, &fnptr!(buildAccumExpInEquations1, metamodelica::Ref<DAE::Exp>, metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)>), metamodelica::nil())?;
                    let __pa2 = ::match_deref::match_deref! { match &(buildAccumExpInEquations2(&(minmaxTerms.clone().reverse()), &(metamodelica::nil()))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rhs = metamodelica::Own::own(__pa2);
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: rhs.clone(), scalar: lhs.clone(), source: source.clone(), attr: attr.clone() }), foldIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(mixEq.clone(), foldIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    foldOut
}

fn buildAccumExpInEquations1(
    mut termIn: metamodelica::Ref<DAE::Exp>,
    mut minmaxTermsIn: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)>,
) -> metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)> {
    let mut minmaxTermsOut: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)>;
    let mut pos: i32;
    let mut idx: i32;
    let mut min: i32;
    let mut max: i32;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut term: metamodelica::Ref<DAE::Exp>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Expression::extractCrefsFromExp(termIn.clone()), '__try0)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa1);
        let true = (ComponentReference::crefHaveSubs(&cref)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        pos = unwrap_break_err!(List::position1OnTrue(&minmaxTermsIn, &move |__a0: (metamodelica::Ref<DAE::Exp>, i32, i32), __a1: metamodelica::Ref<DAE::Exp>| minmaxTermEqual(&__a0, __a1), termIn.clone()), '__try0);
        if intEq(pos, -1) {
            let __pa3 = ::match_deref::match_deref! { match &(unwrap_break_err!(ComponentReferenceBasics::crefSubs(&cref), '__try0)) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa3 } }, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            idx = metamodelica::Own::own(__pa3);
            minmaxTermsOut = metamodelica::cons((termIn.clone(), idx, idx), minmaxTermsIn.clone());
        } else {
            (term, min, max) = unwrap_break_err!((minmaxTermsIn).get(pos), '__try0);
            let __pa6 = ::match_deref::match_deref! { match &(unwrap_break_err!(ComponentReferenceBasics::crefSubs(&cref), '__try0)) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa6 } }, tail: Deref @ metamodelica::ListNode::Nil } => __pa6.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            idx = metamodelica::Own::own(__pa6);
            minmaxTermsOut = unwrap_break_err!(List::replaceAt((term.clone(), intMin(idx, min), intMax(idx, max)), pos, minmaxTermsIn.clone()), '__try0);
        }
        Ok::<_, &'static str>((minmaxTermsOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            minmaxTermsOut = __try0_o0;
        }
        Err(_) => {
            minmaxTermsOut = metamodelica::cons((termIn.clone(), -1, -1), minmaxTermsIn.clone());
        }
    }
    minmaxTermsOut
}

fn buildAccumExpInEquations2(
    mut minmaxTerm: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, i32)>,
    mut foldIn: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut foldOut: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let sumReductionInfo: metamodelica::Ref<DAE::ReductionInfo> = metamodelica::Ref::new(DAE::ReductionInfo {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sum") }),
        iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
        exprType: DAE::T_REAL_DEFAULT().clone(),
        defaultValue: Some(metamodelica::Ref::new(Values::Value::REAL {
            real: metamodelica::OrderedFloat(0.0_f64),
        })),
        foldName: literal!("$sumFold"),
        resultName: literal!("$sumRes"),
        foldExp: Some(metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: literal!("$sumFold"),
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: DAE::T_REAL_DEFAULT().clone(),
            }),
            operator: DAE::Operator::ADD {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: literal!("$sumRes"),
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                }),
                ty: DAE::T_REAL_DEFAULT().clone(),
            }),
        })),
    });
    let sumExp: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: literal!("$sumIter"),
            identType: DAE::T_REAL_DEFAULT().clone(),
            subscriptLst: metamodelica::nil(),
        }),
        ty: DAE::T_REAL_DEFAULT().clone(),
    });
    foldOut = 'mc: {
        let __mc_input = (&**minmaxTerm, &**foldIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    Ok(list![exp1.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp1, min, max), tail: rest }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut iter: metamodelica::Ref<DAE::Exp>;
                    let mut resExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp1 = (*exp1).clone();
                    let mut rest = (*rest).clone();
                    let true = (intNe(min.clone(), max.clone())) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::r#typeof(exp1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (_, rest) = List::split1OnTrue(metamodelica::AsArg::as_arg(&rest), &move |__a0: (metamodelica::Ref<DAE::Exp>, i32, i32), __a1: metamodelica::Ref<DAE::Exp>| minmaxTermEqual(&__a0, __a1), exp1.clone())?;
                    iter = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("i"), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    (exp1, _) = Expression::traverseExpBottomUp(exp1.clone(), &fnptr!(replaceSubscriptInCrefExp, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::Subscript>>), list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: iter.clone() })])?;
                    exp1 = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: sumReductionInfo.clone(), expr: sumExp.clone(), iterators: list![metamodelica::Ref::new(DAE::ReductionIterator { id: literal!("$sumIter"), exp: metamodelica::Ref::new(DAE::Exp::RANGE { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: max.clone() - min.clone() })] }), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: min.clone() }), step: None, stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: max.clone() }) }), guardExp: None, ty: DAE::T_INTEGER_DEFAULT().clone() })] });
                    resExp = buildAccumExpInEquations2(metamodelica::AsArg::as_arg(&rest), &(list![exp1.clone()]))?;
                    Ok(resExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp1, min, max), tail: rest }, Deref @ metamodelica::ListNode::Cons { head: exp0, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut iter: metamodelica::Ref<DAE::Exp>;
                    let mut resExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp1 = (*exp1).clone();
                    let mut rest = (*rest).clone();
                    let true = (intNe(min.clone(), max.clone())) else { return Err("pattern mismatch") };
                    ::match_deref::match_deref! { match &(Expression::r#typeof(exp1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    (_, rest) = List::split1OnTrue(metamodelica::AsArg::as_arg(&rest), &move |__a0: (metamodelica::Ref<DAE::Exp>, i32, i32), __a1: metamodelica::Ref<DAE::Exp>| minmaxTermEqual(&__a0, __a1), exp1.clone())?;
                    iter = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("i"), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    (exp1, _) = Expression::traverseExpBottomUp(exp1.clone(), &fnptr!(replaceSubscriptInCrefExp, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::Subscript>>), list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: iter.clone() })])?;
                    exp1 = metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: sumReductionInfo.clone(), expr: sumExp.clone(), iterators: list![metamodelica::Ref::new(DAE::ReductionIterator { id: literal!("$sumIter"), exp: metamodelica::Ref::new(DAE::Exp::RANGE { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: max.clone() - min.clone() })] }), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: min.clone() }), step: None, stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: max.clone() }) }), guardExp: None, ty: DAE::T_INTEGER_DEFAULT().clone() })] });
                    resExp = buildAccumExpInEquations2(metamodelica::AsArg::as_arg(&rest), &(list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp0.clone(), operator: DAE::Operator::ADD { ty: Expression::r#typeof(exp0.clone())? }, exp2: exp1.clone() })]))?;
                    Ok(resExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp1, _, _), tail: rest }, Deref @ metamodelica::ListNode::Nil) => {
                    let mut resExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    resExp = buildAccumExpInEquations2(metamodelica::AsArg::as_arg(&rest), &(list![exp1.clone()]))?;
                    Ok(resExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (exp1, _, _), tail: rest }, Deref @ metamodelica::ListNode::Cons { head: exp0, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut resExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    resExp = buildAccumExpInEquations2(metamodelica::AsArg::as_arg(&rest), &(list![metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp0.clone(), operator: DAE::Operator::ADD { ty: Expression::r#typeof(exp0.clone())? }, exp2: exp1.clone() })]))?;
                    Ok(resExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(foldOut)
}

pub(crate) fn replaceSubscriptInCrefExp(
    mut expIn: metamodelica::Ref<DAE::Exp>,
    mut subsIn: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut subsOut: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    (expOut, subsOut) = (match &*expIn {
        DAE::Exp::CREF { componentRef: cref, ty } => {
            let mut cref = (*cref).clone();
            cref = replaceFirstSubsInCref(metamodelica::AsArg::as_arg(&cref), &subsIn);
            (
                metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: cref.clone(),
                    ty: ty.clone(),
                }),
                subsIn,
            )
        }
        _ => (expIn, subsIn),
    });
    (expOut, subsOut)
}

fn minmaxTermEqual(
    mut minmaxTerm: &(metamodelica::Ref<DAE::Exp>, i32, i32),
    mut term: metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut b: bool;
    let mut term0: metamodelica::Ref<DAE::Exp>;
    (term0, _, _) = minmaxTerm.clone();
    b = expEqualNoCrefSubs(&term0, term)?;
    Ok(b)
}

pub(crate) fn equationEqualNoCrefSubs(
    mut e1: metamodelica::Ref<BackendDAE::Equation>,
    mut e2: metamodelica::Ref<BackendDAE::Equation>,
) -> bool {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (&*e1, &*e2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (referenceEq(&*(&*e1),&*(&*e2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e11, scalar: e12, .. }, Deref @ BackendDAE::Equation::EQUATION { exp: e21, scalar: e22, .. }) => {
                    let mut terms1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut commCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut res: bool = res.clone();
                    if boolAnd(expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e11), e21.clone())?, expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e12), e22.clone())?) {
                        res = true;
                    } else {
                        crefs1 = BackendEquation::equationCrefs(e1.clone())?;
                        crefs2 = BackendEquation::equationCrefs(e2.clone())?;
                        commCrefs = List::intersectionOnTrue(&crefs1, &crefs2, &fnptr!(ComponentReferenceBasics::crefEqualWithoutSubs, metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>))?;
                        if intEq(((crefs1).len() as i32), ((commCrefs).len() as i32)) && intEq(((crefs2).len() as i32), ((commCrefs).len() as i32)) {
                            terms1 = listAppend(Expression::allTerms(metamodelica::AsArg::as_arg(&e11)), Expression::allTerms(metamodelica::AsArg::as_arg(&e12)));
                            terms2 = listAppend(Expression::allTerms(metamodelica::AsArg::as_arg(&e21)), Expression::allTerms(metamodelica::AsArg::as_arg(&e22)));
                            (_, terms1, terms2) = List::intersection1OnTrue(terms1.clone(), terms2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| expEqualNoCrefSubs(&__a0, __a1))?;
                            res = (terms1).is_empty() && (terms2).is_empty();
                        } else {
                            res = false;
                        }
                    }
                    Ok((res, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e11, right: e12, .. }, Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e21, right: e22, .. }) => {
                    let mut res: bool = res.clone();
                    res = boolAnd(expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e11), e21.clone())?, expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e12), e22.clone())?);
                    Ok((res, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e11, right: e12, .. }, Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e21, right: e22, .. }) => {
                    let mut res: bool = res.clone();
                    res = boolAnd(expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e11), e21.clone())?, expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e12), e22.clone())?);
                    Ok((res, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1, exp: exp1, .. }, Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr2, exp: exp2, .. }) => {
                    let mut res: bool = res.clone();
                    res = boolAnd(ComponentReferenceBasics::crefEqualWithoutSubs(cr1.clone(), cr2.clone()), expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&exp1), exp2.clone())?);
                    Ok((res, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp1, .. }, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: exp2, .. }) => {
                    let mut res: bool = res.clone();
                    res = expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&exp1), exp2.clone())?;
                    Ok((res, res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ALGORITHM { alg: alg1, .. }, Deref @ BackendDAE::Equation::ALGORITHM { alg: alg2, .. }) => {
                    let mut explst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: bool = res.clone();
                    explst1 = Algorithm::getAllExps(metamodelica::AsArg::as_arg(&alg1))?;
                    explst2 = Algorithm::getAllExps(metamodelica::AsArg::as_arg(&alg2))?;
                    res = List::isEqualOnTrue(explst1.clone(), explst2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| expEqualNoCrefSubs(&__a0, __a1))?;
                    Ok((res, res.clone()))
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
                    Ok(false)
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

pub(crate) fn expEqualNoCrefSubs<'__b>(
    mut inExp1: &'__b metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut outEqual: bool;
    if referenceEq(&*(&**inExp1), &*(&*inExp2)) {
        outEqual = true;
        return Ok(outEqual);
    }
    if metamodelica::valueConstructor((&*&**inExp1))? != metamodelica::valueConstructor((&*&*inExp2))? {
        outEqual = false;
        return Ok(outEqual);
    }
    outEqual = (match &**inExp1 {
        DAE::Exp::ICONST { .. } => {
            let mut i: i32;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            i = metamodelica::Own::own(__pa0);
            var_field!((**inExp1).integer, DAE::Exp::ICONST).clone() == i
        }
        DAE::Exp::RCONST { .. } => {
            let mut r: metamodelica::Real;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            var_field!((**inExp1).real, DAE::Exp::RCONST).clone() == r
        }
        DAE::Exp::SCONST { .. } => {
            let mut s: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::SCONST { string: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            s = metamodelica::Own::own(__pa0);
            metamodelica::stringEq(&var_field!((**inExp1).string, DAE::Exp::SCONST), &s)
        }
        DAE::Exp::BCONST { .. } => {
            let mut b: bool;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            b = metamodelica::Own::own(__pa0);
            var_field!((**inExp1).bool, DAE::Exp::BCONST).clone() == b
        }
        DAE::Exp::ENUM_LITERAL { .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ENUM_LITERAL { name: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            AbsynUtil::pathEqual(var_field!((**inExp1).name, DAE::Exp::ENUM_LITERAL), &p)
        }
        DAE::Exp::CREF { .. } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            ComponentReferenceBasics::crefEqualWithoutSubs(
                var_field!((**inExp1).componentRef, DAE::Exp::CREF).clone(),
                cr,
            )
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
            var_field!((**inExp1).ty, DAE::Exp::ARRAY).clone() == ty
                && expEqualNoCrefSubsList(var_field!((**inExp1).array, DAE::Exp::ARRAY), expl)?
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
            var_field!((**inExp1).ty, DAE::Exp::MATRIX).clone() == ty
                && expEqualNoCrefSubsListList(var_field!((**inExp1).matrix, DAE::Exp::MATRIX), mexpl)?
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
            Expression::operatorEqual(var_field!((**inExp1).operator, DAE::Exp::BINARY), &op)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp1, DAE::Exp::BINARY), e1)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp2, DAE::Exp::BINARY), e2)?
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
            Expression::operatorEqual(var_field!((**inExp1).operator, DAE::Exp::LBINARY), &op)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp1, DAE::Exp::LBINARY), e1)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp2, DAE::Exp::LBINARY), e2)?
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
            Expression::operatorEqual(var_field!((**inExp1).operator, DAE::Exp::UNARY), &op)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::UNARY), e)?
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
            Expression::operatorEqual(var_field!((**inExp1).operator, DAE::Exp::LUNARY), &op)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::LUNARY), e)?
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
            Expression::operatorEqual(var_field!((**inExp1).operator, DAE::Exp::RELATION), &op)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp1, DAE::Exp::RELATION), e1)?
                && expEqualNoCrefSubs(var_field!((**inExp1).exp2, DAE::Exp::RELATION), e2)?
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
            expEqualNoCrefSubs(var_field!((**inExp1).expCond, DAE::Exp::IFEXP), e)?
                && expEqualNoCrefSubs(var_field!((**inExp1).expThen, DAE::Exp::IFEXP), e1)?
                && expEqualNoCrefSubs(var_field!((**inExp1).expElse, DAE::Exp::IFEXP), e2)?
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
            AbsynUtil::pathEqual(var_field!((**inExp1).path, DAE::Exp::CALL), &p)
                && expEqualNoCrefSubsList(var_field!((**inExp1).expLst, DAE::Exp::CALL), expl)?
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
            AbsynUtil::pathEqual(var_field!((**inExp1).path, DAE::Exp::RECORD), &p)
                && expEqualNoCrefSubsList(var_field!((**inExp1).exps, DAE::Exp::RECORD), expl)?
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
            AbsynUtil::pathEqual(var_field!((**inExp1).path, DAE::Exp::PARTEVALFUNCTION), &p)
                && expEqualNoCrefSubsList(var_field!((**inExp1).expList, DAE::Exp::PARTEVALFUNCTION), expl)?
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
            expEqualNoCrefSubs(var_field!((**inExp1).start, DAE::Exp::RANGE), e1)?
                && expEqualNoCrefSubs(var_field!((**inExp1).stop, DAE::Exp::RANGE), e2)?
                && expEqualNoCrefSubsOpt(var_field!((**inExp1).step, DAE::Exp::RANGE).clone(), oe)?
        }
        DAE::Exp::TUPLE { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            expEqualNoCrefSubsList(var_field!((**inExp1).PR, DAE::Exp::TUPLE), expl)?
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
            var_field!((**inExp1).ty, DAE::Exp::CAST).clone() == ty
                && expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::CAST), e)?
        }
        DAE::Exp::ASUB { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (var_field!((**inExp1).sub, DAE::Exp::ASUB).clone())
                    .into_iter()
                    .cloned()
                {
                    let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::ASUB { exp: __pa0, sub: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            expl2 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs).into_iter().cloned() {
                    let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::ASUB), e)? && expEqualNoCrefSubsList(&expl, expl2)?
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
            expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::SIZE), e)?
                && expEqualNoCrefSubsOpt(var_field!((**inExp1).sz, DAE::Exp::SIZE).clone(), oe)?
        }
        DAE::Exp::REDUCTION { .. } => inExp1.clone() == inExp2,
        DAE::Exp::LIST { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::LIST { valList: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            expEqualNoCrefSubsList(var_field!((**inExp1).valList, DAE::Exp::LIST), expl)?
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
            expEqualNoCrefSubs(var_field!((**inExp1).car, DAE::Exp::CONS), e1)?
                && expEqualNoCrefSubs(var_field!((**inExp1).cdr, DAE::Exp::CONS), e2)?
        }
        DAE::Exp::META_TUPLE { .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::META_TUPLE { listExp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            expEqualNoCrefSubsList(var_field!((**inExp1).listExp, DAE::Exp::META_TUPLE), expl)?
        }
        DAE::Exp::META_OPTION { .. } => {
            let mut oe: Option<metamodelica::Ref<DAE::Exp>>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::META_OPTION { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            oe = metamodelica::Own::own(__pa0);
            expEqualNoCrefSubsOpt(var_field!((**inExp1).exp, DAE::Exp::META_OPTION).clone(), oe)?
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
            AbsynUtil::pathEqual(var_field!((**inExp1).path, DAE::Exp::METARECORDCALL), &p)
                && expEqualNoCrefSubsList(var_field!((**inExp1).args, DAE::Exp::METARECORDCALL), expl)?
        }
        DAE::Exp::MATCHEXPRESSION { .. } => inExp1.clone() == inExp2,
        DAE::Exp::BOX { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::BOX { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::BOX), e)?
        }
        DAE::Exp::UNBOX { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            expEqualNoCrefSubs(var_field!((**inExp1).exp, DAE::Exp::UNBOX), e)?
        }
        DAE::Exp::SHARED_LITERAL { .. } => {
            let mut i: i32;
            let __pa0 = ::match_deref::match_deref! { match &(inExp2) {
                Deref @ DAE::Exp::SHARED_LITERAL { index: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            i = metamodelica::Own::own(__pa0);
            var_field!((**inExp1).index, DAE::Exp::SHARED_LITERAL).clone() == i
        }
        _ => false,
    });
    Ok(outEqual)
}

fn expEqualNoCrefSubsOpt(
    mut inExp1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inExp2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<bool> {
    let mut outEqual: bool;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    outEqual = (::match_deref::match_deref! { match &((inExp1, inExp2)) {
        (None, None) => true,
        (Some(__esc_e1), Some(__esc_e2)) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e1), e2.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqual)
}

fn expEqualNoCrefSubsList(
    mut inExpl1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExpl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<bool> {
    let mut outEqual: bool;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut rest_expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpl2.clone();
    if ((inExpl1).len() as i32) != ((inExpl2).len() as i32) {
        outEqual = false;
        return Ok(outEqual);
    }
    for mut e1 in &**inExpl1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_expl2 = metamodelica::Own::own(__pa1);
        if !(expEqualNoCrefSubs(metamodelica::AsArg::as_arg(&e1), e2)?) {
            outEqual = false;
            return Ok(outEqual);
        }
    }
    outEqual = true;
    Ok(outEqual)
}

fn expEqualNoCrefSubsListList(
    mut inExpl1: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inExpl2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<bool> {
    let mut outEqual: bool;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rest_expl2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = inExpl2.clone();
    if ((inExpl1).len() as i32) != ((inExpl2).len() as i32) {
        outEqual = false;
        return Ok(outEqual);
    }
    for mut expl1 in &**inExpl1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        expl2 = metamodelica::Own::own(__pa0);
        rest_expl2 = metamodelica::Own::own(__pa1);
        if !(expEqualNoCrefSubsList(metamodelica::AsArg::as_arg(&expl1), expl2)?) {
            outEqual = false;
            return Ok(outEqual);
        }
    }
    outEqual = true;
    Ok(outEqual)
}

fn buildBackendDAEForEquations(
    mut classEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut foldIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> {
    let mut foldOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    foldOut = 'mc: {
        let __mc_input = &**classEqs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(foldIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eq, tail: rest } => {
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut iterator: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut attr: BackendDAE::EquationAttributes;
                    let mut similarEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut foldEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq = (*eq).clone();
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eq.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2, attr: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs = metamodelica::Own::own(__pa0);
                    rhs = metamodelica::Own::own(__pa1);
                    source = metamodelica::Own::own(__pa2);
                    attr = metamodelica::Own::own(__pa3);
                    let true = (ComponentReferenceBasics::crefEqualWithoutSubs(Expression::expCref(&lhs)?, Expression::expCref(&rhs)?)) else { return Err("pattern mismatch") };
                    (similarEqs, rest) = List::separate1OnTrue(classEqs, &fnptr!(equationEqualNoCrefSubs, metamodelica::Ref<BackendDAE::Equation>, metamodelica::Ref<BackendDAE::Equation>), eq.clone())?;
                    iterator = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("i"), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::FOR_EQUATION { iter: iterator.clone(), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: ((similarEqs).len() as i32) }), body: metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: attr }), source: source.clone(), attr: attr });
                    foldEqs = buildBackendDAEForEquations(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(eq.clone(), foldIn.clone())));
                    Ok(foldEqs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eq, tail: rest } => {
                    let mut min: i32;
                    let mut max: i32;
                    let mut numCrefs: i32;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut iterator: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut attr: BackendDAE::EquationAttributes;
                    let mut similarEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut foldEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefMinMax: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>;
                    let mut eq = (*eq).clone();
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eq.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2, attr: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs = metamodelica::Own::own(__pa0);
                    rhs = metamodelica::Own::own(__pa1);
                    source = metamodelica::Own::own(__pa2);
                    attr = metamodelica::Own::own(__pa3);
                    (similarEqs, rest) = List::separate1OnTrue(classEqs, &fnptr!(equationEqualNoCrefSubs, metamodelica::Ref<BackendDAE::Equation>, metamodelica::Ref<BackendDAE::Equation>), eq.clone())?;
                    crefs = BackendEquation::equationCrefs(eq.clone())?;
                    crefs2 = BackendEquation::equationCrefs((similarEqs).get(1)?)?;
                    (crefs2, crefs, _) = List::intersection1OnTrue(crefs.clone(), crefs2.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                    numCrefs = ((crefs).len() as i32);
                    crefMinMax = List::thread3Map(crefs.clone().reverse(), List::fill(999999999, numCrefs), List::fill(0, numCrefs), &fnptr!(Util::make3Tuple, _, _, _))?;
                    crefMinMax = List::fold1(&similarEqs, &fnptr!(getCrefIdcsForEquation, metamodelica::Ref<BackendDAE::Equation>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>), crefs2.clone(), crefMinMax.clone())?;
                    min = 1;
                    max = ((similarEqs).len() as i32);
                    iterator = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("i"), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() });
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(setIteratorSubscriptCrefinEquation(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>))> + 'static>), (crefMinMax.clone(), iterator.clone(), crefs2.clone()))?) {
                        (Deref @ BackendDAE::Equation::EQUATION { exp: __pa4, scalar: __pa5, .. }, _) => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs = metamodelica::Own::own(__pa4);
                    rhs = metamodelica::Own::own(__pa5);
                    eq = metamodelica::Ref::new(BackendDAE::Equation::FOR_EQUATION { iter: iterator.clone(), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: min }), stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: max }), body: metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: source.clone(), attr: attr }), source: source.clone(), attr: attr });
                    foldEqs = buildBackendDAEForEquations(metamodelica::AsArg::as_arg(&rest), &(metamodelica::cons(eq.clone(), foldIn.clone())));
                    Ok(foldEqs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(foldIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    foldOut
}

fn getCrefIdcsForEquation(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut constCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut crefMinMaxIn: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>,
) -> metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)> {
    let mut crefMinMaxOut: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>;
    crefMinMaxOut = 'mc: {
        let __mc_input = (&*eq, crefMinMaxIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: _, .. }, crefMinMax) => {
                    let mut pos: i32;
                    let mut max: i32;
                    let mut min: i32;
                    let mut sub: i32;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
                    let mut refCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut refCrefMinMax: (metamodelica::Ref<DAE::ComponentRef>, i32, i32) = (metamodelica::Ref::new(DAE::ComponentRef::WILD), 0, 0);
                    let mut eqCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefMinMax = (*crefMinMax).clone();
                    eqCrefs = BackendEquation::equationCrefs(eq.clone())?;
                    eqCrefs = List::filter1OnTrue(eqCrefs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| ComponentReferenceBasics::crefNotInLst(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<bool> + 'static>), constCrefs.clone())?;
                    for mut cref in &*eqCrefs {
                        let mut cref = cref.clone();
                        let __pa0 = ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(&cref)?) {
                            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        sub = metamodelica::Own::own(__pa0);
                        pos = 1;
                        for mut refCrefMinMax in &*crefMinMax.clone() {
                            let mut refCrefMinMax = refCrefMinMax.clone();
                            (refCref, min, max) = refCrefMinMax.clone();
                            if ComponentReferenceBasics::crefEqualWithoutSubs(refCref.clone(), cref.clone()) {
                                        max = intMax(max, sub);
                                        min = intMin(min, sub);
                                        crefMinMax = List::replaceAt((refCref.clone(), min, max), pos, crefMinMax.clone())?;
                            }
                            pos = pos + 1;
                        }
                    }
                    Ok(crefMinMax.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(crefMinMaxIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    crefMinMaxOut
}

fn setIteratorSubscriptCrefinEquation(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut tplIn: &(
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>,
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>,
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tplOut: (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>,
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    );
    (outExp, tplOut) = 'mc: {
        let __mc_input = (&**inExp, tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, ty }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut min: i32;
                    let mut refCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut iterator1: metamodelica::Ref<DAE::Exp>;
                    let mut refCrefMinMax: (metamodelica::Ref<DAE::ComponentRef>, i32, i32) = (metamodelica::Ref::new(DAE::ComponentRef::WILD), 0, 0);
                    let mut crefMinMax1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>;
                    let mut cref = (*cref).clone();
                    let true = (!(List::exist1(metamodelica::AsArg::as_arg(&constCrefs), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1), cref.clone())?)) else { return Err("pattern mismatch") };
                    crefMinMax1 = metamodelica::nil();
                    for mut refCrefMinMax in &*crefMinMax0.clone() {
                        let mut refCrefMinMax = refCrefMinMax.clone();
                        (refCref, min, _) = refCrefMinMax.clone();
                        if ComponentReferenceBasics::crefEqualWithoutSubs(refCref.clone(), cref.clone()) {
                            (iterator1, _) = ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: iterator.clone(), operator: DAE::Operator::ADD { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: min - 1 }) }))?;
                            cref = replaceFirstSubsInCref(metamodelica::AsArg::as_arg(&cref), &(list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: iterator1.clone() })]));
                        } else {
                            crefMinMax1 = metamodelica::cons(refCrefMinMax.clone(), crefMinMax1.clone());
                        }
                    }
                    Ok((metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() }), (crefMinMax1.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut exp1 = (*exp1).clone();
                    let mut exp2 = (*exp2).clone();
                    let mut crefMinMax0 = (*crefMinMax0).clone();
                    let mut iterator = (*iterator).clone();
                    let mut constCrefs = (*constCrefs).clone();
                    let (__pa0, (__pa1, __pa2, __pa3)) = setIteratorSubscriptCrefinEquation(metamodelica::AsArg::as_arg(&exp1), tplIn);
                    exp1 = metamodelica::Own::own(__pa0);
                    crefMinMax0 = metamodelica::Own::own(__pa1);
                    iterator = metamodelica::Own::own(__pa2);
                    constCrefs = metamodelica::Own::own(__pa3);
                    let (__pa4, (__pa5, __pa6, __pa7)) = setIteratorSubscriptCrefinEquation(metamodelica::AsArg::as_arg(&exp2), &((crefMinMax0.clone(), iterator.clone(), constCrefs.clone())));
                    exp2 = metamodelica::Own::own(__pa4);
                    crefMinMax0 = metamodelica::Own::own(__pa5);
                    iterator = metamodelica::Own::own(__pa6);
                    constCrefs = metamodelica::Own::own(__pa7);
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: op, exp: exp1 }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut exp1 = (*exp1).clone();
                    let mut crefMinMax0 = (*crefMinMax0).clone();
                    let mut iterator = (*iterator).clone();
                    let mut constCrefs = (*constCrefs).clone();
                    let (__pa0, (__pa1, __pa2, __pa3)) = setIteratorSubscriptCrefinEquation(metamodelica::AsArg::as_arg(&exp1), tplIn);
                    exp1 = metamodelica::Own::own(__pa0);
                    crefMinMax0 = metamodelica::Own::own(__pa1);
                    iterator = metamodelica::Own::own(__pa2);
                    constCrefs = metamodelica::Own::own(__pa3);
                    Ok((metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: exp1.clone() }), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst: eLst, attr }, (crefMinMax0, iterator, constCrefs)) => {
                    let mut eLst = (*eLst).clone();
                    let mut crefMinMax0 = (*crefMinMax0).clone();
                    let mut iterator = (*iterator).clone();
                    let mut constCrefs = (*constCrefs).clone();
                    let (__pa0, (__pa1, __pa2, __pa3)) = List::mapFold(metamodelica::AsArg::as_arg(&eLst), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, i32)>, metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)| -> metamodelica::Result<_> { ::std::result::Result::Ok(setIteratorSubscriptCrefinEquation(&__a0, &__a1)) }, tplIn.clone())?;
                    eLst = metamodelica::Own::own(__pa0);
                    crefMinMax0 = metamodelica::Own::own(__pa1);
                    iterator = metamodelica::Own::own(__pa2);
                    constCrefs = metamodelica::Own::own(__pa3);
                    Ok((metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: eLst.clone(), attr: attr.clone() }), (crefMinMax0.clone(), iterator.clone(), constCrefs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tplIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tplOut)
}

fn getArrayVarCrefs(
    mut varIn: &metamodelica::Ref<BackendDAE::Var>,
    mut tplIn: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            i32,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> (
    metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut tplOut: (
        metamodelica::List<(
            metamodelica::Ref<DAE::ComponentRef>,
            i32,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    tplOut = 'mc: {
        let __mc_input = (&**varIn, &tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { varName: cref, .. }, (tplLst, arrVars)) => {
                    let mut idx: i32;
                    let mut crefHead: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefTailOpt: Option<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut tpl: (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>);
                    let mut tplLst = (*tplLst).clone();
                    let mut arrVars = (*arrVars).clone();
                    let true = (ComponentReference::isArrayElement(metamodelica::AsArg::as_arg(&cref))) else { return Err("pattern mismatch") };
                    (crefHead, idx, crefTailOpt) = ComponentReference::stripArrayCref(metamodelica::AsArg::as_arg(&cref))?;
                    if (crefTailOpt).is_some() {
                        crefLst = list![Util::getOption(crefTailOpt.clone())?];
                    } else {
                        crefLst = metamodelica::nil();
                    }
                    (tplLst, arrVars) = addToArrayCrefLst(metamodelica::AsArg::as_arg(&tplLst), varIn, &((crefHead.clone(), idx, crefLst.clone())), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&arrVars))?;
                    tpl = (tplLst.clone(), arrVars.clone());
                    Ok(tpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(tplIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    tplOut
}

fn addToArrayCrefLst(
    mut tplLstIn: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    mut varIn: &metamodelica::Ref<BackendDAE::Var>,
    mut tplRef: &(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
    mut tplLstFoldIn: &metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    mut varLstIn: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut tplLstFoldOut: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        i32,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    )>;
    let mut varLstOut: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (tplLstFoldOut, varLstOut) = 'mc: {
        let __mc_input = (&**tplLstIn, tplRef);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (cref0, idx0, tailCrefs0), tail: rest }, (cref1, idx1, Deref @ metamodelica::ListNode::Cons { head: crefTailRef, tail: Deref @ metamodelica::ListNode::Nil })) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>;
                    let mut tailCrefs0 = (*tailCrefs0).clone();
                    let true = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cref0), metamodelica::AsArg::as_arg(&cref1))?) else { return Err("pattern mismatch") };
                    if List::notMember(crefTailRef.clone(), tailCrefs0.clone()) {
                        tailCrefs0 = metamodelica::cons(crefTailRef.clone(), tailCrefs0.clone());
                        varLst = metamodelica::cons(varIn.clone(), varLstIn.clone());
                    } else {
                        varLst = varLstIn.clone();
                    }
                    tplLst = metamodelica::cons((cref0.clone(), intMax(idx0.clone(), idx1.clone()), tailCrefs0.clone()), rest.clone());
                    tplLst = List::append_reverse(&tplLst, tplLstFoldIn.clone());
                    Ok((tplLst.clone(), varLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (cref0, idx0, tailCrefs0), tail: rest }, (cref1, _, _)) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>;
                    let false = (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cref0), metamodelica::AsArg::as_arg(&cref1))?) else { return Err("pattern mismatch") };
                    (tplLst, varLst) = addToArrayCrefLst(metamodelica::AsArg::as_arg(&rest), varIn, tplRef, &(metamodelica::cons((cref0.clone(), idx0.clone(), tailCrefs0.clone()), tplLstFoldIn.clone())), varLstIn)?;
                    Ok((tplLst.clone(), varLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, (cref1, idx1, tailCrefs1)) => {
                    let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>;
                    tplLst = metamodelica::cons((cref1.clone(), idx1.clone(), tailCrefs1.clone()), tplLstFoldIn.clone());
                    Ok((tplLst.clone(), metamodelica::cons(varIn.clone(), varLstIn.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((tplLstFoldOut, varLstOut))
}

fn getArrayVars(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut tplIn: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut tplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    tplOut = (::match_deref::match_deref! { match &((varIn.clone(), tplIn.clone())) {
        (Deref @ BackendDAE::Var { varName: cref, .. }, (varLstIn, arrVarLstIn)) if (ComponentReference::isArrayElement(metamodelica::AsArg::as_arg(&cref))) => {
            (varLstIn.clone(), metamodelica::cons(varIn, arrVarLstIn.clone()))
        },
        (_, (varLstIn, arrVarLstIn)) => {
            (metamodelica::cons(varIn, varLstIn.clone()), arrVarLstIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    tplOut
}

fn dispatchLoopEquations(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut arrayCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut tplIn: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut tplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    );
    tplOut = (::match_deref::match_deref! { match &(tplIn) {
        (classEqs, mixEqs, nonArrEqs) => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut arrCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut nonArrCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut classEqs = (*classEqs).clone();
            let mut mixEqs = (*mixEqs).clone();
            let mut nonArrEqs = (*nonArrEqs).clone();
            crefs = BackendEquation::equationCrefs(eqIn.clone())?;
            (arrCrefs, nonArrCrefs) = List::separate1OnTrue(&crefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>| crefPartlyEqualToCrefs(__a0, &__a1), arrayCrefs)?;
            if (nonArrCrefs).is_empty() {
                classEqs = metamodelica::cons(eqIn, classEqs.clone());
            } else if (arrCrefs).is_empty() {
                nonArrEqs = metamodelica::cons(eqIn, nonArrEqs.clone());
            } else {
                mixEqs = metamodelica::cons(eqIn, mixEqs.clone());
            }
            (classEqs.clone(), mixEqs.clone(), nonArrEqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tplOut)
}

fn crefPartlyEqualToCrefs(
    mut cref0: metamodelica::Ref<DAE::ComponentRef>,
    mut crefLst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<bool> {
    let mut b: bool;
    b = List::exist1(
        crefLst,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>,
               __a1: metamodelica::Ref<DAE::ComponentRef>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(crefPartlyEqual(&__a0, &__a1)) },
        cref0,
    )?;
    Ok(b)
}

fn crefPartlyEqual(
    mut cref0: &metamodelica::Ref<DAE::ComponentRef>,
    mut cref1: &metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut partlyEq: bool;
    partlyEq = (::match_deref::match_deref! { match (cref0, cref1) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            metamodelica::stringEq(&var_field!((**cref0).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cref1).ident, DAE::ComponentRef::CREF_IDENT))
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref01, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cref11, .. }) => {
            let mut b: bool;
            if metamodelica::stringEq(&var_field!((**cref0).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cref1).ident, DAE::ComponentRef::CREF_QUAL)) {
                b = crefPartlyEqual(cref01, cref11);
            } else {
                b = false;
            }
            b
        },
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
            metamodelica::stringEq(&var_field!((**cref0).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cref1).ident, DAE::ComponentRef::CREF_IDENT))
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
            metamodelica::stringEq(&var_field!((**cref0).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cref1).ident, DAE::ComponentRef::CREF_QUAL))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    partlyEq
}

pub(crate) fn reduceLoopExpressions(
    mut expIn: &metamodelica::Ref<DAE::Exp>,
    mut maxSub: i32,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut notRemoved: bool;
    (expOut, notRemoved) = 'mc: {
        let __mc_input = &**expIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
                    let mut b: bool;
                    b = intLe(getIndexSubScript(&(((ComponentReferenceBasics::crefSubs(metamodelica::AsArg::as_arg(&cref))?)).head().cloned()?))?, maxSub);
                    Ok((expIn.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut exp1 = (*exp1).clone();
                    let mut exp2 = (*exp2).clone();
                    (exp1, b1) = reduceLoopExpressions(metamodelica::AsArg::as_arg(&exp1), maxSub);
                    (exp2, b2) = reduceLoopExpressions(metamodelica::AsArg::as_arg(&exp2), maxSub);
                    if b1 && !(b2) {
                        exp = exp1.clone();
                    } else if b2 && !(b1) {
                        exp = exp2.clone();
                    } else {
                        exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
                    }
                    Ok((exp.clone(), boolOr(b1, b2)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { exp, .. } => {
                    let mut b: bool;
                    let mut exp = (*exp).clone();
                    (exp, b) = reduceLoopExpressions(metamodelica::AsArg::as_arg(&exp), maxSub);
                    Ok((exp.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((expIn.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (expOut, notRemoved)
}

pub(crate) fn insertSUMexp(
    mut expIn: &metamodelica::Ref<DAE::Exp>,
    mut tplIn: &(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>),
) {
    let mut expOut: metamodelica::Ref<DAE::Exp>;
    let mut tplOut: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>);
    (expOut, tplOut) = (::match_deref::match_deref! { match &((&**expIn, tplIn)) {
        (Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 }, _) => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            (exp1, _) = insertSUMexp(metamodelica::AsArg::as_arg(&exp1), tplIn);
            (exp2, _) = insertSUMexp(metamodelica::AsArg::as_arg(&exp2), tplIn);
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), tplIn.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: op, exp: exp1 }, _) => {
            let mut exp1 = (*exp1).clone();
            (exp1, _) = insertSUMexp(metamodelica::AsArg::as_arg(&exp1), tplIn);
            (metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: exp1.clone() }), tplIn.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, (cref0, repl)) if (crefPartlyEqual(metamodelica::AsArg::as_arg(&cref0), metamodelica::AsArg::as_arg(&cref1))) => {
            (repl.clone(), tplIn.clone())
        },
        _ => {
            (expIn.clone(), tplIn.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (expOut, tplOut)
}

fn getIndexSubScript(mut sub: &metamodelica::Ref<DAE::Subscript>) -> Result<i32> {
    let mut int: i32;
    let __pa0 = ::match_deref::match_deref! { match &((*sub)) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    int = metamodelica::Own::own(__pa0);
    Ok(int)
}

pub(crate) fn replaceFirstSubsInCref(
    mut crefIn: &metamodelica::Ref<DAE::ComponentRef>,
    mut subs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut crefOut: metamodelica::Ref<DAE::ComponentRef>;
    crefOut = (match &**crefIn {
        DAE::ComponentRef::CREF_QUAL {
            ident,
            identType,
            subscriptLst,
            componentRef: cref,
        } => {
            let mut subscriptLst = (*subscriptLst).clone();
            let mut cref = (*cref).clone();
            if List::hasOneElement(metamodelica::AsArg::as_arg(&subscriptLst)) {
                subscriptLst = subs.clone();
            }
            cref = replaceFirstSubsInCref(metamodelica::AsArg::as_arg(&cref), subs);
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: ident.clone(),
                identType: identType.clone(),
                subscriptLst: subscriptLst.clone(),
                componentRef: cref.clone(),
            })
        }
        DAE::ComponentRef::CREF_IDENT {
            ident,
            identType,
            subscriptLst,
        } => {
            let mut subscriptLst = (*subscriptLst).clone();
            if List::hasOneElement(metamodelica::AsArg::as_arg(&subscriptLst)) {
                subscriptLst = subs.clone();
            }
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: ident.clone(),
                identType: identType.clone(),
                subscriptLst: subscriptLst.clone(),
            })
        }
        _ => crefIn.clone(),
    });
    crefOut
}
