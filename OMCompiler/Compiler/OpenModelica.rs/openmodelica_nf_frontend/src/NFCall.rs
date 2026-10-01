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

use crate::BaseModelica;
use crate::NFBinding as Binding;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCallAttributes;
use crate::NFCallParameterTree;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFFunction::FunctionMatchKind;
use crate::NFFunction::MatchedFunction;
use crate::NFFunction::NamedArg;
use crate::NFFunction::TypedArg;
use crate::NFInline as Inline;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFLookup as Lookup;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFRestriction as Restriction;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFCall {
    UNTYPED_CALL {
        r#ref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>,
        /// Weakly: the scope owns the class this call
        ///      sits in.
        call_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    },
    ARG_TYPED_CALL {
        r#ref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        positional_args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        named_args: metamodelica::List<metamodelica::Ref<TypedArg>>,
        /// See UNTYPED_CALL.call_scope.
        call_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    },
    TYPED_CALL {
        r#fn: metamodelica::Ref<Function::Function>,
        ty: metamodelica::Ref<Type::NFType>,
        var: Variability,
        purity: Purity,
        arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        attributes: metamodelica::Ref<NFCallAttributes::NFCallAttributes>,
    },
    UNTYPED_ARRAY_CONSTRUCTOR {
        exp: metamodelica::Ref<Expression::NFExpression>,
        iters: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
    },
    TYPED_ARRAY_CONSTRUCTOR {
        ty: metamodelica::Ref<Type::NFType>,
        var: Variability,
        purity: Purity,
        exp: metamodelica::Ref<Expression::NFExpression>,
        iters: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
    },
    UNTYPED_REDUCTION {
        r#ref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        exp: metamodelica::Ref<Expression::NFExpression>,
        iters: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
    },
    TYPED_REDUCTION {
        r#fn: metamodelica::Ref<Function::Function>,
        ty: metamodelica::Ref<Type::NFType>,
        var: Variability,
        purity: Purity,
        exp: metamodelica::Ref<Expression::NFExpression>,
        iters: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
        defaultExp: Option<metamodelica::Ref<Expression::NFExpression>>,
        foldExp: (Option<metamodelica::Ref<Expression::NFExpression>>, ArcStr, ArcStr),
    },
}
impl metamodelica::gc::MMTrace for NFCall {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFCall::UNTYPED_CALL {
                r#ref,
                arguments,
                named_args,
                call_scope,
            } => {
                metamodelica::gc::MMTrace::mm_accept(r#ref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(arguments, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(named_args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(call_scope, __mmv)?;
                Ok(())
            }
            NFCall::ARG_TYPED_CALL {
                r#ref,
                positional_args,
                named_args,
                call_scope,
            } => {
                metamodelica::gc::MMTrace::mm_accept(r#ref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(positional_args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(named_args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(call_scope, __mmv)?;
                Ok(())
            }
            NFCall::TYPED_CALL {
                r#fn,
                ty,
                var,
                purity,
                arguments,
                attributes,
            } => {
                metamodelica::gc::MMTrace::mm_accept(r#fn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(purity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(arguments, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                Ok(())
            }
            NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp, iters } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                Ok(())
            }
            NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty,
                var,
                purity,
                exp,
                iters,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(purity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                Ok(())
            }
            NFCall::UNTYPED_REDUCTION { r#ref, exp, iters } => {
                metamodelica::gc::MMTrace::mm_accept(r#ref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                Ok(())
            }
            NFCall::TYPED_REDUCTION {
                r#fn,
                ty,
                var,
                purity,
                exp,
                iters,
                defaultExp,
                foldExp,
            } => {
                metamodelica::gc::MMTrace::mm_accept(r#fn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(purity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(defaultExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(foldExp, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFCall {
    fn default() -> Self {
        Self::UNTYPED_ARRAY_CONSTRUCTOR {
            exp: Default::default(),
            iters: Default::default(),
        }
    }
}
pub use self::NFCall::{
    ARG_TYPED_CALL, TYPED_ARRAY_CONSTRUCTOR, TYPED_CALL, TYPED_REDUCTION, UNTYPED_ARRAY_CONSTRUCTOR, UNTYPED_CALL,
    UNTYPED_REDUCTION,
};
pub type ParameterTree = metamodelica::Ref<NFCallParameterTree::Tree>;

pub(crate) fn instantiate(
    mut functionName: metamodelica::Ref<Absyn::ComponentRef>,
    mut functionArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    callExp = (match &**functionArgs {
        Absyn::FunctionArgs::FUNCTIONARGS { .. } => instNormalCall(functionName, functionArgs, scope, context, info)?,
        Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            instIteratorCall(functionName, functionArgs, scope, context, info)?
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.instantiate"));
                    __mm_s.push_str(&*literal!(" got unknown call type"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(callExp)
}

pub(crate) fn typeCall(
    mut callExp: metamodelica::Ref<Expression::NFExpression>,
    mut context: i32,
    mut info: SourceInfo,
    mut retype: bool,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    let mut pur: Purity = Purity::PURE;
    let mut call: metamodelica::Ref<NFCall>;
    let mut ty_call: metamodelica::Ref<NFCall>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    outExp = (match &*call {
        UNTYPED_CALL { r#ref: __esc_cref, .. } => {
            cref = (*__esc_cref).clone();
            if BuiltinCall::needSpecialHandling(&call)? {
                (outExp, ty, var, pur) = BuiltinCall::typeSpecial(call.clone(), context, info)?;
            } else {
                checkNotPartial(metamodelica::AsArg::as_arg(&cref), context, &info)?;
                ty_call = typeMatchNormalCall(call.clone(), context, info, true)?;
                (outExp, ty, var, pur) = typeCallExp(ty_call)?;
            }
            outExp
        }
        UNTYPED_ARRAY_CONSTRUCTOR { .. } => {
            (ty_call, ty, var, pur) = typeArrayConstructor(call.clone(), context, info)?;
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call })
        }
        UNTYPED_REDUCTION { r#ref: __call_ref, .. } => {
            checkNotPartial(metamodelica::AsArg::as_arg(&__call_ref), context, &info)?;
            (ty_call, ty, var, pur) = typeReduction(call.clone(), context, info)?;
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call })
        }
        TYPED_CALL { .. } if (retype && !(BuiltinCall::needSpecialHandling(&call)?)) => {
            ty_call = retypeCall(&call, context, &info)?;
            (outExp, ty, var, pur) = typeCallExp(ty_call)?;
            outExp
        }
        TYPED_CALL {
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
            ..
        } => {
            ty = __call_ty.clone();
            var = __call_var.clone();
            pur = __call_purity.clone();
            callExp
        }
        TYPED_ARRAY_CONSTRUCTOR {
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
            ..
        } => {
            ty = __call_ty.clone();
            var = __call_var.clone();
            pur = __call_purity.clone();
            callExp
        }
        TYPED_REDUCTION {
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
            ..
        } => {
            ty = __call_ty.clone();
            var = __call_var.clone();
            pur = __call_purity.clone();
            callExp
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.typeCall"));
                    __mm_s.push_str(&*literal!(": "));
                    __mm_s.push_str(&*Expression::toString(callExp)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((outExp, ty, var, pur))
}

pub(crate) fn checkNotPartial(
    mut fnRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<()> {
    if NFInstNode::InstNode::isPartial(&(ComponentRef::node(fnRef)?))? && !(InstContext::inRelaxed(context)) {
        Error::addSourceMessage(
            &(Error::PARTIAL_FUNCTION_CALL.clone()),
            list![ComponentRef::toString(fnRef)?],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn typeCallExp(
    mut ty_call: metamodelica::Ref<NFCall>,
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
    ty = typeOf(&ty_call);
    var = variability(&ty_call)?;
    pur = purity(&ty_call);
    if isRecordConstructor(&ty_call)? {
        outExp = toRecordExpression(&ty_call, ty.clone())?;
    } else {
        if Function::hasUnboxArgs(&(typedFunction(&ty_call)?)) {
            outExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
                call: unboxArgs(ty_call.clone()),
            });
        } else {
            outExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: ty_call.clone() });
        }
        outExp = Inline::inlineCallExp(outExp, false)?;
        if Type::isBoxed(&ty)
            && isUnboxableType(Type::unbox(ty.clone()))?
            && Function::isFunctionPointer(&(typedFunction(&ty_call)?))
        {
            ty = Type::unbox(ty);
            outExp = metamodelica::Ref::new(Expression::NFExpression::UNBOX {
                exp: outExp,
                ty: ty.clone(),
            });
        }
    }
    Ok((outExp, ty, var, pur))
}

pub(crate) fn isUnboxableType(mut ty: metamodelica::Ref<Type::NFType>) -> Result<bool> {
    let mut unboxable: bool = Type::isScalarBuiltin(ty.clone())? || Type::isRecord(&ty);
    Ok(unboxable)
}

pub(crate) fn typeNormalCall(
    mut call: metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall> = call;
    call = (match &*call {
        UNTYPED_CALL { r#ref: __call_ref, .. } => {
            let mut fn_context: i32;
            if InstContext::inRelaxed(context) {
                fn_context = InstContext::set(InstContext::FUNCTION.clone(), InstContext::RELAXED.clone());
            } else {
                fn_context = InstContext::FUNCTION.clone();
            }
            Function::typeRefCache(metamodelica::AsArg::as_arg(&__call_ref), fn_context)?;
            typeArgs(call, context, info)?
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.typeNormalCall"));
                    __mm_s.push_str(&*literal!(" got invalid function call expression"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(call)
}

pub fn makeTypedCall(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut variability: Variability,
    mut purity: Purity,
    mut returnType: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<NFCall> {
    let mut call: metamodelica::Ref<NFCall>;
    let mut ca: metamodelica::Ref<NFCallAttributes::NFCallAttributes>;
    ca = metamodelica::Ref::new(NFCallAttributes::NFCallAttributes {
        tuple_: Type::isTuple(&returnType),
        builtin: Function::isBuiltin(&r#fn),
        isImpure: Function::isImpure(&r#fn),
        isFunctionPointerCall: Function::isFunctionPointer(&r#fn),
        inlineType: Function::inlineBuiltin(&r#fn),
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
    });
    call = metamodelica::Ref::new(NFCall::TYPED_CALL {
        r#fn: r#fn,
        ty: returnType,
        var: variability,
        purity: purity,
        arguments: args,
        attributes: ca,
    });
    call
}

pub(crate) fn unboxArgs(mut call: metamodelica::Ref<NFCall>) -> metamodelica::Ref<NFCall> {
    let mut call: metamodelica::Ref<NFCall> = call;
    let mut c: metamodelica::Ref<NFCall>;
    let () = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ TYPED_CALL { arguments: __call_arguments, .. } => {
            assign_variant_field!(call => NFCall::TYPED_CALL; arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (__call_arguments.clone()).into_iter().cloned() {
            let __x = Expression::unbox(arg.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        Deref @ TYPED_ARRAY_CONSTRUCTOR { exp: Deref @ Expression::CALL { call: __esc_c }, .. } => {
            c = (*__esc_c).clone();
            assign_variant_field!(call => NFCall::TYPED_ARRAY_CONSTRUCTOR; exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: unboxArgs(c.clone()) }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    call
}

pub(crate) fn typeMatchNormalCall(
    mut call: metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: SourceInfo,
    mut vectorize: bool,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall> = call;
    let mut argtycall: metamodelica::Ref<NFCall>;
    argtycall = typeNormalCall(call, context, &info)?;
    call = matchTypedNormalCall(argtycall, context, info, vectorize)?;
    Ok(call)
}

pub(crate) fn matchTypedNormalCall(
    mut call: metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: SourceInfo,
    mut vectorize: bool,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall> = call;
    let mut func: metamodelica::Ref<Function::Function>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut typed_args: metamodelica::List<metamodelica::Ref<TypedArg>>;
    let mut matchedFunc: metamodelica::Ref<MatchedFunction::MatchedFunction>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut var: Variability;
    let mut arg_var: Variability;
    let mut pur: Purity;
    let mut arg_pur: Purity;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arg_exp: metamodelica::Ref<Expression::NFExpression>;
    let __pa0 = ::match_deref::match_deref! { match &(call.clone()) {
        Deref @ ARG_TYPED_CALL { call_scope: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    scope = metamodelica::Own::own(__pa0);
    matchedFunc = checkMatchingFunctions(&call, context, &info, vectorize)?;
    func = matchedFunc.func.clone();
    typed_args = matchedFunc.args.clone();
    args = metamodelica::nil();
    var = Variability::CONSTANT.clone();
    pur = if (Function::isImpure(&func)) {
        Purity::IMPURE.clone()
    } else {
        Purity::PURE.clone()
    };
    for mut a in &*typed_args {
        let __arc4 = a.clone();
        let TypedArg {
            value: __pa1,
            var: __pa2,
            purity: __pa3,
            ..
        } = &*__arc4;
        arg_exp = metamodelica::Own::own(__pa1);
        arg_var = metamodelica::Own::own(__pa2);
        arg_pur = metamodelica::Own::own(__pa3);
        args = metamodelica::cons(arg_exp, args);
        var = Prefixes::variabilityMax(var, arg_var);
        pur = Prefixes::purityMin(pur, arg_pur);
    }
    args = metamodelica::Dangerous::listReverseInPlace(args);
    ty = Function::returnType(&func);
    ty = resolvePolymorphicReturnType(&func, typed_args, ty)?;
    if var == Variability::PARAMETER.clone() && Function::isExternal(&func)? {
        var = Variability::NON_STRUCTURAL_PARAMETER.clone();
    } else if Type::isDiscrete(ty.clone())? && var == Variability::CONTINUOUS.clone() {
        var = Variability::IMPLICITLY_DISCRETE.clone();
    }
    (ty, _) = evaluateCallType(ty, &func, &args, 1, crate::NFCallParameterTree::Tree::interned_EMPTY())?;
    call = makeTypedCall(func.clone(), args.clone(), var, pur, ty.clone());
    if MatchedFunction::isVectorized(&matchedFunc) {
        call = vectorizeCall(call, &matchedFunc.mk, scope, info)?;
    }
    if Function::isExternal(&func)? {
        updateExternalRecordArgs(&args)?;
        updateExternalRecordArgsInType(&ty)?;
    }
    Ok(call)
}

pub(crate) fn retypeCall(
    mut call: &metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut ty_call: metamodelica::Ref<NFCall>;
    let mut next_context: i32;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arg_ty: metamodelica::Ref<Type::NFType>;
    let mut arg_var: Variability;
    let mut arg_pur: Purity;
    let mut typed_args: metamodelica::List<metamodelica::Ref<TypedArg>> = metamodelica::nil();
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    ty_call = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            purity: __call_purity,
            var: __call_var,
            ..
        } => {
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            for mut arg in &*__call_arguments.clone().reverse() {
                let mut arg = arg.clone();
                (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg, next_context, info, true)?;
                typed_args = metamodelica::cons(
                    metamodelica::Ref::new(TypedArg {
                        name: None,
                        value: arg.clone(),
                        ty: arg_ty,
                        var: arg_var,
                        purity: arg_pur,
                    }),
                    typed_args,
                );
                args = metamodelica::cons(arg, args);
            }
            ty = Function::returnType(metamodelica::AsArg::as_arg(&__call_fn));
            ty = resolvePolymorphicReturnType(metamodelica::AsArg::as_arg(&__call_fn), typed_args, ty)?;
            (ty, _) = evaluateCallType(
                ty,
                metamodelica::AsArg::as_arg(&__call_fn),
                &args,
                1,
                crate::NFCallParameterTree::Tree::interned_EMPTY(),
            )?;
            ty_call = makeTypedCall(__call_fn.clone(), args, __call_var.clone(), __call_purity.clone(), ty);
            ty_call
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.retypeCall"));
                    __mm_s.push_str(&*literal!(" got invalid function call expression"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(ty_call)
}

pub(crate) fn typeOf(mut call: &metamodelica::Ref<NFCall>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = (match &**call {
        TYPED_CALL { ty: __call_ty, .. } => __call_ty.clone(),
        TYPED_ARRAY_CONSTRUCTOR { ty: __call_ty, .. } => __call_ty.clone(),
        TYPED_REDUCTION { ty: __call_ty, .. } => __call_ty.clone(),
        _ => crate::NFType::interned_UNKNOWN(),
    });
    ty
}

pub(crate) fn setType(
    mut call: metamodelica::Ref<NFCall>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall> = call;
    call = (match &*call {
        TYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::TYPED_CALL; ty = ty);
            call
        }
        TYPED_ARRAY_CONSTRUCTOR { .. } => {
            assign_variant_field!(call => NFCall::TYPED_ARRAY_CONSTRUCTOR; ty = ty);
            call
        }
        TYPED_REDUCTION { .. } => {
            assign_variant_field!(call => NFCall::TYPED_REDUCTION; ty = ty);
            call
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(call)
}

pub(crate) fn variability(mut call: &metamodelica::Ref<NFCall>) -> Result<Variability> {
    let mut var: Variability = Variability::CONTINUOUS.clone();
    var = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            r#ref: __call_ref,
            ..
        } => {
            let mut var_set: bool;
            var_set = true;
            if ComponentRef::isSimple(metamodelica::AsArg::as_arg(&__call_ref)) {
                var = (::match_deref::match_deref! { match &(ComponentRef::firstName(metamodelica::AsArg::as_arg(&__call_ref), false)?) {
                    Deref @ "change" => Variability::DISCRETE.clone(),
                    Deref @ "edge" => Variability::DISCRETE.clone(),
                    Deref @ "pre" => Variability::DISCRETE.clone(),
                    Deref @ "ndims" => Variability::PARAMETER.clone(),
                    Deref @ "cardinality" => Variability::PARAMETER.clone(),
                    _ => {
                        var_set = false;
                        Variability::CONTINUOUS.clone()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            } else {
                var_set = false;
            }
            if !(var_set) {
                var = Expression::variabilityList(
                    metamodelica::AsArg::as_arg(&__call_arguments),
                    Prefixes::Variability::CONSTANT.clone(),
                )?;
                for mut narg in &*__call_named_args.clone() {
                    var = Prefixes::variabilityMax(var, Expression::variability(Util::tuple22(narg.clone()))?);
                }
            }
            var
        }
        UNTYPED_ARRAY_CONSTRUCTOR { exp: __call_exp, .. } => Expression::variability(__call_exp.clone())?,
        UNTYPED_REDUCTION { exp: __call_exp, .. } => Expression::variability(__call_exp.clone())?,
        TYPED_CALL { var: __call_var, .. } => __call_var.clone(),
        TYPED_ARRAY_CONSTRUCTOR { var: __call_var, .. } => __call_var.clone(),
        TYPED_REDUCTION { var: __call_var, .. } => __call_var.clone(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.variability"));
                    __mm_s.push_str(&*literal!(" got untyped call"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(var)
}

pub(crate) fn purity(mut call: &metamodelica::Ref<NFCall>) -> Purity {
    let mut purity: Purity;
    purity = (match &**call {
        TYPED_CALL {
            purity: __call_purity, ..
        } => __call_purity.clone(),
        TYPED_ARRAY_CONSTRUCTOR {
            purity: __call_purity, ..
        } => __call_purity.clone(),
        TYPED_REDUCTION {
            purity: __call_purity, ..
        } => __call_purity.clone(),
        _ => Purity::PURE.clone(),
    });
    purity
}

pub(crate) fn compare(mut call1: &metamodelica::Ref<NFCall>, mut call2: &metamodelica::Ref<NFCall>) -> Result<i32> {
    let mut comp: i32;
    comp = AbsynUtil::pathCompare(&(functionName(call1)?), &(functionName(call2)?))?;
    if comp == 0 {
        comp = Expression::compareList(arguments(call1)?, arguments(call2)?)?;
    }
    if comp == 0 {
        comp = List::compare(iterators(call1), iterators(call2), &move |__a0: (
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        ),
                                                                        __a1: (
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )| {
            compareIterator(&__a0, &__a1)
        })?;
    }
    Ok(comp)
}

pub(crate) fn compareIterator(
    mut iter1: &(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    ),
    mut iter2: &(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    ),
) -> Result<i32> {
    let mut comp: i32;
    let mut n1: metamodelica::Ref<InstNode::InstNode>;
    let mut n2: metamodelica::Ref<InstNode::InstNode>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    (n1, e1) = iter1.clone();
    (n2, e2) = iter2.clone();
    comp = stringCompare(&(NFInstNode::InstNode::name(&n1)?), &(NFInstNode::InstNode::name(&n2)?));
    if comp == 0 {
        comp = Expression::compare(e1, e2)?;
    }
    Ok(comp)
}

pub(crate) fn isExternal(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut isExternal: bool;
    isExternal = (match &**call {
        UNTYPED_CALL { r#ref: __call_ref, .. } => Class::isExternalFunction(NFInstNode::InstNode::getClass(
            ComponentRef::node(metamodelica::AsArg::as_arg(&__call_ref))?,
        )?)?,
        ARG_TYPED_CALL { r#ref: __call_ref, .. } => Class::isExternalFunction(NFInstNode::InstNode::getClass(
            ComponentRef::node(metamodelica::AsArg::as_arg(&__call_ref))?,
        )?)?,
        TYPED_CALL { r#fn: __call_fn, .. } => Function::isExternal(metamodelica::AsArg::as_arg(&__call_fn))?,
        _ => false,
    });
    Ok(isExternal)
}

pub fn isImpure(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut isImpure: bool;
    isImpure = (match &**call {
        UNTYPED_CALL { r#ref: __call_ref, .. } => Function::isImpure(
            &((Function::getRefCache(metamodelica::AsArg::as_arg(&__call_ref))?)
                .head()
                .cloned()?),
        ),
        TYPED_CALL {
            purity: Prefixes::Purity::IMPURE,
            r#fn: __call_fn,
            ..
        } => Function::isImpure(metamodelica::AsArg::as_arg(&__call_fn)),
        _ => false,
    });
    Ok(isImpure)
}

pub(crate) fn isRecordConstructor(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut isConstructor: bool;
    isConstructor = (match &**call {
        UNTYPED_CALL { r#ref: __call_ref, .. } => SCodeUtil::isRecord(
            &(NFInstNode::InstNode::definition(ComponentRef::node(metamodelica::AsArg::as_arg(&__call_ref))?)?),
        ),
        TYPED_CALL { r#fn: __call_fn, .. }
            if (!(NFInstNode::InstNode::isEmpty(&(NFInstNode::InstNode::fromHandle(&__call_fn.node)?)))) =>
        {
            SCodeUtil::isRecord(
                &(NFInstNode::InstNode::definition(NFInstNode::InstNode::fromHandle(&__call_fn.node)?)?),
            )
        }
        _ => false,
    });
    Ok(isConstructor)
}

pub(crate) fn isExternalObjectConstructor(mut call: &metamodelica::Ref<NFCall>) -> bool {
    let mut isConstructor: bool;
    isConstructor = (match &**call {
        TYPED_CALL { ty: __call_ty, .. } => Type::isExternalObject(metamodelica::AsArg::as_arg(&__call_ty)),
        _ => false,
    });
    isConstructor
}

pub(crate) fn isLiteral(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    fn is_literal_iter(
        mut iter: (
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        ),
    ) -> Result<bool> {
        let mut literal: bool = Expression::isLiteral(&(Util::tuple22(iter.clone())))?;
        Ok(literal)
    }

    let mut literal: bool;
    literal = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => List::all(
            metamodelica::AsArg::as_arg(&__call_arguments),
            &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isLiteral(&__a0),
        )?,
        TYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::isLiteral(metamodelica::AsArg::as_arg(&__call_exp))?
                && List::all(metamodelica::AsArg::as_arg(&__call_iters), &is_literal_iter)?
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::isLiteral(metamodelica::AsArg::as_arg(&__call_exp))?
                && List::all(metamodelica::AsArg::as_arg(&__call_iters), &is_literal_iter)?
        }
        _ => false,
    });
    Ok(literal)
}

pub(crate) fn isKnownSizeFill(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    fn is_literal_iter(
        mut iter: (
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        ),
    ) -> Result<bool> {
        let mut literal: bool = Expression::isLiteral(&(Util::tuple22(iter.clone())))?;
        Ok(literal)
    }

    let mut res: bool;
    res = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            isNamed(call, &(literal!("fill")))?
                && List::all(&((__call_arguments).rest()?), &move |__a0: metamodelica::Ref<
                    Expression::NFExpression,
                >| {
                    Expression::isLiteral(&__a0)
                })?
        }
        TYPED_ARRAY_CONSTRUCTOR {
            iters: __call_iters, ..
        } => List::all(metamodelica::AsArg::as_arg(&__call_iters), &is_literal_iter)?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn isReduction(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut b: bool;
    b = (match &**call {
        TYPED_REDUCTION { .. } => true,
        TYPED_CALL { r#fn: __call_fn, .. } => {
            (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)), literal!("."), true, false)?) {
                Deref @ "min" => true,
                Deref @ "max" => true,
                Deref @ "sum" => true,
                Deref @ "product" => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(b)
}

pub(crate) fn isPositive(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut positive: bool;
    positive = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            (::match_deref::match_deref! { match &(functionNameFirst(call)?) {
                Deref @ "abs" => Expression::isNonZero(&((__call_arguments).head().cloned()?))?,
                Deref @ "max" => List::any(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isPositive(&__a0))?,
                Deref @ "min" => List::all(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isPositive(&__a0))?,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(positive)
}

pub(crate) fn isNegative(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut negative: bool;
    negative = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            (::match_deref::match_deref! { match &(functionNameFirst(call)?) {
                Deref @ "abs" => false,
                Deref @ "min" => List::any(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNegative(&__a0))?,
                Deref @ "max" => List::all(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNegative(&__a0))?,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(negative)
}

pub(crate) fn isNonPositive(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut nonPositive: bool;
    nonPositive = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            (::match_deref::match_deref! { match &(functionNameFirst(call)?) {
                Deref @ "abs" => Expression::isZero(&((__call_arguments).head().cloned()?))?,
                Deref @ "max" => List::all(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNonPositive(&__a0))?,
                Deref @ "min" => List::any(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNonPositive(&__a0))?,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(nonPositive)
}

pub(crate) fn isNonNegative(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut nonNegative: bool;
    nonNegative = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            (::match_deref::match_deref! { match &(functionNameFirst(call)?) {
                Deref @ "abs" => true,
                Deref @ "max" => List::any(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNonNegative(&__a0))?,
                Deref @ "min" => List::all(metamodelica::AsArg::as_arg(&__call_arguments), &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNonNegative(&__a0))?,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(nonNegative)
}

pub(crate) fn inlineType(mut call: &metamodelica::Ref<NFCall>) -> DAE::InlineType {
    let mut inlineTy: DAE::InlineType;
    inlineTy = (::match_deref::match_deref! { match call {
        Deref @ TYPED_CALL { attributes: Deref @ NFCallAttributes::CALL_ATTR { inlineType: __esc_inlineTy, .. }, .. } => {
            inlineTy = (*__esc_inlineTy).clone();
            inlineTy.clone()
        },
        _ => openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    inlineTy
}

pub fn typedFunction(mut call: &metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<Function::Function>> {
    let mut r#fn: metamodelica::Ref<Function::Function>;
    r#fn = (match &**call {
        TYPED_CALL { r#fn: __call_fn, .. } => __call_fn.clone(),
        TYPED_ARRAY_CONSTRUCTOR { .. } => NFBuiltinFuncs::ARRAY_FUNC().clone(),
        TYPED_REDUCTION { r#fn: __call_fn, .. } => __call_fn.clone(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.typedFunction"));
                    __mm_s.push_str(&*literal!(" got untyped function"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(r#fn)
}

pub fn functionName(mut call: &metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut name: metamodelica::Ref<Absyn::Path>;
    name = (match &**call {
        UNTYPED_CALL { r#ref: __call_ref, .. } => ComponentRef::toPath(metamodelica::AsArg::as_arg(&__call_ref))?,
        ARG_TYPED_CALL { r#ref: __call_ref, .. } => ComponentRef::toPath(metamodelica::AsArg::as_arg(&__call_ref))?,
        TYPED_CALL { r#fn: __call_fn, .. } => Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)),
        UNTYPED_ARRAY_CONSTRUCTOR { .. } => metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("array"),
        }),
        TYPED_ARRAY_CONSTRUCTOR { .. } => metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("array"),
        }),
        UNTYPED_REDUCTION { r#ref: __call_ref, .. } => ComponentRef::toPath(metamodelica::AsArg::as_arg(&__call_ref))?,
        TYPED_REDUCTION { r#fn: __call_fn, .. } => {
            Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn))
        }
    });
    Ok(name)
}

pub fn functionNameLast(mut call: &metamodelica::Ref<NFCall>) -> Result<ArcStr> {
    let mut ident: ArcStr = AbsynUtil::pathLastIdent(&(functionName(call)?));
    Ok(ident)
}

pub(crate) fn functionNameFirst(mut call: &metamodelica::Ref<NFCall>) -> Result<ArcStr> {
    let mut ident: ArcStr = AbsynUtil::pathFirstIdent(&(functionName(call)?));
    Ok(ident)
}

pub fn isNamed(mut call: &metamodelica::Ref<NFCall>, mut name: &ArcStr) -> Result<bool> {
    let mut res: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = functionName(call)?;
    res = (match &*path {
        Absyn::Path::IDENT { name: __path_name } => metamodelica::stringEq(&__path_name, &name),
        _ => false,
    });
    Ok(res)
}

pub fn arguments(
    mut call: &metamodelica::Ref<NFCall>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    arguments = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            ..
        } => __call_arguments.clone(),
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => __call_arguments.clone(),
        UNTYPED_ARRAY_CONSTRUCTOR { exp: __call_exp, .. } => list![__call_exp.clone()],
        TYPED_ARRAY_CONSTRUCTOR { exp: __call_exp, .. } => list![__call_exp.clone()],
        UNTYPED_REDUCTION { exp: __call_exp, .. } => list![__call_exp.clone()],
        TYPED_REDUCTION { exp: __call_exp, .. } => list![__call_exp.clone()],
        _ => return Err("match: no arm matched"),
    });
    Ok(arguments)
}

pub fn setArguments(
    mut call: metamodelica::Ref<NFCall>,
    mut arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall> = call;
    call = (match &*call {
        UNTYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::UNTYPED_CALL; arguments = arguments);
            call
        }
        TYPED_CALL { .. } => {
            assign_variant_field!(call => NFCall::TYPED_CALL; arguments = arguments);
            call
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(call)
}

pub fn iterators(
    mut call: &metamodelica::Ref<NFCall>,
) -> metamodelica::List<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    iters = (match &**call {
        UNTYPED_ARRAY_CONSTRUCTOR {
            iters: __call_iters, ..
        } => __call_iters.clone(),
        TYPED_ARRAY_CONSTRUCTOR {
            iters: __call_iters, ..
        } => __call_iters.clone(),
        UNTYPED_REDUCTION {
            iters: __call_iters, ..
        } => __call_iters.clone(),
        TYPED_REDUCTION {
            iters: __call_iters, ..
        } => __call_iters.clone(),
        _ => metamodelica::nil(),
    });
    iters
}

pub(crate) fn toRecordExpression(
    mut call: &metamodelica::Ref<NFCall>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            ..
        } => EvalFunction::evaluateRecordConstructor(
            metamodelica::AsArg::as_arg(&__call_fn),
            ty,
            __call_arguments.clone(),
            false,
        )?,
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.toRecordExpression"));
                    __mm_s.push_str(&*literal!(" got unknown call"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub fn toString(mut call: &metamodelica::Ref<NFCall>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut name: ArcStr;
    let mut arg_str: ArcStr;
    let mut c: ArcStr;
    let mut iters: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    r#str = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            r#ref: __call_ref,
            ..
        } => {
            name = ComponentRef::toString(metamodelica::AsArg::as_arg(&__call_ref))?;
            arg_str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                        let __x = Expression::toString(arg.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
            ..
        } => {
            name = ComponentRef::toString(metamodelica::AsArg::as_arg(&__call_ref))?;
            arg_str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut arg in (__call_positional_args.clone()).into_iter().cloned() {
                        let __x = Expression::toString(arg.value.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            for mut arg in &*__call_named_args.clone() {
                c = if (metamodelica::stringEq(&arg_str, &(literal!("")))) {
                    literal!("")
                } else {
                    literal!(", ")
                };
                arg_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*arg_str);
                    __mm_s.push_str(&*c);
                    __mm_s.push_str(&*arg.name.clone().ok_or("pattern mismatch")?);
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*Expression::toString(arg.value.clone())?);
                    ArcStr::from(__mm_s)
                };
            }
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            name = AbsynUtil::pathString(
                Function::nameConsiderBuiltin(&(NFBuiltinFuncs::ARRAY_FUNC().clone())),
                literal!("."),
                true,
                false,
            )?;
            arg_str = Expression::toString(__call_exp.clone())?;
            c = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*NFInstNode::InstNode::name(&(Util::tuple21(iter.clone())))?);
                            __mm_s.push_str(&*literal!(" in "));
                            __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(" for "));
                __mm_s.push_str(&*c);
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            }
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => {
            name = ComponentRef::toString(metamodelica::AsArg::as_arg(&__call_ref))?;
            arg_str = Expression::toString(__call_exp.clone())?;
            c = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*NFInstNode::InstNode::name(&(Util::tuple21(iter.clone())))?);
                            __mm_s.push_str(&*literal!(" in "));
                            __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(" for "));
                __mm_s.push_str(&*c);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            ..
        } => {
            name = AbsynUtil::pathString(
                Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)),
                literal!("."),
                true,
                false,
            )?;
            arg_str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                        let __x = Expression::toString(arg.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            name = AbsynUtil::pathString(
                Function::nameConsiderBuiltin(&(NFBuiltinFuncs::ARRAY_FUNC().clone())),
                literal!("."),
                true,
                false,
            )?;
            arg_str = Expression::toString(__call_exp.clone())?;
            c = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*NFInstNode::InstNode::name(&(Util::tuple21(iter.clone())))?);
                            __mm_s.push_str(&*literal!(" in "));
                            __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(" for "));
                __mm_s.push_str(&*c);
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            }
        }
        TYPED_REDUCTION {
            exp: __call_exp,
            r#fn: __call_fn,
            iters: __call_iters,
            ..
        } => {
            name = AbsynUtil::pathString(
                Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)),
                literal!("."),
                true,
                false,
            )?;
            arg_str = Expression::toString(__call_exp.clone())?;
            c = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*NFInstNode::InstNode::name(&(Util::tuple21(iter.clone())))?);
                            __mm_s.push_str(&*literal!(" in "));
                            __mm_s.push_str(&*Expression::toString(Util::tuple22(iter.clone()))?);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(" for "));
                __mm_s.push_str(&*c);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
    });
    Ok(r#str)
}

pub(crate) fn toFlatString(
    mut call: &metamodelica::Ref<NFCall>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut name: ArcStr;
    let mut arg_str: ArcStr;
    let mut c: ArcStr;
    let mut iters: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    r#str = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            ty: __call_ty,
            ..
        } => {
            name = AbsynUtil::pathString(
                Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)),
                literal!("."),
                true,
                false,
            )?;
            arg_str = toFlatStringArgs(__call_arguments.clone(), &name, format)?;
            if (Function::isBuiltin(metamodelica::AsArg::as_arg(&__call_fn))) {
                stringAppendList(list![name, literal!("("), arg_str, literal!(")")])
            } else if (isExternalObjectConstructor(call)) {
                stringAppendList(list![
                    Type::toFlatString(metamodelica::AsArg::as_arg(&__call_ty), format)?,
                    literal!("("),
                    arg_str,
                    literal!(")")
                ])
            } else {
                stringAppendList(list![
                    Util::makeQuotedIdentifier(name)?,
                    literal!("("),
                    arg_str,
                    literal!(")")
                ])
            }
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            if isVectorized(call)? {
                r#str = Expression::toFlatString(devectorizeCall(call)?, format)?;
            } else {
                name = AbsynUtil::pathString(
                    Function::nameConsiderBuiltin(&(NFBuiltinFuncs::ARRAY_FUNC().clone())),
                    literal!("."),
                    true,
                    false,
                )?;
                arg_str = Expression::toFlatString(__call_exp.clone(), format)?;
                c = stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut iter in (__call_iters.clone()).into_iter().cloned() {
                            let __x = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*Util::makeQuotedIdentifier(NFInstNode::InstNode::name(
                                    &(Util::tuple21(iter.clone())),
                                )?)?);
                                __mm_s.push_str(&*literal!(" in "));
                                __mm_s.push_str(&*Expression::toFlatString(Util::tuple22(iter.clone()), format)?);
                                ArcStr::from(__mm_s)
                            };
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(", "),
                );
                r#str = stringAppendList(list![literal!("{"), arg_str, literal!(" for "), c, literal!("}")]);
            }
            r#str
        }
        TYPED_REDUCTION {
            exp: __call_exp,
            r#fn: __call_fn,
            iters: __call_iters,
            ..
        } => {
            name = AbsynUtil::pathString(
                Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)),
                literal!("."),
                true,
                false,
            )?;
            arg_str = Expression::toFlatString(__call_exp.clone(), format)?;
            c = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*Util::makeQuotedIdentifier(NFInstNode::InstNode::name(
                                &(Util::tuple21(iter.clone())),
                            )?)?);
                            __mm_s.push_str(&*literal!(" in "));
                            __mm_s.push_str(&*Expression::toFlatString(Util::tuple22(iter.clone()), format)?);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            if (Function::isBuiltin(metamodelica::AsArg::as_arg(&__call_fn))) {
                stringAppendList(list![name, literal!("("), arg_str, literal!(" for "), c, literal!(")")])
            } else {
                stringAppendList(list![
                    Util::makeQuotedIdentifier(name)?,
                    literal!("("),
                    arg_str,
                    literal!(" for "),
                    c,
                    literal!(")")
                ])
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub(crate) fn toFlatStringArgs(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut fnName: &ArcStr,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut argsString: ArcStr;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut arg2: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    argsString = (::match_deref::match_deref! { match &(fnName.clone()) {
        Deref @ "String" => (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            arg1 = (*__esc_arg1).clone();
            arg2 = (*__esc_arg2).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(arg1.clone(), format)?); __mm_s.push_str(&*literal!(", format = ")); __mm_s.push_str(&*Expression::toFlatString(arg2.clone(), format)?); ArcStr::from(__mm_s) }
        },
        _ => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            arg1 = metamodelica::Own::own(__pa0);
            rest_args = metamodelica::Own::own(__pa1);
            argsString = Expression::toFlatString(arg1, format)?;
            if ((rest_args).len() as i32) == 3 {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_args) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                arg1 = metamodelica::Own::own(__pa2);
                rest_args = metamodelica::Own::own(__pa3);
                if !(Expression::isIntegerValue(&arg1, 6)) {
                    argsString = { let mut __mm_s = String::new(); __mm_s.push_str(&*argsString); __mm_s.push_str(&*literal!(", significantDigits = ")); __mm_s.push_str(&*Expression::toFlatString(arg1, format)?); ArcStr::from(__mm_s) };
                }
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_args) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            arg1 = metamodelica::Own::own(__pa4);
            rest_args = metamodelica::Own::own(__pa5);
            if !(Expression::isZero(&arg1)?) {
                argsString = { let mut __mm_s = String::new(); __mm_s.push_str(&*argsString); __mm_s.push_str(&*literal!(", minimumLength = ")); __mm_s.push_str(&*Expression::toFlatString(arg1, format)?); ArcStr::from(__mm_s) };
            }
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(rest_args) {
                Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            arg1 = metamodelica::Own::own(__pa6);
            rest_args = metamodelica::Own::own(__pa7);
            if !(Expression::isTrue(&arg1)) {
                argsString = { let mut __mm_s = String::new(); __mm_s.push_str(&*argsString); __mm_s.push_str(&*literal!(", leftJustified = ")); __mm_s.push_str(&*Expression::toFlatString(arg1, format)?); ArcStr::from(__mm_s) };
            }
            argsString
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => stringDelimitList(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut arg in (args).into_iter().cloned() {
            let __x = Expression::toFlatString(arg.clone(), format)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), literal!(", ")),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(argsString)
}

pub(crate) fn typedString(mut call: &metamodelica::Ref<NFCall>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut name: ArcStr;
    let mut arg_str: ArcStr;
    let mut c: ArcStr;
    r#str = (match &**call {
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
            ..
        } => {
            name = ComponentRef::toString(metamodelica::AsArg::as_arg(&__call_ref))?;
            arg_str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut arg in (__call_positional_args.clone()).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("/*"));
                            __mm_s.push_str(&*Type::toString(&(arg.ty.clone()))?);
                            __mm_s.push_str(&*literal!("*/ "));
                            __mm_s.push_str(&*Expression::toString(arg.value.clone())?);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            for mut arg in &*__call_named_args.clone() {
                c = if (metamodelica::stringEq(&arg_str, &(literal!("")))) {
                    literal!("")
                } else {
                    literal!(", ")
                };
                arg_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*arg_str);
                    __mm_s.push_str(&*c);
                    __mm_s.push_str(&*arg.name.clone().ok_or("pattern mismatch")?);
                    __mm_s.push_str(&*literal!(" = /*"));
                    __mm_s.push_str(&*Type::toString(&arg.ty)?);
                    __mm_s.push_str(&*literal!("*/ "));
                    __mm_s.push_str(&*Expression::toString(arg.value.clone())?);
                    ArcStr::from(__mm_s)
                };
            }
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            ..
        } => {
            name = AbsynUtil::pathString(
                Function::name(metamodelica::AsArg::as_arg(&__call_fn)),
                literal!("."),
                true,
                false,
            )?;
            arg_str = stringDelimitList(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                        let __x = Expression::toStringTyped(arg.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                literal!(", "),
            );
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*arg_str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        }
        _ => toString(call)?,
    });
    Ok(r#str)
}

pub(crate) fn toJSON(mut call: &metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<JSON::JSON>> {
    pub(crate) fn iterators_json(
        mut iters: &metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
    ) -> Result<metamodelica::Ref<JSON::JSON>> {
        let mut json: metamodelica::Ref<JSON::JSON> = JSON::emptyArray(((iters).len() as i32));
        let mut j: metamodelica::Ref<JSON::JSON>;
        for mut i in &**iters {
            j = JSON::emptyListObject();
            j = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(NFInstNode::InstNode::name(&(Util::tuple21(i.clone())))?)),
                j,
            )?;
            j = JSON::addPair(
                &(literal!("range")),
                &(Expression::toJSON(Util::tuple22(i.clone()))?),
                j,
            )?;
            json = JSON::addElement(&j, json)?;
        }
        Ok(json)
    }

    let mut json: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
    let mut path: metamodelica::Ref<Absyn::Path>;
    let () = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            ..
        } => {
            path = Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn));
            json = JSON::addPair(&(literal!("$kind")), &(JSON::makeString(literal!("call"))), json)?;
            json = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(AbsynUtil::pathString(path, literal!("."), true, false)?)),
                json,
            )?;
            if isNamed(call, &(literal!("String")))? {
                json = toJSONStringArgs(metamodelica::AsArg::as_arg(&__call_arguments), json)?;
            } else {
                json = JSON::addPair(
                    &(literal!("arguments")),
                    &(JSON::makeArray(
                        ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                            for mut a in (__call_arguments.clone()).into_iter().cloned() {
                                let __x = Expression::toJSON(a.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )),
                    json,
                )?;
            }
            ()
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            json = JSON::addPair(
                &(literal!("$kind")),
                &(JSON::makeString(literal!("iterator_call"))),
                json,
            )?;
            json = JSON::addPair(&(literal!("name")), &(JSON::makeString(literal!("$array"))), json)?;
            json = JSON::addPair(&(literal!("exp")), &(Expression::toJSON(__call_exp.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("iterators")),
                &(iterators_json(metamodelica::AsArg::as_arg(&__call_iters))?),
                json,
            )?;
            ()
        }
        TYPED_REDUCTION {
            exp: __call_exp,
            r#fn: __call_fn,
            iters: __call_iters,
            ..
        } => {
            path = Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn));
            json = JSON::addPair(
                &(literal!("$kind")),
                &(JSON::makeString(literal!("iterator_call"))),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(AbsynUtil::pathString(path, literal!("."), true, false)?)),
                json,
            )?;
            json = JSON::addPair(&(literal!("exp")), &(Expression::toJSON(__call_exp.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("iterators")),
                &(iterators_json(metamodelica::AsArg::as_arg(&__call_iters))?),
                json,
            )?;
            ()
        }
        _ => {
            json = JSON::addPair(&(literal!("$kind")), &(JSON::makeString(literal!("call"))), json)?;
            ()
        }
    });
    Ok(json)
}

pub(crate) fn toJSONStringArgs(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut json: metamodelica::Ref<JSON::JSON>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    fn make_arg(
        mut name: &ArcStr,
        mut value: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<JSON::JSON>> {
        let mut json: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
        json = JSON::addPair(&(literal!("$kind")), &(JSON::makeString(literal!("named_arg"))), json)?;
        json = JSON::addPair(name, &(Expression::toJSON(value)?), json)?;
        Ok(json)
    }

    let mut json: metamodelica::Ref<JSON::JSON> = json;
    let mut arg_count: i32;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut json_args: metamodelica::List<metamodelica::Ref<JSON::JSON>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    value = metamodelica::Own::own(__pa0);
    rest_args = metamodelica::Own::own(__pa1);
    arg_count = ((rest_args).len() as i32);
    json_args = list![Expression::toJSON(value)?];
    if arg_count == 1 {
        let __pa2 = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: _ } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa2);
        json_args = metamodelica::cons(make_arg(&(literal!("format")), arg)?, json_args);
    } else {
        if arg_count == 3 {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest_args) {
                Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            arg = metamodelica::Own::own(__pa3);
            rest_args = metamodelica::Own::own(__pa4);
            if !(Expression::isIntegerValue(&arg, 6)) {
                json_args = metamodelica::cons(make_arg(&(literal!("significantDigits")), arg)?, json_args);
            }
        }
        let (__pa5, __pa6) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa5);
        rest_args = metamodelica::Own::own(__pa6);
        if !(Expression::isZero(&arg)?) {
            json_args = metamodelica::cons(make_arg(&(literal!("minimumLength")), arg)?, json_args);
        }
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: __pa8 } => (__pa7.clone(), __pa8.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa7);
        rest_args = metamodelica::Own::own(__pa8);
        if !(Expression::isTrue(&arg)) {
            json_args = metamodelica::cons(make_arg(&(literal!("leftJustified")), arg)?, json_args);
        }
    }
    json = JSON::addPair(
        &(literal!("arguments")),
        &(JSON::makeList(metamodelica::Dangerous::listReverseInPlace(json_args))),
        json,
    )?;
    Ok(json)
}

pub(crate) fn toAbsyn(mut call: &metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut absynCall: metamodelica::Ref<Absyn::Exp>;
    absynCall = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            r#ref: __call_ref,
            ..
        } => {
            let mut pargs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
            pargs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = Expression::toAbsyn(arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            nargs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>> = metamodelica::nil();
                for mut arg in (__call_named_args.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Absyn::NamedArg {
                        argName: Util::tuple21(arg.clone()),
                        argValue: Expression::toAbsyn(Util::tuple22(arg.clone()))?,
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            AbsynUtil::makeCall(
                ComponentRef::toAbsyn(metamodelica::AsArg::as_arg(&__call_ref))?,
                pargs,
                nargs,
            )
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
            ..
        } => {
            let mut pargs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
            pargs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut arg in (__call_positional_args.clone()).into_iter().cloned() {
                    let __x = Expression::toAbsyn(arg.value.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            nargs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>> = metamodelica::nil();
                for mut arg in (__call_named_args.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Absyn::NamedArg {
                        argName: arg.name.clone().ok_or("pattern mismatch")?,
                        argValue: Expression::toAbsyn(arg.value.clone())?,
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            AbsynUtil::makeCall(
                ComponentRef::toAbsyn(metamodelica::AsArg::as_arg(&__call_ref))?,
                pargs,
                nargs,
            )
        }
        TYPED_CALL {
            arguments: __call_arguments,
            r#fn: __call_fn,
            ..
        } => {
            let mut pargs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            pargs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = Expression::toAbsyn(arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            AbsynUtil::makeCall(
                AbsynUtil::pathToCref(&(Function::name(metamodelica::AsArg::as_arg(&__call_fn)))),
                pargs,
                metamodelica::nil(),
            )
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: literal!("array"),
                subscripts: metamodelica::nil(),
            }),
            functionArgs: toAbsynIterators(__call_exp.clone(), __call_iters.clone())?,
            typeVars: metamodelica::nil(),
        }),
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: literal!("array"),
                subscripts: metamodelica::nil(),
            }),
            functionArgs: toAbsynIterators(__call_exp.clone(), __call_iters.clone())?,
            typeVars: metamodelica::nil(),
        }),
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: ComponentRef::toAbsyn(metamodelica::AsArg::as_arg(&__call_ref))?,
            functionArgs: toAbsynIterators(__call_exp.clone(), __call_iters.clone())?,
            typeVars: metamodelica::nil(),
        }),
        TYPED_REDUCTION {
            exp: __call_exp,
            r#fn: __call_fn,
            iters: __call_iters,
            ..
        } => metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: AbsynUtil::pathToCref(&(Function::name(metamodelica::AsArg::as_arg(&__call_fn)))),
            functionArgs: toAbsynIterators(__call_exp.clone(), __call_iters.clone())?,
            typeVars: metamodelica::nil(),
        }),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.toAbsyn"));
                    __mm_s.push_str(&*literal!(" got unknown call"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(absynCall)
}

pub(crate) fn toAbsynIterators(
    mut iterExp: metamodelica::Ref<Expression::NFExpression>,
    mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    let mut args: metamodelica::Ref<Absyn::FunctionArgs>;
    args = metamodelica::Ref::new(Absyn::FunctionArgs::FOR_ITER_FARG {
        exp: Expression::toAbsyn(iterExp)?,
        iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
        iterators: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
            for mut i in (iters).into_iter().cloned() {
                let __x = metamodelica::Ref::new(Absyn::ForIterator {
                    name: NFInstNode::InstNode::name(&(Util::tuple21(i.clone())))?,
                    guardExp: None,
                    range: Some(Expression::toAbsyn(Util::tuple22(i.clone()))?),
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    });
    Ok(args)
}

pub(crate) fn toDAE(mut call: metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut daeCall: metamodelica::Ref<DAE::Exp>;
    daeCall = toDAE_work(&(expandReduction(call)?))?;
    Ok(daeCall)
}

pub(crate) fn toDAE_work(mut call: &metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut daeCall: metamodelica::Ref<DAE::Exp>;
    daeCall = (match &**call {
        TYPED_CALL {
            arguments: __call_arguments,
            attributes: __call_attributes,
            r#fn: __call_fn,
            ty: __call_ty,
            ..
        } => metamodelica::Ref::new(DAE::Exp::CALL {
            path: Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__call_fn)),
            expLst: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut e in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = Expression::toDAE(e.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            attr: NFCallAttributes::toDAE(
                metamodelica::AsArg::as_arg(&__call_attributes),
                metamodelica::AsArg::as_arg(&__call_ty),
            )?,
        }),
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ty: __call_ty,
            ..
        } => {
            let mut fold_id: ArcStr;
            let mut res_id: ArcStr;
            fold_id = Util::getTempVariableIndex();
            res_id = Util::getTempVariableIndex();
            metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: Function::name(&(NFBuiltinFuncs::ARRAY_FUNC().clone())),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: Type::toDAE(metamodelica::AsArg::as_arg(&__call_ty), true)?,
                    defaultValue: None,
                    foldName: fold_id,
                    resultName: res_id,
                    foldExp: None,
                }),
                expr: Expression::toDAE(__call_exp.clone(), false)?,
                iterators: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = iteratorToDAE(&(iter.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            })
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            r#fn: __call_fn,
            foldExp: __call_foldExp,
            iters: __call_iters,
            ty: __call_ty,
            ..
        } => {
            let mut fold_id: ArcStr;
            let mut res_id: ArcStr;
            let mut fold_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
            (fold_exp, fold_id, res_id) = __call_foldExp.clone();
            metamodelica::Ref::new(DAE::Exp::REDUCTION {
                reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                    path: Function::name(metamodelica::AsArg::as_arg(&__call_fn)),
                    iterType: openmodelica_ast::Absyn::ReductionIterType::COMBINE,
                    exprType: Type::toDAE(metamodelica::AsArg::as_arg(&__call_ty), true)?,
                    defaultValue: Util::applyOption(__call_defaultExp.clone(), &move |__a0: metamodelica::Ref<
                        Expression::NFExpression,
                    >| {
                        Expression::toDAEValue(&__a0)
                    })?,
                    foldName: fold_id,
                    resultName: res_id,
                    foldExp: Util::applyOption(
                        fold_exp,
                        &({
                            let __pe_b1 = false;
                            move |__pe_a0| Expression::toDAE(__pe_a0, __pe_b1.clone())
                        }),
                    )?,
                }),
                expr: Expression::toDAE(__call_exp.clone(), false)?,
                iterators: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>> = metamodelica::nil();
                    for mut iter in (__call_iters.clone()).into_iter().cloned() {
                        let __x = iteratorToDAE(&(iter.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            })
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.toDAE_work"));
                    __mm_s.push_str(&*literal!(" got untyped call"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(daeCall)
}

pub(crate) fn expandReduction(mut call: metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<NFCall>> {
    let mut outCall: metamodelica::Ref<NFCall>;
    outCall = (match &*call {
        TYPED_ARRAY_CONSTRUCTOR {
            iters,
            exp: __call_exp,
            purity: __call_purity,
            var: __call_var,
            ..
        } if (((iters).len() as i32) > 1) => {
            let mut iter: (
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            );
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut iters = (*iters).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            iter = metamodelica::Own::own(__pa0);
            iters = metamodelica::Own::own(__pa1);
            ty = Type::liftArrayLeftList(
                Expression::typeOf(__call_exp.clone()),
                &(Type::arrayDims(Expression::typeOf(Util::tuple22(iter.clone())))),
            );
            outCall = metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: __call_exp.clone(),
                iters: list![iter],
            });
            for mut i in &*iters.clone() {
                ty = Type::liftArrayLeftList(ty, &(Type::arrayDims(Expression::typeOf(Util::tuple22(i.clone())))));
                outCall = metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                    ty: ty.clone(),
                    var: __call_var.clone(),
                    purity: __call_purity.clone(),
                    exp: metamodelica::Ref::new(Expression::NFExpression::CALL { call: outCall }),
                    iters: list![i.clone()],
                });
            }
            outCall
        }
        TYPED_REDUCTION {
            iters,
            exp: __call_exp,
            r#fn: __call_fn,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
            ..
        } if (((iters).len() as i32) > 1) => {
            let mut iter: (
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            );
            let mut iters = (*iters).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            iter = metamodelica::Own::own(__pa0);
            iters = metamodelica::Own::own(__pa1);
            outCall = makeTypedReduction(
                __call_fn.clone(),
                __call_ty.clone(),
                __call_var.clone(),
                __call_purity.clone(),
                __call_exp.clone(),
                list![iter],
                Absyn::dummyInfo.clone(),
            )?;
            for mut i in &*iters.clone() {
                outCall = makeTypedReduction(
                    __call_fn.clone(),
                    __call_ty.clone(),
                    __call_var.clone(),
                    __call_purity.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::CALL { call: outCall }),
                    list![i.clone()],
                    Absyn::dummyInfo.clone(),
                )?;
            }
            outCall
        }
        _ => call,
    });
    Ok(outCall)
}

pub(crate) fn isVectorizeable(mut call: &metamodelica::Ref<NFCall>) -> bool {
    let mut isVect: bool;
    isVect = (::match_deref::match_deref! { match call {
        Deref @ TYPED_CALL { r#fn: Deref @ Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name }, .. }, .. } => {
            (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "der" => false,
        Deref @ "pre" => false,
        Deref @ "previous" => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isVect
}

pub(crate) fn retype(mut call: metamodelica::Ref<NFCall>) -> metamodelica::Ref<NFCall> {
    let mut call: metamodelica::Ref<NFCall> = call;
    let () = (match &*call {
        TYPED_ARRAY_CONSTRUCTOR {
            iters: __call_iters,
            ty: __call_ty,
            ..
        } => {
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            dims = metamodelica::nil();
            for mut i in &*__call_iters.clone().reverse() {
                dims = listAppend(Type::arrayDims(Expression::typeOf(Util::tuple22(i.clone()))), dims);
            }
            assign_variant_field!(call => NFCall::TYPED_ARRAY_CONSTRUCTOR; ty = Type::liftArrayLeftList(Type::arrayElementType(metamodelica::AsArg::as_arg(&__call_ty)), &dims));
            ()
        }
        _ => (),
    });
    call
}

pub(crate) fn typeCast(
    mut callExp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression> = callExp;
    let mut call: metamodelica::Ref<NFCall>;
    let mut cast_ty: metamodelica::Ref<Type::NFType>;
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    callExp = (match &*call {
        TYPED_CALL {
            r#fn: __call_fn,
            ty: __call_ty,
            ..
        } if (Function::isBuiltin(metamodelica::AsArg::as_arg(&__call_fn))) => {
            cast_ty = Type::setArrayElementType(metamodelica::AsArg::as_arg(&__call_ty), &ty);
            (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(Function::name(var_field!((*call).r#fn, NFCall::TYPED_CALL))))) {
                Deref @ "fill" => {
                    assign_variant_field!(call => NFCall::TYPED_CALL;
                        arguments = metamodelica::cons(Expression::typeCast((var_field!((*call).arguments, NFCall::TYPED_CALL)).head().cloned()?, ty)?, (var_field!((*call).arguments, NFCall::TYPED_CALL)).rest()?),
                        ty = cast_ty
                    );
                    metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
                },
                Deref @ "diagonal" => {
                    assign_variant_field!(call => NFCall::TYPED_CALL;
                        arguments = list![Expression::typeCast((var_field!((*call).arguments, NFCall::TYPED_CALL)).head().cloned()?, ty)?],
                        ty = cast_ty
                    );
                    metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
                },
                Deref @ "DynamicSelect" => {
                    assign_variant_field!(call => NFCall::TYPED_CALL; arguments = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (var_field!((*call).arguments, NFCall::TYPED_CALL).clone()).into_iter().cloned() {
                    let __x = Expression::typeCast(arg.clone(), ty.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
                    metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
                },
                _ => metamodelica::Ref::new(Expression::NFExpression::CAST { ty: cast_ty, exp: callExp }),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => Expression::typeCastGeneric(callExp, &ty)?,
    });
    Ok(callExp)
}

pub(crate) fn containsExp(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            ..
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            res = Expression::listContains(metamodelica::AsArg::as_arg(&__call_arguments), func)?;
            if !(res) {
                for mut arg in &*__call_named_args.clone() {
                    (_, e) = arg.clone();
                    if Expression::contains(e, func)? {
                        res = true;
                        break;
                    }
                }
            }
            res
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            ..
        } => {
            for mut arg in &*__call_positional_args.clone() {
                if Expression::contains(arg.value.clone(), func)? {
                    res = true;
                    return Ok(res);
                }
            }
            for mut arg in &*__call_named_args.clone() {
                if Expression::contains(arg.value.clone(), func)? {
                    res = true;
                    return Ok(res);
                }
            }
            false
        }
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => Expression::listContains(metamodelica::AsArg::as_arg(&__call_arguments), func)?,
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            Expression::contains(__call_exp.clone(), func)?
                || itersContainExp(metamodelica::AsArg::as_arg(&__call_iters), func)?
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::contains(__call_exp.clone(), func)?
                || itersContainExp(metamodelica::AsArg::as_arg(&__call_iters), func)?
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::contains(__call_exp.clone(), func)?
                || itersContainExp(metamodelica::AsArg::as_arg(&__call_iters), func)?
        }
        TYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::contains(__call_exp.clone(), func)?
                || itersContainExp(metamodelica::AsArg::as_arg(&__call_iters), func)?
        }
    });
    Ok(res)
}

pub(crate) fn itersContainExp(
    mut iters: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    for mut iter in &**iters {
        if Expression::contains(Util::tuple22(iter.clone()), func)? {
            res = true;
            return Ok(res);
        }
    }
    Ok(res)
}

pub(crate) fn containsExpShallow(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            ..
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            res = List::any(metamodelica::AsArg::as_arg(&__call_arguments), func)?;
            if !(res) {
                for mut arg in &*__call_named_args.clone() {
                    (_, e) = arg.clone();
                    if func(e)? {
                        res = true;
                        break;
                    }
                }
            }
            res
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            ..
        } => {
            for mut arg in &*__call_positional_args.clone() {
                if func(arg.value.clone())? {
                    res = true;
                    return Ok(res);
                }
            }
            for mut arg in &*__call_named_args.clone() {
                if func(arg.value.clone())? {
                    res = true;
                    return Ok(res);
                }
            }
            false
        }
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => List::any(metamodelica::AsArg::as_arg(&__call_arguments), func)?,
        UNTYPED_ARRAY_CONSTRUCTOR { exp: __call_exp, .. } => func(__call_exp.clone())?,
        TYPED_ARRAY_CONSTRUCTOR { exp: __call_exp, .. } => func(__call_exp.clone())?,
        UNTYPED_REDUCTION { exp: __call_exp, .. } => func(__call_exp.clone())?,
        TYPED_REDUCTION { exp: __call_exp, .. } => func(__call_exp.clone())?,
    });
    Ok(res)
}

pub(crate) fn applyExp(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            ..
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            Expression::applyList(metamodelica::AsArg::as_arg(&__call_arguments), func)?;
            for mut arg in &*__call_named_args.clone() {
                (_, e) = arg.clone();
                Expression::apply(e, func)?;
            }
            ()
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            ..
        } => {
            for mut arg in &*__call_positional_args.clone() {
                Expression::apply(arg.value.clone(), func)?;
            }
            for mut arg in &*__call_named_args.clone() {
                Expression::apply(arg.value.clone(), func)?;
            }
            ()
        }
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            Expression::applyList(metamodelica::AsArg::as_arg(&__call_arguments), func)?;
            ()
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            Expression::apply(__call_exp.clone(), func)?;
            for mut i in &*__call_iters.clone() {
                Expression::apply(Util::tuple22(i.clone()), func)?;
            }
            ()
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::apply(__call_exp.clone(), func)?;
            for mut i in &*__call_iters.clone() {
                Expression::apply(Util::tuple22(i.clone()), func)?;
            }
            ()
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            Expression::apply(__call_exp.clone(), func)?;
            for mut i in &*__call_iters.clone() {
                Expression::apply(Util::tuple22(i.clone()), func)?;
            }
            ()
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            foldExp: __call_foldExp,
            iters: __call_iters,
            ..
        } => {
            Expression::apply(__call_exp.clone(), func)?;
            for mut i in &*__call_iters.clone() {
                Expression::apply(Util::tuple22(i.clone()), func)?;
            }
            Expression::applyOpt(__call_defaultExp.clone(), func)?;
            Expression::applyOpt(Util::tuple31(__call_foldExp.clone()), func)?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            ..
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            Expression::applyListShallow(metamodelica::AsArg::as_arg(&__call_arguments), func)?;
            for mut arg in &*__call_named_args.clone() {
                (_, e) = arg.clone();
                func(e)?;
            }
            ()
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            ..
        } => {
            for mut arg in &*__call_positional_args.clone() {
                func(arg.value.clone())?;
            }
            for mut arg in &*__call_named_args.clone() {
                func(arg.value.clone())?;
            }
            ()
        }
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            Expression::applyListShallow(metamodelica::AsArg::as_arg(&__call_arguments), func)?;
            ()
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            func(__call_exp.clone())?;
            for mut i in &*__call_iters.clone() {
                func(Util::tuple22(i.clone()))?;
            }
            ()
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            func(__call_exp.clone())?;
            for mut i in &*__call_iters.clone() {
                func(Util::tuple22(i.clone()))?;
            }
            ()
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            func(__call_exp.clone())?;
            for mut i in &*__call_iters.clone() {
                func(Util::tuple22(i.clone()))?;
            }
            ()
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            foldExp: __call_foldExp,
            iters: __call_iters,
            ..
        } => {
            func(__call_exp.clone())?;
            for mut i in &*__call_iters.clone() {
                func(Util::tuple22(i.clone()))?;
            }
            Expression::applyShallowOpt(__call_defaultExp.clone(), func)?;
            Expression::applyShallowOpt(Util::tuple31(__call_foldExp.clone()), func)?;
            ()
        }
    });
    Ok(())
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut foldArg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut foldArg: ArgT = foldArg;
    let () = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            named_args: __call_named_args,
            ..
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            foldArg = Expression::foldList(metamodelica::AsArg::as_arg(&__call_arguments), func.clone(), foldArg)?;
            for mut arg in &*__call_named_args.clone() {
                (_, e) = arg.clone();
                foldArg = Expression::fold(e, func.clone(), foldArg)?;
            }
            ()
        }
        ARG_TYPED_CALL {
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            ..
        } => {
            for mut arg in &*__call_positional_args.clone() {
                foldArg = Expression::fold(arg.value.clone(), func.clone(), foldArg)?;
            }
            for mut arg in &*__call_named_args.clone() {
                foldArg = Expression::fold(arg.value.clone(), func.clone(), foldArg)?;
            }
            ()
        }
        TYPED_CALL {
            arguments: __call_arguments,
            ..
        } => {
            foldArg = Expression::foldList(metamodelica::AsArg::as_arg(&__call_arguments), func.clone(), foldArg)?;
            ()
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            foldArg = Expression::fold(__call_exp.clone(), func.clone(), foldArg)?;
            for mut i in &*__call_iters.clone() {
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg)?;
            }
            ()
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            foldArg = Expression::fold(__call_exp.clone(), func.clone(), foldArg)?;
            for mut i in &*__call_iters.clone() {
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg)?;
            }
            ()
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            ..
        } => {
            foldArg = Expression::fold(__call_exp.clone(), func.clone(), foldArg)?;
            for mut i in &*__call_iters.clone() {
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg)?;
            }
            ()
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            foldExp: __call_foldExp,
            iters: __call_iters,
            ..
        } => {
            foldArg = Expression::fold(__call_exp.clone(), func.clone(), foldArg)?;
            for mut i in &*__call_iters.clone() {
                foldArg = Expression::fold(Util::tuple22(i.clone()), func.clone(), foldArg)?;
            }
            foldArg = Expression::foldOpt(__call_defaultExp.clone(), &*func, foldArg)?;
            foldArg = Expression::foldOpt(Util::tuple31(__call_foldExp.clone()), &*func, foldArg)?;
            ()
        }
    });
    Ok(foldArg)
}

pub(crate) fn mapExp(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFCall>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outCall: metamodelica::Ref<NFCall>;
    outCall = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            r#ref: __call_ref,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut nargs: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
            let mut s: ArcStr;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = Expression::map(arg.clone(), func.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            nargs = metamodelica::nil();
            for mut arg in &*__call_named_args.clone() {
                (s, e) = arg.clone();
                e = Expression::map(e, func.clone())?;
                nargs = metamodelica::cons((s, e), nargs);
            }
            metamodelica::Ref::new(NFCall::UNTYPED_CALL {
                r#ref: __call_ref.clone(),
                arguments: args,
                named_args: nargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        ARG_TYPED_CALL {
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
        } => {
            let mut targs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut tnargs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*__call_positional_args.clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = Expression::map(arg.value.clone(), func.clone())?);
                targs = metamodelica::cons(arg, targs);
            }
            for mut arg in &*__call_named_args.clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = Expression::map(arg.value.clone(), func.clone())?);
                tnargs = metamodelica::cons(arg, tnargs);
            }
            metamodelica::Ref::new(NFCall::ARG_TYPED_CALL {
                r#ref: __call_ref.clone(),
                positional_args: targs.reverse(),
                named_args: tnargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        TYPED_CALL {
            arguments: __call_arguments,
            attributes: __call_attributes,
            r#fn: __call_fn,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = Expression::map(arg.clone(), func.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(NFCall::TYPED_CALL {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                arguments: args,
                attributes: __call_attributes.clone(),
            })
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            e = Expression::map(__call_exp.clone(), func.clone())?;
            iters = mapIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone())?;
            metamodelica::Ref::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e, iters: iters })
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            e = Expression::map(__call_exp.clone(), func.clone())?;
            iters = mapIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone())?;
            metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
            })
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            e = Expression::map(__call_exp.clone(), func.clone())?;
            iters = mapIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone())?;
            metamodelica::Ref::new(NFCall::UNTYPED_REDUCTION {
                r#ref: __call_ref.clone(),
                exp: e,
                iters: iters,
            })
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            r#fn: __call_fn,
            foldExp: __call_foldExp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            let mut default_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut fold_exp: (Option<metamodelica::Ref<Expression::NFExpression>>, ArcStr, ArcStr);
            e = Expression::map(__call_exp.clone(), func.clone())?;
            iters = mapIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone())?;
            default_exp = Util::applyOption(
                __call_defaultExp.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
                }),
            )?;
            fold_exp = Util::applyTuple31(
                __call_foldExp.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a0| Expression::mapOpt(__pe_a0, __pe_b1.clone())
                }),
            )?;
            metamodelica::Ref::new(NFCall::TYPED_REDUCTION {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
                defaultExp: default_exp,
                foldExp: fold_exp,
            })
        }
    });
    Ok(outCall)
}

pub(crate) fn mapIteratorsExp(
    mut iters: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outIters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    for mut i in &**iters {
        (node, exp) = i.clone();
        new_exp = Expression::map(exp.clone(), func.clone())?;
        outIters = metamodelica::cons(
            if (referenceEq(&*(&*new_exp), &*(exp))) {
                i.clone()
            } else {
                (node, new_exp)
            },
            outIters,
        );
    }
    outIters = metamodelica::Dangerous::listReverseInPlace(outIters);
    Ok(outIters)
}

pub(crate) fn mapExpShallow(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFCall>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outCall: metamodelica::Ref<NFCall>;
    outCall = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            r#ref: __call_ref,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut nargs: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
            let mut s: ArcStr;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = func(arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            nargs = metamodelica::nil();
            for mut arg in &*__call_named_args.clone() {
                (s, e) = arg.clone();
                e = func(e)?;
                nargs = metamodelica::cons((s, e), nargs);
            }
            metamodelica::Ref::new(NFCall::UNTYPED_CALL {
                r#ref: __call_ref.clone(),
                arguments: args,
                named_args: nargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        ARG_TYPED_CALL {
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
        } => {
            let mut targs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut tnargs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*__call_positional_args.clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = func(arg.value.clone())?);
                targs = metamodelica::cons(arg, targs);
            }
            for mut arg in &*__call_named_args.clone() {
                let mut arg = arg.clone();
                assign_field!(arg.value = func(arg.value.clone())?);
                tnargs = metamodelica::cons(arg, tnargs);
            }
            metamodelica::Ref::new(NFCall::ARG_TYPED_CALL {
                r#ref: __call_ref.clone(),
                positional_args: targs.reverse(),
                named_args: tnargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        TYPED_CALL {
            arguments: __call_arguments,
            attributes: __call_attributes,
            r#fn: __call_fn,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (__call_arguments.clone()).into_iter().cloned() {
                    let __x = func(arg.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            metamodelica::Ref::new(NFCall::TYPED_CALL {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                arguments: args,
                attributes: __call_attributes.clone(),
            })
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            e = func(__call_exp.clone())?;
            iters = mapIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), &*func)?;
            metamodelica::Ref::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e, iters: iters })
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            e = func(__call_exp.clone())?;
            iters = mapIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), &*func)?;
            metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
            })
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            e = func(__call_exp.clone())?;
            iters = mapIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), &*func)?;
            metamodelica::Ref::new(NFCall::UNTYPED_REDUCTION {
                r#ref: __call_ref.clone(),
                exp: e,
                iters: iters,
            })
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            r#fn: __call_fn,
            foldExp: __call_foldExp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            let mut default_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut fold_exp: (Option<metamodelica::Ref<Expression::NFExpression>>, ArcStr, ArcStr);
            e = func(__call_exp.clone())?;
            iters = mapIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), &*func)?;
            default_exp = Expression::mapShallowOpt(__call_defaultExp.clone(), &*func)?;
            fold_exp = Util::applyTuple31(
                __call_foldExp.clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a0| Expression::mapShallowOpt(__pe_a0, &*__pe_b1)
                }),
            )?;
            metamodelica::Ref::new(NFCall::TYPED_REDUCTION {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
                defaultExp: default_exp,
                foldExp: fold_exp,
            })
        }
    });
    Ok(outCall)
}

pub(crate) fn mapIteratorsExpShallow(
    mut iters: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outIters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    for mut i in &**iters {
        (node, exp) = i.clone();
        new_exp = func(exp.clone())?;
        outIters = metamodelica::cons(
            if (referenceEq(&*(&*new_exp), &*(exp))) {
                i.clone()
            } else {
                (node, new_exp)
            },
            outIters,
        );
    }
    outIters = metamodelica::Dangerous::listReverseInPlace(outIters);
    Ok(outIters)
}

pub(crate) fn mapFoldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut foldArg: ArgT,
) -> Result<(metamodelica::Ref<NFCall>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outCall: metamodelica::Ref<NFCall>;
    let mut foldArg: ArgT = foldArg;
    outCall = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            r#ref: __call_ref,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut nargs: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
            let mut s: ArcStr;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            (args, foldArg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&__call_arguments),
                &Expression::mapFold,
                func.clone(),
                foldArg,
            )?;
            nargs = metamodelica::nil();
            for mut arg in &*__call_named_args.clone() {
                (s, e) = arg.clone();
                (e, foldArg) = Expression::mapFold(e, func.clone(), foldArg)?;
                nargs = metamodelica::cons((s, e), nargs);
            }
            metamodelica::Ref::new(NFCall::UNTYPED_CALL {
                r#ref: __call_ref.clone(),
                arguments: args,
                named_args: nargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        ARG_TYPED_CALL {
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
        } => {
            let mut targs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut tnargs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*__call_positional_args.clone() {
                let mut arg = arg.clone();
                (e, foldArg) = Expression::mapFold(arg.value.clone(), func.clone(), foldArg)?;
                assign_field!(arg.value = e);
                targs = metamodelica::cons(arg, targs);
            }
            for mut arg in &*__call_named_args.clone() {
                let mut arg = arg.clone();
                (e, foldArg) = Expression::mapFold(arg.value.clone(), func.clone(), foldArg)?;
                assign_field!(arg.value = e);
                targs = metamodelica::cons(arg, targs);
            }
            metamodelica::Ref::new(NFCall::ARG_TYPED_CALL {
                r#ref: __call_ref.clone(),
                positional_args: targs.reverse(),
                named_args: tnargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        TYPED_CALL {
            arguments: __call_arguments,
            attributes: __call_attributes,
            r#fn: __call_fn,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            (args, foldArg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&__call_arguments),
                &Expression::mapFold,
                func.clone(),
                foldArg,
            )?;
            metamodelica::Ref::new(NFCall::TYPED_CALL {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                arguments: args,
                attributes: __call_attributes.clone(),
            })
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            (e, foldArg) = Expression::mapFold(__call_exp.clone(), func.clone(), foldArg)?;
            (iters, foldArg) = mapFoldIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone(), foldArg)?;
            metamodelica::Ref::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e, iters: iters })
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            (e, foldArg) = Expression::mapFold(__call_exp.clone(), func.clone(), foldArg)?;
            (iters, foldArg) = mapFoldIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone(), foldArg)?;
            metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
            })
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            (e, foldArg) = Expression::mapFold(__call_exp.clone(), func.clone(), foldArg)?;
            (iters, foldArg) = mapFoldIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone(), foldArg)?;
            metamodelica::Ref::new(NFCall::UNTYPED_REDUCTION {
                r#ref: __call_ref.clone(),
                exp: e,
                iters: iters,
            })
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            r#fn: __call_fn,
            foldExp: __call_foldExp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            let mut default_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut fold_exp: (Option<metamodelica::Ref<Expression::NFExpression>>, ArcStr, ArcStr);
            let mut oe: Option<metamodelica::Ref<Expression::NFExpression>>;
            (e, foldArg) = Expression::mapFold(__call_exp.clone(), func.clone(), foldArg)?;
            (iters, foldArg) = mapFoldIteratorsExp(metamodelica::AsArg::as_arg(&__call_iters), func.clone(), foldArg)?;
            (default_exp, foldArg) = Expression::mapFoldOpt(__call_defaultExp.clone(), func.clone(), foldArg)?;
            oe = Util::tuple31(__call_foldExp.clone());
            if (oe).is_some() {
                (oe, foldArg) = Expression::mapFoldOpt(oe, func.clone(), foldArg)?;
                fold_exp = Util::applyTuple31(
                    __call_foldExp.clone(),
                    &({
                        let __pe_b1 = oe;
                        move |__pe_a0| Ok(Util::replace(__pe_a0, __pe_b1.clone()))
                    }),
                )?;
            } else {
                fold_exp = __call_foldExp.clone();
            }
            metamodelica::Ref::new(NFCall::TYPED_REDUCTION {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
                defaultExp: default_exp,
                foldExp: fold_exp,
            })
        }
    });
    Ok((outCall, foldArg))
}

pub(crate) fn mapFoldIteratorsExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iters: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    ArgT,
)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outIters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut arg: ArgT = arg;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    for mut i in &**iters {
        (node, exp) = i.clone();
        (new_exp, arg) = Expression::mapFold(exp.clone(), func.clone(), arg)?;
        outIters = metamodelica::cons(
            if (referenceEq(&*(&*new_exp), &*(exp))) {
                i.clone()
            } else {
                (node, new_exp)
            },
            outIters,
        );
    }
    outIters = metamodelica::Dangerous::listReverseInPlace(outIters);
    Ok((outIters, arg))
}

pub(crate) fn mapFoldExpShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut call: &metamodelica::Ref<NFCall>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
        ArgT,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>,
    mut foldArg: ArgT,
) -> Result<(metamodelica::Ref<NFCall>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outCall: metamodelica::Ref<NFCall>;
    let mut foldArg: ArgT = foldArg;
    outCall = (match &**call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            r#ref: __call_ref,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut nargs: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
            let mut s: ArcStr;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            (args, foldArg) = List::mapFold(metamodelica::AsArg::as_arg(&__call_arguments), func, foldArg)?;
            nargs = metamodelica::nil();
            for mut arg in &*__call_named_args.clone() {
                (s, e) = arg.clone();
                (e, foldArg) = func(e, foldArg)?;
                nargs = metamodelica::cons((s, e), nargs);
            }
            metamodelica::Ref::new(NFCall::UNTYPED_CALL {
                r#ref: __call_ref.clone(),
                arguments: args,
                named_args: nargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        ARG_TYPED_CALL {
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            positional_args: __call_positional_args,
            r#ref: __call_ref,
        } => {
            let mut targs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut tnargs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            targs = metamodelica::nil();
            tnargs = metamodelica::nil();
            for mut arg in &*__call_positional_args.clone() {
                let mut arg = arg.clone();
                (e, foldArg) = func(arg.value.clone(), foldArg)?;
                assign_field!(arg.value = e);
                targs = metamodelica::cons(arg, targs);
            }
            for mut arg in &*__call_named_args.clone() {
                let mut arg = arg.clone();
                (e, foldArg) = func(arg.value.clone(), foldArg)?;
                assign_field!(arg.value = e);
                targs = metamodelica::cons(arg, targs);
            }
            metamodelica::Ref::new(NFCall::ARG_TYPED_CALL {
                r#ref: __call_ref.clone(),
                positional_args: targs.reverse(),
                named_args: tnargs.reverse(),
                call_scope: __call_call_scope.clone(),
            })
        }
        TYPED_CALL {
            arguments: __call_arguments,
            attributes: __call_attributes,
            r#fn: __call_fn,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            (args, foldArg) = List::mapFold(metamodelica::AsArg::as_arg(&__call_arguments), func, foldArg)?;
            metamodelica::Ref::new(NFCall::TYPED_CALL {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                arguments: args,
                attributes: __call_attributes.clone(),
            })
        }
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            (e, foldArg) = func(__call_exp.clone(), foldArg)?;
            (iters, _) = mapFoldIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), func, foldArg.clone())?;
            metamodelica::Ref::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: e, iters: iters })
        }
        TYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            (e, foldArg) = func(__call_exp.clone(), foldArg)?;
            (iters, _) = mapFoldIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), func, foldArg.clone())?;
            metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
            })
        }
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            (e, foldArg) = func(__call_exp.clone(), foldArg)?;
            (iters, _) = mapFoldIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), func, foldArg.clone())?;
            metamodelica::Ref::new(NFCall::UNTYPED_REDUCTION {
                r#ref: __call_ref.clone(),
                exp: e,
                iters: iters,
            })
        }
        TYPED_REDUCTION {
            defaultExp: __call_defaultExp,
            exp: __call_exp,
            r#fn: __call_fn,
            foldExp: __call_foldExp,
            iters: __call_iters,
            purity: __call_purity,
            ty: __call_ty,
            var: __call_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut iters: metamodelica::List<(
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::Ref<Expression::NFExpression>,
            )>;
            let mut default_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
            let mut fold_exp: (Option<metamodelica::Ref<Expression::NFExpression>>, ArcStr, ArcStr);
            let mut oe: Option<metamodelica::Ref<Expression::NFExpression>>;
            (e, foldArg) = func(__call_exp.clone(), foldArg)?;
            (iters, _) = mapFoldIteratorsExpShallow(metamodelica::AsArg::as_arg(&__call_iters), func, foldArg.clone())?;
            (default_exp, foldArg) = Expression::mapFoldOptShallow(__call_defaultExp.clone(), func, foldArg)?;
            oe = Util::tuple31(__call_foldExp.clone());
            if (oe).is_some() {
                (oe, foldArg) = Expression::mapFoldOptShallow(oe, func, foldArg)?;
                fold_exp = Util::applyTuple31(
                    __call_foldExp.clone(),
                    &({
                        let __pe_b1 = oe;
                        move |__pe_a0| Ok(Util::replace(__pe_a0, __pe_b1.clone()))
                    }),
                )?;
            } else {
                fold_exp = __call_foldExp.clone();
            }
            metamodelica::Ref::new(NFCall::TYPED_REDUCTION {
                r#fn: __call_fn.clone(),
                ty: __call_ty.clone(),
                var: __call_var.clone(),
                purity: __call_purity.clone(),
                exp: e,
                iters: iters,
                defaultExp: default_exp,
                foldExp: fold_exp,
            })
        }
    });
    Ok((outCall, foldArg))
}

pub(crate) fn mapFoldIteratorsExpShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iters: &metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
        ArgT,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>,
    mut arg: ArgT,
) -> Result<(
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    ArgT,
)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outIters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut arg: ArgT = arg;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    for mut i in &**iters {
        (node, exp) = i.clone();
        (new_exp, arg) = func(exp.clone(), arg)?;
        outIters = metamodelica::cons(
            if (referenceEq(&*(&*new_exp), &*(exp))) {
                i.clone()
            } else {
                (node, new_exp)
            },
            outIters,
        );
    }
    outIters = metamodelica::Dangerous::listReverseInPlace(outIters);
    Ok((outIters, arg))
}

pub(crate) fn updateExternalRecordArgs(
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<()> {
    for mut arg in &**args {
        updateExternalRecordArgsInType(&(Expression::typeOf(arg.clone())))?;
    }
    Ok(())
}

pub(crate) fn updateExternalRecordArgsInType(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<()> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut res: metamodelica::Ref<Restriction::NFRestriction>;
    if Type::isRecord(ty) {
        node = Type::complexNode(ty)?;
        cls = NFInstNode::InstNode::getClass(node.clone())?;
        res = Restriction::setExternalRecord(Class::restriction(&cls));
        cls = Class::setRestriction(res, cls)?;
        NFInstNode::InstNode::updateClass(cls, node)?;
    }
    Ok(())
}

pub(crate) fn toArrayConstructor(
    mut iCall: metamodelica::Ref<NFCall>,
    mut index_ptr: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut oCall: metamodelica::Ref<NFCall>;
    oCall = ({
        let mut iterators: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )> = metamodelica::nil();
        (match &*iCall.clone() {
            TYPED_CALL {
                arguments: __iCall_arguments,
                r#fn: __iCall_fn,
                purity: __iCall_purity,
                ty: __iCall_ty,
                var: __iCall_var,
                ..
            } => {
                let mut iter_name: metamodelica::Ref<InstNode::InstNode>;
                let mut start: metamodelica::Ref<Expression::NFExpression>;
                let mut body: metamodelica::Ref<Expression::NFExpression>;
                let mut iter_range: metamodelica::Ref<Expression::NFExpression>;
                let mut step: Option<metamodelica::Ref<Expression::NFExpression>>;
                let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut body_call: metamodelica::Ref<NFCall>;
                let mut index: i32;
                (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(metamodelica::AsArg::as_arg(&__iCall_fn)), literal!("."), true, false)?) {
                    Deref @ "fill" => {
                        index = Pointer::access(index_ptr.clone());
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(__iCall_arguments.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        body = metamodelica::Own::own(__pa0);
                        rest = metamodelica::Own::own(__pa1);
                        start = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 });
                        step = None;
                        for mut stop in &*rest.reverse() {
                            iter_name = NFInstNode::InstNode::newIndexedIterator(index, &(literal!("f")), Absyn::dummyInfo.clone(), crate::NFType::interned_INTEGER());
                            iter_range = Expression::makeRange(start.clone(), step.clone(), stop.clone())?;
                            iterators = metamodelica::cons((iter_name, iter_range), iterators);
                            index = index + 1;
                        }
                        (body, iterators) = (::match_deref::match_deref! { match &(body.clone()) {
                    Deref @ Expression::CALL { call: __esc_body_call @ Deref @ TYPED_ARRAY_CONSTRUCTOR { .. } } => {
                        body_call = (*__esc_body_call).clone();
                        (var_field!((*body_call).exp, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), listAppend(iterators, var_field!((*body_call).iters, NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()))
                    },
                    _ => (body, iterators),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                        Pointer::update(index_ptr, index);
                        metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: __iCall_ty.clone(), var: __iCall_var.clone(), purity: __iCall_purity.clone(), exp: body, iters: iterators.reverse() })
                    },
                    _ => iCall,
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } })
            }
            _ => iCall,
        })
    });
    Ok(oCall)
}

pub(crate) fn isConnectionsOperator(mut call: &metamodelica::Ref<NFCall>) -> bool {
    let mut isOp: bool;
    isOp = (match &**call {
        TYPED_CALL { r#fn: __call_fn, .. } => {
            Function::isBuiltin(metamodelica::AsArg::as_arg(&__call_fn))
                && metamodelica::stringEq(
                    &(AbsynUtil::pathFirstIdent(&(Function::name(metamodelica::AsArg::as_arg(&__call_fn))))),
                    &(literal!("Connections")),
                )
        }
        _ => false,
    });
    isOp
}

pub(crate) fn isStreamOperator(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut isOp: bool;
    let mut name: ArcStr;
    isOp = (match &**call {
        TYPED_CALL { r#fn: __call_fn, .. } if (Function::isBuiltin(metamodelica::AsArg::as_arg(&__call_fn))) => {
            name = functionNameFirst(call)?;
            metamodelica::stringEq(&name, &(literal!("actualStream")))
                || metamodelica::stringEq(&name, &(literal!("inStream")))
        }
        _ => false,
    });
    Ok(isOp)
}

pub(crate) fn isCardinality(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut isCardinality: bool;
    isCardinality = (match &**call {
        TYPED_CALL { r#fn: __call_fn, .. } if (Function::isBuiltin(metamodelica::AsArg::as_arg(&__call_fn))) => {
            metamodelica::stringEq(&(functionNameFirst(call)?), &(literal!("cardinality")))
        }
        _ => false,
    });
    Ok(isCardinality)
}

fn instNormalCall(
    mut functionName: metamodelica::Ref<Absyn::ComponentRef>,
    mut functionArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut named_args: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    let mut name: ArcStr;
    name = AbsynUtil::crefFirstIdent(&functionName)?;
    if let Ok((__pa0, __pa1)) = instArgs(functionArgs, &scope, context, &info) {
        args = metamodelica::Own::own(__pa0);
        named_args = metamodelica::Own::own(__pa1);
    } else {
        if InstContext::inAnnotation(context)
            && !(InstContext::inInstanceAPI(context))
            && stringEq(&name, &(literal!("DynamicSelect")))
        {
            callExp = (match &**functionArgs {
                Absyn::FunctionArgs::FUNCTIONARGS {
                    args: __functionArgs_args,
                    ..
                } => Inst::instExp((__functionArgs_args).head().cloned()?, &scope, context, &info)?,
                _ => return Err("match: no arm matched"),
            });
            return Ok(callExp);
        } else {
            return Err("fail");
        }
    }
    callExp = (::match_deref::match_deref! { match &(name) {
        Deref @ "size" => BuiltinCall::makeSizeExp(args, &named_args, &info)?,
        Deref @ "array" => BuiltinCall::makeArrayExp(args, &named_args, &info)?,
        _ if (InstContext::inAnnotation(context)) => {
            match '__try0: {
                (fn_ref, _, _) = unwrap_break_err!(Function::instFunction(functionName.clone(), unwrap_break_err!(NFInstNode::InstNode::topScope(scope.clone()), '__try0), context, info.clone()), '__try0);
                Ok::<_, &'static str>((fn_ref.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    fn_ref = __try0_o0;
                }
                Err(_) => {
                    (fn_ref, _, _) = Function::instFunction(functionName.clone(), scope.clone(), context, info.clone())?;
                }
            }
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: metamodelica::Ref::new(NFCall::UNTYPED_CALL { r#ref: fn_ref, arguments: args, named_args: named_args, call_scope: NFInstNode::InstNode::scopeRef(scope) }) })
        },
        _ => {
            (fn_ref, _, _) = Function::instFunction(functionName, scope.clone(), context, info)?;
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: metamodelica::Ref::new(NFCall::UNTYPED_CALL { r#ref: fn_ref, arguments: args, named_args: named_args, call_scope: NFInstNode::InstNode::scopeRef(scope) }) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(callExp)
}

fn instArgs(
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>,
)> {
    let mut posArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut namedArgs: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)>;
    (posArgs, namedArgs) = (match &**args {
        Absyn::FunctionArgs::FUNCTIONARGS {
            argNames: __args_argNames,
            args: __args_args,
        } => {
            posArgs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut a in (__args_args.clone()).into_iter().cloned() {
                    let __x = Inst::instExp(a.clone(), scope, context, info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            namedArgs = ({
                let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Expression::NFExpression>)> =
                    metamodelica::nil();
                for mut a in (__args_argNames.clone()).into_iter().cloned() {
                    let __x = instNamedArg(&(a.clone()), scope, context, info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            (posArgs, namedArgs)
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.instArgs"));
                    __mm_s.push_str(&*literal!(" got unknown function args"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((posArgs, namedArgs))
}

fn instNamedArg(
    mut absynArg: &metamodelica::Ref<Absyn::NamedArg>,
    mut scope: &metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<(ArcStr, metamodelica::Ref<Expression::NFExpression>)> {
    let mut arg: (ArcStr, metamodelica::Ref<Expression::NFExpression>);
    let mut name: ArcStr;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let __arc2 = &(*absynArg);
    let Absyn::NAMEDARG {
        argName: __pa0,
        argValue: __pa1,
    } = &**__arc2;
    name = metamodelica::Own::own(__pa0);
    exp = metamodelica::Own::own(__pa1);
    arg = (name, Inst::instExp(exp, scope, context, info)?);
    Ok(arg)
}

fn instIteratorCall(
    mut functionName: metamodelica::Ref<Absyn::ComponentRef>,
    mut functionArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_name: metamodelica::Ref<Absyn::ComponentRef>;
    let mut fn_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    fn_name = (::match_deref::match_deref! { match &(functionName.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", .. } => metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("array"), subscripts: metamodelica::nil() }),
        _ => functionName,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, iters) = instIteratorCallArgs(functionArgs, scope.clone(), context, info.clone())?;
    if metamodelica::stringEq(&(AbsynUtil::crefFirstIdent(&fn_name)?), &(literal!("array"))) {
        callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(NFCall::UNTYPED_ARRAY_CONSTRUCTOR { exp: exp, iters: iters }),
        });
    } else {
        (fn_ref, _, _) = Function::instFunction(fn_name, scope, context, info)?;
        callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(NFCall::UNTYPED_REDUCTION {
                r#ref: fn_ref,
                exp: exp,
                iters: iters,
            }),
        });
    }
    Ok(callExp)
}

fn instIteratorCallArgs(
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let () = (match &**args {
        Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: __args_exp,
            iterators: __args_iterators,
            ..
        } => {
            let mut for_scope: metamodelica::Ref<InstNode::InstNode>;
            (for_scope, iters) = instIterators(__args_iterators.clone(), scope, context, info.clone())?;
            exp = Inst::instExp(__args_exp.clone(), &for_scope, context, &info)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((exp, iters))
}

fn instIterators(
    mut inIters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
)> {
    let mut outScope: metamodelica::Ref<InstNode::InstNode> = scope;
    let mut outIters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut range_node: metamodelica::Ref<InstNode::InstNode>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    for mut i in &*inIters.reverse() {
        if (i.range).is_some() {
            range = Inst::instExp(i.range.clone().ok_or("pattern mismatch")?, &outScope, context, &info)?;
        } else {
            range = metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: crate::NFType::interned_UNKNOWN(),
            });
        }
        ty = (::match_deref::match_deref! { match &(&*range) {
            Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } if (NFInstNode::InstNode::isComponent(&(ComponentRef::node(var_field!((*range).cref, Expression::NFExpression::CREF))?))?) => metamodelica::Ref::new(Type::NFType::COMPLEX { cls: NFInstNode::InstNode::identityCell(Component::classInstance(&(NFInstNode::InstNode::component(&(ComponentRef::node(var_field!((*range).cref, Expression::NFExpression::CREF))?))?))?), complexTy: crate::NFComplexType::interned_CLASS() }),
            _ => crate::NFType::interned_UNKNOWN(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (outScope, iter) = Inst::addIteratorToScope(i.name.clone(), outScope, info.clone(), ty)?;
        outIters = metamodelica::cons((iter, range), outIters);
    }
    Ok((outScope, outIters))
}

fn typeArrayConstructor(
    mut call: metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<NFCall>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut call: metamodelica::Ref<NFCall> = call;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut iter_ty: metamodelica::Ref<Type::NFType>;
    let mut iter_var: Variability;
    let mut exp_var: Variability;
    let mut iter_pur: Purity;
    let mut exp_pur: Purity;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut next_context: i32;
    let mut is_structural: bool;
    let mut has_iterator: bool;
    (call, ty, variability, purity) = (match &*call {
        UNTYPED_ARRAY_CONSTRUCTOR {
            exp: __call_exp,
            iters: __call_iters,
        } => {
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            is_structural = !(InstContext::inFunction(context));
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            for mut i in &*__call_iters.clone().reverse() {
                (iter, range) = i.clone();
                if Expression::isEmpty(&range) {
                    range = Typing::deduceIterationRangeExp(
                        metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() }),
                        &iter,
                        &info,
                    )?;
                }
                (range, iter_ty, iter_var, iter_pur) =
                    Typing::typeIterator(iter.clone(), range, next_context, is_structural)?;
                has_iterator = iter_pur == Purity::IMPURE.clone()
                    && Expression::contains(range.clone(), &move |__a0: metamodelica::Ref<
                        Expression::NFExpression,
                    >|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(Expression::isIterator(&__a0))
                    })?;
                if is_structural && !(has_iterator) {
                    if InstContext::inRelaxed(context) {
                        range = Ceval::tryEvalExp(range, &(Ceval::noTarget().clone()));
                    } else {
                        range = Ceval::evalExp(
                            range,
                            &(Ceval::EvalTarget::new(info.clone(), InstContext::ITERATION_RANGE.clone(), None)),
                        )?;
                    }
                    iter_ty = Expression::typeOf(range.clone());
                }
                dims = List::append_reverse(&(Type::arrayDims(iter_ty)), dims);
                variability = Prefixes::variabilityMax(variability, iter_var);
                purity = Prefixes::purityMin(purity, iter_pur);
                iters = metamodelica::cons((iter, range), iters);
            }
            dims = metamodelica::Dangerous::listReverseInPlace(dims);
            next_context = InstContext::set(next_context, InstContext::FOR.clone());
            (arg, ty, exp_var, exp_pur) = Typing::typeExp(__call_exp.clone(), next_context, &info, false)?;
            variability = Prefixes::variabilityMax(variability, exp_var);
            purity = Prefixes::purityMin(purity, exp_pur);
            ty = Type::liftArrayLeftList(ty, &dims);
            (
                metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR {
                    ty: ty.clone(),
                    var: variability,
                    purity: purity,
                    exp: arg,
                    iters: iters,
                }),
                ty,
                variability,
                purity,
            )
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.typeArrayConstructor"));
                    __mm_s.push_str(&*literal!(" got invalid function call expression"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((call, ty, variability, purity))
}

fn typeReduction(
    mut call: metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: SourceInfo,
) -> Result<(
    metamodelica::Ref<NFCall>,
    metamodelica::Ref<Type::NFType>,
    Variability,
    Purity,
)> {
    let mut call: metamodelica::Ref<NFCall> = call;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut variability: Variability;
    let mut purity: Purity;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut iter_var: Variability;
    let mut exp_var: Variability;
    let mut iter_pur: Purity;
    let mut exp_pur: Purity;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut next_context: i32;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    (call, ty, variability, purity) = (match &*call {
        UNTYPED_REDUCTION {
            exp: __call_exp,
            iters: __call_iters,
            r#ref: __call_ref,
        } => {
            variability = Variability::CONSTANT.clone();
            purity = Purity::PURE.clone();
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            for mut i in &*__call_iters.clone().reverse() {
                (iter, range) = i.clone();
                if Expression::isEmpty(&range) {
                    range = Typing::deduceIterationRangeExp(
                        metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() }),
                        &iter,
                        &info,
                    )?;
                }
                (range, _, iter_var, iter_pur) = Typing::typeIterator(iter.clone(), range, context, false)?;
                variability = Prefixes::variabilityMax(variability, iter_var);
                purity = Prefixes::purityMin(purity, iter_pur);
                iters = metamodelica::cons((iter, range), iters);
            }
            next_context = InstContext::set(next_context, InstContext::FOR.clone());
            (arg, ty, exp_var, exp_pur) = Typing::typeExp(__call_exp.clone(), next_context, &info, false)?;
            variability = Prefixes::variabilityMax(variability, exp_var);
            purity = Prefixes::purityMin(purity, exp_pur);
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeRefCache(metamodelica::AsArg::as_arg(&__call_ref), InstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r#fn = metamodelica::Own::own(__pa0);
            TypeCheck::checkReductionType(&ty, Function::name(&r#fn), __call_exp.clone(), &info)?;
            (
                makeTypedReduction(r#fn, ty.clone(), variability, purity, arg, iters, info)?,
                ty,
                variability,
                purity,
            )
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCall.typeReduction"));
                    __mm_s.push_str(&*literal!(" got invalid reduction call"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((call, ty, variability, purity))
}

pub(crate) fn makeTypedReduction(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut var: Variability,
    mut purity: Purity,
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall>;
    let mut fold_id: ArcStr;
    let mut res_id: ArcStr;
    let mut default_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut fold_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut fold_tuple: (Option<metamodelica::Ref<Expression::NFExpression>>, ArcStr, ArcStr);
    fold_id = Util::getTempVariableIndex();
    res_id = Util::getTempVariableIndex();
    default_exp = reductionDefaultValue(&r#fn, &ty)?;
    fold_exp = reductionFoldExpression(
        r#fn.clone(),
        ty.clone(),
        var,
        purity,
        fold_id.clone(),
        res_id.clone(),
        info,
    )?;
    fold_tuple = (fold_exp, fold_id, res_id);
    call = metamodelica::Ref::new(NFCall::TYPED_REDUCTION {
        r#fn: r#fn,
        ty: ty,
        var: var,
        purity: purity,
        exp: arg,
        iters: iters,
        defaultExp: default_exp,
        foldExp: fold_tuple,
    });
    Ok(call)
}

fn reductionDefaultValue(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut ty: &metamodelica::Ref<Type::NFType>,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut defaultValue: Option<metamodelica::Ref<Expression::NFExpression>>;
    if Type::isArray(ty) {
        defaultValue = None;
    } else {
        defaultValue = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(Function::name(r#fn)))) {
            Deref @ "sum" => Some(Expression::makeZero(ty)?),
            Deref @ "product" => Some(Expression::makeOne(ty)?),
            Deref @ "min" => Some(Expression::makeMaxValue(ty)?),
            Deref @ "max" => Some(Expression::makeMinValue(ty)?),
            _ => {
                Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.reductionDefaultValue")); __mm_s.push_str(&*literal!(" got unknown reduction name ")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(&(Function::name(r#fn)))); ArcStr::from(__mm_s) }], &(metamodelica::sourceInfo!("NFFrontEnd/NFCall.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(defaultValue)
}

fn reductionFoldExpression(
    mut reductionFn: metamodelica::Ref<Function::Function>,
    mut reductionType: metamodelica::Ref<Type::NFType>,
    mut reductionVar: Variability,
    mut reductionPurity: Purity,
    mut foldId: ArcStr,
    mut resultId: ArcStr,
    mut info: SourceInfo,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut foldExp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut op_node: metamodelica::Ref<InstNode::InstNode>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    if Type::isComplex(&reductionType) {
        foldExp = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(Function::name(&reductionFn)))) {
            Deref @ "sum" => {
                op_node = Type::complexNode(&reductionType)?;
                (op_node, _) = Class::lookupElement(literal!("'+'"), NFInstNode::InstNode::getClass(op_node)?)?;
                Function::instFunctionNode(op_node.clone(), InstContext::NO_CONTEXT.clone(), info)?;
                let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(op_node, InstContext::FUNCTION.clone())?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                r#fn = metamodelica::Own::own(__pa0);
                Some(metamodelica::Ref::new(Expression::NFExpression::CALL { call: makeTypedCall(r#fn.clone(), list![reductionFoldIterator(resultId, reductionType.clone())?, reductionFoldIterator(foldId, reductionType)?], reductionVar, reductionPurity, r#fn.returnType.clone()) }))
            },
            _ => None,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    } else {
        foldExp = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(Function::name(&reductionFn)))) {
            Deref @ "sum" => Some(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: reductionFoldIterator(resultId, reductionType.clone())?, operator: Operator::makeAdd(reductionType.clone()), exp2: reductionFoldIterator(foldId, reductionType)? })),
            Deref @ "product" => Some(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: reductionFoldIterator(resultId, reductionType.clone())?, operator: Operator::makeMul(reductionType.clone()), exp2: reductionFoldIterator(foldId, reductionType)? })),
            Deref @ "$array" => None,
            Deref @ "array" => None,
            Deref @ "list" => None,
            Deref @ "listReverse" => None,
            _ => Some(metamodelica::Ref::new(Expression::NFExpression::CALL { call: makeTypedCall(reductionFn, list![reductionFoldIterator(foldId, reductionType.clone())?, reductionFoldIterator(resultId, reductionType.clone())?], reductionVar, reductionPurity, reductionType) })),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(foldExp)
}

fn reductionFoldIterator(
    mut name: ArcStr,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut iterExp: metamodelica::Ref<Expression::NFExpression>;
    iterExp = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: ty.clone(),
        cref: ComponentRef::makeIterator(metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: name }), ty)?,
    });
    Ok(iterExp)
}

fn typeArgs(
    mut call: metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut call: metamodelica::Ref<NFCall> = call;
    call = (match &*call {
        UNTYPED_CALL {
            arguments: __call_arguments,
            call_scope: __call_call_scope,
            named_args: __call_named_args,
            r#ref: __call_ref,
        } => {
            let mut arg: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
            let mut arg_ty: metamodelica::Ref<Type::NFType>;
            let mut arg_var: Variability;
            let mut arg_pur: Purity;
            let mut typed_args: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut typed_nargs: metamodelica::List<metamodelica::Ref<TypedArg>>;
            let mut name: ArcStr;
            let mut next_context: i32;
            typed_args = metamodelica::nil();
            next_context = InstContext::set(context, InstContext::SUBEXPRESSION.clone());
            for mut arg in &*__call_arguments.clone() {
                let mut arg = arg.clone();
                (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg, next_context, info, false)?;
                typed_args = metamodelica::cons(
                    metamodelica::Ref::new(TypedArg {
                        name: None,
                        value: arg,
                        ty: arg_ty,
                        var: arg_var,
                        purity: arg_pur,
                    }),
                    typed_args,
                );
            }
            typed_args = typed_args.reverse();
            typed_nargs = metamodelica::nil();
            for mut narg in &*__call_named_args.clone() {
                (name, arg) = narg.clone();
                (arg, arg_ty, arg_var, arg_pur) = Typing::typeExp(arg, next_context, info, false)?;
                typed_nargs = metamodelica::cons(
                    metamodelica::Ref::new(TypedArg {
                        name: Some(name),
                        value: arg,
                        ty: arg_ty,
                        var: arg_var,
                        purity: arg_pur,
                    }),
                    typed_nargs,
                );
            }
            typed_nargs = typed_nargs.reverse();
            metamodelica::Ref::new(NFCall::ARG_TYPED_CALL {
                r#ref: __call_ref.clone(),
                positional_args: typed_args,
                named_args: typed_nargs,
                call_scope: __call_call_scope.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(call)
}

fn checkMatchingFunctions(
    mut call: &metamodelica::Ref<NFCall>,
    mut context: i32,
    mut info: &SourceInfo,
    mut vectorize: bool,
) -> Result<metamodelica::Ref<MatchedFunction::MatchedFunction>> {
    let mut matchedFunc: metamodelica::Ref<MatchedFunction::MatchedFunction>;
    let mut matchedFunctions: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    let mut exactMatches: metamodelica::List<metamodelica::Ref<MatchedFunction::MatchedFunction>>;
    let mut func: metamodelica::Ref<Function::Function>;
    let mut allfuncs: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
    let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
    let mut numerr: i32 = Error::getNumErrorMessages();
    ErrorExt::setCheckpoint(literal!("NFCall:checkMatchingFunctions"));
    matchedFunctions = (::match_deref::match_deref! { match call {
        Deref @ ARG_TYPED_CALL { r#ref: Deref @ ComponentRef::CREF { .. }, named_args: __call_named_args, positional_args: __call_positional_args, .. } => {
            fn_node = ComponentRef::node(var_field!((**call).r#ref, NFCall::ARG_TYPED_CALL))?;
            allfuncs = Function::getCachedFuncs(fn_node)?;
            if ((allfuncs).len() as i32) > 1 {
                allfuncs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
        for mut r#fn in (allfuncs).into_iter().cloned() {
            if !(!(Function::isDefaultRecordConstructor(&(r#fn.clone()))?)) { continue; }
            let __x = r#fn.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            Function::matchFunctions(&allfuncs, __call_positional_args.clone(), metamodelica::AsArg::as_arg(&__call_named_args), context, info, vectorize)?
        },
        _ => return Err("match: no arm matched"),
    } });
    if (matchedFunctions).is_empty() {
        if ((allfuncs).len() as i32) > 1 {
            ErrorExt::rollBack(literal!("NFCall:checkMatchingFunctions"));
            Error::addSourceMessage(
                &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
                list![typedString(call)?, Function::candidateFuncListString(allfuncs)?],
                info,
            )?;
        } else if numerr == Error::getNumErrorMessages() {
            ErrorExt::rollBack(literal!("NFCall:checkMatchingFunctions"));
            Error::addSourceMessage(
                &(Error::NO_MATCHING_FUNCTION_FOUND_NFINST.clone()),
                list![typedString(call)?, Function::candidateFuncListString(allfuncs)?],
                info,
            )?;
        } else {
            ErrorExt::delCheckpoint(literal!("NFCall:checkMatchingFunctions"));
        }
        return Err("fail");
    }
    ErrorExt::rollBack(literal!("NFCall:checkMatchingFunctions"));
    if ((matchedFunctions).len() as i32) > 1 {
        exactMatches = MatchedFunction::getExactMatches(matchedFunctions.clone());
        if (exactMatches).is_empty() {
            exactMatches = MatchedFunction::getExactVectorizedMatches(matchedFunctions.clone());
        }
        if ((exactMatches).len() as i32) > 1 {
            Error::addSourceMessage(
                &(Error::AMBIGUOUS_MATCHING_FUNCTIONS_NFINST.clone()),
                list![
                    typedString(call)?,
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
            return Err("fail");
        }
        matchedFunc = (exactMatches).head().cloned()?;
    } else {
        matchedFunc = (matchedFunctions).head().cloned()?;
    }
    if Function::isBuiltin(&matchedFunc.func) {
        func = matchedFunc.func.clone();
        assign_field!(func.path = Function::nameConsiderBuiltin(&func));
        assign_field!(matchedFunc.func = func);
    }
    Ok(matchedFunc)
}

fn iteratorToDAE(
    mut iter: &(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    ),
) -> Result<metamodelica::Ref<DAE::ReductionIterator>> {
    let mut diter: metamodelica::Ref<DAE::ReductionIterator>;
    let mut iter_node: metamodelica::Ref<InstNode::InstNode>;
    let mut iter_range: metamodelica::Ref<Expression::NFExpression>;
    (iter_node, iter_range) = iter.clone();
    diter = metamodelica::Ref::new(DAE::ReductionIterator {
        id: NFInstNode::InstNode::name(&iter_node)?,
        exp: Expression::toDAE(iter_range, false)?,
        guardExp: None,
        ty: Type::toDAE(&(NFInstNode::InstNode::getType(iter_node)?), true)?,
    });
    Ok(diter)
}

fn vectorizeCall(
    mut base_call: metamodelica::Ref<NFCall>,
    mut mk: &metamodelica::Ref<FunctionMatchKind::FunctionMatchKind>,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<NFCall>> {
    let mut vectorized_call: metamodelica::Ref<NFCall>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut vect_ty: metamodelica::Ref<Type::NFType>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut i: i32;
    let mut call_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    vectorized_call = (::match_deref::match_deref! { match &((base_call.clone(), mk.clone())) {
        (Deref @ TYPED_CALL { arguments: __esc_call_args, .. }, Deref @ FunctionMatchKind::VECTORIZED { .. }) => {
            call_args = (*__esc_call_args).clone();
            iters = metamodelica::nil();
            i = 1;
            for mut dim in &*var_field!((**mk).vectDims, FunctionMatchKind::FunctionMatchKind::VECTORIZED).clone() {
                Error::assertion(Dimension::isKnown(metamodelica::AsArg::as_arg(&dim), true), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.vectorizeCall")); __mm_s.push_str(&*literal!(" got unknown dimension for vectorized call")); ArcStr::from(__mm_s) }, &info)?;
                ty = metamodelica::Ref::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![dim.clone()] });
                exp = metamodelica::Ref::new(Expression::NFExpression::RANGE { ty: ty, start: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), step: None, stop: Dimension::sizeExp(metamodelica::AsArg::as_arg(&dim))? });
                iter = NFInstNode::InstNode::newUniqueIterator(info.clone(), crate::NFType::interned_INTEGER());
                iters = metamodelica::cons((iter.clone(), exp), iters);
                exp = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: crate::NFType::interned_INTEGER(), cref: ComponentRef::makeIterator(iter, crate::NFType::interned_INTEGER())? });
                sub = metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: exp });
                call_args = List::mapIndices(call_args.clone(), var_field!((**mk).vectorizedArgs, FunctionMatchKind::FunctionMatchKind::VECTORIZED), &({ let __pe_b0 = sub; let __pe_b2 = metamodelica::nil(); let __pe_b3 = false; move |__pe_a1| Expression::applySubscript(&__pe_b0, &__pe_a1, &__pe_b2, __pe_b3.clone()) }))?;
                i = i + 1;
            }
            vect_ty = Type::liftArrayLeftList(var_field!((*base_call).ty, NFCall::TYPED_CALL).clone(), var_field!((**mk).vectDims, FunctionMatchKind::FunctionMatchKind::VECTORIZED));
            assign_variant_field!(base_call => NFCall::TYPED_CALL; arguments = call_args.clone());
            metamodelica::Ref::new(NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: vect_ty, var: var_field!((*base_call).var, NFCall::TYPED_CALL).clone(), purity: var_field!((*base_call).purity, NFCall::TYPED_CALL).clone(), exp: metamodelica::Ref::new(Expression::NFExpression::CALL { call: base_call }), iters: iters })
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCall.vectorizeCall")); __mm_s.push_str(&*literal!(" got unknown call")); ArcStr::from(__mm_s) }, info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vectorized_call)
}

fn isVectorized(mut call: &metamodelica::Ref<NFCall>) -> Result<bool> {
    let mut vectorized: bool;
    vectorized = (::match_deref::match_deref! { match call {
        Deref @ TYPED_ARRAY_CONSTRUCTOR { exp: Deref @ Expression::CALL { .. }, iters: __call_iters, .. } => stringGet(&(NFInstNode::InstNode::name(&(Util::tuple21((__call_iters).head().cloned()?)))?),1)? == 36,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vectorized)
}

fn devectorizeCall(mut call: &metamodelica::Ref<NFCall>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iter_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut iter_node: metamodelica::Ref<InstNode::InstNode>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ TYPED_ARRAY_CONSTRUCTOR { exp: __pa0, iters: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    iters = metamodelica::Own::own(__pa1);
    for mut i in &*iters {
        (iter_node, iter_exp) = i.clone();
        exp = Expression::replaceIterator(exp, &iter_node, &iter_exp)?;
    }
    result = SimplifyExp::simplify(exp, false)?;
    Ok(result)
}

fn evaluateCallType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut outputIndex: i32,
    mut ptree: ParameterTree,
) -> Result<(metamodelica::Ref<Type::NFType>, ParameterTree)> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    let mut ptree: ParameterTree = ptree;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut binding_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut t: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
    let mut output_index: i32;
    ty = (match &*ty {
        Type::ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => {
            (dims, ptree) = List::mapFold(
                metamodelica::AsArg::as_arg(&__ty_dimensions),
                &({
                    let __pe_b1 = r#fn.clone();
                    let __pe_b2 = args.clone();
                    move |__pe_a0, __pe_a3| evaluateCallTypeDim(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_a3)
                }),
                ptree,
            )?;
            assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = dims);
            ty.clone()
        }
        Type::TUPLE { types: __ty_types, .. } => {
            tys = metamodelica::nil();
            output_index = 1;
            for mut t in &*__ty_types.clone() {
                let mut t = t.clone();
                (t, ptree) = evaluateCallType(t, r#fn, args, output_index, ptree)?;
                tys = metamodelica::cons(t, tys);
                output_index = output_index + 1;
            }
            assign_variant_field!(ty => Type::NFType::TUPLE; types = metamodelica::Dangerous::listReverseInPlace(tys));
            ty.clone()
        }
        Type::COMPLEX { .. } if (Type::isRecord(&ty) && !(Function::isNonDefaultRecordConstructor(r#fn))) => {
            binding = Component::getBinding(
                &(NFInstNode::InstNode::component(
                    &(NFInstNode::InstNode::fromHandle(&((r#fn.outputs).get(outputIndex)?))?),
                )?),
            );
            if Binding::isBound(&binding) {
                binding_exp = Binding::getExp(&binding)?;
                ptree = buildParameterTree(r#fn, args.clone(), ptree)?;
                binding_exp = Expression::map(
                    binding_exp,
                    (std::sync::Arc::new({
                        let __pe_b1 = ptree.clone();
                        move |__pe_a0| evaluateCallTypeDimExp(__pe_a0, &__pe_b1)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                t = Expression::typeOf(binding_exp);
            } else {
                t = ty.clone();
            }
            t
        }
        _ => ty.clone(),
    });
    Ok((ty, ptree))
}

fn evaluateCallTypeDim(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut ptree: ParameterTree,
) -> Result<(metamodelica::Ref<Dimension::NFDimension>, ParameterTree)> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    let mut ptree: ParameterTree = ptree;
    dim = (match &*dim {
        Dimension::EXP { exp: __dim_exp, .. } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            ptree = buildParameterTree(r#fn, args, ptree)?;
            exp = Expression::map(
                __dim_exp.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = ptree.clone();
                    move |__pe_a0| evaluateCallTypeDimExp(__pe_a0, &__pe_b1)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            ErrorExt::setCheckpoint(literal!("NFCall.evaluateCallTypeDim"));
            if '__try0: {
                unwrap_break_err!(Structural::markExp(&exp), '__try0);
                exp = unwrap_break_err!(Ceval::evalExp(exp.clone(), &(Ceval::noTarget().clone())), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
            ErrorExt::rollBack(literal!("NFCall.evaluateCallTypeDim"));
            Dimension::fromExp(exp, Variability::CONSTANT.clone())?
        }
        _ => dim,
    });
    Ok((dim, ptree))
}

fn buildParameterTree(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut ptree: ParameterTree,
) -> Result<ParameterTree> {
    let mut ptree: ParameterTree = ptree;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = args;
    if !(NFCallParameterTree::isEmpty(&ptree)) {
        return Ok(ptree);
    }
    for mut i in &*r#fn.inputs.clone() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        ptree = NFCallParameterTree::add(
            ptree,
            &(NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&i))?),
            &arg,
            &*(std::sync::Arc::new(NFCallParameterTree::addConflictDefault)
                as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
        )?;
    }
    Ok(ptree)
}

fn evaluateCallTypeDimExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ptree: &ParameterTree,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut cref_parts: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ComponentRef::toListReverse(var_field!((*exp).cref, Expression::NFExpression::CREF), true, metamodelica::nil())) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cref = metamodelica::Own::own(__pa0);
            cref_parts = metamodelica::Own::own(__pa1);
            oexp = NFCallParameterTree::getOpt(ptree, ComponentRef::nodeName(&cref)?);
            if (oexp).is_some() {
                let __pa2 = ::match_deref::match_deref! { match &(oexp) {
                    Some(__pa2) => __pa2.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                outExp = metamodelica::Own::own(__pa2);
                outExp = Expression::applySubscripts(&(ComponentRef::getSubscripts(&cref)), outExp, false)?;
                for mut cr in &*cref_parts {
                    outExp = Expression::recordElement(&(ComponentRef::nodeName(metamodelica::AsArg::as_arg(&cr))?), &outExp)?;
                    outExp = Expression::applySubscripts(&(ComponentRef::getSubscripts(metamodelica::AsArg::as_arg(&cr))), outExp, false)?;
                }
            } else {
                outExp = exp;
            }
            outExp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn resolvePolymorphicReturnType(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<TypedArg>>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut outType: metamodelica::Ref<Type::NFType>;
    let mut name: ArcStr;
    let mut input_ty: metamodelica::Ref<Type::NFType>;
    let mut arg: metamodelica::Ref<TypedArg>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<TypedArg>> = args.clone();
    outType = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::POLYMORPHIC { name: __esc_name } => {
            name = (*__esc_name).clone();
            for mut i in &*r#fn.inputs.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                arg = metamodelica::Own::own(__pa0);
                rest_args = metamodelica::Own::own(__pa1);
                input_ty = NFInstNode::InstNode::getType(i.clone())?;
                if Type::isPolymorphicNamed(&(Type::arrayElementType(&input_ty)), metamodelica::AsArg::as_arg(&name)) {
                    outType = Type::unliftArrayN(Type::dimensionCount(input_ty), arg.ty.clone())?;
                    return Ok(outType);
                }
            }
            if metamodelica::stringEq(&name, &(literal!("__Scalar"))) {
                outType = resolvePolymorphicReturnType(r#fn, args, metamodelica::Ref::new(Type::NFType::POLYMORPHIC { name: literal!("__Array") }))?;
                outType = Type::arrayElementType(&outType);
                return Ok(outType);
            }
            return Err("fail")
        },
        Deref @ Type::ARRAY { elementType: Deref @ Type::POLYMORPHIC { .. }, .. } => {
            assign_variant_field!(ty => Type::NFType::ARRAY; elementType = resolvePolymorphicReturnType(r#fn, args, var_field!((*ty).elementType, Type::NFType::ARRAY).clone())?);
            ty
        },
        _ => ty,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}
