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

use crate::AdjacencyMatrix;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendInline;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::Matching;
use crate::RewriteRules;
use crate::SynchronousFeatures;
use crate::Tearing;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::HashTable2;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub(crate) fn simplifyAllExpressions(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE;
    let mut removedEqsList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    BackendDAEUtil::traverseBackendDAEExpsNoCopyWithUpdate(
        &outDAE,
        (std::sync::Arc::new(ExpressionSimplify::simplify1TraverseHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        0,
    )?;
    shared = outDAE.shared.clone();
    for mut eq in &*BackendEquation::equationList(shared.removedEqs.clone())? {
        removedEqsList = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Nil }, .. } => removedEqsList,
            _ => metamodelica::cons(eq.clone(), removedEqsList),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    assign_field!(
        shared.removedEqs =
            BackendEquation::listEquation(&(metamodelica::Dangerous::listReverseInPlace(removedEqsList)))?
    );
    assign_field!(outDAE.shared = shared);
    Ok(outDAE)
}

// =============================================================================
// simplifyInStream
//
// OM introduces $OMC$PositiveMax which can simplified using min or max attribute
// see Modelica spec for inStream
// author: Vitalij Ruge
// see. #3885, 4441, 5104
// =============================================================================
pub(crate) fn simplifyInStream(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = dae.shared.clone();
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = dae.eqs.clone();
    let mut vars: metamodelica::List<BackendDAE::Variables> = ({
        let mut __acc: metamodelica::List<BackendDAE::Variables> = metamodelica::nil();
        for mut eq in (eqs.clone()).into_iter().cloned() {
            let __x = eq.orderedVars.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    vars = metamodelica::cons(shared.globalKnownVars.clone(), vars);
    vars = metamodelica::cons(shared.localKnownVars.clone(), vars);
    BackendDAEUtil::traverseBackendDAEExpsNoCopyWithUpdate(
        &dae,
        (std::sync::Arc::new(simplifyInStreamWork)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        metamodelica::List<BackendDAE::Variables>,
                    )
                        -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<BackendDAE::Variables>)>
                    + 'static,
            >),
        vars,
    )?;
    Ok(dae)
}

fn simplifyInStreamWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVars: metamodelica::List<BackendDAE::Variables>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<BackendDAE::Variables>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVars: metamodelica::List<BackendDAE::Variables> = inVars;
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), &simplifyInStreamWork2, outVars.clone())?;
    if !(ExpressionBasics::expEqual(&outExp, inExp)?) {
        (outExp, _) = ExpressionSimplify::simplify(outExp)?;
    }
    Ok((outExp, outVars))
}

fn simplifyInStreamWork2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVars: metamodelica::List<BackendDAE::Variables>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<BackendDAE::Variables>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVars: metamodelica::List<BackendDAE::Variables> = inVars;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: expr, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ret: metamodelica::Ref<DAE::Exp>;
            let mut eMin: Option<metamodelica::Ref<DAE::Exp>>;
            let mut eMax: Option<metamodelica::Ref<DAE::Exp>>;
            (eMin, eMax) = simplifyInStreamGetMinMaxAttributes(metamodelica::AsArg::as_arg(&cr), &outVars);
            tp = ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr))?;
            ret = if (Util::applyOptionOrDefault(eMax, &Expression::isNegativeOrZero, false)?) {Expression::createZeroExpression(tp)?} else if (Util::applyOptionOrDefault(eMin, &({ let __pe_b1 = expr.clone(); move |__pe_a0| Expression::isGreaterOrEqual(__pe_a0, __pe_b1.clone()) }), false)?) {e.clone()} else {Expression::makePureBuiltinCall(literal!("max"), list![e.clone(), expr.clone()], tp)};
            ret
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, tail: Deref @ metamodelica::ListNode::Cons { head: expr, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut ret: metamodelica::Ref<DAE::Exp>;
            let mut eMin: Option<metamodelica::Ref<DAE::Exp>>;
            let mut eMax: Option<metamodelica::Ref<DAE::Exp>>;
            (eMin, eMax) = simplifyInStreamGetMinMaxAttributes(metamodelica::AsArg::as_arg(&cr), &outVars);
            ret = if (Util::applyOptionOrDefault(eMin, &Expression::isPositiveOrZero, false)?) {Expression::createZeroExpression(tp.clone())?} else if (Util::applyOptionOrDefault(eMax, &({ let __pe_b0 = Expression::negate(expr.clone())?; move |__pe_a1| Expression::isGreaterOrEqual(__pe_b0.clone(), __pe_a1) }), false)?) {e.clone()} else {Expression::makePureBuiltinCall(literal!("max"), list![e.clone(), expr.clone()], tp.clone())};
            ret
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: expr, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            Expression::makePureBuiltinCall(literal!("max"), list![e.clone(), expr.clone()], Expression::r#typeof(e.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$inStreamDiv" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: expr, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut ret: metamodelica::Ref<DAE::Exp>;
            let mut e = (*e).clone();
            (e, _) = ExpressionSimplify::simplify(e.clone())?;
            ret = (match &*e.clone() {
        DAE::Exp::BINARY { exp1: a, operator: DAE::Operator::DIV { .. }, exp2: b } if (Expression::isZero(metamodelica::AsArg::as_arg(&a))? && Expression::isZero(metamodelica::AsArg::as_arg(&b))?) => {
            expr.clone()
        },
        _ => {
            e.clone()
        },
    });
            ret
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outVars))
}

fn simplifyInStreamGetMinMaxAttributes(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inVars: &metamodelica::List<BackendDAE::Variables>,
) -> (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>) {
    let mut outMin: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut outMax: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    '__loop0: for mut vars in &**inVars {
        if '__try1: {
            (v, _) = unwrap_break_err!(BackendVariable::getVarSingle(cr, metamodelica::AsArg::as_arg(&vars)), '__try1);
            (outMin, outMax) = BackendVariable::getMinMaxAttribute(&v);
            break '__loop0;
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    (outMin, outMax)
}

// =============================================================================
// simplify time independent function calls
//
// public functions:
//   - simplifyTimeIndepFuncCalls
// =============================================================================
pub(crate) fn simplifyTimeIndepFuncCalls(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &fnptr!(
            simplifyTimeIndepFuncCalls0,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            bool
        ),
        false,
    )?;
    outDAE = simplifyTimeIndepFuncCallsShared(&outDAE)?;
    Ok(outDAE)
}

fn simplifyTimeIndepFuncCalls0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outChanged: bool;
    (osyst, outShared, outChanged) = 'mc: {
        let __mc_input = (isyst.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared) => {
                    let (_, (_, _, true)) = (BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?) else { return Err("pattern mismatch") };
                    let (_, (_, _, true)) = (BackendDAEUtil::traverseBackendDAEExpsEqns(syst.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?) else { return Err("pattern mismatch") };
                    Ok((isyst.clone(), inShared.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inShared.clone(), inChanged))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, outShared, outChanged)
}

fn traverserExpsimplifyTimeIndepFuncCalls(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, BackendDAE::Variables, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (&*inExp, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    let mut zero: metamodelica::Ref<DAE::Exp>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    let false = (BackendVariable::isVarOnTopLevelAndInput(&var)) else { return Err("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(&(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp))))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous"))))) { return Err("guard") }
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    let false = (BackendVariable::isInput(&var)) else { return Err("pattern mismatch") };
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous"))))) { return Err("guard") }
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. } }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous"))))) { return Err("guard") }
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::ListNode::Nil }, attr }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("pre"))) || metamodelica::stringEq(&idn, &(literal!("previous"))))) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut negate: bool;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&aliasvars))?;
                    (cr, negate) = BackendVariable::getAlias(&var)?;
                    e = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    e = if (negate) {Expression::negate(e.clone())?} else {e.clone()};
                    (e, _) = ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: idn.clone() }), expLst: list![e.clone()], attr: attr.clone() }))?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), &fnptr!(traverserExpsimplifyTimeIndepFuncCalls, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)), (globalKnownVars.clone(), aliasvars.clone(), false))?;
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("change"))) || metamodelica::stringEq(&idn, &(literal!("edge"))))) { return Err("guard") }
                    let mut zero: metamodelica::Ref<DAE::Exp>;
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    zero = Expression::arrayFill(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp)), metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("change"))) || metamodelica::stringEq(&idn, &(literal!("edge"))))) { return Err("guard") }
                    let mut zero: metamodelica::Ref<DAE::Exp>;
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&aliasvars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    zero = Expression::arrayFill(Expression::arrayDimension(metamodelica::AsArg::as_arg(&tp)), metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("change"))) || metamodelica::stringEq(&idn, &(literal!("edge"))))) { return Err("guard") }
                    Ok((metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::ListNode::Nil }, attr }, (globalKnownVars, aliasvars, _)) => {
                    if !((metamodelica::stringEq(&idn, &(literal!("change"))) || metamodelica::stringEq(&idn, &(literal!("edge"))))) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut negate: bool;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&aliasvars))?;
                    (cr, negate) = BackendVariable::getAlias(&var)?;
                    e = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    e = if (negate) {Expression::negate(e.clone())?} else {e.clone()};
                    (e, _) = ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: idn.clone() }), expLst: list![e.clone()], attr: attr.clone() }))?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), &fnptr!(traverserExpsimplifyTimeIndepFuncCalls, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)), (globalKnownVars.clone(), aliasvars.clone(), false))?;
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn simplifyTimeIndepFuncCallsShared(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    shared = inDAE.shared.clone();
    BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(
        &(shared.globalKnownVars.clone()),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpsimplifyTimeIndepFuncCalls,
                metamodelica::Ref<DAE::Exp>,
                (BackendDAE::Variables, BackendDAE::Variables, bool)
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        )> + 'static,
                >),
            (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false),
        ),
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        shared.initialEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpsimplifyTimeIndepFuncCalls,
                metamodelica::Ref<DAE::Exp>,
                (BackendDAE::Variables, BackendDAE::Variables, bool)
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        )> + 'static,
                >),
            (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false),
        ),
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        shared.removedEqs.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpsimplifyTimeIndepFuncCalls,
                metamodelica::Ref<DAE::Exp>,
                (BackendDAE::Variables, BackendDAE::Variables, bool)
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        )> + 'static,
                >),
            (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false),
        ),
    )?;
    let (__asg0_0, _) = traverseEventInfoExps(
        shared.eventInfo.clone(),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                traverserExpsimplifyTimeIndepFuncCalls,
                metamodelica::Ref<DAE::Exp>,
                (BackendDAE::Variables, BackendDAE::Variables, bool)
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (BackendDAE::Variables, BackendDAE::Variables, bool),
                        )> + 'static,
                >),
            (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false),
        ),
    )?;
    assign_field!(shared.eventInfo = __asg0_0.clone());
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: inDAE.eqs.clone(),
        shared: shared,
    });
    Ok(outDAE)
}

fn traverseEventInfoExps<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eventInfo: BackendDAE::EventInfo,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >,
    mut arg: T,
) -> Result<(BackendDAE::EventInfo, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >;

    let mut eventInfo: BackendDAE::EventInfo = eventInfo;
    let mut arg: T = arg;
    arg = DoubleEnded::mapFoldNoCopy(
        eventInfo.zeroCrossings.zc.clone(),
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone();
            move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, &*__pe_b1, __pe_a2)
        }),
        arg,
    )?;
    arg = DoubleEnded::mapFoldNoCopy(
        eventInfo.samples.zc.clone(),
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone();
            move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, &*__pe_b1, __pe_a2)
        }),
        arg,
    )?;
    arg = DoubleEnded::mapFoldNoCopy(
        eventInfo.relations.zc.clone(),
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> = func.clone();
            move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, &*__pe_b1, __pe_a2)
        }),
        arg,
    )?;
    Ok((eventInfo, arg))
}

fn traverseZeroCrossingExps<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut zc: BackendDAE::ZeroCrossing,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)>,
    mut arg: T,
) -> Result<(BackendDAE::ZeroCrossing, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >;

    let mut zc: BackendDAE::ZeroCrossing = zc;
    let mut arg: T = arg;
    let mut relation: metamodelica::Ref<DAE::Exp>;
    (relation, arg) = Expression::traverseExpBottomUp(zc.relation_.clone(), func, arg)?;
    if !(referenceEq(&*(&*relation), &*(zc.relation_.clone()))) {
        zc.relation_ = relation;
    }
    Ok((zc, arg))
}

fn toplevelInputOrUnfixed(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = BackendVariable::isVarOnTopLevelAndInput(inVar)
        || BackendVariable::isParam(inVar) && !(BackendVariable::varFixed(inVar));
    b
}

fn traversingTimeEqnsFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, ty: _ }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, b1, b2)) => {
                    let mut vlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?) {
                        (__pa0, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    vlst = metamodelica::Own::own(__pa0);
                    let false = (List::none(&vlst, &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(toplevelInputOrUnfixed(&__a0)) })?) else { return Err("pattern mismatch") };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, true, b2)) => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    var = metamodelica::Own::own(__pa0);
                    let DAE::INPUT { .. } = (BackendVariable::getVarDirection(&var)) else { return Err("pattern mismatch") };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), true, b2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, b1, true)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, (b, _, _, _, _)) => {
                    Ok((e.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

pub(crate) fn countSimpleEquations(
    mut inDlow: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inM: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<i32> {
    let mut outSimpleEqns: i32;
    outSimpleEqns = (::match_deref::match_deref! { match &(inDlow) {
        dlow => {
            let mut n: i32;
            let (_, (_, __pa0)) = AdjacencyMatrix::traverseAdjacencyMatrix(inM.clone(), &move |__a0: metamodelica::List<i32>, __a1: i32, __a2: (metamodelica::Ref<BackendDAE::BackendDAE>, i32)| -> metamodelica::Result<_> { ::std::result::Result::Ok(countSimpleEquationsFinder(&__a0, __a1, __a2)) }, (dlow.clone(), 0))?;
            n = metamodelica::Own::own(__pa0);
            n
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSimpleEqns)
}

fn countSimpleEquationsFinder(
    mut elem: &metamodelica::List<i32>,
    mut pos: i32,
    mut inTpl: (metamodelica::Ref<BackendDAE::BackendDAE>, i32),
) -> (
    metamodelica::List<i32>,
    (metamodelica::Ref<BackendDAE::BackendDAE>, i32),
) {
    let mut outList: metamodelica::List<i32>;
    let mut outTpl: (metamodelica::Ref<BackendDAE::BackendDAE>, i32);
    (outList, outTpl) = 'mc: {
        let __mc_input = &inTpl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dae @ Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst, tail: Deref @ metamodelica::ListNode::Nil }, shared }, n) => {
                    let mut l: i32;
                    let mut n_1: i32;
                    l = ((elem).len() as i32);
                    let true = (intLt(l, 3)) else { return Err("pattern mismatch") };
                    let true = (intGt(l, 0)) else { return Err("pattern mismatch") };
                    countsimpleEquation(elem, l, pos, metamodelica::AsArg::as_arg(&syst), metamodelica::AsArg::as_arg(&shared))?;
                    n_1 = n.clone() + 1;
                    Ok((metamodelica::nil(), (dae.clone(), n_1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outList, outTpl)
}

fn countsimpleEquation(
    mut elem: &metamodelica::List<i32>,
    mut length: i32,
    mut pos: i32,
    mut syst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**elem, &**shared);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::JACOBIAN { .. }, .. }) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cre: metamodelica::Ref<DAE::Exp>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    vars = BackendVariable::daeVars(syst);
                    var = BackendVariable::getVarAt(&vars, intAbs(i.clone()))?;
                    let false = (BackendVariable::isStateorStateDerVar(&var)) else { return Err("pattern mismatch") };
                    eqns = BackendEquation::getEqnsFromEqSystem(syst);
                    eqn = BackendEquation::get(eqns.clone(), pos)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    e2 = metamodelica::Own::own(__pa1);
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared);
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), &fnptr!(traversingTimeEqnsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)), (false, vars.clone(), globalKnownVars.clone(), true, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), &fnptr!(traversingTimeEqnsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)), (false, vars.clone(), globalKnownVars.clone(), true, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = BackendVariable::varCref(&var);
                    cre = Expression::crefExp(cr.clone())?;
                    ::match_deref::match_deref! { match &(ExpressionSolve::solve(e1.clone(), e2.clone(), cre.clone(), None)?) {
                        (_, Deref @ metamodelica::ListNode::Nil) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cre: metamodelica::Ref<DAE::Exp>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    vars = BackendVariable::daeVars(syst);
                    var = BackendVariable::getVarAt(&vars, intAbs(i.clone()))?;
                    let false = (BackendVariable::isStateorStateDerVar(&var)) else { return Err("pattern mismatch") };
                    eqns = BackendEquation::getEqnsFromEqSystem(syst);
                    eqn = BackendEquation::get(eqns.clone(), pos)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    e2 = metamodelica::Own::own(__pa1);
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared);
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), &fnptr!(traversingTimeEqnsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)), (false, vars.clone(), globalKnownVars.clone(), false, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), &fnptr!(traversingTimeEqnsFinder, metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)), (false, vars.clone(), globalKnownVars.clone(), false, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = BackendVariable::varCref(&var);
                    cre = Expression::crefExp(cr.clone())?;
                    ::match_deref::match_deref! { match &(ExpressionSolve::solve(e1.clone(), e2.clone(), cre.clone(), None)?) {
                        (_, Deref @ metamodelica::ListNode::Nil) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    eqns = BackendEquation::getEqnsFromEqSystem(syst);
                    eqn = BackendEquation::get(eqns.clone(), pos)?;
                    (cr, _, _, _, _) = BackendEquation::derivativeEquation(&eqn)?;
                    vars = BackendVariable::daeVars(syst);
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), &vars)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    eqns = BackendEquation::getEqnsFromEqSystem(syst);
                    let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), pos)?) {
                        __pa0 @ Deref @ BackendDAE::Equation::EQUATION { .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn = metamodelica::Own::own(__pa0);
                    BackendEquation::aliasEquation(&eqn)?;
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

// =============================================================================
// remove parameters stuff
//
// =============================================================================
pub(crate) fn removeParameters(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match inDAE {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            (repl, _) = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(removeParametersFinder, metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, BackendDAE::Variables))> + 'static>), (repl, globalKnownVars.clone()))?;
            (globalKnownVars, repl) = replaceFinalVars(1, globalKnownVars.clone(), repl)?;
            (globalKnownVars, repl) = replaceFinalVars(1, globalKnownVars.clone(), repl)?;
            if Flags::isSet(Flags::DUMP_PARAM_REPL.clone())? {
                BackendVarTransform::dumpReplacements(&repl)?;
            }
            systs = List::map1(systs.clone(), &removeParameterswork, repl)?;
            assign_field!(shared.globalKnownVars = globalKnownVars.clone());
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

fn removeParameterswork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut lsteqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            (vars, _) = replaceFinalVars(1, vars.clone(), repl.clone())?;
            (lsteqns, _) = BackendVarTransform::replaceEquations(BackendEquation::equationList(eqns.clone())?, &repl, None)?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = BackendEquation::listEquation(&lsteqns)?,
                syst.m = None,
                syst.mT = None
            );
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osyst)
}

fn removeParametersFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (BackendVarTransform::VariableReplacements, BackendDAE::Variables),
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(exp), .. }, (repl, vars)) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    (exp1, _) = Expression::traverseExpBottomUp(exp.clone(), &fnptr!(BackendDAEUtil::replaceCrefsWithValues, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::Ref<DAE::ComponentRef>)), (vars.clone(), varName.clone()))?;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp1.clone(), None)?;
                    Ok((v.clone(), (repl_1.clone(), vars.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn replaceFinalVars(
    mut inNumRepl: i32,
    mut inVars: BackendDAE::Variables,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> Result<(BackendDAE::Variables, BackendVarTransform::VariableReplacements)> {
    '__tco: loop {
        let mut numrepl: i32;
        let mut globalKnownVars1: BackendDAE::Variables;
        let mut repl1: BackendVarTransform::VariableReplacements;
        if intEq(0, inNumRepl) {
            return Ok((inVars, inRepl));
        } else {
            let (__pa0, (__pa1, __pa2)) = BackendVariable::traverseBackendDAEVarsWithUpdate(
                inVars,
                (std::sync::Arc::new(replaceFinalVarTraverser)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::Var>,
                                (BackendVarTransform::VariableReplacements, i32),
                            ) -> Result<(
                                metamodelica::Ref<BackendDAE::Var>,
                                (BackendVarTransform::VariableReplacements, i32),
                            )> + 'static,
                    >),
                (inRepl, 0),
            )?;
            globalKnownVars1 = metamodelica::Own::own(__pa0);
            repl1 = metamodelica::Own::own(__pa1);
            numrepl = metamodelica::Own::own(__pa2);
            {
                (inNumRepl, inVars, inRepl) = (numrepl, globalKnownVars1, repl1);
                continue '__tco;
            }
        }
    }
}

fn replaceFinalVarTraverser(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (BackendVarTransform::VariableReplacements, i32),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (BackendVarTransform::VariableReplacements, i32),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (BackendVarTransform::VariableReplacements, i32);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Var { bindExp: Some(Deref @ DAE::Exp::CALL { .. }), .. }, _) => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, bindExp: Some(e), values: attr, .. }, (repl, numrepl)) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut attr = (*attr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    v1 = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    repl_1 = addConstExpReplacement(e1.clone(), cr.clone(), repl.clone());
                    (attr, repl_1) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, BackendVarTransform::VariableReplacements), repl_1.clone())?;
                    v1 = BackendVariable::setVarAttributes(v1.clone(), attr.clone());
                    Ok((v1.clone(), (repl_1.clone(), numrepl.clone() + 1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { values: attr, .. }, (repl, numrepl)) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut new_attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                    let mut repl = (*repl).clone();
                    (new_attr, repl) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, BackendVarTransform::VariableReplacements), repl.clone())?;
                    v1 = BackendVariable::setVarAttributes(v.clone(), new_attr.clone());
                    Ok((v1.clone(), (repl.clone(), numrepl.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, outTpl))
}

fn addConstExpReplacement(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> BackendVarTransform::VariableReplacements {
    let mut outRepl: BackendVarTransform::VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = inRepl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Expression::isConst(inExp.clone())?) else {
                return Err("pattern mismatch");
            };
            Ok(BackendVarTransform::addReplacement(
                inRepl.clone(),
                cr.clone(),
                inExp.clone(),
                None,
            )?)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inRepl.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outRepl
}

fn traverseExpVisitorWrapper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> (metamodelica::Ref<DAE::Exp>, BackendVarTransform::VariableReplacements) {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (exp, repl) = (::match_deref::match_deref! { match &((inExp.clone(), inRepl.clone())) {
        (__esc_exp @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, __esc_repl) => {
            exp = (*__esc_exp).clone();
            repl = (*__esc_repl).clone();
            (exp, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&repl), None);
            (exp.clone(), repl.clone())
        },
        _ => (inExp, inRepl),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, repl)
}

// =============================================================================
// remove protected parameters stuff
//
// =============================================================================
pub(crate) fn removeProtectedParameters(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match inDAE {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            repl = BackendVarTransform::emptyReplacements();
            repl = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(protectedParametersFinder, metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)> + 'static>), repl)?;
            if Flags::isSet(Flags::DUMP_PP_REPL.clone())? {
                BackendVarTransform::dumpReplacements(&repl)?;
            }
            systs = List::map1(systs.clone(), &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: BackendVarTransform::VariableReplacements| removeProtectedParameterswork(__a0, &__a1), repl)?;
            assign_field!(shared.globalKnownVars = globalKnownVars.clone());
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

fn removeProtectedParameterswork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut repl: &BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
            let mut lsteqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut b: bool;
            let mut syst = (*syst).clone();
            lsteqns = BackendEquation::equationList(eqns.clone())?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, repl, None)?;
            if b {
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(&lsteqns)?);
                syst = BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst));
            }
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osyst)
}

fn protectedParametersFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    BackendVarTransform::VariableReplacements,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVar, outRepl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(exp), values, .. }, repl) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let true = (DAEUtil::getProtectedAttr(values.clone())) else { return Err("pattern mismatch") };
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp.clone(), None)?;
                    Ok((v.clone(), repl_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outRepl)
}

// =============================================================================
// remove equal function calls equations stuff
//
// =============================================================================
pub type IntArray = metamodelica::Array<i32>;

pub(crate) fn removeEqualRHS(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut odae: metamodelica::Ref<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae, &removeEqualFunctionCallsWork)?;
    Ok(odae)
}

fn removeEqualFunctionCallsWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    (osyst, oshared) = (::match_deref::match_deref! { match &(isyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut mT: metamodelica::Array<metamodelica::List<i32>>;
            let mut changed: metamodelica::List<i32>;
            let mut isChanged: metamodelica::Array<bool>;
            let mut degree: metamodelica::Array<i32>;
            let mut varMark: metamodelica::Array<i32>;
            let mut ranks: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Array<i32>>>;
            let mut isInitial: bool;
            let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            let mut syst = (*syst).clone();
            let mut eqns = (*eqns).clone();
            isInitial = BackendDAEUtil::isInitializationDAE(&ishared);
            funcs = BackendDAEUtil::getFunctions(&ishared);
            (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), isInitial)?;
            degree = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
            for mut v in 1..=metamodelica::arrayLength(mT.clone()) {
                metamodelica::arrayUpdate(degree.clone(), v, ((({let __elt = (*metamodelica::index_checked(&mT.borrow(), v)?).clone(); __elt})).len() as i32))?;
            }
            ranks = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1);
            varMark = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
            isChanged = arrayCreate(metamodelica::arrayLength(m.clone()), false);
            changed = metamodelica::nil();
            for mut pos in 1..=metamodelica::arrayLength(m.clone()) {
                (eqns, changed) = removeEqualFunctionCallFinder(pos, m.clone(), mT.clone(), degree.clone(), ranks.clone(), varMark.clone(), vars.clone(), eqns.clone(), changed, isChanged.clone(), isInitial);
            }
            assign_field!(
                syst.m = Some(m.clone()),
                syst.mT = Some(mT.clone()),
                syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING()
            );
            syst = BackendDAEUtil::updateAdjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, &changed, isInitial)?;
            (syst.clone(), ishared)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn removeEqualFunctionCallFinder(
    mut pos: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut degree: metamodelica::Array<i32>,
    mut ranks: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Array<i32>>>,
    mut varMark: metamodelica::Array<i32>,
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut changed: metamodelica::List<i32>,
    mut isChanged: metamodelica::Array<bool>,
    mut isInitial: bool,
) -> (
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<i32>,
) {
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = eqns;
    let mut changed: metamodelica::List<i32> = changed;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut ecr: metamodelica::Ref<DAE::Exp>;
    let mut expvars: metamodelica::List<i32>;
    let mut controleqns: metamodelica::List<i32>;
    if '__try0: {
        ::match_deref::match_deref! { match &(({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&m.borrow(), pos), '__try0)).clone(); __elt})) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendEquation::get(eqns.clone(), pos), '__try0)) {
            Deref @ BackendDAE::Equation::EQUATION { exp: __pa1, scalar: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa1);
        e2 = metamodelica::Own::own(__pa2);
        (ecr, exp) = unwrap_break_err!(functionCallEqn(e1.clone(), e2.clone(), &vars), '__try0);
        expvars = unwrap_break_err!(BackendDAEUtil::uniqueRow(unwrap_break_err!(BackendDAEUtil::adjacencyRowExp(exp.clone(), vars.clone(), metamodelica::nil(), None, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, isInitial), '__try0)), '__try0);
        ::match_deref::match_deref! { match &(expvars.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        controleqns = unwrap_break_err!(controlEqns(&expvars, pos, m.clone(), mT.clone(), degree.clone(), ranks.clone(), varMark.clone()), '__try0);
        (eqns, changed) = removeEqualFunctionCall(&controleqns, ecr.clone(), exp.clone(), eqns.clone(), changed.clone(), isChanged.clone());
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    (eqns, changed)
}

fn controlEqns(
    mut expvars: &metamodelica::List<i32>,
    mut pos: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut degree: metamodelica::Array<i32>,
    mut ranks: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Array<i32>>>,
    mut varMark: metamodelica::Array<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_degree = degree.borrow();
    let mut eqns: metamodelica::List<i32> = metamodelica::nil();
    let mut first: i32;
    let mut least: i32;
    let mut eq: i32;
    let mut nvars: i32 = 0;
    let mut rank: metamodelica::Array<i32>;
    first = intAbs((expvars).head().cloned()?);
    least = first;
    for mut v in &**expvars {
        if (*metamodelica::index_checked(&__ab_degree, intAbs(v.clone()))?).clone()
            < (*metamodelica::index_checked(&__ab_degree, least)?).clone()
        {
            least = intAbs(v.clone());
        }
        if ({
            let __elt = (*metamodelica::index_checked(&varMark.borrow(), intAbs(v.clone()))?).clone();
            __elt
        }) != pos
        {
            metamodelica::arrayUpdate(varMark.clone(), intAbs(v.clone()), pos)?;
            nvars = nvars + 1;
        }
    }
    let __range0 = &*({
        let __elt = (*metamodelica::index_checked(&mT.borrow(), least)?).clone();
        __elt
    });
    for mut i in __range0 {
        eq = intAbs(i.clone());
        if eq != pos
            && rowContainsAll(
                &({
                    let __elt = (*metamodelica::index_checked(&m.borrow(), eq)?).clone();
                    __elt
                }),
                nvars,
                pos,
                varMark.clone(),
            )?
        {
            eqns = metamodelica::cons(eq, eqns);
        }
    }
    eqns = eqns.reverse();
    if least != first && !((eqns).is_empty()) {
        if (*metamodelica::index_checked(&__ab_degree, first)?).clone() <= 64 {
            eqns = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in ({
                    let __elt = (*metamodelica::index_checked(&mT.borrow(), first)?).clone();
                    __elt
                })
                .into_iter()
                .cloned()
                {
                    if !(List::isMemberOnTrue(intAbs(i.clone()), &eqns, &fnptr!(intEq, i32, i32))?) {
                        continue;
                    }
                    let __x = intAbs(i.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        } else {
            rank = rowRanks(first, mT.clone(), metamodelica::arrayLength(m.clone()), ranks)?;
            eqns = List::sort(
                eqns,
                (std::sync::Arc::new({
                    let __pe_b2 = rank.clone();
                    move |__pe_a0, __pe_a1| rankGt(__pe_a0, __pe_a1, __pe_b2.clone())
                }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
        }
    }
    Ok(eqns)
}

fn rowContainsAll(
    mut row: &metamodelica::List<i32>,
    mut nvars: i32,
    mut pos: i32,
    mut varMark: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut all: bool;
    let mut hits: i32 = 0;
    for mut v in &**row {
        if ({
            let __elt = (*metamodelica::index_checked(&varMark.borrow(), intAbs(v.clone()))?).clone();
            __elt
        }) == pos
        {
            metamodelica::arrayUpdate(varMark.clone(), intAbs(v.clone()), -(pos))?;
            hits = hits + 1;
        }
    }
    for mut v in &**row {
        if ({
            let __elt = (*metamodelica::index_checked(&varMark.borrow(), intAbs(v.clone()))?).clone();
            __elt
        }) == -(pos)
        {
            metamodelica::arrayUpdate(varMark.clone(), intAbs(v.clone()), pos)?;
        }
    }
    all = hits == nvars;
    Ok(all)
}

fn rowRanks(
    mut var: i32,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut numEqns: i32,
    mut ranks: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Array<i32>>>,
) -> Result<metamodelica::Array<i32>> {
    let __ab_mT = mT.borrow();
    let mut rank: metamodelica::Array<i32>;
    let mut i: i32 = 1;
    rank = (match UnorderedMap::get(var, ranks.clone())? {
        Some(mut __esc_rank) => {
            rank = __esc_rank.clone();
            rank.clone()
        }
        _ => {
            rank = arrayCreate(numEqns, 0);
            for mut eq in &*(*metamodelica::index_checked(&__ab_mT, var)?).clone() {
                metamodelica::arrayUpdate(rank.clone(), intAbs(eq.clone()), i)?;
                i = i + 1;
            }
            UnorderedMap::add(var, rank.clone(), ranks)?;
            rank.clone()
        }
    });
    Ok(rank)
}

fn rankGt(mut a: i32, mut b: i32, mut rank: metamodelica::Array<i32>) -> Result<bool> {
    let __ab_rank = rank.borrow();
    let mut gt: bool =
        (*metamodelica::index_checked(&__ab_rank, a)?).clone() > (*metamodelica::index_checked(&__ab_rank, b)?).clone();
    Ok(gt)
}

fn functionCallEqn(
    mut ie1: metamodelica::Ref<DAE::Exp>,
    mut ie2: metamodelica::Ref<DAE::Exp>,
    mut inVars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut outECr: metamodelica::Ref<DAE::Exp>;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outECr, outExp) = (::match_deref::match_deref! { match &((ie1, ie2)) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
            return Err("fail")
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
            return Err("fail")
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            return Err("fail")
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { .. }, exp: e2 }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (),
                _ => return Err("pattern mismatch"),
            } };
            (metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1.clone() }), e2.clone())
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, e2) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (),
                _ => return Err("pattern mismatch"),
            } };
            (e1.clone(), e2.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { .. }, exp: e1 }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (),
                _ => return Err("pattern mismatch"),
            } };
            (metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), e1.clone())
        },
        (e1, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (),
                _ => return Err("pattern mismatch"),
            } };
            (e2.clone(), e1.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outECr, outExp))
}

fn removeEqualFunctionCall(
    mut inEqsLst: &metamodelica::List<i32>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inECr: metamodelica::Ref<DAE::Exp>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut changed: metamodelica::List<i32>,
    mut isChanged: metamodelica::Array<bool>,
) -> (
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<i32>,
) {
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = eqns;
    let mut changed: metamodelica::List<i32> = changed;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut i: i32;
    for mut pos in &**inEqsLst {
        if '__try0: {
            eqn = unwrap_break_err!(BackendEquation::get(eqns.clone(), pos.clone()), '__try0);
            let (__pa1, (_, _, __pa2)) = unwrap_break_err!(BackendDAETransform::traverseBackendDAEExpsEqnWithSymbolicOperation(&eqn, (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32))| replaceExp(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32))) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32)))> + 'static>), (inECr.clone(), inExp.clone(), 0)), '__try0);
            eqn = metamodelica::Own::own(__pa1);
            i = metamodelica::Own::own(__pa2);
            if i > 0 {
                eqns = unwrap_break_err!(BackendEquation::setAtIndex(eqns.clone(), pos.clone(), eqn.clone()), '__try0);
                if !(({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&isChanged.borrow(), pos.clone()), '__try0)).clone(); __elt})) {
                    unwrap_break_err!(metamodelica::arrayUpdate(isChanged.clone(), pos.clone(), true), '__try0);
                    changed = metamodelica::cons(pos.clone(), changed.clone());
                }
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    (eqns, changed)
}

fn replaceExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
    ),
)> {
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
    );
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut se: metamodelica::Ref<DAE::Exp>;
    let mut te: metamodelica::Ref<DAE::Exp>;
    let mut i: i32;
    let mut j: i32;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    e = inExp;
    let (__pa0, (__pa1, __pa2, __pa3)) = inTpl;
    ops = metamodelica::Own::own(__pa0);
    se = metamodelica::Own::own(__pa1);
    te = metamodelica::Own::own(__pa2);
    i = metamodelica::Own::own(__pa3);
    (e1, j) = Expression::replaceExpNoEvent(e.clone(), se.clone(), te.clone())?;
    ops = if (j > 0) {
        metamodelica::cons(
            metamodelica::Ref::new(DAE::SymbolicOperation::SUBSTITUTION {
                substitutions: list![e1.clone()],
                source: e,
            }),
            ops,
        )
    } else {
        ops
    };
    outTpl = (ops, (se, te, i + j));
    Ok((e1, outTpl))
}

// =============================================================================
// remove unused parameter
//
// =============================================================================
pub(crate) fn removeUnusedParameter(
    mut inDlow: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDlow: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDlow = (match &**inDlow {
        BackendDAE::BackendDAE { eqs, shared } => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut shared = (*shared).clone();
            globalKnownVars1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            globalKnownVars = shared.globalKnownVars.clone();
            globalKnownVars1 = BackendVariable::traverseBackendDAEVars(
                globalKnownVars.clone(),
                (std::sync::Arc::new(copyNonParamVariables)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::Var>,
                                BackendDAE::Variables,
                            )
                                -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)>
                            + 'static,
                    >),
                globalKnownVars1,
            )?;
            (_, globalKnownVars1) = List::fold1(
                eqs,
                &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: _, __a2: _| {
                    BackendDAEUtil::traverseBackendDAEExpsEqSystem(&__a0, __a1, __a2)
                },
                (std::sync::Arc::new(fnptr!(
                    checkUnusedVariables,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                (globalKnownVars.clone(), globalKnownVars1),
            )?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsVars(
                &(globalKnownVars.clone()),
                (std::sync::Arc::new(fnptr!(
                    checkUnusedParameter,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                (globalKnownVars.clone(), globalKnownVars1),
            )?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsVars(
                &shared.aliasVars,
                (std::sync::Arc::new(fnptr!(
                    checkUnusedParameter,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                (globalKnownVars.clone(), globalKnownVars1),
            )?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(
                shared.removedEqs.clone(),
                (std::sync::Arc::new(fnptr!(
                    checkUnusedParameter,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                (globalKnownVars.clone(), globalKnownVars1),
            )?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(
                shared.initialEqs.clone(),
                (std::sync::Arc::new(fnptr!(
                    checkUnusedParameter,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                (globalKnownVars, globalKnownVars1),
            )?;
            assign_field!(shared.globalKnownVars = globalKnownVars1);
            metamodelica::Ref::new(BackendDAE::BackendDAE {
                eqs: eqs.clone(),
                shared: shared.clone(),
            })
        }
    });
    Ok(outDlow)
}

fn copyNonParamVariables(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outVars: BackendDAE::Variables;
    (outVar, outVars) = (::match_deref::match_deref! { match &(inVar.clone()) {
        v @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } => {
            (v.clone(), inVars)
        },
        _ => {
            let mut vars1: BackendDAE::Variables;
            vars1 = BackendVariable::addVar(inVar.clone(), inVars)?;
            (inVar, vars1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outVars))
}

fn checkUnusedParameter(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (BackendDAE::Variables, BackendDAE::Variables),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, BackendDAE::Variables),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (vars, _)) => {
                    let mut vars1: BackendDAE::Variables;
                    let (_, (_, __pa0)) = Expression::traverseExpBottomUp(exp.clone(), &fnptr!(checkUnusedParameterExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)), inTpl.clone())?;
                    vars1 = metamodelica::Own::own(__pa0);
                    Ok((exp.clone(), (vars.clone(), vars1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn checkUnusedParameterExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (BackendDAE::Variables, BackendDAE::Variables),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, BackendDAE::Variables),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, tp) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tp = (*tp).clone();
                    expl = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| Expression::generateCrefsExpFromExpVar(&__a0, &__a1), cr.clone())?;
                    (_, tp) = Expression::traverseExpList(expl.clone(), &fnptr!(checkUnusedParameterExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tp) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut tp = (*tp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), &fnptr!(checkUnusedParameterExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, vars1)) => {
                    BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars1))?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, vars1)) => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut vars1 = (*vars1).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    var = metamodelica::Own::own(__pa0);
                    vars1 = BackendVariable::addVar(var.clone(), vars1.clone())?;
                    Ok((e.clone(), (vars.clone(), vars1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

// =============================================================================
// remove unused variables
//
// =============================================================================
pub(crate) fn removeUnusedVariables(
    mut inDlow: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDlow: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDlow = (match &**inDlow {
        BackendDAE::BackendDAE { eqs, shared } => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut tpl: (BackendDAE::Variables, BackendDAE::Variables);
            let mut shared = (*shared).clone();
            globalKnownVars1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            globalKnownVars = shared.globalKnownVars.clone();
            tpl = List::fold1(
                eqs,
                &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: _, __a2: _| {
                    BackendDAEUtil::traverseBackendDAEExpsEqSystem(&__a0, __a1, __a2)
                },
                (std::sync::Arc::new(fnptr!(
                    checkUnusedVariables,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                (globalKnownVars.clone(), globalKnownVars1),
            )?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsVars(
                &globalKnownVars,
                (std::sync::Arc::new(fnptr!(
                    checkUnusedVariables,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                tpl,
            )?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsVars(
                &shared.aliasVars,
                (std::sync::Arc::new(fnptr!(
                    checkUnusedVariables,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                tpl,
            )?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsEqns(
                shared.removedEqs.clone(),
                (std::sync::Arc::new(fnptr!(
                    checkUnusedVariables,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                tpl,
            )?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(
                shared.initialEqs.clone(),
                (std::sync::Arc::new(fnptr!(
                    checkUnusedVariables,
                    metamodelica::Ref<DAE::Exp>,
                    (BackendDAE::Variables, BackendDAE::Variables)
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (BackendDAE::Variables, BackendDAE::Variables),
                            )> + 'static,
                    >),
                tpl,
            )?;
            assign_field!(shared.globalKnownVars = globalKnownVars1);
            metamodelica::Ref::new(BackendDAE::BackendDAE {
                eqs: eqs.clone(),
                shared: shared.clone(),
            })
        }
    });
    Ok(outDlow)
}

fn checkUnusedVariables(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (BackendDAE::Variables, BackendDAE::Variables),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, BackendDAE::Variables),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp => {
                    let mut tpl: (BackendDAE::Variables, BackendDAE::Variables);
                    (_, tpl) = Expression::traverseExpBottomUp(exp.clone(), &fnptr!(checkUnusedVariablesExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)), inTpl.clone())?;
                    Ok((exp.clone(), tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn checkUnusedVariablesExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (BackendDAE::Variables, BackendDAE::Variables),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendDAE::Variables, BackendDAE::Variables),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tp) => {
                    Ok((e.clone(), tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, tp) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tp = (*tp).clone();
                    expl = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| Expression::generateCrefsExpFromExpVar(&__a0, &__a1), cr.clone())?;
                    (_, tp) = Expression::traverseExpList(expl.clone(), &fnptr!(checkUnusedVariablesExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tp) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut tp = (*tp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), &fnptr!(checkUnusedVariablesExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, vars1)) => {
                    BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars1))?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, vars1)) => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut vars1 = (*vars1).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    var = metamodelica::Own::own(__pa0);
                    vars1 = BackendVariable::addVar(var.clone(), vars1.clone())?;
                    Ok((inExp.clone(), (vars.clone(), vars1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

// =============================================================================
// remove unused functions
//
// =============================================================================
pub(crate) fn removeUnusedFunctions(
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inusedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<AvlTreePathFunction::Tree>> {
    type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<AvlTreePathFunction::Tree>,
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<AvlTreePathFunction::Tree>,
            )> + 'static,
    >;

    let mut outFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<AvlTreePathFunction::Tree>,
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<AvlTreePathFunction::Tree>,
            )> + 'static,
    >;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut usedfuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    funcs = inFunctionTree;
    usedfuncs = inusedFunctions;
    func = (std::sync::Arc::new({
        let __pe_b1 = funcs.clone();
        move |__pe_a0, __pe_a2| checkUnusedFunctions(__pe_a0, &__pe_b1, __pe_a2)
    })
        as std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::Exp>,
                    metamodelica::Ref<AvlTreePathFunction::Tree>,
                ) -> Result<(
                    metamodelica::Ref<DAE::Exp>,
                    metamodelica::Ref<AvlTreePathFunction::Tree>,
                )> + 'static,
        >);
    usedfuncs = List::fold1(
        inEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: _, __a2: _| {
            BackendDAEUtil::traverseBackendDAEExpsEqSystem(&__a0, __a1, __a2)
        },
        func.clone(),
        usedfuncs,
    )?;
    usedfuncs = List::fold1(
        inEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: _, __a2: _| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendDAEUtil::traverseBackendDAEExpsEqSystemJacobians(
                &__a0, __a1, __a2,
            ))
        },
        func.clone(),
        usedfuncs,
    )?;
    usedfuncs = List::fold1(
        inEquationLst,
        &BackendEquation::traverseExpsOfEquationList_WithoutChange,
        func.clone(),
        usedfuncs,
    )?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsVars(&inShared.globalKnownVars, func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsVars(&inShared.externalObjects, func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsVars(&inShared.aliasVars, func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsEqns(inShared.removedEqs.clone(), func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsEqns(inShared.initialEqs.clone(), func.clone(), usedfuncs)?;
    usedfuncs = removeUnusedFunctionsSymJacs(inShared, funcs, usedfuncs)?;
    outFunctionTree = usedfuncs;
    Ok(outFunctionTree)
}

pub(crate) fn copyRecordConstructorAndExternalObjConstructorDestructor(
    mut inAllFunctionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<AvlTreePathFunction::Tree>> {
    let mut outUsedFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut allfuncs_list: metamodelica::List<DAE::Function>;
    outUsedFunctionTree = openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY();
    allfuncs_list = DAEUtil::getFunctionList(inAllFunctionTree, false)?;
    for mut func in &*allfuncs_list {
        let () = (match func.clone() {
            DAE::Function::RECORD_CONSTRUCTOR { path: mut path, .. } => {
                let mut var_list: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                let mut obind: Option<metamodelica::Ref<DAE::Exp>>;
                let mut bind_exp: metamodelica::Ref<DAE::Exp>;
                outUsedFunctionTree = AvlTreePathFunction::add(
                    outUsedFunctionTree,
                    metamodelica::AsArg::as_arg(&path),
                    Some(func.clone()),
                    &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                        as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
                )?;
                match '__try0: {
                    let __pa1 = ::match_deref::match_deref! { match &(var_field!(func.type_, DAE::Function::RECORD_CONSTRUCTOR).clone()) {
                        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_COMPLEX { varLst: __pa1, .. }, .. } => __pa1.clone(),
                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                    } };
                    var_list = metamodelica::Own::own(__pa1);
                    Ok::<_, &'static str>((var_list.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        var_list = __try0_o0;
                    }
                    Err(__try0_err) => {
                        Error::addSourceMessage(
                            &(Error::INTERNAL_ERROR.clone()),
                            list![{
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!(
                                    "BackendDAEOptimize.copyRecordConstructorAndExternalObjConstructorDestructor"
                                ));
                                __mm_s.push_str(&*literal!(" got unxpected record constructor structure for  "));
                                __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
                                ArcStr::from(__mm_s)
                            }],
                            &(metamodelica::sourceInfo!("BackEnd/BackendDAEOptimize.mo")),
                        )?;
                        return Err(__try0_err);
                    }
                }
                for mut var in &*var_list {
                    obind = Types::getBindingExpOptional(metamodelica::AsArg::as_arg(&var));
                    if (obind).is_some() {
                        let __pa3 = ::match_deref::match_deref! { match &(obind) {
                            Some(__pa3) => __pa3.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        bind_exp = metamodelica::Own::own(__pa3);
                        (_, outUsedFunctionTree) =
                            checkUnusedFunctions(bind_exp, inAllFunctionTree, outUsedFunctionTree)?;
                    }
                }
                ()
            }
            DAE::Function::FUNCTION { path: mut path, .. } => {
                if stringEq(
                    &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))),
                    &(literal!("constructor")),
                ) || stringEq(
                    &(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))),
                    &(literal!("destructor")),
                ) {
                    outUsedFunctionTree = AvlTreePathFunction::add(
                        outUsedFunctionTree,
                        metamodelica::AsArg::as_arg(&path),
                        Some(func.clone()),
                        &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                            as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
                    )?;
                }
                ()
            }
        });
    }
    Ok(outUsedFunctionTree)
}

fn removeUnusedFunctionsSymJacs(
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<AvlTreePathFunction::Tree>> {
    let mut outUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree> = inUsedFunctions.clone();
    let mut bdae: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut usedfuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    for mut sjac in &*inShared.symjacs.clone() {
        let () = (::match_deref::match_deref! { match &(sjac.clone()) {
            (Some((__esc_bdae, _, _, _, _, _)), _, _, _) => {
                bdae = (*__esc_bdae).clone();
                bdae = BackendDAEUtil::setFunctionTree(metamodelica::AsArg::as_arg(&bdae), inFunctions.clone());
                shared = bdae.shared.clone();
                usedfuncs = removeUnusedFunctions(&bdae.eqs, &(shared.clone()), &(metamodelica::nil()), shared.functionTree.clone(), inUsedFunctions.clone())?;
                outUsedFunctions = AvlTreePathFunction::join(outUsedFunctions, &usedfuncs, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    let () = (::match_deref::match_deref! { match &(inShared.dataReconciliationData.clone()) {
        None => (),
        Some(BackendDAE::DataReconciliationData { symbolicJacobian: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((__esc_bdae, _, _, _, _, _)), .. }, .. }) => {
            bdae = (*__esc_bdae).clone();
            bdae = BackendDAEUtil::setFunctionTree(metamodelica::AsArg::as_arg(&bdae), inFunctions);
            shared = bdae.shared.clone();
            usedfuncs = removeUnusedFunctions(&bdae.eqs, &(shared.clone()), &(metamodelica::nil()), shared.functionTree.clone(), inUsedFunctions)?;
            outUsedFunctions = AvlTreePathFunction::join(outUsedFunctions, &usedfuncs, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?;
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.removeUnusedFunctionsSymJacs")); __mm_s.push_str(&*literal!(": Unexpected data reconciliation jacobian structure. ")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/BackendDAEOptimize.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUsedFunctions)
}

fn checkUnusedFunctions(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFunctions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>;
    (outExp, outUsedFunctions) = Expression::traverseExpBottomUp(
        inExp,
        &({
            let __pe_b1 = inFunctions.clone();
            move |__pe_a0, __pe_a2| checkUnusedFunctionsExp(__pe_a0, &__pe_b1, __pe_a2)
        }),
        inUsedFunctions,
    )?;
    Ok((outExp, outUsedFunctions))
}

fn checkUnusedFunctionsExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFunctions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>;
    outUsedFunctions = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path, .. } => {
            addUnusedFunction(path.clone(), inFunctions, inUsedFunctions)?
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path, .. } => {
            addUnusedFunction(path.clone(), inFunctions, inUsedFunctions)?
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: Deref @ DAE::Type::T_FUNCTION { path, .. }, .. }, .. } => {
            addUnusedFunction(path.clone(), inFunctions, inUsedFunctions)?
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut usedfuncs: metamodelica::Ref<AvlTreePathFunction::Tree>;
            (_, usedfuncs) = Expression::traverseExpCrefDims(metamodelica::AsArg::as_arg(&cr), &({ let __pe_b1 = inFunctions.clone(); move |__pe_a0, __pe_a2| checkUnusedFunctions(__pe_a0, &__pe_b1, __pe_a2) }), inUsedFunctions)?;
            usedfuncs
        },
        _ => {
            inUsedFunctions
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outUsedFunctions))
}

fn addUnusedFunction(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inFunctions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<AvlTreePathFunction::Tree>> {
    let mut outUsedFunctions: metamodelica::Ref<AvlTreePathFunction::Tree> = inUsedFunctions.clone();
    let mut f: Option<DAE::Function>;
    let mut body: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    if '__try0: {
        unwrap_break_err!(AvlTreePathFunction::get(&inUsedFunctions, inPath.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        (f, body) = getFunctionAndBody(inPath.clone(), inFunctions);
        if (f).is_some() {
            outUsedFunctions = AvlTreePathFunction::add(
                outUsedFunctions.clone(),
                &inPath,
                f.clone(),
                &*(std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            (_, outUsedFunctions) = DAEUtil::traverseDAEElementList(
                body.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = inFunctions.clone();
                    move |__pe_a0, __pe_a2| checkUnusedFunctions(__pe_a0, &__pe_b1, __pe_a2)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                metamodelica::Ref<AvlTreePathFunction::Tree>,
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                metamodelica::Ref<AvlTreePathFunction::Tree>,
                            )> + 'static,
                    >),
                outUsedFunctions.clone(),
            )?;
        }
    }
    Ok(outUsedFunctions)
}

fn getFunctionAndBody(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut fns: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> (
    Option<DAE::Function>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
) {
    let mut outFn: Option<DAE::Function>;
    let mut outFnBody: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut r#fn: DAE::Function;
    match '__try0: {
        let (__pa2, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(AvlTreePathFunction::get(fns, inPath.clone()), '__try0)) {
            __pa2 @ Some(__pa1) => (__pa2.clone(), __pa1.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        r#fn = metamodelica::Own::own(__pa1);
        outFn = metamodelica::Own::own(__pa2);
        outFnBody = unwrap_break_err!(DAEUtil::getFunctionElements(&r#fn), '__try0);
        Ok::<_, &'static str>((outFn.clone(), outFnBody.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outFn = __try0_o0;
            outFnBody = __try0_o1;
        }
        Err(_) => {
            outFn = None;
            outFnBody = metamodelica::nil();
        }
    }
    (outFn, outFnBody)
}

// =============================================================================
// parallel back end stuff (TLM)
//
// =============================================================================
pub(crate) fn collapseIndependentBlocks(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    vars = BackendVariable::emptyVarsSized(
        ((metamodelica::OrderedFloat(
            ({
                let mut __acc: i32 = 0;
                for mut s in (systs.clone()).into_iter().cloned() {
                    let __x = BackendVariable::varsSize(&(s.orderedVars.clone()));
                    __acc += __x;
                }
                __acc
            }) as f64,
        ) * metamodelica::OrderedFloat(1.4_f64))
        .0
        .floor() as i32),
    );
    syst = List::fold(
        &(systs.reverse()),
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: metamodelica::Ref<BackendDAE::EqSystem>| {
            mergeIndependentBlocks(&__a0, &__a1)
        },
        BackendDAEUtil::createEqSystem(
            vars,
            BackendEquation::emptyEqns(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        ),
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: list![syst],
        shared: shared,
    });
    Ok(outDAE)
}

pub(crate) fn collapseIndependentContinuousBlocks(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = collapseIndependentBlocks(
        &(metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: List::filterOnFalse(inDAE.eqs.clone(), &move |__a0: metamodelica::Ref<
                BackendDAE::EqSystem,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendDAEUtil::isClockedSyst(&__a0))
            })?,
            shared: inDAE.shared.clone(),
        })),
    )?;
    Ok(outDAE)
}

fn mergeIndependentBlocks(
    mut syst1: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut syst2: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut removedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    vars = BackendVariable::addVariables(syst1.orderedVars.clone(), syst2.orderedVars.clone())?;
    eqs = BackendEquation::addList(
        &(BackendEquation::equationList(syst1.orderedEqs.clone())?),
        syst2.orderedEqs.clone(),
    )?;
    removedEqs = BackendEquation::addList(
        &(BackendEquation::equationList(syst1.removedEqs.clone())?),
        syst2.removedEqs.clone(),
    )?;
    stateSets = listAppend(syst1.stateSets.clone(), syst2.stateSets.clone());
    syst = BackendDAEUtil::createEqSystem(
        vars,
        eqs,
        stateSets,
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        removedEqs,
    );
    Ok(syst)
}

pub(crate) fn partitionIndependentBlocks(
    mut dlow: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDlow: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match dlow {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst, tail: Deref @ metamodelica::ListNode::Nil }, shared } => {
            let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
            let mut shared = (*shared).clone();
            (systs, shared) = partitionIndependentBlocksHelper(syst.clone(), shared.clone(), Error::getNumErrorMessages(), false)?;
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs, shared: shared.clone() })
        },
        _ => {
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(collapseIndependentBlocks(dlow)?) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            syst = metamodelica::Own::own(__pa0);
            shared = metamodelica::Own::own(__pa1);
            (systs, shared) = partitionIndependentBlocksHelper(syst, shared, Error::getNumErrorMessages(), false)?;
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs, shared: shared })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDlow)
}

pub(crate) fn partitionIndependentBlocksHelper(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut numErrorMessages: i32,
    mut throwNoError: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    (systs, oshared) = 'mc: {
        let __mc_input = (isyst, ishared.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared) => {
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
                    let mut rm: metamodelica::Array<metamodelica::List<i32>>;
                    let mut rmT: metamodelica::Array<metamodelica::List<i32>>;
                    let mut eqPartMap: metamodelica::Array<i32>;
                    let mut varPartMap: metamodelica::Array<i32>;
                    let mut rixs: metamodelica::Array<i32>;
                    let mut vars: metamodelica::Array<bool>;
                    let mut rvars: metamodelica::Array<bool>;
                    let mut b: bool;
                    let mut isInitial: bool;
                    let mut i: i32;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let mut syst = (*syst).clone();
                    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = systs.clone();
                    isInitial = BackendDAEUtil::isInitializationDAE(&ishared);
                    funcs = BackendDAEUtil::getFunctions(&ishared);
                    (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial)?;
                    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(metamodelica::AsArg::as_arg(&syst), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial)?;
                    eqPartMap = arrayCreate(metamodelica::arrayLength(m.clone()), 0);
                    varPartMap = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
                    rixs = arrayCreate(metamodelica::arrayLength(rm.clone()), 0);
                    vars = arrayCreate(metamodelica::arrayLength(mT.clone()), false);
                    rvars = arrayCreate(metamodelica::arrayLength(rmT.clone()), false);
                    i = SynchronousFeatures::partitionIndependentBlocks0(m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?;
                    b = i > 1;
                    systs = if (b) {(SynchronousFeatures::partitionIndependentBlocksSplitBlocks(i, syst.clone(), eqPartMap.clone(), rixs.clone(), mT.clone(), rmT.clone(), throwNoError, funcs.clone(), isInitial)?).0} else {list![syst.clone()]};
                    GCExt::free(eqPartMap.clone());
                    GCExt::free(varPartMap.clone());
                    GCExt::free(rixs.clone());
                    Ok(((systs.clone(), shared.clone()), systs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            systs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion(!(numErrorMessages == Error::getNumErrorMessages()), literal!("BackendDAEOptimize.partitionIndependentBlocks failed without good error message"), &(Absyn::dummyInfo.clone()))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((systs, oshared))
}

// =============================================================================
// residual stuff ... for whatever reason
//
// =============================================================================
pub(crate) fn residualForm(
    mut dlow: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut odlow: metamodelica::Ref<BackendDAE::BackendDAE>;
    odlow = BackendDAEUtil::mapEqSystem1(dlow, &residualForm1, 1)?;
    Ok(odlow)
}

fn residualForm1(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut i: i32,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = syst.clone();
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let __arc1 = syst;
    let BackendDAE::EQSYSTEM { orderedEqs: __pa0, .. } = &*__arc1;
    eqs = metamodelica::Own::own(__pa0);
    BackendEquation::traverseEquationArray_WithUpdate(
        eqs,
        &fnptr!(residualForm2, metamodelica::Ref<BackendDAE::Equation>, i32),
        1,
    )?;
    Ok((osyst, oshared))
}

fn residualForm2(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut ii: i32,
) -> (metamodelica::Ref<BackendDAE::Equation>, i32) {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut oi: i32;
    (outEq, oi) = 'mc: {
        let __mc_input = (&*inEq, ii);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr }, i) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    ::match_deref::match_deref! { match &(Expression::r#typeof(e1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let false = (Expression::isZero(metamodelica::AsArg::as_arg(&e1))? || Expression::isZero(metamodelica::AsArg::as_arg(&e2))?) else { return Err("pattern mismatch") };
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() });
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    source = ElementSource::addSymbolicTransformation(source.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_RESIDUAL { e1: e1.clone(), e2: e2.clone(), e: e.clone() }))?;
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: eqAttr.clone() }), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEq.clone(), ii))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, oi)
}

// =============================================================================
// countOperations
//
// =============================================================================
pub(crate) fn countOperations(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(&inDAE, &countOperations0, false)?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn countOperations0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = inChanged;
    let mut compInfos: metamodelica::List<metamodelica::Ref<BackendDAE::CompInfo>>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    compInfos = countOperationstraverseComps(&comps, &isyst, &inShared, &(metamodelica::nil()))?;
    Ok((osyst, outShared, outChanged))
}

pub(crate) fn countOperationstraverseComps(
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut isyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: &metamodelica::Ref<BackendDAE::Shared>,
    mut compInfosIn: &metamodelica::List<metamodelica::Ref<BackendDAE::CompInfo>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::CompInfo>>> {
    let mut compInfosOut: metamodelica::List<metamodelica::Ref<BackendDAE::CompInfo>>;
    compInfosOut = 'mc: {
        let __mc_input = (&**inComps, &**ishared);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(compInfosIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst);
                    eqn = BackendEquation::get(eqns.clone(), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: (inComps).head().cloned()?, numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
                        BackendDump::dumpCompInfo(&compInfo)?;
                    }
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: (inComps).head().cloned()?, numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog + 1, numOth: numOth, funcCalls: numFuncs });
                    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
                        BackendDump::dumpCompInfo(&compInfo)?;
                    }
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqs, jac, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }, tail: rest }, _) => {
                    let mut size: i32;
                    let mut density: metamodelica::Real;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut allOps: metamodelica::Ref<BackendDAE::CompInfo>;
                    BackendDAETransform::getEquationAndSolvedVar(metamodelica::AsArg::as_arg(&comp), BackendEquation::getEqnsFromEqSystem(isyst), BackendVariable::daeVars(isyst))?;
                    size = ((eqs).len() as i32);
                    density = realDiv(intReal(getNumJacEntries(metamodelica::AsArg::as_arg(&jac))), intReal(size * size));
                    allOps = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: 0, numMul: 0, numDiv: 0, numTrig: 0, numRelations: 0, numLog: 0, numOth: 0, funcCalls: 0 });
                    allOps = countOperationsJac(metamodelica::AsArg::as_arg(&jac), ishared, allOps.clone())?;
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::SYSTEM { comp: comp.clone(), allOperations: allOps.clone(), size: size, density: density });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut size: i32;
                    let mut jacEntries: i32;
                    let mut density: metamodelica::Real;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut allOps: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (eqnlst, _, _) = BackendDAETransform::getEquationAndSolvedVar(metamodelica::AsArg::as_arg(&comp), BackendEquation::getEqnsFromEqSystem(isyst), BackendVariable::daeVars(isyst))?;
                    size = ((eqnlst).len() as i32);
                    (numAdd, numMul, numDiv, numTrig, numRel, numLog, numOth, numFuncs) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::listEquation(&eqnlst)?, (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    allOps = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    jacEntries = getNumJacEntries(metamodelica::AsArg::as_arg(&jac));
                    if intEq(jacEntries, -1) {
                        jacEntries = size * size;
                    }
                    density = realDiv(intReal(jacEntries), intReal(size * size));
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::SYSTEM { comp: comp.clone(), allOperations: allOps.clone(), size: size, density: density });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog + 1, numOth: numOth, funcCalls: numFuncs });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog + 1, numOth: numOth, funcCalls: numFuncs });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog + 1, numOth: numOth, funcCalls: numFuncs });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: tornEqs, innerEquations, .. }, linear: true, .. }, tail: rest }, Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut otherEqs: metamodelica::List<i32>;
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut torn: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut other: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut vLstLst: metamodelica::List<metamodelica::List<i32>>;
                    let mut vlst = (*vlst).clone();
                    comp = (inComps).head().cloned()?;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst);
                    vars = BackendVariable::daeVars(isyst);
                    eqnlst = BackendEquation::getList(tornEqs.clone(), eqns.clone())?;
                    varlst = List::map1(vlst.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), vars.clone())?;
                    (explst, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(&eqnlst)?, BackendVariable::listVar1(&varlst)?, Some(funcs.clone()))?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = Expression::traverseExpList(explst.clone(), &({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    torn = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    (otherEqs, vLstLst, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    vlst = List::flatten(vLstLst.clone())?;
                    eqnlst = BackendEquation::getList(otherEqs.clone(), eqns.clone())?;
                    varlst = List::map1(vlst.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), vars.clone())?;
                    (explst, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(&eqnlst)?, BackendVariable::listVar1(&varlst)?, Some(funcs.clone()))?;
                    let (_, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15)) = Expression::traverseExpList(explst.clone(), &({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa8);
                    numMul = metamodelica::Own::own(__pa9);
                    numDiv = metamodelica::Own::own(__pa10);
                    numTrig = metamodelica::Own::own(__pa11);
                    numRel = metamodelica::Own::own(__pa12);
                    numLog = metamodelica::Own::own(__pa13);
                    numOth = metamodelica::Own::own(__pa14);
                    numFuncs = metamodelica::Own::own(__pa15);
                    other = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::TORN_ANALYSE { comp: comp.clone(), tornEqs: torn.clone(), otherEqs: other.clone(), tornSize: ((tornEqs).len() as i32) });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: tornEqs, innerEquations, .. }, linear: false, .. }, tail: rest }, Deref @ BackendDAE::Shared { .. }) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut otherEqs: metamodelica::List<i32>;
                    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
                    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut compInfo: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut torn: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut other: metamodelica::Ref<BackendDAE::CompInfo>;
                    let mut eqnlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    comp = (inComps).head().cloned()?;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst);
                    eqnlst = BackendEquation::getList(tornEqs.clone(), eqns.clone())?;
                    explst = List::map(eqnlst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| BackendEquation::getEquationRHS(&__a0))?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = Expression::traverseExpList(explst.clone(), &({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa0);
                    numMul = metamodelica::Own::own(__pa1);
                    numDiv = metamodelica::Own::own(__pa2);
                    numTrig = metamodelica::Own::own(__pa3);
                    numRel = metamodelica::Own::own(__pa4);
                    numLog = metamodelica::Own::own(__pa5);
                    numOth = metamodelica::Own::own(__pa6);
                    numFuncs = metamodelica::Own::own(__pa7);
                    torn = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    (otherEqs, _, _) = List::map_3(metamodelica::AsArg::as_arg(&innerEquations), &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0)) })?;
                    eqnlst = BackendEquation::getList(otherEqs.clone(), eqns.clone())?;
                    explst = List::map(eqnlst.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| BackendEquation::getEquationRHS(&__a0))?;
                    let (_, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15)) = Expression::traverseExpList(explst.clone(), &({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = metamodelica::Own::own(__pa8);
                    numMul = metamodelica::Own::own(__pa9);
                    numDiv = metamodelica::Own::own(__pa10);
                    numTrig = metamodelica::Own::own(__pa11);
                    numRel = metamodelica::Own::own(__pa12);
                    numLog = metamodelica::Own::own(__pa13);
                    numOth = metamodelica::Own::own(__pa14);
                    numFuncs = metamodelica::Own::own(__pa15);
                    other = metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
                    compInfo = metamodelica::Ref::new(BackendDAE::CompInfo::TORN_ANALYSE { comp: comp.clone(), tornEqs: torn.clone(), otherEqs: other.clone(), tornSize: ((tornEqs).len() as i32) });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, &(metamodelica::cons(compInfo.clone(), compInfosIn.clone())))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: comp, tail: rest }, _) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("not supported component: ")); __mm_s.push_str(&*BackendDump::strongComponentString(metamodelica::AsArg::as_arg(&comp))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(countOperationstraverseComps(metamodelica::AsArg::as_arg(&rest), isyst, ishared, compInfosIn)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(compInfosOut)
}

fn getNumJacEntries(mut inJac: &metamodelica::Ref<BackendDAE::Jacobian>) -> i32 {
    let mut numEntries: i32;
    numEntries = (::match_deref::match_deref! { match inJac {
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: None } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) } => {
            ((jac).len() as i32)
        },
        Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN { .. } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, .. } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, vars1, vars2, _, _)), .. } if (((vars1).len() as i32) == ((vars2).len() as i32)) => {
            ((vars1).len() as i32)
        },
        _ => {
            metamodelica::print(literal!("another JAC\n"));
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    numEntries
}

fn countOperationsJac(
    mut inJac: &metamodelica::Ref<BackendDAE::Jacobian>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut compInfoIn: metamodelica::Ref<BackendDAE::CompInfo>,
) -> Result<metamodelica::Ref<BackendDAE::CompInfo>> {
    let mut compInfoOut: metamodelica::Ref<BackendDAE::CompInfo>;
    compInfoOut = (::match_deref::match_deref! { match &((inJac.clone(), compInfoIn.clone())) {
        (Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: None }, _) => {
            compInfoIn
        },
        (Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, Deref @ BackendDAE::CompInfo::COUNTER { comp, numAdds: numAdd, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs }) => {
            let mut numAdd = (*numAdd).clone();
            let mut numMul = (*numMul).clone();
            let mut numDiv = (*numDiv).clone();
            let mut numTrig = (*numTrig).clone();
            let mut numRel = (*numRel).clone();
            let mut numLog = (*numLog).clone();
            let mut numOth = (*numOth).clone();
            let mut numFuncs = (*numFuncs).clone();
            (numAdd, numMul, numDiv, numTrig, numRel, numLog, numOth, numFuncs) = List::fold(metamodelica::AsArg::as_arg(&jac), &({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsJac1(__pe_a0, &__pe_b1, __pe_a2) }), (numAdd.clone(), numMul.clone(), numDiv.clone(), numOth.clone(), numTrig.clone(), numRel.clone(), numLog.clone(), numFuncs.clone()))?;
            metamodelica::Ref::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() })
        },
        (_, _) => {
            compInfoIn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(compInfoOut)
}

fn countOperationsJac1(
    mut inJac: (i32, i32, metamodelica::Ref<BackendDAE::Equation>),
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32),
) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    (_, outTpl) = BackendEquation::traverseExpsOfEquation(
        Util::tuple33(inJac),
        (std::sync::Arc::new({
            let __pe_b1 = shared.clone();
            move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (i32, i32, i32, i32, i32, i32, i32, i32),
                    )
                        -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))>
                    + 'static,
            >),
        inTpl,
    )?;
    Ok(outTpl)
}

pub(crate) fn countOperationsExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32),
) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    (outExp, outTpl) = Expression::traverseExpBottomUp(
        inExp,
        &({
            let __pe_b1 = shared.clone();
            move |__pe_a0, __pe_a2| Ok(traversecountOperationsExp(__pe_a0, &__pe_b1, __pe_a2))
        }),
        inTpl,
    )?;
    Ok((outExp, outTpl))
}

fn traversecountOperationsExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inTuple: (i32, i32, i32, i32, i32, i32, i32, i32),
) -> (metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (i32, i32, i32, i32, i32, i32, i32, i32);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(metamodelica::AsArg::as_arg(&op), inTuple);
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(metamodelica::AsArg::as_arg(&op), inTuple);
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(metamodelica::AsArg::as_arg(&op), inTuple);
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(metamodelica::AsArg::as_arg(&op), inTuple);
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(metamodelica::AsArg::as_arg(&op), inTuple);
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: exp1, expElse: exp2 }, _) => {
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut i3: i32;
                    let mut i4: i32;
                    let mut i5: i32;
                    let mut i6: i32;
                    let mut i7: i32;
                    let mut i8: i32;
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared, inTuple);
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared, tpl);
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = traversecountOperationsExp(cond.clone(), shared, tpl);
                    i1 = metamodelica::Own::own(__pa0);
                    i2 = metamodelica::Own::own(__pa1);
                    i3 = metamodelica::Own::own(__pa2);
                    i4 = metamodelica::Own::own(__pa3);
                    i5 = metamodelica::Own::own(__pa4);
                    i6 = metamodelica::Own::own(__pa5);
                    i7 = metamodelica::Own::own(__pa6);
                    i8 = metamodelica::Own::own(__pa7);
                    Ok((e.clone(), (i1, i2, i3, i4, i5, i6 + 1, i7, i8)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RECORD { exps: expLst, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), &({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), inTuple)?;
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: expLst, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), &({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), inTuple)?;
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TUPLE { PR: expLst }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), &({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, &__pe_b1, __pe_a2) }), inTuple)?;
                    Ok((e.clone(), tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: opName }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    if !((stringEq(&opName, &(literal!("sin"))) || stringEq(&opName, &(literal!("cos"))) || stringEq(&opName, &(literal!("tan"))))) { return Err("guard") }
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone() + 1, i5.clone(), i6.clone(), i7.clone(), i8.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, .. }, _) => {
                    let mut func: DAE::Function;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut i3: i32;
                    let mut i4: i32;
                    let mut i5: i32;
                    let mut i6: i32;
                    let mut i7: i32;
                    let mut i8: i32;
                    let mut elemLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    func = DAEUtil::getNamedFunction(path.clone(), &(BackendDAEUtil::getFunctions(shared)))?;
                    elemLst = DAEUtil::getFunctionElements(&func)?;
                    (i1, i2, i3, i4, i5, i6, i7, i8) = countOperationsInFunction(&elemLst, shared, inTuple);
                    Ok((e.clone(), (i1, i2, i3, i4, i5, i6, i7, i8 + 1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn countOperationsInFunction(
    mut elemLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32),
) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    outTpl = 'mc: {
        let __mc_input = &**elemLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inTpl)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| Ok(traversecountOperationsExp(__pe_a0, &__pe_b1, __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTpl)?;
                    Ok(countOperationsInFunction(metamodelica::AsArg::as_arg(&rest), shared, tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: exp1, scalar: exp2, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared, inTpl);
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared, tpl);
                    Ok(countOperationsInFunction(metamodelica::AsArg::as_arg(&rest), shared, tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: exp1, rhs: exp2, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared, inTpl);
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared, tpl);
                    Ok(countOperationsInFunction(metamodelica::AsArg::as_arg(&rest), shared, tpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(countOperationsInFunction(metamodelica::AsArg::as_arg(&rest), shared, inTpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn countOperator(
    mut op: &DAE::Operator,
    mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32),
) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    outTpl = (match (op.clone(), inTpl) {
        (DAE::Operator::ADD { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone() + 1,
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::SUB { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone() + 1,
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::MUL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone() + 1,
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::DIV { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone() + 1,
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::POW { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone() + 1,
            i8.clone(),
        ),
        (DAE::Operator::UMINUS { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone() + 1,
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (
            DAE::Operator::UMINUS_ARR { ty: ref tp },
            (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8),
        ) => {
            let mut i: i32;
            i = Expression::sizeOf(metamodelica::AsArg::as_arg(&tp));
            (
                i1.clone() + i,
                i2.clone(),
                i3.clone(),
                i4.clone(),
                i5.clone(),
                i6.clone(),
                i7.clone(),
                i8.clone(),
            )
        }
        (DAE::Operator::ADD_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(metamodelica::AsArg::as_arg(&tp));
            (
                i1.clone() + i,
                i2.clone(),
                i3.clone(),
                i4.clone(),
                i5.clone(),
                i6.clone(),
                i7.clone(),
                i8.clone(),
            )
        }
        (DAE::Operator::SUB_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(metamodelica::AsArg::as_arg(&tp));
            (
                i1.clone() + i,
                i2.clone(),
                i3.clone(),
                i4.clone(),
                i5.clone(),
                i6.clone(),
                i7.clone(),
                i8.clone(),
            )
        }
        (DAE::Operator::MUL_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(metamodelica::AsArg::as_arg(&tp));
            (
                i1.clone(),
                i2.clone() + i,
                i3.clone(),
                i4.clone(),
                i5.clone(),
                i6.clone(),
                i7.clone(),
                i8.clone(),
            )
        }
        (DAE::Operator::DIV_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(metamodelica::AsArg::as_arg(&tp));
            (
                i1.clone(),
                i2.clone(),
                i3.clone() + i,
                i4.clone(),
                i5.clone(),
                i6.clone(),
                i7.clone(),
                i8.clone(),
            )
        }
        (DAE::Operator::MUL_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone() + 1,
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::ADD_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone() + 1,
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::SUB_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone() + 1,
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (
            DAE::Operator::MUL_SCALAR_PRODUCT { .. },
            (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8),
        ) => (
            i1.clone(),
            i2.clone() + 1,
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (
            DAE::Operator::MUL_MATRIX_PRODUCT { .. },
            (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8),
        ) => (
            i1.clone(),
            i2.clone() + 1,
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::DIV_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone() + 1,
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::DIV_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone() + 1,
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::POW_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone() + 1,
            i8.clone(),
        ),
        (DAE::Operator::POW_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone() + 1,
            i8.clone(),
        ),
        (DAE::Operator::POW_ARR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone() + 1,
            i8.clone(),
        ),
        (DAE::Operator::POW_ARR2 { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone(),
            i7.clone() + 1,
            i8.clone(),
        ),
        (DAE::Operator::AND { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone() + 1,
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::OR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone() + 1,
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::NOT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone() + 1,
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::LESS { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone() + 1,
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::LESSEQ { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone() + 1,
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::GREATER { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone() + 1,
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::GREATEREQ { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone() + 1,
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::EQUAL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone() + 1,
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::NEQUAL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone() + 1,
            i6.clone(),
            i7.clone(),
            i8.clone(),
        ),
        (DAE::Operator::USERDEFINED { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => (
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
            i6.clone() + 1,
            i7.clone(),
            i8.clone(),
        ),
        _ => {
            metamodelica::print(literal!("not supported operator\n"));
            inTpl
        }
    });
    outTpl
}

// =============================================================================
// simplify if equations
//
// =============================================================================
pub(crate) fn simplifyIfEquations(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut odae: metamodelica::Ref<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(
        dae,
        &fnptr!(
            simplifyIfEquationsWork,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>
        ),
    )?;
    Ok(odae)
}

fn simplifyIfEquationsWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { globalKnownVars, initialEqs, .. }) => {
                    let mut eqnslst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut initial_asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut systChanged: bool;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqnslst = BackendEquation::equationList(eqns.clone())?;
                    (eqnslst, asserts, systChanged) = List::fold31(&(eqnslst.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                    assign_field!(syst.orderedEqs = BackendEquation::listEquation(&eqnslst)?);
                    eqnslst = BackendEquation::equationList(initialEqs.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::fold31(&(eqnslst.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), systChanged)?) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqnslst = metamodelica::Own::own(__pa0);
                    initial_asserts = metamodelica::Own::own(__pa1);
                    assign_field!(shared.initialEqs = BackendEquation::listEquation(&(listAppend(initial_asserts.clone(), eqnslst.clone())))?);
                    syst = BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst));
                    syst = BackendEquation::requationsAddDAE(&asserts, syst.clone())?;
                    Ok((syst.clone(), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared)
}

fn simplifyIfEquationsFinder(
    mut inElem: metamodelica::Ref<BackendDAE::Equation>,
    mut inConstArg: BackendDAE::Variables,
    mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut b: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    bool,
)> {
    let mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = acc;
    let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = asserts;
    let mut b: bool = b;
    (acc, asserts, b) = 'mc: {
        let __mc_input = (inElem, inConstArg);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::IF_EQUATION { conditions: explst, eqnstrue: eqnslstlst, eqnsfalse: eqnslst, source, attr }, globalKnownVars) => {
                    let mut asserts1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut explst = (*explst).clone();
                    let mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = acc.clone();
                    (explst, _) = Expression::traverseExpList(explst.clone(), &fnptr!(simplifyEvaluatedParamter, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), (globalKnownVars.clone(), false))?;
                    explst = ExpressionSimplify::simplifyList(explst.clone())?;
                    (acc, asserts1) = simplifyIfEquation(explst.clone(), eqnslstlst.clone(), metamodelica::AsArg::as_arg(&eqnslst), metamodelica::nil(), metamodelica::nil(), metamodelica::AsArg::as_arg(&source), metamodelica::AsArg::as_arg(&globalKnownVars), &acc, attr.clone())?;
                    asserts1 = listAppend(asserts.clone(), asserts1.clone());
                    Ok(((acc.clone(), asserts1.clone(), true), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            acc = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, globalKnownVars) => {
                    let mut eqn = (*eqn).clone();
                    let mut b: bool = b.clone();
                    let (__pa0, (_, __pa1)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(fnptr!(simplifyIfExpevaluatedParamter, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool))> + 'static>), (globalKnownVars.clone(), b))?;
                    eqn = metamodelica::Own::own(__pa0);
                    b = metamodelica::Own::own(__pa1);
                    Ok(((metamodelica::cons(eqn.clone(), acc.clone()), asserts.clone(), b), b.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            b = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((acc, asserts, b))
}

fn simplifyIfExpevaluatedParamter(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl1: (BackendDAE::Variables, bool),
) -> (metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tpl2: (BackendDAE::Variables, bool);
    (outExp, tpl2) = 'mc: {
        let __mc_input = (inExp.clone(), &tpl1);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::IFEXP { expCond: cond, expThen, expElse }, (globalKnownVars, b)) => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut cond = (*cond).clone();
                    let (__pa0, (_, __pa1)) = Expression::traverseExpBottomUp(cond.clone(), &fnptr!(simplifyEvaluatedParamter, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)), (globalKnownVars.clone(), false))?;
                    cond = metamodelica::Own::own(__pa0);
                    b1 = metamodelica::Own::own(__pa1);
                    e2 = if (b1) {metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: expThen.clone(), expElse: expElse.clone() })} else {e1.clone()};
                    (e2, _) = ExpressionSimplify::condsimplify(b1, e2.clone())?;
                    Ok((e2.clone(), (globalKnownVars.clone(), b.clone() || b1)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl2)
}

fn simplifyEvaluatedParamter(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl1: (BackendDAE::Variables, bool),
) -> (metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tpl2: (BackendDAE::Variables, bool);
    (outExp, tpl2) = 'mc: {
        let __mc_input = (&*inExp, &tpl1);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (globalKnownVars, _)) => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    let true = (BackendVariable::hasVarEvaluateAnnotationTrue(&v) || Flags::getConfigBool(Flags::EVALUATE_FINAL_PARAMS.clone())? && BackendVariable::isFinalVar(&v) || Flags::getConfigBool(Flags::EVALUATE_PROTECTED_PARAMS.clone())? && BackendVariable::isProtectedVar(&v)) else { return Err("pattern mismatch") };
                    e = BackendVariable::varBindExpStartValue(&v)?;
                    Ok((e.clone(), (globalKnownVars.clone(), true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl2)
}

fn simplifyIfEquation<'__b>(
    mut conditions: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut elseenqs: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut conditions1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut globalKnownVars: &'__b BackendDAE::Variables,
    mut inEqns: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqAttr: BackendDAE::EquationAttributes,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, theneqns, conditions1.clone(), theneqns1.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                (eqns, asserts, _) = List::fold31(&(elseenqs.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                return Ok((listAppend(eqns, inEqns.clone()), asserts))
            },
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, _) => {
                let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut eqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut elseenqs1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                explst = conditions1.reverse();
                eqnslst = theneqns1.reverse();
                (elseenqs1, asserts, _) = List::fold31(&(elseenqs.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                elseenqs1 = listAppend(elseenqs1, asserts);
                (eqnslst, elseenqs1, asserts) = simplifyIfEquationAsserts(&explst, &eqnslst, &elseenqs1, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                eqns = simplifyIfEquation1(explst, eqnslst, elseenqs1, source.clone(), globalKnownVars, inEqns.clone(), inEqAttr);
                return Ok((eqns, asserts))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: _ }, Deref @ metamodelica::ListNode::Nil, _) => {
                let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                (eqns, asserts, _) = List::fold31(&(eqns.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                return Ok((listAppend(eqns.clone(), inEqns.clone()), asserts))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: _ }, _, _) => {
                let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut eqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut elseenqs1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                explst = conditions1.reverse();
                eqnslst = theneqns1.reverse();
                (elseenqs1, asserts, _) = List::fold31(&(eqns.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                elseenqs1 = listAppend(elseenqs1, asserts);
                (eqnslst, elseenqs1, asserts) = simplifyIfEquationAsserts(&explst, &eqnslst, &elseenqs1, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                eqns = simplifyIfEquation1(explst, eqnslst, elseenqs1, source.clone(), globalKnownVars, inEqns.clone(), inEqAttr);
                return Ok((eqns.clone(), asserts))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: _, tail: eqnslst }, _, _) => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                { (conditions, theneqns, elseenqs, conditions1, theneqns1, source, globalKnownVars, inEqns, inEqAttr) = (explst.clone(), eqnslst.clone(), elseenqs, conditions1, theneqns1, source, globalKnownVars, inEqns, inEqAttr); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnslst }, _, _) => {
                let mut asserts: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                (eqns, asserts, _) = List::fold31(&(eqns.clone().reverse()), &simplifyIfEquationsFinder, globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                eqns = listAppend(eqns.clone(), asserts);
                { (conditions, theneqns, elseenqs, conditions1, theneqns1, source, globalKnownVars, inEqns, inEqAttr) = (explst.clone(), eqnslst.clone(), elseenqs, metamodelica::cons(e.clone(), conditions1), metamodelica::cons(eqns.clone(), theneqns1), source, globalKnownVars, inEqns, inEqAttr); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifyIfEquation1(
    mut conditions: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut elseenqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut globalKnownVars: &BackendDAE::Variables,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqAttr: BackendDAE::EquationAttributes,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outEqns = 'mc: {
        let __mc_input = inEqAttr;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ht: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTable2::FuncHashCref,
                    HashTable2::FuncCrefEqual,
                    HashTable2::FuncCrefStr,
                    HashTable2::FuncExpStr,
                ),
            );
            let mut crexplst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
            countEquationsInBranches(theneqns.clone(), elseenqs.clone(), source.clone())?;
            ht = HashTable2::emptyHashTable();
            ht = simplifySolvedIfEqnsElse(&elseenqs, ht.clone())?;
            ht = simplifySolvedIfEqns(
                &(conditions.clone().reverse()),
                &(theneqns.clone().reverse()),
                ht.clone(),
            )?;
            crexplst = BaseHashTable::hashTableList(&ht)?;
            eqns = simplifySolvedIfEqns2(&crexplst, inEqns.clone(), inEqAttr)?;
            Ok(eqns.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut fbsExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut tbsExp: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            countEquationsInBranches(theneqns.clone(), elseenqs.clone(), source.clone())?;
            fbsExp = makeEquationLstToResidualExpLst(&elseenqs)?;
            tbsExp = List::map(theneqns.clone(), &move |__a0: metamodelica::List<
                metamodelica::Ref<BackendDAE::Equation>,
            >| makeEquationLstToResidualExpLst(&__a0))?;
            eqns = makeEquationsFromResiduals(&conditions, &tbsExp, &fbsExp, &source, inEqAttr)?;
            Ok(listAppend(eqns.clone(), inEqns.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::cons(
                metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION {
                    conditions: conditions.clone(),
                    eqnstrue: theneqns.clone(),
                    eqnsfalse: elseenqs.clone(),
                    source: source.clone(),
                    attr: inEqAttr,
                }),
                inEqns.clone(),
            ))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outEqns
}

fn simplifySolvedIfEqns2<'__b>(
    mut crexplst: &'__b metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqAttr: BackendDAE::EquationAttributes,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match crexplst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inEqns)
            },
            Deref @ metamodelica::ListNode::Cons { head: (cr, e), tail: rest } => {
                let mut crexp: metamodelica::Ref<DAE::Exp>;
                crexp = Expression::crefExp(cr.clone())?;
                { (crexplst, inEqns, inEqAttr) = (rest, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: crexp, scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: inEqAttr }), inEqns), inEqAttr); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifySolvedIfEqns<'__b>(
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (conditions, theneqns) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(iHt)
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: rest }) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                ht = simplifySolvedIfEqns1(metamodelica::AsArg::as_arg(&c), metamodelica::AsArg::as_arg(&eqns), iHt, HashSet::emptyHashSet())?;
                { (conditions, theneqns, iHt) = (explst, rest, ht); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifySolvedIfEqns1<'__b>(
    mut condition: &'__b metamodelica::Ref<DAE::Exp>,
    mut brancheqns: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iHs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match brancheqns {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iHt)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, .. }, tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                hs = BaseHashSet::addUnique(cr.clone(), &iHs)?;
                exp = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
                ht = BaseHashTable::add((cr.clone(), exp), iHt)?;
                { (condition, brancheqns, iHt, iHs) = (condition, rest, ht, hs); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, scalar: e, .. }, tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                let mut e = (*e).clone();
                let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                hs = BaseHashSet::addUnique(cr.clone(), &iHs)?;
                exp = BaseHashTable::get(cr.clone(), &iHt)?;
                e = Expression::negate(e.clone())?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
                ht = BaseHashTable::add((cr.clone(), exp), iHt)?;
                { (condition, brancheqns, iHt, iHs) = (condition, rest, ht, hs); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifySolvedIfEqnsElse<'__b>(
    mut elseenqs: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match elseenqs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iHt.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                ht = BaseHashTable::add((cr.clone(), e.clone()), iHt.clone())?;
                { (elseenqs, iHt) = (rest, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, scalar: e, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                let mut e = (*e).clone();
                let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                e = Expression::negate(e.clone())?;
                ht = BaseHashTable::add((cr.clone(), e.clone()), iHt.clone())?;
                { (elseenqs, iHt) = (rest, ht); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifyIfEquationAsserts<'__b>(
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut elseenqs: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut conditions1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (conditions, theneqns) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                (beqns, eqns) = simplifyIfEquationAsserts1(elseenqs.clone(), None, &conditions1, metamodelica::nil(), inEqns)?;
                return Ok((theneqns1.reverse(), beqns, eqns))
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnslst }) => {
                let mut eqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqnslst1: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut eqns = (*eqns).clone();
                (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), Some(e.clone()), &conditions1, metamodelica::nil(), inEqns)?;
                { (conditions, theneqns, elseenqs, conditions1, theneqns1, inEqns) = (explst, eqnslst, elseenqs, metamodelica::cons(e.clone(), conditions1), metamodelica::cons(beqns, theneqns1), eqns.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simplifyIfEquationAsserts1<'__b>(
    mut brancheqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut condition: Option<metamodelica::Ref<DAE::Exp>>,
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut brancheqns1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((brancheqns, condition.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((brancheqns1.reverse(), inEqns))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond, msg, level, source: source1 }, tail: Deref @ metamodelica::ListNode::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, None) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), cond.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e, msg: msg.clone(), level: level.clone(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond, msg, level, source: source1 }, tail: Deref @ metamodelica::ListNode::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, Some(e)) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                let mut e = (*e).clone();
                e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: cond.clone(), expElse: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }) });
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), e.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e.clone(), msg: msg.clone(), level: level.clone(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { msg, source: source1 }, tail: Deref @ metamodelica::ListNode::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, None) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e, statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source1.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { msg, source: source1 }, tail: Deref @ metamodelica::ListNode::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, Some(e)) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                let mut e = (*e).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), e.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source1.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, _) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqns = (*eqns).clone();
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, metamodelica::cons(eqn.clone(), brancheqns1), inEqns); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn makeIfExp(
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut else_: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    oExp = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: cond,
        expThen: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }),
        expElse: else_,
    });
    oExp
}

fn countEquationsInBranches(
    mut trueBranches: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut falseBranch: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<i32> {
    let mut nrOfEquations: i32 = 0;
    nrOfEquations = 'mc: {
        let __mc_input = &*falseBranch;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: metamodelica::List<bool>;
                    let mut nrOfEquationsBranches: metamodelica::List<i32>;
                    let mut nrOfEquations: i32 = nrOfEquations.clone();
                    nrOfEquations = BackendEquation::equationLstSize(&falseBranch)?;
                    nrOfEquationsBranches = List::map(trueBranches.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>| BackendEquation::equationLstSize(&__a0))?;
                    b = List::map1(nrOfEquationsBranches.clone(), &fnptr!(intEq, i32, i32), nrOfEquations)?;
                    let true = (List::reduce(&b, &fnptr!(boolAnd, bool, bool))?) else { return Err("pattern mismatch") };
                    Ok((nrOfEquations, nrOfEquations.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            nrOfEquations = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Error::addSourceMessage(&(Error::IF_EQUATION_MISSING_ELSE.clone()), metamodelica::nil(), &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                    let mut strs: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut eqstr: ArcStr;
                    let mut nrOfEquationsBranches: metamodelica::List<i32>;
                    let mut nrOfEquations: i32 = nrOfEquations.clone();
                    nrOfEquations = BackendEquation::equationLstSize(&falseBranch)?;
                    nrOfEquationsBranches = List::map(trueBranches.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>| BackendEquation::equationLstSize(&__a0))?;
                    eqstr = stringDelimitList(List::map(listAppend(trueBranches.clone(), list![falseBranch.clone()]), &BackendDump::dumpEqnsStr)?, literal!("\n"));
                    strs = List::map(nrOfEquationsBranches.clone(), &fnptr!(intString, i32))?;
                    r#str = stringDelimitList(strs.clone(), literal!(","));
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(nrOfEquations)); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::IF_EQUATION_UNBALANCED_2.clone()), list![r#str.clone(), eqstr.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok((return Err("fail"), nrOfEquations.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            nrOfEquations = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(nrOfEquations)
}

fn makeEquationLstToResidualExpLst(
    mut eqLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut oExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    oExpLst = 'mc: {
        let __mc_input = &**eqLst;
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
                Deref @ metamodelica::ListNode::Cons { head: eq @ Deref @ BackendDAE::Equation::ALGORITHM { source, .. }, tail: rest } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut r#str: ArcStr;
                    r#str = BackendDump::equationString(metamodelica::AsArg::as_arg(&eq))?;
                    r#str = Util::stringReplaceChar(r#str.clone(), literal!("\n"), literal!(""))?;
                    Error::addSourceMessage(&(Error::IF_EQUATION_WARNING.clone()), list![r#str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    exps = makeEquationLstToResidualExpLst(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eq, tail: rest } => {
                    let mut exps1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exps2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exps1 = makeEquationToResidualExpLst(eq.clone())?;
                    exps2 = makeEquationLstToResidualExpLst(metamodelica::AsArg::as_arg(&rest))?;
                    exps = listAppend(exps1.clone(), exps2.clone());
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oExpLst)
}

fn makeEquationToResidualExpLst(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut oExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    oExpLst = 'mc: {
        let __mc_input = eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { conditions: conds, eqnstrue: tbs, eqnsfalse: fbs, .. } => {
                    let mut fbsExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tbsExp: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    fbsExp = makeEquationLstToResidualExpLst(metamodelica::AsArg::as_arg(&fbs))?;
                    tbsExp = List::map(tbs.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>| makeEquationLstToResidualExpLst(&__a0))?;
                    exps = makeResidualIfExpLst(conds.clone(), tbsExp.clone(), &fbsExp)?;
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                elt => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = makeEquationToResidualExp(metamodelica::AsArg::as_arg(&elt))?;
                    Ok(list![exp.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oExpLst)
}

fn makeResidualIfExpLst(
    mut inExp1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExpLst2: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inExpLst3: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = (::match_deref::match_deref! { match inExpLst3 {
        Deref @ metamodelica::ListNode::Nil => {
            let mut tbs = inExpLst2;
            let true = (List::all(&tbs, &fnptr!(listEmpty, _))?) else { return Err("pattern mismatch") };
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: fb, tail: fbs } => {
            let mut conds = inExp1;
            let mut tbs = inExpLst2;
            let mut tbsRest: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut tbsFirst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut rest_res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ifexp: metamodelica::Ref<DAE::Exp>;
            tbsRest = List::map(tbs.clone(), &listRest)?;
            rest_res = makeResidualIfExpLst(conds.clone(), tbsRest, fbs)?;
            tbsFirst = List::map(tbs, &listHead)?;
            ifexp = Expression::makeNestedIf(&conds, &tbsFirst, metamodelica::AsArg::as_arg(&fb))?;
            metamodelica::cons(ifexp, rest_res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpLst)
}

pub(crate) fn makeEquationToResidualExp(
    mut eq: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    oExp = 'mc: {
        let __mc_input = &**eq;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut oExp: metamodelica::Ref<DAE::Exp> = oExp.clone();
                    ty = Expression::r#typeof(e1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ty.clone())) else { return Err("pattern mismatch") };
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
                    let mut oExp: metamodelica::Ref<DAE::Exp> = oExp.clone();
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1, exp: e2, .. } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut oExp: metamodelica::Ref<DAE::Exp> = oExp.clone();
                    e1 = Expression::crefExp(cr1.clone())?;
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: oExp, .. } => {
                    Ok(oExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::TUPLE { PR: expl }, right: e2, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut idx: i32;
                    let mut idxs: metamodelica::List<i32>;
                    let mut oExp: metamodelica::Ref<DAE::Exp> = oExp.clone();
                    expl1 = metamodelica::nil();
                    idxs = metamodelica::nil();
                    idx = 1;
                    for mut elem in &*expl.clone() {
                        if Expression::isNotWild(metamodelica::AsArg::as_arg(&elem)) {
                            idxs = metamodelica::cons(idx, idxs.clone());
                            expl1 = metamodelica::cons(elem.clone(), expl1.clone());
                        }
                        idx = idx + 1;
                    }
                    let __pa0 = ::match_deref::match_deref! { match &(expl1.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(idxs.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    idx = metamodelica::Own::own(__pa2);
                    oExp = Expression::expSub(e.clone(), metamodelica::Ref::new(DAE::Exp::TSUB { exp: e2.clone(), ix: idx, ty: Expression::r#typeof(e.clone())? }))?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oExp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
                    let mut oExp: metamodelica::Ref<DAE::Exp> = oExp.clone();
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEOptimize.makeEquationToResidualExp failed to transform equation: ")); __mm_s.push_str(&*BackendDump::equationString(eq)?); __mm_s.push_str(&*literal!(" to residual form!")); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oExp)
}

fn makeEquationsFromResiduals(
    mut inExp1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExpLst2: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inExpLst3: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttr: BackendDAE::EquationAttributes,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outExpLst = (::match_deref::match_deref! { match inExpLst3 {
        Deref @ metamodelica::ListNode::Nil => {
            let true = (List::all(inExpLst2, &fnptr!(listEmpty, _))?) else { return Err("pattern mismatch") };
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: fb, tail: fbs } => {
            let mut tbsRest: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut tbsFirst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ifexp: metamodelica::Ref<DAE::Exp>;
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            let mut rest_res: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut zeroExp: metamodelica::Ref<DAE::Exp>;
            let mut size: i32;
            size = Expression::sizeOf(&(Expression::r#typeof(fb.clone())?));
            tbsRest = List::map(inExpLst2.clone(), &listRest)?;
            rest_res = makeEquationsFromResiduals(inExp1, &tbsRest, fbs, inSource, inEqAttr)?;
            tbsFirst = List::map(inExpLst2.clone(), &listHead)?;
            ifexp = Expression::makeNestedIf(inExp1, &tbsFirst, metamodelica::AsArg::as_arg(&fb))?;
            if size == 1 {
                eq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: ifexp, source: inSource.clone(), attr: inEqAttr });
            } else {
                zeroExp = Expression::createZeroExpression(Expression::r#typeof(fb.clone())?)?;
                eq = metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: zeroExp, right: ifexp, source: inSource.clone(), attr: inEqAttr });
            }
            metamodelica::cons(eq, rest_res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpLst)
}

// =============================================================================
// simplify semiLinear calls
//
// =============================================================================
pub(crate) fn simplifysemiLinear(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut odae: metamodelica::Ref<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(
        dae,
        &fnptr!(
            simplifysemiLinearWork,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>
        ),
    )?;
    Ok(odae)
}

fn simplifysemiLinearWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
                    let mut eqnslst: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
                    let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>, i32, bool)| -> metamodelica::Result<_> { ::std::result::Result::Ok(simplifysemiLinearFinder(__a0, &__a1)) }, (metamodelica::nil(), 0, false))?) {
                        (__pa0, (__pa1, _, true)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqns = metamodelica::Own::own(__pa0);
                    eqnslst = metamodelica::Own::own(__pa1);
                    eqnsarray = semiLinearSort(&eqnslst, &(HashTableExpToIndex::emptyHashTable()), 1, arrayCreate(5, metamodelica::nil()))?;
                    eqnsarray = semiLinearSort1(&(eqnsarray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), 1, arrayCreate(5, metamodelica::nil()))?;
                    eqnslst = Array::fold(eqnsarray.clone(), &fnptr!(semiLinearOptimize, metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>, metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>), metamodelica::nil())?;
                    assign_field!(syst.orderedEqs = List::fold(&eqnslst, &move |__a0: (metamodelica::Ref<BackendDAE::Equation>, i32), __a1: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>| semiLinearReplaceEqns(&__a0, __a1), eqns.clone())?);
                    Ok((BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), ishared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared)
}

fn semiLinearReplaceEqns(
    mut iTpl: &(metamodelica::Ref<BackendDAE::Equation>, i32),
    mut iEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>> {
    let mut oEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut index: i32;
    (eqn, index) = iTpl.clone();
    if Flags::isSet(Flags::SEMILINEAR.clone())? {
        BackendDump::debugStrEqnStr(&(literal!("Replace with ")), &eqn, &(literal!("\n")))?;
    }
    oEqns = BackendEquation::setAtIndex(iEqns, index + 1, eqn)?;
    Ok(oEqns)
}

fn semiLinearOptimize(
    mut eqnslst: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut iAcc: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
) -> metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)> {
    let mut oAcc: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
    oAcc = 'mc: {
        let __mc_input = &*eqnslst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(iAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    ht = HashTableExpToIndex::emptyHashTable();
                    ht1 = HashTableExpToIndex::emptyHashTable();
                    (ht, ht1) = semiLinearOptimize1(&eqnslst, 1, ht.clone(), ht1.clone())?;
                    explst = List::fold1(&(BaseHashTable::hashTableKeyList(&ht)?), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), __a2: metamodelica::List<metamodelica::Ref<DAE::Exp>>| semiLinearGetSA(__a0, &__a1, __a2), ht1.clone(), metamodelica::nil())?;
                    eqnsarray = metamodelica::arrayFromVec(eqnslst.clone().into_iter().cloned().collect());
                    Ok(semiLinearOptimize2(&explst, &ht, eqnsarray.clone(), &iAcc))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oAcc
}

fn semiLinearOptimize2(
    mut saLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iHt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut IEqnsarray: metamodelica::Array<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut iAcc: &metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
) -> metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)> {
    let mut oAcc: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
    oAcc = 'mc: {
        let __mc_input = &**saLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: sa, tail: rest } => {
                    let mut sb: metamodelica::Ref<DAE::Exp>;
                    let mut s1: metamodelica::Ref<DAE::Exp>;
                    let mut y: metamodelica::Ref<DAE::Exp>;
                    let mut x: metamodelica::Ref<DAE::Exp>;
                    let mut explst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, metamodelica::Ref<DAE::ElementSource>)>;
                    let mut acc: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut eqn1: metamodelica::Ref<BackendDAE::Equation>;
                    let mut i1: i32;
                    let mut index: i32;
                    let mut index1: i32;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut attr: metamodelica::Ref<DAE::CallAttributes>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut source1: metamodelica::Ref<DAE::ElementSource>;
                    let mut eqAttr: BackendDAE::EquationAttributes;
                    i1 = BaseHashTable::get(sa.clone(), iHt)?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&IEqnsarray.borrow(), i1)?).clone(); __elt})) {
                        (Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: Deref @ DAE::Exp::CALL { path: __pa1, expLst: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } } }, attr: __pa4 }, source: __pa5, attr: __pa6 }, __pa7) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    y = metamodelica::Own::own(__pa0);
                    path = metamodelica::Own::own(__pa1);
                    x = metamodelica::Own::own(__pa2);
                    s1 = metamodelica::Own::own(__pa3);
                    attr = metamodelica::Own::own(__pa4);
                    source = metamodelica::Own::own(__pa5);
                    eqAttr = metamodelica::Own::own(__pa6);
                    index = metamodelica::Own::own(__pa7);
                    (sb, source1, index1, explst) = semiLinearOptimize3(&s1, &source, index, iHt, IEqnsarray.clone(), &(metamodelica::nil()));
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: s1.clone(), scalar: metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noEvent") }), expLst: list![metamodelica::Ref::new(DAE::Exp::RELATION { exp1: x.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: sa.clone(), expElse: sb.clone() }), source: source.clone(), attr: eqAttr });
                    eqn1 = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source1.clone(), attr: eqAttr });
                    acc = semiLinearOptimize4(explst.clone(), metamodelica::cons((eqn1.clone(), index1), iAcc.clone()), eqAttr)?;
                    Ok(semiLinearOptimize2(metamodelica::AsArg::as_arg(&rest), iHt, IEqnsarray.clone(), &(metamodelica::cons((eqn.clone(), index), acc.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(semiLinearOptimize2(metamodelica::AsArg::as_arg(&rest), iHt, IEqnsarray.clone(), iAcc))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oAcc
}

fn semiLinearOptimize4(
    mut explst: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, metamodelica::Ref<DAE::ElementSource>)>,
    mut iAcc: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut inEqAttr: BackendDAE::EquationAttributes,
) -> Result<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(explst) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iAcc)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(iAcc)
            },
            Deref @ metamodelica::ListNode::Cons { head: (s2, index, source), tail: rest @ Deref @ metamodelica::ListNode::Cons { head: (s1, _, _), tail: _ } } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: s2.clone(), scalar: s1.clone(), source: source.clone(), attr: inEqAttr });
                { (explst, iAcc, inEqAttr) = (rest.clone(), metamodelica::cons((eqn, index.clone()), iAcc), inEqAttr); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn semiLinearOptimize3(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut isource: &metamodelica::Ref<DAE::ElementSource>,
    mut iIndex: i32,
    mut iHt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut IEqnsarray: metamodelica::Array<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut iAcc: &metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, metamodelica::Ref<DAE::ElementSource>)>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::ElementSource>,
    i32,
    metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, metamodelica::Ref<DAE::ElementSource>)>,
) {
    let mut slast: metamodelica::Ref<DAE::Exp>;
    let mut osource: metamodelica::Ref<DAE::ElementSource>;
    let mut oIndex: i32;
    let mut oAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, metamodelica::Ref<DAE::ElementSource>)> =
        metamodelica::nil();
    (slast, osource, oIndex, oAcc) = 'mc: {
        let __mc_input = &**iAcc;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sb: metamodelica::Ref<DAE::Exp>;
                    let mut i: i32;
                    let mut index: i32;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut oAcc: metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32, metamodelica::Ref<DAE::ElementSource>)> = oAcc.clone();
                    i = BaseHashTable::get(exp.clone(), iHt)?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&IEqnsarray.borrow(), i)?).clone(); __elt})) {
                        (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, source: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    sb = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    index = metamodelica::Own::own(__pa2);
                    (sb, source, index, oAcc) = semiLinearOptimize3(&sb, &(source.clone()), index, iHt, IEqnsarray.clone(), &(metamodelica::cons((exp.clone(), iIndex, source.clone()), iAcc.clone())));
                    Ok(((sb.clone(), source.clone(), index, oAcc.clone()), oAcc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oAcc = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), isource.clone(), iIndex, iAcc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (slast, osource, oIndex, oAcc)
}

fn semiLinearGetSA(
    mut key: metamodelica::Ref<DAE::Exp>,
    mut iHt1: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut oAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    oAcc = if (BaseHashTable::hasKey(key.clone(), iHt1)?) {
        iAcc
    } else {
        metamodelica::cons(key, iAcc)
    };
    Ok(oAcc)
}

fn semiLinearOptimize1<'__b>(
    mut eqnslst: &'__b metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut i: i32,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iHt1: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match eqnslst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iHt, iHt1))
            },
            Deref @ metamodelica::ListNode::Cons { head: (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }, _), tail: rest } => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                ht = BaseHashTable::add((sa.clone(), i), iHt)?;
                ht1 = BaseHashTable::add((sb.clone(), i), iHt1)?;
                { (eqnslst, i, iHt, iHt1) = (rest, i + 1, ht, ht1); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn semiLinearSort(
    mut eqnslst: &metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut iHt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut size: i32,
    mut iEqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>,
) -> Result<metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>> {
    let mut oEqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
    oEqnsarray = 'mc: {
        let __mc_input = &**eqnslst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(iEqnsarray.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: y, .. }, index), tail: rest } => {
                    let mut i: i32;
                    let mut eqns: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
                    let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                    i = BaseHashTable::get(y.clone(), iHt)?;
                    eqns = ({let __elt = (*metamodelica::index_checked(&iEqnsarray.borrow(), i)?).clone(); __elt});
                    eqnsarray = metamodelica::arrayUpdate(iEqnsarray.clone(), i, metamodelica::cons((eqn.clone(), index.clone()), eqns.clone()))?;
                    Ok(semiLinearSort(metamodelica::AsArg::as_arg(&rest), iHt, size, eqnsarray.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: y, .. }, index), tail: rest } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                    ht = BaseHashTable::add((y.clone(), size), iHt.clone())?;
                    eqnsarray = if (intGt(size, metamodelica::arrayLength(iEqnsarray.clone()))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                    eqnsarray = metamodelica::arrayUpdate(eqnsarray.clone(), size, list![(eqn.clone(), index.clone())])?;
                    Ok(semiLinearSort(metamodelica::AsArg::as_arg(&rest), &ht, size + 1, eqnsarray.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oEqnsarray)
}

fn semiLinearSort1<'__b>(
    mut eqnslstlst: &'__b metamodelica::List<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>,
    mut size: i32,
    mut iEqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>,
) -> Result<metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match eqnslstlst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iEqnsarray.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: tpl, tail: Deref @ metamodelica::ListNode::Nil }, tail: rest } => {
                let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                eqnsarray = if (intGt(size, metamodelica::arrayLength(iEqnsarray.clone()))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                eqnsarray = metamodelica::arrayUpdate(eqnsarray.clone(), size, list![tpl.clone()])?;
                { (eqnslstlst, size, iEqnsarray) = (rest, size + 1, eqnsarray.clone()); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: eqns, tail: rest } => {
                let mut size1: i32;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                ht = HashTableExpToIndex::emptyHashTable();
                (size1, eqnsarray) = semiLinearSort2(metamodelica::AsArg::as_arg(&eqns), &ht, size, iEqnsarray.clone())?;
                { (eqnslstlst, size, iEqnsarray) = (rest, size1, eqnsarray.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn semiLinearSort2(
    mut eqnslst: &metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
    mut iHt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut size: i32,
    mut iEqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>,
) -> Result<(
    i32,
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>,
)> {
    let mut osize: i32;
    let mut oEqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
    (osize, oEqnsarray) = 'mc: {
        let __mc_input = &**eqnslst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((size, iEqnsarray.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: x, tail: _ }, .. }, .. }, index), tail: rest } => {
                    let mut i: i32;
                    let mut eqns: metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>;
                    let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                    i = BaseHashTable::get(x.clone(), iHt)?;
                    eqns = ({let __elt = (*metamodelica::index_checked(&iEqnsarray.borrow(), i)?).clone(); __elt});
                    eqnsarray = metamodelica::arrayUpdate(iEqnsarray.clone(), i, metamodelica::cons((eqn.clone(), index.clone()), eqns.clone()))?;
                    (i, eqnsarray) = semiLinearSort2(metamodelica::AsArg::as_arg(&rest), iHt, size, eqnsarray.clone())?;
                    Ok((i, eqnsarray.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: x, tail: _ }, .. }, .. }, index), tail: rest } => {
                    let mut i: i32;
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>>;
                    ht = BaseHashTable::add((x.clone(), size), iHt.clone())?;
                    eqnsarray = if (intGt(size, metamodelica::arrayLength(iEqnsarray.clone()))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                    eqnsarray = metamodelica::arrayUpdate(eqnsarray.clone(), size, list![(eqn.clone(), index.clone())])?;
                    (i, eqnsarray) = semiLinearSort2(metamodelica::AsArg::as_arg(&rest), &ht, size + 1, eqnsarray.clone())?;
                    Ok((i, eqnsarray.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((osize, oEqnsarray))
}

fn simplifysemiLinearFinder(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: &(
        metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
        i32,
        bool,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    (
        metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
        i32,
        bool,
    ),
) {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (
        metamodelica::List<(metamodelica::Ref<BackendDAE::Equation>, i32)>,
        i32,
        bool,
    );
    (outEq, outTpl) = 'mc: {
        let __mc_input = (inEq, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&y))?) else { return Err("pattern mismatch") };
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&x))?) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&y))?) else { return Err("pattern mismatch") };
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&x))?) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&y))?) else { return Err("pattern mismatch") };
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&x))?) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: x, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&y))?) else { return Err("pattern mismatch") };
                    let true = (Expression::isZero(metamodelica::AsArg::as_arg(&x))?) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: y, .. }, scalar: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }, scalar: Deref @ DAE::Exp::UNARY { exp: y, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: sb, tail: Deref @ metamodelica::ListNode::Cons { head: sa, tail: Deref @ metamodelica::ListNode::Nil } } }, attr }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, (eqnslst, index, _)) => {
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), metamodelica::AsArg::as_arg(&eqn), &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, (eqnslst, index, _)) => {
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), metamodelica::AsArg::as_arg(&eqn), &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::UNARY { exp: x @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: x.clone(), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: x @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: x.clone(), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr(&(literal!("Found semiLinear ")), &eqn, &(literal!("\n")))?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, (eqnslst, index, b)) => {
                    Ok((eqn.clone(), (eqnslst.clone(), index.clone() + 1, b.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, outTpl)
}

// =============================================================================
// remove constants stuff
//
// =============================================================================
pub(crate) fn removeConstants(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match inDAE {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut lsteqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut b: bool;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            repl = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(removeConstantsFinder, metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements) -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendVarTransform::VariableReplacements)> + 'static>), repl)?;
            let (__pa0, (__pa1, _)) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(replaceFinalVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, i32)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, i32))> + 'static>), (repl, 0))?;
            globalKnownVars = metamodelica::Own::own(__pa0);
            repl = metamodelica::Own::own(__pa1);
            if Flags::isSet(Flags::DUMP_CONST_REPL.clone())? {
                BackendVarTransform::dumpReplacements(&repl)?;
            }
            lsteqns = BackendEquation::equationList(shared.initialEqs.clone())?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, &repl, None)?;
            assign_field!(shared.initialEqs = if (b) {BackendEquation::listEquation(&lsteqns)?} else {shared.initialEqs.clone()});
            lsteqns = BackendEquation::equationList(shared.removedEqs.clone())?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, &repl, None)?;
            assign_field!(shared.removedEqs = if (b) {BackendEquation::listEquation(&lsteqns)?} else {shared.removedEqs.clone()});
            systs = List::map1(systs.clone(), &removeConstantsWork, repl)?;
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

fn removeConstantsWork(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    outEqSystem = (::match_deref::match_deref! { match &(inEqSystem) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, .. } => {
            let mut lsteqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut b: bool;
            let mut syst = (*syst).clone();
            BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(replaceFinalVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, i32)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendVarTransform::VariableReplacements, i32))> + 'static>), (repl.clone(), 0))?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(BackendEquation::equationList(syst.orderedEqs.clone())?, &repl, None)?;
            if b {
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(&lsteqns)?);
                syst = BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst));
            }
            (lsteqns, b) = BackendVarTransform::replaceEquations(BackendEquation::equationList(syst.removedEqs.clone())?, &repl, None)?;
            if b {
                assign_field!(syst.removedEqs = BackendEquation::listEquation(&lsteqns)?);
            }
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqSystem)
}

fn removeConstantsFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    BackendVarTransform::VariableReplacements,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVar, outRepl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName, varKind: BackendDAE::VarKind::CONST { .. }, bindExp: Some(exp), .. }, repl) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp.clone(), None)?;
                    Ok((v.clone(), repl_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outRepl)
}

// =============================================================================
// reaplace edge and change with (b and not pre(b)) and (v <> pre(v))
//
// =============================================================================
pub(crate) fn replaceEdgeChange(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &fnptr!(
            replaceEdgeChange0,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            bool
        ),
        false,
    )?;
    outDAE = replaceEdgeChangeShared(&outDAE)?;
    Ok(outDAE)
}

fn replaceEdgeChange0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outChanged: bool;
    (osyst, outChanged) = 'mc: {
        let __mc_input = &*isyst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, removedEqs, .. } => {
                    BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), (std::sync::Arc::new(traverserreplaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false)?;
                    BackendDAEUtil::traverseBackendDAEExpsEqns(removedEqs.clone(), (std::sync::Arc::new(traverserreplaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false)?;
                    Ok((isyst.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inChanged))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, outShared, outChanged)
}

fn traverserreplaceEdgeChange(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut b: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut oe: metamodelica::Ref<DAE::Exp>;
    let mut ob: bool;
    (oe, ob) = Expression::traverseExpBottomUp(
        e,
        &fnptr!(traverserExpreplaceEdgeChange, metamodelica::Ref<DAE::Exp>, bool),
        b,
    )?;
    Ok((oe, ob))
}

fn traverserExpreplaceEdgeChange(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inB: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outB: bool;
    (outExp, outB) = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Expression::r#typeof(e.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::NEQUAL { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("pre") }), expLst: list![e.clone()], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), index: -1, optionExpisASUB: None }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Expression::r#typeof(e.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e.clone(), operator: DAE::Operator::AND { ty: ty.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("pre") }), expLst: list![e.clone()], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }) }) }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inB))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outB)
}

fn replaceEdgeChangeShared(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match inDAE {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { removedEqs: remeqns, .. } } => {
            BackendDAEUtil::traverseBackendDAEExpsEqns(remeqns.clone(), (std::sync::Arc::new(traverserreplaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false)?;
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

// =============================================================================
// section for preOptModule >>removeLocalKnownVars<<
//
// =============================================================================
pub(crate) fn removeLocalKnownVars(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &removeLocalKnownVars2)?;
    Ok(outDAE)
}

pub(crate) fn removeLocalKnownVars2(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut potentialLocalKnownVar: metamodelica::Ref<BackendDAE::Var>;
    let mut potentialGlobalKnownEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut orderedVars: BackendDAE::Variables = syst.orderedVars.clone();
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        syst.orderedEqs.clone();
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut crefExp: metamodelica::Ref<DAE::Exp>;
    let mut binding: metamodelica::Ref<DAE::Exp>;
    let mut localKnownVars: metamodelica::List<i32> = metamodelica::nil();
    let mut localKnownEqns: metamodelica::List<i32> = metamodelica::nil();
    let mut eindex: i32 = 0;
    let mut vindex: i32;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(
        syst.clone(),
        openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
        None,
        BackendDAEUtil::isInitializationDAE(&shared),
    )?;
    m = Array::map(
        m.clone(),
        &fnptr!(Tearing::deleteNegativeEntries, metamodelica::List<i32>),
    )?;
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        eindex = eindex + 1;
        if ((row).len() as i32) == 1 {
            let __pa1 = ::match_deref::match_deref! { match &(row) {
                Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            vindex = metamodelica::Own::own(__pa1);
            potentialLocalKnownVar = BackendVariable::getVarAt(&orderedVars, vindex)?;
            potentialGlobalKnownEquation = BackendEquation::get(orderedEqs.clone(), eindex)?;
            if '__try3: {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(potentialGlobalKnownEquation.clone()) {
                    Deref @ BackendDAE::Equation::EQUATION { exp: __pa4, scalar: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
                    _ => break '__try3 Err::<_, _>("pattern mismatch"),
                } };
                lhs = metamodelica::Own::own(__pa4);
                rhs = metamodelica::Own::own(__pa5);
                crefExp = unwrap_break_err!(BackendVariable::varExp(&potentialLocalKnownVar), '__try3);
                (binding, _) = unwrap_break_err!(ExpressionSolve::solve(lhs.clone(), rhs.clone(), crefExp.clone(), None), '__try3);
                potentialLocalKnownVar = BackendVariable::setBindExp(potentialLocalKnownVar.clone(), Some(binding.clone()));
                localKnownVars = metamodelica::cons(vindex, localKnownVars.clone());
                localKnownEqns = metamodelica::cons(eindex, localKnownEqns.clone());
                assign_field!(shared.localKnownVars = unwrap_break_err!(BackendVariable::addVar(potentialLocalKnownVar.clone(), shared.localKnownVars.clone()), '__try3));
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
    }
    localKnownVars = List::sort(
        localKnownVars,
        (std::sync::Arc::new(fnptr!(intLt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    localKnownEqns = localKnownEqns.reverse();
    for mut var in &*localKnownVars {
        (orderedVars, _) = BackendVariable::removeVar(var.clone(), orderedVars)?;
    }
    for mut eqn in &*localKnownEqns {
        orderedEqs = BackendEquation::delete(eqn.clone(), orderedEqs)?;
    }
    assign_field!(
        syst.m = None,
        syst.mT = None,
        syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(),
        syst.orderedVars = BackendVariable::listVar(BackendVariable::varList(&orderedVars)?)?,
        syst.orderedEqs = orderedEqs
    );
    Ok((syst, shared))
}

// =============================================================================
// section for postOptModule >>addInitialStmtsToAlgorithms<<
//
//   Real a[3];
// algorithm       -->  algorithm
//   a[1] := 1.0;         a[1] := $START.a[1];
//                        a[2] := $START.a[2];
//                        a[3] := $START.a[3];
//                        a[1] := 1.0;
// =============================================================================
pub(crate) fn addInitialStmtsToAlgorithms(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut isInitialSystem: bool,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem1(inDAE, &addInitialStmtsToAlgorithms1, isInitialSystem)?;
    Ok(outDAE)
}

fn addInitialStmtsToAlgorithms1(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut isInitialSystem: bool,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut ordvars: BackendDAE::Variables;
    let mut ordeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let __arc2 = osyst.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        ..
    } = &*__arc2;
    ordvars = metamodelica::Own::own(__pa0);
    ordeqns = metamodelica::Own::own(__pa1);
    BackendEquation::traverseEquationArray_WithUpdate(
        ordeqns,
        &eaddInitialStmtsToAlgorithms1Helper,
        (ordvars, isInitialSystem),
    )?;
    Ok((osyst, oshared))
}

fn eaddInitialStmtsToAlgorithms1Helper(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTpl: (BackendDAE::Variables, bool),
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, (BackendDAE::Variables, bool))> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (BackendDAE::Variables, bool) = inTpl.clone();
    outEq = (::match_deref::match_deref! { match &((inEq.clone(), inTpl)) {
        (Deref @ BackendDAE::Equation::ALGORITHM { size, alg: alg @ Deref @ DAE::Algorithm { statementLst: statements }, source, expand: crExpand, attr }, (vars, isInitialEquations)) => {
            let mut outputs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut statements = (*statements).clone();
            crlst = CheckModel::checkAndGetAlgorithmOutputs(metamodelica::AsArg::as_arg(&alg), metamodelica::AsArg::as_arg(&source), crExpand.clone())?;
            outputs = List::map(crlst, &Expression::crefExp)?;
            statements = expandAlgorithmStmts(statements.clone(), outputs, metamodelica::AsArg::as_arg(&vars), isInitialEquations.clone())?;
            metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: statements.clone() }), source: source.clone(), expand: crExpand.clone(), attr: attr.clone() })
        },
        _ => {
            inEq
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

pub(crate) fn expandAlgorithmStmts<'__b>(
    mut inAlg: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inOutputs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inVars: &'__b BackendDAE::Variables,
    mut isInitialEquation: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAlg, inOutputs)) {
            (statements, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(statements.clone())
            },
            (statements, Deref @ metamodelica::ListNode::Cons { head: out, tail: rest }) => {
                let mut initExp: metamodelica::Ref<DAE::Exp>;
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                let mut stmt: metamodelica::Ref<DAE::Statement>;
                let mut type_: metamodelica::Ref<DAE::Type>;
                let mut statements = (*statements).clone();
                cref = Expression::expCref(metamodelica::AsArg::as_arg(&out))?;
                (vars, _) = BackendVariable::getVar(cref, inVars)?;
                for mut v in &*vars {
                    type_ = v.varType.clone();
                    if BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&v)) && !(isInitialEquation) {
                        initExp = Expression::makePureBuiltinCall(literal!("pre"), list![Expression::crefExp(v.varName.clone())?], type_.clone());
                    } else {
                        initExp = Expression::crefExp(ComponentReference::crefPrefixStart(v.varName.clone()))?;
                    }
                    stmt = Algorithm::makeAssignment(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: v.varName.clone(), ty: type_.clone() }), DAE::Properties::PROP { type_: type_.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, initExp, DAE::Properties::PROP { type_: type_, constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, &(DAE::dummyAttrVar().clone()), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, DAE::emptyElementSource().clone())?;
                    statements = metamodelica::cons(stmt, statements.clone());
                }
                { (inAlg, inOutputs, inVars, isInitialEquation) = (statements.clone(), rest.clone(), inVars, isInitialEquation); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

// =============================================================================
// section for expandDerOperator
//
// =============================================================================
pub(crate) fn expandDerOperator(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &expandDerOperatorWork)?;
    Ok(outDAE)
}

fn expandDerOperatorWork(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    (syst, shared) = (::match_deref::match_deref! { match &((syst.clone(), shared.clone())) {
        (__esc_syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, removedEqs: remeqns, .. }, Deref @ BackendDAE::Shared { initialEqs: inieqns, .. }) => {
            syst = (*__esc_syst).clone();
            let mut shared_arr: Mutable::Mutable<metamodelica::Ref<BackendDAE::Shared>>;
            let mut vars = (*vars).clone();
            let mut remeqns = (*remeqns).clone();
            shared_arr = Mutable::create(shared);
            (_, vars) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), &({ let __pe_b2 = shared_arr.clone(); move |__pe_a0, __pe_a1| traverserexpandDerEquation(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
            (_, vars) = BackendEquation::traverseEquationArray_WithUpdate(inieqns.clone(), &({ let __pe_b2 = shared_arr.clone(); move |__pe_a0, __pe_a1| traverserexpandDerEquation(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
            (remeqns, vars) = BackendEquation::traverseEquationArray_WithUpdate(remeqns.clone(), &({ let __pe_b2 = shared_arr.clone(); move |__pe_a0, __pe_a1| traverserexpandDerEquation(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
            assign_field!(
                syst.removedEqs = remeqns.clone(),
                syst.orderedVars = vars.clone()
            );
            (syst.clone(), Mutable::access(shared_arr))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((syst, shared))
}

fn traverserexpandDerEquation(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut vars: BackendDAE::Variables,
    mut shared: Mutable::Mutable<metamodelica::Ref<BackendDAE::Shared>>,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut eq: metamodelica::Ref<BackendDAE::Equation> = eq;
    let mut vars: BackendDAE::Variables = vars;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let (__pa0, (__pa1, __pa2)) = BackendEquation::traverseExpsOfEquation(
        eq,
        (std::sync::Arc::new({
            let __pe_b2 = shared;
            move |__pe_a0, __pe_a1| traverserexpandDerExp(__pe_a0, __pe_a1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            BackendDAE::Variables,
                            metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            BackendDAE::Variables,
                            metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                        ),
                    )> + 'static,
            >),
        (vars, metamodelica::nil()),
    )?;
    eq = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    ops = metamodelica::Own::own(__pa2);
    eq = List::foldr(&ops, &BackendEquation::addOperation, eq)?;
    Ok((eq, vars))
}

fn traverserexpandDerExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
    ),
    mut shared: Mutable::Mutable<metamodelica::Ref<BackendDAE::Shared>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
    ),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut tpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
    ) = tpl;
    let mut exp_1: metamodelica::Ref<DAE::Exp>;
    let mut vars1: BackendDAE::Variables;
    let mut vars2: BackendDAE::Variables;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    (vars1, ops) = tpl.clone();
    (exp_1, vars2) = Expression::traverseExpBottomUp(
        exp.clone(),
        &({
            let __pe_b2 = shared;
            move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone())
        }),
        vars1.clone(),
    )?;
    if !({
        let __refeq_sl = &(vars1);
        let __refeq_sr = &(vars2.clone());
        referenceEq(&*(__refeq_sl.crefIndices), &*(__refeq_sr.crefIndices))
            && referenceEq(&*(__refeq_sl.prefixIndices), &*(__refeq_sr.prefixIndices))
            && {
                let __refeq_sl = &(__refeq_sl.varArr);
                let __refeq_sr = &(__refeq_sr.varArr);
                ((__refeq_sl.numberOfElements) == (__refeq_sr.numberOfElements))
                    && referenceEq(&*(__refeq_sl.varOptArr), &*(__refeq_sr.varOptArr))
            }
            && ((__refeq_sl.bucketSize) == (__refeq_sr.bucketSize))
            && ((__refeq_sl.numberOfVars) == (__refeq_sr.numberOfVars))
            && ((__refeq_sl.hasStartVars) == (__refeq_sr.hasStartVars))
    } && referenceEq(&*(&*exp), &*(&*exp_1)))
    {
        ops = metamodelica::cons(
            metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE {
                cr: DAE::crefTime().clone(),
                before: exp,
                after: exp_1.clone(),
            }),
            ops,
        );
        exp = exp_1;
        tpl = (vars2, ops);
    }
    Ok((exp, tpl))
}

fn expandDerExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut vars: BackendDAE::Variables,
    mut inShared: Mutable::Mutable<metamodelica::Ref<BackendDAE::Shared>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut vars: BackendDAE::Variables = vars;
    let mut failed: bool = false;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut r#str: ArcStr;
            r#str = ComponentReference::crefStr(metamodelica::AsArg::as_arg(&cr))?;
            r#str = stringAppendList(list![literal!("The model includes derivatives of order > 1 for: "), r#str.clone(), literal!(". That is not supported. Adding 'Real d"), r#str.clone(), literal!(" = der("), r#str, literal!(");' *might* result in a solvable model")]);
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, vars) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2 = metamodelica::Own::own(__pa0);
                    (exp, vars) = Expression::traverseExpBottomUp(e2.clone(), &({ let __pe_b2 = inShared.clone(); move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
                    Ok(((exp.clone(), vars.clone()), exp.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            vars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2 = metamodelica::Own::own(__pa0);
                    (exp, vars) = Expression::traverseExpBottomUp(e2.clone(), &({ let __pe_b2 = inShared.clone(); move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }), vars.clone())?;
                    Ok(((exp.clone(), vars.clone()), exp.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            vars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut e1 = (*e1).clone();
                    let mut failed: bool = failed.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), &vars)?;
                    if '__try0: {
                        (vars, e1) = unwrap_break_err!(updateStatesVar(vars.clone(), v.clone(), e1.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_err() {
                        failed = true;
                    }
                    Ok(((e1.clone(), vars.clone()), failed.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            failed = __wb0;
            vars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut vars: BackendDAE::Variables = vars.clone();
                    (varlst, _) = BackendVariable::getVar(cr.clone(), &vars)?;
                    vars = updateStatesVars(&vars, &varlst, false)?;
                    Ok(((e1.clone(), vars.clone()), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vars = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                    let mut vars: BackendDAE::Variables = vars.clone();
                    (e2, shared) = Differentiate::differentiateExpTime(e1.clone(), vars.clone(), Mutable::access(inShared.clone()))?;
                    let false = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Mutable::update(inShared.clone(), shared.clone());
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (_, vars) = Expression::traverseExpBottomUp(e2.clone(), &fnptr!(derCrefsExp, metamodelica::Ref<DAE::Exp>, BackendDAE::Variables), vars.clone())?;
                    Ok(((e2.clone(), vars.clone()), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            vars = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    if failed {
        return Err("fail");
    }
    Ok((exp, vars))
}

fn derCrefsExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVars: BackendDAE::Variables,
) -> (metamodelica::Ref<DAE::Exp>, BackendDAE::Variables) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outVars: BackendDAE::Variables;
    (outExp, outVars) = 'mc: {
        let __mc_input = (inExp.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut e = (*e).clone();
                    let mut vars = (*vars).clone();
                    (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?;
                    (vars, e) = updateStatesVar(vars.clone(), v.clone(), e.clone())?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, vars) => {
                    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut vars = (*vars).clone();
                    (varlst, _) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?;
                    vars = updateStatesVars(metamodelica::AsArg::as_arg(&vars), &varlst, false)?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outVars)
}

fn updateStatesVar(
    mut inVars: BackendDAE::Variables,
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut iExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(BackendDAE::Variables, metamodelica::Ref<DAE::Exp>)> {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut oExp: metamodelica::Ref<DAE::Exp> = iExp.clone();
    let mut var1: metamodelica::Ref<BackendDAE::Var>;
    let mut arg: metamodelica::Ref<DAE::Exp>;
    if BackendVariable::isVarNonDifferentiable(&var) {
        let __pa0 = ::match_deref::match_deref! { match &(iExp) {
            Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        Error::addSourceMessageAndFail(
            &(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone()),
            list![ExpressionBasics::printExpStr(arg)?],
            &var.source.info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else if BackendVariable::isVarDiscrete(&var) {
        oExp = metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        });
    } else if !(BackendVariable::isStateVar(&var)) || BackendVariable::varStateSelectForced(&var) {
        var1 = BackendVariable::setVarKind(
            var,
            BackendDAE::VarKind::STATE {
                index: 1,
                derName: None,
                natural: true,
            },
        )?;
        outVars = BackendVariable::addVar(var1, inVars)?;
        oExp = iExp;
    }
    Ok((outVars, oExp))
}

fn updateStatesVars(
    mut inVars: &BackendDAE::Variables,
    mut inNewStates: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut noStateFound: bool,
) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = 'mc: {
        let __mc_input = (&**inNewStates, noStateFound);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, true) => {
                    Ok(inVars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: var, tail: newStates }, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut var = (*var).clone();
                    let false = (BackendVariable::isVarDiscrete(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&var))) else { return Err("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true })?;
                    vars = BackendVariable::addVar(var.clone(), inVars.clone())?;
                    vars = updateStatesVars(&vars, metamodelica::AsArg::as_arg(&newStates), true)?;
                    Ok(vars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: newStates }, _) => {
                    let mut vars: BackendDAE::Variables;
                    vars = updateStatesVars(inVars, metamodelica::AsArg::as_arg(&newStates), noStateFound)?;
                    Ok(vars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVars)
}

// =============================================================================
// section for addedScaledVars
//
// =============================================================================
pub(crate) fn addedScaledVars_states(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut osystlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut lst_states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut tmpv: metamodelica::Ref<BackendDAE::Var>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut norm: metamodelica::Ref<DAE::Exp>;
    let mut y_norm: metamodelica::Ref<DAE::Exp>;
    let mut y: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systlst = metamodelica::Own::own(__pa0);
    oshared = metamodelica::Own::own(__pa1);
    for mut syst in &*systlst {
        let mut syst = syst.clone();
        syst = (::match_deref::match_deref! { match &(syst) {
            syst1 @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
                vars = (*__esc_vars).clone();
                eqns = (*__esc_eqns).clone();
                let mut syst1 = (*syst1).clone();
                lst_states = List::select(BackendVariable::varList(metamodelica::AsArg::as_arg(&vars))?, (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendVariable::isStateVar(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>))?;
                for mut v in &*lst_states {
                    cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                    tmpv = BackendVariable::createVar(&cref, &(literal!("__OMC$scaled_state")))?;
                    y = Expression::crefExp(cref)?;
                    norm = BackendVariable::getVarNominalValue(metamodelica::AsArg::as_arg(&v));
                    y_norm = Expression::expDiv(y, norm)?;
                    (y_norm, _) = ExpressionSimplify::simplify(y_norm)?;
                    cref = BackendVariable::varCref(&tmpv);
                    lhs = Expression::crefExp(cref)?;
                    eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs, scalar: y_norm, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    eqns = BackendEquation::add(eqn, eqns.clone())?;
                    vars = BackendVariable::addVar(tmpv, vars.clone())?;
                }
                assign_field!(
                    syst1.orderedVars = vars.clone(),
                    syst1.orderedEqs = eqns.clone()
                );
                BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst1))
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        osystlst = metamodelica::cons(syst, osystlst);
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: osystlst,
        shared: oshared,
    });
    Ok(outDAE)
}

pub(crate) fn addedScaledVars_inputs(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut osystlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut kvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut lst_inputs: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut tmpv: metamodelica::Ref<BackendDAE::Var>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut norm: metamodelica::Ref<DAE::Exp>;
    let mut y_norm: metamodelica::Ref<DAE::Exp>;
    let mut y: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systlst = metamodelica::Own::own(__pa0);
    oshared = metamodelica::Own::own(__pa1);
    kvarlst = BackendVariable::varList(&oshared.globalKnownVars)?;
    lst_inputs = List::select(
        kvarlst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndInputNoDerInput(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(systlst) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    syst = metamodelica::Own::own(__pa3);
    osystlst = metamodelica::Own::own(__pa4);
    syst = (::match_deref::match_deref! { match &(syst) {
        syst1 @ Deref @ BackendDAE::EqSystem { orderedEqs: __esc_eqns, orderedVars: __esc_vars, .. } => {
            eqns = (*__esc_eqns).clone();
            vars = (*__esc_vars).clone();
            let mut syst1 = (*syst1).clone();
            for mut v in &*lst_inputs {
                cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                tmpv = BackendVariable::createVar(&cref, &(literal!("__OMC$scaled_input")))?;
                y = Expression::crefExp(cref)?;
                norm = BackendVariable::getVarNominalValue(metamodelica::AsArg::as_arg(&v));
                y_norm = Expression::expDiv(y, norm)?;
                (y_norm, _) = ExpressionSimplify::simplify(y_norm)?;
                cref = BackendVariable::varCref(&tmpv);
                lhs = Expression::crefExp(cref)?;
                eqn = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs, scalar: y_norm, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                eqns = BackendEquation::add(eqn, eqns.clone())?;
                vars = BackendVariable::addVar(tmpv, vars.clone())?;
            }
            assign_field!(
                syst1.orderedEqs = eqns.clone(),
                syst1.orderedVars = vars.clone()
            );
            BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst1))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    osystlst = metamodelica::cons(syst, osystlst);
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: osystlst,
        shared: oshared,
    });
    Ok(outDAE)
}

// =============================================================================
// section for sortEqnsVars
//
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn sortEqnsVars(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut new_systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut ne: i32;
    let mut nv: i32;
    let mut w_vars: metamodelica::Array<i32>;
    let mut w_eqns: metamodelica::Array<i32>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut tplIndexWeight: metamodelica::List<(i32, i32)>;
    let mut indexs: metamodelica::List<i32>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let __arc2 = inDAE;
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &*__arc2;
    systlst = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    let __arc4 = shared.clone();
    let BackendDAE::SHARED {
        functionTree: __pa3, ..
    } = &*__arc4;
    functionTree = metamodelica::Own::own(__pa3);
    for mut syst in &*systlst {
        let mut syst = syst.clone();
        syst = (::match_deref::match_deref! { match &(syst.clone()) {
            syst1 @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
                vars = (*__esc_vars).clone();
                eqns = (*__esc_eqns).clone();
                let mut syst1 = (*syst1).clone();
                (_, m, mT) = BackendDAEUtil::getAdjacencyMatrix(syst, openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), BackendDAEUtil::isInitializationDAE(&shared))?;
                if Flags::isSet(Flags::SORT_EQNS_AND_VARS.clone())? {
                    BackendDump::dumpAdjacencyMatrix(m.clone())?;
                    BackendDump::dumpAdjacencyMatrixT(mT.clone())?;
                }
                let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { numberOfElements: __pa0, .. }, .. } = &vars;
                nv = metamodelica::Own::own(__pa0);
                ne = ExpandableArray::getNumberOfElements(eqns.clone());
                w_vars = arrayCreate(nv, -1);
                w_eqns = arrayCreate(ne, -1);
                sortEqnsVarsWeights(w_vars.clone(), nv, mT.clone())?;
                sortEqnsVarsWeights(w_eqns.clone(), ne, m.clone())?;
                tplIndexWeight = ({
            let mut __acc: metamodelica::List<(i32, i32)> = metamodelica::nil();
            for mut i in (1..=nv).into_iter() {
                let __x = (i.clone(), ({let __elt = (*metamodelica::index_checked(&w_vars.borrow(), i.clone())?).clone(); __elt}));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                tplIndexWeight = List::sort(tplIndexWeight, std::sync::Arc::new(fnptr!(Util::compareTuple2IntLt, _, _)))?;
                indexs = sortEqnsVarsWorkTpl(tplIndexWeight);
                var_lst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut i in (indexs).into_iter().cloned() {
                let __x = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), i.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                vars = BackendVariable::listVar1(&var_lst)?;
                tplIndexWeight = ({
            let mut __acc: metamodelica::List<(i32, i32)> = metamodelica::nil();
            for mut i in (1..=ne).into_iter() {
                let __x = (i.clone(), ({let __elt = (*metamodelica::index_checked(&w_eqns.borrow(), i.clone())?).clone(); __elt}));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                tplIndexWeight = List::sort(tplIndexWeight, std::sync::Arc::new(fnptr!(Util::compareTuple2IntGt, _, _)))?;
                indexs = sortEqnsVarsWorkTpl(tplIndexWeight);
                eqn_lst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
            for mut i in (indexs).into_iter().cloned() {
                let __x = BackendEquation::get(eqns.clone(), i.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                eqns = BackendEquation::listEquation(&eqn_lst)?;
                assign_field!(
                    syst1.orderedEqs = eqns.clone(),
                    syst1.orderedVars = vars.clone()
                );
                if Flags::isSet(Flags::SORT_EQNS_AND_VARS.clone())? {
                    (_, m, mT) = BackendDAEUtil::getAdjacencyMatrix(syst1.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), BackendDAEUtil::isInitializationDAE(&shared))?;
                    BackendDump::dumpAdjacencyMatrix(m.clone())?;
                    BackendDump::dumpAdjacencyMatrixT(mT.clone())?;
                }
                GCExt::free(w_vars.clone());
                GCExt::free(w_eqns.clone());
                BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst1))
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        new_systlst = metamodelica::cons(syst, new_systlst);
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: new_systlst,
        shared: shared,
    });
    Ok(outDAE)
}

fn sortEqnsVarsWorkTpl(mut tplIndexWeight: metamodelica::List<(i32, i32)>) -> metamodelica::List<i32> {
    let mut outIndexs: metamodelica::List<i32>;
    outIndexs = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut elem in (tplIndexWeight).into_iter().cloned() {
            let __x = Util::tuple21(elem.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outIndexs
}

fn sortEqnsVarsWeights(
    mut inW: metamodelica::Array<i32>,
    mut n: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<i32>> {
    let __ab_m = m.borrow();
    let mut outW: metamodelica::Array<i32> = inW;
    let mut i: i32 = 0;
    for mut i in 1..=n {
        {
            let __cell0 = ((*metamodelica::index_checked(&__ab_m, i)?).len() as i32);
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut outW.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    Ok(outW)
}

// =============================================================================
// fix some bugs for complex function
//
// e.g. (a,-b) = f(.) -> (a,c) = f(.) with c = -b
//      (a,b) = (c,d) -> a=c and b = d
//      {a,b} = {c,d} -> a=c and b = d
//      (a,b) = f(a) fixed iterration var
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn simplifyComplexFunction(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = simplifyComplexFunction1(inDAE, true)?;
    Ok(outDAE)
}

pub(crate) fn simplifyComplexFunction1(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut withTmpVars: bool,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut n: i32;
    let mut size: i32;
    let mut idx: i32 = 1;
    let mut m: i32;
    let mut j: i32;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation> =
        openmodelica_backend_types::BackendDAE::Equation::interned_DUMMY_EQUATION();
    let mut eqn1: metamodelica::Ref<BackendDAE::Equation>;
    let mut left: metamodelica::Ref<DAE::Exp>;
    let mut right: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e2: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e3: metamodelica::Ref<DAE::Exp>;
    let mut left_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut right_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut indRemove: metamodelica::List<i32>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut attr: BackendDAE::EquationAttributes;
    let mut update: bool;
    let mut sc: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut arrayLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut arrayLst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut cattr: metamodelica::Ref<DAE::CallAttributes>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut tmpvar: metamodelica::Ref<BackendDAE::Var>;
    let mut tmpVarPrefix: ArcStr;
    shared = inDAE.shared.clone();
    tmpVarPrefix = (match &*shared {
        BackendDAE::Shared {
            backendDAEType: BackendDAE::BackendDAEType::SIMULATION { .. },
            ..
        } => literal!("$OMC$CF$sim"),
        BackendDAE::Shared {
            backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM { .. },
            ..
        } => literal!("$OMC$CF$init"),
        _ => literal!("$OMC$CF$unknown"),
    });
    for mut syst in &*inDAE.eqs.clone() {
        let __arc2 = syst.clone();
        let BackendDAE::EQSYSTEM {
            orderedVars: __pa0,
            orderedEqs: __pa1,
            ..
        } = &*__arc2;
        vars = metamodelica::Own::own(__pa0);
        eqns = metamodelica::Own::own(__pa1);
        n = ExpandableArray::getNumberOfElements(eqns.clone());
        update = false;
        indRemove = metamodelica::nil();
        '__loop3: for mut i in 1..=n {
            if let Ok(__iflet4) = BackendEquation::get(eqns.clone(), i) {
                eqn = __iflet4;
            } else {
                continue '__loop3;
            }
            if BackendEquation::isComplexEquation(&eqn) || BackendEquation::isArrayEquation(&eqn) {
                if BackendEquation::isComplexEquation(&eqn) {
                    let (__pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size: __pa5, left: __pa6, right: __pa7, attr: __pa8, source: __pa9 } => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    size = metamodelica::Own::own(__pa5);
                    left = metamodelica::Own::own(__pa6);
                    right = metamodelica::Own::own(__pa7);
                    attr = metamodelica::Own::own(__pa8);
                    source = metamodelica::Own::own(__pa9);
                } else {
                    let (__pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: __pa10, right: __pa11, attr: __pa12, source: __pa13, .. } => (__pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    left = metamodelica::Own::own(__pa10);
                    right = metamodelica::Own::own(__pa11);
                    attr = metamodelica::Own::own(__pa12);
                    source = metamodelica::Own::own(__pa13);
                }
                if Expression::isTuple(&left) && Expression::isTuple(&right) {
                    let __pa14 = ::match_deref::match_deref! { match &(left) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa14 } => __pa14.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    left_lst = metamodelica::Own::own(__pa14);
                    let __pa15 = ::match_deref::match_deref! { match &(right) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa15 } => __pa15.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    right_lst = metamodelica::Own::own(__pa15);
                    update = true;
                    indRemove = metamodelica::cons(i, indRemove);
                    for mut e1 in &*left_lst {
                        let mut e1 = e1.clone();
                        let (__pa16, __pa17) = ::match_deref::match_deref! { match &(right_lst) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa16, tail: __pa17 } => (__pa16.clone(), __pa17.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        e2 = metamodelica::Own::own(__pa16);
                        right_lst = metamodelica::Own::own(__pa17);
                        if !(Expression::isWild(&e1)) {
                            if Expression::isScalar(&e2)? {
                                eqn1 = BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr)?;
                                eqns = BackendEquation::add(eqn1.clone(), eqns)?;
                            } else {
                                expLst = simplifyComplexFunction2(e1.clone());
                                arrayLst = simplifyComplexFunction2(e2.clone());
                                for mut e_asub in &*arrayLst {
                                    let (__pa18, __pa19) = ::match_deref::match_deref! { match &(expLst) {
                                        Deref @ metamodelica::ListNode::Cons { head: __pa18, tail: __pa19 } => (__pa18.clone(), __pa19.clone()),
                                        _ => return Err("pattern mismatch"),
                                    } };
                                    e3 = metamodelica::Own::own(__pa18);
                                    expLst = metamodelica::Own::own(__pa19);
                                    eqn1 = BackendEquation::generateEquation(
                                        e_asub.clone(),
                                        e3.clone(),
                                        source.clone(),
                                        attr,
                                    )?;
                                    eqns = BackendEquation::add(eqn1.clone(), eqns)?;
                                }
                            }
                        }
                    }
                } else if Expression::isArray(&left) && Expression::isArray(&right) {
                    match '__try20: {
                        left_lst = unwrap_break_err!(Expression::getArrayOrRangeContents(left.clone()), '__try20);
                        right_lst = unwrap_break_err!(Expression::getArrayOrRangeContents(right.clone()), '__try20);
                        update = true;
                        indRemove = metamodelica::cons(i, indRemove.clone());
                        for mut e1 in &*left_lst {
                            let mut e1 = e1.clone();
                            let (__pa21, __pa22) = ::match_deref::match_deref! { match &(right_lst.clone()) {
                                Deref @ metamodelica::ListNode::Cons { head: __pa21, tail: __pa22 } => (__pa21.clone(), __pa22.clone()),
                                _ => break '__try20 Err::<_, _>("pattern mismatch"),
                            } };
                            e2 = metamodelica::Own::own(__pa21);
                            right_lst = metamodelica::Own::own(__pa22);
                            if !(Expression::isWild(&e1)) {
                                if unwrap_break_err!(Expression::isScalar(&e2), '__try20) {
                                    eqn1 = unwrap_break_err!(BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr), '__try20);
                                    eqns =
                                        unwrap_break_err!(BackendEquation::add(eqn1.clone(), eqns.clone()), '__try20);
                                } else {
                                    expLst = simplifyComplexFunction2(e1.clone());
                                    arrayLst = simplifyComplexFunction2(e2.clone());
                                    for mut e_asub in &*arrayLst {
                                        let (__pa23, __pa24) = ::match_deref::match_deref! { match &(expLst.clone()) {
                                            Deref @ metamodelica::ListNode::Cons { head: __pa23, tail: __pa24 } => (__pa23.clone(), __pa24.clone()),
                                            _ => break '__try20 Err::<_, _>("pattern mismatch"),
                                        } };
                                        e3 = metamodelica::Own::own(__pa23);
                                        expLst = metamodelica::Own::own(__pa24);
                                        eqn1 = unwrap_break_err!(BackendEquation::generateEquation(e_asub.clone(), e3.clone(), source.clone(), attr), '__try20);
                                        eqns = unwrap_break_err!(BackendEquation::add(eqn1.clone(), eqns.clone()), '__try20);
                                    }
                                }
                            }
                        }
                        Ok::<_, &'static str>((indRemove.clone(), left_lst.clone(), right_lst.clone(), update.clone()))
                    } {
                        Ok((__try20_o0, __try20_o1, __try20_o2, __try20_o3)) => {
                            indRemove = __try20_o0;
                            left_lst = __try20_o1;
                            right_lst = __try20_o2;
                            update = __try20_o3;
                        }
                        Err(_) => {
                            continue '__loop3;
                        }
                    }
                } else if withTmpVars && Expression::isTuple(&left) && Expression::isCall(&right) {
                    let __pa25 = ::match_deref::match_deref! { match &(left) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa25 } => __pa25.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    left_lst = metamodelica::Own::own(__pa25);
                    let (__pa26, __pa27, __pa28) = ::match_deref::match_deref! { match &(right.clone()) {
                        Deref @ DAE::Exp::CALL { path: __pa26, expLst: __pa27, attr: __pa28 } => (__pa26.clone(), __pa27.clone(), __pa28.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa26);
                    expLst = metamodelica::Own::own(__pa27);
                    cattr = metamodelica::Own::own(__pa28);
                    expLst = metamodelica::nil();
                    for mut e1 in &*left_lst {
                        let mut e1 = e1.clone();
                        if Expression::isCref(&e1) {
                            let __pa29 = ::match_deref::match_deref! { match &(e1.clone()) {
                                Deref @ DAE::Exp::CREF { componentRef: __pa29, .. } => __pa29.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            cr = metamodelica::Own::own(__pa29);
                            if Expression::expHasCrefNoPreOrStart(right.clone(), cr)? {
                                update = true;
                                cr = ComponentReferenceBasics::makeCrefIdent(
                                    {
                                        let mut __mm_s = String::new();
                                        __mm_s.push_str(&*tmpVarPrefix);
                                        __mm_s.push_str(&*intString(idx));
                                        ArcStr::from(__mm_s)
                                    },
                                    Expression::r#typeof(e1.clone())?,
                                    metamodelica::nil(),
                                );
                                idx = idx + 1;
                                e = Expression::crefExp(cr.clone())?;
                                tmpvar = BackendVariable::makeVar(cr)?;
                                tmpvar = BackendVariable::setVarTS(
                                    tmpvar,
                                    Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID),
                                );
                                vars = BackendVariable::addVar(tmpvar, vars)?;
                                eqn1 = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                                    exp: e.clone(),
                                    scalar: e1.clone(),
                                    source: DAE::emptyElementSource().clone(),
                                    attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                                });
                                eqns = BackendEquation::add(eqn1.clone(), eqns)?;
                            } else {
                                e = e1.clone();
                            }
                        } else if Expression::isUnaryCref(&e1) {
                            update = true;
                            cr = ComponentReferenceBasics::makeCrefIdent(
                                {
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*tmpVarPrefix);
                                    __mm_s.push_str(&*intString(idx));
                                    ArcStr::from(__mm_s)
                                },
                                Expression::r#typeof(e1.clone())?,
                                metamodelica::nil(),
                            );
                            idx = idx + 1;
                            e = Expression::crefExp(cr.clone())?;
                            tmpvar = BackendVariable::makeVar(cr)?;
                            tmpvar = BackendVariable::setVarTS(
                                tmpvar,
                                Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID),
                            );
                            vars = BackendVariable::addVar(tmpvar, vars)?;
                            eqn1 = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                                exp: e.clone(),
                                scalar: e1.clone(),
                                source: DAE::emptyElementSource().clone(),
                                attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                            });
                            eqns = BackendEquation::add(eqn1.clone(), eqns)?;
                        } else if Expression::isArray(&e1) {
                            update = true;
                            let (__pa30, __pa31) = ::match_deref::match_deref! { match &(e1.clone()) {
                                Deref @ DAE::Exp::ARRAY { array: __pa30, scalar: __pa31, .. } => (__pa30.clone(), __pa31.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            arrayLst = metamodelica::Own::own(__pa30);
                            sc = metamodelica::Own::own(__pa31);
                            m = ((arrayLst).len() as i32);
                            cr = ComponentReferenceBasics::makeCrefIdent(
                                {
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*tmpVarPrefix);
                                    __mm_s.push_str(&*intString(idx));
                                    ArcStr::from(__mm_s)
                                },
                                Expression::r#typeof(e1.clone())?,
                                metamodelica::nil(),
                            );
                            idx = idx + 1;
                            e = Expression::crefExp(cr.clone())?;
                            tmpvar = BackendVariable::makeVar(cr)?;
                            tmpvar = BackendVariable::setVarTS(
                                tmpvar,
                                Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID),
                            );
                            assign_field!(
                                tmpvar.arryDim =
                                    list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: m })]
                            );
                            arrayLst2 = ({
                                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                                for mut k in (1..=m).into_iter() {
                                    let __x = Expression::makeAsubAddIndex(e.clone(), k.clone())?;
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            });
                            j = 1;
                            for mut e2 in &*arrayLst2 {
                                let mut e2 = e2.clone();
                                let (__pa32, __pa33) = ::match_deref::match_deref! { match &(arrayLst) {
                                    Deref @ metamodelica::ListNode::Cons { head: __pa32, tail: __pa33 } => (__pa32.clone(), __pa33.clone()),
                                    _ => return Err("pattern mismatch"),
                                } };
                                e3 = metamodelica::Own::own(__pa32);
                                arrayLst = metamodelica::Own::own(__pa33);
                                eqn1 = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                                    exp: e2.clone(),
                                    scalar: e3.clone(),
                                    source: DAE::emptyElementSource().clone(),
                                    attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                                });
                                eqns = BackendEquation::add(eqn1.clone(), eqns)?;
                                cr = ComponentReferenceBasics::makeCrefIdent(
                                    {
                                        let mut __mm_s = String::new();
                                        __mm_s.push_str(&*tmpVarPrefix);
                                        __mm_s.push_str(&*intString(idx - 1));
                                        ArcStr::from(__mm_s)
                                    },
                                    Expression::r#typeof(e1.clone())?,
                                    list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                                        exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: j })
                                    })],
                                );
                                j = j + 1;
                                assign_field!(tmpvar.varName = cr);
                                vars = BackendVariable::addVar(tmpvar.clone(), vars)?;
                            }
                        } else {
                            e = e1.clone();
                        }
                        expLst = metamodelica::cons(e, expLst);
                    }
                    left = metamodelica::Ref::new(DAE::Exp::TUPLE {
                        PR: metamodelica::Dangerous::listReverseInPlace(expLst.clone()),
                    });
                    eqn = BackendEquation::generateEquation(left, right, source, attr)?;
                    eqns = BackendEquation::setAtIndex(eqns, i, eqn.clone())?;
                }
            }
        }
        if update {
            for mut i in &*indRemove.reverse() {
                eqns = BackendEquation::delete(i.clone(), eqns)?;
            }
            eqns = BackendEquation::listEquation(&(BackendEquation::equationList(eqns)?))?;
            systlst = metamodelica::cons(
                BackendDAEUtil::createEqSystem(
                    vars,
                    eqns,
                    syst.stateSets.clone(),
                    syst.partitionKind.clone(),
                    syst.removedEqs.clone(),
                ),
                systlst,
            );
        } else {
            systlst = metamodelica::cons(syst.clone(), systlst);
        }
    }
    assign_field!(outDAE.eqs = systlst);
    Ok(outDAE)
}

pub(crate) fn simplifyComplexFunction2(
    mut e1: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut out_lst_e1: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut lst_e: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    if '__try0: {
        if Expression::isArray(&e1) || Expression::isArrayType(&(unwrap_break_err!(Expression::r#typeof(e1.clone()), '__try0))) {
            lst_e = unwrap_break_err!(Expression::getArrayOrRangeContents(e1.clone()), '__try0);
            for mut e in &*lst_e {
                out_lst_e1 = listAppend(simplifyComplexFunction2(e.clone()), out_lst_e1.clone());
            }
        } else if Expression::isRecord(&e1) {
            lst_e = unwrap_break_err!(Expression::splitRecord(&(e1.clone()), &(unwrap_break_err!(Expression::r#typeof(e1.clone()), '__try0))), '__try0);
            for mut e in &*lst_e {
                out_lst_e1 = listAppend(simplifyComplexFunction2(e.clone()), out_lst_e1.clone());
            }
            out_lst_e1 = list![e1.clone()];
        } else {
            out_lst_e1 = list![e1.clone()];
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        out_lst_e1 = list![e1.clone()];
    }
    out_lst_e1
}

// =============================================================================
// section for hets
//
// (h)euristic (e)quation (t)erms (s)ort
// heuristic sorting of terms for better numeric in equations(res, torn,...)
//
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn hets(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    if !metamodelica::stringEq(&(Flags::getConfigString(Flags::HETS.clone())?), &(literal!("none"))) {
        outDAE = hetsWork(inDAE)?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn hetsWork(mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut innerEquation: BackendDAE::InnerEquation =
        <BackendDAE::InnerEquation as ::std::default::Default>::default();
    let mut i: i32 = 0;
    let mut j: i32;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut tvars: metamodelica::List<i32>;
    let mut teqns: metamodelica::List<i32>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    shared = outDAE.shared.clone();
    for mut syst in &*outDAE.eqs.clone() {
        let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, stateSets: __pa4, partitionKind: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        vars = metamodelica::Own::own(__pa0);
        eqns = metamodelica::Own::own(__pa1);
        comps = metamodelica::Own::own(__pa2);
        matching = metamodelica::Own::own(__pa3);
        stateSets = metamodelica::Own::own(__pa4);
        partitionKind = metamodelica::Own::own(__pa5);
        for mut comp in &*comps {
            if BackendEquation::isTornSystem(metamodelica::AsArg::as_arg(&comp)) {
                let (__pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa7, residualequations: __pa8, innerEquations: __pa9, .. }, .. } => (__pa7.clone(), __pa8.clone(), __pa9.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                tvars = metamodelica::Own::own(__pa7);
                teqns = metamodelica::Own::own(__pa8);
                innerEquations = metamodelica::Own::own(__pa9);
                for mut innerEquation in &*innerEquations {
                    let mut innerEquation = innerEquation.clone();
                    if '__try10: {
                        let (__pa11, __pa12) = ::match_deref::match_deref! { match &(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&innerEquation)) {
                            (__pa11, Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Nil }, _) => (__pa11.clone(), __pa12.clone()),
                            _ => break '__try10 Err::<_, _>("pattern mismatch"),
                        } };
                        i = metamodelica::Own::own(__pa11);
                        j = metamodelica::Own::own(__pa12);
                        eqn = unwrap_break_err!(BackendEquation::get(eqns.clone(), i), '__try10);
                        let __arc15 = unwrap_break_err!(BackendVariable::getVarAt(&vars, j), '__try10);
                        let BackendDAE::VAR { varName: __pa14, .. } = &*__arc15;
                        cr = metamodelica::Own::own(__pa14);
                        eqn = unwrap_break_err!(BackendEquation::solveEquation(eqn.clone(), unwrap_break_err!(Expression::crefExp(cr.clone()), '__try10), Some(shared.functionTree.clone())), '__try10);
                        eqn = unwrap_break_err!(hetsSplitRhs(eqn.clone()), '__try10);
                        eqns = unwrap_break_err!(BackendEquation::setAtIndex(eqns.clone(), i, eqn.clone()), '__try10);
                        Ok::<(), &'static str>(())
                    }.is_err() {
                    }
                }
                for mut i in &*teqns {
                    let mut i = i.clone();
                    eqn = BackendEquation::get(eqns.clone(), i)?;
                    eqn = hetsSplitRes(eqn)?;
                    eqns = BackendEquation::setAtIndex(eqns, i, eqn.clone())?;
                }
            } else if BackendEquation::isEquationsSystem(metamodelica::AsArg::as_arg(&comp)) {
                let __pa16 = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __pa16, .. } => __pa16.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                teqns = metamodelica::Own::own(__pa16);
                for mut i in &*teqns {
                    let mut i = i.clone();
                    eqn = BackendEquation::get(eqns.clone(), i)?;
                    eqn = hetsSplitRes(eqn)?;
                    eqns = BackendEquation::setAtIndex(eqns, i, eqn.clone())?;
                }
            }
        }
    }
    Ok(outDAE)
}

fn hetsSplitRes(mut iEqn: metamodelica::Ref<BackendDAE::Equation>) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut oEqn: metamodelica::Ref<BackendDAE::Equation>;
    oEqn = (match &*iEqn {
        BackendDAE::Equation::EQUATION {
            exp: e1,
            scalar: e2,
            source,
            attr,
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = Expression::createResidualExp(e1.clone(), e2.clone())?;
            e = hetsSplitExp(e)?;
            metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION {
                exp: e,
                source: source.clone(),
                attr: attr.clone(),
            })
        }
        BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, attr } => {
            let mut e = (*e).clone();
            e = hetsSplitExp(e.clone())?;
            metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION {
                exp: e.clone(),
                source: source.clone(),
                attr: attr.clone(),
            })
        }
        _ => iEqn,
    });
    Ok(oEqn)
}

fn hetsSplitRhs(mut iEqn: metamodelica::Ref<BackendDAE::Equation>) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut oEqn: metamodelica::Ref<BackendDAE::Equation>;
    oEqn = (match &*iEqn {
        BackendDAE::Equation::EQUATION {
            exp: e1,
            scalar: e2,
            source,
            attr,
        } => {
            let mut e2 = (*e2).clone();
            e2 = hetsSplitExp(e2.clone())?;
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: e1.clone(),
                scalar: e2.clone(),
                source: source.clone(),
                attr: attr.clone(),
            })
        }
        _ => iEqn,
    });
    Ok(oEqn)
}

fn hetsSplitExp(mut iExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (Expression::isMulOrDiv(metamodelica::AsArg::as_arg(&op))) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = hetsSplitExp(e1.clone())?;
            e2 = hetsSplitExp(e2.clone())?;
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        e @ Deref @ DAE::Exp::BINARY { exp1: _, operator: op, exp2: _ } if (Expression::isAddOrSub(metamodelica::AsArg::as_arg(&op))) => {
            let mut terms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut termsDer: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            terms = Expression::terms(e.clone())?;
            terms = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut t in (terms).into_iter().cloned() {
            let __x = hetsSplitExp(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (termsDer, terms) = List::splitOnTrue(&terms, &Expression::expHasDer)?;
            Expression::expAdd(Expression::makeSum1(terms, false)?, Expression::makeSum1(termsDer, false)?)?
        },
        _ => {
            iExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

// =============================================================================
// section inlineFunctionInLoops
// force inlining function of loop
// author: Vitalij Ruge
// motivation see #3997 library devs introduce annotation(Inline=true) for simplify loops
// =============================================================================
pub(crate) fn inlineFunctionInLoops(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    dae = inlineFunctionInLoopsMain(&dae)?;
    Ok(dae)
}

fn inlineFunctionInLoopsMain(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut _syst: metamodelica::Ref<BackendDAE::EqSystem>;
    shared = inDAE.shared.clone();
    functionTree = shared.functionTree.clone();
    eqs = metamodelica::nil();
    for mut syst in &*inDAE.eqs.clone() {
        (_syst, shared) = inlineFunctionInLoopsWork(syst.clone(), functionTree.clone(), shared)?;
        eqs = metamodelica::cons(_syst, eqs);
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqs,
        shared: shared,
    });
    Ok(outDAE)
}

fn inlineFunctionInLoopsWork(
    mut syst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> = syst;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ) = (
        Some(functionTree.clone()),
        list![
            openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
            openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE,
            openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE
        ],
    );
    let mut inlined: bool;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqNew: metamodelica::Ref<BackendDAE::Equation>;
    let mut tmpEqs: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut tmpEqs1: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut idEqns: metamodelica::List<i32>;
    let mut inlined1: bool;
    let mut id: i32 = 0;
    inlined = false;
    inlined1 = false;
    tmpEqs1 = BackendDAEUtil::createEqSystem(
        BackendVariable::listVar(metamodelica::nil())?,
        BackendEquation::listEquation(&(metamodelica::nil()))?,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, stateSets: __pa4, partitionKind: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    vars = metamodelica::Own::own(__pa0);
    eqns = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    matching = metamodelica::Own::own(__pa3);
    stateSets = metamodelica::Own::own(__pa4);
    partitionKind = metamodelica::Own::own(__pa5);
    for mut comp in &*comps {
        if BackendEquation::isEquationsSystem(metamodelica::AsArg::as_arg(&comp))
            || BackendEquation::isTornSystem(metamodelica::AsArg::as_arg(&comp))
            || (match &*comp.clone() {
                BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { .. } => true,
                _ => false,
            })
        {
            idEqns = (match &*comp.clone() {
                BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __esc_idEqns, .. } => {
                    idEqns = (*__esc_idEqns).clone();
                    idEqns.clone()
                }
                BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: __esc_id, .. } => {
                    id = (*__esc_id).clone();
                    list![id.clone()]
                }
                _ => return Err("match: no arm matched"),
            });
            for mut id in &*idEqns {
                let mut id = id.clone();
                eq = BackendEquation::get(eqns.clone(), id)?;
                (eqNew, tmpEqs, inlined, shared) =
                    BackendInline::inlineEqAppend_debug(eq.clone(), fns.clone(), shared)?;
                if inlined || !(BackendEquation::equationEqual(&eq, &eqNew)?) {
                    tmpEqs1 = BackendDAEUtil::mergeEqSystems(&tmpEqs, tmpEqs1)?;
                    eqns = BackendEquation::setAtIndexFirst(id, eqNew, eqns)?;
                    inlined1 = true;
                }
            }
        }
    }
    assign_field!(syst.orderedEqs = eqns);
    if inlined1 {
        syst = BackendDAEUtil::clearEqSyst(&syst);
        syst = BackendDAEUtil::mergeEqSystems(&tmpEqs1, syst)?;
    }
    Ok((syst, shared))
}

// =============================================================================
// section for simplifyLoops
//
// simplify(hopful) loops for simulation/optimization
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn simplifyLoops(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = if (Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 0) {
        simplifyLoopsMain(inDAE)?
    } else {
        inDAE
    };
    Ok(outDAE)
}

fn simplifyLoopsMain(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut new_systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut stateSets: metamodelica::List<BackendDAE::StateSet>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut update: bool;
    let mut index: i32 = 1;
    let mut ii: i32;
    let mut nSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut ass1: metamodelica::List<i32>;
    let mut ass2: metamodelica::List<i32>;
    let mut compOrders: metamodelica::List<i32>;
    let mut ne: i32;
    let mut nv: i32;
    let mut simDAE: bool;
    shared = inDAE.shared.clone();
    functionTree = shared.functionTree.clone();
    simDAE = (match &*shared {
        BackendDAE::Shared {
            backendDAEType: BackendDAE::BackendDAEType::SIMULATION { .. },
            ..
        } => true,
        BackendDAE::Shared {
            backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM { .. },
            ..
        } => true,
        _ => false,
    });
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print(literal!("START: simplifyLoops\n"));
        if !(simDAE) {
            metamodelica::print(literal!("\n***noSIM***\n"));
        }
    }
    for mut syst in &*inDAE.eqs.clone() {
        update = false;
        ass1 = metamodelica::nil();
        ass2 = metamodelica::nil();
        compOrders = metamodelica::nil();
        ii = 1;
        let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, stateSets: __pa4, partitionKind: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        vars = metamodelica::Own::own(__pa0);
        eqns = metamodelica::Own::own(__pa1);
        comps = metamodelica::Own::own(__pa2);
        matching = metamodelica::Own::own(__pa3);
        stateSets = metamodelica::Own::own(__pa4);
        partitionKind = metamodelica::Own::own(__pa5);
        ne = ExpandableArray::getNumberOfElements(eqns.clone());
        let BackendDAE::VARIABLES {
            numberOfVars: __pa7, ..
        } = &vars;
        nv = metamodelica::Own::own(__pa7);
        for mut comp in &*comps {
            if BackendEquation::isEquationsSystem(metamodelica::AsArg::as_arg(&comp))
                || BackendEquation::isTornSystem(metamodelica::AsArg::as_arg(&comp))
            {
                (index, vars, eqns, shared, update, ass1, ass2, compOrders) = simplifyLoopsWork(
                    metamodelica::AsArg::as_arg(&comp),
                    index,
                    vars,
                    eqns,
                    shared,
                    update,
                    ass1,
                    ass2,
                    simDAE,
                    ii,
                    compOrders,
                )?;
            }
            ii = ii + 1;
        }
        nSyst = if (update) {
            simplifyLoopsUpdateMatching(
                vars,
                eqns,
                syst.clone(),
                ass1.reverse(),
                ass2.reverse(),
                ne,
                nv,
                &functionTree,
                compOrders.reverse(),
            )?
        } else {
            syst.clone()
        };
        new_systlst = metamodelica::cons(nSyst, new_systlst);
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: new_systlst,
        shared: shared,
    });
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print(literal!("END: simplifyLoops\n"));
    }
    Ok(outDAE)
}

fn simplifyLoopsUpdateMatching(
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ass1_: metamodelica::List<i32>,
    mut ass2_: metamodelica::List<i32>,
    mut nEqns: i32,
    mut nVars: i32,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut compOrders: metamodelica::List<i32>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem> = inSyst.clone();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut n1: i32;
    let mut n2: i32;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    matching = inSyst.matching.clone();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(matching) {
        Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, ass1: __pa1, ass2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    ass1 = metamodelica::Own::own(__pa1);
    ass2 = metamodelica::Own::own(__pa2);
    n1 = ((ass1_).len() as i32);
    n2 = ((ass2_).len() as i32);
    ass1 = Array::expand(n1, ass1.clone(), -1)?;
    ass2 = Array::expand(n2, ass2.clone(), -1)?;
    ass1 = simplifyLoopsUpdateAss(ass1.clone(), &ass1_, nVars)?;
    ass2 = simplifyLoopsUpdateAss(ass2.clone(), &ass2_, nEqns)?;
    comps = simplifyLoopsUpdateComps(comps, ass1_, ass2_, compOrders)?;
    assign_field!(
        outSyst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
            ass1: ass1.clone(),
            ass2: ass2.clone(),
            comps: comps
        }),
        outSyst.orderedEqs = inEqns,
        outSyst.orderedVars = inVars
    );
    outSyst = BackendDAEUtil::setEqSystMatrices(outSyst, None, None, None);
    Ok(outSyst)
}

fn simplifyLoopsUpdateAss(
    mut inAss: metamodelica::Array<i32>,
    mut new_ass: &metamodelica::List<i32>,
    mut n: i32,
) -> Result<metamodelica::Array<i32>> {
    let mut outAss: metamodelica::Array<i32> = inAss;
    let mut i: i32 = 1;
    for mut a in &**new_ass {
        {
            let __cell0 = a.clone();
            let __idx0 = i + n;
            *metamodelica::index_mut_checked(&mut outAss.clone().borrow_mut(), __idx0)? = __cell0;
        }
        i = i + 1;
    }
    Ok(outAss)
}

fn simplifyLoopsUpdateComps(
    mut inComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inAss1: metamodelica::List<i32>,
    mut inAss2: metamodelica::List<i32>,
    mut inCompOrders: metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>> {
    let mut outComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = inComps;
    let mut a1: i32 = 0;
    let mut a2: i32;
    let mut shift: i32 = 0;
    let mut o: i32;
    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut ass1: metamodelica::List<i32> = inAss1;
    let mut ass2: metamodelica::List<i32> = inAss2;
    let mut compOrders: metamodelica::List<i32> = inCompOrders;
    for mut a1 in &*ass1 {
        let mut a1 = a1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(compOrders) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        o = metamodelica::Own::own(__pa0);
        compOrders = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ass2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        a2 = metamodelica::Own::own(__pa2);
        ass2 = metamodelica::Own::own(__pa3);
        comp = metamodelica::Ref::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: a1, var: a2 });
        outComps = List::insert(outComps, o + shift, comp)?;
        shift = shift + 1;
    }
    Ok(outComps)
}

fn simplifyLoopsWork(
    mut inComp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut inIndx: i32,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inUpdate: bool,
    mut ass1_: metamodelica::List<i32>,
    mut ass2_: metamodelica::List<i32>,
    mut simDAE: bool,
    mut ii: i32,
    mut inCompOrders: metamodelica::List<i32>,
) -> Result<(
    i32,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut outIndx: i32 = inIndx;
    let mut outVars: BackendDAE::Variables = inVars;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        inEqns;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outUpdate: bool = inUpdate;
    let mut ass1: metamodelica::List<i32> = ass1_;
    let mut ass2: metamodelica::List<i32> = ass2_;
    let mut outCompOrders: metamodelica::List<i32> = inCompOrders;
    let mut eqns: metamodelica::List<i32>;
    let mut vv: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut update: bool;
    let mut linear: bool;
    let mut i: i32 = 0;
    let mut k: i32;
    let mut innerEquations: metamodelica::List<BackendDAE::InnerEquation>;
    let mut innerEquation: BackendDAE::InnerEquation =
        <BackendDAE::InnerEquation as ::std::default::Default>::default();
    if BackendEquation::isEquationsSystem(inComp) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inComp)) {
            Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __pa0, vars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        eqns = metamodelica::Own::own(__pa0);
        vars = metamodelica::Own::own(__pa1);
        if BackendDAEUtil::isLinearEqSystemComp(inComp) {
            return Ok((
                outIndx,
                outVars,
                outEqns,
                outShared,
                outUpdate,
                ass1,
                ass2,
                outCompOrders,
            ));
        }
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print(literal!("------ EquationsSystem ------\n"));
        }
    } else {
        let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*inComp)) {
            Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: __pa2, strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa3, residualequations: __pa4, innerEquations: __pa5, .. }, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        linear = metamodelica::Own::own(__pa2);
        vars = metamodelica::Own::own(__pa3);
        eqns = metamodelica::Own::own(__pa4);
        innerEquations = metamodelica::Own::own(__pa5);
        if linear {
            return Ok((
                outIndx,
                outVars,
                outEqns,
                outShared,
                outUpdate,
                ass1,
                ass2,
                outCompOrders,
            ));
        }
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print(literal!("------ Tearing ------\n"));
        }
        for mut innerEquation in &*innerEquations {
            let mut innerEquation = innerEquation.clone();
            (k, vv, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(&innerEquation);
            eqns = metamodelica::cons(k, eqns);
            vars = listAppend(vv, vars);
        }
    }
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print(literal!("------ loop-vars ------\n"));
    }
    for mut i in &*vars {
        let mut i = i.clone();
        let __arc7 = BackendVariable::getVarAt(&outVars, i)?;
        let BackendDAE::VAR { varName: __pa6, .. } = &*__arc7;
        cr = metamodelica::Own::own(__pa6);
        var_lst = metamodelica::cons(cr.clone(), var_lst);
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print(literal!("------------\n"));
    }
    for mut i in &*eqns {
        let mut i = i.clone();
        if '__try8: {
            eqn = unwrap_break_err!(BackendEquation::get(outEqns.clone(), i), '__try8);
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone()), '__try8) {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update eqn[")); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!("]\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(BackendDump::equationString(&eqn), '__try8)); __mm_s.push_str(&*literal!("--old--\n")); ArcStr::from(__mm_s) });
            }
            (outIndx, outVars, outEqns, outShared, update, eqn, ass1, ass2, outCompOrders) = unwrap_break_err!(simplifyLoopEqn(outIndx, outVars.clone(), outEqns.clone(), outShared.clone(), &var_lst, eqn.clone(), ass1.clone(), ass2.clone(), simDAE, ii, outCompOrders.clone()), '__try8);
            outUpdate = outUpdate || update;
            outEqns = unwrap_break_err!(BackendEquation::setAtIndex(outEqns.clone(), i, eqn.clone()), '__try8);
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone()), '__try8) {
                metamodelica::print(literal!("=> "));
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(BackendDump::equationString(&eqn), '__try8)); __mm_s.push_str(&*literal!("--new--\n")); ArcStr::from(__mm_s) });
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok((
        outIndx,
        outVars,
        outEqns,
        outShared,
        outUpdate,
        ass1,
        ass2,
        outCompOrders,
    ))
}

fn simplifyLoopEqn(
    mut inIndx: i32,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut var_lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inEqn: metamodelica::Ref<BackendDAE::Equation>,
    mut ass1_: metamodelica::List<i32>,
    mut ass2_: metamodelica::List<i32>,
    mut simDAE: bool,
    mut ii: i32,
    mut inCompOrders: metamodelica::List<i32>,
) -> Result<(
    i32,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut outIndx: i32 = inIndx;
    let mut outVars: BackendDAE::Variables = inVars;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        inEqns;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outUpdate: bool = false;
    let mut outEqn: metamodelica::Ref<BackendDAE::Equation> = inEqn;
    let mut ass1: metamodelica::List<i32> = ass1_;
    let mut ass2: metamodelica::List<i32> = ass2_;
    let mut outCompOrder: metamodelica::List<i32> = inCompOrders;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut update_lhs: bool;
    let mut update_rhs: bool;
    let mut loopTerms_lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut noLoopTerms_lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut loopTerms_rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut noLoopTerms_rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut useTmpVars: bool = Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 1;
    if BackendEquation::isAlgorithm(&outEqn) {
        return Ok((
            outIndx,
            outVars,
            outEqns,
            outShared,
            outUpdate,
            outEqn,
            ass1,
            ass2,
            outCompOrder,
        ));
    }
    lhs = BackendEquation::getEquationLHS(&outEqn)?;
    if !(Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(lhs.clone())?)) {
        return Ok((
            outIndx,
            outVars,
            outEqns,
            outShared,
            outUpdate,
            outEqn,
            ass1,
            ass2,
            outCompOrder,
        ));
    }
    rhs = BackendEquation::getEquationRHS(&outEqn)?;
    (loopTerms_lhs, noLoopTerms_lhs) = simplifyLoops_SplitTerms(var_lst, lhs)?;
    (loopTerms_rhs, noLoopTerms_rhs) = simplifyLoops_SplitTerms(var_lst, rhs)?;
    if ((loopTerms_lhs).len() as i32) > ((loopTerms_rhs).len() as i32) {
        lhs = Expression::expSub(
            Expression::makeSum1(loopTerms_lhs, false)?,
            Expression::makeSum1(loopTerms_rhs, false)?,
        )?;
        rhs = Expression::expSub(
            Expression::makeSum1(noLoopTerms_rhs, false)?,
            Expression::makeSum1(noLoopTerms_lhs, false)?,
        )?;
    } else {
        lhs = Expression::expSub(
            Expression::makeSum1(loopTerms_rhs, false)?,
            Expression::makeSum1(loopTerms_lhs, false)?,
        )?;
        rhs = Expression::expSub(
            Expression::makeSum1(noLoopTerms_lhs, false)?,
            Expression::makeSum1(noLoopTerms_rhs, false)?,
        )?;
    }
    (lhs, rhs, _) = Expression::createResidualExp3(lhs, rhs);
    (lhs, e) = Expression::makeFraction(lhs)?;
    (lhs, _) = ExpressionSimplify::simplify(lhs)?;
    (e, _) = ExpressionSimplify::simplify(e)?;
    rhs = ExpressionSimplify::simplifySumOperatorExpression(
        rhs.clone(),
        DAE::Operator::MUL {
            ty: Expression::r#typeof(rhs)?,
        },
        e,
    )?;
    (
        outIndx,
        outVars,
        outEqns,
        outShared,
        update_rhs,
        rhs,
        ass1,
        ass2,
        outCompOrder,
    ) = simplifyLoopExp(
        outIndx,
        outVars,
        outEqns,
        outShared,
        var_lst,
        rhs,
        ass1,
        ass2,
        simDAE,
        useTmpVars,
        ii,
        outCompOrder,
        &(literal!("LOOP")),
        false,
    )?;
    (
        outIndx,
        outVars,
        outEqns,
        outShared,
        update_lhs,
        lhs,
        ass1,
        ass2,
        outCompOrder,
    ) = simplifyLoopExp(
        outIndx,
        outVars,
        outEqns,
        outShared,
        var_lst,
        lhs,
        ass1,
        ass2,
        simDAE,
        useTmpVars,
        ii,
        outCompOrder,
        &(literal!("LOOP")),
        false,
    )?;
    outEqn = BackendEquation::setEquationLHS(outEqn, lhs)?;
    outEqn = BackendEquation::setEquationRHS(outEqn, rhs)?;
    outUpdate = outUpdate || update_rhs || update_lhs;
    Ok((
        outIndx,
        outVars,
        outEqns,
        outShared,
        outUpdate,
        outEqn,
        ass1,
        ass2,
        outCompOrder,
    ))
}

pub(crate) fn simplifyLoopExp(
    mut inIndx: i32,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut var_lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut ass1_: metamodelica::List<i32>,
    mut ass2_: metamodelica::List<i32>,
    mut simDAE: bool,
    mut useTmpVars: bool,
    mut ii: i32,
    mut inCompOrders: metamodelica::List<i32>,
    mut tmpVarName: &ArcStr,
    mut noPara: bool,
) -> Result<(
    i32,
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut outIndx: i32 = inIndx;
    let mut outVars: BackendDAE::Variables = inVars;
    let mut outEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        inEqns;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outUpdate: bool = false;
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut ass1: metamodelica::List<i32> = ass1_;
    let mut ass2: metamodelica::List<i32> = ass2_;
    let mut outCompOrder: metamodelica::List<i32> = inCompOrders;
    let mut loopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut noLoopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut loopFactors: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut noLoopFactors: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut loopTermsUpdatedFactors: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut loopFacotrsUpdatedTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut res: metamodelica::Ref<DAE::Exp>;
    let mut noLoopTerm: metamodelica::Ref<DAE::Exp>;
    let mut noLoopFactor: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut update: bool;
    let mut op: DAE::Operator;
    let mut para: bool;
    (loopTerms, noLoopTerms) = simplifyLoops_SplitTerms(var_lst, outExp)?;
    (noLoopTerm, _) = ExpressionSimplify::simplify1(Expression::makeSum1(noLoopTerms, false)?)?;
    if useTmpVars && simDAE {
        (noLoopTerm, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(
            noLoopTerm,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*tmpVarName);
                __mm_s.push_str(&*literal!("T"));
                ArcStr::from(__mm_s)
            }),
            System::tmpTickIndex(Global::tmpVariableIndex.clone()),
            outEqns,
            outVars,
            outShared,
            false,
        )?;
        (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(
            update,
            outUpdate,
            para,
            ass1,
            ass2,
            outVars.clone(),
            outEqns.clone(),
            outIndx,
            ii,
            outCompOrder,
        );
    }
    loopTermsUpdatedFactors = metamodelica::nil();
    for mut factor in &*loopTerms {
        (loopFactors, noLoopFactors) = simplifyLoops_SplitFactors(var_lst, metamodelica::AsArg::as_arg(&factor))?;
        (noLoopFactor, _) = ExpressionSimplify::simplify1(Expression::makeProductLst(noLoopFactors)?)?;
        if useTmpVars && simDAE {
            if (match &*noLoopFactor {
                DAE::Exp::BINARY {
                    operator: DAE::Operator::DIV { .. },
                    ..
                } => true,
                DAE::Exp::BINARY {
                    operator: DAE::Operator::POW { .. },
                    ..
                } => true,
                _ => false,
            }) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(noLoopFactor) {
                    Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e1 = metamodelica::Own::own(__pa0);
                op = metamodelica::Own::own(__pa1);
                e2 = metamodelica::Own::own(__pa2);
                (e1, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(
                    e1,
                    &(literal!("LOOPF")),
                    if (simDAE) { outIndx } else { -(outIndx) },
                    outEqns,
                    outVars,
                    outShared,
                    noPara,
                )?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(
                    update,
                    outUpdate,
                    para,
                    ass1,
                    ass2,
                    outVars.clone(),
                    outEqns.clone(),
                    outIndx,
                    ii,
                    outCompOrder,
                );
                (e2, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(
                    e2,
                    &(literal!("LOOPF")),
                    if (simDAE) { outIndx } else { -(outIndx) },
                    outEqns,
                    outVars,
                    outShared,
                    noPara,
                )?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(
                    update,
                    outUpdate,
                    para,
                    ass1,
                    ass2,
                    outVars.clone(),
                    outEqns.clone(),
                    outIndx,
                    ii,
                    outCompOrder,
                );
                noLoopFactor = metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: e1,
                    operator: op,
                    exp2: e2,
                });
            } else {
                (noLoopFactor, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(
                    noLoopFactor,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmpVarName);
                        __mm_s.push_str(&*literal!("F"));
                        ArcStr::from(__mm_s)
                    }),
                    if (simDAE) { outIndx } else { -(outIndx) },
                    outEqns,
                    outVars,
                    outShared,
                    false,
                )?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(
                    update,
                    outUpdate,
                    para,
                    ass1,
                    ass2,
                    outVars.clone(),
                    outEqns.clone(),
                    outIndx,
                    ii,
                    outCompOrder,
                );
            }
        }
        loopFacotrsUpdatedTerms = metamodelica::nil();
        for mut term in &*loopFactors {
            res = term.clone();
            if Expression::isBinary(&res) {
                let __pa3 = ::match_deref::match_deref! { match &(res.clone()) {
                    Deref @ DAE::Exp::BINARY { operator: __pa3, .. } => __pa3.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                op = metamodelica::Own::own(__pa3);
                if Expression::isAddOrSub(&op) || Expression::isMulOrDiv(&op) || Expression::isPow(&op) {
                    if !(ExpressionBasics::expEqual(&res, inExp.clone())?) {
                        if Expression::isDiv(&op) || Expression::isPow(&op) {
                            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(res) {
                                Deref @ DAE::Exp::BINARY { exp1: __pa4, operator: __pa5, exp2: __pa6 } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            e1 = metamodelica::Own::own(__pa4);
                            op = metamodelica::Own::own(__pa5);
                            e2 = metamodelica::Own::own(__pa6);
                            (
                                outIndx,
                                outVars,
                                outEqns,
                                outShared,
                                update,
                                e1,
                                ass1,
                                ass2,
                                outCompOrder,
                            ) = simplifyLoopExp(
                                outIndx,
                                outVars,
                                outEqns,
                                outShared,
                                var_lst,
                                e1,
                                ass1,
                                ass2,
                                simDAE,
                                useTmpVars,
                                ii,
                                outCompOrder,
                                &(literal!("LOOP")),
                                false,
                            )?;
                            outUpdate = update || outUpdate;
                            (
                                outIndx,
                                outVars,
                                outEqns,
                                outShared,
                                update,
                                e2,
                                ass1,
                                ass2,
                                outCompOrder,
                            ) = simplifyLoopExp(
                                outIndx,
                                outVars,
                                outEqns,
                                outShared,
                                var_lst,
                                e2,
                                ass1,
                                ass2,
                                simDAE,
                                useTmpVars,
                                ii,
                                outCompOrder,
                                &(literal!("LOOP")),
                                false,
                            )?;
                            outUpdate = update || outUpdate;
                            (e2, _) = ExpressionSimplify::simplify1(e2)?;
                            res = metamodelica::Ref::new(DAE::Exp::BINARY {
                                exp1: e1,
                                operator: op,
                                exp2: e2,
                            });
                        } else {
                            (
                                outIndx,
                                outVars,
                                outEqns,
                                outShared,
                                update,
                                res,
                                ass1,
                                ass2,
                                outCompOrder,
                            ) = simplifyLoopExp(
                                outIndx,
                                outVars,
                                outEqns,
                                outShared,
                                var_lst,
                                res,
                                ass1,
                                ass2,
                                simDAE,
                                useTmpVars,
                                ii,
                                outCompOrder,
                                &(literal!("LOOP")),
                                false,
                            )?;
                            outUpdate = update || outUpdate;
                        }
                    }
                }
            }
            loopFacotrsUpdatedTerms = metamodelica::cons(res, loopFacotrsUpdatedTerms);
        }
        loopTermsUpdatedFactors = metamodelica::cons(
            Expression::makeProductLst(metamodelica::cons(noLoopFactor, loopFacotrsUpdatedTerms))?,
            loopTermsUpdatedFactors,
        );
    }
    (outExp, _) = ExpressionSimplify::simplify(Expression::makeSum1(
        metamodelica::cons(noLoopTerm, loopTermsUpdatedFactors),
        true,
    )?)?;
    Ok((
        outIndx,
        outVars,
        outEqns,
        outShared,
        outUpdate,
        outExp,
        ass1,
        ass2,
        outCompOrder,
    ))
}

fn simplifyLoopExpHelper(
    mut update: bool,
    mut update_: bool,
    mut para: bool,
    mut ass1_: metamodelica::List<i32>,
    mut ass2_: metamodelica::List<i32>,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inIndex: i32,
    mut ii: i32,
    mut inCompOrders: metamodelica::List<i32>,
) -> (
    bool,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    i32,
    metamodelica::List<i32>,
) {
    let mut outUpdate: bool = update_;
    let mut ass1: metamodelica::List<i32> = ass1_;
    let mut ass2: metamodelica::List<i32> = ass2_;
    let mut outIndx: i32 = inIndex;
    let mut outCompOrder: metamodelica::List<i32> = inCompOrders;
    let mut ne: i32;
    let mut nv: i32;
    if update {
        outIndx = outIndx + 1;
        outUpdate = update;
        if !(para) {
            ne = ExpandableArray::getNumberOfElements(inEqns);
            let BackendDAE::VARIABLES {
                numberOfVars: __pa0, ..
            } = inVars;
            nv = metamodelica::Own::own(__pa0);
            ass1 = metamodelica::cons(ne, ass1);
            ass2 = metamodelica::cons(nv, ass2);
            outCompOrder = metamodelica::cons(ii, outCompOrder);
        }
    }
    (outUpdate, ass1, ass2, outIndx, outCompOrder)
}

pub(crate) fn simplifyLoops_SplitTerms(
    mut var_lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut loopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut noLoopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tmp_loopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    noLoopTerms = Expression::terms(inExp)?;
    for mut cr in &**var_lst {
        if (noLoopTerms).is_empty() {
            break;
        } else {
            (tmp_loopTerms, noLoopTerms) =
                List::split1OnTrue(&noLoopTerms, &Expression::expHasCrefNoPreOrStart, cr.clone())?;
            loopTerms = listAppend(tmp_loopTerms, loopTerms);
        }
    }
    Ok((loopTerms, noLoopTerms))
}

fn simplifyLoops_SplitFactors(
    mut var_lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut loopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut noLoopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tmp_loopTerms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    noLoopTerms = Expression::factors(inExp)?;
    for mut cr in &**var_lst {
        if (noLoopTerms).is_empty() {
            break;
        } else {
            (tmp_loopTerms, noLoopTerms) =
                List::split1OnTrue(&noLoopTerms, &Expression::expHasCrefNoPreOrStart, cr.clone())?;
            loopTerms = listAppend(tmp_loopTerms, loopTerms);
        }
    }
    Ok((loopTerms, noLoopTerms))
}

// =============================================================================
// section for introduceDerAlias
//
// =============================================================================
pub(crate) fn introduceDerAlias(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &introduceDerAliasWork)?;
    Ok(outDAE)
}

fn introduceDerAliasWork(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqnsList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    osyst = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
            vars = (*__esc_vars).clone();
            eqns = (*__esc_eqns).clone();
            let mut syst = (*syst).clone();
            let (__pa0, (__pa1, __pa2, _, _)) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, metamodelica::Ref<BackendDAE::Shared>, bool)| traverserintroduceDerAliasEquation(__a0, &__a1), (vars.clone(), metamodelica::nil(), shared, true))?;
            eqns = metamodelica::Own::own(__pa0);
            vars = metamodelica::Own::own(__pa1);
            eqnsList = metamodelica::Own::own(__pa2);
            eqns = BackendEquation::addList(&eqnsList, eqns.clone())?;
            assign_field!(
                syst.orderedEqs = eqns.clone(),
                syst.orderedVars = vars.clone()
            );
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn traverserintroduceDerAliasEquation(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut tpl: &(
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
    );
    let mut e: metamodelica::Ref<BackendDAE::Equation>;
    let mut vars: BackendDAE::Variables;
    let mut b: bool;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (vars, eqnLst, shared, b) = tpl.clone();
    let (__pa0, (__pa1, __pa2, __pa3, __pa4, _)) = BackendEquation::traverseExpsOfEquation(
        inEq,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Exp>,
                  __a1: (
                BackendDAE::Variables,
                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                metamodelica::Ref<BackendDAE::Shared>,
                metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                bool,
            )| traverserintroduceDerAliasExp(__a0, &__a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            BackendDAE::Variables,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                            bool,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            BackendDAE::Variables,
                            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                            metamodelica::Ref<BackendDAE::Shared>,
                            metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                            bool,
                        ),
                    )> + 'static,
            >),
        (vars, eqnLst, shared, metamodelica::nil(), b),
    )?;
    e = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    eqnLst = metamodelica::Own::own(__pa2);
    shared = metamodelica::Own::own(__pa3);
    ops = metamodelica::Own::own(__pa4);
    outEq = List::foldr(&ops, &BackendEquation::addOperation, e)?;
    outTpl = (vars, eqnLst, shared, b);
    Ok((outEq, outTpl))
}

fn traverserintroduceDerAliasExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: &(
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
        bool,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
        bool,
    );
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut ext_arg: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
        bool,
    );
    let mut vars: BackendDAE::Variables;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let mut b: bool;
    let mut addVars: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    e = inExp;
    (vars, eqnLst, shared, ops, addVars) = tpl.clone();
    ext_arg = (vars, eqnLst, shared, addVars, false);
    let (__pa0, (__pa1, __pa2, __pa3, _, __pa4)) = Expression::traverseExpBottomUp(
        e.clone(),
        &fnptr!(
            introDerAlias,
            metamodelica::Ref<DAE::Exp>,
            (
                BackendDAE::Variables,
                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                metamodelica::Ref<BackendDAE::Shared>,
                bool,
                bool
            )
        ),
        ext_arg,
    )?;
    e1 = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    eqnLst = metamodelica::Own::own(__pa2);
    shared = metamodelica::Own::own(__pa3);
    b = metamodelica::Own::own(__pa4);
    ops = List::consOnTrue(
        b,
        metamodelica::Ref::new(DAE::SymbolicOperation::SUBSTITUTION {
            substitutions: list![e1.clone()],
            source: e,
        }),
        ops,
    );
    outExp = e1;
    outTpl = (vars, eqnLst, shared, ops, addVars);
    Ok((outExp, outTpl))
}

fn introDerAlias(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
        bool,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
        bool,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut tpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::Ref<BackendDAE::Shared>,
        bool,
        bool,
    );
    (outExp, tpl) = 'mc: {
        let __mc_input = (&*inExp, &itpl);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, eqnLst, shared, addVar, _)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut numVars: i32;
                    let mut vars = (*vars).clone();
                    let mut eqnLst = (*eqnLst).clone();
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?;
                    cref = BackendVariable::varCref(&v);
                    v1 = BackendVariable::createAliasDerVar(&cref)?;
                    v1 = BackendVariable::mergeNominalAttribute(v.clone(), v1.clone(), false);
                    cref = BackendVariable::varCref(&v1);
                    outExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                    if addVar.clone() {
                        numVars = BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars));
                        vars = BackendVariable::addVar(v1.clone(), vars.clone())?;
                        eqnLst = if (numVars < BackendVariable::varsSize(metamodelica::AsArg::as_arg(&vars))) {metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: inExp.clone(), scalar: outExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), eqnLst.clone())} else {eqnLst.clone()};
                    }
                    Ok(((outExp.clone(), (vars.clone(), eqnLst.clone(), shared.clone(), addVar.clone(), true)), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    let mut r#str: ArcStr;
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.introduceDerAlias failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl)
}

// =============================================================================
// section for replaceDerCall
//
// =============================================================================
pub(crate) fn replaceDerCalls(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, &replaceDerCallWork)?;
    Ok(outDAE)
}

fn replaceDerCallWork(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = shared;
    osyst = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut localKnowns: BackendDAE::Variables;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            (eqns, vars) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), &traverserreplaceDerCall, vars.clone())?;
            (localKnowns, vars) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(moveStatesVariables) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(metamodelica::Ref<BackendDAE::Var>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (oshared.localKnownVars.clone(), vars.clone()))?;
            assign_field!(oshared.localKnownVars = localKnowns);
            assign_field!(
                syst.orderedEqs = eqns.clone(),
                syst.orderedVars = vars.clone()
            );
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn traverserreplaceDerCall(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inVars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outVars: BackendDAE::Variables = inVars;
    let mut e: metamodelica::Ref<BackendDAE::Equation>;
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    (e, ops) = BackendEquation::traverseExpsOfEquation(
        inEq,
        (std::sync::Arc::new(traverserreplaceDerCallExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
                    )> + 'static,
            >),
        metamodelica::nil(),
    )?;
    outEq = List::foldr(&ops, &BackendEquation::addOperation, e)?;
    Ok((outEq, outVars))
}

fn traverserreplaceDerCallExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut b: bool;
    e = inExp;
    (e1, b) = Expression::traverseExpBottomUp(
        e.clone(),
        &fnptr!(replaceDerCall, metamodelica::Ref<DAE::Exp>, bool),
        false,
    )?;
    outTpl = List::consOnTrue(
        b,
        metamodelica::Ref::new(DAE::SymbolicOperation::SUBSTITUTION {
            substitutions: list![e1.clone()],
            source: e,
        }),
        tpl,
    );
    outExp = e1;
    Ok((outExp, outTpl))
}

fn replaceDerCall(mut inExp: metamodelica::Ref<DAE::Exp>, mut itpl: bool) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut tpl: bool;
    (outExp, tpl) = 'mc: {
        let __mc_input = &*inExp;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    cref = ComponentReference::crefPrefixDer(cr.clone());
                    outExp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                    Ok(((outExp.clone(), true), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExp = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.replaceDerCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl)
}

fn moveStatesVariables(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: (BackendDAE::Variables, BackendDAE::Variables),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (BackendDAE::Variables, BackendDAE::Variables),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables) = inTpl.clone();
    let () = (match &*inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { .. },
            varName: cref,
            ..
        } => {
            let mut newVar: metamodelica::Ref<BackendDAE::Var>;
            let mut localKnowns: BackendDAE::Variables;
            let mut newVars: BackendDAE::Variables;
            let mut cref = (*cref).clone();
            (localKnowns, newVars) = inTpl;
            newVars = BackendVariable::deleteVar(cref.clone(), &newVars)?;
            localKnowns = BackendVariable::addVar(inVar.clone(), localKnowns)?;
            cref = ComponentReference::crefPrefixDer(cref.clone());
            newVar = BackendVariable::copyVarNewName(cref.clone(), inVar);
            newVar = BackendVariable::setVarKind(newVar, openmodelica_backend_types::BackendDAE::VarKind::STATE_DER)?;
            newVars = BackendVariable::addVar(newVar, newVars)?;
            outTpl = (localKnowns, newVars);
            ()
        }
        _ => (),
    });
    Ok((outVar, outTpl))
}

// =============================================================================
// replace expression with rewritten expression
//
// =============================================================================
pub fn applyRewriteRulesBackend(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &fnptr!(
            applyRewriteRulesBackend0,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            bool
        ),
        false,
    )?;
    outDAE = applyRewriteRulesBackendShared(&outDAE)?;
    Ok(outDAE)
}

fn applyRewriteRulesBackend0(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outChanged: bool;
    match '__try0: {
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(&isyst.orderedVars, (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(isyst.orderedEqs.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(isyst.removedEqs.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false), '__try0);
        outChanged = true;
        Ok::<_, &'static str>((outChanged.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outChanged = __try0_o0;
        }
        Err(_) => {
            outChanged = false;
        }
    }
    (osyst, outShared, outChanged)
}

fn traverserapplyRewriteRulesBackend(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inB: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outB: bool;
    (outExp, outB) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(traverserExpapplyRewriteRulesBackend, metamodelica::Ref<DAE::Exp>, bool),
        inB,
    )?;
    Ok((outExp, outB))
}

fn traverserExpapplyRewriteRulesBackend(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inB: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outB: bool;
    (outExp, outB) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(RewriteRules::rewriteBackEnd(metamodelica::AsArg::as_arg(&e))?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    Ok((e.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inB))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outB)
}

fn applyRewriteRulesBackendShared(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    shared = inDAE.shared.clone();
    BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(
        &shared.globalKnownVars,
        (std::sync::Arc::new(traverserapplyRewriteRulesBackend)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        shared.initialEqs.clone(),
        (std::sync::Arc::new(traverserapplyRewriteRulesBackend)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(
        shared.removedEqs.clone(),
        (std::sync::Arc::new(traverserapplyRewriteRulesBackend)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        false,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: inDAE.eqs.clone(),
        shared: shared,
    });
    Ok(outDAE)
}

// =============================================================================
// generates a list with all iteration variables
//
// =============================================================================
pub(crate) fn listAllIterationVariables(mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<()> {
    let mut backendDAEType: BackendDAE::BackendDAEType;
    let mut warnings: metamodelica::List<ArcStr>;
    let __arc2 = &(*inBackendDAE);
    let BackendDAE::DAE { shared: __t1, .. } = &**__arc2;
    let __arc3 = __t1.clone();
    let BackendDAE::SHARED {
        backendDAEType: __pa0, ..
    } = &*__arc3;
    backendDAEType = metamodelica::Own::own(__pa0);
    (warnings, _) = listAllIterationVariables0(&inBackendDAE.eqs)?;
    Error::addCompilerNotification({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("List of all iteration variables (DAE kind: "));
        __mm_s.push_str(&*BackendDump::printBackendDAEType2String(backendDAEType)?);
        __mm_s.push_str(&*literal!(")\n"));
        __mm_s.push_str(&*stringDelimitList(warnings, literal!("\n")));
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

pub(crate) fn listAllIterationVariables0(
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outWarnings: metamodelica::List<ArcStr>;
    let mut outComponentRef: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut warnings: metamodelica::List<ArcStr>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut warnings_accum: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut crefs_accum: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> =
        metamodelica::nil();
    for mut eq in &**inEqs {
        (warnings, crefs) = listAllIterationVariables1(metamodelica::AsArg::as_arg(&eq))?;
        warnings_accum = metamodelica::cons(warnings, warnings_accum);
        crefs_accum = metamodelica::cons(crefs, crefs_accum);
    }
    outWarnings = List::flattenReverse(warnings_accum)?;
    outComponentRef = List::flattenReverse(crefs_accum)?;
    Ok((outWarnings, outComponentRef))
}

fn listAllIterationVariables1(
    mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outWarning: metamodelica::List<ArcStr>;
    let mut outComponentRef: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut vars: BackendDAE::Variables;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inEqSystem)) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    vars = metamodelica::Own::own(__pa0);
    comps = metamodelica::Own::own(__pa1);
    (outWarning, outComponentRef) = listAllIterationVariables2(comps, &vars)?;
    Ok((outWarning, outComponentRef))
}

fn listAllIterationVariables2(
    mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut vars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut warnings: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut componentRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut var_idxs: metamodelica::List<i32>;
    let mut var_idxs2: metamodelica::List<i32>;
    let NONLINEAR_SYSTEM: ArcStr = literal!("Iteration variables of nonlinear equation system:\n");
    let ANALYTIC_JACOBIAN: ArcStr = literal!("Iteration variables of equation system with analytic Jacobian:\n");
    let NO_ANALYTIC_JACOBIAN: ArcStr = literal!("Iteration variables of equation system without analytic Jacobian:\n");
    let TORN_LINEAR: ArcStr = literal!("Iteration variables of torn linear equation system:\n");
    let TORN_NONLINEAR: ArcStr = literal!("Iteration variables of torn nonlinear equation system:\n");
    for mut comp in &*comps.reverse() {
        (warnings, componentRefs) = (match &*comp.clone() {
            BackendDAE::StrongComponent::EQUATIONSYSTEM {
                jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. },
                vars: __comp_vars,
                ..
            } => listAllIterationVariables3(__comp_vars.clone(), vars, &NONLINEAR_SYSTEM, warnings, componentRefs)?,
            BackendDAE::StrongComponent::EQUATIONSYSTEM {
                jacType: BackendDAE::JacobianType::JAC_GENERIC { .. },
                vars: __comp_vars,
                ..
            } => listAllIterationVariables3(__comp_vars.clone(), vars, &ANALYTIC_JACOBIAN, warnings, componentRefs)?,
            BackendDAE::StrongComponent::EQUATIONSYSTEM {
                jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. },
                vars: __comp_vars,
                ..
            } => listAllIterationVariables3(
                __comp_vars.clone(),
                vars,
                &NO_ANALYTIC_JACOBIAN,
                warnings,
                componentRefs,
            )?,
            BackendDAE::StrongComponent::TORNSYSTEM {
                strictTearingSet:
                    BackendDAE::TearingSet {
                        tearingvars: __esc_var_idxs,
                        ..
                    },
                casualTearingSet: None,
                linear: __comp_linear,
                ..
            } => {
                var_idxs = (*__esc_var_idxs).clone();
                listAllIterationVariables3(
                    var_idxs.clone(),
                    vars,
                    &(if (__comp_linear.clone()) {
                        TORN_LINEAR.clone()
                    } else {
                        TORN_NONLINEAR.clone()
                    }),
                    warnings,
                    componentRefs,
                )?
            }
            BackendDAE::StrongComponent::TORNSYSTEM {
                strictTearingSet:
                    BackendDAE::TearingSet {
                        tearingvars: __esc_var_idxs,
                        ..
                    },
                casualTearingSet:
                    Some(BackendDAE::TearingSet {
                        tearingvars: __esc_var_idxs2,
                        ..
                    }),
                linear: __comp_linear,
                ..
            } => {
                var_idxs = (*__esc_var_idxs).clone();
                var_idxs2 = (*__esc_var_idxs2).clone();
                listAllIterationVariables3(
                    List::union(
                        metamodelica::AsArg::as_arg(&var_idxs),
                        metamodelica::AsArg::as_arg(&var_idxs2),
                    ),
                    vars,
                    &(if (__comp_linear.clone()) {
                        TORN_LINEAR.clone()
                    } else {
                        TORN_NONLINEAR.clone()
                    }),
                    warnings,
                    componentRefs,
                )?
            }
            _ => (warnings, componentRefs),
        });
    }
    Ok((warnings, componentRefs))
}

fn listAllIterationVariables3(
    mut varIndices: metamodelica::List<i32>,
    mut allVars: &BackendDAE::Variables,
    mut message: &ArcStr,
    mut warnings: metamodelica::List<ArcStr>,
    mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut warnings: metamodelica::List<ArcStr> = warnings;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = crefs;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    if !((varIndices).is_empty()) {
        vars = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut v in (varIndices).into_iter().cloned() {
                let __x = BackendVariable::getVarAt(allVars, v.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        crefs = List::append_reverse(
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut v in (vars.clone()).into_iter().cloned() {
                    let __x = BackendVariable::varCref(&(v.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            crefs,
        );
        warnings = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*message);
                __mm_s.push_str(&*warnAboutVars(vars)?);
                ArcStr::from(__mm_s)
            },
            warnings,
        );
    }
    Ok((warnings, crefs))
}

fn warnAboutVars(mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut v in (vars).into_iter().cloned() {
                let __x = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("  "));
                    __mm_s.push_str(&*BackendDump::varString(&(v.clone()))?);
                    ArcStr::from(__mm_s)
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        literal!("\n"),
    );
    Ok(r#str)
}

pub(crate) fn addTimeAsState(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut eq: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let (__t2, _) = BackendDAEUtil::mapEqSystemAndFold(
        &inDAE,
        &fnptr!(
            addTimeAsState1,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            i32
        ),
        0,
    )?;
    let __arc3 = __t2.clone();
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &*__arc3;
    eqs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    orderedVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    var = metamodelica::Ref::new(BackendDAE::Var {
        varName: DAE::crefTimeState().clone(),
        varKind: BackendDAE::VarKind::STATE {
            index: 1,
            derName: None,
            natural: true,
        },
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: DAE::T_REAL_DEFAULT().clone(),
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: true,
        initNonlinear: false,
        encrypted: false,
    });
    var = BackendVariable::setVarFixed(var, true)?;
    var = BackendVariable::setVarStartValue(
        var,
        metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: DAE::crefTime().clone(),
            ty: DAE::T_REAL_DEFAULT().clone(),
        }),
    )?;
    orderedVars = BackendVariable::addVar(var, orderedVars)?;
    orderedEqs = BackendEquation::emptyEqnsSized(1);
    orderedEqs = BackendEquation::add(
        metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: metamodelica::Ref::new(DAE::Exp::CALL {
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
                expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: DAE::crefTimeState().clone(),
                    ty: DAE::T_REAL_DEFAULT().clone()
                })],
                attr: DAE::callAttrBuiltinReal().clone(),
            }),
            scalar: metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(1.0_f64),
            }),
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
        }),
        orderedEqs,
    )?;
    eq = BackendDAEUtil::createEqSystem(
        orderedVars,
        orderedEqs,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION,
        BackendEquation::emptyEqns(),
    );
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: metamodelica::cons(eq, eqs),
        shared: shared,
    });
    Ok(outDAE)
}

fn addTimeAsState1(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inFoo: i32,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
) {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outFoo: i32 = inFoo;
    outSystem = 'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), &addTimeAsState2, inFoo)?;
                    Ok(syst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inSystem.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outSystem, outShared, outFoo)
}

fn addTimeAsState2(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inFoo: i32,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, i32)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outFoo: i32 = inFoo;
    (outEq, _) = BackendEquation::traverseExpsOfEquation(
        inEq,
        (std::sync::Arc::new(addTimeAsState3)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, i32) -> Result<(metamodelica::Ref<DAE::Exp>, i32)>
                    + 'static,
            >),
        inFoo,
    )?;
    Ok((outEq, outFoo))
}

fn addTimeAsState3(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: i32;
    (outExp, outTuple) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(addTimeAsState4, metamodelica::Ref<DAE::Exp>, i32),
        inTuple,
    )?;
    Ok((outExp, outTuple))
}

fn addTimeAsState4(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: i32,
) -> (metamodelica::Ref<DAE::Exp>, bool, i32) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTuple: i32 = inTuple;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, ty } => {
            metamodelica::Ref::new(DAE::Exp::CREF { componentRef: DAE::crefTimeState().clone(), ty: ty.clone() })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outTuple)
}

//-------------------------------------
//Evaluate Output Variables Only.
//-------------------------------------
pub(crate) fn evaluateOutputsOnly(
    mut daeIn: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut daeOut: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut size: i32;
    let mut nVars: i32;
    let mut nEqs: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut varVisited: metamodelica::Array<i32>;
    let mut outputVarIndxs: metamodelica::List<i32>;
    let mut stateIndxs: metamodelica::List<i32>;
    let mut stateTasks: metamodelica::List<i32>;
    let mut stateTasks1: metamodelica::List<i32>;
    let mut outputTasks: metamodelica::List<i32>;
    let mut predecessors: metamodelica::List<i32>;
    let mut tasks: metamodelica::List<i32>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut compsNew: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut addComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut comp: metamodelica::Ref<BackendDAE::StrongComponent> =
        <metamodelica::Ref<BackendDAE::StrongComponent> as ::std::default::Default>::default();
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut systsNew: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut matching: metamodelica::Ref<BackendDAE::Matching>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqLstNew: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varLstNew: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut states: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut taskGraph: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphT: metamodelica::Array<metamodelica::List<i32>>;
    let mut taskGraphData: HpcOmTaskGraph::TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut systemNumber: i32 = 0;
    let mut numberOfSystems: i32;
    let mut eqIndLst: metamodelica::List<i32>;
    let mut eqIndexLst: metamodelica::List<i32> = metamodelica::nil();
    let mut der_replacement: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>>,
    >;
    let mut derVar: metamodelica::Ref<BackendDAE::Var>;
    let debug: bool = false;
    daeOut = daeIn.clone();
    let __arc2 = daeIn;
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &*__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    let __arc4 = shared.clone();
    let BackendDAE::SHARED {
        functionTree: __pa3, ..
    } = &*__arc4;
    funcTree = metamodelica::Own::own(__pa3);
    systsNew = metamodelica::nil();
    numberOfSystems = ((systs).len() as i32);
    for mut syst in &*systs {
        let mut syst = syst.clone();
        systemNumber = systemNumber + 1;
        let __arc8 = syst.clone();
        let BackendDAE::EQSYSTEM {
            orderedVars: __pa5,
            orderedEqs: __pa6,
            matching: __pa7,
            ..
        } = &*__arc8;
        vars = metamodelica::Own::own(__pa5);
        eqs = metamodelica::Own::own(__pa6);
        matching = metamodelica::Own::own(__pa7);
        let (__pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(matching) {
            Deref @ BackendDAE::Matching::MATCHING { ass1: __pa9, ass2: __pa10, comps: __pa11 } => (__pa9.clone(), __pa10.clone(), __pa11.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ass1 = metamodelica::Own::own(__pa9);
        ass2 = metamodelica::Own::own(__pa10);
        comps = metamodelica::Own::own(__pa11);
        (taskGraph, taskGraphData) = HpcOmTaskGraph::getEmptyTaskGraph(0, 0, 0);
        (taskGraph, taskGraphData, _) = HpcOmTaskGraph::createTaskGraph0(
            syst.clone(),
            shared.clone(),
            false,
            &((taskGraph.clone(), taskGraphData, 1)),
        )?;
        let HpcOmTaskGraph::TASKGRAPHMETA {
            varCompMapping: __pa12,
            eqCompMapping: __pa13,
            ..
        } = taskGraphData;
        varCompMapping = metamodelica::Own::own(__pa12);
        eqCompMapping = metamodelica::Own::own(__pa13);
        size = metamodelica::arrayLength(taskGraph.clone());
        taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraph.clone(), size)?;
        let __arc15 = syst.clone();
        let BackendDAE::EQSYSTEM {
            orderedVars: __pa14, ..
        } = &*__arc15;
        vars = metamodelica::Own::own(__pa14);
        varLst = BackendVariable::varList(&vars)?;
        varLst = List::filterOnTrue(
            varLst,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isVarOnTopLevelAndOutput(&__a0))
                },
            )
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
        )?;
        if !((varLst).is_empty()) {
            outputVarIndxs = BackendVariable::getVarIndexFromVars(&varLst, &vars);
            outputTasks = List::map(
                List::map1(outputVarIndxs, &Array::getIndexFirst, varCompMapping.clone())?,
                &fnptr!(Util::tuple31, _),
            )?;
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("outputTasks "));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(outputTasks.clone(), &fnptr!(intString, i32))?,
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            predecessors = HpcOmTaskGraph::getAllSuccessors(outputTasks.clone(), taskGraphT.clone())?;
            predecessors = List::sort(
                predecessors,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            compsNew = List::map1(
                listAppend(outputTasks.clone(), predecessors.clone()),
                &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
                comps.clone(),
            )?;
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("predecessors of outputs "));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(predecessors, &fnptr!(intString, i32))?,
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            eqLstNew = BackendDAEUtil::getStrongComponentEquations(&compsNew, eqs.clone(), vars.clone())?;
            stateTasks = metamodelica::nil();
            varVisited = arrayCreate(BackendVariable::varsSize(&vars), -1);
            while !((eqLstNew).is_empty()) {
                let (__pa16, __pa17) = ::match_deref::match_deref! { match &(eqLstNew) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa16, tail: __pa17 } => (__pa16.clone(), __pa17.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                eq = metamodelica::Own::own(__pa16);
                eqLstNew = metamodelica::Own::own(__pa17);
                if debug {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("eq: "));
                        __mm_s.push_str(&*BackendDump::equationString(&eq)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                crefs = BackendEquation::equationCrefs(eq)?;
                crefs = List::filter1OnTrue(
                    crefs,
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<DAE::ComponentRef>,
                              __a1: BackendDAE::Variables|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(BackendVariable::isState(__a0, &__a1))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::ComponentRef>,
                                    BackendDAE::Variables,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    vars.clone(),
                )?;
                (states, stateIndxs) = BackendVariable::getVarLst(&crefs, &vars);
                (stateIndxs, states) =
                    List::filter1OnTrueSync(&stateIndxs, &stateVarIsNotVisited, varVisited.clone(), states)?;
                if !((stateIndxs).is_empty()) {
                    if debug {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("states "));
                            __mm_s.push_str(&*stringDelimitList(
                                List::map(states, &move |__a0: metamodelica::Ref<BackendDAE::Var>| {
                                    BackendDump::varString(&__a0)
                                })?,
                                literal!("\n "),
                            ));
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    List::map2_0(&stateIndxs, &Array::updateIndexFirst, 1, varVisited.clone())?;
                    stateTasks1 = List::map(
                        List::map1(stateIndxs, &Array::getIndexFirst, varCompMapping.clone())?,
                        &fnptr!(Util::tuple31, _),
                    )?;
                    stateTasks = List::append_reverse(&stateTasks1, stateTasks);
                    predecessors = HpcOmTaskGraph::getAllSuccessors(stateTasks1.clone(), taskGraphT.clone())?;
                    addComps = List::map1(
                        listAppend(stateTasks1, predecessors),
                        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
                        comps.clone(),
                    )?;
                    eqLstNew = listAppend(
                        BackendDAEUtil::getStrongComponentEquations(&addComps, eqs.clone(), vars.clone())?,
                        eqLstNew,
                    );
                }
            }
            stateTasks = Dangerous::listReverseInPlace(stateTasks);
            predecessors = HpcOmTaskGraph::getAllSuccessors(
                listAppend(outputTasks.clone(), stateTasks.clone()),
                taskGraphT.clone(),
            )?;
            tasks = List::sort(
                listAppend(predecessors, listAppend(outputTasks, stateTasks)),
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?;
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("predecessors of outputs and states "));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(tasks.clone(), &fnptr!(intString, i32))?,
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            compsNew = List::map1(
                tasks,
                &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
                comps.clone(),
            )?;
            compsNew = List::unique(&compsNew);
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("There have been "));
                    __mm_s.push_str(&*intString(((comps).len() as i32)));
                    __mm_s.push_str(&*literal!(" SCCs and now there are "));
                    __mm_s.push_str(&*intString(((compsNew).len() as i32)));
                    __mm_s.push_str(&*literal!(" SCCs.\n"));
                    ArcStr::from(__mm_s)
                });
            }
            eqLstNew = metamodelica::nil();
            varLstNew = metamodelica::nil();
            for mut comp in &*compsNew {
                let mut comp = comp.clone();
                (varLst, _, eqLst, eqIndLst) =
                    BackendDAEUtil::getStrongComponentVarsAndEquations(&comp, vars.clone(), eqs.clone())?;
                varLstNew = listAppend(varLst, varLstNew);
                eqLstNew = listAppend(eqLst, eqLstNew);
                eqIndexLst = listAppend(eqIndLst, eqIndexLst);
            }
            assign_field!(
                syst.orderedVars = BackendVariable::listVar1(&(varLstNew.clone().reverse()))?,
                syst.orderedEqs = BackendEquation::listEquation(&(eqLstNew.clone().reverse()))?,
                syst.m = None,
                syst.mT = None,
                syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING()
            );
            (m, mT) = BackendDAEUtil::adjacencyMatrix(
                &syst,
                openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
                None,
                BackendDAEUtil::isInitializationDAE(&shared),
            )?;
            assign_field!(syst.m = Some(m.clone()), syst.mT = Some(mT.clone()));
            nVars = ((varLstNew).len() as i32);
            nEqs = ((eqLstNew).len() as i32);
            ass1 = arrayCreate(nVars, -1);
            ass2 = arrayCreate(nEqs, -1);
            Matching::matchingExternalsetAdjacencyMatrix(nVars, nEqs, m.clone())?;
            BackendDAEEXT::matching(nVars, nEqs, 5, -1, metamodelica::OrderedFloat(0.0_f64), 1);
            BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
            matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                ass1: ass1.clone(),
                ass2: ass2.clone(),
                comps: compsNew,
            });
            assign_field!(syst.matching = matching);
            (syst, _, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(
                syst,
                openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
                Some(funcTree.clone()),
                BackendDAEUtil::isInitializationDAE(&shared),
            )?;
            (syst, _) = BackendDAETransform::strongComponentsScalar(
                syst,
                shared.clone(),
                mapEqnIncRow.clone(),
                mapIncRowEqn.clone(),
            )?;
            assign_field!(syst.removedEqs = BackendEquation::emptyEqns());
            systsNew = metamodelica::cons(syst.clone(), systsNew);
            vars = BackendVariable::deleteVars(syst.orderedVars.clone(), vars);
            eqs = BackendEquation::deleteList(eqs, &eqIndexLst)?;
        } else {
            if debug {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("No output variables in this system ("));
                    __mm_s.push_str(&*intString(systemNumber));
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*intString(numberOfSystems));
                    __mm_s.push_str(&*literal!(")\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
        der_replacement = UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::hashComponentRef(&__a0)
            })
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::crefEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        for mut state in &*BackendVariable::varList(&vars)? {
            if BackendVariable::isStateVar(metamodelica::AsArg::as_arg(&state)) {
                derVar = BackendVariable::makeVar(ComponentReference::prependStringCref(
                    literal!("$DER_REM_"),
                    &state.varName,
                )?)?;
                UnorderedMap::add(
                    state.varName.clone(),
                    Expression::crefExp(derVar.varName.clone())?,
                    der_replacement.clone(),
                )?;
                vars = BackendVariable::addVar(derVar, vars)?;
            }
        }
        (eqs, _) = BackendEquation::traverseEquationArray_WithUpdate(
            eqs,
            &({
                let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                    (std::sync::Arc::new(replaceDerCallOutputsOnly)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    metamodelica::Ref<
                                        UnorderedMap::UnorderedMap<
                                            metamodelica::Ref<DAE::ComponentRef>,
                                            metamodelica::Ref<DAE::Exp>,
                                        >,
                                    >,
                                ) -> Result<(
                                    metamodelica::Ref<DAE::Exp>,
                                    metamodelica::Ref<
                                        UnorderedMap::UnorderedMap<
                                            metamodelica::Ref<DAE::ComponentRef>,
                                            metamodelica::Ref<DAE::Exp>,
                                        >,
                                    >,
                                )> + 'static,
                        >);
                move |__pe_a0, __pe_a2| BackendEquation::traverseExpsOfEquation(__pe_a0, __pe_b1.clone(), __pe_a2)
            }),
            der_replacement,
        )?;
        (vars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
            vars,
            (std::sync::Arc::new(BackendVariable::makeParamOutputsOnly)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            bool,
                        ) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool)>
                        + 'static,
                >),
            false,
        )?;
        (eqs, _) = BackendEquation::traverseEquationArray_WithUpdate(
            eqs,
            &BackendEquation::setEquationKind,
            openmodelica_backend_types::BackendDAE::EquationKind::INITIAL_EQUATION,
        )?;
        assign_field!(
            shared.globalKnownVars = BackendVariable::addVariables(vars, shared.globalKnownVars.clone())?,
            shared.initialEqs =
                BackendEquation::addList(&(BackendEquation::equationList(eqs)?), shared.initialEqs.clone())?
        );
    }
    assign_field!(shared.aliasVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()));
    daeOut = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systsNew,
        shared: shared,
    });
    Ok(daeOut)
}

fn stateVarIsNotVisited(mut idx: i32, mut varArr: metamodelica::Array<i32>) -> Result<bool> {
    let mut b: bool;
    b = intLt(metamodelica::arrayGet(varArr.clone(), idx)?, 0);
    Ok(b)
}

fn replaceDerCallOutputsOnly(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut der_replacement: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>>,
    >,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>>>,
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut der_replacement: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>>,
    > = der_replacement;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            UnorderedMap::getOrDefault(cr.clone(), der_replacement.clone(), exp)?
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, der_replacement))
}

// =============================================================================
// section for initOptModule >>inlineHomotopy<<
//
// =============================================================================
pub(crate) fn inlineHomotopy(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE.clone();
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut foundHomotopy: bool;
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        orderedEqs = syst.orderedEqs.clone();
        (orderedEqs, foundHomotopy) =
            BackendEquation::traverseEquationArray_WithUpdate(orderedEqs, &inlineHomotopy2, false)?;
        assign_field!(syst.orderedEqs = orderedEqs);
    }
    Ok(outDAE)
}

fn inlineHomotopy2(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inFoundHomotopy: bool,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, bool)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outFoundHomotopy: bool = inFoundHomotopy;
    (outEq, outFoundHomotopy) = BackendEquation::traverseExpsOfEquation(
        inEq,
        (std::sync::Arc::new(inlineHomotopy3)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                    + 'static,
            >),
        inFoundHomotopy,
    )?;
    Ok((outEq, outFoundHomotopy))
}

fn inlineHomotopy3(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFoundHomotopy: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outFoundHomotopy: bool = inFoundHomotopy;
    (outExp, outFoundHomotopy) =
        Expression::traverseExpTopDown(inExp, &replaceHomotopyWithLambdaExpression, inFoundHomotopy)?;
    Ok((outExp, outFoundHomotopy))
}

fn replaceHomotopyWithLambdaExpression(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFoundHomotopy: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut cont: bool = true;
    let mut outFoundHomotopy: bool;
    outFoundHomotopy = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::ListNode::Cons { head: actual, tail: Deref @ metamodelica::ListNode::Cons { head: simplified, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut lambda: metamodelica::Ref<DAE::Exp>;
            lambda = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent(arcstr::literal!(BackendDAE::homotopyLambda), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            outExp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: simplified.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda.clone() }) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: actual.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda }) });
            true
        },
        _ => {
            inFoundHomotopy
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outFoundHomotopy))
}

// =============================================================================
// section for initOptModule >>generateHomotopyComponents<<
//
// =============================================================================
pub(crate) fn generateHomotopyComponents(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE> = inDAE;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut newEqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    if Config::adaptiveHomotopy()? {
        for mut syst in &*outDAE.eqs.clone() {
            let mut syst = syst.clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.matching.clone()) {
                Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ass1 = metamodelica::Own::own(__pa0);
            ass2 = metamodelica::Own::own(__pa1);
            comps = metamodelica::Own::own(__pa2);
            if Config::globalHomotopy()? {
                (comps, syst) = traverseStrongComponentsForHomotopyLoop(comps, syst)?;
            } else {
                (comps, syst) = traverseStrongComponentsAddLambda(comps, syst)?;
            }
            assign_field!(
                syst.matching = metamodelica::Ref::new(BackendDAE::Matching::MATCHING {
                    ass1: ass1.clone(),
                    ass2: ass2.clone(),
                    comps: comps
                })
            );
            newEqSystems = metamodelica::cons(syst, newEqSystems);
        }
        assign_field!(outDAE.eqs = newEqSystems.reverse());
    } else {
        Error::addCompilerWarning(literal!(
            "InitOptModule generateHomotopyComponents is activated for an equidistant homotopy method and will therefore be ignored."
        ))?;
    }
    Ok(outDAE)
}

fn traverseStrongComponentsForHomotopyLoop(
    mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut system: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::Ref<BackendDAE::EqSystem>,
)> {
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = comps;
    let mut system: metamodelica::Ref<BackendDAE::EqSystem> = system;
    let mut nComps: i32;
    let mut compIndex: i32 = 0;
    let mut homotopyLoopBeginning: i32 = 0;
    let mut homotopyLoopEnd: i32 = 0;
    let mut preHomotopyComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut homotopyComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut postHomotopyComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut homotopyComponent: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut lambda: metamodelica::Ref<BackendDAE::Var>;
    let mut lambdaIdx: i32;
    nComps = ((comps).len() as i32);
    for mut comp in &*comps {
        compIndex = compIndex + 1;
        let () = (match &*comp.clone() {
            BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIndex, .. } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut hasHomotopy: bool;
                eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(
                    eqn,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    homotopyLoopEnd = compIndex;
                    if homotopyLoopBeginning == 0 {
                        homotopyLoopBeginning = compIndex;
                    }
                }
                ()
            }
            BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes, .. } => {
                let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut hasHomotopy: bool;
                if homotopyLoopBeginning == 0 {
                    eqnLst = BackendEquation::getList(eqnIndexes.clone(), system.orderedEqs.clone())?;
                    (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(
                        &eqnLst,
                        (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        bool,
                                    )
                                        -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                    + 'static,
                            >),
                        false,
                    )?;
                    if hasHomotopy {
                        homotopyLoopBeginning = compIndex;
                        homotopyLoopEnd = compIndex;
                    }
                } else {
                    homotopyLoopEnd = compIndex;
                }
                ()
            }
            BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIndex, .. } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut hasHomotopy: bool;
                eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(
                    eqn,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    homotopyLoopEnd = compIndex;
                    if homotopyLoopBeginning == 0 {
                        homotopyLoopBeginning = compIndex;
                    }
                }
                ()
            }
            BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIndex, .. } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut hasHomotopy: bool;
                eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(
                    eqn,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    homotopyLoopEnd = compIndex;
                    if homotopyLoopBeginning == 0 {
                        homotopyLoopBeginning = compIndex;
                    }
                }
                ()
            }
            BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIndex, .. } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut hasHomotopy: bool;
                eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(
                    eqn,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    homotopyLoopEnd = compIndex;
                    if homotopyLoopBeginning == 0 {
                        homotopyLoopBeginning = compIndex;
                    }
                }
                ()
            }
            BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIndex, .. } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut hasHomotopy: bool;
                eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(
                    eqn,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    homotopyLoopEnd = compIndex;
                    if homotopyLoopBeginning == 0 {
                        homotopyLoopBeginning = compIndex;
                    }
                }
                ()
            }
            BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIndex, .. } => {
                let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                let mut hasHomotopy: bool;
                eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(
                    eqn,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    homotopyLoopEnd = compIndex;
                    if homotopyLoopBeginning == 0 {
                        homotopyLoopBeginning = compIndex;
                    }
                }
                ()
            }
            BackendDAE::StrongComponent::TORNSYSTEM {
                strictTearingSet:
                    BackendDAE::TearingSet {
                        residualequations: resEqnIndexes,
                        innerEquations,
                        ..
                    },
                ..
            } => {
                let mut innerEqnIndexes: metamodelica::List<i32>;
                let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut hasHomotopy: bool;
                if homotopyLoopBeginning == 0 {
                    eqnLst = BackendEquation::getList(resEqnIndexes.clone(), system.orderedEqs.clone())?;
                    (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(
                        &eqnLst,
                        (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        bool,
                                    )
                                        -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                    + 'static,
                            >),
                        false,
                    )?;
                    if !(hasHomotopy) {
                        (innerEqnIndexes, _, _) = List::map_3(
                            metamodelica::AsArg::as_arg(&innerEquations),
                            &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> {
                                ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0))
                            },
                        )?;
                        eqnLst = BackendEquation::getList(innerEqnIndexes, system.orderedEqs.clone())?;
                        (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(
                            &eqnLst,
                            (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<DAE::Exp>,
                                            bool,
                                        )
                                            -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                        + 'static,
                                >),
                            false,
                        )?;
                    }
                    if hasHomotopy {
                        homotopyLoopBeginning = compIndex;
                        homotopyLoopEnd = compIndex;
                    }
                } else {
                    homotopyLoopEnd = compIndex;
                }
                ()
            }
        });
    }
    if homotopyLoopBeginning > 0 {
        lambda = metamodelica::Ref::new(BackendDAE::Var {
            varName: ComponentReferenceBasics::makeCrefIdent(
                arcstr::literal!(BackendDAE::homotopyLambda),
                DAE::T_REAL_DEFAULT().clone(),
                metamodelica::nil(),
            ),
            varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
            varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            varType: DAE::T_REAL_DEFAULT().clone(),
            bindExp: None,
            tplExp: None,
            arryDim: metamodelica::nil(),
            source: DAE::emptyElementSource().clone(),
            values: None,
            tearingSelectOption: None,
            hideResult: None,
            comment: None,
            connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
            innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
            unreplaceable: true,
            initNonlinear: false,
            encrypted: false,
        });
        assign_field!(system.orderedVars = BackendVariable::addVar(lambda, system.orderedVars.clone())?);
        lambdaIdx = BackendVariable::varsSize(&system.orderedVars);
        (preHomotopyComponents, homotopyComponents, postHomotopyComponents) = getHomotopyComponents(
            &(List::intRange(nComps)),
            &comps,
            homotopyLoopBeginning,
            homotopyLoopEnd,
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
        )?;
        homotopyComponent = createOneHomotopyComponent(&homotopyComponents, &system, lambdaIdx)?;
        comps = metamodelica::cons(homotopyComponent, postHomotopyComponents);
        comps = listAppend(preHomotopyComponents, comps);
    }
    Ok((comps, system))
}

fn getHomotopyComponents<'__b>(
    mut componentIndexes: &'__b metamodelica::List<i32>,
    mut components: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut homotopyLoopBeginning: i32,
    mut homotopyLoopEnd: i32,
    mut outPreHomotopyComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut outHomotopyComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut outPostHomotopyComponents: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (componentIndexes, components) {
            (Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: comp, tail: Deref @ metamodelica::ListNode::Nil }) => {
                if intLt(i.clone(), homotopyLoopBeginning) {
                    outPreHomotopyComponents = metamodelica::cons(comp.clone(), outPreHomotopyComponents);
                } else if intGt(i.clone(), homotopyLoopEnd) {
                    outPostHomotopyComponents = metamodelica::cons(comp.clone(), outPostHomotopyComponents);
                } else {
                    outHomotopyComponents = metamodelica::cons(comp.clone(), outHomotopyComponents);
                }
                return Ok((outPreHomotopyComponents.reverse(), outHomotopyComponents.reverse(), outPostHomotopyComponents.reverse()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: i, tail: indexes }, Deref @ metamodelica::ListNode::Cons { head: comp, tail: comps }) => {
                if intLt(i.clone(), homotopyLoopBeginning) {
                    outPreHomotopyComponents = metamodelica::cons(comp.clone(), outPreHomotopyComponents);
                } else if intGt(i.clone(), homotopyLoopEnd) {
                    outPostHomotopyComponents = metamodelica::cons(comp.clone(), outPostHomotopyComponents);
                } else {
                    outHomotopyComponents = metamodelica::cons(comp.clone(), outHomotopyComponents);
                }
                { (componentIndexes, components, homotopyLoopBeginning, homotopyLoopEnd, outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents) = (indexes, comps, homotopyLoopBeginning, homotopyLoopEnd, outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn createOneHomotopyComponent(
    mut homotopyComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut lambdaIdx: i32,
) -> Result<metamodelica::Ref<BackendDAE::StrongComponent>> {
    let mut outHomotopyComponent: metamodelica::Ref<BackendDAE::StrongComponent>;
    let mut newInnerEquations: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
    let mut newResEquations: metamodelica::List<i32> = metamodelica::nil();
    let mut newIterationVars: metamodelica::List<i32> = metamodelica::nil();
    let mut isMixed: bool = false;
    for mut comp in &**homotopyComponents {
        (newInnerEquations, newResEquations, newIterationVars) = (match &*comp.clone() {
            BackendDAE::StrongComponent::SINGLEEQUATION {
                eqn: eqnIndex,
                var: varIndex,
            } => {
                let mut newInnerEquation: BackendDAE::InnerEquation;
                newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqnIndex.clone(),
                    vars: list![varIndex.clone()],
                };
                (
                    metamodelica::cons(newInnerEquation, newInnerEquations),
                    newResEquations,
                    newIterationVars,
                )
            }
            BackendDAE::StrongComponent::EQUATIONSYSTEM {
                eqns: eqnIndexes,
                vars: varIndexes,
                mixedSystem,
                ..
            } => {
                if mixedSystem.clone() {
                    isMixed = true;
                }
                (
                    newInnerEquations,
                    listAppend(newResEquations, eqnIndexes.clone()),
                    listAppend(newIterationVars, varIndexes.clone()),
                )
            }
            BackendDAE::StrongComponent::SINGLEARRAY {
                eqn: eqnIndex,
                vars: varIndexes,
            } => {
                let mut newInnerEquation: BackendDAE::InnerEquation;
                newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqnIndex.clone(),
                    vars: varIndexes.clone(),
                };
                (
                    metamodelica::cons(newInnerEquation, newInnerEquations),
                    newResEquations,
                    newIterationVars,
                )
            }
            BackendDAE::StrongComponent::SINGLEALGORITHM {
                eqn: eqnIndex,
                vars: varIndexes,
            } => {
                let mut newInnerEquation: BackendDAE::InnerEquation;
                newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqnIndex.clone(),
                    vars: varIndexes.clone(),
                };
                (
                    metamodelica::cons(newInnerEquation, newInnerEquations),
                    newResEquations,
                    newIterationVars,
                )
            }
            BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION {
                eqn: eqnIndex,
                vars: varIndexes,
            } => {
                let mut newInnerEquation: BackendDAE::InnerEquation;
                newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqnIndex.clone(),
                    vars: varIndexes.clone(),
                };
                (
                    metamodelica::cons(newInnerEquation, newInnerEquations),
                    newResEquations,
                    newIterationVars,
                )
            }
            BackendDAE::StrongComponent::SINGLEWHENEQUATION {
                eqn: eqnIndex,
                vars: varIndexes,
            } => {
                let mut newInnerEquation: BackendDAE::InnerEquation;
                newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqnIndex.clone(),
                    vars: varIndexes.clone(),
                };
                (
                    metamodelica::cons(newInnerEquation, newInnerEquations),
                    newResEquations,
                    newIterationVars,
                )
            }
            BackendDAE::StrongComponent::SINGLEIFEQUATION {
                eqn: eqnIndex,
                vars: varIndexes,
            } => {
                let mut newInnerEquation: BackendDAE::InnerEquation;
                newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: eqnIndex.clone(),
                    vars: varIndexes.clone(),
                };
                (
                    metamodelica::cons(newInnerEquation, newInnerEquations),
                    newResEquations,
                    newIterationVars,
                )
            }
            BackendDAE::StrongComponent::TORNSYSTEM {
                strictTearingSet:
                    BackendDAE::TearingSet {
                        residualequations: resEqnIndexes,
                        tearingvars: tVarIndexes,
                        innerEquations,
                        ..
                    },
                mixedSystem,
                ..
            } => {
                if mixedSystem.clone() {
                    isMixed = true;
                }
                for mut innerEquation in &*innerEquations.clone() {
                    newInnerEquations = metamodelica::cons(innerEquation.clone(), newInnerEquations);
                }
                (
                    newInnerEquations,
                    listAppend(newResEquations, resEqnIndexes.clone()),
                    listAppend(newIterationVars, tVarIndexes.clone()),
                )
            }
        });
    }
    outHomotopyComponent = metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
        strictTearingSet: BackendDAE::TearingSet {
            tearingvars: listAppend(newIterationVars, list![lambdaIdx]),
            residualequations: newResEquations,
            innerEquations: newInnerEquations.reverse(),
            jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN(),
        },
        casualTearingSet: None,
        linear: false,
        mixedSystem: isMixed,
    });
    Ok(outHomotopyComponent)
}

fn traverseStrongComponentsAddLambda(
    mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut system: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    metamodelica::Ref<BackendDAE::EqSystem>,
)> {
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = comps;
    let mut system: metamodelica::Ref<BackendDAE::EqSystem> = system;
    let mut newComps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>> = metamodelica::nil();
    let mut lambda: metamodelica::Ref<BackendDAE::Var>;
    let mut lambdaIdx: i32;
    let mut hasAnyHomotopy: bool = false;
    lambdaIdx = BackendVariable::varsSize(&system.orderedVars) + 1;
    for mut comp in &*comps {
        let mut comp = comp.clone();
        comp = (match &*comp.clone() {
            BackendDAE::StrongComponent::EQUATIONSYSTEM {
                eqns: eqnIndexes,
                vars: varIndexes,
                jac,
                jacType,
                mixedSystem,
            } => {
                let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut hasHomotopy: bool;
                eqnLst = BackendEquation::getList(eqnIndexes.clone(), system.orderedEqs.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(
                    &eqnLst,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if hasHomotopy {
                    hasAnyHomotopy = true;
                    comp = metamodelica::Ref::new(BackendDAE::StrongComponent::EQUATIONSYSTEM {
                        eqns: eqnIndexes.clone(),
                        vars: metamodelica::cons(lambdaIdx, varIndexes.clone()),
                        jac: jac.clone(),
                        jacType: jacType.clone(),
                        mixedSystem: mixedSystem.clone(),
                    });
                }
                comp
            }
            BackendDAE::StrongComponent::TORNSYSTEM {
                strictTearingSet:
                    BackendDAE::TearingSet {
                        residualequations: resEqnIndexes,
                        tearingvars: tVarIndexes,
                        innerEquations,
                        jac,
                    },
                casualTearingSet,
                linear,
                mixedSystem,
            } => {
                let mut innerEqnIndexes: metamodelica::List<i32>;
                let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut hasHomotopy: bool;
                eqnLst = BackendEquation::getList(resEqnIndexes.clone(), system.orderedEqs.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(
                    &eqnLst,
                    (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<DAE::Exp>,
                                    bool,
                                )
                                    -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                + 'static,
                        >),
                    false,
                )?;
                if !(hasHomotopy) {
                    (innerEqnIndexes, _, _) = List::map_3(
                        metamodelica::AsArg::as_arg(&innerEquations),
                        &move |__a0: BackendDAE::InnerEquation| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(BackendDAEUtil::getEqnAndVarsFromInnerEquation(&__a0))
                        },
                    )?;
                    eqnLst = BackendEquation::getList(innerEqnIndexes, system.orderedEqs.clone())?;
                    (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(
                        &eqnLst,
                        (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        bool,
                                    )
                                        -> Result<(metamodelica::Ref<DAE::Exp>, bool)>
                                    + 'static,
                            >),
                        false,
                    )?;
                }
                if hasHomotopy {
                    hasAnyHomotopy = true;
                    comp = metamodelica::Ref::new(BackendDAE::StrongComponent::TORNSYSTEM {
                        strictTearingSet: BackendDAE::TearingSet {
                            tearingvars: listAppend(tVarIndexes.clone(), list![lambdaIdx]),
                            residualequations: resEqnIndexes.clone(),
                            innerEquations: innerEquations.clone(),
                            jac: jac.clone(),
                        },
                        casualTearingSet: casualTearingSet.clone(),
                        linear: linear.clone(),
                        mixedSystem: mixedSystem.clone(),
                    });
                }
                comp
            }
            _ => comp,
        });
        newComps = metamodelica::cons(comp, newComps);
    }
    if hasAnyHomotopy {
        lambda = metamodelica::Ref::new(BackendDAE::Var {
            varName: ComponentReferenceBasics::makeCrefIdent(
                arcstr::literal!(BackendDAE::homotopyLambda),
                DAE::T_REAL_DEFAULT().clone(),
                metamodelica::nil(),
            ),
            varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
            varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            varType: DAE::T_REAL_DEFAULT().clone(),
            bindExp: None,
            tplExp: None,
            arryDim: metamodelica::nil(),
            source: DAE::emptyElementSource().clone(),
            values: None,
            tearingSelectOption: None,
            hideResult: None,
            comment: None,
            connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
            innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
            unreplaceable: true,
            initNonlinear: false,
            encrypted: false,
        });
        assign_field!(system.orderedVars = BackendVariable::addVar(lambda, system.orderedVars.clone())?);
    }
    comps = newComps.reverse();
    Ok((comps, system))
}
