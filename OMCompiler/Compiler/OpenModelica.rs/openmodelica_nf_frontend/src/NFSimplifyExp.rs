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

use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub(crate) const MAX_CHAIN_TERMS: i32 = 32;

pub fn simplifyDump(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut includeScope: bool,
    mut name: &ArcStr,
    mut indent: &ArcStr,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    res = simplify(exp.clone(), includeScope)?;
    if Flags::isSet(Flags::DUMP_SIMPLIFY.clone())? && !(Expression::isEqual(exp.clone(), res.clone())?) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("### dumpSimplify | "));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" ###\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("[BEFORE] "));
            __mm_s.push_str(&*Expression::toString(exp)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("[AFTER ] "));
            __mm_s.push_str(&*Expression::toString(res.clone())?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(res)
}

pub fn simplify(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut includeScope: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut old: metamodelica::Ref<Type::NFType>;
    let mut new: metamodelica::Ref<Type::NFType>;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::simplifySubscripts(__exp_cref.clone(), false)?);
            assign_variant_field!(exp => Expression::NFExpression::CREF; ty = ComponentRef::getSubscriptedType(var_field!((*exp).cref, Expression::NFExpression::CREF), includeScope)?);
            exp
        }
        Expression::ARRAY {
            literal: __exp_literal, ..
        } if (!(__exp_literal.clone())) => {
            assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = false; move |__pe_a0| simplify(__pe_a0, __pe_b1.clone()) }))?);
            exp
        }
        Expression::RANGE { .. } => simplifyRange(exp)?,
        Expression::RECORD {
            elements: __exp_elements,
            ..
        } => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut e in (__exp_elements.clone()).into_iter().cloned() {
                    let __x = simplify(e.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            exp
        }
        Expression::CALL { .. } => simplifyCall(exp)?,
        Expression::SIZE { .. } => simplifySize(exp)?,
        Expression::MULTARY { .. } => simplifyMultary(exp)?,
        Expression::BINARY { .. } => simplifyBinary(exp)?,
        Expression::UNARY { .. } => simplifyUnary(exp)?,
        Expression::LBINARY { .. } => simplifyLogicBinary(exp)?,
        Expression::LUNARY { .. } => simplifyLogicUnary(exp)?,
        Expression::RELATION { .. } => simplifyRelation(exp)?,
        Expression::IF { .. } => simplifyIf(exp)?,
        Expression::CAST {
            exp: __exp_exp,
            ty: __exp_ty,
        } => simplifyCast(simplify(__exp_exp.clone(), false)?, __exp_ty.clone())?,
        Expression::UNBOX {
            exp: __exp_exp,
            ty: __exp_ty,
        } => metamodelica::Ref::new(Expression::NFExpression::UNBOX {
            exp: simplify(__exp_exp.clone(), false)?,
            ty: __exp_ty.clone(),
        }),
        Expression::SUBSCRIPTED_EXP { .. } => simplifySubscriptedExp(exp)?,
        Expression::TUPLE_ELEMENT { .. } => simplifyTupleElement(exp)?,
        Expression::RECORD_ELEMENT { .. } => simplifyRecordElement(exp)?,
        Expression::BOX { exp: __exp_exp } => metamodelica::Ref::new(Expression::NFExpression::BOX {
            exp: simplify(__exp_exp.clone(), false)?,
        }),
        Expression::MUTABLE { exp: __exp_exp } => simplify(Mutable::access(__exp_exp.clone()), false)?,
        Expression::INSTANCE_NAME { scope: __exp_scope } => Ceval::evalGetInstanceName(__exp_scope.clone())?,
        _ => exp,
    });
    old = Expression::typeOf(exp.clone());
    new = Type::simplify(old.clone())?;
    if !(referenceEq(&*(old), &*(&*new))) {
        exp = Expression::setType(new, exp)?;
    }
    Ok(exp)
}

pub(crate) fn simplifyRange(
    mut range: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut start_exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut stop_exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut start_exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut stop_exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut step_exp1: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut step_exp2: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { ty: __pa0, start: __pa1, step: __pa2, stop: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    start_exp1 = metamodelica::Own::own(__pa1);
    step_exp1 = metamodelica::Own::own(__pa2);
    stop_exp1 = metamodelica::Own::own(__pa3);
    start_exp2 = simplify(start_exp1.clone(), false)?;
    step_exp2 = Util::applyOption(
        step_exp1.clone(),
        &({
            let __pe_b1 = false;
            move |__pe_a0| simplify(__pe_a0, __pe_b1.clone())
        }),
    )?;
    stop_exp2 = simplify(stop_exp1.clone(), false)?;
    ty2 = Type::simplify(ty.clone())?;
    if referenceEq(&*(start_exp1), &*(&*start_exp2))
        && (match (&(step_exp1), &(step_exp2)) {
            (None, None) => true,
            (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
            _ => false,
        })
        && referenceEq(&*(stop_exp1), &*(&*stop_exp2))
        && referenceEq(&*(&*ty), &*(&*ty2))
    {
        exp = range;
    } else {
        if !(Type::isResizable(ty.clone())?) {
            ty = TypeCheck::keepRangeSize(
                TypeCheck::getRangeType(
                    start_exp2.clone(),
                    step_exp2.clone(),
                    stop_exp2.clone(),
                    Type::arrayElementType(&ty),
                    &(Absyn::dummyInfo.clone()),
                )?,
                ty,
            )?;
        } else {
            ty = ty2;
        }
        exp = metamodelica::Ref::new(Expression::NFExpression::RANGE {
            ty: ty,
            start: start_exp2,
            step: step_exp2,
            stop: stop_exp2,
        });
    }
    Ok(exp)
}

pub(crate) fn simplifyCall(
    mut callExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression> = callExp;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut builtin: bool;
    let mut is_pure: bool;
    let mut scalarize: bool;
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    callExp = (match &*call {
        Call::TYPED_CALL {
            arguments: __esc_args, ..
        } if (!(Call::isExternal(&call)?)) => {
            args = (*__esc_args).clone();
            if Flags::isSet(Flags::NF_EXPAND_FUNC_ARGS.clone())? {
                args = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut arg in (args.clone()).into_iter().cloned() {
                        let __x = if (Expression::hasArrayCall(arg.clone())?) {
                            arg.clone()
                        } else {
                            (ExpandExp::expand(arg.clone(), false, false)?).0
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
            }
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (args.clone()).into_iter().cloned() {
                    let __x = simplify(arg.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = args.clone());
            builtin = Function::isBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL));
            is_pure = !(Function::isImpure(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL)));
            if builtin {
                scalarize = Flags::isSet(Flags::NF_SCALARIZE.clone())?;
                if is_pure
                    && List::all(metamodelica::AsArg::as_arg(&args), &move |__a0: metamodelica::Ref<
                        Expression::NFExpression,
                    >| {
                        Expression::isLiteral(&__a0)
                    })?
                    && (scalarize || Type::isScalar(var_field!((*call).ty, Call::NFCall::TYPED_CALL)))
                {
                    match '__try0: {
                        callExp =
                            unwrap_break_err!(Ceval::evalCall(call.clone(), &(Ceval::noTarget().clone())), '__try0);
                        Ok::<_, &'static str>((callExp.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            callExp = __try0_o0;
                        }
                        Err(_) => {
                            callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() });
                        }
                    }
                } else {
                    callExp = simplifyBuiltinCall(
                        &(Function::nameConsiderBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL))),
                        args.clone(),
                        call.clone(),
                        scalarize,
                    )?;
                }
            } else if Flags::isSet(Flags::NF_EVAL_CONST_ARG_FUNCS.clone())?
                && is_pure
                && List::all(metamodelica::AsArg::as_arg(&args), &move |__a0: metamodelica::Ref<
                    Expression::NFExpression,
                >| {
                    Expression::isLiteral(&__a0)
                })?
            {
                callExp = simplifyCall2(call.clone())?;
            } else {
                callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() });
            }
            callExp
        }
        Call::TYPED_CALL {
            arguments: __esc_args, ..
        } => {
            args = (*__esc_args).clone();
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (args.clone()).into_iter().cloned() {
                    let __x = simplify(arg.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = args.clone());
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() })
        }
        Call::TYPED_ARRAY_CONSTRUCTOR { .. } => simplifyArrayConstructor(&call)?,
        Call::TYPED_REDUCTION { .. } => simplifyReduction(call.clone())?,
        _ => callExp,
    });
    Ok(callExp)
}

pub(crate) fn simplifyCall2(
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    ErrorExt::setCheckpoint(literal!("NFSimplifyExp.simplifyCall2"));
    match '__try0: {
        outExp = unwrap_break_err!(Ceval::evalCall(call.clone(), &(Ceval::noTarget().clone())), '__try0);
        ErrorExt::delCheckpoint(literal!("NFSimplifyExp.simplifyCall2"));
        Ok::<_, &'static str>((outExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExp = __try0_o0;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                ErrorExt::delCheckpoint(literal!("NFSimplifyExp.simplifyCall2"));
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- "));
                    __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyCall2"));
                    __mm_s.push_str(&*literal!(" failed to evaluate "));
                    __mm_s.push_str(&*Call::toString(&call)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                })?;
            } else {
                ErrorExt::rollBack(literal!("NFSimplifyExp.simplifyCall2"));
            }
            outExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() });
        }
    }
    Ok(outExp)
}

pub(crate) fn simplifyBuiltinCall(
    mut name: &metamodelica::Ref<Absyn::Path>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut expand: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(name)) {
        Deref @ "cat" => {
            if !(Flags::getConfigBool(Flags::NEW_BACKEND.clone())?) || List::all(&args, &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isLiteral(&__a0))? {
                (exp, _) = ExpandExp::expandBuiltinCat(&args, call, false)?;
            } else {
                exp = simplifyCat(args, call)?;
            }
            exp
        },
        Deref @ "pre" => (::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_exp @ Deref @ Expression::BOOLEAN { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            exp = (*__esc_exp).clone();
            exp.clone()
        },
        _ => metamodelica::Ref::new(Expression::NFExpression::CALL { call: call }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Deref @ "delay" => simplifyDelay(&args, call)?,
        Deref @ "der" => simplifyDer((args).head().cloned()?, call)?,
        Deref @ "fill" => simplifyFill((args).head().cloned()?, (args).rest()?, call, expand)?,
        Deref @ "homotopy" => simplifyHomotopy(&args, call)?,
        Deref @ "max" => simplifyMinMax(&args, call, false)?,
        Deref @ "min" => simplifyMinMax(&args, call, true)?,
        Deref @ "ones" => simplifyFill(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), args, call, expand)?,
        Deref @ "product" => simplifySumProduct((args).head().cloned()?, call, expand, false)?,
        Deref @ "sum" => simplifySumProduct((args).head().cloned()?, call, expand, true)?,
        Deref @ "transpose" => simplifyTranspose((args).head().cloned()?, call, expand)?,
        Deref @ "vector" => simplifyVector((args).head().cloned()?, call)?,
        Deref @ "zeros" => simplifyFill(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }), args, call, expand)?,
        Deref @ "semiLinear" => simplifySemiLinear(&args, call)?,
        Deref @ "$OMC$PositiveMax" => simplifyPositiveMax(&args, call)?,
        Deref @ "$OMC$inStreamDiv" => simplifyInStreamDiv(args, call, false)?,
        Deref @ "OpenModelica_uriToFilename" => simplifyURIToFilename((args).head().cloned()?, call)?,
        _ => metamodelica::Ref::new(Expression::NFExpression::CALL { call: call }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifyCat(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut nonempty_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (args.clone()).into_iter().cloned() {
            if !(!(Expression::sizeZero(arg.clone())?)) {
                continue;
            }
            let __x = arg.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if ((nonempty_args).len() as i32) == 2 {
        let __pa0 = ::match_deref::match_deref! { match &(nonempty_args) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
    } else if ((nonempty_args).len() as i32) == 1 {
        let __pa2 = ::match_deref::match_deref! { match &(args) {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: _ } } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa2);
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::setArguments(call, nonempty_args)?,
        });
    }
    Ok(exp)
}

pub(crate) fn simplifySemiLinear(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Ref<Expression::NFExpression>;
    let mut m1: metamodelica::Ref<Expression::NFExpression>;
    let mut m2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    x = metamodelica::Own::own(__pa0);
    m1 = metamodelica::Own::own(__pa1);
    m2 = metamodelica::Own::own(__pa2);
    ty = Expression::typeOf(x.clone());
    if Expression::isZero(&x)? || Expression::isZero(&m1)? && Expression::isZero(&m2)? {
        exp = Expression::makeZero(&ty)?;
    } else if Expression::isEqual(m1.clone(), m2)? {
        exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: x,
            operator: Operator::makeMul(ty),
            exp2: m1,
        });
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub(crate) fn simplifyMinMax(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut isMin: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if ((args).len() as i32) == 1 {
        arg = (args).head().cloned()?;
        ty = Expression::typeOf(arg.clone());
        if Type::isEmptyArray(&ty)? {
            ty = Type::arrayElementType(&ty);
            exp = if (isMin) {
                Expression::makeMaxValue(&ty)?
            } else {
                Expression::makeMinValue(&ty)?
            };
        } else {
            exp = simplifyReducedArrayConstructor(&arg, call)?;
        }
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub(crate) fn simplifyPositiveMax(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut eps: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    flow_exp = metamodelica::Own::own(__pa0);
    eps = metamodelica::Own::own(__pa1);
    if Expression::isNonPositive(&flow_exp)? {
        exp = Expression::makeZero(&(Expression::typeOf(flow_exp)))?;
    } else if Expression::isGreaterOrEqual(flow_exp.clone(), eps)? {
        exp = flow_exp;
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub(crate) fn simplifyInStreamDiv(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut removeStream: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stream_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut fallback: metamodelica::Ref<Expression::NFExpression>;
    if ((args).len() as i32) == 2 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        stream_exp = metamodelica::Own::own(__pa0);
        fallback = metamodelica::Own::own(__pa1);
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyInStreamDiv"));
                __mm_s.push_str(&*literal!(
                    " failed because inStreamDiv needs to have exactly two arguments:\n  "
                ));
                __mm_s.push_str(&*List::toString(
                    args,
                    &Expression::toString,
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    if Expression::isNaN(&stream_exp)? {
        exp = fallback;
    } else if removeStream {
        exp = stream_exp;
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub fn removeStream(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::mapReverse(
        exp,
        (std::sync::Arc::new(removeInStreamDiv)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

pub(crate) fn removeInStreamDiv(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest }, .. } } if (metamodelica::stringEq(&(literal!("$OMC$inStreamDiv")), &(AbsynUtil::pathFirstIdent(&(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL))))))) => {
            let mut arg = (*arg).clone();
            arg = simplify(Expression::map(arg.clone(), (std::sync::Arc::new(removePositiveMax) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?, true)?;
            simplifyInStreamDiv(metamodelica::cons(arg.clone(), rest.clone()), call.clone(), true)?
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn removePositiveMax(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: _ }, .. } } if (metamodelica::stringEq(&(literal!("$OMC$PositiveMax")), &(AbsynUtil::pathFirstIdent(&(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL))))))) => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            res = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_REAL().clone(), var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone(), Expression::variability(arg.clone())?, Purity::PURE.clone(), NFBuiltinFuncs::MAX_REAL().returnType.clone()) });
            res
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifySumProduct(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut expand: bool,
    mut isSum: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut ety: metamodelica::Ref<Type::NFType>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    ty = Expression::typeOf(arg.clone());
    if Type::isEmptyArray(&ty)? {
        ety = Type::arrayElementType(&ty);
        exp = if (isSum) {
            Expression::makeZero(&ety)?
        } else {
            Expression::makeOne(&ety)?
        };
    } else if expand {
        (exp, expanded) = ExpandExp::expand(arg.clone(), false, false)?;
        if expanded {
            args = Expression::arrayScalarElements(&exp);
            ety = Type::arrayElementType(&ty);
            if (args).is_empty() {
                exp = if (isSum) {
                    Expression::makeZero(&ety)?
                } else {
                    Expression::makeOne(&ety)?
                };
            } else {
                op = if (isSum) {
                    Operator::makeAdd(ety)
                } else {
                    Operator::makeMul(ety)
                };
                exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: args,
                    inv_arguments: metamodelica::nil(),
                    operator: op,
                });
                exp = simplify(exp, false)?;
            }
        } else {
            exp = simplifyReducedArrayConstructor(&arg, call)?;
        }
    } else {
        exp = simplifyReducedArrayConstructor(&arg, call)?;
    }
    Ok(exp)
}

pub(crate) fn simplifyReducedArrayConstructor(
    mut arg: &metamodelica::Ref<Expression::NFExpression>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match arg {
        Deref @ Expression::CALL { call: arr_call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } if (Type::dimensionCount(var_field!((**arr_call).ty, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()) == 1) => {
            let mut r#fn: metamodelica::Ref<Function::Function>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut var: Variability;
            let mut purity: Purity;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(call) {
                Deref @ Call::TYPED_CALL { r#fn: __pa0, ty: __pa1, var: __pa2, purity: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            r#fn = metamodelica::Own::own(__pa0);
            ty = metamodelica::Own::own(__pa1);
            var = metamodelica::Own::own(__pa2);
            purity = metamodelica::Own::own(__pa3);
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedReduction(r#fn, ty, var, purity, var_field!((**arr_call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**arr_call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), Absyn::dummyInfo.clone())? })
        },
        _ => {
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifyTranspose(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut expand: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    e = if (!(expand) || Expression::hasArrayCall(arg.clone())?) {
        arg
    } else {
        (ExpandExp::expand(arg, false, false)?).0
    };
    exp = (match &*e {
        Expression::ARRAY { .. }
            if (Array::all(
                var_field!((*e).elements, Expression::NFExpression::ARRAY).clone(),
                &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Expression::isArray(&__a0))
                },
            )?) =>
        {
            Expression::transposeArray(&e)?
        }
        _ => metamodelica::Ref::new(Expression::NFExpression::CALL { call: call }),
    });
    Ok(exp)
}

pub(crate) fn simplifyVector(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut is_literal: bool;
    let mut ty: metamodelica::Ref<Type::NFType>;
    expl = Expression::arrayScalarElements(&arg);
    is_literal = Expression::isLiteral(&arg)?;
    if is_literal {
        (expl, _) = ExpandExp::expandList(expl, true)?;
    }
    if is_literal
        || List::all(
            &expl,
            &fnptr!(Expression::isScalar, metamodelica::Ref<Expression::NFExpression>),
        )?
    {
        ty = Type::arrayElementType(&(Expression::typeOf(arg)));
        exp = Expression::makeExpArray(
            metamodelica::arrayFromVec(expl.into_iter().cloned().collect()),
            ty,
            false,
        );
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub(crate) fn simplifyFill(
    mut fillArg: metamodelica::Ref<Expression::NFExpression>,
    mut dimArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut expand: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    if List::all(&dimArgs, &move |__a0: metamodelica::Ref<Expression::NFExpression>| {
        Expression::isLiteral(&__a0)
    })? && expand
    {
        exp = Expression::fillArgs(fillArg, dimArgs)?;
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub(crate) fn simplifyHomotopy(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::REPLACE_HOMOTOPY.clone())?) {
        Deref @ "actual" => (args).head().cloned()?,
        Deref @ "simplified" => (((args).rest()?)).head().cloned()?,
        _ => metamodelica::Ref::new(Expression::NFExpression::CALL { call: call }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifyDelay(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut delayTime: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    delayTime = metamodelica::Own::own(__pa1);
    if Expression::variability(delayTime.clone())? <= Variability::PARAMETER.clone() {
        delayTime = Ceval::tryEvalExp(delayTime, &(Ceval::noTarget().clone()));
        if Expression::isZero(&delayTime)? {
            callExp = exp;
            return Ok(callExp);
        }
    }
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    Ok(callExp)
}

pub(crate) fn simplifyDer(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    if Call::variability(&call)? < Variability::DISCRETE.clone() {
        exp = Expression::makeZero(&(Expression::typeOf(arg)))?;
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(exp)
}

pub(crate) fn simplifyArrayConstructor(
    mut call: &metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut pur: Purity;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = metamodelica::Ref::new(Dimension::BOOLEAN);
    let mut dim_size: i32 = 0;
    let mut expanded: bool = false;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { ty: __pa0, var: __pa1, purity: __pa2, exp: __pa3, iters: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    var = metamodelica::Own::own(__pa1);
    pur = metamodelica::Own::own(__pa2);
    exp = metamodelica::Own::own(__pa3);
    iters = metamodelica::Own::own(__pa4);
    iters = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )> = metamodelica::nil();
        for mut i in (iters).into_iter().cloned() {
            let __x = (Util::tuple21(i.clone()), simplify(Util::tuple22(i.clone()), false)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outExp = 'mc: {
        let __mc_input = &*iters;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (iter, e), tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut e = (*e).clone();
                    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim.clone();
                    let mut dim_size: i32 = dim_size.clone();
                    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                    let mut expanded: bool = expanded.clone();
                    let mut outExp: metamodelica::Ref<Expression::NFExpression> = outExp.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::typeOf(e.clone())) {
                        Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    dim = metamodelica::Own::own(__pa0);
                    dim_size = Dimension::size(&dim, false)?;
                    if dim_size == 0 {
                        outExp = Expression::makeEmptyArray(ty.clone())?;
                    } else if dim_size == 1 {
                        (e, _) = ExpandExp::expand(e.clone(), false, false)?;
                        e = Expression::arrayScalarElement(metamodelica::AsArg::as_arg(&e))?;
                        exp = Expression::replaceIterator(exp.clone(), metamodelica::AsArg::as_arg(&iter), metamodelica::AsArg::as_arg(&e))?;
                        exp = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![exp.clone()].into_iter().cloned().collect()), false);
                        outExp = simplify(exp.clone(), false)?;
                    } else if Expression::isLiteral(metamodelica::AsArg::as_arg(&e))? && isIteratorSubscriptedArray(&exp, metamodelica::AsArg::as_arg(&iter))? {
                        (outExp, expanded) = ExpandExp::expandArrayConstructor(exp.clone(), ty.clone(), &iters)?;
                        if expanded {
                            outExp = simplify(outExp.clone(), false)?;
                        }
                    } else {
                        return Err("fail");
                    }
                    Ok((outExp.clone(), dim.clone(), dim_size.clone(), exp.clone(), expanded.clone(), outExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dim = __wb0;
            dim_size = __wb1;
            exp = __wb2;
            expanded = __wb3;
            outExp = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                    let mut ty: metamodelica::Ref<Type::NFType> = ty.clone();
                    exp = simplify(exp.clone(), false)?;
                    ty = Type::simplify(ty.clone())?;
                    Ok((metamodelica::Ref::new(Expression::NFExpression::CALL { call: metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty.clone(), var: var, purity: pur, exp: exp.clone(), iters: iters.clone() }) }), exp.clone(), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            ty = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub(crate) fn isIteratorSubscriptedArray(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    let mut res: bool;
    res = (match &**exp {
        Expression::SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            Expression::isArray(metamodelica::AsArg::as_arg(&__exp_exp))
                && List::all(
                    metamodelica::AsArg::as_arg(&__exp_subscripts),
                    &({
                        let __pe_b1 = iterator.clone();
                        move |__pe_a0| Subscript::equalsIterator(&__pe_a0, &__pe_b1)
                    }),
                )?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn simplifyReduction(
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    outExp = (match &*call {
        Call::TYPED_REDUCTION {
            iters: __call_iters, ..
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            let mut iter: metamodelica::Ref<InstNode::InstNode>;
            let mut dim: metamodelica::Ref<Dimension::NFDimension> = metamodelica::Ref::new(Dimension::BOOLEAN);
            let mut dim_size: i32 = 0;
            iters = ({
                let mut __acc: metamodelica::List<(
                    metamodelica::Ref<InstNode::InstNode>,
                    metamodelica::Ref<Expression::NFExpression>,
                )> = metamodelica::nil();
                for mut i in (__call_iters.clone()).into_iter().cloned() {
                    let __x = (Util::tuple21(i.clone()), simplify(Util::tuple22(i.clone()), false)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            'mc: {
                let __mc_input = &*iters;
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: (iter, e), tail: Deref @ metamodelica::ListNode::Nil } => {
                            let mut e = (*e).clone();
                            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
                            let mut dim_size: i32;
                            let mut outExp: metamodelica::Ref<Expression::NFExpression> = outExp.clone();
                            let __pa0 = ::match_deref::match_deref! { match &(Expression::typeOf(e.clone())) {
                                Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            dim = metamodelica::Own::own(__pa0);
                            dim_size = Dimension::size(&dim, false)?;
                            if dim_size == 0 {
                                let __pa2 = ::match_deref::match_deref! { match &(var_field!((*call).defaultExp, Call::NFCall::TYPED_REDUCTION).clone()) {
                                    Some(__pa2) => __pa2.clone(),
                                    _ => return Err("pattern mismatch"),
                                } };
                                outExp = metamodelica::Own::own(__pa2);
                            } else if dim_size == 1 {
                                (e, _) = ExpandExp::expand(e.clone(), false, false)?;
                                e = Expression::arrayScalarElement(metamodelica::AsArg::as_arg(&e))?;
                                outExp = Expression::replaceIterator(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), metamodelica::AsArg::as_arg(&iter), metamodelica::AsArg::as_arg(&e))?;
                                outExp = simplify(outExp.clone(), false)?;
                            } else {
                                return Err("fail");
                            }
                            Ok((outExp.clone(), outExp.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    outExp = __wb0;
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            if !((var_field!((*call).var, Call::NFCall::TYPED_REDUCTION).clone() <= Variability::STRUCTURAL_PARAMETER.clone())) { return Err("guard") }
                            Ok(Ceval::tryEvalExp(metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() }), &(Ceval::noTarget().clone())))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            if !((Flags::isSet(Flags::NF_SCALARIZE.clone())?)) { return Err("guard") }
                            Ok(simplifyReduction2(&(AbsynUtil::pathString(Function::name(var_field!((*call).r#fn, Call::NFCall::TYPED_REDUCTION)), literal!("."), true, false)?), var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), &iters)?)
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            let mut call: metamodelica::Ref<Call::NFCall> = call.clone();
                            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION;
                                exp = simplify(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), false)?,
                                iters = iters.clone()
                            );
                            Ok((metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() }), call.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    call = __wb0;
                    break 'mc __v;
                }
                return Err("matchcontinue: no arm matched");
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn simplifyReduction2(
    mut name: &ArcStr,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iterators: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut default_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    ty = Expression::typeOf(exp.clone());
    let false = (Type::isRecord(&(Type::arrayElementType(&ty)))) else {
        return Err("pattern mismatch");
    };
    (default_exp, op) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "sum" => (Expression::makeZero(&ty)?, Operator::makeAdd(ty)),
        Deref @ "product" => (Expression::makeOne(&ty)?, Operator::makeMul(ty)),
        _ => return Err("match: no arm matched"),
    } });
    for mut i in &**iterators {
        (iter, range) = i.clone();
        let __pa0 = ::match_deref::match_deref! { match &(ExpandExp::expand(range, false, false)?) {
            (__pa0, true) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa0);
        iters = metamodelica::cons((iter, range), iters);
    }
    outExp = Expression::foldReduction(
        simplify(exp, false)?,
        &(metamodelica::Dangerous::listReverseInPlace(iters)),
        default_exp,
        &({
            let __pe_b1 = false;
            move |__pe_a0| simplify(__pe_a0, __pe_b1.clone())
        }),
        &({
            let __pe_b1 = op;
            move |__pe_a0, __pe_a2| simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
    )?;
    Ok(outExp)
}

pub(crate) fn simplifySize(
    mut sizeExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut sizeExp: metamodelica::Ref<Expression::NFExpression> = sizeExp;
    sizeExp = (::match_deref::match_deref! { match &(sizeExp.clone()) {
        Deref @ Expression::SIZE { exp, dimIndex: Some(index) } => {
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            let mut exp = (*exp).clone();
            let mut index = (*index).clone();
            index = simplify(index.clone(), false)?;
            if Expression::isLiteral(metamodelica::AsArg::as_arg(&index))? {
                dim = ((Type::arrayDims(Expression::typeOf(exp.clone())))).get(Expression::toInteger(metamodelica::AsArg::as_arg(&index))?)?;
                if Dimension::isKnown(&dim, false) {
                    exp = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: Dimension::size(&dim, false)? });
                } else {
                    exp = metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
                }
            } else {
                exp = metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: exp.clone(), dimIndex: Some(index.clone()) });
            }
            exp.clone()
        },
        Deref @ Expression::SIZE { exp: __sizeExp_exp, .. } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            dims = Type::arrayDims(Expression::typeOf(__sizeExp_exp.clone()));
            if List::all(&dims, &({ let __pe_b1 = true; move |__pe_a0| Ok(Dimension::isKnown(&__pe_a0, __pe_b1.clone())) }))? {
                exp = Expression::makeArray(metamodelica::Ref::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![Dimension::fromInteger(((dims).len() as i32), Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut d in (dims).into_iter().cloned() {
            let __x = Dimension::sizeExp(&(d.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()), false);
            } else {
                exp = sizeExp;
            }
            exp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub(crate) fn simplifyMultary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Nil, inv_arguments: Deref @ metamodelica::ListNode::Nil, operator } if (Operator::isDashClassification(Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?)) => {
            Expression::makeZero(&operator.ty)?
        },
        Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Nil, inv_arguments: Deref @ metamodelica::ListNode::Nil, operator } => {
            Expression::makeOne(&operator.ty)?
        },
        Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Cons { head: tmp, tail: Deref @ metamodelica::ListNode::Nil }, inv_arguments: Deref @ metamodelica::ListNode::Nil, .. } => {
            simplify(tmp.clone(), false)?
        },
        Deref @ Expression::MULTARY { arguments, inv_arguments, operator } => {
            let mut const_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut inv_const_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut new_const: metamodelica::Ref<Expression::NFExpression>;
            let mut tmp: metamodelica::Ref<Expression::NFExpression>;
            let mut result: metamodelica::Ref<Expression::NFExpression>;
            let mut mcl: Operator::MathClassification;
            let mut neutralConst: bool;
            let mut isNegative: bool;
            let mut arguments = (*arguments).clone();
            let mut inv_arguments = (*inv_arguments).clone();
            mcl = Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?;
            arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (arguments.clone()).into_iter().cloned() {
            let __x = simplify(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            inv_arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (inv_arguments.clone()).into_iter().cloned() {
            let __x = simplify(arg.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (arguments, inv_arguments, isNegative) = simplifyMultarySigns(arguments.clone(), inv_arguments.clone(), mcl)?;
            (const_args, arguments) = List::splitOnTrue(metamodelica::AsArg::as_arg(&arguments), &isEvaluableLiteral)?;
            (inv_const_args, inv_arguments) = List::splitOnTrue(metamodelica::AsArg::as_arg(&inv_arguments), &isEvaluableLiteral)?;
            if mcl == Operator::MathClassification::ADDITION.clone() {
                (new_const, neutralConst) = Ceval::evalMultaryAddSub(&const_args, &inv_const_args, Operator::typeOf(metamodelica::AsArg::as_arg(&operator)))?;
            } else if mcl == Operator::MathClassification::MULTIPLICATION.clone() {
                (new_const, neutralConst) = Ceval::evalMultaryMulDiv(&const_args, &inv_const_args, Operator::typeOf(metamodelica::AsArg::as_arg(&operator)))?;
            } else {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyMultary")); __mm_s.push_str(&*literal!(" detected non-commutative operator in MULTARY(): [")); __mm_s.push_str(&*Operator::mathSymbol(mcl)?); __mm_s.push_str(&*literal!("]\n with following arguments: ")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (const_args).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), literal!(", "))); __mm_s.push_str(&*literal!("\n and following inverse arguments: ")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut e in (inv_const_args).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), literal!(", "))); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFSimplifyExp.mo")))?;
                return Err("fail");
            }
            (arguments, inv_arguments) = cancelTermsInMultary(arguments.clone(), inv_arguments.clone())?;
            if neutralConst && !((arguments).is_empty()) && Type::dimensionCount(Expression::typeOf(new_const.clone())) > List::fold(&(listAppend(arguments.clone(), inv_arguments.clone())), &fnptr!(maxDimensionCount, metamodelica::Ref<Expression::NFExpression>, i32), 0)? {
                neutralConst = false;
            }
            result = (::match_deref::match_deref! { match &((mcl, arguments.clone(), inv_arguments.clone())) {
        (Operator::MathClassification::ADDITION, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => if (Expression::isEmpty(&new_const)) {Expression::makeZero(&(Expression::typeOf(new_const.clone())))?} else {new_const.clone()},
        (Operator::MathClassification::MULTIPLICATION, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => if (Expression::isEmpty(&new_const)) {Expression::makeOne(&(Expression::typeOf(new_const.clone())))?} else {new_const.clone()},
        (_, Deref @ metamodelica::ListNode::Cons { head: __esc_tmp, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) if (neutralConst) => {
            tmp = (*__esc_tmp).clone();
            tmp.clone()
        },
        (Operator::MathClassification::ADDITION, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: __esc_tmp, tail: Deref @ metamodelica::ListNode::Nil }) if (neutralConst) => {
            tmp = (*__esc_tmp).clone();
            Expression::negate(tmp.clone())
        },
        (Operator::MathClassification::MULTIPLICATION, _, _) if (Expression::isZero(&new_const)? && !(Type::isArray(&(Operator::typeOf(metamodelica::AsArg::as_arg(&operator)))))) => new_const.clone(),
        (Operator::MathClassification::MULTIPLICATION, _, _) if (Expression::isZero(&new_const)? && Type::hasKnownSize(Operator::typeOf(metamodelica::AsArg::as_arg(&operator)))?) => Expression::makeZero(&(Operator::typeOf(metamodelica::AsArg::as_arg(&operator))))?,
        _ => metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: if (neutralConst) {arguments.clone()} else {metamodelica::cons(new_const.clone(), arguments.clone())}, inv_arguments: inv_arguments.clone(), operator: operator.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if (isNegative) {Expression::negate(result)} else {result}
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyMultary")); __mm_s.push_str(&*literal!(" failed for expression: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifyMultarySigns(
    mut arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut inv_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut mcl: Operator::MathClassification,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    bool,
)> {
    let mut new_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut new_inv_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut isNegative: bool = false;
    let () = (match mcl {
        Operator::MathClassification::ADDITION => {
            for mut arg in &*arguments.reverse() {
                if Expression::isNegated(metamodelica::AsArg::as_arg(&arg)) {
                    new_inv_arguments = metamodelica::cons(Expression::negate(arg.clone()), new_inv_arguments);
                } else {
                    new_arguments = metamodelica::cons(arg.clone(), new_arguments);
                }
            }
            for mut arg in &*inv_arguments.reverse() {
                if Expression::isNegated(metamodelica::AsArg::as_arg(&arg)) {
                    new_arguments = metamodelica::cons(Expression::negate(arg.clone()), new_arguments);
                } else {
                    new_inv_arguments = metamodelica::cons(arg.clone(), new_inv_arguments);
                }
            }
            ()
        }
        Operator::MathClassification::MULTIPLICATION => {
            for mut arg in &*arguments.reverse() {
                if Expression::isNegated(metamodelica::AsArg::as_arg(&arg)) {
                    new_arguments = metamodelica::cons(Expression::negate(arg.clone()), new_arguments);
                    isNegative = !(isNegative);
                } else {
                    new_arguments = metamodelica::cons(arg.clone(), new_arguments);
                }
            }
            for mut arg in &*inv_arguments.reverse() {
                if Expression::isNegated(metamodelica::AsArg::as_arg(&arg)) {
                    new_inv_arguments = metamodelica::cons(Expression::negate(arg.clone()), new_inv_arguments);
                    isNegative = !(isNegative);
                } else {
                    new_inv_arguments = metamodelica::cons(arg.clone(), new_inv_arguments);
                }
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyMultarySigns"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((new_arguments, new_inv_arguments, isNegative))
}

pub(crate) fn simplifyBinary(
    mut binaryExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression> = binaryExp;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut se1: metamodelica::Ref<Expression::NFExpression>;
    let mut se2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    se1 = simplify(e1, false)?;
    se2 = simplify(e2, false)?;
    binaryExp = simplifyBinaryOp(se1, op, se2)?;
    if Flags::isSet(Flags::NF_EXPAND_OPERATIONS.clone())? && !(Expression::hasArrayCall(binaryExp.clone())?) {
        (binaryExp, _) = ExpandExp::expand(binaryExp, false, false)?;
    }
    Ok(binaryExp)
}

pub(crate) fn simplifyBinaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    if Expression::isLiteral(&exp1)? && Expression::isLiteral(&exp2)? {
        outExp = Ceval::evalBinaryOp(
            (ExpandExp::expand(exp1, false, false)?).0,
            op,
            (ExpandExp::expand(exp2, false, false)?).0,
            &(Ceval::noTarget().clone()),
        )?;
    } else if Expression::isArray(&exp1) && Expression::isArray(&exp2) {
        outExp = (match op.op.clone() {
            Operator::Op::ADD => simplifyBinaryEW(&exp1, op, &exp2)?,
            Operator::Op::SUB => simplifyBinaryEW(&exp1, op, &exp2)?,
            Operator::Op::ADD_EW => simplifyBinaryEW(&exp1, op, &exp2)?,
            Operator::Op::SUB_EW => simplifyBinaryEW(&exp1, op, &exp2)?,
            Operator::Op::MUL_EW => simplifyBinaryEW(&exp1, op, &exp2)?,
            Operator::Op::DIV_EW => simplifyBinaryEW(&exp1, op, &exp2)?,
            Operator::Op::POW_EW => simplifyBinaryEW(&exp1, op, &exp2)?,
            _ => metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp1,
                operator: op,
                exp2: exp2,
            }),
        });
    } else {
        outExp = (match op.op.clone() {
            Operator::Op::ADD => simplifyBinaryAdd(exp1.clone(), op, exp2.clone())?,
            Operator::Op::SUB => simplifyBinarySub(exp1.clone(), op, exp2.clone())?,
            Operator::Op::MUL => simplifyBinaryMul(exp1.clone(), &op, exp2.clone(), false)?,
            Operator::Op::DIV => simplifyBinaryDiv(exp1.clone(), op, exp2.clone())?,
            Operator::Op::POW => simplifyBinaryPow(exp1.clone(), op, exp2.clone())?,
            Operator::Op::POW_SCALAR_ARRAY { .. } => simplifyBinaryPow(exp1.clone(), op, exp2.clone())?,
            Operator::Op::POW_ARRAY_SCALAR { .. } => simplifyBinaryPow(exp1.clone(), op, exp2.clone())?,
            Operator::Op::SCALAR_PRODUCT if (Expression::isZero(&exp1)? || Expression::isZero(&exp2)?) => {
                Expression::makeZero(&op.ty)?
            }
            _ => metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp1.clone(),
                operator: op,
                exp2: exp2.clone(),
            }),
        });
    }
    Ok(outExp)
}

pub(crate) fn simplifyBinaryAdd(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    if Expression::isZero(&exp1)? {
        outExp = exp2;
    } else if Expression::isZero(&exp2)? {
        outExp = exp1;
    } else if Expression::isNegated(&exp1) {
        if Expression::isNegated(&exp2) {
            outExp = Expression::negate(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: Expression::negate(exp1),
                operator: op,
                exp2: Expression::negate(exp2),
            }));
        } else {
            outExp = simplifyBinarySub(exp2, Operator::invert(op)?, Expression::negate(exp1))?;
        }
    } else if Expression::isNegated(&exp2) {
        outExp = simplifyBinarySub(exp1, Operator::invert(op)?, Expression::negate(exp2))?;
    } else {
        outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: exp1,
            operator: op,
            exp2: exp2,
        });
    }
    Ok(outExp)
}

pub(crate) fn simplifyBinarySub(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    if Expression::isZero(&exp1)? {
        outExp = Expression::negate(exp2);
    } else if Expression::isZero(&exp2)? {
        outExp = exp1;
    } else if Expression::isEqual(exp1.clone(), exp2.clone())? {
        outExp = Expression::makeZero(&(Operator::typeOf(&op)))?;
    } else if Expression::isNegated(&exp1) {
        if Expression::isNegated(&exp2) {
            outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: Expression::negate(exp2),
                operator: op,
                exp2: Expression::negate(exp1),
            });
        } else {
            outExp = Expression::negate(simplifyBinaryAdd(
                Expression::negate(exp1),
                Operator::invert(op)?,
                exp2,
            )?);
        }
    } else if Expression::isNegated(&exp2) {
        outExp = simplifyBinaryAdd(exp1, Operator::invert(op)?, Expression::negate(exp2))?;
    } else {
        outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: exp1,
            operator: op,
            exp2: exp2,
        });
    }
    Ok(outExp)
}

pub(crate) fn simplifyBinaryMul<'__b>(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: &'__b metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut switched: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    '__tco: loop {
        match &*exp1 {
            Expression::INTEGER { value: 0 } if (!(Type::isArray(&(Operator::typeOf(op))))) => return Ok(exp1),
            Expression::REAL { value: __rlit_0 }
                if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64))
                    && (!(Type::isArray(&(Operator::typeOf(op))))) =>
            {
                return Ok(exp1);
            }
            Expression::INTEGER { value: 0 } if (Type::hasKnownSize(Operator::typeOf(op))?) => {
                return Ok(Expression::makeZero(&(Operator::typeOf(op)))?);
            }
            Expression::REAL { value: __rlit_1 }
                if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64))
                    && (Type::hasKnownSize(Operator::typeOf(op))?) =>
            {
                return Ok(Expression::makeZero(&(Operator::typeOf(op)))?);
            }
            Expression::INTEGER { value: 1 } => return Ok(exp2),
            Expression::REAL { value: __rlit_2 } if __rlit_2.eq(&metamodelica::OrderedFloat((1.0) as f64)) => {
                return Ok(exp2);
            }
            _ => {
                if (switched) {
                    return Ok(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: exp2,
                        operator: op.clone(),
                        exp2: exp1,
                    }));
                } else {
                    {
                        (exp1, op, exp2, switched) = (exp2, op, exp1, true);
                        continue '__tco;
                    }
                }
            }
        }
    }
}

pub(crate) fn simplifyBinaryDiv(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = if (Expression::isOne(&exp2)?) {
        exp1
    } else if (Expression::isMinusOne(&exp2)?) {
        Expression::negate(exp1)
    } else if (Expression::isZero(&exp1)? && Expression::isNonZero(&exp2)?) {
        exp1
    } else {
        (match (Expression::isNegated(&exp1), Expression::isNegated(&exp1)) {
            (true, true) => metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: Expression::negate(exp1),
                operator: op,
                exp2: Expression::negate(exp2),
            }),
            (false, true) => Expression::negate(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp1,
                operator: op,
                exp2: Expression::negate(exp2),
            })),
            (true, false) => Expression::negate(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: Expression::negate(exp1),
                operator: op,
                exp2: exp2,
            })),
            (false, false) => metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp1,
                operator: op,
                exp2: exp2,
            }),
            _ => return Err("match: no arm matched"),
        })
    };
    Ok(outExp)
}

pub(crate) fn simplifyBinaryPow(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    if Expression::isZero(&exp2)? {
        outExp = Expression::makeOne(&(Operator::typeOf(&op)))?;
    } else if Expression::isOne(&exp2)? {
        outExp = exp1;
    } else if Expression::isZero(&exp1)? && Expression::isPositive(&exp2)? {
        outExp = Expression::makeZero(&(Operator::typeOf(&op)))?;
    } else if Expression::isOne(&exp1)? {
        outExp = Expression::makeOne(&(Operator::typeOf(&op)))?;
    } else {
        outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: exp1,
            operator: op,
            exp2: exp2,
        });
    }
    Ok(outExp)
}

pub(crate) fn simplifyBinaryEW(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = Expression::makeArray(
        Operator::typeOf(&op),
        Array::threadMap(
            Expression::arrayElements(exp1)?,
            Expression::arrayElements(exp2)?,
            &({
                let __pe_b1 = Operator::stripEW(Operator::unlift(op)?);
                move |__pe_a0, __pe_a2| simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2)
            }),
        )?,
        false,
    );
    Ok(outExp)
}

pub(crate) fn simplifyUnary(
    mut unaryExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut unaryExp: metamodelica::Ref<Expression::NFExpression> = unaryExp;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut se: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    unaryExp = (::match_deref::match_deref! { match &(unaryExp) {
        Deref @ Expression::UNARY { operator: _, exp: Deref @ Expression::UNARY { operator: _, exp: __esc_e } } => {
            e = (*__esc_e).clone();
            simplify(e.clone(), false)?
        },
        Deref @ Expression::UNARY { operator: __esc_op, exp: __esc_e } => {
            op = (*__esc_op).clone();
            e = (*__esc_e).clone();
            se = simplify(e.clone(), false)?;
            simplifyUnaryOp(se, op.clone())?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSimplifyExp.simplifyUnary")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::NF_EXPAND_OPERATIONS.clone())? && !(Expression::hasArrayCall(unaryExp.clone())?) {
        (unaryExp, _) = ExpandExp::expand(unaryExp, false, false)?;
    }
    Ok(unaryExp)
}

pub(crate) fn simplifyUnaryOp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    if Expression::isLiteral(&exp)? {
        outExp = Ceval::evalUnaryOp(exp, op)?;
    } else {
        outExp = simplifyUnarySign(exp, true);
    }
    Ok(outExp)
}

pub(crate) fn simplifyUnarySign(
    mut unaryExp: metamodelica::Ref<Expression::NFExpression>,
    mut isNegative: bool,
) -> metamodelica::Ref<Expression::NFExpression> {
    '__tco: loop {
        match &*unaryExp {
            Expression::UNARY {
                exp: __unaryExp_exp, ..
            } => {
                (unaryExp, isNegative) = (__unaryExp_exp.clone(), !(isNegative));
                continue '__tco;
            }
            _ => {
                if (isNegative) {
                    return Expression::negate(unaryExp);
                } else {
                    return unaryExp;
                }
            }
        }
    }
}

pub(crate) fn simplifyLogicBinary(
    mut binaryExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression> = binaryExp;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut se1: metamodelica::Ref<Expression::NFExpression>;
    let mut se2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp) {
        Deref @ Expression::LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    se1 = simplify(e1, false)?;
    se2 = simplify(e2, false)?;
    binaryExp = (match op.op.clone() {
        Operator::Op::AND => simplifyLogicBinaryAnd(se1, op, se2)?,
        Operator::Op::OR => simplifyLogicBinaryOr(se1, op, se2)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(binaryExp)
}

pub(crate) fn simplifyLogicBinaryAnd(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::BOOLEAN { value: false }, _) => {
            exp1
        },
        (_, Deref @ Expression::BOOLEAN { value: false }) => {
            exp2
        },
        (Deref @ Expression::BOOLEAN { value: true }, _) => {
            exp2
        },
        (_, Deref @ Expression::BOOLEAN { value: true }) => {
            exp1
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) => {
            let mut o: metamodelica::Ref<Operator::NFOperator>;
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            o = Operator::unlift(op.clone())?;
            arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = o; move |__pe_a0, __pe_a2| simplifyLogicBinaryAnd(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
            Expression::makeArray(Operator::typeOf(&op), arr.clone(), false)
        },
        _ => {
            metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: exp1, operator: op, exp2: exp2 })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifyLogicBinaryOr(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::BOOLEAN { value: true }, _) => {
            exp1
        },
        (_, Deref @ Expression::BOOLEAN { value: true }) => {
            exp2
        },
        (Deref @ Expression::BOOLEAN { value: false }, _) => {
            exp2
        },
        (_, Deref @ Expression::BOOLEAN { value: false }) => {
            exp1
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) => {
            let mut o: metamodelica::Ref<Operator::NFOperator>;
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            o = Operator::unlift(op.clone())?;
            arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = o; move |__pe_a0, __pe_a2| simplifyLogicBinaryOr(__pe_a0, __pe_b1.clone(), __pe_a2) }))?;
            Expression::makeArray(Operator::typeOf(&op), arr.clone(), false)
        },
        _ => {
            metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: exp1, operator: op, exp2: exp2 })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn simplifyLogicUnary(
    mut unaryExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut unaryExp: metamodelica::Ref<Expression::NFExpression> = unaryExp;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut se: metamodelica::Ref<Expression::NFExpression>;
    let mut newExp: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    unaryExp = (::match_deref::match_deref! { match &(unaryExp.clone()) {
        Deref @ Expression::LUNARY { operator: _, exp: Deref @ Expression::LUNARY { operator: _, exp: __esc_e } } => {
            e = (*__esc_e).clone();
            simplify(e.clone(), false)?
        },
        Deref @ Expression::LUNARY { operator: __esc_op, exp: __esc_e } => {
            op = (*__esc_op).clone();
            e = (*__esc_e).clone();
            se = simplify(e.clone(), false)?;
            if Expression::isLiteral(&se)? {
                newExp = Ceval::evalLogicUnaryOp(se, op.clone())?;
            } else if !(referenceEq(&*(e.clone()),&*(&*se))) {
                newExp = metamodelica::Ref::new(Expression::NFExpression::LUNARY { operator: op.clone(), exp: se });
            } else {
                newExp = unaryExp;
            }
            newExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(unaryExp)
}

pub(crate) fn simplifyRelation(
    mut relationExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut relationExp: metamodelica::Ref<Expression::NFExpression> = relationExp;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut se1: metamodelica::Ref<Expression::NFExpression>;
    let mut se2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let mut index: i32;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(relationExp.clone()) {
        Deref @ Expression::RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, index: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    index = metamodelica::Own::own(__pa3);
    se1 = simplify(e1.clone(), false)?;
    se2 = simplify(e2.clone(), false)?;
    if Expression::isLiteral(&se1)? && Expression::isLiteral(&se2)? {
        relationExp = Ceval::evalRelationOp(se1, op, se2)?;
    } else if !(referenceEq(&*(e1), &*(&*se1)) && referenceEq(&*(e2), &*(&*se2))) {
        relationExp = metamodelica::Ref::new(Expression::NFExpression::RELATION {
            exp1: se1,
            operator: op,
            exp2: se2,
            index: index,
        });
    }
    Ok(relationExp)
}

pub(crate) fn simplifyIf(
    mut ifExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut ifExp: metamodelica::Ref<Expression::NFExpression> = ifExp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut tb: metamodelica::Ref<Expression::NFExpression>;
    let mut fb: metamodelica::Ref<Expression::NFExpression>;
    let mut tb_val: bool;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    tb = metamodelica::Own::own(__pa2);
    fb = metamodelica::Own::own(__pa3);
    cond = simplify(cond, false)?;
    ifExp = (match &*cond {
        Expression::BOOLEAN { value: __cond_value } => simplify(if (__cond_value.clone()) { tb } else { fb }, false)?,
        _ => {
            tb = simplify(tb, false)?;
            fb = simplify(fb, false)?;
            if Expression::isEqual(tb.clone(), fb.clone())? {
                ifExp = tb;
            } else if Expression::isBoolean(&tb) && Expression::isBoolean(&fb) {
                let __pa0 = ::match_deref::match_deref! { match &(tb) {
                    Deref @ Expression::BOOLEAN { value: __pa0 } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                tb_val = metamodelica::Own::own(__pa0);
                ifExp = if (tb_val) { cond } else { Expression::logicNegate(cond) };
            } else {
                ty = if (Type::isConditionalArray(&ty)) {
                    Type::setConditionalArrayTypes(&ty, Expression::typeOf(tb.clone()), Expression::typeOf(fb.clone()))?
                } else {
                    Expression::typeOf(tb.clone())
                };
                ifExp = metamodelica::Ref::new(Expression::NFExpression::IF {
                    ty: ty,
                    condition: cond,
                    trueBranch: tb,
                    falseBranch: fb,
                });
            }
            ifExp
        }
    });
    Ok(ifExp)
}

pub(crate) fn simplifyCast(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut castExp: metamodelica::Ref<Expression::NFExpression>;
    castExp = (::match_deref::match_deref! { match &((ty.clone(), exp.clone())) {
        (Deref @ Type::REAL, Deref @ Expression::INTEGER { .. }) => {
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: intReal(var_field!((*exp).value, Expression::NFExpression::INTEGER).clone()) })
        },
        (Deref @ Type::ARRAY { elementType: Deref @ Type::REAL, .. }, Deref @ Expression::ARRAY { .. }) => {
            let mut ety: metamodelica::Ref<Type::NFType>;
            ety = Type::unliftArray(ty.clone())?;
            assign_variant_field!(exp => Expression::NFExpression::ARRAY;
                elements = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = ety; move |__pe_a0| simplifyCast(__pe_a0, __pe_b1.clone()) }))?,
                ty = Type::setArrayElementType(var_field!((*exp).ty, Expression::NFExpression::ARRAY), &(Type::arrayElementType(&ty)))
            );
            exp
        },
        _ => {
            metamodelica::Ref::new(Expression::NFExpression::CAST { ty: ty, exp: exp })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(castExp)
}

pub(crate) fn isEvaluableLiteral(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool = Expression::isLiteral(&exp)?
        && !(Type::isComplex(&(Type::arrayElementType(&(Expression::typeOf(exp.clone()))))));
    Ok(b)
}

pub(crate) fn maxDimensionCount(mut exp: metamodelica::Ref<Expression::NFExpression>, mut count: i32) -> i32 {
    let mut count: i32 = count;
    count = std::cmp::max(count, Type::dimensionCount(Expression::typeOf(exp)));
    count
}

pub(crate) fn simplifySubscriptedExp(
    mut subscriptedExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut subscriptedExp: metamodelica::Ref<Expression::NFExpression> = subscriptedExp;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut split: bool;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(subscriptedExp) {
        Deref @ Expression::SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, ty: __pa2, split: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    subs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    split = metamodelica::Own::own(__pa3);
    subscriptedExp = simplify(e.clone(), false)?;
    subs = Subscript::simplifyList(subs, Type::arrayDims(Expression::typeOf(e)), false)?;
    if !(split)
        && !(List::all(&subs, &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| {
            Subscript::isLiteral(&__a0)
        })?)
        && Type::isScalar(&ty)
    {
        while !((subs).is_empty())
            && Expression::isArray(&subscriptedExp)
            && !(Expression::isEmptyArray(&subscriptedExp))
            && Array::allEqual(Expression::arrayElements(&subscriptedExp)?, &Expression::isEqual)?
        {
            subs = (subs).rest()?;
            subscriptedExp = metamodelica::arrayGet(Expression::arrayElements(&subscriptedExp)?, 1)?;
        }
        if (subs).is_empty() {
            return Ok(subscriptedExp);
        }
    }
    if split {
        subscriptedExp = metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP {
            exp: subscriptedExp,
            subscripts: subs,
            ty: ty,
            split: split,
        });
    } else {
        subscriptedExp = Expression::applySubscripts(&subs, subscriptedExp, false)?;
    }
    Ok(subscriptedExp)
}

pub(crate) fn simplifyTupleElement(
    mut tupleExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut tupleExp: metamodelica::Ref<Expression::NFExpression> = tupleExp;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut index: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tupleExp) {
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    index = metamodelica::Own::own(__pa1);
    e = simplify(e, false)?;
    tupleExp = Expression::tupleElement(e, index)?;
    Ok(tupleExp)
}

pub(crate) fn simplifyRecordElement(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut idx: i32;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: __pa0, index: __pa1, fieldName: _, ty: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    idx = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    e2 = simplify(e.clone(), false)?;
    if !(referenceEq(&*(e), &*(&*e2))) {
        exp = Expression::nthRecordElement(idx, &e2)?;
    }
    Ok(exp)
}

pub(crate) fn combineConstantNumbers(
    mut r#const: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut inv_const: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut mcl: Operator::MathClassification,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut tmp: metamodelica::Real;
    let mut result: metamodelica::Real;
    res = (match mcl {
        Operator::MathClassification::ADDITION => {
            result = metamodelica::OrderedFloat(0.0_f64);
            for mut exp in &*r#const {
                tmp = getConstantValue(exp.clone())?;
                result = result + tmp;
            }
            for mut exp in &*inv_const {
                tmp = getConstantValue(exp.clone())?;
                result = result - tmp;
            }
            res = if (Type::isInteger(&ty)?) {
                metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                    value: ((result).0.floor() as i32),
                })
            } else {
                metamodelica::Ref::new(Expression::NFExpression::REAL { value: result })
            };
            res
        }
        Operator::MathClassification::MULTIPLICATION => {
            result = metamodelica::OrderedFloat(1.0_f64);
            for mut exp in &*r#const {
                tmp = getConstantValue(exp.clone())?;
                result = result * tmp;
            }
            if result == metamodelica::OrderedFloat(0.0_f64) {
                if List::any(&inv_const, &move |__a0: metamodelica::Ref<Expression::NFExpression>| {
                    Expression::isZero(&__a0)
                })? {
                    res = Expression::makeNaN(ty)?;
                } else {
                    res = Expression::makeZero(&ty)?;
                }
            } else {
                for mut exp in &*inv_const {
                    tmp = getConstantValue(exp.clone())?;
                    result = metamodelica::real_div_checked(result, tmp)?;
                }
                res = if (Type::isInteger(&ty)?) {
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: ((result).0.floor() as i32),
                    })
                } else {
                    metamodelica::Ref::new(Expression::NFExpression::REAL { value: result })
                };
            }
            res
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSimplifyExp.combineConstantNumbers"));
                    __mm_s.push_str(&*literal!(" detected non-commutative operator in MULTARY(): ["));
                    __mm_s.push_str(&*Operator::mathSymbol(mcl)?);
                    __mm_s.push_str(&*literal!("]\n with following arguments: "));
                    __mm_s.push_str(&*stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut e in (r#const).into_iter().cloned() {
                                let __x = Expression::toString(e.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!("\n and following inverse arguments: "));
                    __mm_s.push_str(&*stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut e in (inv_const).into_iter().cloned() {
                                let __x = Expression::toString(e.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(", "),
                    ));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSimplifyExp.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(res)
}

fn getConstantValue(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    if let Ok(__iflet0) = Expression::realValue(&(Ceval::evalExp(exp.clone(), &(Ceval::noTarget().clone()))?)) {
        value = __iflet0;
    } else {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFSimplifyExp.getConstantValue"));
                __mm_s.push_str(&*literal!(" expression is not known to be a constant number: "));
                __mm_s.push_str(&*Expression::toString(exp.clone())?);
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("NFFrontEnd/NFSimplifyExp.mo"),
        )?;
        return Err("fail");
    }
    Ok(value)
}

fn cancelTermsInMultary(
    mut inArguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut inInv_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
)> {
    fn inc(mut oldValue: Option<i32>, mut step: i32) -> i32 {
        let mut value: i32;
        value = (match oldValue {
            Some(mut __esc_value) => {
                value = __esc_value.clone();
                value + step
            }
            _ => step,
        });
        value
    }

    let mut outArguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut outInv_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut counter: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Expression::NFExpression>, i32>>;
    let mut arg: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut count: i32;
    if (inArguments).is_empty() || (inInv_arguments).is_empty() {
        outArguments = inArguments;
        outInv_arguments = inInv_arguments;
        return Ok((outArguments, outInv_arguments));
    }
    counter = UnorderedMap::new(
        (std::sync::Arc::new(Expression::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(Expression::isEqual)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    for mut arg in &*inArguments {
        let mut arg = arg.clone();
        UnorderedMap::addUpdate(
            arg,
            &({
                let __pe_b1 = 1;
                move |__pe_a0| Ok(inc(__pe_a0, __pe_b1.clone()))
            }),
            counter.clone(),
        )?;
    }
    for mut arg in &*inInv_arguments {
        let mut arg = arg.clone();
        UnorderedMap::addUpdate(
            arg,
            &({
                let __pe_b1 = -1;
                move |__pe_a0| Ok(inc(__pe_a0, __pe_b1.clone()))
            }),
            counter.clone(),
        )?;
    }
    for mut tpl in &*UnorderedMap::toList(counter) {
        (arg, count) = tpl.clone();
        if count > 0 {
            for mut i in 1..=count {
                outArguments = metamodelica::cons(arg.clone(), outArguments);
            }
        } else if count < 0 {
            for mut i in 1..=-(count) {
                outInv_arguments = metamodelica::cons(arg.clone(), outInv_arguments);
            }
        }
    }
    outArguments = metamodelica::Dangerous::listReverseInPlace(outArguments);
    outInv_arguments = metamodelica::Dangerous::listReverseInPlace(outInv_arguments);
    Ok((outArguments, outInv_arguments))
}

pub fn combineBinaries(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(removeTrivialScalarProduct)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    exp = combineBinariesExp(
        exp.clone(),
        None,
        metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: Expression::typeOf(exp.clone()),
        }),
        false,
    )?;
    Ok(exp)
}

pub(crate) fn splitMultary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            operator: __exp_operator,
        } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut rhs: metamodelica::Ref<Expression::NFExpression>;
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut inv_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut inv_op: metamodelica::Ref<Operator::NFOperator>;
            let mut is_add: bool;
            args = __exp_arguments.clone();
            inv_args = __exp_inv_arguments.clone();
            inv_op = Operator::invert(__exp_operator.clone())?;
            is_add = Operator::getMathClassification(metamodelica::AsArg::as_arg(&__exp_operator))?
                == Operator::MathClassification::ADDITION.clone();
            if (args).is_empty() {
                if (inv_args).is_empty() {
                    new_exp = if (is_add) {
                        Expression::makeZero(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))))?
                    } else {
                        Expression::makeOne(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))))?
                    };
                } else if is_add {
                    new_exp = Expression::negate(chainBinaries(inv_args, __exp_operator.clone())?);
                    inv_args = metamodelica::nil();
                } else {
                    new_exp = Expression::makeOne(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))))?;
                }
            } else {
                new_exp = chainBinaries(args, __exp_operator.clone())?;
            }
            if ((inv_args).len() as i32) > MAX_CHAIN_TERMS.clone() {
                rhs = chainBinaries(inv_args, __exp_operator.clone())?;
                new_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: new_exp.clone(),
                    operator: Operator::repairBinary(
                        inv_op,
                        Expression::typeOf(new_exp),
                        Expression::typeOf(rhs.clone()),
                    )?,
                    exp2: rhs,
                });
            } else {
                for mut arg in &*inv_args {
                    new_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: new_exp.clone(),
                        operator: Operator::repairBinary(
                            inv_op.clone(),
                            Expression::typeOf(new_exp),
                            Expression::typeOf(arg.clone()),
                        )?,
                        exp2: arg.clone(),
                    });
                }
            }
            new_exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn chainBinaries(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut level: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = args.clone();
    let mut next: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut fixed_op: metamodelica::Ref<Operator::NFOperator>;
    if ((args).len() as i32) <= MAX_CHAIN_TERMS.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        level = metamodelica::Own::own(__pa1);
        for mut e in &*level {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp.clone(),
                operator: Operator::repairBinary(op.clone(), Expression::typeOf(exp), Expression::typeOf(e.clone()))?,
                exp2: e.clone(),
            });
        }
        return Ok(exp);
    }
    while !((level).is_empty()) && !(((level).rest()?).is_empty()) {
        next = metamodelica::nil();
        while !((level).is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(level) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa2);
            level = metamodelica::Own::own(__pa3);
            if (level).is_empty() {
                next = metamodelica::cons(e1, next);
            } else {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(level) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e2 = metamodelica::Own::own(__pa4);
                level = metamodelica::Own::own(__pa5);
                fixed_op = Operator::repairBinary(
                    op.clone(),
                    Expression::typeOf(e1.clone()),
                    Expression::typeOf(e2.clone()),
                )?;
                next = metamodelica::cons(
                    metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: e1,
                        operator: fixed_op,
                        exp2: e2,
                    }),
                    next,
                );
            }
        }
        level = metamodelica::Dangerous::listReverseInPlace(next);
    }
    exp = (level).head().cloned()?;
    Ok(exp)
}

fn combineBinariesExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut optOperator: Option<metamodelica::Ref<Operator::NFOperator>>,
    mut result: metamodelica::Ref<Expression::NFExpression>,
    mut inverse: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((optOperator, exp.clone())) {
            (Some(op), Deref @ Expression::BINARY { .. }) if (Operator::compare(metamodelica::AsArg::as_arg(&op), var_field!((*exp).operator, Expression::NFExpression::BINARY)) == 0) => {
                result = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result, inverse)?;
                { (exp, optOperator, result, inverse) = (var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result, inverse); continue '__tco; }
            },
            (Some(op), Deref @ Expression::MULTARY { .. }) if (Operator::compare(metamodelica::AsArg::as_arg(&op), var_field!((*exp).operator, Expression::NFExpression::MULTARY)) == 0) => {
                for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                    result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result, inverse)?;
                }
                for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                    result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result, !(inverse))?;
                }
                return Ok(result)
            },
            (Some(op), Deref @ Expression::BINARY { .. }) if (Operator::isCombineable(metamodelica::AsArg::as_arg(&op), var_field!((*exp).operator, Expression::NFExpression::BINARY))?) => {
                result = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result, inverse)?;
                { (exp, optOperator, result, inverse) = (var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(op.clone()), result, !(inverse)); continue '__tco; }
            },
            (Some(op), Deref @ Expression::MULTARY { .. }) if (Operator::isCombineable(metamodelica::AsArg::as_arg(&op), var_field!((*exp).operator, Expression::NFExpression::MULTARY))?) => {
                for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                    result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result, inverse)?;
                }
                for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                    result = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), result, !(inverse))?;
                }
                return Ok(result)
            },
            (_, Deref @ Expression::BINARY { .. }) if (Operator::isCommutative(var_field!((*exp).operator, Expression::NFExpression::BINARY))) => {
                let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
                new_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: metamodelica::nil(), operator: var_field!((*exp).operator, Expression::NFExpression::BINARY).clone() });
                new_exp = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone()), new_exp, false)?;
                new_exp = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone()), new_exp, false)?;
                return Ok(addArgument(result, new_exp, inverse)?)
            },
            (_, Deref @ Expression::MULTARY { .. }) if (Operator::isCommutative(var_field!((*exp).operator, Expression::NFExpression::MULTARY))) => {
                let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
                new_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: metamodelica::nil(), operator: var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone() });
                for mut arg in &*var_field!((*exp).arguments, Expression::NFExpression::MULTARY).clone() {
                    new_exp = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), new_exp, false)?;
                }
                for mut arg in &*var_field!((*exp).inv_arguments, Expression::NFExpression::MULTARY).clone() {
                    new_exp = combineBinariesExp(arg.clone(), Some(var_field!((*exp).operator, Expression::NFExpression::MULTARY).clone()), new_exp, true)?;
                }
                return Ok(addArgument(result, new_exp, inverse)?)
            },
            (_, Deref @ Expression::BINARY { .. }) if (Operator::isSoftCommutative(var_field!((*exp).operator, Expression::NFExpression::BINARY))) => {
                let mut op: metamodelica::Ref<Operator::NFOperator>;
                let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
                op = Operator::invert(var_field!((*exp).operator, Expression::NFExpression::BINARY).clone())?;
                new_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: metamodelica::nil(), inv_arguments: metamodelica::nil(), operator: op.clone() });
                new_exp = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), Some(op.clone()), new_exp, false)?;
                new_exp = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), Some(op.clone()), new_exp, true)?;
                return Ok(addArgument(result, new_exp, inverse)?)
            },
            (_, Deref @ Expression::CREF { cref: cref @ Deref @ ComponentRef::CREF { .. }, .. }) => {
                let mut cref = (*cref).clone();
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut sub in (var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone()).into_iter().cloned() {
                let __x = combineBinariesSubscript(sub.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                assign_variant_field!(exp => Expression::NFExpression::CREF; cref = cref.clone());
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::ARRAY { .. }) => {
                if !(var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone()) {
                    assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = None; let __pe_b2 = metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(exp.clone()) }); let __pe_b3 = false; move |__pe_a0| combineBinariesExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?);
                }
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::RANGE { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::RANGE;
                    start = combineBinariesExp(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).start, Expression::NFExpression::RANGE).clone()) }), false)?,
                    stop = combineBinariesExp(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone()) }), false)?
                );
                if (var_field!((*exp).step, Expression::NFExpression::RANGE)).is_some() {
                    assign_variant_field!(exp => Expression::NFExpression::RANGE; step = Some(combineBinariesExp(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?, None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(Util::getOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone())?) }), false)?));
                }
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::TUPLE { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut element in (var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()).into_iter().cloned() {
                let __x = combineBinariesExp(element.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(element.clone()) }), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::RECORD { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut element in (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
                let __x = combineBinariesExp(element.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(element.clone()) }), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } }) => {
                let mut call = (*call).clone();
                assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
                let __x = combineBinariesExp(arg.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(arg.clone()) }), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::SIZE { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::SIZE; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone()) }), false)?);
                if (var_field!((*exp).dimIndex, Expression::NFExpression::SIZE)).is_some() {
                    assign_variant_field!(exp => Expression::NFExpression::SIZE; dimIndex = Some(combineBinariesExp(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?) }), false)?));
                }
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::UNARY { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::UNARY; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::LBINARY { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::LBINARY;
                    exp1 = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone()) }), false)?,
                    exp2 = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone()) }), false)?
                );
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::LUNARY { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::LUNARY; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::RELATION { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::RELATION;
                    exp1 = combineBinariesExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone()) }), false)?,
                    exp2 = combineBinariesExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone()) }), false)?
                );
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::IF { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::IF;
                    condition = combineBinariesExp(var_field!((*exp).condition, Expression::NFExpression::IF).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).condition, Expression::NFExpression::IF).clone()) }), false)?,
                    trueBranch = combineBinariesExp(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).trueBranch, Expression::NFExpression::IF).clone()) }), false)?,
                    falseBranch = combineBinariesExp(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone()) }), false)?
                );
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::CAST { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::CAST; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::CAST).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::BOX { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::BOX; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::BOX).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::UNBOX { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::UNBOX; exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::SUBSCRIPTED_EXP { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::SUBSCRIPTED_EXP;
                    exp = combineBinariesExp(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) }), false)?,
                    subscripts = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut sub in (var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
                let __x = combineBinariesSubscript(sub.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
                );
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::TUPLE_ELEMENT { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::TUPLE_ELEMENT; tupleExp = combineBinariesExp(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::RECORD_ELEMENT { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::RECORD_ELEMENT; recordExp = combineBinariesExp(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(var_field!((*exp).recordExp, Expression::NFExpression::RECORD_ELEMENT).clone()) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::MUTABLE { .. }) => {
                Mutable::update(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone(), combineBinariesExp(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone())) }), false)?);
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            (_, Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { .. }) => {
                assign_variant_field!(exp => Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut arg in (var_field!((*exp).args, Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION).clone()).into_iter().cloned() {
                let __x = combineBinariesExp(arg.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(arg.clone()) }), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            _ => {
                return Ok(addArgument(result, exp.clone(), inverse)?)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn combineBinariesSubscript(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
) -> Result<metamodelica::Ref<Subscript::NFSubscript>> {
    let mut subscript: metamodelica::Ref<Subscript::NFSubscript> = subscript;
    subscript = (match &*subscript {
        Subscript::UNTYPED { exp: __subscript_exp } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::UNTYPED; exp = combineBinariesExp(__subscript_exp.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(__subscript_exp.clone()) }), false)?);
            subscript
        }
        Subscript::INDEX {
            index: __subscript_index,
        } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::INDEX; index = combineBinariesExp(__subscript_index.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(__subscript_index.clone()) }), false)?);
            subscript
        }
        Subscript::SLICE {
            slice: __subscript_slice,
        } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::SLICE; slice = combineBinariesExp(__subscript_slice.clone(), None, metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: Expression::typeOf(__subscript_slice.clone()) }), false)?);
            subscript
        }
        Subscript::EXPANDED_SLICE {
            indices: __subscript_indices,
        } => {
            assign_variant_field!(subscript => Subscript::NFSubscript::EXPANDED_SLICE; indices = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut sub in (__subscript_indices.clone()).into_iter().cloned() {
                    let __x = combineBinariesSubscript(sub.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            subscript
        }
        _ => subscript,
    });
    Ok(subscript)
}

fn addArgument(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut inverse: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::MULTARY {
            inv_arguments: __exp_inv_arguments,
            ..
        } if (inverse) => {
            assign_variant_field!(exp => Expression::NFExpression::MULTARY; inv_arguments = metamodelica::cons(arg, __exp_inv_arguments.clone()));
            exp
        }
        Expression::MULTARY {
            arguments: __exp_arguments,
            ..
        } => {
            assign_variant_field!(exp => Expression::NFExpression::MULTARY; arguments = metamodelica::cons(arg, __exp_arguments.clone()));
            exp
        }
        Expression::EMPTY { .. } => arg,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSimplifyExp.addArgument"));
                    __mm_s.push_str(&*literal!(" failed to add : "));
                    __mm_s.push_str(&*Expression::toString(arg)?);
                    __mm_s.push_str(&*literal!(" to "));
                    __mm_s.push_str(&*Expression::toString(exp)?);
                    __mm_s.push_str(&*literal!(". Only works for MULTARY()!"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

fn removeTrivialScalarProduct(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { ty: __esc_ty, op: Operator::Op::SCALAR_PRODUCT }, exp1: __exp_exp1, exp2: __exp_exp2 } if (Type::sizeOf(&(Expression::typeOf(__exp_exp1.clone())), false)? == 1 && Type::sizeOf(&(Expression::typeOf(__exp_exp2.clone())), false)? == 1) => {
            ty = (*__esc_ty).clone();
            subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut d in (Type::arrayDims(Expression::typeOf(__exp_exp1.clone()))).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp1 = Expression::applySubscripts(&subs, __exp_exp1.clone(), false)?;
            subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut d in (Type::arrayDims(Expression::typeOf(__exp_exp2.clone()))).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp2 = Expression::applySubscripts(&subs, __exp_exp2.clone(), false)?;
            metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1, operator: metamodelica::Ref::new(Operator::NFOperator { ty: ty.clone(), op: Operator::Op::MUL.clone() }), exp2: exp2 })
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn simplifyURIToFilename(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())? {
        outExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::FMU_LOAD_RESOURCE().clone(),
                list![arg],
                Call::variability(&call)?,
                Purity::IMPURE.clone(),
                NFBuiltinFuncs::FMU_LOAD_RESOURCE().returnType.clone(),
            ),
        });
    } else {
        outExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    }
    Ok(outExp)
}
