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

use crate::NFCall as Call;
use crate::NFCallAttributes;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub struct NFExpandExp;
pub fn expand(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut backend: bool,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut expanded: bool;
    (exp, expanded) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            (exp, true)
        },
        Deref @ Expression::REAL { .. } => {
            (exp, true)
        },
        Deref @ Expression::STRING { .. } => {
            (exp, true)
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (exp, true)
        },
        Deref @ Expression::ENUM_LITERAL { .. } => {
            (exp, true)
        },
        Deref @ Expression::CREF { ty: Deref @ Type::ARRAY { .. }, .. } => {
            expandCref(exp, backend, resize)?
        },
        Deref @ Expression::ARRAY { ty: __exp_ty, .. } if (Type::isVector(metamodelica::AsArg::as_arg(&__exp_ty))?) => {
            (exp, true)
        },
        Deref @ Expression::ARRAY { .. } => {
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            (arr, expanded) = expandArray(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone())?;
            assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = arr.clone());
            (exp, expanded)
        },
        Deref @ Expression::TYPENAME { ty: __exp_ty } => {
            (expandTypename(__exp_ty.clone())?, true)
        },
        Deref @ Expression::RANGE { .. } => {
            expandRange(exp)?
        },
        Deref @ Expression::CALL { call: __exp_call } => {
            expandCall(__exp_call.clone(), exp, resize)?
        },
        Deref @ Expression::SIZE { .. } => {
            expandSize(exp)
        },
        Deref @ Expression::BINARY { operator: __exp_operator, .. } => {
            expandBinary(exp, metamodelica::AsArg::as_arg(&__exp_operator), resize)?
        },
        Deref @ Expression::MULTARY { .. } => {
            expand(SimplifyExp::splitMultary(exp)?, resize, false)?
        },
        Deref @ Expression::UNARY { .. } => {
            expandUnary(exp)?
        },
        Deref @ Expression::LBINARY { .. } => {
            expandLogicalBinary(exp)?
        },
        Deref @ Expression::LUNARY { .. } => {
            expandLogicalUnary(exp)?
        },
        Deref @ Expression::RELATION { .. } => {
            (exp, true)
        },
        Deref @ Expression::CAST { .. } => {
            expandCast(exp)?
        },
        Deref @ Expression::FILENAME { .. } => {
            (exp, true)
        },
        _ => {
            expandGeneric(exp, resize)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, expanded))
}

pub(crate) fn expandArray(
    mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>, bool)> {
    let mut outArray: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut expanded: bool = true;
    let mut res: bool;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    outArray = metamodelica::arrayFromVec(arr.clone().borrow().clone());
    for mut i in 1..=metamodelica::arrayLength(outArray.clone()) {
        (e, res) = expand(
            metamodelica::Dangerous::arrayGetNoBoundsChecking(outArray.clone(), i),
            false,
            false,
        )?;
        if !(res) {
            expanded = false;
            return Ok((outArray.clone(), expanded));
        }
        metamodelica::Dangerous::arrayUpdateNoBoundsChecking(outArray.clone(), i, e);
    }
    Ok((outArray, expanded))
}

pub(crate) fn expandList(
    mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut abortOnFailure: bool,
) -> Result<(metamodelica::List<metamodelica::Ref<Expression::NFExpression>>, bool)> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut expanded: bool = true;
    let mut res: bool;
    for mut exp in &*expl {
        let mut exp = exp.clone();
        (exp, res) = expand(exp, false, false)?;
        expanded = res && expanded;
        if !(res) && abortOnFailure {
            outExpl = expl;
            return Ok((outExpl, expanded));
        }
        outExpl = metamodelica::cons(exp, outExpl);
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl);
    Ok((outExpl, expanded))
}

pub(crate) fn expandCref(
    mut crefExp: metamodelica::Ref<Expression::NFExpression>,
    mut backend: bool,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut arrayExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
    (arrayExp, expanded) = (::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, ty: __crefExp_ty } => {
            if Type::hasZeroDimension(metamodelica::AsArg::as_arg(&__crefExp_ty))? {
                arrayExp = Expression::makeEmptyArray(__crefExp_ty.clone())?;
                expanded = true;
            } else if Type::hasKnownSize(__crefExp_ty.clone())? {
                subs = expandCref2(var_field!((*crefExp).cref, Expression::NFExpression::CREF), backend, resize, metamodelica::nil())?;
                arrayExp = expandCref3(&subs, var_field!((*crefExp).cref, Expression::NFExpression::CREF).clone(), Type::arrayElementType(metamodelica::AsArg::as_arg(&__crefExp_ty)), &(metamodelica::nil()))?;
                expanded = true;
            } else {
                arrayExp = crefExp;
                expanded = false;
            }
            (arrayExp, expanded)
        },
        _ => (crefExp, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((arrayExp, expanded))
}

pub(crate) fn expandCref2<'__b>(
    mut cref: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut backend: bool,
    mut resize: bool,
    mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>> {
    use crate::NFComponentRef::Origin;
    '__tco: loop {
        let mut cr_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        match &**cref {
            ComponentRef::CREF { .. }
                if (backend
                    || var_field!((**cref).origin, ComponentRef::NFComponentRef::CREF).clone()
                        == Origin::CREF.clone()) =>
            {
                dims = Type::arrayDims(var_field!((**cref).ty, ComponentRef::NFComponentRef::CREF).clone());
                cr_subs = Subscript::expandList(
                    var_field!((**cref).subscripts, ComponentRef::NFComponentRef::CREF),
                    dims.clone(),
                    resize,
                )?;
                if ((cr_subs).is_empty() && !((dims).is_empty())) {
                    return Ok(metamodelica::nil());
                } else {
                    {
                        (cref, backend, resize, subs) = (
                            var_field!((**cref).restCref, ComponentRef::NFComponentRef::CREF),
                            backend,
                            resize,
                            metamodelica::cons(cr_subs, subs),
                        );
                        continue '__tco;
                    }
                }
            }
            _ => return Ok(subs),
        }
    }
}

pub(crate) fn expandCref3(
    mut subs: &metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefType: metamodelica::Ref<Type::NFType>,
    mut accum: &metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut arrayExp: metamodelica::Ref<Expression::NFExpression>;
    arrayExp = (::match_deref::match_deref! { match subs {
        Deref @ metamodelica::ListNode::Nil => metamodelica::Ref::new(Expression::NFExpression::CREF { ty: crefType, cref: ComponentRef::setSubscriptsList(accum, cref)? }),
        _ => expandCref4((subs).head().cloned()?, metamodelica::nil(), accum, &((subs).rest()?), &cref, &crefType)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arrayExp)
}

pub(crate) fn expandCref4<'__b>(
    mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut comb: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut accum: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
    mut restSubs: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
    mut cref: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefType: &'__b metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    '__tco: loop {
        let mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
        let mut arr_ty: metamodelica::Ref<Type::NFType>;
        let mut slice: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut rest: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut i: i32;
        ::match_deref::match_deref! { match &(subs.clone()) {
            Deref @ metamodelica::ListNode::Nil => return Ok(expandCref3(restSubs, cref.clone(), crefType.clone(), &(metamodelica::cons(comb.reverse(), accum.clone())))?),
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::EXPANDED_SLICE { indices: __esc_slice }, tail: __esc_rest } => {
                slice = (*__esc_slice).clone();
                rest = (*__esc_rest).clone();
                expl = metamodelica::arrayCreate(((slice).len() as i32), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }));
                i = 1;
                for mut idx in &*slice.clone() {
                    unsafe { metamodelica::Dangerous::arrayInitSlot(expl.clone(), i, expandCref4(rest.clone(), metamodelica::cons(idx.clone(), comb.clone()), accum, restSubs, cref, crefType)?) };
                    i = i + 1;
                }
                arr_ty = Type::liftArrayLeft(Expression::typeOf(metamodelica::arrayGet(expl.clone(), 1)?), &(Dimension::fromExpArray(expl.clone())));
                return Ok(Expression::makeArray(arr_ty, expl.clone(), false))
            },
            _ => { (subs, comb, accum, restSubs, cref, crefType) = ((subs).rest()?, metamodelica::cons((subs).head().cloned()?, comb), accum, restSubs, cref, crefType); continue '__tco; },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn expandTypename(
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { elementType: Deref @ Type::BOOLEAN, .. } => {
            Expression::makeArray(ty, metamodelica::arrayFromVec(list![metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }), metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })].into_iter().cloned().collect()), true)
        },
        Deref @ Type::ARRAY { elementType: Deref @ Type::ENUMERATION { .. }, .. } => {
            let mut lits: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            lits = Expression::makeEnumLiterals(var_field!((*ty).elementType, Type::NFType::ARRAY).clone())?;
            Expression::makeArray(ty, metamodelica::arrayFromVec(lits.into_iter().cloned().collect()), true)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpandExp.expandTypename")); __mm_s.push_str(&*literal!(" got invalid typename")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("NFFrontEnd/NFExpandExp.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn expandRange(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { ty: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    expanded = Expression::isLiteral(&exp)?;
    if expanded {
        outExp = Ceval::evalExp(exp, &(Ceval::noTarget().clone()))?;
    } else {
        (outExp, expanded) = expandNonLiteralRange(exp, ty)?;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandNonLiteralRange(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut ety: metamodelica::Ref<Type::NFType>;
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut step_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut ostep_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut sz: i32;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    ety = Type::arrayElementType(&ty);
    if !(Type::hasKnownSize(ty.clone())? && (Type::isInteger(&ety)? || Type::isReal(&ety)?)) {
        outExp = exp;
        expanded = false;
        return Ok((outExp, expanded));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::RANGE { start: __pa0, step: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    start_exp = metamodelica::Own::own(__pa0);
    ostep_exp = metamodelica::Own::own(__pa1);
    step_exp = ostep_exp.unwrap_or(Expression::makeOne(&ety)?);
    sz = Dimension::size(&(Type::nthDimension(ty.clone(), 1)?), false)?;
    for mut i in ({
        let __s = sz;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if i == 1 {
            e = start_exp.clone();
        } else {
            e = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: makeIndexOffset(i - 1, &ety)?,
                operator: Operator::makeMul(ety.clone()),
                exp2: step_exp.clone(),
            });
            e = SimplifyExp::simplify(
                metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: start_exp.clone(),
                    operator: Operator::makeAdd(ety.clone()),
                    exp2: e,
                }),
                false,
            )?;
        }
        expl = metamodelica::cons(e, expl);
    }
    outExp = Expression::makeArray(
        ty,
        metamodelica::arrayFromVec(expl.into_iter().cloned().collect()),
        false,
    );
    expanded = true;
    Ok((outExp, expanded))
}

pub(crate) fn makeIndexOffset(
    mut offset: i32,
    mut ty: &metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = if (Type::isReal(ty)?) {
        metamodelica::Ref::new(Expression::NFExpression::REAL { value: intReal(offset) })
    } else {
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: offset })
    };
    Ok(exp)
}

pub(crate) fn expandCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    (outExp, expanded) = 'mc: {
        let __mc_input = &*call;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Call::TYPED_CALL { .. } => {
                    if !((Function::isBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL)) && !(Function::isImpure(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL))))) { return Err("guard") }
                    Ok(expandBuiltinCall(&(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone()), &(var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()), call.clone(), resize)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
                    Ok(expandArrayConstructor(var_field!((*call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*call).ty, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), &(var_field!((*call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(expandGeneric(exp.clone(), resize)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, expanded))
}

pub(crate) fn expandBuiltinCall(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut fn_path: metamodelica::Ref<Absyn::Path> = Function::nameConsiderBuiltin(r#fn);
    (outExp, expanded) = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&fn_path)) {
        Deref @ "cat" => expandBuiltinCat(args, call, resize)?,
        Deref @ "der" => expandBuiltinGeneric(&call)?,
        Deref @ "diagonal" => expandBuiltinDiagonal((args).head().cloned()?)?,
        Deref @ "fill" => expandBuiltinFill(args)?,
        Deref @ "pre" => expandBuiltinGeneric(&call)?,
        Deref @ "previous" => expandBuiltinGeneric(&call)?,
        Deref @ "promote" => expandBuiltinPromote(args)?,
        Deref @ "transpose" => expandBuiltinTranspose((args).head().cloned()?)?,
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExp, expanded))
}

pub(crate) fn expandBuiltinCat(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    (expl, expanded) = expandList((args).rest()?, true)?;
    if expanded {
        exp = Ceval::evalBuiltinCat(&((args).head().cloned()?), expl, &(Ceval::noTarget().clone()))?;
    } else {
        (exp, _) = expandGeneric(
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: call }),
            resize,
        )?;
    }
    Ok((exp, expanded))
}

pub(crate) fn expandBuiltinPromote(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut n: i32;
    let mut eexp: metamodelica::Ref<Expression::NFExpression>;
    let mut nexp: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    eexp = metamodelica::Own::own(__pa0);
    nexp = metamodelica::Own::own(__pa1);
    let __pa3 = ::match_deref::match_deref! { match &(nexp) {
        Deref @ Expression::INTEGER { value: __pa3 } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa3);
    (eexp, expanded) = expand(eexp, false, false)?;
    (exp, _) = Expression::promote(eexp.clone(), Expression::typeOf(eexp), n)?;
    Ok((exp, expanded))
}

pub(crate) fn expandBuiltinDiagonal(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    (outExp, expanded) = expand(arg, false, false)?;
    if expanded {
        outExp = Ceval::evalBuiltinDiagonal(outExp)?;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBuiltinFill(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool = true;
    outExp = Expression::fillArgs((args).head().cloned()?, (args).rest()?)?;
    Ok((outExp, expanded))
}

pub(crate) fn expandBuiltinTranspose(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    (outExp, expanded) = expand(arg, false, false)?;
    if expanded {
        outExp = Expression::transposeArray(&outExp)?;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBuiltinGeneric(
    mut call: &metamodelica::Ref<Call::NFCall>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool = true;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut pur: Purity;
    let mut attr: metamodelica::Ref<NFCallAttributes::NFCallAttributes>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::TYPED_CALL { r#fn: __pa0, ty: __pa1, var: __pa2, purity: __pa3, arguments: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil }, attributes: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    var = metamodelica::Own::own(__pa2);
    pur = metamodelica::Own::own(__pa3);
    arg = metamodelica::Own::own(__pa4);
    attr = metamodelica::Own::own(__pa5);
    ty = Type::arrayElementType(&ty);
    let __pa7 = ::match_deref::match_deref! { match &(expand(arg, false, false)?) {
        (__pa7, true) => __pa7.clone(),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa7);
    outExp = expandBuiltinGeneric2(arg, r#fn, ty, var, pur, attr)?;
    Ok((outExp, expanded))
}

pub(crate) fn expandBuiltinGeneric2(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut var: Variability,
    mut pur: Purity,
    mut attr: metamodelica::Ref<NFCallAttributes::NFCallAttributes>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::ARRAY { literal: true, .. } => exp,
        Expression::ARRAY { ty: __exp_ty, .. } => {
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            arr = Array::map(
                var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(),
                &({
                    let __pe_b1 = r#fn;
                    let __pe_b2 = ty.clone();
                    let __pe_b3 = var;
                    let __pe_b4 = pur;
                    let __pe_b5 = attr;
                    move |__pe_a0| {
                        expandBuiltinGeneric2(
                            __pe_a0,
                            __pe_b1.clone(),
                            __pe_b2.clone(),
                            __pe_b3.clone(),
                            __pe_b4.clone(),
                            __pe_b5.clone(),
                        )
                    }
                }),
            )?;
            Expression::makeArray(
                Type::setArrayElementType(metamodelica::AsArg::as_arg(&__exp_ty), &ty),
                arr.clone(),
                false,
            )
        }
        _ => metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(Call::NFCall::TYPED_CALL {
                r#fn: r#fn,
                ty: ty,
                var: var,
                purity: pur,
                arguments: list![exp],
                attributes: attr,
            }),
        }),
    });
    Ok(exp)
}

pub(crate) fn expandArrayConstructor(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut iterators: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool = true;
    let mut e: metamodelica::Ref<Expression::NFExpression> = exp.clone();
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut iter: Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>;
    let mut iters: metamodelica::List<Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>> =
        metamodelica::nil();
    if Type::hasKnownSize(ty.clone())?
        && !(List::any(
            iterators,
            &({
                let __pe_b1 = exp.clone();
                move |__pe_a0| usesIterator(__pe_a0, __pe_b1.clone())
            }),
        )?)
    {
        result = fillArrayConstructor(
            (expand(SimplifyExp::simplify(exp, false)?, false, false)?).0,
            ty,
            ((iterators).len() as i32),
        )?;
        return Ok((result, expanded));
    }
    for mut i in &**iterators {
        (node, range) = i.clone();
        iter = Mutable::create(metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: InstNode::getType(node.clone())?,
        }));
        e = Expression::replaceIterator(
            e,
            &node,
            &(metamodelica::Ref::new(Expression::NFExpression::MUTABLE { exp: iter.clone() })),
        )?;
        iters = metamodelica::cons(iter, iters);
        let __pa0 = ::match_deref::match_deref! { match &(expand(range, false, false)?) {
            (__pa0, true) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa0);
        ranges = metamodelica::cons(range, ranges);
    }
    result = expandArrayConstructor2(e, ty, &ranges, &iters)?;
    Ok((result, expanded))
}

pub(crate) fn usesIterator(
    mut iterator: (
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    ),
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut used: bool = Expression::containsIterator(exp.clone(), &(Util::tuple21(iterator.clone())))?;
    Ok(used)
}

pub(crate) fn fillArrayConstructor(
    mut value: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut levels: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = if (levels == 0) {
        value
    } else {
        Expression::makeArray(
            ty.clone(),
            arrayCreate(
                Dimension::size(&(Type::nthDimension(ty.clone(), 1)?), false)?,
                fillArrayConstructor(value, Type::unliftArray(ty)?, levels - 1)?,
            ),
            false,
        )
    };
    Ok(result)
}

pub(crate) fn expandArrayConstructor2(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut ranges: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut iterators: &metamodelica::List<Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut ranges_rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut iter: Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>;
    let mut iters_rest: metamodelica::List<Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>>;
    let mut range_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    let mut el_ty: metamodelica::Ref<Type::NFType>;
    if (ranges).is_empty() {
        (result, _) = expand(SimplifyExp::simplify(exp, false)?, false, false)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*ranges)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa0);
        ranges_rest = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*iterators)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        iter = metamodelica::Own::own(__pa2);
        iters_rest = metamodelica::Own::own(__pa3);
        range_iter = ExpressionIterator::fromExp(range, false, false)?;
        el_ty = Type::unliftArray(ty.clone())?;
        while ExpressionIterator::hasNext(&range_iter) {
            (range_iter, value) = ExpressionIterator::next(range_iter)?;
            Mutable::update(iter.clone(), value);
            expl = metamodelica::cons(
                expandArrayConstructor2(exp.clone(), el_ty.clone(), &ranges_rest, &iters_rest)?,
                expl,
            );
        }
        result = Expression::makeArray(
            ty,
            metamodelica::arrayFromVec(
                metamodelica::Dangerous::listReverseInPlace(expl)
                    .into_iter()
                    .cloned()
                    .collect(),
            ),
            false,
        );
    }
    Ok(result)
}

pub(crate) fn expandSize(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> (metamodelica::Ref<Expression::NFExpression>, bool) {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool = true;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SIZE { exp: e, dimIndex: None } => {
            let mut dims: i32;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            ty = Expression::typeOf(e.clone());
            dims = Type::dimensionCount(ty.clone());
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut i in (1..=dims).into_iter() {
            let __x = metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: e.clone(), dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() })) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Expression::makeArray(metamodelica::Ref::new(Type::NFType::ARRAY { elementType: ty, dimensions: list![Dimension::fromInteger(dims, Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(expl.into_iter().cloned().collect()), false)
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, expanded)
}

pub(crate) fn expandBinary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut op: &metamodelica::Ref<Operator::NFOperator>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    use crate::NFOperator::Op;
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    (outExp, expanded) = (match op.op.clone() {
        Operator::Op::ADD_SCALAR_ARRAY => expandBinaryScalarArray(exp.clone(), Op::ADD.clone())?,
        Operator::Op::ADD_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::ADD.clone())?,
        Operator::Op::SUB_SCALAR_ARRAY { .. } => expandBinaryScalarArray(exp.clone(), Op::SUB.clone())?,
        Operator::Op::SUB_ARRAY_SCALAR => expandBinaryArrayScalar(exp.clone(), Op::SUB.clone())?,
        Operator::Op::MUL_SCALAR_ARRAY => expandBinaryScalarArray(exp.clone(), Op::MUL.clone())?,
        Operator::Op::MUL_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::MUL.clone())?,
        Operator::Op::MUL_VECTOR_MATRIX => expandBinaryVectorMatrix(exp.clone())?,
        Operator::Op::MUL_MATRIX_VECTOR => expandBinaryMatrixVector(exp.clone())?,
        Operator::Op::SCALAR_PRODUCT => expandBinaryDotProduct(exp.clone())?,
        Operator::Op::MATRIX_PRODUCT => expandBinaryMatrixProduct(exp.clone())?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => expandBinaryScalarArray(exp.clone(), Op::DIV.clone())?,
        Operator::Op::DIV_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::DIV.clone())?,
        Operator::Op::POW_SCALAR_ARRAY { .. } => expandBinaryScalarArray(exp.clone(), Op::POW.clone())?,
        Operator::Op::POW_ARRAY_SCALAR { .. } => expandBinaryArrayScalar(exp.clone(), Op::POW.clone())?,
        Operator::Op::POW_MATRIX => expandBinaryPowMatrix(exp.clone(), resize)?,
        _ => expandBinaryElementWise(exp.clone())?,
    });
    if !(expanded) {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBinaryElementWise(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    exp2 = metamodelica::Own::own(__pa2);
    if Type::isArray(&(Operator::typeOf(&op))) {
        (exp1, expanded) = expand(exp1, false, false)?;
        if expanded {
            (exp2, expanded) = expand(exp2, false, false)?;
        }
        if expanded {
            outExp = expandBinaryElementWise2(
                &exp1,
                Operator::stripEW(op),
                &exp2,
                (std::sync::Arc::new(SimplifyExp::simplifyBinaryOp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                metamodelica::Ref<Operator::NFOperator>,
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
        } else {
            outExp = exp;
        }
    } else {
        outExp = exp;
        expanded = true;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBinaryElementWise2(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Operator::NFOperator>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    pub type MakeFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Operator::NFOperator>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl1: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl2: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut eop: metamodelica::Ref<Operator::NFOperator>;
    expl1 = Expression::arrayElements(exp1)?;
    expl2 = Expression::arrayElements(exp2)?;
    ty = Operator::typeOf(&op);
    eop = Operator::setType(Type::unliftArray(ty.clone())?, op);
    if Type::dimensionCount(ty.clone()) > 1 {
        expl = Array::threadMap(
            expl1.clone(),
            expl2.clone(),
            &({
                let __pe_b1 = eop;
                let __pe_b3: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Operator::NFOperator>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                > = func.clone();
                move |__pe_a0, __pe_a2| expandBinaryElementWise2(&__pe_a0, __pe_b1.clone(), &__pe_a2, __pe_b3.clone())
            }),
        )?;
    } else {
        expl = Array::threadMap(
            expl1.clone(),
            expl2.clone(),
            &({
                let __pe_b1 = eop;
                move |__pe_a0, __pe_a2| func(__pe_a0, __pe_b1.clone(), __pe_a2)
            }),
        )?;
    }
    exp = Expression::makeArray(ty, expl.clone(), false);
    Ok(exp)
}

pub(crate) fn expandBinaryScalarArray(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut scalarOp: Operator::Op,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    exp2 = metamodelica::Own::own(__pa2);
    (exp2, expanded) = expand(exp2, false, false)?;
    if expanded {
        op = metamodelica::Ref::new(Operator::NFOperator {
            ty: Type::arrayElementType(&(Operator::typeOf(&op))),
            op: scalarOp,
        });
        outExp = Expression::mapArrayElements(
            exp2,
            (std::sync::Arc::new({
                let __pe_b0 = exp1;
                let __pe_b1 = op;
                move |__pe_a2| SimplifyExp::simplifyBinaryOp(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn makeScalarArrayBinary_traverser(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &*exp2 {
        Expression::ARRAY { .. } => exp2,
        _ => SimplifyExp::simplifyBinaryOp(exp1, op, exp2)?,
    });
    Ok(exp)
}

pub(crate) fn expandBinaryArrayScalar(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut scalarOp: Operator::Op,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    exp2 = metamodelica::Own::own(__pa2);
    (exp1, expanded) = expand(exp1, false, false)?;
    if expanded {
        op = metamodelica::Ref::new(Operator::NFOperator {
            ty: Type::arrayElementType(&(Operator::typeOf(&op))),
            op: scalarOp,
        });
        outExp = Expression::mapArrayElements(
            exp1,
            (std::sync::Arc::new({
                let __pe_b1 = op;
                let __pe_b2 = exp2;
                move |__pe_a0| SimplifyExp::simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBinaryVectorMatrix(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut m: metamodelica::Ref<Dimension::NFDimension>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, exp2: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exp2 = metamodelica::Own::own(__pa1);
    (exp2, expanded) = expand(exp2, false, false)?;
    if expanded {
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Expression::transposeArray(&exp2)?) {
            Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __pa2, dimensions: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __pa4, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa2);
        m = metamodelica::Own::own(__pa3);
        arr = metamodelica::Own::own(__pa4);
        ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: ty,
            dimensions: list![m],
        });
        if arr.clone().borrow().is_empty() || Type::isEmptyArray(&ty)? {
            outExp = Expression::makeZero(&ty)?;
        } else {
            (exp1, expanded) = expand(exp1, false, false)?;
            if expanded {
                arr = Array::map(
                    arr.clone(),
                    &({
                        let __pe_b0 = exp1;
                        move |__pe_a1| makeScalarProduct(&__pe_b0, &__pe_a1)
                    }),
                )?;
                outExp = Expression::makeArray(ty, arr.clone(), false);
            } else {
                outExp = exp;
            }
        }
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBinaryMatrixVector(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut n: metamodelica::Ref<Dimension::NFDimension>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, exp2: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exp2 = metamodelica::Own::own(__pa1);
    (exp1, expanded) = expand(exp1, false, false)?;
    if expanded {
        let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(exp1) {
            Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __pa2, dimensions: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __pa4, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa2);
        n = metamodelica::Own::own(__pa3);
        arr = metamodelica::Own::own(__pa4);
        ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: ty,
            dimensions: list![n],
        });
        if arr.clone().borrow().is_empty() || Type::isEmptyArray(&ty)? {
            outExp = Expression::makeZero(&ty)?;
        } else {
            (exp2, expanded) = expand(exp2, false, false)?;
            if expanded {
                arr = Array::map(
                    arr.clone(),
                    &({
                        let __pe_b1 = exp2;
                        move |__pe_a0| makeScalarProduct(&__pe_a0, &__pe_b1)
                    }),
                )?;
                outExp = Expression::makeArray(ty, arr.clone(), false);
            } else {
                outExp = exp;
            }
        }
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandBinaryDotProduct(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, exp2: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exp2 = metamodelica::Own::own(__pa1);
    (exp1, expanded) = expand(exp1, false, false)?;
    if expanded {
        (exp2, expanded) = expand(exp2, false, false)?;
    }
    if expanded {
        outExp = makeScalarProduct(&exp1, &exp2)?;
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn makeScalarProduct(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut arr1: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr2: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut mul_op: metamodelica::Ref<Operator::NFOperator>;
    let mut add_op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*exp1)) {
        Deref @ Expression::ARRAY { ty: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    arr1 = metamodelica::Own::own(__pa1);
    let __pa2 = ::match_deref::match_deref! { match &((*exp2)) {
        Deref @ Expression::ARRAY { ty: _, elements: __pa2, .. } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    arr2 = metamodelica::Own::own(__pa2);
    elem_ty = Type::unliftArray(ty)?;
    if arr1.clone().borrow().is_empty() {
        exp = Expression::makeZero(&elem_ty)?;
    } else {
        mul_op = Operator::makeMul(elem_ty.clone());
        add_op = Operator::makeAdd(elem_ty);
        arr1 = Array::threadMap(
            arr1.clone(),
            arr2.clone(),
            &({
                let __pe_b1 = mul_op;
                move |__pe_a0, __pe_a2| SimplifyExp::simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2)
            }),
        )?;
        exp = Array::reduce(
            arr1.clone(),
            &({
                let __pe_b1 = add_op;
                move |__pe_a0, __pe_a2| SimplifyExp::simplifyBinaryOp(__pe_a0, __pe_b1.clone(), __pe_a2)
            }),
        )?;
    }
    Ok(exp)
}

pub(crate) fn expandBinaryMatrixProduct(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, exp2: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    exp2 = metamodelica::Own::own(__pa1);
    (exp1, expanded) = expand(exp1, false, false)?;
    if expanded {
        (exp2, expanded) = expand(exp2, false, false)?;
    }
    if expanded {
        outExp = makeBinaryMatrixProduct(exp1, &exp2)?;
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn makeBinaryMatrixProduct(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut arr1: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr2: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut mat_ty: metamodelica::Ref<Type::NFType>;
    let mut n: metamodelica::Ref<Dimension::NFDimension>;
    let mut p: metamodelica::Ref<Dimension::NFDimension>;
    let mut len: i32;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __pa0, dimensions: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    arr1 = metamodelica::Own::own(__pa2);
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(Expression::transposeArray(exp2)?) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, elements: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    p = metamodelica::Own::own(__pa4);
    arr2 = metamodelica::Own::own(__pa5);
    mat_ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: ty.clone(),
        dimensions: list![n, p.clone()],
    });
    if arr2.clone().borrow().is_empty() {
        exp = Expression::makeZero(&mat_ty)?;
    } else {
        row_ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: ty,
            dimensions: list![p],
        });
        len = metamodelica::arrayLength(arr1.clone());
        arr = metamodelica::arrayCreate(len, exp1);
        for mut i in 1..=len {
            e = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i);
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    arr.clone(),
                    i,
                    Expression::makeArray(row_ty.clone(), makeBinaryMatrixProduct2(&e, arr2.clone())?, false),
                )
            };
        }
        exp = Expression::makeArray(mat_ty, arr.clone(), false);
    }
    Ok(exp)
}

pub(crate) fn makeBinaryMatrixProduct2(
    mut row: &metamodelica::Ref<Expression::NFExpression>,
    mut matrix: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>> {
    let mut outRow: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    outRow = Array::map(
        matrix.clone(),
        &({
            let __pe_b0 = row.clone();
            move |__pe_a1| makeScalarProduct(&__pe_b0, &__pe_a1)
        }),
    )?;
    Ok(outRow)
}

pub(crate) fn expandBinaryPowMatrix(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let mut n: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    exp2 = metamodelica::Own::own(__pa2);
    (outExp, expanded) = (match &*exp2 {
        Expression::INTEGER { value: 0 } => {
            n = Dimension::size(&((Type::arrayDims(Operator::typeOf(&op))).head().cloned()?), false)?;
            (Expression::makeIdentityMatrix(n, crate::NFType::interned_REAL())?, true)
        }
        Expression::INTEGER { value: n } if (n.clone() > 0) => {
            (exp1, expanded) = expand(exp1, false, false)?;
            if expanded {
                outExp = expandBinaryPowMatrix2(&exp1, n.clone())?;
            }
            (outExp, expanded)
        }
        _ => expandGeneric(exp, resize)?,
    });
    Ok((outExp, expanded))
}

pub(crate) fn expandBinaryPowMatrix2(
    mut matrix: &metamodelica::Ref<Expression::NFExpression>,
    mut n: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match n {
        1 => matrix.clone(),
        2 => makeBinaryMatrixProduct(matrix.clone(), matrix)?,
        _ if (intMod(n, 2) == 0) => {
            exp = expandBinaryPowMatrix2(matrix, intDiv(n, 2))?;
            makeBinaryMatrixProduct(exp.clone(), &(exp))?
        }
        _ => {
            exp = expandBinaryPowMatrix2(matrix, n - 1)?;
            makeBinaryMatrixProduct(matrix.clone(), &exp)?
        }
    });
    Ok(exp)
}

pub(crate) fn expandUnary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut operand: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let mut scalar_op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::UNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    op = metamodelica::Own::own(__pa0);
    operand = metamodelica::Own::own(__pa1);
    (operand, expanded) = expand(operand, false, false)?;
    if expanded {
        scalar_op = Operator::scalarize(op);
        outExp = Expression::mapArrayElements(
            operand,
            (std::sync::Arc::new({
                let __pe_b1 = scalar_op;
                move |__pe_a0| SimplifyExp::simplifyUnaryOp(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandLogicalBinary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    exp2 = metamodelica::Own::own(__pa2);
    if Type::isArray(&(Operator::typeOf(&op))) {
        (exp1, expanded) = expand(exp1, false, false)?;
        if expanded {
            (exp2, expanded) = expand(exp2, false, false)?;
        }
        if expanded {
            outExp = expandBinaryElementWise2(
                &exp1,
                op,
                &exp2,
                (std::sync::Arc::new(makeLBinaryOp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                metamodelica::Ref<Operator::NFOperator>,
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
        } else {
            outExp = exp;
        }
    } else {
        outExp = exp;
        expanded = true;
    }
    Ok((outExp, expanded))
}

pub(crate) fn makeLBinaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    if Expression::isScalarLiteral(&exp1) && Expression::isScalarLiteral(&exp2) {
        exp = Ceval::evalLogicBinaryOp(exp1, op, exp2, &(Ceval::noTarget().clone()))?;
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
            exp1: exp1,
            operator: op,
            exp2: exp2,
        });
    }
    Ok(exp)
}

pub(crate) fn expandLogicalUnary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut operand: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let mut scalar_op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LUNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    op = metamodelica::Own::own(__pa0);
    operand = metamodelica::Own::own(__pa1);
    (operand, expanded) = expand(operand, false, false)?;
    if expanded {
        scalar_op = Operator::scalarize(op);
        outExp = Expression::mapArrayElements(
            operand,
            (std::sync::Arc::new({
                let __pe_b1 = scalar_op;
                move |__pe_a0| Ok(makeLogicalUnaryOp(__pe_a0, __pe_b1.clone()))
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        outExp = exp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn makeLogicalUnaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::LUNARY {
            operator: op.clone(),
            exp: exp1.clone(),
        });
    exp
}

pub(crate) fn expandCast(
    mut castExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(castExp.clone()) {
        Deref @ Expression::CAST { exp: __pa0, ty: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    (outExp, expanded) = expand(exp.clone(), false, false)?;
    if expanded && !(referenceEq(&*(exp), &*(&*outExp))) {
        outExp = Expression::typeCast(outExp, ty)?;
    } else {
        outExp = castExp;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandGeneric(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
    ty = Expression::typeOf(exp.clone());
    if Type::isArray(&ty) {
        expanded = Type::hasKnownSize(ty.clone())?;
        if expanded {
            dims = Type::arrayDims(ty.clone());
            subs = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> =
                    metamodelica::nil();
                for mut d in (dims).into_iter().cloned() {
                    let __x = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
                            metamodelica::nil();
                        for mut e in (RangeIterator::toList(RangeIterator::fromDim(&(d.clone()), resize)?)?)
                            .into_iter()
                            .cloned()
                        {
                            let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: e.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            outExp = expandGeneric2(&subs, &exp, &ty, &(metamodelica::nil()))?;
        } else {
            outExp = exp;
        }
    } else {
        outExp = exp;
        expanded = true;
    }
    Ok((outExp, expanded))
}

pub(crate) fn expandGeneric2(
    mut subs: &metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut accum: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut t: metamodelica::Ref<Type::NFType>;
    let mut sub: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut rest_subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
    let mut i: i32;
    outExp = (::match_deref::match_deref! { match subs {
        Deref @ metamodelica::ListNode::Cons { head: __esc_sub, tail: __esc_rest_subs } => {
            sub = (*__esc_sub).clone();
            rest_subs = (*__esc_rest_subs).clone();
            t = Type::unliftArray(ty.clone())?;
            expl = metamodelica::arrayCreate(((sub).len() as i32), exp.clone());
            i = 1;
            for mut s in &*sub.clone() {
                unsafe { metamodelica::Dangerous::arrayInitSlot(expl.clone(), i, expandGeneric2(metamodelica::AsArg::as_arg(&rest_subs), exp, &t, &(metamodelica::cons(s.clone(), accum.clone())))?) };
                i = i + 1;
            }
            Expression::makeArray(ty.clone(), expl.clone(), false)
        },
        Deref @ metamodelica::ListNode::Nil => {
            outExp = exp.clone();
            for mut s in &*accum.clone().reverse() {
                outExp = Expression::applySubscript(metamodelica::AsArg::as_arg(&s), &outExp, &(metamodelica::nil()), false)?;
            }
            outExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn expandCallArgs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { .. } } => {
            call = (*__esc_call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = (expand(arg.clone(), false, false)?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}
