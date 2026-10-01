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

use crate::NFBinding as Binding;
use crate::NFBuiltin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::Slot;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFOperatorOverloading as OperatorOverloading;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum MatchKind {
    /// Exact match
    EXACT = 1,
    /// Matched by casting, e.g. Integer to Real
    CAST = 2,
    /// The expected type was unknown
    UNKNOWN_EXPECTED = 3,
    /// The actual type was unknown
    UNKNOWN_ACTUAL = 4,
    /// Matched with a generic type e.g. function F<T> input T i; end F; F(1)
    GENERIC = 5,
    /// Component by component matching, e.g. class A R r; end A; is plug compatible with class B R r; end B;
    PLUG_COMPATIBLE = 6,
    NOT_COMPATIBLE = 7,
}
impl PartialOrd for MatchKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MatchKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for MatchKind {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn isCompatibleMatch(mut kind: MatchKind) -> bool {
    let mut isCompatible: bool = kind != MatchKind::NOT_COMPATIBLE.clone();
    isCompatible
}

pub(crate) fn isIncompatibleMatch(mut kind: MatchKind) -> bool {
    let mut isIncompatible: bool = kind == MatchKind::NOT_COMPATIBLE.clone();
    isIncompatible
}

pub(crate) fn isExactMatch(mut kind: MatchKind) -> bool {
    let mut isCompatible: bool = kind == MatchKind::EXACT.clone();
    isCompatible
}

pub(crate) fn isCastMatch(mut kind: MatchKind) -> bool {
    let mut isCast: bool = kind == MatchKind::CAST.clone();
    isCast
}

pub(crate) fn isGenericMatch(mut kind: MatchKind) -> bool {
    let mut isCast: bool = kind == MatchKind::GENERIC.clone();
    isCast
}

pub(crate) fn isValidAssignmentMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind == MatchKind::EXACT.clone()
        || kind == MatchKind::CAST.clone()
        || kind == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub(crate) fn isValidArgumentMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind == MatchKind::EXACT.clone()
        || kind == MatchKind::CAST.clone()
        || kind == MatchKind::GENERIC.clone()
        || kind == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub(crate) fn isValidPlugCompatibleMatch(mut kind: MatchKind) -> bool {
    let mut v: bool = kind == MatchKind::EXACT.clone() || kind == MatchKind::PLUG_COMPATIBLE.clone();
    v
}

pub type MatchOptions = i32;

pub const DEFAULT_OPTIONS: i32 = 0;

pub(crate) const ALLOW_UNKNOWN: i32 = intBitLShift(1, 0);

pub(crate) const IGNORE_DIMENSIONS: i32 = intBitLShift(1, 1);

pub(crate) const IGNORE_DIMENSIONS_IN_RECORDS: i32 = intBitLShift(1, 2);

pub(crate) fn setOption(mut currentOptions: MatchOptions, mut newOption: MatchOptions) -> MatchOptions {
    let mut newOptions: MatchOptions = intBitOr(currentOptions, newOption);
    newOptions
}

pub(crate) fn getOption(mut options: MatchOptions, mut option: MatchOptions) -> bool {
    let mut isSet: bool = intBitAnd(options, option) > 0;
    isSet
}

pub(crate) fn checkBinaryOperation(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut operator: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut context: i32,
    mut info: &SourceInfo,
    mut retype: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    if Type::isConditionalArray(&type1) || Type::isConditionalArray(&type2) {
        (binaryExp, resultType) =
            checkConditionalBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info, retype)?;
    } else if Type::isComplex(&(Type::arrayElementType(&type1))) || Type::isComplex(&(Type::arrayElementType(&type2))) {
        (binaryExp, resultType) =
            checkOverloadedBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info)?;
    } else if Type::isBoxed(&type1) && Type::isBoxed(&type2) {
        (binaryExp, resultType) =
            checkBinaryOperationBoxed(exp1, type1, var1, operator, exp2, type2, var2, context, info, retype)?;
    } else {
        (binaryExp, resultType) = (match operator.op.clone() {
            Operator::Op::ADD => checkBinaryOperationAdd(exp1, type1, exp2, type2, info)?,
            Operator::Op::SUB => checkBinaryOperationSub(exp1, type1, exp2, type2, info)?,
            Operator::Op::MUL => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::DIV => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, retype)?,
            Operator::Op::POW => checkBinaryOperationPow(exp1, type1, exp2, type2, info)?,
            Operator::Op::ADD_EW => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::ADD.clone(), info)?,
            Operator::Op::SUB_EW => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::SUB.clone(), info)?,
            Operator::Op::MUL_EW => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::MUL.clone(), info)?,
            Operator::Op::DIV_EW => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, true)?,
            Operator::Op::POW_EW => checkBinaryOperationPowEW(exp1, type1, exp2, type2, info)?,
            Operator::Op::ADD_SCALAR_ARRAY => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::ADD.clone(), info)?,
            Operator::Op::ADD_ARRAY_SCALAR { .. } => {
                checkBinaryOperationEW(exp1, type1, exp2, type2, Op::ADD.clone(), info)?
            }
            Operator::Op::SUB_SCALAR_ARRAY { .. } => {
                checkBinaryOperationEW(exp1, type1, exp2, type2, Op::SUB.clone(), info)?
            }
            Operator::Op::SUB_ARRAY_SCALAR => checkBinaryOperationEW(exp1, type1, exp2, type2, Op::SUB.clone(), info)?,
            Operator::Op::MUL_SCALAR_ARRAY => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::MUL_ARRAY_SCALAR { .. } => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::MUL_VECTOR_MATRIX => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::MUL_MATRIX_VECTOR => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::SCALAR_PRODUCT => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::MATRIX_PRODUCT => checkBinaryOperationMul(exp1, type1, exp2, type2, info)?,
            Operator::Op::DIV_SCALAR_ARRAY { .. } => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, retype)?,
            Operator::Op::DIV_ARRAY_SCALAR { .. } => checkBinaryOperationDiv(exp1, type1, exp2, type2, info, retype)?,
            Operator::Op::POW_SCALAR_ARRAY { .. } => checkBinaryOperationPowEW(exp1, type1, exp2, type2, info)?,
            Operator::Op::POW_ARRAY_SCALAR { .. } => checkBinaryOperationPowEW(exp1, type1, exp2, type2, info)?,
            Operator::Op::POW_MATRIX => checkBinaryOperationPow(exp1, type1, exp2, type2, info)?,
            _ => return Err("match: no arm matched"),
        });
    }
    Ok((binaryExp, resultType))
}

pub(crate) fn checkOverloadedBinaryOperator(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut op_str: ArcStr;
    let mut candidates: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut ety1: metamodelica::Ref<Type::NFType>;
    let mut ety2: metamodelica::Ref<Type::NFType>;
    op_str = Operator::symbol(&(Operator::stripEW(op.clone())), &(literal!("'")))?;
    ety1 = Type::arrayElementType(&type1);
    ety2 = Type::arrayElementType(&type2);
    candidates = OperatorOverloading::lookupOperatorFunctionsInType(op_str.clone(), &ety1)?;
    if !(Type::isEqual(&ety1, &ety2)?) {
        candidates = listAppend(
            OperatorOverloading::lookupOperatorFunctionsInType(op_str, &ety2)?,
            candidates,
        );
    }
    if (candidates).is_empty() {
        printUnresolvableTypeError(
            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp1.clone(),
                operator: op.clone(),
                exp2: exp2.clone(),
            }),
            list![type1.clone(), type2.clone()],
            info,
            true,
        )?;
    }
    if Operator::isElementWise(&op) {
        (outExp, outType) = checkOverloadedBinaryArrayEW(
            exp1,
            type1,
            var1,
            Operator::stripEW(op),
            exp2,
            type2,
            var2,
            &candidates,
            context,
            info,
        )?;
    } else {
        (outExp, outType) = matchOverloadedBinaryOperator(
            exp1,
            type1,
            var1,
            op,
            exp2,
            type2,
            var2,
            &candidates,
            context,
            info,
            true,
        )?;
    }
    outExp = Inline::inlineCallExp(outExp, false)?;
    Ok((outExp, outType))
}

pub(crate) fn matchOverloadedBinaryOperator(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
    mut showErrors: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut matchedFunc: metamodelica::Ref<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    let mut exactMatches: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    args = list![
        metamodelica::Ref::new(TypedArg {
            name: None,
            value: exp1.clone(),
            ty: type1.clone(),
            var: var1,
            purity: Purity::PURE.clone()
        }),
        metamodelica::Ref::new(TypedArg {
            name: None,
            value: exp2.clone(),
            ty: type2.clone(),
            var: var2,
            purity: Purity::PURE.clone()
        })
    ];
    matchedFunctions = Function::matchFunctionsSilent(candidates, args, &(metamodelica::nil()), context, info, true)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if (exactMatches).is_empty() {
        ErrorExt::setCheckpoint(literal!("NFTypeCheck:implicitConstruction"));
        match '__try0: {
            (outExp, outType) = unwrap_break_err!(implicitConstructAndMatch(candidates, exp1.clone(), type1.clone(), op.clone(), exp2.clone(), type2.clone(), info), '__try0);
            if showErrors {
                ErrorExt::delCheckpoint(literal!("NFTypeCheck:implicitConstruction"));
            } else {
                ErrorExt::rollBack(literal!("NFTypeCheck:implicitConstruction"));
            }
            Ok::<_, &'static str>((outExp.clone(), outType.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outExp = __try0_o0;
                outType = __try0_o1;
            }
            Err(_) => {
                ErrorExt::rollBack(literal!("NFTypeCheck:implicitConstruction"));
                if Type::isArray(&type1) || Type::isArray(&type2) {
                    (outExp, outType) = (match op.op.clone() {
                        Operator::Op::ADD => checkOverloadedBinaryArrayAddSub(
                            exp1.clone(),
                            type1.clone(),
                            var1,
                            op.clone(),
                            exp2.clone(),
                            type2.clone(),
                            var2,
                            candidates,
                            context,
                            info,
                        )?,
                        Operator::Op::SUB => checkOverloadedBinaryArrayAddSub(
                            exp1.clone(),
                            type1.clone(),
                            var1,
                            op.clone(),
                            exp2.clone(),
                            type2.clone(),
                            var2,
                            candidates,
                            context,
                            info,
                        )?,
                        Operator::Op::MUL => checkOverloadedBinaryArrayMul(
                            exp1.clone(),
                            type1.clone(),
                            var1,
                            op.clone(),
                            exp2.clone(),
                            type2.clone(),
                            var2,
                            candidates,
                            context,
                            info,
                        )?,
                        Operator::Op::DIV => checkOverloadedBinaryArrayDiv(
                            exp1.clone(),
                            type1.clone(),
                            var1,
                            op.clone(),
                            exp2.clone(),
                            type2.clone(),
                            var2,
                            candidates,
                            context,
                            info,
                        )?,
                        _ => {
                            printUnresolvableTypeError(
                                metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                    exp1: exp1.clone(),
                                    operator: op.clone(),
                                    exp2: exp2.clone(),
                                }),
                                list![type1.clone(), type2.clone()],
                                info,
                                showErrors,
                            )?;
                            return Err("fail");
                        }
                    });
                } else {
                    printUnresolvableTypeError(
                        metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: exp1.clone(),
                            operator: op.clone(),
                            exp2: exp2.clone(),
                        }),
                        list![type1.clone(), type2.clone()],
                        info,
                        showErrors,
                    )?;
                    return Err("fail");
                }
            }
        }
    } else if ((exactMatches).len() as i32) == 1 {
        let __pa1 = ::match_deref::match_deref! { match &(exactMatches) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        matchedFunc = metamodelica::Own::own(__pa1);
        r#fn = matchedFunc.func.clone();
        outType = Function::returnType(&r#fn);
        outExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
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
                Prefixes::variabilityMax(var1, var2),
                Purity::PURE.clone(),
                outType.clone(),
            ),
        });
    } else {
        if showErrors {
            Error::addSourceMessage(
                &(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone()),
                list![
                    Expression::toString(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: exp1,
                        operator: op,
                        exp2: exp2
                    }))?,
                    Function::candidateFuncListString(
                        ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<Function::Function>> =
                                metamodelica::nil();
                            for mut mfn in (matchedFunctions).into_iter().cloned() {
                                let __x = mfn.func.clone();
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                    )?
                ],
                info,
            )?;
        }
        return Err("fail");
    }
    Ok((outExp, outType))
}

pub(crate) fn checkBinaryOperationBoxed(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut context: i32,
    mut info: &SourceInfo,
    mut retype: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    (e1, ty1, _) = matchTypes(type1.clone(), Type::unbox(type1), exp1, DEFAULT_OPTIONS.clone())?;
    (e2, ty2, _) = matchTypes(type2.clone(), Type::unbox(type2), exp2, DEFAULT_OPTIONS.clone())?;
    (outExp, outType) = checkBinaryOperation(e1, ty1, var1, op, e2, ty2, var2, context, info, retype)?;
    Ok((outExp, outType))
}

fn checkConditionalBinaryOperator(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut context: i32,
    mut info: &SourceInfo,
    mut retype: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut tty1: metamodelica::Ref<Type::NFType>;
    let mut fty1: metamodelica::Ref<Type::NFType>;
    let mut tty2: metamodelica::Ref<Type::NFType>;
    let mut fty2: metamodelica::Ref<Type::NFType>;
    let mut ty1: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty2: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut e1: metamodelica::Ref<Expression::NFExpression> = exp1.clone();
    let mut e2: metamodelica::Ref<Expression::NFExpression> = exp2.clone();
    let mut valid1: bool;
    let mut valid2: bool;
    let mut branch: Type::Branch;
    (tty1, fty1, tty2, fty2, branch) = (::match_deref::match_deref! { match &((&*type1, &*type2)) {
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => (var_field!((*type1).trueType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type1).falseType, Type::NFType::CONDITIONAL_ARRAY).clone(), type2.clone(), type2.clone(), var_field!((*type1).matchedBranch, Type::NFType::CONDITIONAL_ARRAY).clone()),
        (_, Deref @ Type::CONDITIONAL_ARRAY { .. }) => (type1.clone(), type1.clone(), var_field!((*type2).trueType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type2).falseType, Type::NFType::CONDITIONAL_ARRAY).clone(), var_field!((*type2).matchedBranch, Type::NFType::CONDITIONAL_ARRAY).clone()),
        _ => return Err("match: no arm matched"),
    } });
    ErrorExt::setCheckpoint(literal!("NFTypeCheck.checkConditionalBinaryOperator"));
    match '__try0: {
        (e1, ty1) = unwrap_break_err!(checkBinaryOperation(exp1.clone(), tty1.clone(), var1, op.clone(), exp2.clone(), tty2.clone(), var2, context, info, retype), '__try0);
        valid1 = true;
        Ok::<_, &'static str>((valid1.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valid1 = __try0_o0;
        }
        Err(_) => {
            valid1 = false;
        }
    }
    match '__try1: {
        (e2, ty2) = unwrap_break_err!(checkBinaryOperation(exp1.clone(), fty1.clone(), var1, op.clone(), exp2.clone(), fty2.clone(), var2, context, info, retype), '__try1);
        valid2 = true;
        Ok::<_, &'static str>((valid2.clone(),))
    } {
        Ok((__try1_o0,)) => {
            valid2 = __try1_o0;
        }
        Err(_) => {
            valid2 = false;
        }
    }
    ErrorExt::rollBack(literal!("NFTypeCheck.checkConditionalBinaryOperator"));
    if valid1 && valid2 {
        outType = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
            trueType: ty1,
            falseType: ty2,
            matchedBranch: branch,
        });
        outExp = e1;
    } else if valid1 {
        outType = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
            trueType: ty1,
            falseType: crate::NFType::interned_UNKNOWN(),
            matchedBranch: Type::Branch::TRUE.clone(),
        });
        outExp = e1;
    } else if valid2 {
        outType = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
            trueType: crate::NFType::interned_UNKNOWN(),
            falseType: ty2,
            matchedBranch: Type::Branch::FALSE.clone(),
        });
        outExp = e2;
    } else {
        printUnresolvableTypeError(exp1, list![type1, type2], info, true)?;
        return Err("fail");
    }
    outExp = Expression::setType(outType.clone(), outExp)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayAddSub(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    (e1, e2, _, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    if !(isCompatibleMatch(mk)) {
        printUnresolvableTypeError(
            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: e1.clone(),
                operator: op.clone(),
                exp2: e2.clone(),
            }),
            list![type1.clone(), type2.clone()],
            info,
            true,
        )?;
    }
    (e1, _) = ExpandExp::expand(e1, false, false)?;
    (e2, _) = ExpandExp::expand(e2, false, false)?;
    (outExp, outType) =
        checkOverloadedBinaryArrayAddSub2(&e1, &type1, var1, &op, &e2, &type2, var2, candidates, context, info)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayAddSub2(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: &metamodelica::Ref<Operator::NFOperator>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
    mut type2: &metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    (outExp, outType) = (::match_deref::match_deref! { match (exp1, exp2) {
        (Deref @ Expression::ARRAY { elements: arr1, .. }, Deref @ Expression::ARRAY { elements: arr2, .. }) => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut ty1: metamodelica::Ref<Type::NFType>;
            let mut ty2: metamodelica::Ref<Type::NFType>;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            ty = crate::NFType::interned_UNKNOWN();
            if arr1.clone().borrow().is_empty() {
                ty1 = Type::arrayElementType(type1);
                ty2 = Type::arrayElementType(type2);
                arr = metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
                if '__try0: {
                    (_, ty) = unwrap_break_err!(matchOverloadedBinaryOperator(metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty1.clone() }), ty1.clone(), var1, op.clone(), metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty2.clone() }), ty2.clone(), var2, candidates, context, info, false), '__try0);
                    Ok::<(), &'static str>(())
                }.is_err() {
                    printUnresolvableTypeError(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info, true)?;
                }
            } else {
                ty1 = Type::unliftArray(type1.clone())?;
                ty2 = Type::unliftArray(type2.clone())?;
                arr = metamodelica::arrayCreateDefault(metamodelica::arrayLength(arr1.clone()));
                for mut i in 1..=metamodelica::arrayLength(arr1.clone()) {
                    e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i);
                    e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr2.clone(), i);
                    (e, ty) = checkOverloadedBinaryArrayAddSub2(&e1, &ty1, var1, op, &e2, &ty2, var2, candidates, context, info)?;
                    unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i, e) };
                }
            }
            outType = Type::setArrayElementType(type1, &ty);
            outExp = Expression::makeArray(outType.clone(), arr.clone(), false);
            (outExp, outType)
        },
        _ => {
            matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates, context, info, true)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayMul(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut valid: bool;
    let mut dims1: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dim11: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim12: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim21: metamodelica::Ref<Dimension::NFDimension>;
    dims1 = Type::arrayDims(type1.clone());
    dims2 = Type::arrayDims(type2.clone());
    (valid, outExp) = (::match_deref::match_deref! { match &((dims1, dims2)) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }) => {
            (outExp, _) = checkOverloadedBinaryScalarArray(&exp1, &type1, var1, &op, exp2, &type2, var2, candidates, context, info)?;
            (true, outExp)
        },
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            (outExp, _) = checkOverloadedBinaryArrayScalar(exp1, &type1, var1, &op, &exp2, &type2, var2, candidates, context, info)?;
            (true, outExp)
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_dim11, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim12, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Cons { head: __esc_dim21, tail: Deref @ metamodelica::ListNode::Nil }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(metamodelica::AsArg::as_arg(&dim12), metamodelica::AsArg::as_arg(&dim21))?;
            outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 });
            valid = false;
            (valid, outExp)
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_dim11, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim12, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Cons { head: __esc_dim21, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(metamodelica::AsArg::as_arg(&dim12), metamodelica::AsArg::as_arg(&dim21))?;
            outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 });
            valid = false;
            (valid, outExp)
        },
        _ => (false, metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(valid) {
        printUnresolvableTypeError(outExp.clone(), list![type1, type2], info, true)?;
    }
    outType = Expression::typeOf(outExp.clone());
    Ok((outExp, outType))
}

fn checkOverloadedBinaryScalarArray(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: &metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: &metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    (outExp, outType) = checkOverloadedBinaryScalarArray2(
        exp1,
        type1,
        var1,
        op,
        &((ExpandExp::expand(exp2, false, false)?).0),
        type2,
        var2,
        candidates,
        context,
        info,
    )?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryScalarArray2(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: &metamodelica::Ref<Operator::NFOperator>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
    mut type2: &metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    (outExp, outType) = (match &**exp2 {
        Expression::ARRAY { ty: __exp2_ty, .. }
            if (var_field!((**exp2).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .is_empty()) =>
        {
            match '__try0: {
                ty = unwrap_break_err!(Type::unliftArray(type2.clone()), '__try0);
                (_, outType) = unwrap_break_err!(matchOverloadedBinaryOperator(exp1.clone(), type1.clone(), var1, op.clone(), metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: type2.clone() }), ty.clone(), var2, candidates, context, info, false), '__try0);
                Ok::<_, &'static str>((outType.clone(), ty.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    outType = __try0_o0;
                    ty = __try0_o1;
                }
                Err(__try0_err) => {
                    printUnresolvableTypeError(
                        metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: exp1.clone(),
                            operator: op.clone(),
                            exp2: exp2.clone(),
                        }),
                        list![type1.clone(), __exp2_ty.clone()],
                        info,
                        true,
                    )?;
                    return Err(__try0_err);
                }
            }
            outType = Type::setArrayElementType(metamodelica::AsArg::as_arg(&__exp2_ty), &outType);
            (Expression::makeEmptyArray(outType.clone())?, outType)
        }
        Expression::ARRAY { ty: __exp2_ty, .. } => {
            ty = Type::unliftArray(type2.clone())?;
            arr = metamodelica::arrayCreate(
                metamodelica::arrayLength(var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone()),
                exp2.clone(),
            );
            for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
                e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(
                    var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone(),
                    i,
                );
                unsafe {
                    metamodelica::Dangerous::arrayInitSlot(
                        arr.clone(),
                        i,
                        (checkOverloadedBinaryScalarArray2(
                            exp1, type1, var1, op, &e2, &ty, var2, candidates, context, info,
                        )?)
                        .0,
                    )
                };
            }
            outType = Type::setArrayElementType(
                metamodelica::AsArg::as_arg(&__exp2_ty),
                &(Expression::typeOf(
                    ({
                        let __elt = (*metamodelica::index_checked(&arr.borrow(), 1)?).clone();
                        __elt
                    }),
                )),
            );
            (Expression::makeArray(outType.clone(), arr.clone(), false), outType)
        }
        _ => matchOverloadedBinaryOperator(
            exp1.clone(),
            type1.clone(),
            var1,
            op.clone(),
            exp2.clone(),
            type2.clone(),
            var2,
            candidates,
            context,
            info,
            true,
        )?,
    });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayScalar(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: &metamodelica::Ref<Operator::NFOperator>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
    mut type2: &metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    (outExp, outType) = checkOverloadedBinaryArrayScalar2(
        &((ExpandExp::expand(exp1, false, false)?).0),
        type1,
        var1,
        op,
        exp2,
        type2,
        var2,
        candidates,
        context,
        info,
    )?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayScalar2(
    mut exp1: &metamodelica::Ref<Expression::NFExpression>,
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: &metamodelica::Ref<Operator::NFOperator>,
    mut exp2: &metamodelica::Ref<Expression::NFExpression>,
    mut type2: &metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    (outExp, outType) = (match &**exp1 {
        Expression::ARRAY { ty: __exp1_ty, .. }
            if (var_field!((**exp1).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .is_empty()) =>
        {
            match '__try0: {
                ty = unwrap_break_err!(Type::unliftArray(type1.clone()), '__try0);
                (_, outType) = unwrap_break_err!(matchOverloadedBinaryOperator(metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: type1.clone() }), ty.clone(), var1, op.clone(), exp2.clone(), type2.clone(), var2, candidates, context, info, false), '__try0);
                Ok::<_, &'static str>((outType.clone(), ty.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    outType = __try0_o0;
                    ty = __try0_o1;
                }
                Err(__try0_err) => {
                    printUnresolvableTypeError(
                        metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: exp1.clone(),
                            operator: op.clone(),
                            exp2: exp2.clone(),
                        }),
                        list![type1.clone(), __exp1_ty.clone()],
                        info,
                        true,
                    )?;
                    return Err(__try0_err);
                }
            }
            outType = Type::setArrayElementType(metamodelica::AsArg::as_arg(&__exp1_ty), &outType);
            (Expression::makeEmptyArray(outType.clone())?, outType)
        }
        Expression::ARRAY { ty: __exp1_ty, .. } => {
            ty = Type::unliftArray(type1.clone())?;
            arr = metamodelica::arrayCreate(
                metamodelica::arrayLength(var_field!((**exp1).elements, Expression::NFExpression::ARRAY).clone()),
                exp1.clone(),
            );
            for mut i in 1..=metamodelica::arrayLength(arr.clone()) {
                e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(
                    var_field!((**exp1).elements, Expression::NFExpression::ARRAY).clone(),
                    i,
                );
                unsafe {
                    metamodelica::Dangerous::arrayInitSlot(
                        arr.clone(),
                        i,
                        (checkOverloadedBinaryArrayScalar2(
                            &e1, &ty, var1, op, exp2, type2, var2, candidates, context, info,
                        )?)
                        .0,
                    )
                };
            }
            outType = Type::setArrayElementType(
                metamodelica::AsArg::as_arg(&__exp1_ty),
                &(Expression::typeOf(
                    ({
                        let __elt = (*metamodelica::index_checked(&arr.borrow(), 1)?).clone();
                        __elt
                    }),
                )),
            );
            (Expression::makeArray(outType.clone(), arr.clone(), false), outType)
        }
        _ => matchOverloadedBinaryOperator(
            exp1.clone(),
            type1.clone(),
            var1,
            op.clone(),
            exp2.clone(),
            type2.clone(),
            var2,
            candidates,
            context,
            info,
            true,
        )?,
    });
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayDiv(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    if Type::isArray(&type1) && Type::isScalar(&type2) {
        (outExp, outType) =
            checkOverloadedBinaryArrayScalar(exp1, &type1, var1, &op, &exp2, &type2, var2, candidates, context, info)?;
    } else {
        printUnresolvableTypeError(
            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: exp1,
                operator: op,
                exp2: exp2,
            }),
            list![type1, type2],
            info,
            true,
        )?;
        return Err("fail");
    }
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayEW(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    if Type::isArray(&type1) && Type::isArray(&type2) {
        (e1, e2, _, mk) = matchExpressions(
            exp1.clone(),
            type1.clone(),
            exp2.clone(),
            type2.clone(),
            ALLOW_UNKNOWN.clone(),
        )?;
    } else {
        (e1, e2, _, mk) = matchExpressions(
            exp1.clone(),
            Type::arrayElementType(&type1),
            exp2.clone(),
            Type::arrayElementType(&type2),
            ALLOW_UNKNOWN.clone(),
        )?;
    }
    if !(isCompatibleMatch(mk)) {
        printUnresolvableTypeError(
            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: e1,
                operator: op.clone(),
                exp2: e2,
            }),
            list![type1.clone(), type2.clone()],
            info,
            true,
        )?;
    }
    (e1, _) = ExpandExp::expand(exp1, false, false)?;
    (e2, _) = ExpandExp::expand(exp2, false, false)?;
    (outExp, outType) = checkOverloadedBinaryArrayEW2(e1, type1, var1, op, e2, type2, var2, candidates, context, info)?;
    Ok((outExp, outType))
}

fn checkOverloadedBinaryArrayEW2(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl1: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl2: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut is_array1: bool;
    let mut is_array2: bool;
    is_array1 = Type::isArray(&type1);
    is_array2 = Type::isArray(&type2);
    if is_array1 || is_array2 {
        expl = metamodelica::nil();
        if Expression::isEmptyArray(&exp1) || Expression::isEmptyArray(&exp2) {
            ty1 = Type::arrayElementType(&type1);
            ty2 = Type::arrayElementType(&type2);
            if '__try0: {
                (_, ty) = unwrap_break_err!(matchOverloadedBinaryOperator(metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty1.clone() }), ty1.clone(), var1, op.clone(), metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty2.clone() }), ty2.clone(), var2, candidates, context, info, true), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
                printUnresolvableTypeError(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() }), list![type1.clone(), type2.clone()], info, true)?;
            }
        } else if is_array1 && is_array2 {
            ty1 = Type::unliftArray(type1.clone())?;
            ty2 = Type::unliftArray(type2)?;
            expl1 = Expression::arrayElements(&exp1)?;
            expl2 = Expression::arrayElements(&exp2)?;
            if metamodelica::arrayLength(expl1.clone()) > metamodelica::arrayLength(expl2.clone()) {
                return Err("fail");
            }
            for mut i in 1..=metamodelica::arrayLength(expl1.clone()) {
                e1 = metamodelica::Dangerous::arrayGetNoBoundsChecking(expl1.clone(), i);
                e2 = metamodelica::Dangerous::arrayGetNoBoundsChecking(expl2.clone(), i);
                (e1, ty) = checkOverloadedBinaryArrayEW2(
                    e1,
                    ty1.clone(),
                    var1,
                    op.clone(),
                    e2,
                    ty2.clone(),
                    var2,
                    candidates,
                    context,
                    info,
                )?;
                expl = metamodelica::cons(e1, expl);
            }
        } else if is_array1 {
            ty1 = Type::unliftArray(type1.clone())?;
            expl1 = Expression::arrayElements(&exp1)?;
            let __range1 = expl1.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range1 {
                (e, ty) = checkOverloadedBinaryArrayEW2(
                    e,
                    ty1.clone(),
                    var1,
                    op.clone(),
                    exp2.clone(),
                    type2.clone(),
                    var2,
                    candidates,
                    context,
                    info,
                )?;
                expl = metamodelica::cons(e, expl);
            }
        } else if is_array2 {
            ty2 = Type::unliftArray(type2)?;
            expl2 = Expression::arrayElements(&exp2)?;
            let __range2 = expl2.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut e in __range2 {
                (e, ty) = checkOverloadedBinaryArrayEW2(
                    exp1.clone(),
                    type1.clone(),
                    var1,
                    op.clone(),
                    e,
                    ty2.clone(),
                    var2,
                    candidates,
                    context,
                    info,
                )?;
                expl = metamodelica::cons(e, expl);
            }
        }
        outType = Type::setArrayElementType(&type1, &ty);
        outExp = Expression::makeArray(
            outType.clone(),
            metamodelica::arrayFromVec(
                metamodelica::Dangerous::listReverseInPlace(expl)
                    .into_iter()
                    .cloned()
                    .collect(),
            ),
            false,
        );
    } else {
        (outExp, outType) = matchOverloadedBinaryOperator(
            exp1, type1, var1, op, exp2, type2, var2, candidates, context, info, true,
        )?;
    }
    Ok((outExp, outType))
}

fn implicitConstructAndMatch(
    mut candidates: &metamodelica::List<metamodelica::Ref<Function::Function>>,
    mut inExp1: metamodelica::Ref<Expression::NFExpression>,
    mut inType1: metamodelica::Ref<Type::NFType>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut inExp2: metamodelica::Ref<Expression::NFExpression>,
    mut inType2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut in1: metamodelica::Ref<InstNode::InstNode>;
    let mut in2: metamodelica::Ref<InstNode::InstNode>;
    let mut operfn: metamodelica::Ref<Function::Function>;
    let mut matchedfuncs: metamodelica::List<(
        metamodelica::Ref<Function::Function>,
        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        Variability,
    )> = metamodelica::nil();
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut arg1_ty: metamodelica::Ref<Type::NFType>;
    let mut arg2_ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut matched: bool;
    let mut arg1_info: SourceInfo;
    let mut arg2_info: SourceInfo;
    exp1 = inExp1.clone();
    exp2 = inExp2.clone();
    for mut r#fn in &**candidates {
        if ((r#fn.inputs).len() as i32) != 2 {
            continue;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#fn.inputs.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        in1 = metamodelica::Own::own(__pa0);
        in2 = metamodelica::Own::own(__pa1);
        arg1_ty = NFInstNode::InstNode::getType(in1.clone())?;
        arg2_ty = NFInstNode::InstNode::getType(in2.clone())?;
        arg1_info = NFInstNode::InstNode::info(&in1);
        arg2_info = NFInstNode::InstNode::info(&in2);
        (matchedfuncs, matched) = implicitConstructAndMatch2(
            inExp1.clone(),
            inType1.clone(),
            inExp2.clone(),
            arg1_ty.clone(),
            arg1_info.clone(),
            arg2_ty.clone(),
            arg2_info.clone(),
            NFInstNode::InstNode::classScope(in2)?,
            r#fn.clone(),
            false,
            matchedfuncs,
        )?;
        if matched {
            continue;
        }
        (matchedfuncs, matched) = implicitConstructAndMatch2(
            inExp2.clone(),
            inType2.clone(),
            inExp1.clone(),
            arg2_ty,
            arg2_info,
            arg1_ty,
            arg1_info,
            NFInstNode::InstNode::classScope(in1)?,
            r#fn.clone(),
            true,
            matchedfuncs,
        )?;
    }
    if ((matchedfuncs).len() as i32) == 1 {
        let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(matchedfuncs) {
            Deref @ metamodelica::ListNode::Cons { head: (__pa3, Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } }, __pa6), tail: _ } => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => return Err("pattern mismatch"),
        } };
        operfn = metamodelica::Own::own(__pa3);
        exp1 = metamodelica::Own::own(__pa4);
        exp2 = metamodelica::Own::own(__pa5);
        var = metamodelica::Own::own(__pa6);
        outType = Function::returnType(&operfn);
        outExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(operfn, list![exp1, exp2], var, Purity::PURE.clone(), outType.clone()),
        });
    } else {
        Error::addSourceMessage(
            &(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone()),
            list![
                Expression::toString(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: exp1,
                    operator: op,
                    exp2: exp2
                }))?,
                Function::candidateFuncListString(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
                        for mut r#fn in (matchedfuncs).into_iter().cloned() {
                            let __x = Util::tuple31(r#fn.clone());
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                )?
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok((outExp, outType))
}

fn implicitConstructAndMatch2(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut paramType1: metamodelica::Ref<Type::NFType>,
    mut paramInfo1: SourceInfo,
    mut paramType2: metamodelica::Ref<Type::NFType>,
    mut paramInfo2: SourceInfo,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut reverseArgs: bool,
    mut matchedFns: metamodelica::List<(
        metamodelica::Ref<Function::Function>,
        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        Variability,
    )>,
) -> Result<(
    metamodelica::List<(
        metamodelica::Ref<Function::Function>,
        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        Variability,
    )>,
    bool,
)> {
    let mut matchedFns: metamodelica::List<(
        metamodelica::Ref<Function::Function>,
        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        Variability,
    )> = matchedFns;
    let mut matched: bool;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut var: Variability;
    let mut ty: metamodelica::Ref<Type::NFType>;
    (e1, _, mk) = matchTypes(paramType1, type1, exp1, DEFAULT_OPTIONS.clone())?;
    if mk == MatchKind::EXACT.clone() {
        (fn_ref, _, _) = Function::instFunction(
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: literal!("'constructor'"),
                subscripts: metamodelica::nil(),
            }),
            scope.clone(),
            InstContext::NO_CONTEXT.clone(),
            paramInfo2,
        )?;
        e2 = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(Call::NFCall::UNTYPED_CALL {
                r#ref: fn_ref,
                arguments: list![exp2],
                named_args: metamodelica::nil(),
                call_scope: NFInstNode::InstNode::scopeRef(scope),
            }),
        });
        (e2, ty, var, _) = Call::typeCall(e2, 0, paramInfo1, false)?;
        (_, _, mk) = matchTypes(paramType2, ty, e2.clone(), DEFAULT_OPTIONS.clone())?;
        if mk == MatchKind::EXACT.clone() {
            matchedFns = metamodelica::cons(
                (r#fn, if (reverseArgs) { list![e2, e1] } else { list![e1, e2] }, var),
                matchedFns,
            );
            matched = true;
        } else {
            matched = false;
        }
    } else {
        matched = false;
    }
    Ok((matchedFns, matched))
}

fn checkBinaryOperationAdd(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut valid: bool;
    (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    valid = (match &*(Type::arrayElementType(&resultType)) {
        Type::INTEGER => valid,
        Type::REAL => valid,
        Type::STRING => valid,
        _ => false,
    });
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: Operator::makeAdd(resultType.clone()),
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationSub(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut valid: bool;
    (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    valid = (match &*(Type::arrayElementType(&resultType)) {
        Type::INTEGER => valid,
        Type::REAL => valid,
        _ => false,
    });
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: Operator::makeSub(resultType.clone()),
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationMul(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut dims1: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dim11: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim12: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim21: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim22: metamodelica::Ref<Dimension::NFDimension>;
    let mut mk: MatchKind;
    let mut op: Op;
    let mut valid: bool;
    ty1 = Type::arrayElementType(&type1);
    ty2 = Type::arrayElementType(&type2);
    (e1, e2, resultType, mk) = matchExpressions(exp1, ty1, exp2, ty2, ALLOW_UNKNOWN.clone())?;
    valid = isCompatibleMatch(mk);
    valid = (match &*resultType {
        Type::INTEGER => valid,
        Type::REAL => valid,
        _ => false,
    });
    dims1 = Type::arrayDims(type1.clone());
    dims2 = Type::arrayDims(type2.clone());
    (resultType, op) = (::match_deref::match_deref! { match &((dims1.clone(), dims2.clone())) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => (resultType, Op::MUL.clone()),
        (Deref @ metamodelica::ListNode::Nil, _) => (metamodelica::Ref::new(Type::NFType::ARRAY { elementType: resultType, dimensions: dims2 }), Op::MUL_SCALAR_ARRAY.clone()),
        (_, Deref @ metamodelica::ListNode::Nil) => (metamodelica::Ref::new(Type::NFType::ARRAY { elementType: resultType, dimensions: dims1 }), Op::MUL_ARRAY_SCALAR.clone()),
        (Deref @ metamodelica::ListNode::Cons { head: __esc_dim11, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __esc_dim21, tail: Deref @ metamodelica::ListNode::Nil }) => {
            dim11 = (*__esc_dim11).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(metamodelica::AsArg::as_arg(&dim11), metamodelica::AsArg::as_arg(&dim21))?;
            (resultType, Op::SCALAR_PRODUCT.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_dim11, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __esc_dim21, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim22, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            dim11 = (*__esc_dim11).clone();
            dim21 = (*__esc_dim21).clone();
            dim22 = (*__esc_dim22).clone();
            valid = Dimension::isEqual(metamodelica::AsArg::as_arg(&dim11), metamodelica::AsArg::as_arg(&dim21))?;
            (metamodelica::Ref::new(Type::NFType::ARRAY { elementType: resultType, dimensions: list![dim22.clone()] }), Op::MUL_VECTOR_MATRIX.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_dim11, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim12, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Cons { head: __esc_dim21, tail: Deref @ metamodelica::ListNode::Nil }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            valid = Dimension::isEqual(metamodelica::AsArg::as_arg(&dim12), metamodelica::AsArg::as_arg(&dim21))?;
            (metamodelica::Ref::new(Type::NFType::ARRAY { elementType: resultType, dimensions: list![dim11.clone()] }), Op::MUL_MATRIX_VECTOR.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: __esc_dim11, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim12, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Cons { head: __esc_dim21, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim22, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            dim11 = (*__esc_dim11).clone();
            dim12 = (*__esc_dim12).clone();
            dim21 = (*__esc_dim21).clone();
            dim22 = (*__esc_dim22).clone();
            valid = Dimension::isEqual(metamodelica::AsArg::as_arg(&dim12), metamodelica::AsArg::as_arg(&dim21))?;
            (metamodelica::Ref::new(Type::NFType::ARRAY { elementType: resultType, dimensions: list![dim11.clone(), dim22.clone()] }), Op::MATRIX_PRODUCT.clone())
        },
        _ => {
            valid = false;
            (resultType, Op::MUL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: metamodelica::Ref::new(Operator::NFOperator {
            ty: resultType.clone(),
            op: op,
        }),
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationDiv(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
    mut isElementWise: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    (e1, ty1, mk) = matchTypes(
        type1.clone(),
        Type::setArrayElementType(&type1, &(crate::NFType::interned_REAL())),
        exp1,
        ALLOW_UNKNOWN.clone(),
    )?;
    valid = isCompatibleMatch(mk);
    (e2, ty2, mk) = matchTypes(
        type2.clone(),
        Type::setArrayElementType(&type2, &(crate::NFType::interned_REAL())),
        exp2,
        ALLOW_UNKNOWN.clone(),
    )?;
    valid = valid && isCompatibleMatch(mk);
    (resultType, op) = (match (Type::isArray(&ty1), Type::isArray(&ty2), isElementWise) {
        (false, false, _) => (ty1.clone(), Operator::makeDiv(ty1)),
        (_, false, _) => (
            ty1.clone(),
            metamodelica::Ref::new(Operator::NFOperator {
                ty: ty1,
                op: Op::DIV_ARRAY_SCALAR.clone(),
            }),
        ),
        (false, _, true) => (
            ty2.clone(),
            metamodelica::Ref::new(Operator::NFOperator {
                ty: ty2,
                op: Op::DIV_SCALAR_ARRAY.clone(),
            }),
        ),
        (true, _, true) => {
            (_, _, mk) = matchArrayTypes(&ty1, &ty2, e1.clone(), ALLOW_UNKNOWN.clone())?;
            valid = valid && isCompatibleMatch(mk);
            (ty1.clone(), Operator::makeDiv(ty1))
        }
        _ => {
            valid = false;
            (ty1.clone(), Operator::makeDiv(ty1))
        }
    });
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationPow(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    (e1, resultType, mk) = matchTypes(
        type1.clone(),
        Type::setArrayElementType(&type1, &(crate::NFType::interned_REAL())),
        exp1,
        ALLOW_UNKNOWN.clone(),
    )?;
    valid = isCompatibleMatch(mk);
    if Type::isArray(&resultType) {
        valid = valid && Type::isSquareMatrix(&resultType)?;
        valid = valid && Type::isInteger(&type2)?;
        valid = valid && !(Expression::isNegative(&exp2)?);
        op = metamodelica::Ref::new(Operator::NFOperator {
            ty: resultType.clone(),
            op: Op::POW_MATRIX.clone(),
        });
        e2 = exp2;
    } else {
        (e2, _, mk) = matchTypes(
            type2.clone(),
            crate::NFType::interned_REAL(),
            exp2,
            ALLOW_UNKNOWN.clone(),
        )?;
        valid = valid && isCompatibleMatch(mk);
        op = metamodelica::Ref::new(Operator::NFOperator {
            ty: resultType.clone(),
            op: Op::POW.clone(),
        });
    }
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationPowEW(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    (e1, ty1, mk) = matchTypes(
        type1.clone(),
        Type::setArrayElementType(&type1, &(crate::NFType::interned_REAL())),
        exp1,
        ALLOW_UNKNOWN.clone(),
    )?;
    valid = isCompatibleMatch(mk);
    (e2, ty2, mk) = matchTypes(
        type2.clone(),
        Type::setArrayElementType(&type2, &(crate::NFType::interned_REAL())),
        exp2,
        ALLOW_UNKNOWN.clone(),
    )?;
    valid = valid && isCompatibleMatch(mk);
    (resultType, op) = (match (Type::isArray(&ty1), Type::isArray(&ty2)) {
        (false, false) => (ty1.clone(), Operator::makePow(ty1)),
        (_, false) => (
            ty1.clone(),
            metamodelica::Ref::new(Operator::NFOperator {
                ty: ty1,
                op: Op::POW_ARRAY_SCALAR.clone(),
            }),
        ),
        (false, _) => (
            ty2.clone(),
            metamodelica::Ref::new(Operator::NFOperator {
                ty: ty2,
                op: Op::POW_SCALAR_ARRAY.clone(),
            }),
        ),
        _ => {
            (_, _, mk) = matchArrayTypes(&ty1, &ty2, e1.clone(), ALLOW_UNKNOWN.clone())?;
            valid = valid && isCompatibleMatch(mk);
            (ty1.clone(), Operator::makePow(ty1))
        }
    });
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

fn checkBinaryOperationEW(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut elemOp: Op,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut binaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let mut ty2: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut is_arr1: bool;
    let mut is_arr2: bool;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    is_arr1 = Type::isArray(&type1);
    is_arr2 = Type::isArray(&type2);
    if is_arr1 && is_arr2 {
        (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    } else {
        ty1 = Type::arrayElementType(&type1);
        ty2 = Type::arrayElementType(&type2);
        (e1, e2, resultType, mk) = matchExpressions(exp1, ty1, exp2, ty2, ALLOW_UNKNOWN.clone())?;
    }
    valid = isCompatibleMatch(mk);
    valid = (::match_deref::match_deref! { match &((Type::arrayElementType(&resultType), elemOp)) {
        (Deref @ Type::INTEGER, _) => valid,
        (Deref @ Type::REAL, _) => valid,
        (Deref @ Type::STRING, Operator::Op::ADD) => valid,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (resultType, op) = (match (is_arr1, is_arr2) {
        (true, false) => {
            resultType = Type::copyDims(type1.clone(), resultType);
            op = Operator::makeArrayScalar(resultType.clone(), elemOp)?;
            (resultType, op)
        }
        (false, true) => {
            resultType = Type::copyDims(type2.clone(), resultType);
            op = Operator::makeScalarArray(resultType.clone(), elemOp)?;
            (resultType, op)
        }
        (true, true) => (
            resultType.clone(),
            Operator::makeEW(metamodelica::Ref::new(Operator::NFOperator {
                ty: resultType,
                op: elemOp,
            })),
        ),
        _ => (
            resultType.clone(),
            metamodelica::Ref::new(Operator::NFOperator {
                ty: resultType,
                op: elemOp,
            }),
        ),
    });
    binaryExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2,
    });
    if !(valid) {
        printUnresolvableTypeError(binaryExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((binaryExp, resultType))
}

pub(crate) fn checkUnaryOperation(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut operator: metamodelica::Ref<Operator::NFOperator>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut unaryExp: metamodelica::Ref<Expression::NFExpression>;
    let mut unaryType: metamodelica::Ref<Type::NFType>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    if Type::isComplex(&(Type::arrayElementType(&type1))) {
        (unaryExp, unaryType) = checkOverloadedUnaryOperator(exp1, type1, var1, operator, context, info)?;
        return Ok((unaryExp, unaryType));
    }
    unaryType = type1.clone();
    op = Operator::setType(unaryType.clone(), operator.clone());
    unaryExp = (match operator.op.clone() {
        Operator::Op::ADD => exp1,
        _ => metamodelica::Ref::new(Expression::NFExpression::UNARY {
            operator: op,
            exp: exp1,
        }),
    });
    if !(Type::isNumeric(&type1)?) {
        printUnresolvableTypeError(unaryExp.clone(), list![type1], info, true)?;
    }
    Ok((unaryExp, unaryType))
}

pub(crate) fn checkOverloadedUnaryOperator(
    mut inExp1: metamodelica::Ref<Expression::NFExpression>,
    mut inType1: metamodelica::Ref<Type::NFType>,
    mut var: Variability,
    mut inOp: metamodelica::Ref<Operator::NFOperator>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut opstr: ArcStr;
    let mut candidates: metamodelica::List<metamodelica::Ref<Function::Function>>;
    let mut args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut matchedFunc: metamodelica::Ref<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>> =
        metamodelica::nil();
    let mut exactMatches: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    opstr = Operator::symbol(&inOp, &(literal!("'")))?;
    candidates = OperatorOverloading::lookupOperatorFunctionsInType(opstr, &inType1)?;
    args = list![metamodelica::Ref::new(TypedArg {
        name: None,
        value: inExp1.clone(),
        ty: inType1.clone(),
        var: var,
        purity: Purity::PURE.clone()
    })];
    matchedFunctions = Function::matchFunctionsSilent(&candidates, args, &(metamodelica::nil()), context, info, false)?;
    exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
    if (exactMatches).is_empty() {
        printUnresolvableTypeError(
            metamodelica::Ref::new(Expression::NFExpression::UNARY {
                operator: inOp.clone(),
                exp: inExp1.clone(),
            }),
            list![inType1],
            info,
            true,
        )?;
        return Err("fail");
    }
    if ((exactMatches).len() as i32) == 1 {
        let __pa0 = ::match_deref::match_deref! { match &(exactMatches) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        matchedFunc = metamodelica::Own::own(__pa0);
        outType = Function::returnType(&matchedFunc.func);
        outExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
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
                Purity::PURE.clone(),
                outType.clone(),
            ),
        });
    } else {
        Error::addSourceMessage(
            &(Error::AMBIGUOUS_MATCHING_OPERATOR_FUNCTIONS_NFINST.clone()),
            list![
                Expression::toString(metamodelica::Ref::new(Expression::NFExpression::UNARY {
                    operator: inOp,
                    exp: inExp1
                }))?,
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
            info,
        )?;
        return Err("fail");
    }
    outExp = Inline::inlineCallExp(outExp, false)?;
    Ok((outExp, outType))
}

pub(crate) fn checkLogicalBinaryOperation(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut operator: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    if Type::isComplex(&(Type::arrayElementType(&type1))) || Type::isComplex(&(Type::arrayElementType(&type2))) {
        (outExp, resultType) =
            checkOverloadedBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info)?;
        return Ok((outExp, resultType));
    }
    (e1, e2, resultType, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), ALLOW_UNKNOWN.clone())?;
    outExp = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
        exp1: e1,
        operator: Operator::setType(resultType.clone(), operator),
        exp2: e2,
    });
    if !(isCompatibleMatch(mk)) || !(Type::isBoolean(&(Type::arrayElementType(&resultType)))) {
        printUnresolvableTypeError(outExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((outExp, resultType))
}

pub(crate) fn checkLogicalUnaryOperation(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut operator: metamodelica::Ref<Operator::NFOperator>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType> = type1.clone();
    if Type::isComplex(&(Type::arrayElementType(&type1))) {
        (outExp, resultType) = checkOverloadedUnaryOperator(exp1, type1, var1, operator, context, info)?;
        return Ok((outExp, resultType));
    }
    outExp = metamodelica::Ref::new(Expression::NFExpression::LUNARY {
        operator: Operator::setType(type1.clone(), operator),
        exp: exp1,
    });
    if !(Type::isBoolean(&(Type::arrayElementType(&type1)))) {
        printUnresolvableTypeError(outExp.clone(), list![type1], info, true)?;
    }
    Ok((outExp, resultType))
}

pub(crate) fn checkRelationOperation(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut var1: Variability,
    mut operator: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut var2: Variability,
    mut index: i32,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut mk: MatchKind;
    let mut valid: bool;
    let mut o: Op;
    if Type::isComplex(&(Type::arrayElementType(&type1))) || Type::isComplex(&(Type::arrayElementType(&type2))) {
        (outExp, resultType) =
            checkOverloadedBinaryOperator(exp1, type1, var1, operator, exp2, type2, var2, context, info)?;
        return Ok((outExp, resultType));
    }
    (e1, e2, ty, mk) = matchExpressions(exp1, type1.clone(), exp2, type2.clone(), DEFAULT_OPTIONS.clone())?;
    valid = isCompatibleMatch(mk);
    resultType = crate::NFType::interned_BOOLEAN();
    outExp = metamodelica::Ref::new(Expression::NFExpression::RELATION {
        exp1: e1,
        operator: Operator::setType(ty.clone(), operator.clone()),
        exp2: e2,
        index: index,
    });
    valid = (match &*ty {
        Type::INTEGER => valid,
        Type::REAL => {
            o = operator.op.clone();
            if !(InstContext::inFunction(context)) && (o == Op::EQUAL.clone() || o == Op::NEQUAL.clone()) {
                Error::addStrictMessage(
                    Error::WARNING_RELATION_ON_REAL.clone(),
                    list![
                        Expression::toString(outExp.clone())?,
                        Operator::symbol(&operator, &(literal!("")))?
                    ],
                    info,
                )?;
            }
            valid
        }
        Type::STRING => valid,
        Type::BOOLEAN => valid,
        Type::ENUMERATION { .. } => valid,
        _ => false,
    });
    if !(valid) {
        printUnresolvableTypeError(outExp.clone(), list![type1, type2], info, true)?;
    }
    Ok((outExp, resultType))
}

pub(crate) fn printUnresolvableTypeError(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut types: metamodelica::List<metamodelica::Ref<Type::NFType>>,
    mut info: &SourceInfo,
    mut printError: bool,
) -> Result<()> {
    let mut exp_str: ArcStr;
    let mut ty_str: ArcStr;
    if printError {
        exp_str = Expression::toString(exp)?;
        ty_str = List::toStringCustom(
            types,
            &move |__a0: metamodelica::Ref<Type::NFType>| Type::toString(&__a0),
            literal!(""),
            literal!(""),
            literal!(", "),
            literal!(""),
            false,
            0,
        )?;
        Error::addSourceMessage(
            &(Error::UNRESOLVABLE_TYPE.clone()),
            list![exp_str, ty_str, literal!("<NO_COMPONENT>")],
            info,
        )?;
    }
    return Err("fail");
    Ok(())
}

pub(crate) fn matchExpressions(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp1: metamodelica::Ref<Expression::NFExpression> = exp1;
    let mut exp2: metamodelica::Ref<Expression::NFExpression> = exp2;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    if referenceEq(&*(&*type1), &*(&*type2)) {
        compatibleType = type1;
        matchKind = MatchKind::EXACT.clone();
        return Ok((exp1, exp2, compatibleType, matchKind));
    }
    if metamodelica::valueConstructor((&*&*type1))? != metamodelica::valueConstructor((&*&*type2))? {
        (exp1, exp2, compatibleType, matchKind) = matchExpressions_cast(exp1, type1, exp2, type2, options)?;
        return Ok((exp1, exp2, compatibleType, matchKind));
    }
    matchKind = MatchKind::EXACT.clone();
    compatibleType = (match &*type1 {
        Type::INTEGER => type1,
        Type::REAL => type1,
        Type::STRING => type1,
        Type::BOOLEAN => type1,
        Type::CLOCK => type1,
        Type::ENUMERATION { .. } => {
            matchKind = matchEnumerationTypes(&type1, &type2)?;
            type1
        }
        Type::ARRAY { .. } => {
            (exp1, exp2, compatibleType, matchKind) = matchArrayExpressions(exp1, &type1, exp2, &type2, options)?;
            compatibleType
        }
        Type::TUPLE { .. } => {
            (exp1, compatibleType, matchKind) = matchTupleTypes(type1, &type2, exp1, options)?;
            compatibleType
        }
        Type::UNKNOWN => {
            matchKind = if (getOption(options, ALLOW_UNKNOWN.clone())) {
                MatchKind::EXACT.clone()
            } else {
                MatchKind::NOT_COMPATIBLE.clone()
            };
            type1
        }
        Type::COMPLEX { .. } => {
            (exp1, compatibleType, matchKind) = matchComplexTypes(type1, type2, exp1, options)?;
            compatibleType
        }
        Type::METABOXED { .. } => {
            (exp1, exp2, compatibleType, matchKind) = matchBoxedExpressions(exp1, type1, exp2, type2, options)?;
            compatibleType
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTypeCheck.matchExpressions"));
                    __mm_s.push_str(&*literal!(" got unknown type."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTypeCheck.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub fn matchTypes(
    mut actualType: metamodelica::Ref<Type::NFType>,
    mut expectedType: metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    if referenceEq(&*(&*actualType), &*(&*expectedType)) {
        compatibleType = actualType;
        matchKind = MatchKind::EXACT.clone();
        return Ok((expression, compatibleType, matchKind));
    }
    if metamodelica::valueConstructor((&*&*actualType))? != metamodelica::valueConstructor((&*&*expectedType))? {
        (expression, compatibleType, matchKind) = matchTypes_cast(actualType, expectedType, expression, options)?;
        return Ok((expression, compatibleType, matchKind));
    }
    matchKind = MatchKind::EXACT.clone();
    compatibleType = (match &*actualType {
        Type::INTEGER => actualType,
        Type::REAL => actualType,
        Type::STRING => actualType,
        Type::BOOLEAN => actualType,
        Type::CLOCK => actualType,
        Type::ENUMERATION { .. } => {
            if Type::isUnspecifiedEnumeration(&expectedType) {
                matchKind = MatchKind::EXACT.clone();
            } else {
                matchKind = matchEnumerationTypes(&actualType, &expectedType)?;
            }
            actualType
        }
        Type::ARRAY { .. } => {
            (expression, compatibleType, matchKind) = matchArrayTypes(&actualType, &expectedType, expression, options)?;
            compatibleType
        }
        Type::TUPLE { .. } => {
            (expression, compatibleType, matchKind) = matchTupleTypes(actualType, &expectedType, expression, options)?;
            compatibleType
        }
        Type::UNKNOWN => {
            matchKind = if (getOption(options, ALLOW_UNKNOWN.clone())) {
                MatchKind::EXACT.clone()
            } else {
                MatchKind::NOT_COMPATIBLE.clone()
            };
            actualType
        }
        Type::COMPLEX { .. } => {
            (expression, compatibleType, matchKind) = matchComplexTypes(actualType, expectedType, expression, options)?;
            compatibleType
        }
        Type::FUNCTION { .. } => {
            (expression, compatibleType, matchKind) =
                matchFunctionTypes(actualType, &expectedType, expression, options)?;
            compatibleType
        }
        Type::METABOXED { ty: __actualType_ty } => {
            (expression, compatibleType, matchKind) = matchTypes(
                __actualType_ty.clone(),
                Type::unbox(expectedType),
                Expression::unbox(expression),
                options,
            )?;
            expression = Expression::r#box(&expression);
            compatibleType = Type::r#box(&compatibleType);
            compatibleType
        }
        Type::CONDITIONAL_ARRAY { .. } => {
            (expression, compatibleType, matchKind) =
                matchConditionalArrayTypes(actualType, &expectedType, expression, options)?;
            compatibleType
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTypeCheck.matchTypes"));
                    __mm_s.push_str(&*literal!(" got unknown type."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTypeCheck.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchExpressions_cast(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp1: metamodelica::Ref<Expression::NFExpression> = exp1;
    let mut exp2: metamodelica::Ref<Expression::NFExpression> = exp2;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut before: metamodelica::Ref<Expression::NFExpression> = exp1.clone();
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((type1.clone(), type2.clone())) {
        (Deref @ Type::INTEGER, Deref @ Type::REAL) => {
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::ENUMERATION { .. }, Deref @ Type::INTEGER) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("nonStdEnumerationAsIntegers"))?) => {
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of enumeration expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" to Integer: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use Integer(")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(") instead!")); ArcStr::from(__mm_s) })?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::INTEGER, Deref @ Type::ENUMERATION { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("nonStdIntegersAsEnumeration"))?) => {
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Integer expression: ")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(" to enumeration: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use the actual enumeration instead!")); ArcStr::from(__mm_s) })?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::REAL, Deref @ Type::INTEGER) => {
            exp2 = Expression::typeCast(exp2, type1.clone())?;
            (type1, MatchKind::CAST.clone())
        },
        (Deref @ Type::BOOLEAN, Deref @ Type::REAL) if (Flags::isSet(Flags::NF_API.clone())?) => {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Boolean expression: ")); __mm_s.push_str(&*Expression::toString(exp1.clone())?); __mm_s.push_str(&*literal!(" to Real.")); ArcStr::from(__mm_s) })?;
            exp1 = Expression::typeCast(exp1, type2.clone())?;
            (type2, MatchKind::CAST.clone())
        },
        (Deref @ Type::REAL, Deref @ Type::BOOLEAN) if (Flags::isSet(Flags::NF_API.clone())?) => {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing casting of Boolean expression: ")); __mm_s.push_str(&*Expression::toString(exp2.clone())?); __mm_s.push_str(&*literal!(" to Real.")); ArcStr::from(__mm_s) })?;
            exp2 = Expression::typeCast(exp2, type1.clone())?;
            (type1, MatchKind::CAST.clone())
        },
        (Deref @ Type::TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: __esc_compatibleType, tail: _ }, .. }, _) => {
            compatibleType = (*__esc_compatibleType).clone();
            exp1 = Expression::tupleElement(exp1, 1)?;
            (exp1, compatibleType, matchKind) = matchTypes(compatibleType.clone(), type2, exp1, options)?;
            if isCompatibleMatch(matchKind) {
                matchKind = MatchKind::CAST.clone();
            }
            (compatibleType.clone(), matchKind)
        },
        (Deref @ Type::UNKNOWN, _) => (type2, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (_, Deref @ Type::UNKNOWN) => (type1, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::EXACT.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (Deref @ Type::METABOXED { .. }, _) => {
            (exp1, exp2, compatibleType, matchKind) = matchExpressions(Expression::unbox(exp1), var_field!((*type1).ty, Type::NFType::METABOXED).clone(), exp2, type2, options)?;
            (compatibleType, matchKind)
        },
        (_, Deref @ Type::METABOXED { .. }) => {
            (exp1, exp2, compatibleType, matchKind) = matchExpressions(exp1, type1, Expression::unbox(exp2), var_field!((*type2).ty, Type::NFType::METABOXED).clone(), options)?;
            (compatibleType, matchKind)
        },
        (_, Deref @ Type::POLYMORPHIC { .. }) => {
            exp1 = Expression::r#box(&exp1);
            (Type::r#box(&type1), MatchKind::GENERIC.clone())
        },
        (Deref @ Type::POLYMORPHIC { .. }, _) => {
            exp2 = Expression::r#box(&exp2);
            (Type::r#box(&type2), MatchKind::GENERIC.clone())
        },
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => {
            (exp1, exp2, compatibleType, matchKind) = matchConditionalArrayExp(exp1, type1, exp2, type2, options)?;
            (compatibleType, matchKind)
        },
        (_, Deref @ Type::CONDITIONAL_ARRAY { .. }) => {
            (exp2, exp1, compatibleType, matchKind) = matchConditionalArrayExp(exp2, type2, exp1, type1, options)?;
            (compatibleType, matchKind)
        },
        _ => (crate::NFType::interned_UNKNOWN(), MatchKind::NOT_COMPATIBLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub(crate) fn matchComplexTypes(
    mut actualType: metamodelica::Ref<Type::NFType>,
    mut expectedType: metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut compatibleType: metamodelica::Ref<Type::NFType> = actualType.clone();
    let mut matchKind: MatchKind = MatchKind::NOT_COMPATIBLE.clone();
    let mut cls1: metamodelica::Ref<Class::NFClass>;
    let mut cls2: metamodelica::Ref<Class::NFClass>;
    let mut ctree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut anode: metamodelica::Ref<InstNode::InstNode>;
    let mut enode: metamodelica::Ref<InstNode::InstNode>;
    let mut comps1: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut comps2: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cty1: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut cty2: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut matched_elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut elem_arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut opt: MatchOptions = options;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    anode = Type::complexNode(&actualType)?;
    enode = Type::complexNode(&expectedType)?;
    if NFInstNode::InstNode::isSame(anode.clone(), enode.clone()) {
        matchKind = MatchKind::EXACT.clone();
        return Ok((expression, compatibleType, matchKind));
    }
    cls1 = NFInstNode::InstNode::getClass(anode)?;
    cls2 = NFInstNode::InstNode::getClass(enode.clone())?;
    if getOption(opt, IGNORE_DIMENSIONS_IN_RECORDS.clone()) {
        opt = setOption(opt, IGNORE_DIMENSIONS.clone());
    }
    let () = (::match_deref::match_deref! { match &((cls1, actualType, cls2, expectedType.clone())) {
        (_, Deref @ Type::COMPLEX { complexTy: __esc_cty1 @ Deref @ ComplexType::CONNECTOR { .. }, .. }, _, Deref @ Type::COMPLEX { complexTy: __esc_cty2 @ Deref @ ComplexType::CONNECTOR { .. }, .. }) => {
            cty1 = (*__esc_cty1).clone();
            cty2 = (*__esc_cty2).clone();
            matchKind = matchComponentList(var_field!((*cty1).potentials, ComplexType::NFComplexType::CONNECTOR), var_field!((*cty2).potentials, ComplexType::NFComplexType::CONNECTOR).clone(), options)?;
            if matchKind != MatchKind::NOT_COMPATIBLE.clone() {
                matchKind = matchComponentList(var_field!((*cty1).flows, ComplexType::NFComplexType::CONNECTOR), var_field!((*cty2).flows, ComplexType::NFComplexType::CONNECTOR).clone(), options)?;
                if matchKind != MatchKind::NOT_COMPATIBLE.clone() {
                    matchKind = matchComponentList(var_field!((*cty1).streams, ComplexType::NFComplexType::CONNECTOR), var_field!((*cty2).streams, ComplexType::NFComplexType::CONNECTOR).clone(), options)?;
                }
            }
            if matchKind != MatchKind::NOT_COMPATIBLE.clone() {
                matchKind = MatchKind::PLUG_COMPATIBLE.clone();
            }
            ()
        },
        (Deref @ Class::INSTANCED_CLASS { elements: __esc_ctree @ Deref @ ClassTree::FLAT_TREE { components: __esc_comps1, .. }, .. }, _, Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_comps2, .. }, .. }, _) => {
            ctree = (*__esc_ctree).clone();
            comps1 = (*__esc_comps1).clone();
            comps2 = (*__esc_comps2).clone();
            if metamodelica::arrayLength(comps1.clone()) != metamodelica::arrayLength(comps2.clone()) {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok((expression, compatibleType, matchKind));
            }
            matchKind = MatchKind::PLUG_COMPATIBLE.clone();
            elem_arr = (match &*expression {
        Expression::RECORD { elements: __expression_elements, .. } => metamodelica::arrayFromVec(__expression_elements.clone().into_iter().cloned().collect()),
        _ => {
            elem_arr = metamodelica::arrayCreate(metamodelica::arrayLength(comps1.clone()), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }));
            dims = Type::arrayDims(Expression::typeOf(expression.clone()));
            for mut i in ({let __s=metamodelica::arrayLength(comps1.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                ty = Component::getType(&(NFInstNode::InstNode::component(&({let __elt = (*metamodelica::index_checked(&comps1.borrow(), i)?).clone(); __elt}))?))?;
                ty = Type::liftArrayRightList(ty, &dims);
                {
                    let __cell0 = metamodelica::Ref::new(Expression::NFExpression::RECORD_ELEMENT { recordExp: expression.clone(), index: i, fieldName: NFInstNode::InstNode::name(&({let __elt = (*metamodelica::index_checked(&comps1.borrow(), i)?).clone(); __elt}))?, ty: ty });
                    let __idx0 = i;
                    let _ = unsafe { metamodelica::Dangerous::arrayInitSlotChecked(elem_arr.clone().clone(), __idx0, __cell0) }?;
                }
            }
            elem_arr.clone()
        },
    });
            (matched_elements, matchKind) = matchComplexComponents(comps1.clone(), comps2.clone(), elem_arr.clone(), metamodelica::AsArg::as_arg(&ctree), opt)?;
            if matchKind == MatchKind::CAST.clone() {
                expression = typeCastRecord(matched_elements, enode, expectedType, expression)?;
            }
            ()
        },
        _ => {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchComplexComponents(
    mut actualComponents: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut expectedComponents: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut expressions: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    mut classTree: &metamodelica::Ref<ClassTree::ClassTree>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    MatchKind,
)> {
    let mut matchedExpressions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut matchKind: MatchKind = MatchKind::PLUG_COMPATIBLE.clone();
    let mut anode: metamodelica::Ref<InstNode::InstNode>;
    let mut enode: metamodelica::Ref<InstNode::InstNode>;
    let mut acomp: metamodelica::Ref<Component::NFComponent>;
    let mut ecomp: metamodelica::Ref<Component::NFComponent>;
    let mut idx: i32;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut mk: MatchKind;
    if metamodelica::arrayLength(actualComponents.clone()) != metamodelica::arrayLength(expectedComponents.clone())
        || metamodelica::arrayLength(actualComponents.clone()) != metamodelica::arrayLength(expressions.clone())
    {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((matchedExpressions, matchKind));
    }
    for mut i in 1..=metamodelica::arrayLength(actualComponents.clone()) {
        enode = ({
            let __elt = (*metamodelica::index_checked(&expectedComponents.borrow(), i)?).clone();
            __elt
        });
        ecomp = NFInstNode::InstNode::component(&enode)?;
        anode = ({
            let __elt = (*metamodelica::index_checked(&actualComponents.borrow(), i)?).clone();
            __elt
        });
        if metamodelica::stringEq(
            &(NFInstNode::InstNode::name(&anode)?),
            &(NFInstNode::InstNode::name(&enode)?),
        ) {
            idx = i;
        } else {
            if let Ok(__iflet0) = ClassTree::lookupComponentIndex(NFInstNode::InstNode::name(&enode)?, classTree) {
                idx = __iflet0;
            } else {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok((matchedExpressions, matchKind));
            }
            anode = ({
                let __elt = (*metamodelica::index_checked(&actualComponents.borrow(), idx)?).clone();
                __elt
            });
        }
        if i != idx {
            matchKind = MatchKind::CAST.clone();
        }
        acomp = NFInstNode::InstNode::component(&anode)?;
        e = ({
            let __elt = (*metamodelica::index_checked(&expressions.borrow(), idx)?).clone();
            __elt
        });
        (e, _, mk) = matchTypes(Component::getType(&acomp)?, Component::getType(&ecomp)?, e, options)?;
        matchedExpressions = metamodelica::cons(e, matchedExpressions);
        if mk == MatchKind::CAST.clone() {
            matchKind = mk;
        } else if !(isValidPlugCompatibleMatch(mk)) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            break;
        }
    }
    matchedExpressions = metamodelica::Dangerous::listReverseInPlace(matchedExpressions);
    Ok((matchedExpressions, matchKind))
}

pub(crate) fn typeCastRecord(
    mut expressions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut expectedType: metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut iters: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut i: i32;
    ty = Expression::typeOf(expression.clone());
    if Type::isArray(&ty) {
        dims = Type::arrayDims(ty.clone());
        ranges = metamodelica::nil();
        iters = metamodelica::nil();
        subs = metamodelica::nil();
        i = 1;
        for mut d in &*dims.reverse() {
            if Dimension::isUnknown(metamodelica::AsArg::as_arg(&d)) {
                ranges = metamodelica::cons(
                    metamodelica::Ref::new(Expression::NFExpression::RANGE {
                        ty: crate::NFType::interned_INTEGER(),
                        start: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                        step: None,
                        stop: metamodelica::Ref::new(Expression::NFExpression::SIZE {
                            exp: expression.clone(),
                            dimIndex: Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })),
                        }),
                    }),
                    ranges,
                );
            } else {
                ranges = metamodelica::cons(Dimension::toRange(metamodelica::AsArg::as_arg(&d))?, ranges);
            }
            iter = NFInstNode::InstNode::newUniqueIterator(
                NFInstNode::InstNode::info(&node),
                crate::NFType::interned_INTEGER(),
            );
            iters = metamodelica::cons(iter.clone(), iters);
            sub = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                index: metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: crate::NFType::interned_INTEGER(),
                    cref: ComponentRef::makeIterator(iter, crate::NFType::interned_INTEGER())?,
                }),
            });
            subs = metamodelica::cons(sub, subs);
            i = i + 1;
        }
        expression = metamodelica::Ref::new(Expression::NFExpression::RECORD {
            path: NFInstNode::InstNode::scopePath(node, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?,
            ty: expectedType,
            elements: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut e in (expressions).into_iter().cloned() {
                    let __x = Expression::applySubscripts(&subs, e.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        });
        expression = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: ty,
                var: Expression::variability(expression.clone())?,
                purity: Expression::purity(expression.clone())?,
                exp: expression,
                iters: ({
                    let mut __acc: metamodelica::List<(
                        metamodelica::Ref<InstNode::InstNode>,
                        metamodelica::Ref<Expression::NFExpression>,
                    )> = metamodelica::nil();
                    let __thr_src0 = iters;
                    let mut __thr_it0 = (&__thr_src0).into_iter();
                    let __thr_src1 = ranges;
                    let mut __thr_it1 = (&__thr_src1).into_iter();
                    loop {
                        match (__thr_it0.next(), __thr_it1.next()) {
                            (Some(i), Some(r)) => {
                                let __x = (i.clone(), r.clone());
                                __acc = cons(__x, __acc);
                            }
                            (None, None) => break,
                            _ => return Err("threaded for: ranges of unequal length"),
                        }
                    }
                    __acc.reverse()
                }),
            }),
        });
    } else {
        expression = metamodelica::Ref::new(Expression::NFExpression::RECORD {
            path: NFInstNode::InstNode::scopePath(node, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?,
            ty: expectedType,
            elements: expressions,
        });
    }
    Ok(expression)
}

pub(crate) fn matchComponentList(
    mut comps1: &metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>>,
    mut comps2: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>>,
    mut options: MatchOptions,
) -> Result<MatchKind> {
    let mut matchKind: MatchKind;
    let mut c1: metamodelica::Ref<InstNode::InstNode>;
    let mut c2: metamodelica::Ref<InstNode::InstNode>;
    let mut c2_ref: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut rest_c2: metamodelica::List<Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>> =
        comps2.clone();
    let mut dummy: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
    if ((comps1).len() as i32) != ((comps2).len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
    } else {
        for mut c1_ref in &**comps1 {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_c2) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            c2_ref = metamodelica::Own::own(__pa0);
            rest_c2 = metamodelica::Own::own(__pa1);
            c1 = NFInstNode::InstNode::borrow(c1_ref.clone())?;
            c2 = NFInstNode::InstNode::borrow(c2_ref)?;
            if !metamodelica::stringEq(&(NFInstNode::InstNode::name(&c1)?), &(NFInstNode::InstNode::name(&c2)?)) {
                matchKind = MatchKind::NOT_COMPATIBLE.clone();
                return Ok(matchKind);
            }
            (_, _, matchKind) = matchTypes(
                NFInstNode::InstNode::getType(c1)?,
                NFInstNode::InstNode::getType(c2)?,
                dummy.clone(),
                options,
            )?;
            if matchKind == MatchKind::NOT_COMPATIBLE.clone() {
                return Ok(matchKind);
            }
        }
    }
    matchKind = MatchKind::PLUG_COMPATIBLE.clone();
    Ok(matchKind)
}

pub(crate) fn matchFunctionTypes(
    mut actualType: metamodelica::Ref<Type::NFType>,
    mut expectedType: &metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut compatibleType: metamodelica::Ref<Type::NFType> = actualType.clone();
    let mut matchKind: MatchKind = MatchKind::EXACT.clone();
    let mut inputs1: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut inputs2: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut outputs1: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>;
    let mut outputs2: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>;
    let mut slots1: metamodelica::List<metamodelica::Ref<Slot::Slot>>;
    let mut slots2: metamodelica::List<metamodelica::Ref<Slot::Slot>>;
    let mut slot1: metamodelica::Ref<Slot::Slot>;
    let mut slot2: metamodelica::Ref<Slot::Slot>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(actualType) {
        Deref @ Type::FUNCTION { r#fn: Deref @ Function::FUNCTION { inputs: __pa0, outputs: __pa1, slots: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    inputs1 = metamodelica::Own::own(__pa0);
    outputs1 = metamodelica::Own::own(__pa1);
    slots1 = metamodelica::Own::own(__pa2);
    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &((*expectedType)) {
        Deref @ Type::FUNCTION { r#fn: Deref @ Function::FUNCTION { inputs: __pa4, outputs: __pa5, slots: __pa6, .. }, .. } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    inputs2 = metamodelica::Own::own(__pa4);
    outputs2 = metamodelica::Own::own(__pa5);
    slots2 = metamodelica::Own::own(__pa6);
    if ((outputs1).len() as i32) != ((outputs2).len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression, compatibleType, matchKind));
    }
    if !(matchFunctionParameters(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut o in (outputs1).into_iter().cloned() {
                let __x = NFInstNode::InstNode::fromHandle(&(o.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut o in (outputs2).into_iter().cloned() {
                let __x = NFInstNode::InstNode::fromHandle(&(o.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        options,
    )?) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression, compatibleType, matchKind));
    }
    if !(matchFunctionParameters(inputs1, inputs2.clone(), options)?) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression, compatibleType, matchKind));
    }
    for mut i in &*inputs2 {
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(slots1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
            _ => return Err("pattern mismatch"),
        } };
        slot1 = metamodelica::Own::own(__pa8);
        slots1 = metamodelica::Own::own(__pa9);
        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(slots2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
            _ => return Err("pattern mismatch"),
        } };
        slot2 = metamodelica::Own::own(__pa10);
        slots2 = metamodelica::Own::own(__pa11);
        if (slot2.default).is_some() && (slot1.default).is_none() {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((expression, compatibleType, matchKind));
        }
    }
    for mut slot in &*slots1 {
        if (slot.default).is_none() {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            return Ok((expression, compatibleType, matchKind));
        }
    }
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchFunctionParameters(
    mut params1: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut params2: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut options: MatchOptions,
) -> Result<bool> {
    let mut matching: bool = true;
    let mut pl1: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = params1;
    let mut pl2: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = params2;
    let mut p1: metamodelica::Ref<InstNode::InstNode>;
    let mut dummy: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
    let mut mk: MatchKind;
    for mut p2 in &*pl2 {
        if (pl1).is_empty() {
            matching = false;
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pl1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        p1 = metamodelica::Own::own(__pa0);
        pl1 = metamodelica::Own::own(__pa1);
        if !metamodelica::stringEq(
            &(NFInstNode::InstNode::name(&p1)?),
            &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&p2))?),
        ) {
            matching = false;
            break;
        }
        (_, _, mk) = matchTypes(
            Type::unbox(NFInstNode::InstNode::getType(p1)?),
            Type::unbox(NFInstNode::InstNode::getType(p2.clone())?),
            dummy.clone(),
            options,
        )?;
        if mk != MatchKind::EXACT.clone() {
            matching = false;
            break;
        }
    }
    Ok(matching)
}

pub(crate) fn matchEnumerationTypes(
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut type2: &metamodelica::Ref<Type::NFType>,
) -> Result<MatchKind> {
    let mut matchKind: MatchKind;
    let mut lits1: metamodelica::List<ArcStr>;
    let mut lits2: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &((*type1)) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    lits1 = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &((*type2)) {
        Deref @ Type::ENUMERATION { literals: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    lits2 = metamodelica::Own::own(__pa1);
    matchKind = if (List::isEqualOnTrue(lits1, lits2, &fnptr!(stringEqual, ArcStr, ArcStr))?) {
        MatchKind::EXACT.clone()
    } else {
        MatchKind::NOT_COMPATIBLE.clone()
    };
    Ok(matchKind)
}

pub(crate) fn matchArrayExpressions(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: &metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: &metamodelica::Ref<Type::NFType>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp1: metamodelica::Ref<Expression::NFExpression> = exp1;
    let mut exp2: metamodelica::Ref<Expression::NFExpression> = exp2;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut ety1: metamodelica::Ref<Type::NFType>;
    let mut ety2: metamodelica::Ref<Type::NFType>;
    let mut dims1: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*type1)) {
        Deref @ Type::ARRAY { elementType: __pa0, dimensions: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ety1 = metamodelica::Own::own(__pa0);
    dims1 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*type2)) {
        Deref @ Type::ARRAY { elementType: __pa2, dimensions: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ety2 = metamodelica::Own::own(__pa2);
    dims2 = metamodelica::Own::own(__pa3);
    (exp1, exp2, compatibleType, matchKind) = matchExpressions(exp1, ety1, exp2, ety2, options)?;
    (compatibleType, matchKind) = matchArrayDims(&dims1, dims2, compatibleType, matchKind, options)?;
    if isCompatibleMatch(matchKind) {
        exp1 = setRangeSize(exp1, compatibleType.clone())?;
        exp2 = setRangeSize(exp2, compatibleType.clone())?;
    }
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub(crate) fn matchArrayTypes(
    mut arrayType1: &metamodelica::Ref<Type::NFType>,
    mut arrayType2: &metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut ety1: metamodelica::Ref<Type::NFType>;
    let mut ety2: metamodelica::Ref<Type::NFType>;
    let mut dims1: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*arrayType1)) {
        Deref @ Type::ARRAY { elementType: __pa0, dimensions: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ety1 = metamodelica::Own::own(__pa0);
    dims1 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*arrayType2)) {
        Deref @ Type::ARRAY { elementType: __pa2, dimensions: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ety2 = metamodelica::Own::own(__pa2);
    dims2 = metamodelica::Own::own(__pa3);
    (expression, compatibleType, matchKind) = matchTypes(ety1, ety2, expression, options)?;
    (compatibleType, matchKind) = matchArrayDims(&dims1, dims2, compatibleType, matchKind, options)?;
    if isCompatibleMatch(matchKind) {
        expression = setRangeSize(expression, compatibleType.clone())?;
    }
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn keepRangeSize(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut oldTy: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    if Type::isArray(&oldTy) && Type::hasKnownSize(oldTy.clone())? && !(Type::hasKnownSize(ty.clone())?) {
        ty = Type::setArrayElementType(&oldTy, &(Type::arrayElementType(&ty)));
    }
    Ok(ty)
}

pub(crate) fn setRangeSize(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::RANGE { ty: __exp_ty, .. }
            if (Type::hasKnownSize(ty.clone())? && !(Type::hasKnownSize(__exp_ty.clone())?)) =>
        {
            assign_variant_field!(exp => Expression::NFExpression::RANGE; ty = Type::setArrayElementType(&ty, &(Type::arrayElementType(metamodelica::AsArg::as_arg(&__exp_ty)))));
            exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn matchArrayDims(
    mut dims1: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut dims2: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut matchKind: MatchKind,
    mut options: MatchOptions,
) -> Result<(metamodelica::Ref<Type::NFType>, MatchKind)> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    let mut matchKind: MatchKind = matchKind;
    let mut rest_dims2: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = dims2.clone();
    let mut cdims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut dim2: metamodelica::Ref<Dimension::NFDimension>;
    let mut compat: bool;
    let mut match_kind: MatchKind;
    if !(isCompatibleMatch(matchKind)) {
        return Ok((ty, matchKind));
    }
    if ((dims1).len() as i32) != ((dims2).len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((ty, matchKind));
    }
    for mut dim1 in &**dims1 {
        let mut dim1 = dim1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim2 = metamodelica::Own::own(__pa0);
        rest_dims2 = metamodelica::Own::own(__pa1);
        (dim1, compat) = matchDimensions(dim1, dim2)?;
        if !(compat) && !(getOption(options, IGNORE_DIMENSIONS.clone())) {
            matchKind = MatchKind::NOT_COMPATIBLE.clone();
            break;
        }
        cdims = metamodelica::cons(dim1, cdims);
    }
    ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: ty,
        dimensions: metamodelica::Dangerous::listReverseInPlace(cdims),
    });
    Ok((ty, matchKind))
}

pub(crate) fn matchDimensions(
    mut dim1: metamodelica::Ref<Dimension::NFDimension>,
    mut dim2: metamodelica::Ref<Dimension::NFDimension>,
) -> Result<(metamodelica::Ref<Dimension::NFDimension>, bool)> {
    let mut compatibleDim: metamodelica::Ref<Dimension::NFDimension>;
    let mut compatible: bool = true;
    if Dimension::isEqualKnown(&dim1, &dim2)? {
        compatibleDim = dim1;
    } else {
        if !(Dimension::isKnown(&dim1, false)) {
            compatibleDim = dim2;
        } else if !(Dimension::isKnown(&dim2, false)) {
            compatibleDim = dim1;
        } else if Dimension::isResizable(&dim1) && Dimension::isResizable(&dim2) {
            compatibleDim = dim1;
        } else {
            compatibleDim = dim1;
            compatible = false;
        }
    }
    Ok((compatibleDim, compatible))
}

pub(crate) fn matchTupleTypes(
    mut tupleType1: metamodelica::Ref<Type::NFType>,
    mut tupleType2: &metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut compatibleType: metamodelica::Ref<Type::NFType> = tupleType1.clone();
    let mut matchKind: MatchKind = MatchKind::EXACT.clone();
    let mut tyl1: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut tyl2: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut ty1: metamodelica::Ref<Type::NFType>;
    let __pa0 = ::match_deref::match_deref! { match &(tupleType1) {
        Deref @ Type::TUPLE { types: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    tyl1 = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &((*tupleType2)) {
        Deref @ Type::TUPLE { types: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    tyl2 = metamodelica::Own::own(__pa1);
    if ((tyl1).len() as i32) < ((tyl2).len() as i32) {
        matchKind = MatchKind::NOT_COMPATIBLE.clone();
        return Ok((expression, compatibleType, matchKind));
    }
    for mut ty2 in &*tyl2 {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tyl1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty1 = metamodelica::Own::own(__pa2);
        tyl1 = metamodelica::Own::own(__pa3);
        if Type::isUnknown(metamodelica::AsArg::as_arg(&ty2)) {
            continue;
        }
        (_, _, matchKind) = matchTypes(ty1, ty2.clone(), expression.clone(), options)?;
        if matchKind != MatchKind::EXACT.clone() {
            break;
        }
    }
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchBoxedExpressions(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut type1: metamodelica::Ref<Type::NFType>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut type2: metamodelica::Ref<Type::NFType>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp1: metamodelica::Ref<Expression::NFExpression> = exp1;
    let mut exp2: metamodelica::Ref<Expression::NFExpression> = exp2;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    e1 = Expression::unbox(exp1.clone());
    e2 = Expression::unbox(exp2.clone());
    (e1, e2, compatibleType, matchKind) = matchExpressions(e1, Type::unbox(type1), e2, Type::unbox(type2), options)?;
    if isCastMatch(matchKind) {
        exp1 = Expression::r#box(&e1);
        exp2 = Expression::r#box(&e2);
    }
    compatibleType = Type::r#box(&compatibleType);
    Ok((exp1, exp2, compatibleType, matchKind))
}

pub(crate) fn matchConditionalArrayExp(
    mut condExp: metamodelica::Ref<Expression::NFExpression>,
    mut condType: metamodelica::Ref<Type::NFType>,
    mut otherExp: metamodelica::Ref<Expression::NFExpression>,
    mut otherType: metamodelica::Ref<Type::NFType>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut condExp: metamodelica::Ref<Expression::NFExpression> = condExp;
    let mut otherExp: metamodelica::Ref<Expression::NFExpression> = otherExp;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut true_ty: metamodelica::Ref<Type::NFType>;
    let mut false_ty: metamodelica::Ref<Type::NFType>;
    let mut cond_ty: metamodelica::Ref<Type::NFType>;
    let mut comp_ty1: metamodelica::Ref<Type::NFType>;
    let mut comp_ty2: metamodelica::Ref<Type::NFType>;
    let mut e1_1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2_1: metamodelica::Ref<Expression::NFExpression>;
    let mut e1_2: metamodelica::Ref<Expression::NFExpression>;
    let mut e2_2: metamodelica::Ref<Expression::NFExpression>;
    let mut branch: Type::Branch;
    let mut mk1: MatchKind;
    let mut mk2: MatchKind;
    let mut compat1: bool;
    let mut compat2: bool;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa0, falseType: __pa1, matchedBranch: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    true_ty = metamodelica::Own::own(__pa0);
    false_ty = metamodelica::Own::own(__pa1);
    branch = metamodelica::Own::own(__pa2);
    if branch == Type::Branch::NONE.clone() {
        (e1_1, e2_1, comp_ty1, mk1) =
            matchExpressions(condExp.clone(), true_ty, otherExp.clone(), otherType.clone(), options)?;
        (e1_2, e2_2, comp_ty2, mk2) =
            matchExpressions(condExp.clone(), false_ty, otherExp.clone(), otherType, options)?;
        compat1 = isCompatibleMatch(mk1);
        compat2 = isCompatibleMatch(mk2);
        (compatibleType, otherExp, matchKind) = (match (isCompatibleMatch(mk1), isCompatibleMatch(mk2)) {
            (true, true) => {
                cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                    trueType: comp_ty1.clone(),
                    falseType: comp_ty2,
                    matchedBranch: Type::Branch::NONE.clone(),
                });
                condExp = Expression::typeCast(condExp, cond_ty)?;
                (comp_ty1, otherExp, mk1)
            }
            (true, _) => {
                cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                    trueType: comp_ty1.clone(),
                    falseType: comp_ty2,
                    matchedBranch: Type::Branch::TRUE.clone(),
                });
                condExp = Expression::typeCast(e1_1, cond_ty)?;
                (comp_ty1, e2_1, mk1)
            }
            (_, true) => {
                comp_ty1 = Type::setArrayElementType(&comp_ty1, &(Type::arrayElementType(&comp_ty2)));
                cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                    trueType: comp_ty1,
                    falseType: comp_ty2.clone(),
                    matchedBranch: Type::Branch::FALSE.clone(),
                });
                condExp = Expression::typeCast(e1_2, cond_ty)?;
                (comp_ty2, e2_2, mk2)
            }
            _ => (condType, condExp.clone(), mk1),
        });
    } else {
        if branch == Type::Branch::TRUE.clone() {
            (condExp, otherExp, compatibleType, matchKind) =
                matchExpressions(condExp, true_ty, otherExp, otherType, options)?;
            cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                trueType: compatibleType.clone(),
                falseType: false_ty,
                matchedBranch: branch,
            });
        } else {
            (condExp, otherExp, compatibleType, matchKind) =
                matchExpressions(condExp, false_ty, otherExp, otherType, options)?;
            true_ty = Type::setArrayElementType(&true_ty, &(Type::arrayElementType(&compatibleType)));
            cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                trueType: true_ty,
                falseType: compatibleType.clone(),
                matchedBranch: branch,
            });
        }
        if isCompatibleMatch(matchKind) {
            condExp = Expression::typeCast(condExp, cond_ty)?;
        }
    }
    Ok((condExp, otherExp, compatibleType, matchKind))
}

pub(crate) fn matchConditionalArrayTypes(
    mut actualType: metamodelica::Ref<Type::NFType>,
    mut expectedType: &metamodelica::Ref<Type::NFType>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut compatibleType: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
    let mut matchKind: MatchKind = MatchKind::EXACT;
    let mut actual_true_ty: metamodelica::Ref<Type::NFType>;
    let mut actual_false_ty: metamodelica::Ref<Type::NFType>;
    let mut expected_true_ty: metamodelica::Ref<Type::NFType>;
    let mut expected_false_ty: metamodelica::Ref<Type::NFType>;
    let mut true_ty: metamodelica::Ref<Type::NFType>;
    let mut false_ty: metamodelica::Ref<Type::NFType>;
    let mut true_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut false_exp: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(actualType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa0, falseType: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    actual_true_ty = metamodelica::Own::own(__pa0);
    actual_false_ty = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*expectedType)) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa2, falseType: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    expected_true_ty = metamodelica::Own::own(__pa2);
    expected_false_ty = metamodelica::Own::own(__pa3);
    let () = (match &*exp {
        Expression::IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => {
            (true_exp, true_ty, matchKind) =
                matchTypes(actual_true_ty, expected_true_ty, __exp_trueBranch.clone(), options)?;
            if !(isCompatibleMatch(matchKind)) {
                compatibleType = actualType;
                return Ok((exp, compatibleType, matchKind));
            }
            (false_exp, false_ty, matchKind) =
                matchTypes(actual_false_ty, expected_false_ty, __exp_falseBranch.clone(), options)?;
            if !(isCompatibleMatch(matchKind)) {
                compatibleType = actualType;
                return Ok((exp, compatibleType, matchKind));
            }
            compatibleType = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                trueType: true_ty,
                falseType: false_ty,
                matchedBranch: Type::Branch::NONE.clone(),
            });
            exp = metamodelica::Ref::new(Expression::NFExpression::IF {
                ty: compatibleType.clone(),
                condition: __exp_condition.clone(),
                trueBranch: true_exp,
                falseBranch: false_exp,
            });
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((exp, compatibleType, matchKind))
}

pub(crate) fn matchConditionalArrayTypes_cast(
    mut condType: metamodelica::Ref<Type::NFType>,
    mut expectedType: metamodelica::Ref<Type::NFType>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut true_ty: metamodelica::Ref<Type::NFType>;
    let mut false_ty: metamodelica::Ref<Type::NFType>;
    let mut cond_ty: metamodelica::Ref<Type::NFType>;
    let mut comp_ty1: metamodelica::Ref<Type::NFType>;
    let mut comp_ty2: metamodelica::Ref<Type::NFType>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut branch: Type::Branch;
    let mut mk1: MatchKind;
    let mut mk2: MatchKind;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(condType.clone()) {
        Deref @ Type::CONDITIONAL_ARRAY { trueType: __pa0, falseType: __pa1, matchedBranch: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    true_ty = metamodelica::Own::own(__pa0);
    false_ty = metamodelica::Own::own(__pa1);
    branch = metamodelica::Own::own(__pa2);
    if branch == Type::Branch::NONE.clone() {
        (e1, comp_ty1, mk1) = matchTypes(true_ty.clone(), expectedType.clone(), exp.clone(), options)?;
        (e2, comp_ty2, mk2) = matchTypes(false_ty.clone(), expectedType, exp.clone(), options)?;
        (compatibleType, matchKind) = (match (isCompatibleMatch(mk1), isCompatibleMatch(mk2)) {
            (true, true) => {
                cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                    trueType: comp_ty1.clone(),
                    falseType: comp_ty2,
                    matchedBranch: Type::Branch::NONE.clone(),
                });
                exp = Expression::typeCast(exp, cond_ty)?;
                (comp_ty1, mk1)
            }
            (true, _) => {
                cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                    trueType: comp_ty1.clone(),
                    falseType: false_ty,
                    matchedBranch: Type::Branch::TRUE.clone(),
                });
                exp = Expression::typeCast(e1, cond_ty)?;
                (comp_ty1, mk1)
            }
            (_, true) => {
                true_ty = Type::setArrayElementType(&true_ty, &(Type::arrayElementType(&comp_ty2)));
                cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                    trueType: true_ty,
                    falseType: comp_ty2.clone(),
                    matchedBranch: Type::Branch::FALSE.clone(),
                });
                exp = Expression::typeCast(e2, cond_ty)?;
                (comp_ty2, mk2)
            }
            _ => (condType, mk1),
        });
    } else {
        if branch == Type::Branch::TRUE.clone() {
            (exp, compatibleType, matchKind) = matchTypes(true_ty, expectedType, exp, options)?;
            cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                trueType: compatibleType.clone(),
                falseType: false_ty,
                matchedBranch: branch,
            });
        } else {
            (exp, compatibleType, matchKind) = matchTypes(false_ty, expectedType, exp, options)?;
            true_ty = Type::setArrayElementType(&true_ty, &(Type::arrayElementType(&compatibleType)));
            cond_ty = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY {
                trueType: true_ty,
                falseType: compatibleType.clone(),
                matchedBranch: branch,
            });
        }
        if isCompatibleMatch(matchKind) {
            exp = Expression::typeCast(exp, cond_ty)?;
        }
    }
    Ok((exp, compatibleType, matchKind))
}

pub(crate) fn matchTypes_cast(
    mut actualType: metamodelica::Ref<Type::NFType>,
    mut expectedType: metamodelica::Ref<Type::NFType>,
    mut expression: metamodelica::Ref<Expression::NFExpression>,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut expression: metamodelica::Ref<Expression::NFExpression> = expression;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    let mut before: metamodelica::Ref<Expression::NFExpression> = expression.clone();
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((actualType.clone(), expectedType.clone())) {
        (Deref @ Type::INTEGER, Deref @ Type::REAL) => {
            expression = Expression::typeCast(expression, expectedType.clone())?;
            (expectedType, MatchKind::CAST.clone())
        },
        (Deref @ Type::ENUMERATION { .. }, Deref @ Type::INTEGER) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("nonStdEnumerationAsIntegers"))?) => {
            expression = Expression::typeCast(expression, expectedType.clone())?;
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing usage of enumeration expression: ")); __mm_s.push_str(&*Expression::toString(before.clone())?); __mm_s.push_str(&*literal!(" as Integer: ")); __mm_s.push_str(&*Expression::toString(expression.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use Integer(")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(") instead!")); ArcStr::from(__mm_s) })?;
            (expectedType, MatchKind::CAST.clone())
        },
        (Deref @ Type::INTEGER, Deref @ Type::ENUMERATION { .. }) if (Flags::isConfigFlagSet(Flags::ALLOW_NON_STANDARD_MODELICA.clone(), literal!("nonStdIntegersAsEnumeration"))?) => {
            expression = Expression::typeCast(expression, expectedType.clone())?;
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Allowing usage of Integer expression: ")); __mm_s.push_str(&*Expression::toString(before)?); __mm_s.push_str(&*literal!(" as enumeration: ")); __mm_s.push_str(&*Expression::toString(expression.clone())?); __mm_s.push_str(&*literal!(". This is non-standard Modelica, use the actual enumeration instead!")); ArcStr::from(__mm_s) })?;
            (expectedType, MatchKind::CAST.clone())
        },
        (Deref @ Type::TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, _) => {
            (expression, compatibleType, matchKind) = matchTypes((var_field!((*actualType).types, Type::NFType::TUPLE)).head().cloned()?, expectedType, expression, options)?;
            if isCompatibleMatch(matchKind) {
                expression = (match &*expression {
        Expression::TUPLE { elements: __expression_elements, .. } => (__expression_elements).head().cloned()?,
        _ => metamodelica::Ref::new(Expression::NFExpression::TUPLE_ELEMENT { tupleExp: expression.clone(), index: 1, ty: Type::setArrayElementType(&(Expression::typeOf(expression)), &compatibleType) }),
    });
                matchKind = MatchKind::CAST.clone();
            }
            (compatibleType, matchKind)
        },
        (Deref @ Type::UNKNOWN, _) => (expectedType, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::UNKNOWN_ACTUAL.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (_, Deref @ Type::UNKNOWN) => (actualType, if (getOption(options, ALLOW_UNKNOWN.clone())) {MatchKind::UNKNOWN_EXPECTED.clone()} else {MatchKind::NOT_COMPATIBLE.clone()}),
        (Deref @ Type::METABOXED { .. }, _) => {
            expression = Expression::unbox(expression);
            (expression, compatibleType, matchKind) = matchTypes(var_field!((*actualType).ty, Type::NFType::METABOXED).clone(), expectedType, expression, options)?;
            (compatibleType, if (isCompatibleMatch(matchKind)) {MatchKind::CAST.clone()} else {matchKind})
        },
        (_, Deref @ Type::METABOXED { .. }) => {
            (expression, compatibleType, matchKind) = matchTypes(actualType, var_field!((*expectedType).ty, Type::NFType::METABOXED).clone(), expression, options)?;
            expression = Expression::r#box(&expression);
            compatibleType = Type::r#box(&compatibleType);
            (compatibleType, if (isCompatibleMatch(matchKind)) {MatchKind::CAST.clone()} else {matchKind})
        },
        (_, Deref @ Type::POLYMORPHIC { .. }) => {
            (expression, compatibleType, matchKind) = matchPolymorphic(var_field!((*expectedType).name, Type::NFType::POLYMORPHIC), actualType, expression)?;
            (compatibleType, matchKind)
        },
        (Deref @ Type::POLYMORPHIC { .. }, _) => (expectedType, MatchKind::GENERIC.clone()),
        (_, Deref @ Type::ANY) => (expectedType, MatchKind::EXACT.clone()),
        (Deref @ Type::CONDITIONAL_ARRAY { .. }, _) => {
            (expression, compatibleType, matchKind) = matchConditionalArrayTypes_cast(actualType, expectedType, expression, options)?;
            (compatibleType, matchKind)
        },
        _ => (crate::NFType::interned_UNKNOWN(), MatchKind::NOT_COMPATIBLE.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((expression, compatibleType, matchKind))
}

pub(crate) fn matchPolymorphic(
    mut polymorphicName: &ArcStr,
    mut actualType: metamodelica::Ref<Type::NFType>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &(polymorphicName.clone()) {
        Deref @ "__Any" => (actualType, MatchKind::GENERIC.clone()),
        Deref @ "__Scalar" => {
            matchKind = if (Type::isScalar(&actualType)) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__Array" => {
            matchKind = if (Type::isArray(&actualType)) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__Connector" => {
            matchKind = if (Type::isScalar(&actualType) && Expression::isConnector(&exp)?) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__ComponentExpression" => {
            matchKind = if (Type::isScalar(&actualType) && Expression::isComponentExpression(&exp)?) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        Deref @ "__Block" => {
            matchKind = if (Type::isComplex(&actualType)) {MatchKind::GENERIC.clone()} else {MatchKind::NOT_COMPATIBLE.clone()};
            (actualType, matchKind)
        },
        _ => {
            exp = Expression::r#box(&exp);
            (metamodelica::Ref::new(Type::NFType::METABOXED { ty: actualType }), MatchKind::GENERIC.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, compatibleType, matchKind))
}

pub(crate) fn getRangeType(
    mut startExp: metamodelica::Ref<Expression::NFExpression>,
    mut stepExp: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut stopExp: metamodelica::Ref<Expression::NFExpression>,
    mut rangeElemType: metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut rangeType: metamodelica::Ref<Type::NFType>;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    dim = (match &*rangeElemType {
        Type::INTEGER => getRangeTypeInt(startExp, stepExp, stopExp, info)?,
        Type::REAL => getRangeTypeReal(startExp, stepExp, stopExp, info)?,
        Type::BOOLEAN => {
            if (stepExp).is_some() {
                Error::addSourceMessageAndFail(
                    &(Error::RANGE_INVALID_STEP.clone()),
                    list![Type::toString(&rangeElemType)?],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            getRangeTypeBool(startExp, stopExp)?
        }
        Type::ENUMERATION { .. } => {
            if (stepExp).is_some() {
                Error::addSourceMessageAndFail(
                    &(Error::RANGE_INVALID_STEP.clone()),
                    list![Type::toString(&rangeElemType)?],
                    info,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            getRangeTypeEnum(startExp, stopExp)?
        }
        _ => {
            Error::addSourceMessage(
                &(Error::RANGE_INVALID_TYPE.clone()),
                list![Type::toString(&rangeElemType)?],
                info,
            )?;
            return Err("fail");
        }
    });
    rangeType = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: rangeElemType,
        dimensions: list![dim],
    });
    Ok(rangeType)
}

pub(crate) fn getRangeTypeInt(
    mut startExp: metamodelica::Ref<Expression::NFExpression>,
    mut stepExp: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut stopExp: metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone(), stopExp.clone())) {
        (Deref @ Expression::INTEGER { .. }, None, Deref @ Expression::INTEGER { .. }) => {
            Dimension::fromInteger(std::cmp::max(var_field!((*stopExp).value, Expression::NFExpression::INTEGER).clone() - var_field!((*startExp).value, Expression::NFExpression::INTEGER).clone() + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::INTEGER { .. }, Some(Deref @ Expression::INTEGER { value: step }), Deref @ Expression::INTEGER { .. }) => {
            if step.clone() == 0 {
                Error::addSourceMessageAndFail(&(Error::RANGE_TOO_SMALL_STEP.clone()), list![ArcStr::from(::std::format!("{}", step.clone()))], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            Dimension::fromInteger(std::cmp::max(intDiv(var_field!((*stopExp).value, Expression::NFExpression::INTEGER).clone() - var_field!((*startExp).value, Expression::NFExpression::INTEGER).clone(), step.clone()) + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::INTEGER { value: 1 }, None, _) => {
            let mut dim_exp: metamodelica::Ref<Expression::NFExpression>;
            dim_exp = SimplifyExp::simplify(stopExp.clone(), false)?;
            Dimension::fromExp(dim_exp.clone(), Expression::variability(dim_exp)?)?
        },
        (_, None, _) if (Expression::isEqual(startExp.clone(), stopExp.clone())?) => {
            Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut step_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut dim_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut var: Variability;
            let mut pur: Purity;
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: stopExp.clone(), operator: Operator::makeSub(crate::NFType::interned_INTEGER()), exp2: startExp.clone() });
            var = Prefixes::variabilityMax(Expression::variability(stopExp.clone())?, Expression::variability(startExp.clone())?);
            pur = Prefixes::purityMin(Expression::purity(stopExp.clone())?, Expression::purity(startExp.clone())?);
            if (stepExp).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(stepExp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                step_exp = metamodelica::Own::own(__pa0);
                var = Prefixes::variabilityMax(var, Expression::variability(step_exp.clone())?);
                pur = Prefixes::purityMin(pur, Expression::purity(step_exp.clone())?);
                dim_exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::DIV_INT().clone(), list![dim_exp, step_exp], var, pur, NFBuiltinFuncs::DIV_INT().returnType.clone()) });
            }
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: dim_exp, operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }) });
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_INT().clone(), list![dim_exp, metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })], var, pur, NFBuiltinFuncs::MAX_INT().returnType.clone()) });
            dim_exp = SimplifyExp::simplify(dim_exp, false)?;
            Dimension::fromExp(dim_exp, var)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn getRangeTypeReal(
    mut startExp: metamodelica::Ref<Expression::NFExpression>,
    mut stepExp: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut stopExp: metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stepExp.clone(), stopExp.clone())) {
        (Deref @ Expression::REAL { .. }, None, Deref @ Expression::REAL { .. }) => {
            Dimension::fromInteger(Util::realRangeSize(var_field!((*startExp).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat(1.0_f64), var_field!((*stopExp).value, Expression::NFExpression::REAL).clone())?, Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::REAL { value: start }, Some(Deref @ Expression::REAL { value: step }), Deref @ Expression::REAL { .. }) => {
            if start.clone() == start.clone() + step.clone() {
                Error::addSourceMessageAndFail(&(Error::RANGE_TOO_SMALL_STEP.clone()), list![ArcStr::from(::std::format!("{}", step.clone()))], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            Dimension::fromInteger(Util::realRangeSize(var_field!((*startExp).value, Expression::NFExpression::REAL).clone(), step.clone(), var_field!((*stopExp).value, Expression::NFExpression::REAL).clone())?, Prefixes::Variability::CONSTANT.clone())
        },
        (_, None, _) if (Expression::isEqual(startExp.clone(), stopExp.clone())?) => {
            Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut dim_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut step_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut var: Variability;
            let mut pur: Purity;
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: stopExp.clone(), operator: Operator::makeSub(crate::NFType::interned_REAL()), exp2: startExp.clone() });
            var = Prefixes::variabilityMax(Expression::variability(stopExp.clone())?, Expression::variability(startExp.clone())?);
            pur = Prefixes::purityMin(Expression::purity(stopExp.clone())?, Expression::purity(startExp.clone())?);
            if (stepExp).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(stepExp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                step_exp = metamodelica::Own::own(__pa0);
                var = Prefixes::variabilityMax(var, Expression::variability(step_exp.clone())?);
                pur = Prefixes::purityMin(pur, Expression::purity(step_exp.clone())?);
                dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: dim_exp, operator: Operator::makeDiv(crate::NFType::interned_REAL()), exp2: step_exp });
                dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: dim_exp, operator: Operator::makeAdd(crate::NFType::interned_REAL()), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(5e-15_f64) }) });
            }
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FLOOR().clone(), list![dim_exp], var, pur, NFBuiltinFuncs::FLOOR().returnType.clone()) });
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INTEGER_REAL().clone(), list![dim_exp], var, pur, NFBuiltinFuncs::INTEGER_REAL().returnType.clone()) });
            dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: dim_exp, operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }) });
            dim_exp = SimplifyExp::simplify(dim_exp, false)?;
            Dimension::fromExp(dim_exp, var)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn getRangeTypeBool(
    mut startExp: metamodelica::Ref<Expression::NFExpression>,
    mut stopExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stopExp.clone())) {
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => {
            let mut sz: i32;
            sz = if (var_field!((*startExp).value, Expression::NFExpression::BOOLEAN).clone() == var_field!((*stopExp).value, Expression::NFExpression::BOOLEAN).clone()) {1} else if (var_field!((*startExp).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*stopExp).value, Expression::NFExpression::BOOLEAN).clone()) {2} else {0};
            Dimension::fromInteger(sz, Prefixes::Variability::CONSTANT.clone())
        },
        _ => {
            let mut dim_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut var: Variability;
            if Expression::isEqual(startExp.clone(), stopExp.clone())? {
                dim = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            } else {
                var = Prefixes::variabilityMax(Expression::variability(startExp.clone())?, Expression::variability(stopExp.clone())?);
                dim_exp = metamodelica::Ref::new(Expression::NFExpression::IF { ty: crate::NFType::interned_INTEGER(), condition: metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: startExp.clone(), operator: Operator::makeEqual(crate::NFType::interned_BOOLEAN()), exp2: stopExp.clone(), index: -1 }), trueBranch: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), falseBranch: metamodelica::Ref::new(Expression::NFExpression::IF { ty: crate::NFType::interned_INTEGER(), condition: metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: startExp, operator: Operator::makeLess(crate::NFType::interned_BOOLEAN()), exp2: stopExp, index: -1 }), trueBranch: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 2 }), falseBranch: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }) }) });
                dim_exp = SimplifyExp::simplify(dim_exp, false)?;
                dim = Dimension::fromExp(dim_exp, var)?;
            }
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn getRangeTypeEnum(
    mut startExp: metamodelica::Ref<Expression::NFExpression>,
    mut stopExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    dim = (::match_deref::match_deref! { match &((startExp.clone(), stopExp.clone())) {
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => {
            Dimension::fromInteger(std::cmp::max(var_field!((*stopExp).index, Expression::NFExpression::ENUM_LITERAL).clone() - var_field!((*startExp).index, Expression::NFExpression::ENUM_LITERAL).clone() + 1, 0), Prefixes::Variability::CONSTANT.clone())
        },
        (Deref @ Expression::ENUM_LITERAL { index: 1, .. }, _) => {
            Dimension::fromExp(stopExp.clone(), Expression::variability(stopExp)?)?
        },
        _ => {
            let mut dim_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut var: Variability;
            if Expression::isEqual(startExp.clone(), stopExp.clone())? {
                dim = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            } else {
                var = Prefixes::variabilityMax(Expression::variability(startExp.clone())?, Expression::variability(stopExp.clone())?);
                dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: Expression::enumIndexExp(startExp)?, operator: Operator::makeSub(crate::NFType::interned_INTEGER()), exp2: Expression::enumIndexExp(stopExp)? });
                dim_exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: dim_exp, operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }) });
                dim_exp = SimplifyExp::simplify(dim_exp, false)?;
                dim = Dimension::fromExp(dim_exp, var)?;
            }
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn matchBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut componentType: metamodelica::Ref<Type::NFType>,
    mut name: ArcStr,
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let () = (match &*binding {
        Binding::TYPED_BINDING {
            bindingExp: exp,
            bindingType: __binding_bindingType,
            ..
        } => {
            let mut ty_match: MatchKind;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut bind_ty: metamodelica::Ref<Type::NFType>;
            let mut comp_ty: metamodelica::Ref<Type::NFType>;
            let mut exp = (*exp).clone();
            (bind_ty, comp_ty) = elaborateBindingType(
                metamodelica::AsArg::as_arg(&exp),
                component.clone(),
                __binding_bindingType.clone(),
                componentType,
            )?;
            (exp, ty, ty_match) = matchTypes(bind_ty.clone(), comp_ty.clone(), exp.clone(), ALLOW_UNKNOWN.clone())?;
            if !(isValidAssignmentMatch(ty_match)) {
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = Expression::expandSplitIndices(exp.clone())?);
                printBindingTypeError(name, &binding, comp_ty, bind_ty, &component, context)?;
                if !(InstContext::inInstanceAPI(context)) {
                    return Err("fail");
                }
            } else if isCastMatch(ty_match) {
                binding = metamodelica::Ref::new(Binding::NFBinding::TYPED_BINDING {
                    bindingExp: exp.clone(),
                    bindingType: ty,
                    variability: var_field!((*binding).variability, Binding::NFBinding::TYPED_BINDING).clone(),
                    purity: var_field!((*binding).purity, Binding::NFBinding::TYPED_BINDING).clone(),
                    eachType: var_field!((*binding).eachType, Binding::NFBinding::TYPED_BINDING).clone(),
                    evalState: var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(),
                    isFlattened: var_field!((*binding).isFlattened, Binding::NFBinding::TYPED_BINDING).clone(),
                    source: var_field!((*binding).source, Binding::NFBinding::TYPED_BINDING).clone(),
                    confidence: var_field!((*binding).confidence, Binding::NFBinding::TYPED_BINDING).clone(),
                    info: var_field!((*binding).info, Binding::NFBinding::TYPED_BINDING).clone(),
                });
            }
            ()
        }
        Binding::UNBOUND => (),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFTypeCheck.matchBinding"));
                    __mm_s.push_str(&*literal!(" got untyped binding "));
                    __mm_s.push_str(&*Binding::toString(&binding, &(literal!("")))?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFTypeCheck.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(binding)
}

pub(crate) fn elaborateBindingType(
    mut bindingExp: &metamodelica::Ref<Expression::NFExpression>,
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut bindingType: metamodelica::Ref<Type::NFType>,
    mut componentType: metamodelica::Ref<Type::NFType>,
) -> Result<(metamodelica::Ref<Type::NFType>, metamodelica::Ref<Type::NFType>)> {
    fn isParent(
        mut parent: &metamodelica::Ref<InstNode::InstNode>,
        mut node: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<bool> {
        let mut res: bool;
        let mut n: metamodelica::Ref<InstNode::InstNode> = NFInstNode::InstNode::getDerivedNode(node.clone(), true)?;
        let mut p: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
        res = (::match_deref::match_deref! { match &(n.clone()) {
            Deref @ NFInstNode::InstNode::COMPONENT_NODE { nodeType: Deref @ NFInstNode::InstNodeType::REDECLARED_COMP { parent: __esc_p }, .. } => {
                p = (*__esc_p).clone();
                NFInstNode::InstNode::refEqual(parent, &n)? || isParent(parent, NFInstNode::InstNode::borrow(p.clone())?)?
            },
            Deref @ NFInstNode::InstNode::COMPONENT_NODE { .. } => NFInstNode::InstNode::refEqual(parent, &n)? || isParent(parent, NFInstNode::InstNode::parent(&n)?)?,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(res)
    }

    let mut bindingType: metamodelica::Ref<Type::NFType> = bindingType;
    let mut componentType: metamodelica::Ref<Type::NFType> = componentType;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let () = (match &**bindingExp {
        Expression::SUBSCRIPTED_EXP {
            exp: __bindingExp_exp,
            subscripts: __bindingExp_subscripts,
            ..
        } => {
            bindingType = Expression::typeOf(__bindingExp_exp.clone());
            dims = metamodelica::nil();
            for mut s in &*__bindingExp_subscripts.clone() {
                dims = (match &*s.clone() {
                    Subscript::SPLIT_INDEX {
                        dimIndex: __s_dimIndex,
                        node: __s_node,
                    } => {
                        if isParent(&(NFInstNode::InstNode::borrow(__s_node.clone())?), component.clone())? {
                            dims = metamodelica::cons(
                                Type::nthDimension(
                                    NFInstNode::InstNode::getType(NFInstNode::InstNode::borrow(__s_node.clone())?)?,
                                    __s_dimIndex.clone(),
                                )?,
                                dims,
                            );
                        }
                        dims
                    }
                    _ => metamodelica::cons(crate::NFDimension::interned_UNKNOWN(), dims),
                });
            }
            dims = metamodelica::Dangerous::listReverseInPlace(dims);
            componentType = Type::liftArrayLeftList(componentType, &dims);
            ()
        }
        Expression::CREF {
            cref: __bindingExp_cref,
            ..
        } => {
            bindingType = ComponentRef::getSubscriptedType(
                &(ComponentRef::expandSplitSubscripts(__bindingExp_cref.clone())?),
                false,
            )?;
            dims = metamodelica::nil();
            for mut s in &*ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&__bindingExp_cref))? {
                dims = (match &*s.clone() {
                    Subscript::SPLIT_INDEX {
                        dimIndex: __s_dimIndex,
                        node: __s_node,
                    } => {
                        if isParent(&(NFInstNode::InstNode::borrow(__s_node.clone())?), component.clone())? {
                            dims = metamodelica::cons(
                                Type::nthDimension(
                                    NFInstNode::InstNode::getType(NFInstNode::InstNode::borrow(__s_node.clone())?)?,
                                    __s_dimIndex.clone(),
                                )?,
                                dims,
                            );
                        }
                        dims
                    }
                    _ => dims,
                });
            }
            dims = metamodelica::Dangerous::listReverseInPlace(dims);
            componentType = Type::liftArrayLeftList(componentType, &dims);
            ()
        }
        _ => (),
    });
    Ok((bindingType, componentType))
}

pub(crate) fn printBindingTypeError(
    mut name: ArcStr,
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut componentType: metamodelica::Ref<Type::NFType>,
    mut bindingType: metamodelica::Ref<Type::NFType>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
) -> Result<()> {
    let mut binding_info: SourceInfo;
    let mut comp_info: SourceInfo;
    let mut mk: MatchKind;
    binding_info = Binding::getInfo(binding);
    comp_info = NFInstNode::InstNode::info(component);
    if Type::isScalar(&bindingType) && Type::isArray(&componentType) {
        Error::addMultiSourceMessage(
            &(Error::MODIFIER_NON_ARRAY_TYPE_ERROR.clone()),
            &(list![Binding::toString(binding, &(literal!("")))?, name]),
            &(list![binding_info, comp_info]),
        )?;
    } else {
        (_, _, mk) = matchTypes(
            Type::arrayElementType(&bindingType),
            Type::arrayElementType(&componentType),
            metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: bindingType.clone(),
            }),
            ALLOW_UNKNOWN.clone(),
        )?;
        if !(InstContext::inAnnotation(context)) {
            if isValidAssignmentMatch(mk) {
                Error::addMultiSourceMessage(
                    &(Error::VARIABLE_BINDING_DIMS_MISMATCH.clone()),
                    &(list![
                        name,
                        Binding::toString(binding, &(literal!("")))?,
                        Dimension::toStringList(Type::arrayDims(componentType), true)?,
                        Dimension::toStringList(Type::arrayDims(bindingType), true)?
                    ]),
                    &(list![binding_info, comp_info]),
                )?;
            } else {
                Error::addMultiSourceMessage(
                    &(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone()),
                    &(list![
                        name,
                        Binding::toString(binding, &(literal!("")))?,
                        Type::toString(&componentType)?,
                        Type::toString(&bindingType)?
                    ]),
                    &(list![binding_info, comp_info]),
                )?;
            }
        }
    }
    Ok(())
}

pub(crate) fn checkDimensionType(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut info: &SourceInfo,
) -> Result<()> {
    if !(Type::isInteger(ty)?) {
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::BOOLEAN, .. } } => (),
            Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::ENUMERATION { .. }, .. } } => (),
            _ => {
                Error::addSourceMessage(&(Error::INVALID_DIMENSION_TYPE.clone()), list![Expression::toString(exp)?, Type::toString(ty)?], info)?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(())
}

pub(crate) fn checkReductionType(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut name: metamodelica::Ref<Absyn::Path>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut err: ArcStr;
    err = (::match_deref::match_deref! { match &(&*name) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } => (match &*(Type::arrayElementType(ty)) {
        Type::INTEGER => literal!(""),
        Type::REAL => literal!(""),
        Type::COMPLEX { .. } if (checkSumComplexType(ty, exp.clone(), info)?) => literal!(""),
        _ => literal!("Integer or Real, or operator record"),
    }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "product" } => (match &**ty {
        Type::INTEGER => literal!(""),
        Type::REAL => literal!(""),
        _ => literal!("scalar Integer or Real"),
    }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "min" } => (match &**ty {
        Type::INTEGER => literal!(""),
        Type::REAL => literal!(""),
        Type::BOOLEAN => literal!(""),
        Type::ENUMERATION { .. } => literal!(""),
        _ => literal!("scalar enumeration, Boolean, Integer, or Real"),
    }),
        Deref @ Absyn::Path::IDENT { name: Deref @ "max" } => (match &**ty {
        Type::INTEGER => literal!(""),
        Type::REAL => literal!(""),
        Type::BOOLEAN => literal!(""),
        Type::ENUMERATION { .. } => literal!(""),
        _ => literal!("scalar enumeration, Boolean, Integer, or Real"),
    }),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(stringEmpty(&err)) {
        Error::addSourceMessageAndFail(
            &(Error::INVALID_REDUCTION_TYPE.clone()),
            list![
                Expression::toString(exp)?,
                Type::toString(ty)?,
                AbsynUtil::pathString(name, literal!("."), true, false)?,
                err
            ],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok(())
}

pub(crate) fn checkSumComplexType(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
) -> Result<bool> {
    let mut valid: bool = true;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    cls_node = Type::complexNode(ty)?;
    cls = NFInstNode::InstNode::getClass(cls_node)?;
    for mut op in &*list![literal!("'+'"), literal!("'0'")] {
        if !(Class::hasOperator(op.clone(), cls.clone())) {
            Error::addSourceMessage(
                &(Error::OPERATOR_RECORD_MISSING_OPERATOR.clone()),
                list![
                    Type::toString(ty)?,
                    Expression::toString(exp.clone())?,
                    literal!("sum"),
                    op.clone()
                ],
                info,
            )?;
            valid = false;
        }
    }
    Ok(valid)
}

pub(crate) fn matchIfBranches(
    mut trueBranch: metamodelica::Ref<Expression::NFExpression>,
    mut trueType: metamodelica::Ref<Type::NFType>,
    mut falseBranch: metamodelica::Ref<Expression::NFExpression>,
    mut falseType: metamodelica::Ref<Type::NFType>,
    mut context: i32,
    mut options: MatchOptions,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    MatchKind,
)> {
    let mut trueBranch: metamodelica::Ref<Expression::NFExpression> = trueBranch;
    let mut falseBranch: metamodelica::Ref<Expression::NFExpression> = falseBranch;
    let mut compatibleType: metamodelica::Ref<Type::NFType>;
    let mut matchKind: MatchKind;
    (compatibleType, matchKind) = (::match_deref::match_deref! { match &((&*trueType, &*falseType)) {
        (Deref @ Type::ARRAY { .. }, Deref @ Type::ARRAY { .. }) => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch, var_field!((*trueType).elementType, Type::NFType::ARRAY).clone(), falseBranch, var_field!((*falseType).elementType, Type::NFType::ARRAY).clone(), options)?;
            if isIncompatibleMatch(matchKind) {
                return Ok((trueBranch, falseBranch, compatibleType, matchKind));
            }
            (compatibleType, matchKind) = matchArrayDims(var_field!((*trueType).dimensions, Type::NFType::ARRAY), var_field!((*falseType).dimensions, Type::NFType::ARRAY).clone(), compatibleType, matchKind, options)?;
            if ((var_field!((*trueType).dimensions, Type::NFType::ARRAY)).len() as i32) == ((var_field!((*falseType).dimensions, Type::NFType::ARRAY)).len() as i32) && (isIncompatibleMatch(matchKind) || !(List::isEqualOnTrue(var_field!((*trueType).dimensions, Type::NFType::ARRAY).clone(), var_field!((*falseType).dimensions, Type::NFType::ARRAY).clone(), &move |__a0: metamodelica::Ref<Dimension::NFDimension>, __a1: metamodelica::Ref<Dimension::NFDimension>| Dimension::isSame(&__a0, &__a1))?)) {
                if InstContext::inSubexpression(context) || InstContext::inFunction(context) {
                    compatibleType = Type::unifyArrays(Type::copyElementType(&trueType, &compatibleType), Type::copyElementType(&falseType, &compatibleType))?;
                } else {
                    compatibleType = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Type::copyElementType(&trueType, &compatibleType), falseType: Type::copyElementType(&falseType, &compatibleType), matchedBranch: Type::Branch::NONE.clone() });
                }
                matchKind = MatchKind::EXACT.clone();
            }
            (compatibleType, matchKind)
        },
        (_, _) if (Type::isConditionalArray(&trueType) || Type::isConditionalArray(&falseType)) => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch, Type::arrayElementType(&trueType), falseBranch, Type::arrayElementType(&falseType), options)?;
            if isIncompatibleMatch(matchKind) {
                return Ok((trueBranch, falseBranch, compatibleType, matchKind));
            }
            compatibleType = metamodelica::Ref::new(Type::NFType::CONDITIONAL_ARRAY { trueType: Type::copyElementType(&trueType, &compatibleType), falseType: Type::copyElementType(&falseType, &compatibleType), matchedBranch: Type::Branch::NONE.clone() });
            (compatibleType, matchKind)
        },
        _ => {
            (trueBranch, falseBranch, compatibleType, matchKind) = matchExpressions(trueBranch, trueType.clone(), falseBranch, falseType.clone(), options)?;
            (compatibleType, matchKind)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((trueBranch, falseBranch, compatibleType, matchKind))
}
