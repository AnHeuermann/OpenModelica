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
use crate::NFClass as Class;
use crate::NFClockKind as ClockKind;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::NamedArg;
use crate::NFFunction::TypedArg;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

pub(crate) fn needSpecialHandling(mut call: &metamodelica::Ref<Call::NFCall>) -> Result<bool> {
    let mut special: bool = false;
    let () = (match &**call {
        Call::UNTYPED_CALL { r#ref: __call_ref, .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(NFInstNode::InstNode::getFuncCache(&(NFInstNode::InstNode::classScope(ComponentRef::node(metamodelica::AsArg::as_arg(&__call_ref))?)?))?) {
                Deref @ NFInstNode::CachedData::FUNCTION { specialBuiltin: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            special = metamodelica::Own::own(__pa0);
            ()
        }
        Call::TYPED_CALL { r#fn: __call_fn, .. } => {
            special = Function::isSpecialBuiltin(metamodelica::AsArg::as_arg(&__call_fn));
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBuiltinCall.needSpecialHandling"));
                    __mm_s.push_str(&*literal!(" got unknown call: "));
                    __mm_s.push_str(&*Call::toString(call)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFBuiltinCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(special)
}

pub(crate) fn typeSpecial(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cref = metamodelica::Own::own(__pa0);
    (callExp, ty, variability, purity) = (::match_deref::match_deref! { match &(ComponentRef::firstName(&cref, false)?) {
        Deref @ "actualStream" => typeActualInStreamCall(literal!("actualStream"), &call, context, &info)?,
        Deref @ "backSample" => typeBackSampleCall(call, context, info)?,
        Deref @ "branch" => typeBranchCall(&call, context, &info)?,
        Deref @ "cardinality" => typeCardinalityCall(call, context, info)?,
        Deref @ "cat" => typeCatCall(&call, context, info)?,
        Deref @ "change" => typeChangeCall(&call, context, &info)?,
        Deref @ "Clock" => typeClockCall(call, context, info)?,
        Deref @ "der" => typeDerCall(&call, context, &info)?,
        Deref @ "DynamicSelect" => typeDynamicSelectCall(literal!("DynamicSelect"), &call, context, &info)?,
        Deref @ "edge" => typeEdgeCall(call, context, info)?,
        Deref @ "fill" => typeFillCall(&call, context, &info)?,
        Deref @ "getInstanceName" => typeGetInstanceName(call, context, info)?,
        Deref @ "initial" => typeDiscreteCall(call, context, info)?,
        Deref @ "inStream" => typeActualInStreamCall(literal!("inStream"), &call, context, &info)?,
        Deref @ "isRoot" => typeIsRootCall(&call, context, &info)?,
        Deref @ "matrix" => typeMatrixCall(&call, context, &info)?,
        Deref @ "max" => typeMinMaxCall(literal!("max"), call, context, info)?,
        Deref @ "min" => typeMinMaxCall(literal!("min"), call, context, info)?,
        Deref @ "ndims" => typeNdimsCall(&call, context, &info)?,
        Deref @ "noEvent" => typeNoEventCall(&call, context, &info)?,
        Deref @ "nthRoot" => typeNthRootCall(call, context, info)?,
        Deref @ "ones" => typeZerosOnesCall(literal!("ones"), &call, context, &info)?,
        Deref @ "potentialRoot" => typePotentialRootCall(&call, context, &info)?,
        Deref @ "pre" => typePreCall(&call, context, &info)?,
        Deref @ "promote" => typePromoteCall(&call, context, info)?,
        Deref @ "pure" => typePureCall(call, context, info)?,
        Deref @ "rooted" => typeRootedCall(&call, context, &info)?,
        Deref @ "root" => typeRootCall(&call, context, &info)?,
        Deref @ "sample" => typeSampleCall(call, context, &info)?,
        Deref @ "scalar" => typeScalarCall(&call, context, &info)?,
        Deref @ "shiftSample" => typeShiftSampleCall(call, context, info)?,
        Deref @ "smooth" => typeSmoothCall(&call, context, &info)?,
        Deref @ "spatialDistribution" => typeSpatialDistribution(call, context, info)?,
        Deref @ "String" => typeStringCall(call, context, info)?,
        Deref @ "subSample" => typeSubSampleCall(call, context, info)?,
        Deref @ "superSample" => typeSuperSampleCall(call, context, info)?,
        Deref @ "symmetric" => typeSymmetricCall(&call, context, &info)?,
        Deref @ "terminal" => typeDiscreteCall(call, context, info)?,
        Deref @ "transpose" => typeTransposeCall(&call, context, &info)?,
        Deref @ "uniqueRootIndices" => typeUniqueRootIndicesCall(&call, context, &info)?,
        Deref @ "uniqueRoot" => typeUniqueRootCall(&call, context, &info)?,
        Deref @ "vector" => typeVectorCall(&call, context, &info)?,
        Deref @ "zeros" => typeZerosOnesCall(literal!("zeros"), &call, context, &info)?,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBuiltinCall.typeSpecial")); __mm_s.push_str(&*literal!(" got unhandled builtin function: ")); __mm_s.push_str(&*Call::toString(&call)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBuiltinCall.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((callExp, ty, variability, purity))
}

pub(crate) fn makeSizeExp(
    mut posArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut namedArgs: &metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    assertNoNamedParams(literal!("size"), namedArgs, info)?;
    callExp = (::match_deref::match_deref! { match &(posArgs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
            arg1 = (*__esc_arg1).clone();
            metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: arg1.clone(), dimIndex: None })
        },
        Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            arg1 = (*__esc_arg1).clone();
            arg2 = (*__esc_arg2).clone();
            metamodelica::Ref::new(Expression::NFExpression::SIZE { exp: arg1.clone(), dimIndex: Some(arg2.clone()) })
        },
        _ => {
            Error::addSourceMessage(&(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size")); __mm_s.push_str(&*List::toString(posArgs, &Expression::toString, List::Style::FLAT_BRACKETS.clone())?); ArcStr::from(__mm_s) }, literal!("size(Any[:, ...]) => Integer[:]\n  size(Any[:, ...], Integer) => Integer")], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

pub(crate) fn makeArrayExp(
    mut posArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut namedArgs: &metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut arrayExp: metamodelica::Ref<Expression::NFExpression>;
    assertNoNamedParams(literal!("array"), namedArgs, info)?;
    if (posArgs).is_empty() {
        Error::addSourceMessage(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("array"));
                    __mm_s.push_str(&*List::toString(
                        posArgs.clone(),
                        &Expression::toString,
                        List::Style::FLAT_BRACKETS.clone(),
                    )?);
                    ArcStr::from(__mm_s)
                },
                literal!("array(Any, Any, ...) => Any[:]")
            ],
            info,
        )?;
        return Err("fail");
    }
    arrayExp = Expression::makeArray(
        crate::NFType::interned_UNKNOWN(),
        metamodelica::arrayFromVec(posArgs.into_iter().cloned().collect()),
        false,
    );
    Ok(arrayExp)
}

pub(crate) fn makeCatExp(
    mut n: i32,
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>>,
    mut variability: Variability,
    mut purity: Purity,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut args2: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut res: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut tys2: metamodelica::List<metamodelica::Ref<Type::NFType>> = tys.clone();
    let mut tys3: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut dimsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>> =
        metamodelica::nil();
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut resTy: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut resTyToMatch: metamodelica::Ref<Type::NFType>;
    let mut mk: TypeCheck::MatchKind;
    let mut maxn: i32;
    let mut pos: i32;
    let mut sumDim: metamodelica::Ref<Dimension::NFDimension>;
    Error::assertion(
        ((args).len() as i32) == ((tys).len() as i32) && !((args).is_empty()),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NFBuiltinCall.makeCatExp"));
            __mm_s.push_str(&*literal!(" got wrong input sizes"));
            ArcStr::from(__mm_s)
        },
        &(metamodelica::sourceInfo!("NFFrontEnd/NFBuiltinCall.mo")),
    )?;
    for mut arg in &**args {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tys2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa0);
        tys2 = metamodelica::Own::own(__pa1);
        dimsLst = metamodelica::cons(Type::arrayDims(ty.clone()), dimsLst);
        if Type::isEqual(&resTy, &(crate::NFType::interned_UNKNOWN()))? {
            resTy = Type::arrayElementType(&ty);
        } else {
            (_, _, ty1, mk) = TypeCheck::matchExpressions(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                Type::arrayElementType(&ty),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                resTy.clone(),
                TypeCheck::DEFAULT_OPTIONS.clone(),
            )?;
            if TypeCheck::isCompatibleMatch(mk) {
                resTy = ty1;
            }
        }
    }
    maxn = ({
        let mut __acc: Option<i32> = None;
        for mut d in (dimsLst.clone()).into_iter().cloned() {
            let __x = ((d).len() as i32);
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
    });
    if maxn
        != ({
            let mut __acc: Option<i32> = None;
            for mut d in (dimsLst.clone()).into_iter().cloned() {
                let __x = ((d).len() as i32);
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
        })
    {
        Error::addSourceMessageAndFail(
            &(Error::NF_DIFFERENT_NUM_DIM_IN_ARGUMENTS.clone()),
            list![
                stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<_> = metamodelica::nil();
                        for mut d in (dimsLst.clone()).into_iter().cloned() {
                            let __x = ArcStr::from(::std::format!("{}", ((d).len() as i32)));
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(", ")
                ),
                literal!("cat")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if n < 1 || n > maxn {
        Error::addSourceMessageAndFail(
            &(Error::NF_CAT_WRONG_DIMENSION.clone()),
            list![
                ArcStr::from(::std::format!("{}", maxn)),
                ArcStr::from(::std::format!("{}", n))
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    tys2 = tys;
    tys3 = metamodelica::nil();
    args2 = metamodelica::nil();
    pos = ((args).len() as i32) + 2;
    for mut arg in &**args {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tys2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa2);
        tys2 = metamodelica::Own::own(__pa3);
        pos = pos - 1;
        ty2 = Type::setArrayElementType(&ty, &resTy);
        (arg2, ty1, mk) =
            TypeCheck::matchTypes(ty.clone(), ty2.clone(), arg.clone(), TypeCheck::ALLOW_UNKNOWN.clone())?;
        if TypeCheck::isIncompatibleMatch(mk) {
            Error::addSourceMessageAndFail(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    ArcStr::from(::std::format!("{}", pos)),
                    literal!("cat"),
                    literal!("arg"),
                    Expression::toString(arg.clone())?,
                    Type::toString(&ty)?,
                    Type::toString(&ty2)?
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        args2 = metamodelica::cons(arg2, args2);
        tys3 = metamodelica::cons(ty1, tys3);
    }
    resTy = crate::NFType::interned_UNKNOWN();
    tys2 = tys3.clone();
    for mut arg in &*args2 {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(tys2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa4);
        tys2 = metamodelica::Own::own(__pa5);
        if Type::isEqual(&resTy, &(crate::NFType::interned_UNKNOWN()))? {
            resTy = ty;
        } else {
            (_, _, ty1, mk) = TypeCheck::matchExpressions(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                ty,
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                resTy.clone(),
                TypeCheck::DEFAULT_OPTIONS.clone(),
            )?;
            if TypeCheck::isCompatibleMatch(mk) {
                resTy = ty1;
            }
        }
    }
    dims = Type::arrayDims(resTy.clone());
    resTyToMatch = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: Type::arrayElementType(&resTy),
        dimensions: List::set(dims, n, crate::NFDimension::interned_UNKNOWN())?,
    });
    dims = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
        for mut lst in (dimsLst).into_iter().cloned() {
            let __x = (lst).get(n)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    sumDim = Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone());
    for mut d in &*dims {
        sumDim = Dimension::add(&sumDim, metamodelica::AsArg::as_arg(&d));
    }
    resTy = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: Type::arrayElementType(&resTy),
        dimensions: List::set(Type::arrayDims(resTy), n, sumDim)?,
    });
    tys2 = tys3;
    tys3 = metamodelica::nil();
    res = metamodelica::nil();
    pos = ((args).len() as i32) + 2;
    for mut arg in &*args2 {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(tys2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa6);
        tys2 = metamodelica::Own::own(__pa7);
        pos = pos - 1;
        (arg2, ty1, mk) = TypeCheck::matchTypes(
            ty.clone(),
            resTyToMatch.clone(),
            arg.clone(),
            TypeCheck::ALLOW_UNKNOWN.clone(),
        )?;
        if TypeCheck::isIncompatibleMatch(mk) {
            Error::addSourceMessageAndFail(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    ArcStr::from(::std::format!("{}", pos)),
                    literal!("cat"),
                    literal!("arg"),
                    Expression::toString(arg.clone())?,
                    Type::toString(&ty)?,
                    Type::toString(&resTyToMatch)?
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        res = metamodelica::cons(arg2, res);
        tys3 = metamodelica::cons(ty1, tys3);
    }
    ty = resTy.clone();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::CAT().clone(),
            metamodelica::cons(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: n }),
                res,
            ),
            variability,
            purity,
            resTy,
        ),
    });
    Ok((callExp, ty))
}

fn assertNoNamedParams(
    mut fnName: ArcStr,
    mut namedArgs: &metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>,
    mut info: &SourceInfo,
) -> Result<()> {
    if !((namedArgs).is_empty()) {
        Error::addSourceMessage(
            &(Error::NO_SUCH_INPUT_PARAMETER.clone()),
            list![fnName, Util::tuple21((namedArgs).head().cloned()?)],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

fn typeStringCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut arg_ty: metamodelica::Ref<Type::NFType>;
    let mut args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut named_args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut arg: metamodelica::Ref<TypedArg>;
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(Call::typeNormalCall(call, context, &info)?) {
        __pa2 @ Deref @ Call::ARG_TYPED_CALL { r#ref: _, positional_args: __pa0, named_args: __pa1, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    args = metamodelica::Own::own(__pa0);
    named_args = metamodelica::Own::own(__pa1);
    ty_call = metamodelica::Own::own(__pa2);
    arg = (args).head().cloned()?;
    arg_ty = Type::arrayElementType(&arg.ty);
    if Type::isComplex(&arg_ty) {
        (callExp, outType, var, purity) = typeOverloadedStringCall(&arg_ty, args, &named_args, ty_call, context, info)?;
    } else {
        (callExp, outType, var, purity) = typeBuiltinStringCall(ty_call, context, info)?;
    }
    Ok((callExp, outType, var, purity))
}

fn typeBuiltinStringCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    ty_call = Call::matchTypedNormalCall(call, context, info, true)?;
    ty = Call::typeOf(&ty_call);
    var = Call::variability(&ty_call)?;
    purity = Call::purity(&ty_call);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call });
    Ok((callExp, ty, var, purity))
}

fn typeOverloadedStringCall(
    mut overloadedType: &metamodelica::Ref<Type::NFType>,
    mut args: metamodelica::List<metamodelica::Ref<TypedArg>>,
    mut namedArgs: &metamodelica::List<metamodelica::Ref<TypedArg>>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut candidates: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut recopnode: metamodelica::Ref<InstNode::InstNode>;
    let mut matchedFunc: metamodelica::Ref<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    let mut exactMatches: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    recopnode = Type::complexNode(overloadedType)?;
    if let Ok(__iflet0) = Function::lookupFunctionSimple(literal!("'String'"), recopnode.clone(), context) {
        fn_ref = __iflet0;
    } else {
        typeBuiltinStringCall(call.clone(), context, info.clone())?;
        return Err("fail");
    }
    (fn_ref, _, _) = Function::instFunctionRef(fn_ref, context, NFInstNode::InstNode::info(&recopnode))?;
    candidates = Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?;
    matchedFunctions = Function::matchFunctionsSilent(&candidates, args, namedArgs, context, &info, true)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if (exactMatches).is_empty() {
        Error::addSourceMessage(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                Call::typedString(&call)?,
                Function::candidateFuncListString(candidates)?
            ],
            &info,
        )?;
        return Err("fail");
    }
    if ((exactMatches).len() as i32) == 1 {
        let __pa1 = ::match_deref::match_deref! { match &(exactMatches) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        matchedFunc = metamodelica::Own::own(__pa1);
        outType = Function::returnType(&matchedFunc.func);
        for mut arg in &*matchedFunc.args.clone() {
            var = Prefixes::variabilityMax(var, arg.var.clone());
            purity = Prefixes::purityMin(purity, arg.purity.clone());
        }
        callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                matchedFunc.func.clone(),
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut a in (matchedFunc.args.clone()).into_iter().cloned() {
                        let __x = a.value.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                var,
                purity,
                outType.clone(),
            ),
        });
        return Ok((callExp, outType, var, purity));
    } else {
        Error::addSourceMessage(
            &(Error::AMBIGUOUS_MATCHING_FUNCTIONS_NFINST.clone()),
            list![
                Call::typedString(&call)?,
                Function::candidateFuncListString(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
                        for mut mfn in (matchedFunctions).into_iter().cloned() {
                            let __x = mfn.func.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                )?
            ],
            &info,
        )?;
        return Err("fail");
    }
    Ok((callExp, outType, var, purity))
}

fn typeDiscreteCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity;
    let mut argtycall: metamodelica::Ref<Call::NFCall>;
    argtycall = Call::typeMatchNormalCall(call, context, info, true)?;
    ty = Call::typeOf(&argtycall);
    purity = Call::purity(&argtycall);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::unboxArgs(argtycall),
    });
    Ok((callExp, ty, var, purity))
}

fn typeNdimsCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_INTEGER();
    let mut variability: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg_ty: metamodelica::Ref<Type::NFType>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { arguments: __pa0, named_args: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    args = metamodelica::Own::own(__pa0);
    named_args = metamodelica::Own::own(__pa1);
    assertNoNamedParams(literal!("ndims"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessage(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("ndims(Any) => Integer")],
            info,
        )?;
        return Err("fail");
    }
    (_, arg_ty, _, _) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    callExp = metamodelica::Ref::new(Expression::NFExpression::INTEGER {
        value: Type::dimensionCount(arg_ty),
    });
    Ok((callExp, ty, variability, purity))
}

fn typePreCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    (callExp, ty, variability, purity) = typePreChangeCall(literal!("pre"), call, context, info)?;
    Ok((callExp, ty, variability, purity))
}

fn typeChangeCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    (callExp, ty, variability, purity) = typePreChangeCall(literal!("change"), call, context, info)?;
    ty = Type::setArrayElementType(&ty, &(crate::NFType::interned_BOOLEAN()));
    Ok((callExp, ty, variability, purity))
}

fn typePreChangeCall(
    mut name: ArcStr,
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(name, &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Any) => Any"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, var, purity) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    if !(Expression::isCref(&arg)) {
        Error::addSourceMessage(
            &(Error::ARGUMENT_MUST_BE_VARIABLE.clone()),
            list![
                literal!("First"),
                ComponentRef::toString(&fn_ref)?,
                literal!("<REMOVE ME>")
            ],
            info,
        )?;
        return Err("fail");
    }
    if var == Variability::CONTINUOUS.clone() {
        Error::addSourceMessageAndFail(
            &(Error::INVALID_ARGUMENT_VARIABILITY.clone()),
            list![
                literal!("1"),
                ComponentRef::toString(&fn_ref)?,
                Prefixes::variabilityString(Variability::DISCRETE.clone())?,
                Expression::toString(arg.clone())?,
                Prefixes::variabilityString(var)?
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg], var, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeDerCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut ety: metamodelica::Ref<Type::NFType>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    if InstContext::inFunction(context) {
        Error::addSourceMessage(&(Error::EXP_INVALID_IN_FUNCTION.clone()), list![literal!("der")], info)?;
        return Err("fail");
    }
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("der"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("der(Real) => Real")],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa3);
    (arg, ty, variability, purity) = Typing::typeExp(arg, arg_context, info, false)?;
    ety = Type::arrayElementType(&ty);
    if Type::isInteger(&ety)? {
        if variability < Variability::DISCRETE.clone() {
            ty = Type::setArrayElementType(&ty, &(crate::NFType::interned_REAL()));
            arg = Expression::typeCast(arg, crate::NFType::interned_REAL())?;
        } else {
            Error::addSourceMessageAndFail(
                &(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone()),
                list![Expression::toString(arg.clone())?],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    } else if !(Type::isReal(&ety)?) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("1"),
                ComponentRef::toString(&fn_ref)?,
                literal!(""),
                Expression::toString(arg.clone())?,
                Type::toString(&ty)?,
                literal!("Real")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if variability == Variability::DISCRETE.clone() && !(InstContext::inDiscreteScope(context)) {
        Error::addSourceMessageAndFail(
            &(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone()),
            list![Expression::toString(arg.clone())?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa5);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeEdgeCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity;
    let mut argtycall: metamodelica::Ref<Call::NFCall>;
    let mut args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut arg: metamodelica::Ref<TypedArg>;
    let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if InstContext::inFunction(context) {
        Error::addSourceMessage(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![literal!("edge")],
            &info,
        )?;
        return Err("fail");
    }
    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(Call::typeNormalCall(call, context, &info)?) {
        __pa2 @ Deref @ Call::ARG_TYPED_CALL { r#ref: __pa0 @ Deref @ ComponentRef::CREF { .. }, positional_args: __pa1, named_args: _, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    argtycall = metamodelica::Own::own(__pa2);
    fn_node = ComponentRef::node(&fn_ref)?;
    argtycall = Call::matchTypedNormalCall(argtycall, context, info.clone(), true)?;
    ty = Call::typeOf(&argtycall);
    purity = Call::purity(&argtycall);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::unboxArgs(argtycall),
    });
    let __pa4 = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } => __pa4.clone(),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa4);
    if !(Expression::isCref(&arg.value)) {
        Error::addSourceMessage(
            &(Error::ARGUMENT_MUST_BE_VARIABLE.clone()),
            list![literal!("First"), literal!("edge"), literal!("<REMOVE ME>")],
            &info,
        )?;
        return Err("fail");
    }
    Ok((callExp, ty, variability, purity))
}

fn typeMinMaxCall(
    mut name: ArcStr,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    fn is_valid_type(mut ty: &metamodelica::Ref<Type::NFType>) -> bool {
        let mut res: bool;
        res = (match &**ty {
            Type::REAL => true,
            Type::INTEGER => true,
            Type::BOOLEAN => true,
            Type::ENUMERATION { .. } => true,
            _ => false,
        });
        res
    }

    fn invalid_args_error(
        mut call: &metamodelica::Ref<Call::NFCall>,
        mut name: &ArcStr,
        mut info: &SourceInfo,
    ) -> Result<()> {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(Real, Real) => Real\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(Integer, Integer) => Integer\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(Boolean, Boolean) => Boolean\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(enumeration(:), enumeration(:)) => enumeration(:)\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(Real[:, ...]) => Real\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(Integer[:, ...]) => Integer\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(Boolean[:, ...]) => Boolean\n  "));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("(enumeration(:)[:, ...]) => enumeration(:)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        Ok(())
    }

    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut var1: Variability;
    let mut var2: Variability;
    let mut pur1: Purity;
    let mut pur2: Purity;
    let mut mk: TypeCheck::MatchKind;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(name.clone(), &named_args, &info)?;
    (args, ty, var, purity) = (::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
            arg1 = (*__esc_arg1).clone();
            (arg1, ty1, var, purity) = Typing::typeExp(arg1.clone(), arg_context, &info, false)?;
            ty = Type::arrayElementType(&ty1);
            if !(Type::isArray(&ty1) && is_valid_type(&ty)) {
                invalid_args_error(&call, &name, &info)?;
            }
            if Type::isSingleElementArray(&ty1)? {
                callExp = Expression::applySubscript(&(Subscript::first(&(((Type::arrayDims(ty1))).head().cloned()?))?), metamodelica::AsArg::as_arg(&arg1), &(metamodelica::nil()), false)?;
                return Ok((callExp, ty, var, purity));
            }
            (list![arg1.clone()], ty, var, purity)
        },
        Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            arg1 = (*__esc_arg1).clone();
            arg2 = (*__esc_arg2).clone();
            (arg1, ty1, var1, pur1) = Typing::typeExp(arg1.clone(), arg_context, &info, false)?;
            (arg2, ty2, var2, pur2) = Typing::typeExp(arg2.clone(), arg_context, &info, false)?;
            if !(is_valid_type(&ty1) && is_valid_type(&ty2)) {
                invalid_args_error(&call, &name, &info)?;
            }
            (arg1, arg2, ty, mk) = TypeCheck::matchExpressions(arg1.clone(), ty1, arg2.clone(), ty2, TypeCheck::DEFAULT_OPTIONS.clone())?;
            if !(TypeCheck::isValidArgumentMatch(mk)) {
                invalid_args_error(&call, &name, &info)?;
            }
            (list![arg1.clone(), arg2.clone()], ty, Prefixes::variabilityMax(var1, var2), Prefixes::purityMin(pur1, pur2))
        },
        _ => {
            invalid_args_error(&call, &name, &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#fn = (Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?)
        .head()
        .cloned()?;
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, args, var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typePromoteCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut exp_arg: metamodelica::Ref<Expression::NFExpression>;
    let mut n_arg: metamodelica::Ref<Expression::NFExpression>;
    let mut exp_ty: metamodelica::Ref<Type::NFType>;
    let mut n_ty: metamodelica::Ref<Type::NFType>;
    let mut n_var: Variability;
    let mut n: i32;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("promote"), &named_args, &info)?;
    if ((args).len() as i32) != 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                Call::toString(call)?,
                literal!("promote(Any[...], Integer) => Any[...]")
            ],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp_arg = metamodelica::Own::own(__pa3);
    n_arg = metamodelica::Own::own(__pa4);
    (exp_arg, exp_ty, variability, purity) = Typing::typeExp(exp_arg, arg_context, &info, false)?;
    (n_arg, n_ty, n_var, _) = Typing::typeExp(n_arg, arg_context, &info, false)?;
    if !(Type::isInteger(&n_ty)?) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("2"),
                literal!("promote"),
                literal!(""),
                Expression::toString(n_arg.clone())?,
                Type::toString(&n_ty)?,
                literal!("Integer")
            ],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if n_var > Variability::CONSTANT.clone() {
        Error::addSourceMessageAndFail(
            &(Error::INVALID_ARGUMENT_VARIABILITY.clone()),
            list![
                literal!("2"),
                literal!("promote"),
                Prefixes::variabilityString(Variability::CONSTANT.clone())?,
                Expression::toString(n_arg.clone())?,
                Prefixes::variabilityString(n_var)?
            ],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    n_arg = Ceval::evalExp(n_arg, &(Ceval::EvalTarget::new(info.clone(), arg_context, None)))?;
    n = Expression::integerValue(n_arg.clone())?;
    if n < Type::dimensionCount(exp_ty.clone()) {
        Error::addSourceMessageAndFail(
            &(Error::INVALID_NUMBER_OF_DIMENSIONS_FOR_PROMOTE.clone()),
            list![
                ArcStr::from(::std::format!("{}", n)),
                ArcStr::from(::std::format!("{}", Type::dimensionCount(exp_ty)))
            ],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (callExp, ty) = Expression::promote(
        exp_arg.clone(),
        Expression::typeOf(exp_arg),
        Expression::integerValue(n_arg)?,
    )?;
    Ok((callExp, ty, variability, purity))
}

fn typeSmoothCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut mk: TypeCheck::MatchKind;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("smooth"), &named_args, info)?;
    if ((args).len() as i32) != 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("smooth(Integer, Any) => Any")],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg1 = metamodelica::Own::own(__pa3);
    arg2 = metamodelica::Own::own(__pa4);
    (arg1, ty1, var, _) = Typing::typeExp(arg1, arg_context, info, false)?;
    (arg2, ty2, variability, purity) = Typing::typeExp(arg2, arg_context, info, false)?;
    if !(Type::isInteger(&ty1)?) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("1"),
                ComponentRef::toString(&fn_ref)?,
                literal!(""),
                Expression::toString(arg1.clone())?,
                Type::toString(&ty1)?,
                literal!("Integer")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if var > Variability::PARAMETER.clone() {
        Error::addSourceMessageAndFail(
            &(Error::INVALID_ARGUMENT_VARIABILITY.clone()),
            list![
                literal!("1"),
                ComponentRef::toString(&fn_ref)?,
                Prefixes::variabilityString(Variability::PARAMETER.clone())?,
                Expression::toString(arg1.clone())?,
                Prefixes::variabilityString(variability)?
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg2, ty, mk) = TypeCheck::matchTypes(
        ty2.clone(),
        Type::setArrayElementType(&ty2, &(crate::NFType::interned_REAL())),
        arg2,
        TypeCheck::ALLOW_UNKNOWN.clone(),
    )?;
    if !(TypeCheck::isValidArgumentMatch(mk)) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("2"),
                ComponentRef::toString(&fn_ref)?,
                literal!(""),
                Expression::toString(arg2.clone())?,
                Type::toString(&ty2)?,
                literal!("Real\n  Real[:, ...]\n  Real record\n  Real record[:, ...]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa6 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil } => __pa6.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa6);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg1, arg2], var, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeFillCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut fill_arg: metamodelica::Ref<Expression::NFExpression>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("fill"), &named_args, info)?;
    if ((args).len() as i32) < 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                Call::toString(call)?,
                literal!("fill(Any, Integer, ...) => Any[:, ...]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fill_arg = metamodelica::Own::own(__pa3);
    args = metamodelica::Own::own(__pa4);
    (fill_arg, ty, variability, purity) = Typing::typeExp(fill_arg, arg_context, info, false)?;
    (callExp, ty, variability, purity) =
        typeFillCall2(&fn_ref, ty, fill_arg, variability, purity, args, arg_context, info)?;
    Ok((callExp, ty, variability, purity))
}

fn typeFillCall2(
    mut fnRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut fillType: metamodelica::Ref<Type::NFType>,
    mut fillArg: metamodelica::Ref<Expression::NFExpression>,
    mut fillVariability: Variability,
    mut fillPurity: Purity,
    mut dimensionArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability = fillVariability;
    let mut purity: Purity = fillPurity;
    let mut ty_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut arg_var: Variability;
    let mut arg_pur: Purity;
    let mut arg_ty: metamodelica::Ref<Type::NFType>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut index: i32 = 1;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    ty_args = list![fillArg.clone()];
    dims = metamodelica::nil();
    for mut arg in &*dimensionArgs {
        let mut arg = arg.clone();
        (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg, arg_context, info, false)?;
        if !(InstContext::inAlgorithm(context) || InstContext::inFunction(context)) {
            if arg_var > Variability::PARAMETER.clone()
                && !(InstContext::inInstanceAPI(context)
                    || Expression::contains(arg.clone(), &move |__a0: metamodelica::Ref<
                        Expression::NFExpression,
                    >| Expression::isResizableCref(&__a0))?)
            {
                Error::addSourceMessageAndFail(
                    &(Error::NON_PARAMETER_EXPRESSION_DIMENSION.clone()),
                    list![
                        Expression::toString(arg.clone())?,
                        ArcStr::from(::std::format!("{}", index)),
                        List::toStringCustom(
                            metamodelica::cons(fillArg.clone(), dimensionArgs.clone()),
                            &Expression::toString,
                            ComponentRef::toString(fnRef)?,
                            literal!("("),
                            literal!(", "),
                            literal!(")"),
                            true,
                            0
                        )?
                    ],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if arg_pur == Purity::PURE.clone() && !(Structural::isExpressionNotFixed(&arg, false, 4)?) {
                Structural::markExp(&arg)?;
                arg = if (InstContext::inInstanceAPI(context)) {
                    Ceval::tryEvalExp(arg, &(Ceval::noTarget().clone()))
                } else {
                    Ceval::tryEvalExpResizable(arg, &(Ceval::noTarget().clone()))?
                };
                arg_ty = Expression::typeOf(arg.clone());
            }
        }
        if !(Type::isInteger(&arg_ty)?) {
            Error::addSourceMessageAndFail(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    intString(((ty_args).len() as i32) + 1),
                    ComponentRef::toString(fnRef)?,
                    literal!(""),
                    Expression::toString(arg.clone())?,
                    Type::toString(&arg_ty)?,
                    literal!("Integer")
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        variability = Prefixes::variabilityMax(variability, arg_var);
        purity = Prefixes::purityMin(purity, arg_pur);
        ty_args = metamodelica::cons(arg.clone(), ty_args);
        dims = metamodelica::cons(Dimension::fromExp(arg, arg_var)?, dims);
        index = index + 1;
    }
    ty_args = metamodelica::Dangerous::listReverseInPlace(ty_args);
    dims = metamodelica::Dangerous::listReverseInPlace(dims);
    let __pa0 = ::match_deref::match_deref! { match &(Function::typeRefCache(fnRef, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa0);
    ty = Type::liftArrayLeftList(fillType, &dims);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::FILL_FUNC().clone(),
            ty_args,
            variability,
            purity,
            ty.clone(),
        ),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeZerosOnesCall(
    mut name: ArcStr,
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut fill_arg: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(name.clone(), &named_args, info)?;
    if (args).is_empty() {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Integer, ...) => Integer[:, ...]"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    fill_arg = metamodelica::Ref::new(Expression::NFExpression::INTEGER {
        value: if (metamodelica::stringEq(&name, &(literal!("ones")))) {
            1
        } else {
            0
        },
    });
    (callExp, ty, variability, purity) = typeFillCall2(
        &fn_ref,
        crate::NFType::interned_INTEGER(),
        fill_arg,
        Variability::CONSTANT.clone(),
        Purity::PURE.clone(),
        args,
        context,
        info,
    )?;
    Ok((callExp, ty, variability, purity))
}

fn typeScalarCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut expanded: bool;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("scalar"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("scalar(Any[1, ...]) => Any")],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    for mut dim in &*Type::arrayDims(ty.clone()) {
        if Dimension::isKnown(metamodelica::AsArg::as_arg(&dim), false)
            && !(Dimension::size(metamodelica::AsArg::as_arg(&dim), false)? == 1)
        {
            Error::addSourceMessageAndFail(
                &(Error::INVALID_ARRAY_DIM_IN_SCALAR_OP.clone()),
                list![Type::toString(&ty)?],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    (arg, expanded) = ExpandExp::expand(arg, false, false)?;
    ty = Type::arrayElementType(&ty);
    if expanded {
        args = Expression::arrayScalarElements(&arg);
        if ((args).len() as i32) != 1 {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFBuiltinCall.typeScalarCall"));
                    __mm_s.push_str(&*literal!(" failed to expand scalar("));
                    __mm_s.push_str(&*Expression::toString(arg)?);
                    __mm_s.push_str(&*literal!(") correctly"));
                    ArcStr::from(__mm_s)
                },
                info,
            )?;
        }
        callExp = (args).head().cloned()?;
    } else {
        let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
            _ => return Err("pattern mismatch"),
        } };
        r#fn = metamodelica::Own::own(__pa3);
        callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
        });
    }
    Ok((callExp, ty, variability, purity))
}

fn typeVectorCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut vector_dim: metamodelica::Ref<Dimension::NFDimension> =
        Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
    let mut dim_found: bool = false;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("vector"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                Call::toString(call)?,
                literal!("vector(Any) => Any[:]\n  vector(Any[:, ...]) => Any[:]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    for mut dim in &*Type::arrayDims(ty.clone()) {
        if !(Dimension::isKnown(metamodelica::AsArg::as_arg(&dim), false))
            || Dimension::size(metamodelica::AsArg::as_arg(&dim), false)? > 1
        {
            if dim_found {
                Error::addSourceMessageAndFail(
                    &(Error::NF_VECTOR_INVALID_DIMENSIONS.clone()),
                    list![Type::toString(&ty)?, Call::toString(call)?],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else {
                vector_dim = dim.clone();
                dim_found = true;
            }
        }
    }
    if Type::isEmptyArray(&ty)? {
        vector_dim = Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone());
    }
    ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: Type::arrayElementType(&ty),
        dimensions: list![vector_dim],
    });
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeMatrixCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dim1: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim2: metamodelica::Ref<Dimension::NFDimension>;
    let mut i: i32;
    let mut ndims: i32;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("matrix"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                Call::toString(call)?,
                literal!("matrix(Any) => Any[:]\n  matrix(Any[:, ...]) => Any[:]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    dims = Type::arrayDims(ty.clone());
    ndims = ((dims).len() as i32);
    if ndims < 2 {
        (callExp, ty) = Expression::promote(arg, ty, 2)?;
    } else if ndims == 2 {
        callExp = arg;
    } else {
        let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(dims) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim1 = metamodelica::Own::own(__pa3);
        dim2 = metamodelica::Own::own(__pa4);
        dims = metamodelica::Own::own(__pa5);
        i = 3;
        for mut dim in &*dims {
            if Dimension::isKnown(metamodelica::AsArg::as_arg(&dim), false)
                && Dimension::size(metamodelica::AsArg::as_arg(&dim), false)? > 1
            {
                Error::addSourceMessageAndFail(
                    &(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone()),
                    list![
                        ArcStr::from(::std::format!("{}", i)),
                        literal!("matrix"),
                        literal!("1"),
                        Dimension::toString(metamodelica::AsArg::as_arg(&dim))?
                    ],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            i = i + 1;
        }
        ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: Type::arrayElementType(&ty),
            dimensions: list![dim1, dim2],
        });
        let __pa7 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } => __pa7.clone(),
            _ => return Err("pattern mismatch"),
        } };
        r#fn = metamodelica::Own::own(__pa7);
        callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
        });
    }
    Ok((callExp, ty, variability, purity))
}

fn typeCatCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut res: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut pur: Purity;
    let mut mk: TypeCheck::MatchKind;
    let mut n: i32;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("cat"), &named_args, &info)?;
    if ((args).len() as i32) < 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("cat(Integer, Any[:,:], ...) => Any[:]")],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa3);
    args = metamodelica::Own::own(__pa4);
    (arg, ty, variability, purity) = Typing::typeExp(arg, arg_context, &info, false)?;
    (arg, ty, mk) = TypeCheck::matchTypes(
        ty,
        crate::NFType::interned_INTEGER(),
        arg,
        TypeCheck::DEFAULT_OPTIONS.clone(),
    )?;
    if variability > Variability::PARAMETER.clone() || purity != Purity::PURE.clone() {
        Error::addSourceMessageAndFail(
            &(Error::NF_CAT_FIRST_ARG_EVAL.clone()),
            list![
                Expression::toString(arg.clone())?,
                Prefixes::variabilityString(variability)?
            ],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa5 = ::match_deref::match_deref! { match &(Ceval::evalExp(arg, &(Ceval::EvalTarget::new(info.clone(), arg_context, None)))?) {
        Deref @ Expression::INTEGER { value: __pa5 } => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa5);
    res = metamodelica::nil();
    tys = metamodelica::nil();
    for mut a in &*args {
        (arg, ty, var, pur) = Typing::typeExp(a.clone(), arg_context, &info, false)?;
        variability = Prefixes::variabilityMax(var, variability);
        purity = Prefixes::purityMin(pur, purity);
        res = metamodelica::cons(arg, res);
        tys = metamodelica::cons(ty, tys);
    }
    (callExp, ty) = makeCatExp(n, &(res.reverse()), tys.reverse(), variability, purity, &info)?;
    Ok((callExp, ty, variability, purity))
}

fn typeSymmetricCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("symmetric"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("symmetric(Any[n, n]) => Any[n, n]")],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    if !(Type::isSquareMatrix(&ty)?) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("1"),
                ComponentRef::toString(&fn_ref)?,
                literal!(""),
                Expression::toString(arg.clone())?,
                Type::toString(&ty)?,
                literal!("Any[n, n]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeTransposeCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut dim1: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim2: metamodelica::Ref<Dimension::NFDimension>;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("transpose"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![
                Call::toString(call)?,
                literal!("transpose(Any[n, m, ...]) => Any[m, n, ...]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, variability, purity) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: __esc_dim1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim2, tail: __esc_rest_dims } }, elementType: __ty_elementType } => {
            dim1 = (*__esc_dim1).clone();
            dim2 = (*__esc_dim2).clone();
            rest_dims = (*__esc_rest_dims).clone();
            metamodelica::Ref::new(Type::NFType::ARRAY { elementType: __ty_elementType.clone(), dimensions: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) })
        },
        _ => {
            Error::addSourceMessage(&(Error::ARG_TYPE_MISMATCH.clone()), list![literal!("1"), ComponentRef::toString(&fn_ref)?, literal!(""), Expression::toString(arg.clone())?, Type::toString(&ty)?, literal!("Any[:, :, ...]")], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeCardinalityCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    if !(InstContext::inCondition(context) && (InstContext::inIf(context) || InstContext::inAssert(context))) {
        Error::addSourceMessageAndFail(
            &(Error::INVALID_CARDINALITY_CONTEXT.clone()),
            metamodelica::nil(),
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![AbsynUtil::pathString(
                Call::functionName(&call)?,
                literal!("."),
                true,
                false
            )?],
            &info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (callExp, ty, _, _) = typeBuiltinCallExp(call, context, info, false)?;
    System::setUsesCardinality(true);
    Ok((callExp, ty, var, purity))
}

fn typeConnectionsArgs(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut context: i32,
    mut info: &SourceInfo,
    mut fnRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut index: i32 = 1;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    for mut arg in &**args {
        outArgs = metamodelica::cons(
            (typeConnectionsArg(arg.clone(), arg_context, info, fnRef, index)?).0,
            outArgs,
        );
        index = index + 1;
    }
    outArgs = metamodelica::Dangerous::listReverseInPlace(outArgs);
    Ok(outArgs)
}

fn typeConnectionsArg(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: &SourceInfo,
    mut fnRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut index: i32,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outArg: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    (outArg, outType, _, _) = Typing::typeExp(arg, context, info, false)?;
    checkConnectionsArgument(outArg.clone(), &outType, fnRef, index, info)?;
    Ok((outArg, outType))
}

fn typeBranchCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("Connections.branch"), &named_args, info)?;
    if ((args).len() as i32) != 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector, Connector)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(&args, arg_context, info, &fn_ref)?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    ty = crate::NFType::interned_NORETCALL();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, args, var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typeIsRootCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("Connections.isRoot"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(&args, arg_context, info, &fn_ref)?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    ty = crate::NFType::interned_BOOLEAN();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, args, var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typePotentialRootCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args_len: i32;
    let mut name: ArcStr;
    let mut arg_var: Variability;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    for mut narg in &*named_args {
        (name, arg2) = narg.clone();
        if metamodelica::stringEq(&name, &(literal!("priority"))) {
            args = List::appendElt(arg2, args);
        } else {
            Error::addSourceMessageAndFail(
                &(Error::NO_SUCH_INPUT_PARAMETER.clone()),
                list![ComponentRef::toString(&fn_ref)?, name],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    args_len = ((args).len() as i32);
    if args_len < 1 || args_len > 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector, Integer = 0)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg1 = metamodelica::Own::own(__pa3);
    args = metamodelica::Own::own(__pa4);
    (arg1, _) = typeConnectionsArg(arg1, arg_context, info, &fn_ref, 1)?;
    if args_len == 2 {
        arg2 = (args).head().cloned()?;
        (arg2, ty, arg_var, _) = Typing::typeExp(arg2, arg_context, info, false)?;
        if !(Type::isInteger(&ty)?) {
            Error::addSourceMessageAndFail(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    literal!("2"),
                    ComponentRef::toString(&fn_ref)?,
                    literal!(""),
                    Expression::toString(arg2.clone())?,
                    Type::toString(&ty)?,
                    literal!("Integer")
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        if arg_var > Variability::PARAMETER.clone() {
            Error::addSourceMessageAndFail(
                &(Error::INVALID_ARGUMENT_VARIABILITY.clone()),
                list![
                    literal!("2"),
                    ComponentRef::toString(&fn_ref)?,
                    Prefixes::variabilityString(Variability::PARAMETER.clone())?,
                    Expression::toString(arg2.clone())?,
                    Prefixes::variabilityString(arg_var)?
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        Structural::markExp(&arg2)?;
    } else {
        arg2 = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
    }
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa5);
    ty = crate::NFType::interned_NORETCALL();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg1, arg2], var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typeRootCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("Connections.root"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(&args, arg_context, info, &fn_ref)?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    ty = crate::NFType::interned_NORETCALL();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, args, var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typeRootedCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("Connections.rooted"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    args = typeConnectionsArgs(&args, arg_context, info, &fn_ref)?;
    if ComponentRef::isSimple(&fn_ref) {
        Error::addSourceMessage(
            &(Error::DEPRECATED_API_CALL.clone()),
            list![literal!("rooted"), literal!("Connections.rooted")],
            info,
        )?;
    }
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    ty = crate::NFType::interned_BOOLEAN();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, args, var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typeUniqueRootCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args_len: i32;
    let mut name: ArcStr;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    Error::addSourceMessage(
        &(Error::NON_STANDARD_OPERATOR.clone()),
        list![literal!("Connections.uniqueRoot")],
        info,
    )?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    for mut narg in &*named_args {
        (name, arg2) = narg.clone();
        if metamodelica::stringEq(&name, &(literal!("message"))) {
            args = List::appendElt(arg2, args);
        } else {
            Error::addSourceMessageAndFail(
                &(Error::NO_SUCH_INPUT_PARAMETER.clone()),
                list![ComponentRef::toString(&fn_ref)?, name],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    args_len = ((args).len() as i32);
    if args_len < 1 || args_len > 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector, String = \"\")"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg1 = metamodelica::Own::own(__pa3);
    args = metamodelica::Own::own(__pa4);
    (arg1, _) = typeConnectionsArg(arg1, arg_context, info, &fn_ref, 1)?;
    if args_len == 2 {
        arg2 = (args).head().cloned()?;
        (arg2, ty, _, _) = Typing::typeExp(arg2, arg_context, info, false)?;
        if !(Type::isString(&ty)?) {
            Error::addSourceMessageAndFail(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    literal!("2"),
                    ComponentRef::toString(&fn_ref)?,
                    literal!(""),
                    Expression::toString(arg2.clone())?,
                    Type::toString(&ty)?,
                    literal!("String")
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    } else {
        arg2 = metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") });
    }
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa5);
    ty = crate::NFType::interned_NORETCALL();
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg1, arg2], var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn typeUniqueRootIndicesCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut arg3: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args_len: i32;
    let mut name: ArcStr;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut ty3: metamodelica::Ref<Type::NFType>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    Error::addSourceMessage(
        &(Error::NON_STANDARD_OPERATOR.clone()),
        list![literal!("Connections.uniqueRootIndices")],
        info,
    )?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    for mut narg in &*named_args {
        (name, arg3) = narg.clone();
        if metamodelica::stringEq(&name, &(literal!("message"))) {
            args = List::appendElt(arg3, args);
        } else {
            Error::addSourceMessageAndFail(
                &(Error::NO_SUCH_INPUT_PARAMETER.clone()),
                list![ComponentRef::toString(&fn_ref)?, name],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    args_len = ((args).len() as i32);
    if args_len < 2 || args_len > 3 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(Connector, Connector, String = \"\")"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if InstContext::inFunction(context) {
        Error::addSourceMessageAndFail(
            &(Error::EXP_INVALID_IN_FUNCTION.clone()),
            list![ComponentRef::toString(&fn_ref)?],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg1 = metamodelica::Own::own(__pa3);
    arg2 = metamodelica::Own::own(__pa4);
    args = metamodelica::Own::own(__pa5);
    (arg1, ty1) = typeConnectionsArg(arg1, arg_context, info, &fn_ref, 1)?;
    if !(Type::isArray(&ty1)) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("1"),
                ComponentRef::toString(&fn_ref)?,
                literal!(""),
                Expression::toString(arg1.clone())?,
                Type::toString(&ty1)?,
                literal!("Connector[:]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg2, ty2) = typeConnectionsArg(arg2, arg_context, info, &fn_ref, 2)?;
    if !(Type::isArray(&ty2)) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                literal!("2"),
                ComponentRef::toString(&fn_ref)?,
                literal!(""),
                Expression::toString(arg2.clone())?,
                Type::toString(&ty2)?,
                literal!("Connector[:]")
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if args_len == 3 {
        arg3 = (args).head().cloned()?;
        (arg3, ty3, _, _) = Typing::typeExp(arg3, arg_context, info, false)?;
        if !(Type::isString(&ty3)?) {
            Error::addSourceMessageAndFail(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    literal!("3"),
                    ComponentRef::toString(&fn_ref)?,
                    literal!(""),
                    Expression::toString(arg2.clone())?,
                    Type::toString(&ty3)?,
                    literal!("String")
                ],
                info,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    } else {
        arg3 = metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") });
    }
    let __pa7 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } => __pa7.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa7);
    assert!(
        ((Type::arrayDims(ty1.clone())).len() as i32) == ((Type::arrayDims(ty2)).len() as i32),
        "{}",
        &*literal!("the first two parameters need to have the same size")
    );
    ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_INTEGER(),
        dimensions: Type::arrayDims(ty1),
    });
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg1, arg2, arg3], var, purity, ty.clone()),
    });
    Ok((callExp, ty, var, purity))
}

fn checkConnectionsArgument(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut fnRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut argIndex: i32,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match &*arg {
        Expression::CREF { cref: __arg_cref, .. } => {
            let mut ty2: metamodelica::Ref<Type::NFType>;
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut valid_cref: bool;
            let mut isConnector: bool;
            (valid_cref, isConnector) = (::match_deref::match_deref! { match &(__arg_cref.clone()) {
                Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::CREF, restCref: Deref @ ComponentRef::CREF { ty: __esc_ty2, origin: ComponentRef::Origin::CREF, .. }, .. } => {
                    ty2 = (*__esc_ty2).clone();
                    node = ComponentRef::node(metamodelica::AsArg::as_arg(&__arg_cref))?;
                    ty2 = (match &*ty2.clone() {
                Type::ARRAY { dimensions: __ty2_dimensions, elementType: __ty2_elementType } if ((((ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&__arg_cref))?)).len() as i32) == ((__ty2_dimensions).len() as i32)) => __ty2_elementType.clone(),
                _ => ty2.clone(),
            });
                    (Class::isOverdetermined(NFInstNode::InstNode::getClass(node)?), Type::isConnector(metamodelica::AsArg::as_arg(&ty2)))
                },
                Deref @ ComponentRef::CREF { ty: __esc_ty2, .. } => {
                    ty2 = (*__esc_ty2).clone();
                    node = ComponentRef::node(metamodelica::AsArg::as_arg(&__arg_cref))?;
                    ty2 = (match &*ty2.clone() {
                Type::ARRAY { dimensions: __ty2_dimensions, elementType: __ty2_elementType } if ((((ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&__arg_cref))?)).len() as i32) == ((__ty2_dimensions).len() as i32)) => __ty2_elementType.clone(),
                _ => ty2.clone(),
            });
                    (Class::isOverdetermined(NFInstNode::InstNode::getClass(node)?), Type::isConnector(metamodelica::AsArg::as_arg(&ty2)))
                },
                _ => (false, false),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            if !(valid_cref && isConnector) {
                if valid_cref {
                    Error::addSourceMessage(
                        &(if (argIndex == 1) {
                            Error::W_INVALID_ARGUMENT_TYPE_BRANCH_FIRST.clone()
                        } else {
                            Error::W_INVALID_ARGUMENT_TYPE_BRANCH_SECOND.clone()
                        }),
                        list![
                            ComponentRef::toString(metamodelica::AsArg::as_arg(&__arg_cref))?,
                            ComponentRef::toString(fnRef)?
                        ],
                        info,
                    )?;
                } else {
                    Error::addSourceMessageAndFail(
                        &(if (argIndex == 1) {
                            Error::INVALID_ARGUMENT_TYPE_BRANCH_FIRST.clone()
                        } else {
                            Error::INVALID_ARGUMENT_TYPE_BRANCH_SECOND.clone()
                        }),
                        list![
                            ComponentRef::toString(metamodelica::AsArg::as_arg(&__arg_cref))?,
                            ComponentRef::toString(fnRef)?
                        ],
                        info,
                    )?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            ()
        }
        _ => {
            Error::addSourceMessage(
                &(Error::ARG_TYPE_MISMATCH.clone()),
                list![
                    ArcStr::from(::std::format!("{}", argIndex)),
                    ComponentRef::toString(fnRef)?,
                    literal!(""),
                    Expression::toString(arg)?,
                    Type::toString(ty)?,
                    literal!("overconstrained type/record")
                ],
                info,
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

fn typeNoEventCall(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(literal!("noEvent"), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, literal!("noEvent(Any) => Any")],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let __pa3 = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa3);
    arg_context = InstContext::set(context, InstContext::NOEVENT.clone());
    (arg, ty, variability, purity) = Typing::typeExp(arg, arg_context, info, false)?;
    let __pa5 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa5);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(r#fn, list![arg], variability, purity, ty.clone()),
    });
    Ok((callExp, ty, variability, purity))
}

fn typeNthRootCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut v: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut c: metamodelica::Ref<Call::NFCall>;
    c = Call::typeMatchNormalCall(call, context, info.clone(), true)?;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } }, ty: __pa2, var: __pa3, purity: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    v = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    var = metamodelica::Own::own(__pa3);
    purity = metamodelica::Own::own(__pa4);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: c });
    if Expression::isNonPositive(&n)? {
        Error::addSourceMessage(
            &(Error::NON_POSITIVE_NTH_ROOT.clone()),
            list![Expression::toString(v.clone())?, Expression::toString(n.clone())?],
            &info,
        )?;
        return Err("fail");
    }
    if Expression::isEven(&n) && Expression::isNegative(&v)? {
        Error::addSourceMessage(
            &(Error::NEGATIVE_NTH_ROOT.clone()),
            list![Expression::toString(v)?, Expression::toString(n)?],
            &info,
        )?;
        return Err("fail");
    }
    Ok((callExp, ty, var, purity))
}

fn typeGetInstanceName(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_STRING();
    let mut var: Variability = Variability::CONSTANT.clone();
    let mut purity: Purity = Purity::PURE.clone();
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let __pa0 = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::UNTYPED_CALL { call_scope: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    scope = metamodelica::Own::own(__pa0);
    Call::typeMatchNormalCall(call, context, info, true)?;
    result = metamodelica::Ref::new(Expression::NFExpression::INSTANCE_NAME {
        scope: NFInstNode::InstNode::fromCell(scope)?,
    });
    Ok((result, ty, var, purity))
}

fn typeClockCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType> = crate::NFType::interned_CLOCK();
    let mut var: Variability = Variability::PARAMETER.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut args_count: i32;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let __pa0 = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info.clone(), false)?) {
        Deref @ Call::TYPED_CALL { arguments: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    args = metamodelica::Own::own(__pa0);
    args_count = ((args).len() as i32);
    callExp = (::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::ListNode::Nil => metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(ClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) }),
        Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Nil } => {
            e1 = (*__esc_e1).clone();
            metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(ClockKind::NFClockKind::REAL_CLOCK { interval: e1.clone() }) })
        },
        Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            e2 = Ceval::evalExp(e2.clone(), &(Ceval::noTarget().clone()))?;
            callExp = (match &*(Expression::typeOf(e2.clone())) {
        Type::INTEGER => {
            Error::assertionOrAddSourceMessage(Expression::integerValue(e2.clone())? >= 1, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("Clock"), literal!("resolution"), Expression::toString(e2.clone())?, literal!("=> 1")], &info)?;
            metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(ClockKind::NFClockKind::RATIONAL_CLOCK { intervalCounter: e1.clone(), resolution: e2.clone() }) })
        },
        Type::REAL => metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(ClockKind::NFClockKind::EVENT_CLOCK { condition: e1.clone(), startInterval: e2.clone() }) }),
        Type::STRING => metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(ClockKind::NFClockKind::SOLVER_CLOCK { c: e1.clone(), solverMethod: e2.clone() }) }),
        _ => return Err("match: no arm matched"),
    });
            callExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((callExp, outType, var, purity))
}

fn typeSampleCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let mut args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut namedArgs: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut t1: metamodelica::Ref<Type::NFType>;
    let mut v1: Variability;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut normalSample: metamodelica::Ref<Function::Function>;
    let mut clockedSample: metamodelica::Ref<Function::Function>;
    let mut recopnode: metamodelica::Ref<InstNode::InstNode>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeNormalCall(call.clone(), context, info)?) {
        Deref @ Call::ARG_TYPED_CALL { r#ref: __pa0, positional_args: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    namedArgs = metamodelica::Own::own(__pa2);
    recopnode = ComponentRef::node(&fn_ref)?;
    (fn_ref, _, _) = Function::instFunctionRef(fn_ref, context, NFInstNode::InstNode::info(&recopnode))?;
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    normalSample = metamodelica::Own::own(__pa3);
    clockedSample = metamodelica::Own::own(__pa4);
    (callExp, outType, var) = (::match_deref::match_deref! { match &((args, namedArgs)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e1, ty: __esc_t1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e2, ty: Deref @ Type::INTEGER, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            e1 = (*__esc_e1).clone();
            t1 = (*__esc_t1).clone();
            e2 = (*__esc_e2).clone();
            if Type::isInteger(metamodelica::AsArg::as_arg(&t1))? {
                e1 = metamodelica::Ref::new(Expression::NFExpression::CAST { ty: crate::NFType::interned_REAL(), exp: e1.clone() });
            }
            ty_call = Call::makeTypedCall(normalSample, list![e1.clone(), metamodelica::Ref::new(Expression::NFExpression::CAST { ty: crate::NFType::interned_REAL(), exp: e2.clone() })], Variability::PARAMETER.clone(), purity, crate::NFType::interned_BOOLEAN());
            (metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call }), crate::NFType::interned_BOOLEAN(), Variability::PARAMETER.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e1, ty: __esc_t1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e2, ty: Deref @ Type::REAL, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            e1 = (*__esc_e1).clone();
            t1 = (*__esc_t1).clone();
            e2 = (*__esc_e2).clone();
            if Type::isInteger(metamodelica::AsArg::as_arg(&t1))? {
                e1 = metamodelica::Ref::new(Expression::NFExpression::CAST { ty: crate::NFType::interned_REAL(), exp: e1.clone() });
            }
            ty_call = Call::makeTypedCall(normalSample, list![e1.clone(), e2.clone()], Variability::PARAMETER.clone(), purity, crate::NFType::interned_BOOLEAN());
            (metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call }), crate::NFType::interned_BOOLEAN(), Variability::PARAMETER.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e1, ty: __esc_t1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { name: Some(Deref @ "interval"), value: __esc_e2, ty: Deref @ Type::REAL, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            e1 = (*__esc_e1).clone();
            t1 = (*__esc_t1).clone();
            e2 = (*__esc_e2).clone();
            if Type::isInteger(metamodelica::AsArg::as_arg(&t1))? {
                e1 = metamodelica::Ref::new(Expression::NFExpression::CAST { ty: crate::NFType::interned_REAL(), exp: e1.clone() });
            }
            ty_call = Call::makeTypedCall(normalSample, list![e1.clone(), e2.clone()], Variability::PARAMETER.clone(), purity, crate::NFType::interned_BOOLEAN());
            (metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call }), crate::NFType::interned_BOOLEAN(), Variability::PARAMETER.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e1, ty: __esc_t1, var: __esc_v1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            e1 = (*__esc_e1).clone();
            t1 = (*__esc_t1).clone();
            v1 = (*__esc_v1).clone();
            ty_call = Call::makeTypedCall(clockedSample, list![e1.clone(), metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(ClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) })], v1.clone(), purity, t1.clone());
            (metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call }), t1.clone(), v1.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e1, ty: __esc_t1, var: __esc_v1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e2, ty: Deref @ Type::CLOCK, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            e1 = (*__esc_e1).clone();
            t1 = (*__esc_t1).clone();
            v1 = (*__esc_v1).clone();
            e2 = (*__esc_e2).clone();
            ty_call = Call::makeTypedCall(clockedSample, list![e1.clone(), e2.clone()], v1.clone(), purity, t1.clone());
            (metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call }), t1.clone(), v1.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { value: __esc_e1, ty: __esc_t1, var: __esc_v1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ TypedArg { name: Some(Deref @ "c"), value: __esc_e2, ty: Deref @ Type::CLOCK, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            e1 = (*__esc_e1).clone();
            t1 = (*__esc_t1).clone();
            v1 = (*__esc_v1).clone();
            e2 = (*__esc_e2).clone();
            ty_call = Call::makeTypedCall(clockedSample, list![e1.clone(), e2.clone()], v1.clone(), purity, t1.clone());
            (metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call }), t1.clone(), v1.clone())
        },
        _ => {
            Error::addSourceMessage(&(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()), list![Call::toString(&call)?, literal!("<NO COMPONENT>")], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((callExp, outType, var, purity))
}

fn typeActualInStreamCall(
    mut name: ArcStr,
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability = Variability::DISCRETE.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(name.clone(), &named_args, info)?;
    if ((args).len() as i32) != 1 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(stream variable) => Real"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (arg, ty, var, _) = Typing::typeExp((args).head().cloned()?, arg_context, info, false)?;
    (arg, _) = ExpandExp::expand(arg, false, false)?;
    let __pa3 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa3);
    callExp = typeActualInStreamCall2(name, r#fn, arg, var, info)?;
    Ok((callExp, ty, variability, purity))
}

fn typeActualInStreamCall2(
    mut name: ArcStr,
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut var: Variability,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    callExp = (match &*arg.clone() {
        Expression::CREF {
            cref: __arg_cref,
            ty: __arg_ty,
        } => {
            let mut arg_node: metamodelica::Ref<InstNode::InstNode>;
            arg_node = ComponentRef::node(metamodelica::AsArg::as_arg(&__arg_cref))?;
            if !(NFInstNode::InstNode::isComponent(&arg_node)?)
                || !(Prefixes::ConnectorType::isStream(Component::connectorType(
                    &(NFInstNode::InstNode::component(&arg_node)?),
                )))
            {
                Error::addSourceMessageAndFail(
                    &(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone()),
                    list![ComponentRef::toString(metamodelica::AsArg::as_arg(&__arg_cref))?, name],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            for mut sub in &*ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&__arg_cref))? {
                if Subscript::variability(metamodelica::AsArg::as_arg(&sub))? > Variability::PARAMETER.clone() {
                    Error::addSourceMessageAndFail(
                        &(Error::CONNECTOR_NON_PARAMETER_SUBSCRIPT.clone()),
                        list![
                            ComponentRef::toString(metamodelica::AsArg::as_arg(&__arg_cref))?,
                            Subscript::toString(metamodelica::AsArg::as_arg(&sub))?
                        ],
                        info,
                    )?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            metamodelica::Ref::new(Expression::NFExpression::CALL {
                call: Call::makeTypedCall(r#fn, list![arg], var, Purity::IMPURE.clone(), __arg_ty.clone()),
            })
        }
        Expression::ARRAY { .. } => {
            assign_variant_field!(arg => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b0 = name; let __pe_b1 = r#fn; let __pe_b3 = var; let __pe_b4 = info.clone(); move |__pe_a2| typeActualInStreamCall2(__pe_b0.clone(), __pe_b1.clone(), __pe_a2, __pe_b3.clone(), &__pe_b4) }))?);
            arg
        }
        _ => {
            Error::addSourceMessage(
                &(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone()),
                list![Expression::toString(arg)?, name],
                info,
            )?;
            return Err("fail");
        }
    });
    Ok(callExp)
}

fn typeDynamicSelectCall(
    mut name: ArcStr,
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
    let mut variability: Variability = Variability::CONTINUOUS.clone();
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut var1: Variability;
    let mut var2: Variability;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut expStatic: metamodelica::Ref<Expression::NFExpression>;
    let mut expDynamic: metamodelica::Ref<Expression::NFExpression>;
    let mut arg_context: i32 = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::UNTYPED_CALL { r#ref: __pa0, arguments: __pa1, named_args: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    fn_ref = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    assertNoNamedParams(name, &named_args, info)?;
    if ((args).len() as i32) != 2 {
        Error::addSourceMessageAndFail(
            &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
            list![Call::toString(call)?, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ComponentRef::toString(&fn_ref)?);
                __mm_s.push_str(&*literal!("(static expression, dynamic expression)"));
                ArcStr::from(__mm_s)
            }],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (args).into_iter().cloned() {
            let __x = Expression::unbox(arg.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })) {
        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    expStatic = metamodelica::Own::own(__pa3);
    expDynamic = metamodelica::Own::own(__pa4);
    (arg1, ty1, var1, _) = Typing::typeExp(expStatic, arg_context, info, false)?;
    (arg1, _) = ExpandExp::expand(arg1, false, false)?;
    if let Ok((__pa6, __pa7, __pa8, _)) = Typing::typeExp(expDynamic.clone(), arg_context, info, false) {
        arg2 = metamodelica::Own::own(__pa6);
        ty2 = metamodelica::Own::own(__pa7);
        var2 = metamodelica::Own::own(__pa8);
    } else {
        if InstContext::inInstanceAPI(context) {
            return Err("fail");
        } else {
            variability = var1;
            callExp = arg1.clone();
            return Ok((callExp, ty, variability, purity));
        }
    }
    (arg2, _) = ExpandExp::expand(arg2, false, false)?;
    ty = ty1.clone();
    variability = var2;
    let __pa9 = ::match_deref::match_deref! { match &(Function::typeRefCache(&fn_ref, InstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Nil } => __pa9.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa9);
    if Flags::isSet(Flags::NF_API_DYNAMIC_SELECT.clone())? {
        callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(r#fn, list![arg1, arg2], variability, purity, ty1),
        });
    } else {
        variability = var1;
        callExp = arg1;
    }
    Ok((callExp, ty, variability, purity))
}

fn typeBackSampleCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let mut counter: metamodelica::Ref<Expression::NFExpression>;
    let mut resolution: metamodelica::Ref<Expression::NFExpression>;
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info, false)?) {
        __pa4 @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }, ty: __pa2, var: __pa3, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    counter = metamodelica::Own::own(__pa0);
    resolution = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    var = metamodelica::Own::own(__pa3);
    ty_call = metamodelica::Own::own(__pa4);
    Structural::markExp(&counter)?;
    Structural::markExp(&resolution)?;
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call });
    Ok((callExp, ty, var, purity))
}

fn typeShiftSampleCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let mut counter: metamodelica::Ref<Expression::NFExpression>;
    let mut resolution: metamodelica::Ref<Expression::NFExpression>;
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info, false)?) {
        __pa4 @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }, ty: __pa2, var: __pa3, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    counter = metamodelica::Own::own(__pa0);
    resolution = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    var = metamodelica::Own::own(__pa3);
    ty_call = metamodelica::Own::own(__pa4);
    Structural::markExp(&counter)?;
    Structural::markExp(&resolution)?;
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call });
    Ok((callExp, ty, var, purity))
}

fn typeSubSampleCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let mut factor: metamodelica::Ref<Expression::NFExpression>;
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info, false)?) {
        __pa3 @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } }, ty: __pa1, var: __pa2, .. } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    factor = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    var = metamodelica::Own::own(__pa2);
    ty_call = metamodelica::Own::own(__pa3);
    Structural::markExp(&factor)?;
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call });
    Ok((callExp, ty, var, purity))
}

fn typeSuperSampleCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::IMPURE.clone();
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let mut factor: metamodelica::Ref<Expression::NFExpression>;
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info, false)?) {
        __pa3 @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } }, ty: __pa1, var: __pa2, .. } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    factor = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    var = metamodelica::Own::own(__pa2);
    ty_call = metamodelica::Own::own(__pa3);
    Structural::markExp(&factor)?;
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call });
    Ok((callExp, ty, var, purity))
}

fn typeSpatialDistribution(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity;
    let mut ty_call: metamodelica::Ref<Call::NFCall>;
    let mut context_str: ArcStr;
    if InstContext::inSubexpression(context) || InstContext::inAlgorithm(context) {
        Error::addSourceMessage(
            &(Error::SPATIAL_DISTRIBUTION_CONTEXT.clone()),
            metamodelica::nil(),
            &info,
        )?;
        return Err("fail");
    }
    if InstContext::inIf(context) || InstContext::inWhen(context) {
        context_str = if (InstContext::inIf(context)) {
            literal!("an if-equation")
        } else {
            literal!("a when-equation")
        };
        Error::addSourceMessage(
            &(Error::ELEMENT_IS_NOT_ALLOWED_IN_CONTEXT.clone()),
            list![literal!("spatialDistribution"), context_str],
            &info,
        )?;
        return Err("fail");
    }
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info, false)?) {
        __pa3 @ Deref @ Call::TYPED_CALL { ty: __pa0, var: __pa1, purity: __pa2, .. } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    var = metamodelica::Own::own(__pa1);
    purity = metamodelica::Own::own(__pa2);
    ty_call = metamodelica::Own::own(__pa3);
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call });
    Ok((callExp, ty, var, purity))
}

fn typePureCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut purity: Purity = Purity::PURE.clone();
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut c: metamodelica::Ref<Call::NFCall>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Call::typeMatchNormalCall(call, context, info.clone(), false)?) {
        Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, ty: __pa1, var: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    var = metamodelica::Own::own(__pa2);
    callExp = Expression::unbox(arg);
    callExp = (::match_deref::match_deref! { match &(callExp) {
        Deref @ Expression::CALL { call: __esc_c @ Deref @ Call::TYPED_CALL { .. } } => {
            c = (*__esc_c).clone();
            assign_variant_field!(c => Call::NFCall::TYPED_CALL; purity = Expression::purityList(var_field!((*c).arguments, Call::NFCall::TYPED_CALL), Prefixes::Purity::PURE.clone())?);
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: c.clone() })
        },
        _ => {
            Error::addSourceMessage(&(Error::FUNCTION_ARGUMENT_MUST_BE.clone()), list![literal!("pure"), arcstr::literal!(Error::FUNCTION_CALL_EXPRESSION)], &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((callExp, ty, var, purity))
}

fn typeBuiltinCallExp(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
    mut vectorize: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut pur: Purity;
    let mut c: metamodelica::Ref<Call::NFCall>;
    (c, ty, var, pur) = typeBuiltinCall(call, context, info, vectorize)?;
    outExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: c });
    Ok((outExp, ty, var, pur))
}

fn typeBuiltinCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut context: i32,
    mut info: SourceInfo,
    mut vectorize: bool,
) -> Result<(
    metamodelica::Ref<Call::NFCall>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut outCall: metamodelica::Ref<Call::NFCall>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut pur: Purity;
    outCall = Call::typeMatchNormalCall(call, context, info, vectorize)?;
    ty = Call::typeOf(&outCall);
    var = Call::variability(&outCall)?;
    pur = Call::purity(&outCall);
    Ok((outCall, ty, var, pur))
}
