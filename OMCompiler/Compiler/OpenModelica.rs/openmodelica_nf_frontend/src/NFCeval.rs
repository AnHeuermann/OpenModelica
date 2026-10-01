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
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use crate::NFTyping::TypingError;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

pub mod EvalTarget {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct EvalTarget {
        pub info: SourceInfo,
        pub context: i32,
        pub extra: Option<metamodelica::Ref<EvalTargetData>>,
    }

    impl metamodelica::gc::MMTrace for EvalTarget {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.context, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.extra, __mmv)?;
            Ok(())
        }
    }
    impl Default for EvalTarget {
        fn default() -> Self {
            Self {
                info: Default::default(),
                context: Default::default(),
                extra: Default::default(),
            }
        }
    }

    pub type EVAL_TARGET = EvalTarget;

    pub fn new(
        mut info: SourceInfo,
        mut context: i32,
        mut extra: Option<metamodelica::Ref<EvalTargetData>>,
    ) -> metamodelica::Ref<EvalTarget> {
        let mut target: metamodelica::Ref<EvalTarget> = metamodelica::Ref::new(EvalTarget {
            info: info.clone(),
            context: context,
            extra: extra.clone(),
        });
        target
    }

    pub(crate) fn hasInfo(mut target: &metamodelica::Ref<EvalTarget>) -> bool {
        let mut res: bool = !(stringEmpty(&target.info.fileName));
        res
    }

    pub(crate) fn getInfo(mut target: &metamodelica::Ref<EvalTarget>) -> SourceInfo {
        let mut info: SourceInfo = target.info.clone();
        info
    }
}

thread_local! { static __noTarget_TLS: metamodelica::Ref<EvalTarget::EvalTarget> = metamodelica::Ref::new(EvalTarget::EvalTarget { info: Absyn::dummyInfo.clone(), context: InstContext::NO_CONTEXT.clone(), extra: None }); }
pub fn noTarget() -> metamodelica::Ref<EvalTarget::EvalTarget> {
    __noTarget_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EvalTargetData {
    pub component: metamodelica::Ref<InstNode::InstNode>,
    pub index: i32,
    pub exp: metamodelica::Ref<Expression::NFExpression>,
}

impl metamodelica::gc::MMTrace for EvalTargetData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.component, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exp, __mmv)?;
        Ok(())
    }
}
impl Default for EvalTargetData {
    fn default() -> Self {
        Self {
            component: Default::default(),
            index: Default::default(),
            exp: Default::default(),
        }
    }
}

pub type DIMENSION_DATA = EvalTargetData;

pub(crate) fn tryEvalExpResizable(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint(literal!("NFCeval.tryEvalExpResizable"));
    match '__try0: {
        exp = unwrap_break_err!(evalExp(exp.clone(), target), '__try0);
        ErrorExt::delCheckpoint(literal!("NFCeval.tryEvalExpResizable"));
        Ok::<_, &'static str>((exp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            exp = __try0_o0;
        }
        Err(_) => {
            exp = tryEvalExpPartial(exp.clone(), target);
            if Expression::contains(exp.clone(), &move |__a0: metamodelica::Ref<
                Expression::NFExpression,
            >| Expression::isResizableCref(&__a0))?
            {
                ErrorExt::rollBack(literal!("NFCeval.tryEvalExpResizable"));
            } else {
                ErrorExt::delCheckpoint(literal!("NFCeval.tryEvalExpResizable"));
                return Err("fail");
            }
        }
    }
    Ok(exp)
}

pub fn tryEvalExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint(literal!("NFCeval.tryEvalExp"));
    if '__try0: {
        exp = unwrap_break_err!(evalExp(exp.clone(), target), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    ErrorExt::rollBack(literal!("NFCeval.tryEvalExp"));
    exp
}

pub fn evalExp<'__b>(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &'__b metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    '__tco: loop {
        match &*exp.clone() {
            Expression::CREF { cref: __exp_cref, .. } => {
                return Ok(evalCref(
                    metamodelica::AsArg::as_arg(&__exp_cref),
                    exp,
                    target,
                    true,
                    true,
                )?);
            }
            Expression::TYPENAME { ty: __exp_ty } => return Ok(evalTypename(__exp_ty.clone(), exp, target)?),
            Expression::ARRAY {
                literal: __exp_literal,
                ty: __exp_ty,
                ..
            } => {
                if (__exp_literal.clone()) {
                    return Ok(exp);
                } else {
                    return Ok(Expression::makeArrayCheckLiteral(
                        __exp_ty.clone(),
                        Array::map(
                            var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(),
                            &({
                                let __pe_b1 = target.clone();
                                move |__pe_a0| evalExp(__pe_a0, &__pe_b1)
                            }),
                        )?,
                    )?);
                }
            }
            Expression::RANGE { .. } => return Ok(evalRange(&exp, target)?),
            Expression::TUPLE {
                elements: __exp_elements,
                ..
            } => {
                assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                    for mut e in (__exp_elements.clone()).into_iter().cloned() {
                        let __x = evalExp(e.clone(), target)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                return Ok(exp);
            }
            Expression::RECORD {
                elements: __exp_elements,
                ..
            } => {
                assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                    for mut e in (__exp_elements.clone()).into_iter().cloned() {
                        let __x = evalExp(e.clone(), target)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                return Ok(exp);
            }
            Expression::CALL { call: __exp_call } => return Ok(evalCall(__exp_call.clone(), target)?),
            Expression::SIZE {
                dimIndex: __exp_dimIndex,
                exp: __exp_exp,
            } => return Ok(evalSize(__exp_exp.clone(), __exp_dimIndex.clone(), target)?),
            Expression::BINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut exp2: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_exp1.clone(), target)?;
                exp2 = evalExp(__exp_exp2.clone(), target)?;
                return Ok(evalBinaryOp(exp1, __exp_operator.clone(), exp2, target)?);
            }
            Expression::MULTARY { .. } => {
                (exp, target) = (SimplifyExp::splitMultary(exp)?, target);
                continue '__tco;
            }
            Expression::UNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_exp.clone(), target)?;
                return Ok(evalUnaryOp(exp1, __exp_operator.clone())?);
            }
            Expression::LBINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut exp2: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_exp1.clone(), target)?;
                if Expression::isSplitSubscriptedExp(&exp1) {
                    exp2 = evalExp(__exp_exp2.clone(), target)?;
                } else {
                    exp2 = __exp_exp2.clone();
                }
                return Ok(evalLogicBinaryOp(exp1, __exp_operator.clone(), exp2, target)?);
            }
            Expression::LUNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_exp.clone(), target)?;
                return Ok(evalLogicUnaryOp(exp1, __exp_operator.clone())?);
            }
            Expression::RELATION {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
                ..
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut exp2: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_exp1.clone(), target)?;
                exp2 = evalExp(__exp_exp2.clone(), target)?;
                return Ok(evalRelationOp(exp1, __exp_operator.clone(), exp2)?);
            }
            Expression::IF { .. } => return Ok(evalIfExp(&exp, target)?),
            Expression::CAST {
                exp: __exp_exp,
                ty: __exp_ty,
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_exp.clone(), target)?;
                return Ok(evalCast(exp1, __exp_ty.clone())?);
            }
            Expression::BOX { exp: __exp_exp } => {
                (exp, target) = (__exp_exp.clone(), target);
                continue '__tco;
            }
            Expression::UNBOX { exp: __exp_exp, .. } => {
                (exp, target) = (__exp_exp.clone(), target);
                continue '__tco;
            }
            Expression::SUBSCRIPTED_EXP {
                exp: __exp_exp,
                subscripts: __exp_subscripts,
                ..
            } => return Ok(evalSubscriptedExp(__exp_exp.clone(), __exp_subscripts.clone(), target)?),
            Expression::TUPLE_ELEMENT {
                index: __exp_index,
                tupleExp: __exp_tupleExp,
                ..
            } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                exp1 = evalExp(__exp_tupleExp.clone(), target)?;
                return Ok(Expression::tupleElement(exp1, __exp_index.clone())?);
            }
            Expression::RECORD_ELEMENT { .. } => return Ok(evalRecordElement(exp, target)?),
            Expression::MUTABLE { exp: __exp_exp } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                {
                    (exp, target) = (Mutable::access(__exp_exp.clone()), target);
                    continue '__tco;
                }
            }
            Expression::INSTANCE_NAME { scope: __exp_scope } => return Ok(evalGetInstanceName(__exp_scope.clone())?),
            _ => return Ok(exp),
        }
    }
}

pub(crate) fn tryEvalExpPartial(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint(literal!("NFCeval.tryEvalExpPartial"));
    if '__try0: {
        (exp, _) = unwrap_break_err!(evalExpPartial(exp.clone(), target, true), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    ErrorExt::rollBack(literal!("NFCeval.tryEvalExpPartial"));
    exp
}

pub(crate) fn evalExpPartialDefault(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    (exp, _) = evalExpPartial(exp, &(noTarget().clone()), true)?;
    Ok(exp)
}

pub(crate) fn evalExpPartial(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
    mut evaluated: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outEvaluated: bool;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    (e, outEvaluated) = Expression::mapFoldShallow(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = target.clone();
            move |__pe_a0, __pe_a2| evalExpPartial(__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        bool,
                    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                    + 'static,
            >),
        true,
    )?;
    outExp = (match &*e.clone() {
        Expression::CREF { cref: __e_cref, .. } => {
            if ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__e_cref)) {
                outExp = e;
                outEvaluated = false;
            } else {
                outExp = evalCref(metamodelica::AsArg::as_arg(&__e_cref), e, target, false, true)?;
                outEvaluated = Expression::isLiteral(&outExp)?;
            }
            outExp
        }
        Expression::MUTABLE { .. } => {
            outEvaluated = false;
            e
        }
        _ => {
            if (outEvaluated) {
                evalExp(e, target)?
            } else {
                e
            }
        }
    });
    outEvaluated = evaluated && outEvaluated;
    Ok((outExp, outEvaluated))
}

pub(crate) fn evalCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut defaultExp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
    mut evalSubscripts: bool,
    mut liftExp: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut c: metamodelica::Ref<InstNode::InstNode>;
    exp = (match &**cref {
        ComponentRef::CREF { .. }
            if (NFInstNode::InstNode::isComponent(&(ComponentRef::node(cref)?))?
                && !(ComponentRef::isIterator(cref))
                && ComponentRef::nodeVariability(cref)? <= Variability::NON_STRUCTURAL_PARAMETER.clone()) =>
        {
            evalComponentBinding(
                ComponentRef::node(cref)?,
                cref,
                defaultExp,
                target,
                evalSubscripts,
                liftExp,
            )?
        }
        _ => defaultExp,
    });
    Ok(exp)
}

pub(crate) fn evalComponentBinding(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut defaultExp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
    mut evalSubscripts: bool,
    mut liftExp: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut exp_context: i32;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut evaluated: bool;
    let mut start_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut cref_ty: metamodelica::Ref<Type::NFType>;
    let mut exp_ty: metamodelica::Ref<Type::NFType>;
    let mut dim_diff: i32;
    let mut errors: metamodelica::List<i32>;
    exp_context = InstContext::nodeContext(&node, target.context.clone())?;
    Typing::typeComponentBinding(node.clone(), exp_context, false)?;
    comp = NFInstNode::InstNode::component(&node)?;
    binding = Component::getBinding(&comp);
    if Binding::isUnbound(&binding) {
        binding = makeComponentBinding(comp.clone(), node.clone(), Expression::toCref(&defaultExp)?, target);
        if Binding::isUnbound(&binding) {
            start_exp = evalComponentStartBinding(node.clone(), &comp, cref, target, evalSubscripts)?;
            if (start_exp).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(start_exp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                return Ok(exp);
            }
        }
    }
    (exp, evaluated) = (match &*binding {
        Binding::TYPED_BINDING { .. } => {
            exp = (match Mutable::access(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone()) {
                Binding::EvalState::NOT_EVALUATED => {
                    Mutable::update(
                        var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(),
                        Binding::EvalState::EVALUATING.clone(),
                    );
                    ErrorExt::setCheckpoint(literal!("NFCeval.evalComponentBinding"));
                    match '__try0: {
                        exp = unwrap_break_err!(evalExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), target), '__try0);
                        ErrorExt::delCheckpoint(literal!("NFCeval.evalComponentBinding"));
                        Ok::<_, &'static str>((exp.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            exp = __try0_o0;
                        }
                        Err(__try0_err) => {
                            Mutable::update(
                                var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(),
                                Binding::EvalState::NOT_EVALUATED.clone(),
                            );
                            errors = ErrorExt::popCheckPoint(literal!("NFCeval.evalComponentBinding"));
                            Error::addSourceMessage(
                                &(Error::ERROR_FROM_HERE.clone()),
                                metamodelica::nil(),
                                var_field!((*binding).info, Binding::NFBinding::TYPED_BINDING),
                            )?;
                            ErrorExt::pushMessages(errors.clone());
                            return Err(__try0_err);
                        }
                    }
                    assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = exp.clone());
                    comp = Component::setBinding(binding.clone(), comp)?;
                    NFInstNode::InstNode::updateComponent(comp, node)?;
                    Mutable::update(
                        var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(),
                        Binding::EvalState::EVALUATED.clone(),
                    );
                    exp
                }
                Binding::EvalState::EVALUATED => {
                    var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone()
                }
                _ => {
                    Error::addSourceMessage(
                        &(Error::CIRCULAR_PARAM.clone()),
                        list![
                            NFInstNode::InstNode::name(&node)?,
                            Prefixes::variabilityString(Component::variability(&comp)?)?
                        ],
                        &(NFInstNode::InstNode::info(&node)),
                    )?;
                    return Err("fail");
                }
            });
            (exp, true)
        }
        Binding::CEVAL_BINDING {
            bindingExp: __binding_bindingExp,
        } => (__binding_bindingExp.clone(), true),
        Binding::UNBOUND => {
            printUnboundError(&comp, target, defaultExp.clone())?;
            (defaultExp.clone(), false)
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalComponentBinding"));
                    __mm_s.push_str(&*literal!(" failed on untyped binding"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    if evaluated {
        exp = subscriptBinding(exp, cref, evalSubscripts)?;
    }
    if liftExp
        && !(Expression::contains(exp.clone(), &move |__a0: metamodelica::Ref<
            Expression::NFExpression,
        >|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Expression::isSplitSubscriptedExp(&__a0))
        })?)
    {
        exp_ty = Expression::typeOf(exp.clone());
        cref_ty = Expression::typeOf(defaultExp);
        dim_diff = Type::dimensionDiff(cref_ty.clone(), exp_ty);
        if dim_diff > 0 {
            (exp, _) = Expression::liftArrayList(List::firstN(Type::arrayDims(cref_ty), dim_diff)?, exp)?;
        }
    }
    Ok(exp)
}

pub(crate) fn subscriptBinding(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut evalSubscripts: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    subs = ComponentRef::getSubscripts(cref);
    if evalSubscripts {
        subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut s in (subs).into_iter().cloned() {
                let __x = Subscript::eval(s.clone(), &(noTarget().clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    subs = List::trimToLength(subs, Expression::dimensionCount(exp.clone(), false)?)?;
    exp = Expression::applySubscripts(&subs, exp, false)?;
    (exp, _) = subscriptBinding2(exp, cref, evalSubscripts, None)?;
    Ok(exp)
}

pub(crate) fn subscriptBinding2(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut evalSubscripts: bool,
    mut subMap: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
            >,
        >,
    >,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
            >,
        >,
    >,
)> {
    pub(crate) type SubscriptList = metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;

    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut subMap: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<InstNode::InstNode>,
                metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
            >,
        >,
    > = subMap;
    let mut sub_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        >,
    >;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut cref_parts: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    (exp, subMap) = (match &*exp {
        Expression::SUBSCRIPTED_EXP {
            subscripts: __esc_subs,
            exp: __exp_exp,
            ..
        } => {
            subs = (*__esc_subs).clone();
            if (subMap).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(subMap.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                sub_map = metamodelica::Own::own(__pa0);
            } else {
                cref_parts = ComponentRef::toListReverse(cref, isFlatCref(cref), metamodelica::nil());
                sub_map = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| {
                        NFInstNode::InstNode::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static,
                        >),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<InstNode::InstNode>,
                              __a1: metamodelica::Ref<InstNode::InstNode>| {
                            NFInstNode::InstNode::refEqual(&__a0, &__a1)
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<InstNode::InstNode>,
                                    metamodelica::Ref<InstNode::InstNode>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    Util::nextPrime(((cref_parts).len() as i32)),
                );
                for mut cr in &*cref_parts {
                    UnorderedMap::addUnique(
                        ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?,
                        ComponentRef::getSubscripts(metamodelica::AsArg::as_arg(&cr)),
                        sub_map.clone(),
                    )?;
                }
                subMap = Some(sub_map.clone());
            }
            subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (subs.clone()).into_iter().cloned() {
                    let __x = subscriptBinding3(s.clone(), sub_map.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if evalSubscripts {
                subs = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                    for mut s in (subs.clone()).into_iter().cloned() {
                        let __x = Subscript::eval(s.clone(), &(noTarget().clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
            }
            (e, subMap) = subscriptBinding2(__exp_exp.clone(), cref, evalSubscripts, subMap)?;
            e = Expression::applySubscripts(metamodelica::AsArg::as_arg(&subs), e, false)?;
            (e, subMap)
        }
        Expression::ARRAY { literal: true, .. } => (exp, subMap),
        _ => Expression::mapFoldShallow(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = cref.clone();
                let __pe_b2 = evalSubscripts;
                move |__pe_a0, __pe_a3| subscriptBinding2(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_a3)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            Option<
                                metamodelica::Ref<
                                    UnorderedMap::UnorderedMap<
                                        metamodelica::Ref<InstNode::InstNode>,
                                        metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
                                    >,
                                >,
                            >,
                        ) -> Result<(
                            metamodelica::Ref<Expression::NFExpression>,
                            Option<
                                metamodelica::Ref<
                                    UnorderedMap::UnorderedMap<
                                        metamodelica::Ref<InstNode::InstNode>,
                                        metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
                                    >,
                                >,
                            >,
                        )> + 'static,
                >),
            subMap,
        )?,
    });
    Ok((exp, subMap))
}

pub(crate) fn isFlatCref<'__b>(mut cref: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>) -> bool {
    '__tco: loop {
        match &**cref {
            ComponentRef::CREF {
                origin: ComponentRef::Origin::SCOPE,
                ..
            } if (Type::isArray(var_field!((**cref).ty, ComponentRef::NFComponentRef::CREF))) => {
                return !((var_field!((**cref).subscripts, ComponentRef::NFComponentRef::CREF)).is_empty());
            }
            ComponentRef::CREF { .. } => {
                cref = var_field!((**cref).restCref, ComponentRef::NFComponentRef::CREF);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn subscriptBinding3(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut subMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        >,
    >,
) -> Result<metamodelica::Ref<Subscript::NFSubscript>> {
    let mut outSubscript: metamodelica::Ref<Subscript::NFSubscript>;
    let mut osubs: Option<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    outSubscript = (match &*subscript.clone() {
        Subscript::SPLIT_INDEX {
            dimIndex: __subscript_dimIndex,
            node: __subscript_node,
        } => {
            osubs = UnorderedMap::get(NFInstNode::InstNode::borrow(__subscript_node.clone())?, subMap)?;
            if (osubs).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(osubs) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                subs = metamodelica::Own::own(__pa0);
                if __subscript_dimIndex.clone() > ((subs).len() as i32) {
                    outSubscript = crate::NFSubscript::interned_WHOLE();
                } else {
                    outSubscript = (subs).get(__subscript_dimIndex.clone())?;
                }
            } else {
                outSubscript = subscript;
            }
            outSubscript
        }
        _ => subscript,
    });
    Ok(outSubscript)
}

pub(crate) fn evalComponentStartBinding(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut comp: &metamodelica::Ref<Component::NFComponent>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
    mut evalSubscripts: bool,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut outExp: Option<metamodelica::Ref<Expression::NFExpression>> = None;
    let mut var: Variability;
    let mut start_node: metamodelica::Ref<InstNode::InstNode>;
    let mut start_comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    var = Component::variability(comp)?;
    if var != Variability::PARAMETER.clone() && var != Variability::STRUCTURAL_PARAMETER.clone()
        || !(Component::isFixed(comp)?)
    {
        return Ok(outExp);
    }
    if let Ok(__iflet0) = Class::lookupElement(literal!("start"), NFInstNode::InstNode::getClass(node.clone())?) {
        start_node = __iflet0.0;
    } else {
        return Ok(outExp);
    }
    start_comp = NFInstNode::InstNode::component(&start_node)?;
    if !(Component::isTypeAttribute(&start_comp)) {
        return Ok(outExp);
    }
    binding = Component::getBinding(&start_comp);
    outExp = (match &*binding {
        Binding::TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            ..
        } => {
            exp = evalExp(__binding_bindingExp.clone(), target)?;
            if !(referenceEq(
                &*(&*exp),
                &*(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone()),
            )) {
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = exp.clone());
                start_comp = Component::setBinding(binding, start_comp)?;
                NFInstNode::InstNode::updateComponent(start_comp, start_node)?;
            }
            Some(exp)
        }
        _ => outExp,
    });
    Ok(outExp)
}

pub(crate) fn makeComponentBinding(
    mut component: metamodelica::Ref<Component::NFComponent>,
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> metamodelica::Ref<Binding::NFBinding> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = metamodelica::Ref::new(Binding::UNBOUND);
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut rec_node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    binding = 'mc: {
        let __mc_input = &*component;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                    exp = makeRecordFieldBindingFromParent(&cref, target)?;
                    Ok((if (Expression::isEmpty(&exp)) {Binding::EMPTY_BINDING().clone()} else {metamodelica::Ref::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() })}, exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Component::COMPONENT { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. }, .. } => {
                    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding.clone();
                    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                    exp = makeRecordBindingExp(var_field!((*component).classInst, Component::NFComponent::COMPONENT).clone(), NFInstNode::InstNode::borrow(rec_node.clone())?, var_field!((*component).ty, Component::NFComponent::COMPONENT).clone(), cref.clone(), target)?;
                    binding = metamodelica::Ref::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() });
                    if !(ComponentRef::hasSubscripts(&cref)?) {
                        NFInstNode::InstNode::updateComponent(Component::setBinding(binding.clone(), component.clone())?, node.clone())?;
                    }
                    Ok((binding.clone(), binding.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            binding = __wb0;
            exp = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Component::COMPONENT { ty: Deref @ Type::ARRAY { elementType: ty @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. }, .. }, .. } => {
                    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding.clone();
                    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                    exp = Expression::mapCrefScalars(Expression::fromCref(cref.clone(), false)?, (std::sync::Arc::new({ let __pe_b0 = var_field!((*component).classInst, Component::NFComponent::COMPONENT).clone(); let __pe_b1 = NFInstNode::InstNode::borrow(rec_node.clone())?; let __pe_b2 = ty.clone(); let __pe_b4 = target.clone(); move |__pe_a3| makeRecordBindingExp(__pe_b0.clone(), __pe_b1.clone(), __pe_b2.clone(), __pe_a3, &__pe_b4) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    binding = metamodelica::Ref::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() });
                    if !(ComponentRef::hasSubscripts(&cref)?) {
                        NFInstNode::InstNode::updateComponent(Component::setBinding(binding.clone(), component.clone())?, node.clone())?;
                    }
                    Ok((binding.clone(), binding.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            binding = __wb0;
            exp = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Binding::EMPTY_BINDING().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    binding
}

pub(crate) fn makeRecordFieldBindingFromParent(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut parent_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut exp_context: i32;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    parent_cr = ComponentRef::rest(cref)?;
    parent = ComponentRef::node(&parent_cr)?;
    exp_context = InstContext::nodeContext(&parent, target.context.clone())?;
    comp = NFInstNode::InstNode::component(&parent)?;
    binding = Component::getBinding(&comp);
    subs = ComponentRef::getSubscripts(&parent_cr);
    if Binding::hasExp(&binding) {
        if !(Binding::isTyped(&binding)) {
            binding = Typing::typeBinding(binding, InstContext::set(exp_context, InstContext::BINDING.clone()))?;
            comp = Component::setBinding(binding.clone(), comp)?;
            NFInstNode::InstNode::updateComponent(comp, parent)?;
        }
        exp = Binding::getExp(&binding)?;
        exp = Expression::applySubscripts(&subs, exp, false)?;
        exp = Expression::recordElement(&(ComponentRef::firstName(cref, false)?), &exp)?;
        exp = evalExp(exp, target)?;
        exp = Expression::map(
            exp,
            (std::sync::Arc::new({
                let __pe_b1 = ComponentRef::nodesIncludingSplitSubs(cref, metamodelica::nil())?;
                move |__pe_a0| Expression::expandNonListedSplitIndices(__pe_a0, &__pe_b1)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        exp = makeRecordFieldBindingFromParent(&parent_cr, target)?;
        exp = Expression::applySubscripts(&subs, exp, false)?;
        exp = Expression::recordElement(&(ComponentRef::firstName(cref, false)?), &exp)?;
    }
    Ok(exp)
}

pub(crate) fn makeRecordBindingExp(
    mut typeNode: metamodelica::Ref<InstNode::InstNode>,
    mut recordNode: metamodelica::Ref<InstNode::InstNode>,
    mut recordType: metamodelica::Ref<Type::NFType>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut c: metamodelica::Ref<InstNode::InstNode>;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    tree = Class::classTree(NFInstNode::InstNode::getClass(typeNode)?)?;
    comps = ClassTree::getComponents(&tree)?;
    args = metamodelica::nil();
    ErrorExt::setCheckpoint(literal!("NFCeval.makeRecordBindingExp"));
    for mut i in ({
        let __s = metamodelica::arrayLength(comps.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        c = ({
            let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
            __elt
        });
        ty = NFInstNode::InstNode::getType(c.clone())?;
        cr = ComponentRef::prefixCref(c.clone(), ty.clone(), metamodelica::nil(), cref.clone())?;
        arg = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: ty, cref: cr });
        if Component::variability(&(NFInstNode::InstNode::component(&c)?))? <= Variability::PARAMETER.clone() {
            if '__try0: {
                arg = unwrap_break_err!(evalExp(arg.clone(), target), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
        }
        args = metamodelica::cons(arg, args);
    }
    ErrorExt::rollBack(literal!("NFCeval.makeRecordBindingExp"));
    exp = Expression::makeRecord(NFInstNode::InstNode::fullPath(recordNode, false)?, recordType, args);
    Ok(exp)
}

pub(crate) fn evalTypename(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut originExp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = if (InstContext::inIterationRange(target.context.clone())) {
        ExpandExp::expandTypename(ty)?
    } else {
        originExp
    };
    Ok(exp)
}

pub(crate) fn evalRange(
    mut rangeExp: &metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stop_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut step_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*rangeExp)) {
        Deref @ Expression::RANGE { ty: __pa0, start: __pa1, step: __pa2, stop: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    start_exp = metamodelica::Own::own(__pa1);
    step_exp = metamodelica::Own::own(__pa2);
    stop_exp = metamodelica::Own::own(__pa3);
    start_exp = evalExp(start_exp, target)?;
    step_exp = Util::applyOption(
        step_exp,
        &({
            let __pe_b1 = target.clone();
            move |__pe_a0| evalExp(__pe_a0, &__pe_b1)
        }),
    )?;
    stop_exp = evalExp(stop_exp, target)?;
    if InstContext::inIterationRange(target.context.clone()) {
        ty = TypeCheck::getRangeType(
            start_exp.clone(),
            step_exp.clone(),
            stop_exp.clone(),
            Type::arrayElementType(&ty),
            &(EvalTarget::getInfo(target)),
        )?;
        result = metamodelica::Ref::new(Expression::NFExpression::RANGE {
            ty: ty,
            start: start_exp,
            step: step_exp,
            stop: stop_exp,
        });
    } else {
        result = metamodelica::Ref::new(Expression::NFExpression::RANGE {
            ty: ty,
            start: start_exp,
            step: step_exp,
            stop: stop_exp,
        });
        result = Expression::mapSplitExpressions(result, &evalRangeExp)?;
    }
    Ok(result)
}

pub(crate) fn evalRangeExp(
    mut rangeExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut start: metamodelica::Ref<Expression::NFExpression>;
    let mut step: metamodelica::Ref<Expression::NFExpression>;
    let mut stop: metamodelica::Ref<Expression::NFExpression>;
    let mut opt_step: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut literals: metamodelica::List<ArcStr>;
    let mut istep: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(SimplifyExp::simplify(Expression::map(rangeExp, (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?, false)?) {
        Deref @ Expression::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    start = metamodelica::Own::own(__pa0);
    opt_step = metamodelica::Own::own(__pa1);
    stop = metamodelica::Own::own(__pa2);
    if (opt_step).is_some() {
        let __pa3 = ::match_deref::match_deref! { match &(opt_step) {
            Some(__pa3) => __pa3.clone(),
            _ => return Err("pattern mismatch"),
        } };
        step = metamodelica::Own::own(__pa3);
        (ty, expl) = (::match_deref::match_deref! { match &((start.clone(), step.clone(), stop.clone())) {
            (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { value: __esc_istep }, Deref @ Expression::INTEGER { .. }) => {
                istep = (*__esc_istep).clone();
                expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut i in (({let __s=var_field!((*start).value, Expression::NFExpression::INTEGER).clone(); let __e=var_field!((*stop).value, Expression::NFExpression::INTEGER).clone(); let __step=istep.clone(); (0i32..).map(move |__k| __s + __k * __step).take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))})).into_iter() {
                let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                (crate::NFType::interned_INTEGER(), expl)
            },
            (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
                expl = evalRangeReal(var_field!((*start).value, Expression::NFExpression::REAL).clone(), var_field!((*step).value, Expression::NFExpression::REAL).clone(), var_field!((*stop).value, Expression::NFExpression::REAL).clone())?;
                (crate::NFType::interned_REAL(), expl)
            },
            _ => {
                printWrongArgsError(&(literal!("NFCeval.evalRangeExp")), list![start, step, stop], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    } else {
        (ty, expl) = (::match_deref::match_deref! { match &((start.clone(), stop.clone())) {
            (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
                expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut i in (var_field!((*start).value, Expression::NFExpression::INTEGER).clone()..=var_field!((*stop).value, Expression::NFExpression::INTEGER).clone()).into_iter() {
                let __x = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                (crate::NFType::interned_INTEGER(), expl)
            },
            (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
                expl = evalRangeReal(var_field!((*start).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat(1.0_f64), var_field!((*stop).value, Expression::NFExpression::REAL).clone())?;
                (crate::NFType::interned_REAL(), expl)
            },
            (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => {
                expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut b in (({let __bs = var_field!((*start).value, Expression::NFExpression::BOOLEAN).clone(); let __be = var_field!((*stop).value, Expression::NFExpression::BOOLEAN).clone(); if !__bs && !__be { vec![false] } else if !__bs && __be { vec![false, true] } else if __bs && __be { vec![true] } else { Vec::<bool>::new() }})).into_iter() {
                let __x = metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: b.clone() });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                (crate::NFType::interned_BOOLEAN(), expl)
            },
            (Deref @ Expression::ENUM_LITERAL { ty: __esc_ty @ Deref @ Type::ENUMERATION { .. }, .. }, Deref @ Expression::ENUM_LITERAL { .. }) => {
                ty = (*__esc_ty).clone();
                expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut i in (var_field!((*start).index, Expression::NFExpression::ENUM_LITERAL).clone()..=var_field!((*stop).index, Expression::NFExpression::ENUM_LITERAL).clone()).into_iter() {
                let __x = metamodelica::Ref::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: (var_field!((*ty).literals, Type::NFType::ENUMERATION)).get(i.clone())?, index: i.clone() });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                (ty.clone(), expl)
            },
            _ => {
                printWrongArgsError(&(literal!("NFCeval.evalRangeExp")), list![start, stop], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    exp = Expression::makeArray(
        metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: ty,
            dimensions: list![Dimension::fromInteger(
                ((expl).len() as i32),
                Prefixes::Variability::CONSTANT.clone()
            )],
        }),
        metamodelica::arrayFromVec(expl.into_iter().cloned().collect()),
        true,
    );
    Ok(exp)
}

pub(crate) fn evalRangeReal(
    mut start: metamodelica::Real,
    mut step: metamodelica::Real,
    mut stop: metamodelica::Real,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut steps: i32;
    steps = Util::realRangeSize(start, step, stop)?;
    if steps == 0 {
        result = metamodelica::nil();
    } else if steps == 1 {
        result = list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: start })];
    } else {
        result = list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: stop })];
        for mut i in ({
            let __s = steps - 2;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            result = metamodelica::cons(
                metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: start + metamodelica::OrderedFloat((i) as f64) * step,
                }),
                result,
            );
        }
        result = metamodelica::cons(
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: start }),
            result,
        );
    }
    Ok(result)
}

pub(crate) fn printFailedEvalError(
    mut name: &ArcStr,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut info: SourceInfo,
) -> Result<()> {
    Error::addInternalError(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" failed to evaluate ‘"));
            __mm_s.push_str(&*Expression::toString(exp)?);
            __mm_s.push_str(&*literal!("‘"));
            ArcStr::from(__mm_s)
        },
        info,
    )?;
    Ok(())
}

pub(crate) fn evalBinaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = Expression::mapSplitExpressions(
        metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: exp1,
            operator: op,
            exp2: exp2,
        }),
        &({
            let __pe_b1 = target.clone();
            move |__pe_a0| evalBinaryExp(&__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(exp)
}

pub(crate) fn evalBinaryExp(
    mut binaryExp: &metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*binaryExp)) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    result = evalBinaryOp_dispatch(e1, op, e2, target)?;
    Ok(result)
}

pub(crate) fn evalBinaryOp_dispatch(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::ADD => evalBinaryAdd(exp1, exp2)?,
        Operator::Op::SUB => evalBinarySub(exp1, &exp2)?,
        Operator::Op::MUL => evalBinaryMul(exp1, exp2)?,
        Operator::Op::DIV => evalBinaryDiv(exp1, &exp2, target)?,
        Operator::Op::POW => evalBinaryPow(exp1, exp2, target)?,
        Operator::Op::ADD_EW => evalBinaryAdd(exp1, exp2)?,
        Operator::Op::SUB_EW => evalBinarySub(exp1, &exp2)?,
        Operator::Op::MUL_EW => evalBinaryMul(exp1, exp2)?,
        Operator::Op::ADD_SCALAR_ARRAY => evalBinaryScalarArray(
            exp1,
            exp2,
            (std::sync::Arc::new(evalBinaryAdd)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::ADD_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(
            exp1,
            exp2,
            (std::sync::Arc::new(evalBinaryAdd)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::SUB_SCALAR_ARRAY { .. } => evalBinaryScalarArray(
            exp1,
            exp2,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Expression::NFExpression>,
                      __a1: metamodelica::Ref<Expression::NFExpression>| evalBinarySub(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::SUB_ARRAY_SCALAR => evalBinaryArrayScalar(
            exp1,
            exp2,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Expression::NFExpression>,
                      __a1: metamodelica::Ref<Expression::NFExpression>| evalBinarySub(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::MUL_SCALAR_ARRAY => evalBinaryScalarArray(
            exp1,
            exp2,
            (std::sync::Arc::new(evalBinaryMul)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::MUL_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(
            exp1,
            exp2,
            (std::sync::Arc::new(evalBinaryMul)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::MUL_VECTOR_MATRIX => evalBinaryMulVectorMatrix(exp1, exp2)?,
        Operator::Op::MUL_MATRIX_VECTOR => evalBinaryMulMatrixVector(exp1, exp2)?,
        Operator::Op::SCALAR_PRODUCT => evalBinaryScalarProduct(exp1, exp2)?,
        Operator::Op::MATRIX_PRODUCT => evalBinaryMatrixProduct(exp1, exp2)?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => evalBinaryScalarArray(
            exp1,
            exp2,
            (std::sync::Arc::new({
                let __pe_b2 = target.clone();
                move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, &__pe_a1, &__pe_b2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::DIV_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(
            exp1,
            exp2,
            (std::sync::Arc::new({
                let __pe_b2 = target.clone();
                move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, &__pe_a1, &__pe_b2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::POW_SCALAR_ARRAY { .. } => evalBinaryScalarArray(
            exp1,
            exp2,
            (std::sync::Arc::new({
                let __pe_b2 = target.clone();
                move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, &__pe_b2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::POW_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(
            exp1,
            exp2,
            (std::sync::Arc::new({
                let __pe_b2 = target.clone();
                move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, &__pe_b2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        Operator::Op::POW_MATRIX => evalBinaryPowMatrix(exp1, exp2)?,
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalBinaryOp_dispatch"));
                    __mm_s.push_str(&*literal!(": unimplemented case for "));
                    __mm_s.push_str(&*Expression::toString(metamodelica::Ref::new(
                        Expression::NFExpression::BINARY {
                            exp1: exp1,
                            operator: op,
                            exp2: exp2,
                        },
                    ))?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalBinaryAdd(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((&*exp1, &*exp2)) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() + metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) + var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() + var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() + var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => metamodelica::Ref::new(Expression::NFExpression::STRING { value: { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).value, Expression::NFExpression::STRING)); __mm_s.push_str(&*var_field!((*exp2).value, Expression::NFExpression::STRING)); ArcStr::from(__mm_s) } }),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => metamodelica::Ref::new(Expression::NFExpression::STRING { value: { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).value, Expression::NFExpression::STRING)); __mm_s.push_str(&*var_field!((*exp2).filename, Expression::NFExpression::FILENAME)); ArcStr::from(__mm_s) } }),
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => metamodelica::Ref::new(Expression::NFExpression::STRING { value: { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).filename, Expression::NFExpression::FILENAME)); __mm_s.push_str(&*var_field!((*exp2).value, Expression::NFExpression::STRING)); ArcStr::from(__mm_s) } }),
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => metamodelica::Ref::new(Expression::NFExpression::STRING { value: { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).filename, Expression::NFExpression::FILENAME)); __mm_s.push_str(&*var_field!((*exp2).filename, Expression::NFExpression::FILENAME)); ArcStr::from(__mm_s) } }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &evalBinaryAdd)?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinaryAdd(__pe_a0, __pe_b1.clone()) }))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinaryAdd(__pe_b0.clone(), __pe_a1) }))?, var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeAdd(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryAdd")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinarySub<'__b>(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: &'__b metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((&*exp1, &**exp2)) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() - metamodelica::OrderedFloat((var_field!((**exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) - var_field!((**exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() - var_field!((**exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() - var_field!((**exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone(), &move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: metamodelica::Ref<Expression::NFExpression>| evalBinarySub(__a0, &__a1))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinarySub(__pe_a0, &__pe_b1) }))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((**exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinarySub(__pe_b0.clone(), &__pe_a1) }))?, var_field!((**exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => evalBinarySub(Expression::makeZero(&(Expression::typeOf(exp2.clone())))?, exp2)?,
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeSub(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(&(literal!("NFCeval.evalBinarySub")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn expandLiteralRange(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::RANGE { .. } if (Expression::isLiteral(&exp)?) => {
            Expression::mapSplitExpressions(exp.clone(), &evalRangeExp)?
        }
        _ => exp.clone(),
    });
    Ok(exp)
}

pub(crate) fn evalMultaryAddSub(
    mut arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut inv_arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut operator_ty: metamodelica::Ref<Type::NFType>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: operator_ty.clone(),
        });
    let mut isNeutral: bool;
    for mut arg in &**arguments {
        exp = evalBinaryAdd(exp, expandLiteralRange(arg.clone())?)?;
    }
    for mut arg in &**inv_arguments {
        exp = evalBinarySub(exp, &(expandLiteralRange(arg.clone())?))?;
    }
    isNeutral = Expression::isEmpty(&exp) || Expression::isZero(&exp)?;
    Ok((exp, isNeutral))
}

pub(crate) fn evalBinaryMul(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((&*exp1, &*exp2)) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() * metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) * var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() * var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() * var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &evalBinaryMul)?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinaryMul(__pe_a0, __pe_b1.clone()) }))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinaryMul(__pe_b0.clone(), __pe_a1) }))?, var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryMul")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryDiv<'__b>(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: &'__b metamodelica::Ref<Expression::NFExpression>,
    mut target: &'__b metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((&*exp1, &**exp2)) {
        (_, _) if (Expression::isZero(exp2)?) => {
            if EvalTarget::hasInfo(target) {
                Error::addSourceMessage(&(Error::DIVISION_BY_ZERO.clone()), list![Expression::toString(exp1.clone())?, Expression::toString(exp2.clone())?], &(EvalTarget::getInfo(target)))?;
                return Err("fail");
            } else {
                exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeDiv(crate::NFType::interned_REAL()), exp2: exp2.clone() });
            }
            exp
        },
        (_, Deref @ Expression::INTEGER { value: 1 }) => exp1.clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::real_div_checked(var_field!((*exp1).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat((var_field!((**exp2).value, Expression::NFExpression::INTEGER).clone()) as f64))? }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::real_div_checked(metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64), var_field!((**exp2).value, Expression::NFExpression::REAL).clone())? }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (intMod(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone(), var_field!((**exp2).value, Expression::NFExpression::INTEGER).clone()) == 0) {metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intDiv(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone(), var_field!((**exp2).value, Expression::NFExpression::INTEGER).clone()) })} else {metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::real_div_checked(metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64), metamodelica::OrderedFloat((var_field!((**exp2).value, Expression::NFExpression::INTEGER).clone()) as f64))? })},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::real_div_checked(var_field!((*exp1).value, Expression::NFExpression::REAL).clone(), var_field!((**exp2).value, Expression::NFExpression::REAL).clone())? }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, &__pe_a1, &__pe_b2) }))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = exp2.clone(); let __pe_b2 = target.clone(); move |__pe_a0| evalBinaryDiv(__pe_a0, &__pe_b1, &__pe_b2) }))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((**exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((**exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b0 = exp1.clone(); let __pe_b2 = target.clone(); move |__pe_a1| evalBinaryDiv(__pe_b0.clone(), &__pe_a1, &__pe_b2) }))?, var_field!((**exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => evalBinaryDiv(Expression::makeOne(&(Expression::typeOf(exp2.clone())))?, exp2, target)?,
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeDiv(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryDiv")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalMultaryMulDiv(
    mut arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut inv_arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut operator_ty: metamodelica::Ref<Type::NFType>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: operator_ty.clone(),
        });
    let mut isNeutral: bool;
    for mut arg in &**arguments {
        exp = evalBinaryMul(exp, expandLiteralRange(arg.clone())?)?;
    }
    for mut arg in &**inv_arguments {
        exp = evalBinaryDiv(exp, &(expandLiteralRange(arg.clone())?), &(noTarget().clone()))?;
    }
    isNeutral = Expression::isEmpty(&exp) || Expression::isOne(&exp)?;
    Ok((exp, isNeutral))
}

pub(crate) fn evalBinaryPow(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((&*exp1, &*exp2)) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64) && metamodelica::OrderedFloat((((var_field!((*exp2).value, Expression::NFExpression::REAL).clone()).0.floor() as i32)) as f64) != var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) => {
            if EvalTarget::hasInfo(target) {
                Error::addSourceMessage(&(Error::INVALID_NEGATIVE_POW.clone()), list![Expression::toString(exp1.clone())?, Expression::toString(exp2.clone())?], &(EvalTarget::getInfo(target)))?;
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makePow(crate::NFType::interned_REAL()), exp2: exp2.clone() })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => metamodelica::Ref::new(Expression::NFExpression::REAL { value: (var_field!((*exp1).value, Expression::NFExpression::REAL).clone()).powf(var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, &__pe_b2) }))?, true),
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makePow(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryPow")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryScalarArray(
    mut scalarExp: metamodelica::Ref<Expression::NFExpression>,
    mut arrayExp: metamodelica::Ref<Expression::NFExpression>,
    mut opFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    pub type FuncT = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &*arrayExp {
        Expression::ARRAY { ty: __arrayExp_ty, .. } => Expression::makeArray(
            __arrayExp_ty.clone(),
            Array::map(
                var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(),
                &({
                    let __pe_b0 = scalarExp;
                    let __pe_b2: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = opFunc.clone();
                    move |__pe_a1| evalBinaryScalarArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone())
                }),
            )?,
            true,
        ),
        _ => opFunc(scalarExp, arrayExp)?,
    });
    Ok(exp)
}

pub(crate) fn evalBinaryArrayScalar(
    mut arrayExp: metamodelica::Ref<Expression::NFExpression>,
    mut scalarExp: metamodelica::Ref<Expression::NFExpression>,
    mut opFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    pub type FuncT = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &*arrayExp {
        Expression::ARRAY { ty: __arrayExp_ty, .. } => Expression::makeArray(
            __arrayExp_ty.clone(),
            Array::map(
                var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(),
                &({
                    let __pe_b1 = scalarExp;
                    let __pe_b2: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = opFunc.clone();
                    move |__pe_a0| evalBinaryArrayScalar(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?,
            true,
        ),
        _ => opFunc(arrayExp, scalarExp)?,
    });
    Ok(exp)
}

pub(crate) fn evalBinaryMulVectorMatrix(
    mut vectorExp: metamodelica::Ref<Expression::NFExpression>,
    mut matrixExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut m: metamodelica::Ref<Dimension::NFDimension>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    exp = (::match_deref::match_deref! { match &(Expression::transposeArray(&matrixExp)?) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_ty, dimensions: Deref @ metamodelica::ListNode::Cons { head: __esc_m, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __esc_arr, .. } => {
            ty = (*__esc_ty).clone();
            m = (*__esc_m).clone();
            arr = (*__esc_arr).clone();
            arr = Array::map(arr.clone(), &({ let __pe_b0 = vectorExp; move |__pe_a1| evalBinaryScalarProduct(__pe_b0.clone(), __pe_a1) }))?;
            Expression::makeArray(metamodelica::Ref::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![m.clone()] }), arr.clone(), true)
        },
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: vectorExp, operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: matrixExp });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryMulVectorMatrix")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryMulMatrixVector(
    mut matrixExp: metamodelica::Ref<Expression::NFExpression>,
    mut vectorExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Dimension::NFDimension>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    exp = (::match_deref::match_deref! { match &(matrixExp.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_ty, dimensions: Deref @ metamodelica::ListNode::Cons { head: __esc_n, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __esc_arr, .. } => {
            ty = (*__esc_ty).clone();
            n = (*__esc_n).clone();
            arr = (*__esc_arr).clone();
            arr = Array::map(arr.clone(), &({ let __pe_b1 = vectorExp; move |__pe_a0| evalBinaryScalarProduct(__pe_a0, __pe_b1.clone()) }))?;
            Expression::makeArray(metamodelica::Ref::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![n.clone()] }), arr.clone(), true)
        },
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: matrixExp, operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: vectorExp });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryMulMatrixVector")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryScalarProduct(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((&*exp1, &*exp2)) {
        (Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: elem_ty, .. }, .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => {
            exp = Expression::makeZero(metamodelica::AsArg::as_arg(&elem_ty))?;
            for mut i in 1..=metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) {
                exp = evalBinaryAdd(exp, evalBinaryMul(metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), i), metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), i))?)?;
            }
            exp
        },
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryScalarProduct")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryMatrixProduct(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut mat_ty: metamodelica::Ref<Type::NFType>;
    let mut n: metamodelica::Ref<Dimension::NFDimension>;
    let mut p: metamodelica::Ref<Dimension::NFDimension>;
    let mut arr1: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr2: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    e2 = Expression::transposeArray(&exp2)?;
    exp = (::match_deref::match_deref! { match &((exp1.clone(), e2)) {
        (Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_elem_ty, dimensions: Deref @ metamodelica::ListNode::Cons { head: __esc_n, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __esc_arr1, .. }, Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: _, dimensions: Deref @ metamodelica::ListNode::Cons { head: __esc_p, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, elements: __esc_arr2, .. }) => {
            elem_ty = (*__esc_elem_ty).clone();
            n = (*__esc_n).clone();
            arr1 = (*__esc_arr1).clone();
            p = (*__esc_p).clone();
            arr2 = (*__esc_arr2).clone();
            mat_ty = metamodelica::Ref::new(Type::NFType::ARRAY { elementType: elem_ty.clone(), dimensions: list![n.clone(), p.clone()] });
            if arr2.clone().borrow().is_empty() {
                exp = Expression::makeZero(&mat_ty)?;
            } else {
                row_ty = metamodelica::Ref::new(Type::NFType::ARRAY { elementType: elem_ty.clone(), dimensions: list![p.clone()] });
                arr = metamodelica::arrayCreate(metamodelica::arrayLength(arr1.clone()), exp1);
                for mut i in 1..=metamodelica::arrayLength(arr1.clone()) {
                    unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i, Expression::makeArray(row_ty.clone(), Array::map(arr2.clone(), &({ let __pe_b0 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i); move |__pe_a1| evalBinaryScalarProduct(__pe_b0.clone(), __pe_a1) }))?, true)) };
                }
                exp = Expression::makeArray(mat_ty, arr.clone(), true);
            }
            exp
        },
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp1, operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: exp2 });
            printFailedEvalError(&(literal!("NFCeval.evalBinaryMatrixProduct")), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryPowMatrix(
    mut matrixExp: metamodelica::Ref<Expression::NFExpression>,
    mut nExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut n: i32;
    exp = (match &*nExp {
        Expression::INTEGER { value: 0 } => {
            n = Dimension::size(
                &((Type::arrayDims(Expression::typeOf(matrixExp))).head().cloned()?),
                false,
            )?;
            Expression::makeIdentityMatrix(n, crate::NFType::interned_REAL())?
        }
        Expression::INTEGER { value: __esc_n } => {
            n = (*__esc_n).clone();
            evalBinaryPowMatrix2(&matrixExp, n.clone())?
        }
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: matrixExp,
                operator: Operator::makePow(crate::NFType::interned_UNKNOWN()),
                exp2: nExp,
            });
            printFailedEvalError(
                &(literal!("NFCeval.evalBinaryPowMatrix")),
                exp,
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalBinaryPowMatrix2(
    mut matrix: &metamodelica::Ref<Expression::NFExpression>,
    mut n: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match n {
        1 => matrix.clone(),
        2 => evalBinaryMatrixProduct(matrix.clone(), matrix.clone())?,
        _ if (intMod(n, 2) == 0) => {
            exp = evalBinaryPowMatrix2(matrix, intDiv(n, 2))?;
            evalBinaryMatrixProduct(exp.clone(), exp)?
        }
        _ => {
            exp = evalBinaryPowMatrix2(matrix, n - 1)?;
            evalBinaryMatrixProduct(matrix.clone(), exp)?
        }
    });
    Ok(exp)
}

pub(crate) fn evalUnaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::UMINUS if (Expression::isZero(&exp1)?) => exp1.clone(),
        Operator::Op::UMINUS => Expression::mapSplitExpressions(exp1.clone(), &evalUnaryMinus)?,
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalUnaryOp"));
                    __mm_s.push_str(&*literal!(": unimplemented case for "));
                    __mm_s.push_str(&*Expression::toString(metamodelica::Ref::new(
                        Expression::NFExpression::UNARY {
                            operator: op,
                            exp: exp1.clone(),
                        },
                    ))?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalUnaryMinus(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &*exp1 {
        Expression::INTEGER { value: __exp1_value } => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
            value: -(__exp1_value.clone()),
        }),
        Expression::REAL { value: __exp1_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: -(__exp1_value.clone()),
        }),
        Expression::ARRAY { .. } => {
            assign_variant_field!(exp1 => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), &evalUnaryMinus)?);
            exp1
        }
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::UNARY {
                operator: Operator::makeUMinus(crate::NFType::interned_UNKNOWN()),
                exp: exp1,
            });
            printFailedEvalError(
                &(literal!("NFCeval.evalUnaryMinus")),
                exp,
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalLogicBinaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = Expression::mapSplitExpressions(
        metamodelica::Ref::new(Expression::NFExpression::LBINARY {
            exp1: exp1,
            operator: op,
            exp2: exp2,
        }),
        &({
            let __pe_b1 = target.clone();
            move |__pe_a0| evalLogicBinaryExp(&__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(exp)
}

pub(crate) fn evalLogicBinaryExp(
    mut binaryExp: &metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*binaryExp)) {
        Deref @ Expression::LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    result = evalLogicBinaryOp_dispatch(e1, op, e2, target)?;
    Ok(result)
}

pub(crate) fn evalLogicBinaryOp_dispatch(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::AND => evalLogicBinaryAnd(evalExp(exp1, target)?, exp2, target)?,
        Operator::Op::OR => evalLogicBinaryOr(evalExp(exp1, target)?, exp2, target)?,
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalLogicBinaryOp_dispatch"));
                    __mm_s.push_str(&*literal!(": unimplemented case for "));
                    __mm_s.push_str(&*Expression::toString(metamodelica::Ref::new(
                        Expression::NFExpression::LBINARY {
                            exp1: exp1,
                            operator: op,
                            exp2: exp2,
                        },
                    ))?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalLogicBinaryAnd(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    exp = 'mc: {
        let __mc_input = &*exp1;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BOOLEAN { .. } => {
                    Ok(if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) {evalExp(exp2.clone(), target)?} else {exp1.clone()})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::ARRAY { .. } => {
                    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
                    let __pa0 = ::match_deref::match_deref! { match &(evalExp(exp2.clone(), target)?) {
                        Deref @ Expression::ARRAY { elements: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    arr = metamodelica::Own::own(__pa0);
                    arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), arr.clone(), &({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalLogicBinaryAnd(__pe_a0, __pe_a1, &__pe_b2) }))?;
                    Ok(Expression::makeArray(Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY), &(crate::NFType::interned_BOOLEAN())), arr.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp.clone();
                    exp = metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: Operator::makeAnd(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
                    printFailedEvalError(&(literal!("NFCeval.evalLogicBinaryAnd")), exp.clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
                    Ok((return Err("fail"), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            exp = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(exp)
}

pub(crate) fn evalLogicBinaryOr(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &*exp1 {
        Expression::BOOLEAN { value: __exp1_value } => {
            if (__exp1_value.clone()) {
                exp1
            } else {
                evalExp(exp2, target)?
            }
        }
        Expression::ARRAY { ty: __exp1_ty, .. } => {
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(evalExp(exp2, target)?) {
                Deref @ Expression::ARRAY { elements: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            arr = metamodelica::Own::own(__pa0);
            arr = Array::threadMap(
                var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(),
                arr.clone(),
                &({
                    let __pe_b2 = target.clone();
                    move |__pe_a0, __pe_a1| evalLogicBinaryOr(__pe_a0, __pe_a1, &__pe_b2)
                }),
            )?;
            Expression::makeArray(
                Type::setArrayElementType(
                    metamodelica::AsArg::as_arg(&__exp1_ty),
                    &(crate::NFType::interned_BOOLEAN()),
                ),
                arr.clone(),
                true,
            )
        }
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
                exp1: exp1,
                operator: Operator::makeOr(crate::NFType::interned_UNKNOWN()),
                exp2: exp2,
            });
            printFailedEvalError(
                &(literal!("NFCeval.evalLogicBinaryOr")),
                exp,
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalLogicUnaryOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::NOT => Expression::mapSplitExpressions(exp1, &evalLogicUnaryNot)?,
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalLogicUnaryOp"));
                    __mm_s.push_str(&*literal!(": unimplemented case for "));
                    __mm_s.push_str(&*Expression::toString(metamodelica::Ref::new(
                        Expression::NFExpression::LUNARY {
                            operator: op,
                            exp: exp1,
                        },
                    ))?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalLogicUnaryNot(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &*exp1 {
        Expression::BOOLEAN { value: __exp1_value } => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN {
            value: !(__exp1_value.clone()),
        }),
        Expression::ARRAY { .. } => Expression::mapArrayElements(
            exp1,
            (std::sync::Arc::new(evalLogicUnaryNot)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?,
        _ => {
            exp = metamodelica::Ref::new(Expression::NFExpression::LUNARY {
                operator: Operator::makeNot(crate::NFType::interned_UNKNOWN()),
                exp: exp1,
            });
            printFailedEvalError(
                &(literal!("NFCeval.evalLogicUnaryNot")),
                exp,
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalRelationOp(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = Expression::mapSplitExpressions(
        metamodelica::Ref::new(Expression::NFExpression::RELATION {
            exp1: exp1,
            operator: op,
            exp2: exp2,
            index: -1,
        }),
        &move |__a0: metamodelica::Ref<Expression::NFExpression>| evalRelationExp(&__a0),
    )?;
    Ok(exp)
}

pub(crate) fn evalRelationExp(
    mut relationExp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*relationExp)) {
        Deref @ Expression::RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    result = evalRelationOp_dispatch(e1, op, e2)?;
    Ok(result)
}

pub(crate) fn evalRelationOp_dispatch(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut res: bool;
    res = (match op.op.clone() {
        Operator::Op::LESS => evalRelationLess(exp1, exp2)?,
        Operator::Op::LESSEQ => evalRelationLessEq(exp1, exp2)?,
        Operator::Op::GREATER => evalRelationGreater(exp1, exp2)?,
        Operator::Op::GREATEREQ => evalRelationGreaterEq(exp1, exp2)?,
        Operator::Op::EQUAL => evalRelationEqual(exp1, exp2)?,
        Operator::Op::NEQUAL => evalRelationNotEqual(exp1, exp2)?,
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalRelationOp_dispatch"));
                    __mm_s.push_str(&*literal!(": unimplemented case for "));
                    __mm_s.push_str(&*Expression::toString(metamodelica::Ref::new(
                        Expression::NFExpression::RELATION {
                            exp1: exp1,
                            operator: op,
                            exp2: exp2,
                            index: -1,
                        },
                    ))?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    exp = metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: res });
    Ok(exp)
}

pub(crate) fn evalRelationLess(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() < var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).value, Expression::NFExpression::STRING)) < 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) < 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).value, Expression::NFExpression::STRING)) < 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) < 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() < var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(&(literal!("NFCeval.evalRelationLess")), metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeLess(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationLessEq(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() <= var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() <= var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() <= var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).value, Expression::NFExpression::STRING)) <= 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) <= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).value, Expression::NFExpression::STRING)) <= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) <= 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() <= var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(&(literal!("NFCeval.evalRelationLessEq")), metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeLessEq(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationGreater(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() > var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() > var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() > var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).value, Expression::NFExpression::STRING)) > 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) > 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).value, Expression::NFExpression::STRING)) > 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) > 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() > var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(&(literal!("NFCeval.evalRelationGreater")), metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeGreater(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationGreaterEq(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() >= var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() >= var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() >= var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).value, Expression::NFExpression::STRING)) >= 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) >= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).value, Expression::NFExpression::STRING)) >= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) >= 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() >= var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(&(literal!("NFCeval.evalRelationGreaterEq")), metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeGreaterEq(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationEqual(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() == var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() == var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() == var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).value, Expression::NFExpression::STRING)) == 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) == 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).value, Expression::NFExpression::STRING)) == 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) == 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() == var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(&(literal!("NFCeval.evalRelationEqual")), metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeEqual(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationNotEqual(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() != var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() != var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() != var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).value, Expression::NFExpression::STRING)) != 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).value, Expression::NFExpression::STRING), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) != 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).value, Expression::NFExpression::STRING)) != 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare(&var_field!((*exp1).filename, Expression::NFExpression::FILENAME), &var_field!((*exp2).filename, Expression::NFExpression::FILENAME)) != 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() != var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(&(literal!("NFCeval.evalRelationNotEqual")), metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeNotEqual(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalIfExp(
    mut ifExp: &metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut btrue: metamodelica::Ref<Expression::NFExpression>;
    let mut bfalse: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*ifExp)) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    btrue = metamodelica::Own::own(__pa2);
    bfalse = metamodelica::Own::own(__pa3);
    result = metamodelica::Ref::new(Expression::NFExpression::IF {
        ty: ty,
        condition: evalExp(cond, target)?,
        trueBranch: btrue,
        falseBranch: bfalse,
    });
    result = Expression::mapSplitExpressions(
        result,
        &({
            let __pe_b1 = target.clone();
            move |__pe_a0| evalIfExp2(__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(result)
}

pub(crate) fn evalIfExp2(
    mut ifExp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut tb: metamodelica::Ref<Expression::NFExpression>;
    let mut fb: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp.clone()) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    tb = metamodelica::Own::own(__pa2);
    fb = metamodelica::Own::own(__pa3);
    result = (match &*cond {
        Expression::BOOLEAN { value: __cond_value } => {
            if Type::isConditionalArray(&ty) && !(Type::isMatchedBranch(__cond_value.clone(), &ty)?) {
                (tb, fb) = Util::swap(__cond_value.clone(), fb, tb);
                Error::addSourceMessage(
                    &(Error::ARRAY_DIMENSION_MISMATCH.clone()),
                    list![
                        Expression::toString(tb.clone())?,
                        Type::toString(&(Expression::typeOf(tb.clone())))?,
                        Dimension::toStringList(Type::arrayDims(Expression::typeOf(fb.clone())), false)?
                    ],
                    &(EvalTarget::getInfo(target)),
                )?;
                return Err("fail");
            }
            evalExp(if (__cond_value.clone()) { tb } else { fb }, target)?
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalIfExp2"));
                    __mm_s.push_str(&*literal!(": unimplemented case for "));
                    __mm_s.push_str(&*Expression::toString(ifExp)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalCast(
    mut castExp: metamodelica::Ref<Expression::NFExpression>,
    mut castTy: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = Expression::typeCast(castExp.clone(), castTy.clone())?;
    let () = (match &*exp {
        Expression::CAST { .. } => {
            exp = metamodelica::Ref::new(Expression::NFExpression::CAST {
                ty: castTy,
                exp: castExp,
            });
            printFailedEvalError(
                &(literal!("NFCeval.evalCast")),
                exp.clone(),
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
        _ => (),
    });
    Ok(exp)
}

pub(crate) fn evalCall(
    mut call: metamodelica::Ref<Call::NFCall>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut c: metamodelica::Ref<Call::NFCall> = call;
    exp = (match &*c {
        Call::TYPED_CALL {
            arguments: __c_arguments,
            ..
        } => {
            assign_variant_field!(c => Call::NFCall::TYPED_CALL; arguments = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut arg in (__c_arguments.clone()).into_iter().cloned() {
                    let __x = evalExp(arg.clone(), target)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            if (Function::isBuiltin(var_field!((*c).r#fn, Call::NFCall::TYPED_CALL))) {
                Expression::mapSplitExpressions(
                    metamodelica::Ref::new(Expression::NFExpression::CALL { call: c }),
                    &({
                        let __pe_b1 = target.clone();
                        move |__pe_a0| evalBuiltinCallExp(&__pe_a0, &__pe_b1)
                    }),
                )?
            } else {
                Expression::mapSplitExpressions(
                    metamodelica::Ref::new(Expression::NFExpression::CALL { call: c }),
                    &({
                        let __pe_b1 = target.clone();
                        move |__pe_a0| evalNormalCallExp(&__pe_a0, &__pe_b1)
                    }),
                )?
            }
        }
        Call::TYPED_ARRAY_CONSTRUCTOR { exp: __c_exp, .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR;
                exp = evalExpPartial(__c_exp.clone(), &(noTarget().clone()), true)?.0,
                iters = Call::mapIteratorsExpShallow(var_field!((*c).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR), &evalExpPartialDefault)?
            );
            Expression::mapSplitExpressions(
                metamodelica::Ref::new(Expression::NFExpression::CALL { call: c }),
                &move |__a0: metamodelica::Ref<Expression::NFExpression>| evalArrayConstructor(&__a0),
            )?
        }
        Call::TYPED_REDUCTION { exp: __c_exp, .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_REDUCTION;
                exp = evalExpPartial(__c_exp.clone(), &(noTarget().clone()), true)?.0,
                iters = Call::mapIteratorsExpShallow(var_field!((*c).iters, Call::NFCall::TYPED_REDUCTION), &evalExpPartialDefault)?
            );
            Expression::mapSplitExpressions(
                metamodelica::Ref::new(Expression::NFExpression::CALL { call: c }),
                &move |__a0: metamodelica::Ref<Expression::NFExpression>| evalReduction(&__a0),
            )?
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFCeval.evalCall"));
                    __mm_s.push_str(&*literal!(" got untyped call"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(exp)
}

pub(crate) fn evalBuiltinCallExp(
    mut callExp: &metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*callExp)) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: __pa0, arguments: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    result = evalBuiltinCall(&r#fn, args, target)?;
    Ok(result)
}

pub(crate) fn evalBuiltinCall(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_path: metamodelica::Ref<Absyn::Path> = Function::nameConsiderBuiltin(r#fn);
    result = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&fn_path)) {
        Deref @ "abs" => evalBuiltinAbs((args).head().cloned()?)?,
        Deref @ "acos" => evalBuiltinAcos((args).head().cloned()?, target)?,
        Deref @ "array" => evalBuiltinArray(args)?,
        Deref @ "asin" => evalBuiltinAsin((args).head().cloned()?, target)?,
        Deref @ "atan2" => evalBuiltinAtan2(args)?,
        Deref @ "atan" => evalBuiltinAtan((args).head().cloned()?)?,
        Deref @ "cat" => evalBuiltinCat(&((args).head().cloned()?), (args).rest()?, target)?,
        Deref @ "ceil" => evalBuiltinCeil((args).head().cloned()?)?,
        Deref @ "cosh" => evalBuiltinCosh((args).head().cloned()?)?,
        Deref @ "cos" => evalBuiltinCos((args).head().cloned()?)?,
        Deref @ "der" => evalBuiltinDer((args).head().cloned()?)?,
        Deref @ "diagonal" => evalBuiltinDiagonal(Expression::unbox((args).head().cloned()?))?,
        Deref @ "div" => evalBuiltinDiv(args, target)?,
        Deref @ "exp" => evalBuiltinExp((args).head().cloned()?)?,
        Deref @ "fill" => evalBuiltinFill(args)?,
        Deref @ "floor" => evalBuiltinFloor((args).head().cloned()?)?,
        Deref @ "identity" => evalBuiltinIdentity((args).head().cloned()?)?,
        Deref @ "integer" => evalBuiltinInteger((args).head().cloned()?)?,
        Deref @ "Integer" => evalBuiltinIntegerEnum((args).head().cloned()?)?,
        Deref @ "log10" => evalBuiltinLog10((args).head().cloned()?, target)?,
        Deref @ "log" => evalBuiltinLog((args).head().cloned()?, target)?,
        Deref @ "matrix" => evalBuiltinMatrix((args).head().cloned()?)?,
        Deref @ "max" => evalBuiltinMax(args, r#fn)?,
        Deref @ "min" => evalBuiltinMin(args, r#fn)?,
        Deref @ "mod" => evalBuiltinMod(args, target)?,
        Deref @ "noEvent" => (args).head().cloned()?,
        Deref @ "nthRoot" => evalBuiltinNthRoot(args, target)?,
        Deref @ "ones" => evalBuiltinOnes(args)?,
        Deref @ "pre" => (args).head().cloned()?,
        Deref @ "product" => evalBuiltinProduct((args).head().cloned()?)?,
        Deref @ "promote" => evalBuiltinPromote((args).get(1)?, (args).get(2)?)?,
        Deref @ "rem" => evalBuiltinRem(args, target)?,
        Deref @ "scalar" => evalBuiltinScalar((args).head().cloned()?)?,
        Deref @ "sign" => evalBuiltinSign((args).head().cloned()?)?,
        Deref @ "sinh" => evalBuiltinSinh((args).head().cloned()?)?,
        Deref @ "sin" => evalBuiltinSin((args).head().cloned()?)?,
        Deref @ "skew" => evalBuiltinSkew((args).head().cloned()?)?,
        Deref @ "smooth" => (args).get(2)?,
        Deref @ "sqrt" => evalBuiltinSqrt((args).head().cloned()?)?,
        Deref @ "String" => evalBuiltinString(args)?,
        Deref @ "sum" => evalBuiltinSum((args).head().cloned()?)?,
        Deref @ "symmetric" => evalBuiltinSymmetric((args).head().cloned()?)?,
        Deref @ "tanh" => evalBuiltinTanh((args).head().cloned()?)?,
        Deref @ "tan" => evalBuiltinTan((args).head().cloned()?)?,
        Deref @ "transpose" => evalBuiltinTranspose((args).head().cloned()?)?,
        Deref @ "vector" => evalBuiltinVector((args).head().cloned()?),
        Deref @ "zeros" => evalBuiltinZeros(args)?,
        Deref @ "OpenModelica_uriToFilename" => evalUriToFilename(r#fn, (args).head().cloned()?, target)?,
        Deref @ "intBitAnd" => evalIntBitAnd(args)?,
        Deref @ "intBitOr" => evalIntBitOr(args)?,
        Deref @ "intBitXor" => evalIntBitXor(args)?,
        Deref @ "intBitLShift" => evalIntBitLShift(args)?,
        Deref @ "intBitRShift" => evalIntBitRShift(args)?,
        Deref @ "intMaxLit" => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: System::intMaxLit() }),
        Deref @ "inferredClock" => evalInferredClock(args)?,
        Deref @ "rationalClock" => evalRationalClock(args)?,
        Deref @ "realClock" => evalRealClock(args)?,
        Deref @ "booleanClock" => evalBooleanClock(args)?,
        Deref @ "solverClock" => evalSolverClock(args)?,
        Deref @ "$OMC$PositiveMax" => evalPositiveMax((args).get(1)?, (args).get(2)?)?,
        Deref @ "$OMC$inStreamDiv" => (args).head().cloned()?,
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalBuiltinCall")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*AbsynUtil::pathString(fn_path, literal!("."), true, false)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalNormalCallExp(
    mut callExp: &metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*callExp)) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: __pa0, arguments: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    result = evalNormalCall(&r#fn, args, target)?;
    Ok(result)
}

pub(crate) fn evalNormalCall(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression> = EvalFunction::evaluate(r#fn, args.clone(), target)?;
    Ok(result)
}

pub(crate) fn evalBuiltinAbs(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::INTEGER { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
            value: (__arg_value.clone()).abs(),
        }),
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).abs(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinAbs")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinAcos(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Real;
    result = (match &*arg {
        Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() < metamodelica::OrderedFloat(-1.0_f64) || x.clone() > metamodelica::OrderedFloat(1.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(
                        &(Error::ARGUMENT_OUT_OF_RANGE.clone()),
                        list![
                            ArcStr::from(::std::format!("{}", x.clone())),
                            literal!("acos"),
                            literal!("-1 <= x <= 1")
                        ],
                        &(EvalTarget::getInfo(target)),
                    )?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: (x.clone()).acos(),
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinAcos")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinArray(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = Expression::typeOf((args).head().cloned()?);
    ty = Type::liftArrayLeft(
        ty,
        &(Dimension::fromInteger(((args).len() as i32), Prefixes::Variability::CONSTANT.clone())),
    );
    result = Expression::makeArray(
        ty,
        metamodelica::arrayFromVec(args.into_iter().cloned().collect()),
        true,
    );
    Ok(result)
}

pub(crate) fn evalBuiltinAsin(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Real;
    result = (match &*arg {
        Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() < metamodelica::OrderedFloat(-1.0_f64) || x.clone() > metamodelica::OrderedFloat(1.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(
                        &(Error::ARGUMENT_OUT_OF_RANGE.clone()),
                        list![
                            ArcStr::from(::std::format!("{}", x.clone())),
                            literal!("asin"),
                            literal!("-1 <= x <= 1")
                        ],
                        &(EvalTarget::getInfo(target)),
                    )?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: (x.clone()).asin(),
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinAsin")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinAtan2(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut y: metamodelica::Real;
    let mut x: metamodelica::Real;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: __esc_y }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: __esc_x }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            y = (*__esc_y).clone();
            x = (*__esc_x).clone();
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: (y.clone()).atan2(x.clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinAtan2")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinAtan(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).atan(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinAtan")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinCat(
    mut argN: &metamodelica::Ref<Expression::NFExpression>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut n: i32;
    let mut nd: i32;
    let mut sz: i32;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut es: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut dims: metamodelica::List<i32>;
    let __pa0 = ::match_deref::match_deref! { match &((*argN)) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    ty = Expression::typeOf((args).head().cloned()?);
    nd = Type::dimensionCount(ty);
    if n > nd || n < 1 {
        if EvalTarget::hasInfo(target) {
            Error::addSourceMessage(
                &(Error::ARGUMENT_OUT_OF_RANGE.clone()),
                list![ArcStr::from(::std::format!("{}", n)), literal!("cat"), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("1 <= x <= "));
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nd)));
                    ArcStr::from(__mm_s)
                }],
                &(EvalTarget::getInfo(target)),
            )?;
        }
        return Err("fail");
    }
    es = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (args.clone()).into_iter().cloned() {
            if !(!(Expression::isEmptyArray(&(e.clone())))) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    sz = ((es).len() as i32);
    if sz == 0 {
        result = (args).head().cloned()?;
    } else if sz == 1 {
        result = (es).head().cloned()?;
    } else {
        (es, dims) = ExpressionBasics::evalCat(
            n,
            es,
            &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::arrayElementList(&__a0),
            &Expression::toString,
        )?;
        result = Expression::arrayFromList(
            es.clone(),
            Expression::typeOf((es).head().cloned()?),
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                for mut d in (dims).into_iter().cloned() {
                    let __x = Dimension::fromInteger(d.clone(), Prefixes::Variability::CONSTANT.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
    }
    Ok(result)
}

pub(crate) fn evalBuiltinCeil(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).ceil(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinCeil")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinCosh(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).cosh(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinCosh")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinCos(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).cos(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinCos")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinDer(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = Expression::fillType(
        Expression::typeOf(arg),
        metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64),
        }),
    )?;
    Ok(result)
}

pub(crate) fn evalBuiltinDiagonal(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut zero: metamodelica::Ref<Expression::NFExpression>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut n: i32;
    let mut i: i32 = 1;
    let mut e_lit: bool;
    let mut arg_lit: bool = true;
    let mut arr_zero: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr_row: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut arr_rows: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    result = (match &*arg {
        Expression::ARRAY { .. }
            if (var_field!((*arg).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .is_empty()) =>
        {
            arg.clone()
        }
        Expression::ARRAY { ty: __arg_ty, .. } => {
            n = metamodelica::arrayLength(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone());
            elem_ty = Type::unliftArray(__arg_ty.clone())?;
            row_ty = Type::liftArrayLeft(
                elem_ty.clone(),
                &(Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())),
            );
            zero = Expression::makeZero(&elem_ty)?;
            arr_zero = arrayCreate(n, zero.clone());
            arr_rows = metamodelica::arrayCreate(n, zero);
            for mut i in 1..=n {
                arr_row = metamodelica::arrayFromVec(arr_zero.clone().borrow().clone());
                exp = metamodelica::Dangerous::arrayGetNoBoundsChecking(
                    var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(),
                    i,
                );
                e_lit = Expression::isLiteral(&exp)?;
                arg_lit = arg_lit && e_lit;
                metamodelica::Dangerous::arrayUpdateNoBoundsChecking(arr_row.clone(), i, exp);
                exp = Expression::makeArray(row_ty.clone(), arr_row.clone(), e_lit);
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr_rows.clone(), i, exp) };
            }
            Expression::makeArray(
                Type::liftArrayLeft(
                    row_ty,
                    &(Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())),
                ),
                arr_rows.clone(),
                arg_lit,
            )
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinDiagonal")),
                list![arg.clone()],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinDiv(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut rx: metamodelica::Real;
    let mut ry: metamodelica::Real;
    let mut ix: i32;
    let mut iy: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_ix }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_iy }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            ix = (*__esc_ix).clone();
            iy = (*__esc_iy).clone();
            if iy.clone() == 0 {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(&(Error::DIVISION_BY_ZERO.clone()), list![ArcStr::from(::std::format!("{}", ix.clone())), ArcStr::from(::std::format!("{}", iy.clone()))], &(EvalTarget::getInfo(target)))?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intDiv(ix.clone(), iy.clone()) })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: __esc_rx }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: __esc_ry }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            rx = (*__esc_rx).clone();
            ry = (*__esc_ry).clone();
            if ry.clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(&(Error::DIVISION_BY_ZERO.clone()), list![ArcStr::from(::std::format!("{}", rx.clone())), ArcStr::from(::std::format!("{}", ry.clone()))], &(EvalTarget::getInfo(target)))?;
                }
                return Err("fail");
            }
            rx = metamodelica::real_div_checked(rx.clone(), ry.clone())?;
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: if (rx.clone() < metamodelica::OrderedFloat(0.0_f64)) {(rx.clone()).ceil()} else {(rx.clone()).floor()} })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinDiv")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinExp(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).exp(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinExp")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

pub(crate) fn evalBuiltinFill(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut fill_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut dims: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        fill_exp = metamodelica::Own::own(__pa1);
        dims = metamodelica::Own::own(__pa2);
        result = unwrap_break_err!(Expression::fillArgs(fill_exp.clone(), dims.clone()), '__try0);
        Ok::<_, &'static str>((dims.clone(), fill_exp.clone(), result.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            dims = __try0_o0;
            fill_exp = __try0_o1;
            result = __try0_o2;
        }
        Err(__try0_err) => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinFill")),
                args.clone(),
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(result)
}

fn evalBuiltinFloor(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).floor(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinFloor")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinIdentity(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::INTEGER { value: __arg_value } => {
            Expression::makeIdentityMatrix(__arg_value.clone(), crate::NFType::interned_INTEGER())?
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinIdentity")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinInteger(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::INTEGER { .. } => arg,
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
            value: ((__arg_value.clone()).0.floor() as i32),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinInteger")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinIntegerEnum(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::ENUM_LITERAL { index: __arg_index, .. } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                value: __arg_index.clone(),
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinIntegerEnum")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinLog10(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Real;
    result = (match &*arg {
        Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() <= metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(
                        &(Error::ARGUMENT_OUT_OF_RANGE.clone()),
                        list![
                            ArcStr::from(::std::format!("{}", x.clone())),
                            literal!("log10"),
                            literal!("x > 0")
                        ],
                        &(EvalTarget::getInfo(target)),
                    )?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: (x.clone()).log10(),
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinLog10")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinLog(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Real;
    result = (match &*arg {
        Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() <= metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(
                        &(Error::ARGUMENT_OUT_OF_RANGE.clone()),
                        list![
                            ArcStr::from(::std::format!("{}", x.clone())),
                            literal!("log"),
                            literal!("x > 0")
                        ],
                        &(EvalTarget::getInfo(target)),
                    )?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: (x.clone()).ln(),
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinLog")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinMatrix(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::ARRAY { ty, .. } => {
            let mut dim_count: i32;
            let mut dim1: metamodelica::Ref<Dimension::NFDimension>;
            let mut dim2: metamodelica::Ref<Dimension::NFDimension>;
            let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            let mut ty = (*ty).clone();
            dim_count = Type::dimensionCount(ty.clone());
            if dim_count < 2 {
                (result, _) = Expression::promote(arg, ty.clone(), 2)?;
            } else if dim_count == 2 {
                result = arg;
            } else {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Type::arrayDims(ty.clone())) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                dim1 = metamodelica::Own::own(__pa0);
                dim2 = metamodelica::Own::own(__pa1);
                ty = Type::liftArrayLeft(Type::arrayElementType(metamodelica::AsArg::as_arg(&ty)), &dim2);
                arr = Array::map(
                    var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(),
                    &({
                        let __pe_b1 = ty.clone();
                        move |__pe_a0| evalBuiltinMatrix2(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                ty = Type::liftArrayLeft(ty.clone(), &dim1);
                result = Expression::makeArray(ty.clone(), arr.clone(), false);
            }
            result
        }
        _ => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = Expression::typeOf(arg.clone());
            if Type::isScalar(&ty) {
                (result, _) = Expression::promote(arg, ty, 2)?;
            } else {
                printWrongArgsError(
                    &(literal!("NFCeval.evalBuiltinMatrix")),
                    list![arg],
                    metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
                )?;
                return Err("fail");
            }
            result
        }
    });
    Ok(result)
}

fn evalBuiltinMatrix2(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::ARRAY {
            literal: __arg_literal, ..
        } => Expression::makeArray(
            ty,
            Array::map(
                var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(),
                &Expression::toScalar,
            )?,
            __arg_literal.clone(),
        ),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinMatrix2")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinMax(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            evalBuiltinMax2(e1.clone(), e2.clone())?
        },
        Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } if (Expression::isArray(metamodelica::AsArg::as_arg(&e1))) => {
            ty = Expression::typeOf(e1.clone());
            result = Expression::fold(e1.clone(), (std::sync::Arc::new(evalBuiltinMax2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty.clone() }))?;
            if Expression::isEmpty(&result) {
                result = Expression::makeMinValue(&(Type::arrayElementType(&ty)))?;
            }
            result
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinMax")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinMax2(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() < var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) {exp2} else {exp1},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone()) {exp2} else {exp1},
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => if (var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() < var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::ARRAY { .. }, _) => exp2,
        (_, Deref @ Expression::EMPTY { .. }) => exp1,
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinMax2")), list![exp1, exp2], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalPositiveMax(
    mut flow_exp: metamodelica::Ref<Expression::NFExpression>,
    mut eps: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = if (Expression::isNonPositive(&flow_exp)?) {
        Expression::makeZero(&(Expression::typeOf(flow_exp)))?
    } else {
        evalBuiltinMax2(flow_exp, eps)?
    };
    Ok(result)
}

fn evalBuiltinMin(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut r#fn: &metamodelica::Ref<Function::Function>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            evalBuiltinMin2(e1.clone(), e2.clone())?
        },
        Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } if (Expression::isArray(metamodelica::AsArg::as_arg(&e1))) => {
            ty = Expression::typeOf(e1.clone());
            result = Expression::fold(e1.clone(), (std::sync::Arc::new(evalBuiltinMin2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty.clone() }))?;
            if Expression::isEmpty(&result) {
                result = Expression::makeMaxValue(&(Type::arrayElementType(&ty)))?;
            }
            result
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinMin")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinMin2(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() > var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) {exp2} else {exp1},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() > var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() > var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone()) {exp2} else {exp1},
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => if (var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() > var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::ARRAY { .. }, _) => exp2,
        (_, Deref @ Expression::EMPTY { .. }) => exp1,
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinMin2")), list![exp1, exp2], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMod(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Ref<Expression::NFExpression>;
    let mut y: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    x = metamodelica::Own::own(__pa0);
    y = metamodelica::Own::own(__pa1);
    result = (::match_deref::match_deref! { match &((&*x, &*y)) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::INTEGER).clone() == 0 {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(&(Error::MODULO_BY_ZERO.clone()), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::INTEGER).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::INTEGER).clone()))], &(EvalTarget::getInfo(target)))?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intMod(var_field!((*x).value, Expression::NFExpression::INTEGER).clone(), var_field!((*y).value, Expression::NFExpression::INTEGER).clone()) })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(&(Error::MODULO_BY_ZERO.clone()), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::REAL).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::REAL).clone()))], &(EvalTarget::getInfo(target)))?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: realMod(var_field!((*x).value, Expression::NFExpression::REAL).clone(), var_field!((*y).value, Expression::NFExpression::REAL).clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinMod")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinNthRoot(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut v: metamodelica::Real;
    let mut n: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: __esc_v }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_n }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            v = (*__esc_v).clone();
            n = (*__esc_n).clone();
            if n.clone() <= 0 && EvalTarget::hasInfo(target) {
                Error::addSourceMessage(&(Error::NON_POSITIVE_NTH_ROOT.clone()), list![ArcStr::from(::std::format!("{}", v.clone())), ArcStr::from(::std::format!("{}", n.clone()))], &(EvalTarget::getInfo(target)))?;
                return Err("fail");
            }
            if intMod(n.clone(), 2) == 0 && v.clone() < metamodelica::OrderedFloat((0) as f64) && EvalTarget::hasInfo(target) {
                Error::addSourceMessage(&(Error::NEGATIVE_NTH_ROOT.clone()), list![ArcStr::from(::std::format!("{}", v.clone())), ArcStr::from(::std::format!("{}", n.clone()))], &(EvalTarget::getInfo(target)))?;
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: if (v.clone() < metamodelica::OrderedFloat(0.0_f64) && intMod(n.clone(), 2) != 0) {-((-(v.clone())).powf((metamodelica::real_div_checked(metamodelica::OrderedFloat((1) as f64), metamodelica::OrderedFloat((n.clone()) as f64))?)))} else {(v.clone()).powf((metamodelica::real_div_checked(metamodelica::OrderedFloat((1) as f64), metamodelica::OrderedFloat((n.clone()) as f64))?))} })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinNthRoot")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinOnes(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = evalBuiltinFill(metamodelica::cons(
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
        args,
    ))?;
    Ok(result)
}

fn evalBuiltinProduct(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        _ if (Expression::isArray(&arg)) => {
            (match &*(Type::arrayElementType(&(Expression::typeOf(arg.clone())))) {
                Type::INTEGER => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                    value: Expression::fold(
                        arg.clone(),
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: i32| {
                            evalBuiltinProductInt(&__a0, __a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, i32) -> Result<i32>
                                    + 'static,
                            >),
                        1,
                    )?,
                }),
                Type::REAL => metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: Expression::fold(
                        arg.clone(),
                        (std::sync::Arc::new(
                            move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: metamodelica::Real| {
                                evalBuiltinProductReal(&__a0, __a1)
                            },
                        )
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                        metamodelica::Real,
                                    ) -> Result<metamodelica::Real>
                                    + 'static,
                            >),
                        metamodelica::OrderedFloat(1.0_f64),
                    )?,
                }),
                _ => {
                    printWrongArgsError(
                        &(literal!("NFCeval.evalBuiltinProduct")),
                        list![arg.clone()],
                        metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
                    )?;
                    return Err("fail");
                }
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinProduct")),
                list![arg.clone()],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinProductInt(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut result: i32) -> Result<i32> {
    let mut result: i32 = result;
    result = (match &**exp {
        Expression::INTEGER { value: __exp_value } => result * __exp_value.clone(),
        Expression::ARRAY { .. } => result,
        _ => return Err("fail"),
    });
    Ok(result)
}

fn evalBuiltinProductReal(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut result: metamodelica::Real,
) -> Result<metamodelica::Real> {
    let mut result: metamodelica::Real = result;
    result = (match &**exp {
        Expression::REAL { value: __exp_value } => result * __exp_value.clone(),
        Expression::ARRAY { .. } => result,
        _ => return Err("fail"),
    });
    Ok(result)
}

fn evalBuiltinPromote(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut argN: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut n: i32;
    if Expression::isInteger(&argN) {
        let __pa0 = ::match_deref::match_deref! { match &(argN) {
            Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        n = metamodelica::Own::own(__pa0);
        (result, _) = Expression::promote(arg.clone(), Expression::typeOf(arg), n)?;
    } else {
        printWrongArgsError(
            &(literal!("NFCeval.evalBuiltinPromote")),
            list![arg, argN],
            metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
        )?;
        return Err("fail");
    }
    Ok(result)
}

fn evalBuiltinRem(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Ref<Expression::NFExpression>;
    let mut y: metamodelica::Ref<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    x = metamodelica::Own::own(__pa0);
    y = metamodelica::Own::own(__pa1);
    result = (::match_deref::match_deref! { match &((&*x, &*y)) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::INTEGER).clone() == 0 {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(&(Error::REM_ARG_ZERO.clone()), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::INTEGER).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::INTEGER).clone()))], &(EvalTarget::getInfo(target)))?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((*x).value, Expression::NFExpression::INTEGER).clone() - intDiv(var_field!((*x).value, Expression::NFExpression::INTEGER).clone(), var_field!((*y).value, Expression::NFExpression::INTEGER).clone()) * var_field!((*y).value, Expression::NFExpression::INTEGER).clone() })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target) {
                    Error::addSourceMessage(&(Error::REM_ARG_ZERO.clone()), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::REAL).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::REAL).clone()))], &(EvalTarget::getInfo(target)))?;
                }
                return Err("fail");
            }
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: var_field!((*x).value, Expression::NFExpression::REAL).clone() - realDiv(var_field!((*x).value, Expression::NFExpression::REAL).clone(), var_field!((*y).value, Expression::NFExpression::REAL).clone()) * var_field!((*y).value, Expression::NFExpression::REAL).clone() })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinRem")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinScalar(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression> = arg;
    while Expression::isArray(&result) {
        result = Expression::arrayScalarElement(&result)?;
    }
    Ok(result)
}

fn evalBuiltinSign(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
            value: if (__arg_value.clone() > metamodelica::OrderedFloat((0) as f64)) {
                1
            } else {
                if (__arg_value.clone() < metamodelica::OrderedFloat((0) as f64)) {
                    -1
                } else {
                    0
                }
            },
        }),
        Expression::INTEGER { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
            value: if (__arg_value.clone() > 0) {
                1
            } else {
                if (__arg_value.clone() < 0) { -1 } else { 0 }
            },
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinSign")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinSinh(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).sinh(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinSinh")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinSin(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).sin(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinSin")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinSkew(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut x1: metamodelica::Ref<Expression::NFExpression>;
    let mut x2: metamodelica::Ref<Expression::NFExpression>;
    let mut x3: metamodelica::Ref<Expression::NFExpression>;
    let mut y1: metamodelica::Ref<Expression::NFExpression>;
    let mut y2: metamodelica::Ref<Expression::NFExpression>;
    let mut y3: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut zero: metamodelica::Ref<Expression::NFExpression>;
    let mut literal: bool;
    result = (match &*arg {
        Expression::ARRAY {
            ty: __esc_ty,
            literal: __esc_literal,
            ..
        } => {
            ty = (*__esc_ty).clone();
            literal = (*__esc_literal).clone();
            x1 = metamodelica::arrayGet(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), 1)?;
            x2 = metamodelica::arrayGet(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), 2)?;
            x3 = metamodelica::arrayGet(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), 3)?;
            zero = Expression::makeZero(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&ty))))?;
            y1 = Expression::makeArray(
                ty.clone(),
                metamodelica::arrayFromVec(
                    list![zero.clone(), Expression::negate(x3.clone()), x2.clone()]
                        .into_iter()
                        .cloned()
                        .collect(),
                ),
                literal.clone(),
            );
            y2 = Expression::makeArray(
                ty.clone(),
                metamodelica::arrayFromVec(
                    list![x3, zero.clone(), Expression::negate(x1.clone())]
                        .into_iter()
                        .cloned()
                        .collect(),
                ),
                literal.clone(),
            );
            y3 = Expression::makeArray(
                ty.clone(),
                metamodelica::arrayFromVec(list![Expression::negate(x2), x1, zero].into_iter().cloned().collect()),
                literal.clone(),
            );
            ty = Type::liftArrayLeft(
                ty.clone(),
                &(Dimension::fromInteger(3, Prefixes::Variability::CONSTANT.clone())),
            );
            Expression::makeArray(
                ty.clone(),
                metamodelica::arrayFromVec(list![y1, y2, y3].into_iter().cloned().collect()),
                literal.clone(),
            )
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinSkew")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinSqrt(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } if (__arg_value.clone() >= metamodelica::OrderedFloat(0.0_f64)) => {
            metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: (__arg_value.clone()).sqrt(),
            })
        }
        Expression::REAL { .. } => return Err("fail"),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinSqrt")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinString(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: arg, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: min_len }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::BOOLEAN { value: left_justified }, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            let mut str_len: i32;
            let mut r#str: ArcStr;
            r#str = (match &*arg.clone() {
        Expression::INTEGER { value: __arg_value } => intString(__arg_value.clone()),
        Expression::BOOLEAN { value: __arg_value } => boolString(__arg_value.clone()),
        Expression::ENUM_LITERAL { name: __arg_name, .. } => __arg_name.clone(),
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBuiltinString")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
    });
            str_len = ((r#str).len() as i32);
            if str_len < min_len.clone() {
                if left_justified.clone() {
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*stringAppendList(List::fill(literal!(" "), min_len.clone() - str_len))); ArcStr::from(__mm_s) };
                } else {
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*stringAppendList(List::fill(literal!(" "), min_len.clone() - str_len))); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                }
            }
            metamodelica::Ref::new(Expression::NFExpression::STRING { value: r#str })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: r }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: significant_digits }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: min_len }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::BOOLEAN { value: left_justified }, tail: Deref @ metamodelica::ListNode::Nil } } } } => {
            let mut r#str: ArcStr;
            let mut format: ArcStr;
            format = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*if (left_justified.clone()) {literal!("-")} else {literal!("")}); __mm_s.push_str(&*intString(min_len.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(significant_digits.clone())); __mm_s.push_str(&*literal!("g")); ArcStr::from(__mm_s) };
            r#str = System::sprintff(format, r.clone())?;
            metamodelica::Ref::new(Expression::NFExpression::STRING { value: r#str })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::REAL { value: r }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::STRING { value: format }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut r#str: ArcStr;
            r#str = System::sprintff({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*format); ArcStr::from(__mm_s) }, r.clone())?;
            metamodelica::Ref::new(Expression::NFExpression::STRING { value: r#str })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(result)
}

fn evalBuiltinSum(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        _ if (Expression::isArray(&arg)) => {
            (match &*(Type::arrayElementType(&(Expression::typeOf(arg.clone())))) {
                Type::INTEGER => metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                    value: Expression::fold(
                        arg.clone(),
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: i32| {
                            evalBuiltinSumInt(&__a0, __a1)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, i32) -> Result<i32>
                                    + 'static,
                            >),
                        0,
                    )?,
                }),
                Type::REAL => metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: Expression::fold(
                        arg.clone(),
                        (std::sync::Arc::new(
                            move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: metamodelica::Real| {
                                evalBuiltinSumReal(&__a0, __a1)
                            },
                        )
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                        metamodelica::Real,
                                    ) -> Result<metamodelica::Real>
                                    + 'static,
                            >),
                        metamodelica::OrderedFloat(0.0_f64),
                    )?,
                }),
                _ => {
                    printWrongArgsError(
                        &(literal!("NFCeval.evalBuiltinSum")),
                        list![arg.clone()],
                        metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
                    )?;
                    return Err("fail");
                }
            })
        }
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinSum")),
                list![arg.clone()],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinSumInt(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut result: i32) -> Result<i32> {
    let mut result: i32 = result;
    result = (match &**exp {
        Expression::INTEGER { value: __exp_value } => result + __exp_value.clone(),
        Expression::ARRAY { .. } => result,
        _ => return Err("fail"),
    });
    Ok(result)
}

fn evalBuiltinSumReal(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut result: metamodelica::Real,
) -> Result<metamodelica::Real> {
    let mut result: metamodelica::Real = result;
    result = (match &**exp {
        Expression::REAL { value: __exp_value } => result + __exp_value.clone(),
        Expression::ARRAY { .. } => result,
        _ => return Err("fail"),
    });
    Ok(result)
}

fn evalBuiltinSymmetric(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut mat: metamodelica::Array<metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>>;
    let mut n: i32;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut accum: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    ty = Expression::typeOf(arg.clone());
    if Expression::isArray(&arg) && Type::isSquareMatrix(&ty)? {
        mat = Array::map(Expression::arrayElements(&arg)?, &move |__a0: metamodelica::Ref<
            Expression::NFExpression,
        >| {
            Expression::arrayElements(&__a0)
        })?;
        n = metamodelica::arrayLength(mat.clone());
        row_ty = Type::unliftArray(Expression::typeOf(arg.clone()))?;
        accum = metamodelica::arrayCreate(n, arg.clone());
        for mut i in 1..=n {
            arr = metamodelica::arrayCreate(n, arg.clone());
            for mut j in 1..=n {
                unsafe {
                    metamodelica::Dangerous::arrayInitSlot(
                        arr.clone(),
                        j,
                        if (i > j) {
                            metamodelica::arrayGet(
                                ({
                                    let __elt = (*metamodelica::index_checked(&mat.borrow(), j)?).clone();
                                    __elt
                                }),
                                i,
                            )?
                        } else {
                            metamodelica::arrayGet(
                                ({
                                    let __elt = (*metamodelica::index_checked(&mat.borrow(), i)?).clone();
                                    __elt
                                }),
                                j,
                            )?
                        },
                    )
                };
            }
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    accum.clone(),
                    i,
                    Expression::makeArray(row_ty.clone(), arr.clone(), true),
                )
            };
        }
        result = Expression::makeArray(ty, accum.clone(), true);
    } else {
        printWrongArgsError(
            &(literal!("NFCeval.evalBuiltinSymmetric")),
            list![arg],
            metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
        )?;
        return Err("fail");
    }
    Ok(result)
}

fn evalBuiltinTanh(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).tanh(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinTanh")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinTan(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::REAL { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (__arg_value.clone()).tan(),
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalBuiltinTan")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalBuiltinTranspose(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = Expression::typeOf(arg.clone());
    if Expression::isArray(&arg) && Type::dimensionCount(ty) >= 2 {
        result = Expression::transposeArray(&arg)?;
    } else {
        printWrongArgsError(
            &(literal!("NFCeval.evalBuiltinTranspose")),
            list![arg],
            metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
        )?;
        return Err("fail");
    }
    Ok(result)
}

fn evalBuiltinVector(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    expl = Expression::arrayScalarElements(&arg);
    result = Expression::makeExpArray(
        metamodelica::arrayFromVec(expl.into_iter().cloned().collect()),
        Type::arrayElementType(&(Expression::typeOf(arg))),
        true,
    );
    result
}

fn evalBuiltinZeros(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = evalBuiltinFill(metamodelica::cons(
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
        args,
    ))?;
    Ok(result)
}

fn evalUriToFilename(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*arg {
        Expression::STRING { value: __arg_value } => metamodelica::Ref::new(Expression::NFExpression::FILENAME {
            filename: uriToFilename(__arg_value.clone())?,
        }),
        Expression::FILENAME {
            filename: __arg_filename,
        } => metamodelica::Ref::new(Expression::NFExpression::FILENAME {
            filename: uriToFilename(__arg_filename.clone())?,
        }),
        _ => {
            printWrongArgsError(
                &(literal!("NFCeval.evalUriToFilename")),
                list![arg],
                metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(result)
}

fn evalIntBitAnd(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut i1: i32;
    let mut i2: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intBitAnd(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalIntBitAnd")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitOr(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut i1: i32;
    let mut i2: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intBitOr(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalIntBitOr")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitXor(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut i1: i32;
    let mut i2: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intBitXor(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalIntBitXor")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitLShift(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut i1: i32;
    let mut i2: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intBitLShift(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalIntBitLShift")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitRShift(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut i1: i32;
    let mut i2: i32;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: intBitRShift(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalIntBitRShift")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalInferredClock(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Nil => metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(NFClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) }),
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalInferredClock")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalRationalClock(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: interval @ Deref @ Expression::INTEGER { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: resolution @ Deref @ Expression::INTEGER { .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(NFClockKind::NFClockKind::RATIONAL_CLOCK { intervalCounter: interval.clone(), resolution: resolution.clone() }) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalRationalClock")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalRealClock(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: interval @ Deref @ Expression::REAL { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(NFClockKind::NFClockKind::REAL_CLOCK { interval: interval.clone() }) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalRealClock")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBooleanClock(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: condition @ Deref @ Expression::BOOLEAN { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: interval @ Deref @ Expression::REAL { .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(NFClockKind::NFClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: interval.clone() }) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalBooleanClock")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalSolverClock(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: c @ Deref @ Expression::CLKCONST { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: solver @ Deref @ Expression::STRING { .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(Expression::NFExpression::CLKCONST { clk: metamodelica::Ref::new(NFClockKind::NFClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solver.clone() }) })
        },
        _ => {
            printWrongArgsError(&(literal!("NFCeval.evalSolverClock")), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalGetInstanceName(
    mut scope: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = metamodelica::Ref::new(Expression::NFExpression::STRING {
        value: AbsynUtil::pathString(
            NFInstNode::InstNode::rootPath(scope, false)?,
            literal!("."),
            true,
            false,
        )?,
    });
    Ok(result)
}

fn evalArrayConstructor(
    mut callExp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut iter_exps: metamodelica::List<Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*callExp)) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: __pa0, iters: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    iters = metamodelica::Own::own(__pa1);
    (exp, ranges, iter_exps) = Expression::createIterationRanges(exp, &iters)?;
    result = evalArrayConstructor2(exp, &ranges, &iter_exps)?;
    Ok(result)
}

fn evalArrayConstructor2(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ranges: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut iterators: &metamodelica::List<Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut ranges_rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut iter: Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>;
    let mut iters_rest: metamodelica::List<Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>>;
    let mut range_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if (ranges).is_empty() {
        result = evalExp(exp, &(noTarget().clone()))?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*ranges)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa0);
        ranges_rest = metamodelica::Own::own(__pa1);
        range = evalExp(range, &(noTarget().clone()))?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*iterators)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        iter = metamodelica::Own::own(__pa2);
        iters_rest = metamodelica::Own::own(__pa3);
        range_iter = ExpressionIterator::fromExp(range, false, false)?;
        while ExpressionIterator::hasNext(&range_iter) {
            (range_iter, value) = ExpressionIterator::next(range_iter)?;
            Mutable::update(iter.clone(), value);
            expl = metamodelica::cons(evalArrayConstructor2(exp.clone(), &ranges_rest, &iters_rest)?, expl);
        }
        arr = metamodelica::arrayFromVec(
            metamodelica::Dangerous::listReverseInPlace(expl.clone())
                .into_iter()
                .cloned()
                .collect(),
        );
        ty = if (arr.clone().borrow().is_empty()) {
            Type::liftArrayLeftList(
                Expression::typeOf(exp),
                &(List::mapFlat(
                    &ranges_rest,
                    &fnptr!(Expression::dimensions, metamodelica::Ref<Expression::NFExpression>),
                )?),
            )
        } else {
            Expression::typeOf((expl).head().cloned()?)
        };
        ty = Type::liftArrayLeft(
            ty,
            &(Dimension::fromInteger(
                metamodelica::arrayLength(arr.clone()),
                Prefixes::Variability::CONSTANT.clone(),
            )),
        );
        result = Expression::makeArray(ty, arr.clone(), true);
    }
    Ok(result)
}

type ReductionFn = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
        + 'static,
>;

fn evalReduction(
    mut callExp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    fn reductionFn(
        mut exp1: metamodelica::Ref<Expression::NFExpression>,
        mut exp2: metamodelica::Ref<Expression::NFExpression>,
        mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
        mut r#fn: &dyn ::std::ops::Fn(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::Ref<Expression::NFExpression>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut result: metamodelica::Ref<Expression::NFExpression> =
            r#fn(exp1.clone(), evalExp(exp2.clone(), target)?)?;
        Ok(result)
    }

    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut default_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut red_fn: ReductionFn;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*callExp)) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { r#fn: __pa0, exp: __pa1, iters: __pa2, .. } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r#fn = metamodelica::Own::own(__pa0);
    exp = metamodelica::Own::own(__pa1);
    iters = metamodelica::Own::own(__pa2);
    ty = Expression::typeOf(exp.clone());
    (red_fn, default_exp) = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::name(&r#fn), literal!("."), true, false)?) {
        Deref @ "sum" => ((std::sync::Arc::new(evalBinaryAdd) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), Expression::makeZero(&ty)?),
        Deref @ "product" => ((std::sync::Arc::new(evalBinaryMul) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), Expression::makeOne(&ty)?),
        Deref @ "min" => ((std::sync::Arc::new(evalBuiltinMin2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), Expression::makeMaxValue(&ty)?),
        Deref @ "max" => ((std::sync::Arc::new(evalBuiltinMax2) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), Expression::makeMinValue(&ty)?),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalReduction")); __mm_s.push_str(&*literal!(" got unknown reduction function ")); __mm_s.push_str(&*AbsynUtil::pathString(Function::name(&r#fn), literal!("."), true, false)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = Expression::foldReduction(
        exp,
        &iters,
        default_exp,
        &({
            let __pe_b1 = noTarget().clone();
            move |__pe_a0| evalExp(__pe_a0, &__pe_b1)
        }),
        &*(red_fn.clone()),
    )?;
    Ok(result)
}

fn evalSize(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut optIndex: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut index_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut index: i32;
    let mut ty_err: metamodelica::Ref<TypingError::TypingError>;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut info: SourceInfo;
    let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    info = EvalTarget::getInfo(target);
    if (optIndex).is_some() {
        index_exp = evalExp(Util::getOption(optIndex)?, target)?;
        index = Expression::toInteger(&index_exp)?;
        (dim, _, ty_err) = Typing::typeExpDim(exp.clone(), index, InstContext::CLASS.clone(), info.clone())?;
        Typing::checkSizeTypingError(&ty_err, exp, index, &info)?;
        outExp = Dimension::sizeExp(&dim)?;
        outExp = evalExp(outExp, target)?;
    } else {
        (outExp, ty, _, _) = Typing::typeExp(exp, InstContext::CLASS.clone(), &info, false)?;
        arr = Array::mapList(&(Type::arrayDims(ty)), &move |__a0: metamodelica::Ref<
            Dimension::NFDimension,
        >| Dimension::sizeExp(&__a0))?;
        Array::mapNoCopy(
            arr.clone(),
            &({
                let __pe_b1 = target.clone();
                move |__pe_a0| evalExp(__pe_a0, &__pe_b1)
            }),
        )?;
        dim = Dimension::fromInteger(metamodelica::arrayLength(arr.clone()), Variability::PARAMETER.clone());
        outExp = Expression::makeArray(
            metamodelica::Ref::new(Type::NFType::ARRAY {
                elementType: crate::NFType::interned_INTEGER(),
                dimensions: list![dim],
            }),
            arr.clone(),
            false,
        );
    }
    Ok(outExp)
}

fn evalSubscriptedExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    result = (match &*exp {
        Expression::RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ty: __exp_ty,
        } => metamodelica::Ref::new(Expression::NFExpression::RANGE {
            ty: __exp_ty.clone(),
            start: evalExp(__exp_start.clone(), target)?,
            step: Util::applyOption(
                __exp_step.clone(),
                &({
                    let __pe_b1 = target.clone();
                    move |__pe_a0| evalExp(__pe_a0, &__pe_b1)
                }),
            )?,
            stop: evalExp(__exp_stop.clone(), target)?,
        }),
        _ => evalExp(exp, target)?,
    });
    subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (subscripts).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(
                s.clone(),
                &({
                    let __pe_b1 = target.clone();
                    move |__pe_a0| evalExp(__pe_a0, &__pe_b1)
                }),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    result = Expression::applySubscripts(&subs, result, false)?;
    Ok(result)
}

fn evalRecordElement(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut index: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    index = metamodelica::Own::own(__pa1);
    e = evalExp(e, target)?;
    if '__try2: {
        result = unwrap_break_err!(Expression::mapSplitExpressions(e.clone(), &({ let __pe_b0 = index; move |__pe_a1| Expression::nthRecordElement(__pe_b0.clone(), &__pe_a1) })), '__try2);
        Ok::<(), &'static str>(())
    }.is_err() {
        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalRecordElement")); __mm_s.push_str(&*literal!(" could not evaluate ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo")))?;
    }
    Ok(result)
}

fn evalRecordElement2(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut index: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &**exp {
        Expression::RECORD {
            elements: __exp_elements,
            ..
        } => (__exp_elements).get(index)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(result)
}

fn printUnboundError(
    mut component: &metamodelica::Ref<Component::NFComponent>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<()> {
    let mut extra: metamodelica::Ref<EvalTargetData>;
    if !(EvalTarget::hasInfo(target)) {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(target.extra.clone()) {
        Some(__esc_extra @ Deref @ EvalTargetData { .. }) => {
            extra = (*__esc_extra).clone();
            Error::addSourceMessage(&(Error::STRUCTURAL_PARAMETER_OR_CONSTANT_WITH_NO_BINDING.clone()), list![Expression::toString(extra.exp.clone())?, NFInstNode::InstNode::name(&extra.component)?], &target.info)?;
            return Err("fail")
        },
        _ if (InstContext::inCondition(target.context.clone())) => {
            Error::addSourceMessage(&(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone()), list![Expression::toString(exp)?], &target.info)?;
            return Err("fail")
        },
        _ => {
            if listMember(Component::variability(component)?, list![Variability::STRUCTURAL_PARAMETER.clone(), Variability::PARAMETER.clone()]) && Util::getOptionOrDefault(Component::getEvaluateAnnotation(component)?, false) {
                if Component::isFixed(component)? {
                    Error::addMultiSourceMessage(&(Error::UNBOUND_PARAMETER_EVALUATE_TRUE.clone()), &(list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("(fixed = true)")); ArcStr::from(__mm_s) }]), &(list![NFInstNode::InstNode::info(&(ComponentRef::node(&(Expression::toCref(&exp)?))?)), EvalTarget::getInfo(target)]))?;
                }
            } else {
                Error::addMultiSourceMessage(&(Error::UNBOUND_CONSTANT.clone()), &(list![Expression::toString(exp.clone())?]), &(list![NFInstNode::InstNode::info(&(ComponentRef::node(&(Expression::toCref(&exp)?))?)), EvalTarget::getInfo(target)]))?;
                return Err("fail");
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printWrongArgsError(
    mut evalFunc: &ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut info: SourceInfo,
) -> Result<()> {
    Error::addInternalError(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*evalFunc);
            __mm_s.push_str(&*literal!(" got invalid arguments "));
            __mm_s.push_str(&*List::toString(
                args,
                &Expression::toString,
                List::Style::FLAT_BRACKETS.clone(),
            )?);
            ArcStr::from(__mm_s)
        },
        info,
    )?;
    Ok(())
}
