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

use crate::FFI;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunctionExt as EvalFunctionExt;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Autoconf;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum FlowControl {
    NEXT = 1,
    CONTINUE = 2,
    BREAK = 3,
    RETURN = 4,
    ASSERTION = 5,
}
impl PartialOrd for FlowControl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FlowControl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for FlowControl {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub type ArgumentMap = metamodelica::Ref<
    UnorderedMap::UnorderedMap<metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<Expression::NFExpression>>,
>;

pub(crate) const STATEMENT_CONTEXT: i32 = intBitOr(InstContext::FUNCTION, InstContext::ALGORITHM);

pub(crate) const IF_COND_CONTEXT: i32 = intBitOr(STATEMENT_CONTEXT, intBitOr(InstContext::IF, InstContext::CONDITION));

pub(crate) fn evaluate(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    if Function::isExternal(r#fn)? {
        result = evaluateExternal(r#fn, args, target)?;
    } else if Function::isPartialDerivative(r#fn) {
        return Err("fail");
    } else {
        result = evaluateNormal(r#fn, args, target.context.clone())?;
    }
    Ok(result)
}

pub(crate) fn evaluateNormal(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut context: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut arg_map: ArgumentMap;
    let mut call_count: i32;
    let mut limit: i32;
    let mut call_counter: Pointer::Pointer<i32> = r#fn.callCounter.clone();
    let mut ctrl: FlowControl;
    let mut body_context: i32;
    call_count = Pointer::access(call_counter.clone()) + 1;
    limit = Flags::getConfigInt(Flags::EVAL_RECURSION_LIMIT.clone())?;
    if call_count > limit {
        Pointer::update(call_counter.clone(), 0);
        Error::addSourceMessage(
            &(Error::EVAL_RECURSION_LIMIT_REACHED.clone()),
            list![
                ArcStr::from(::std::format!("{}", limit)),
                AbsynUtil::pathString(Function::name(r#fn), literal!("."), true, false)?
            ],
            &(InstNode::info(&(InstNode::fromHandle(&r#fn.node)?))),
        )?;
        return Err("fail");
    }
    Pointer::update(call_counter.clone(), call_count);
    body_context = InstContext::clearScopeFlags(context);
    match '__try0: {
        fn_body = unwrap_break_err!(Function::getBody(r#fn), '__try0);
        arg_map = unwrap_break_err!(createArgumentMap(&r#fn.inputs, &r#fn.outputs, &r#fn.locals, args.clone(), true, true), '__try0);
        fn_body = unwrap_break_err!(applyReplacements(arg_map.clone(), fn_body.clone()), '__try0);
        fn_body = unwrap_break_err!(optimizeBody(fn_body.clone()), '__try0);
        ctrl = unwrap_break_err!(evaluateStatements(&fn_body, body_context), '__try0);
        if ctrl != FlowControl::ASSERTION.clone() {
            result = unwrap_break_err!(createResult(arg_map.clone(), &r#fn.outputs), '__try0);
        } else {
            break '__try0 Err::<_, _>("fail");
        }
        Ok::<_, &'static str>((arg_map.clone(), ctrl.clone(), fn_body.clone(), result.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            arg_map = __try0_o0;
            ctrl = __try0_o1;
            fn_body = __try0_o2;
            result = __try0_o3;
        }
        Err(__try0_err) => {
            Pointer::update(call_counter.clone(), call_count - 1);
            return Err(__try0_err);
        }
    }
    if Flags::isSet(Flags::EVAL_FUNC_DUMP.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*AbsynUtil::pathString(
                Function::name(r#fn),
                literal!("."),
                true,
                false,
            )?);
            __mm_s.push_str(&*literal!(" => "));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(Expression::toString(result.clone())?);
        metamodelica::print(literal!("\nArguments:\n"));
        metamodelica::print(UnorderedMap::toString(
            arg_map,
            &move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::name(&__a0),
            &Expression::toString,
            literal!("\n"),
            &(literal!(", ")),
        )?);
        metamodelica::print(literal!("\n"));
    }
    Pointer::update(call_counter, call_count - 1);
    Ok(result)
}

pub(crate) fn evaluateExternal(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut name: ArcStr;
    let mut lang: ArcStr;
    let mut output_ref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
    let mut ext_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Class::getSections(InstNode::getClass(InstNode::fromHandle(&r#fn.node)?)?)?) {
        Deref @ Sections::EXTERNAL { name: __pa0, args: __pa1, outputRef: __pa2, language: __pa3, ann: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    ext_args = metamodelica::Own::own(__pa1);
    output_ref = metamodelica::Own::own(__pa2);
    lang = metamodelica::Own::own(__pa3);
    ann = metamodelica::Own::own(__pa4);
    result = 'mc: {
        let __mc_input = lang;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "builtin" => {
                    Ok(Ceval::evalBuiltinCall(r#fn, args.clone(), &(Ceval::noTarget().clone()))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "FORTRAN 77" => {
                    Ok(evaluateExternal2(&name, r#fn, args.clone(), ext_args.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((!(InstContext::inInstanceAPI(target.context.clone())))) { return Err("guard") }
                    Ok(callExternalFunction(name.clone(), r#fn, args.clone(), &ext_args, &output_ref, ann.clone(), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Ceval::EvalTarget::hasInfo(target) {
                        Error::addSourceMessage(&(Error::FAILED_TO_EVALUATE_FUNCTION.clone()), list![AbsynUtil::pathString(r#fn.path.clone(), literal!("."), true, false)?], &(Ceval::EvalTarget::getInfo(target)))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(result)
}

pub(crate) fn evaluateRecordConstructor(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut evaluate: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut arg_map: ArgumentMap;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut out_ty: metamodelica::Ref<InstNode::InstNode>;
    arg_map = createArgumentMap(&r#fn.inputs, &(metamodelica::nil()), &r#fn.locals, args, false, true)?;
    out_ty = Type::complexNode(&r#fn.returnType)?;
    let __range0 = ClassTree::getComponents(&(Class::classTree(InstNode::getClass(out_ty)?)?))?
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    for mut c in __range0 {
        expl = metamodelica::cons(UnorderedMap::getOrFail(c, arg_map.clone())?, expl);
    }
    result = Expression::makeRecord(
        Function::name(r#fn),
        ty,
        metamodelica::Dangerous::listReverseInPlace(expl),
    );
    if evaluate {
        result = Ceval::evalExp(result, &(Ceval::noTarget().clone()))?;
    }
    Ok(result)
}

fn createArgumentMap(
    mut inputs: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut outputs: &metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>,
    mut locals: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut mutableParams: bool,
    mut buildArrayBinding: bool,
) -> Result<ArgumentMap> {
    let mut map: ArgumentMap;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = args;
    let mut r#fn: metamodelica::Ref<Function::Function> =
        <metamodelica::Ref<Function::Function> as ::std::default::Default>::default();
    map = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                InstNode::refEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<InstNode::InstNode>,
                        metamodelica::Ref<InstNode::InstNode>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    for mut i in &**inputs {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        UnorderedMap::add(i.clone(), arg.clone(), map.clone())?;
        if Expression::isFunctionPointer(&arg) {
            for mut r#fn in &*Function::getCachedFuncs(i.clone())? {
                let mut r#fn = r#fn.clone();
                UnorderedMap::add(InstNode::fromHandle(&r#fn.node)?, arg.clone(), map.clone())?;
            }
        }
    }
    if mutableParams {
        for mut o in &**outputs {
            map = addMutableArgument(
                InstNode::fromHandle(metamodelica::AsArg::as_arg(&o))?,
                map,
                buildArrayBinding,
            )?;
        }
        List::fold(
            locals,
            &({
                let __pe_b2 = buildArrayBinding;
                move |__pe_a0, __pe_a1| addMutableArgument(__pe_a0, __pe_a1, __pe_b2.clone())
            }),
            map.clone(),
        )?;
    } else {
        for mut o in &**outputs {
            map = addImmutableArgument(
                InstNode::fromHandle(metamodelica::AsArg::as_arg(&o))?,
                map,
                buildArrayBinding,
            )?;
        }
        List::fold(
            locals,
            &({
                let __pe_b2 = buildArrayBinding;
                move |__pe_a0, __pe_a1| addImmutableArgument(__pe_a0, __pe_a1, __pe_b2.clone())
            }),
            map.clone(),
        )?;
    }
    UnorderedMap::apply(
        map.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = map.clone();
            move |__pe_a0| applyBindingReplacement(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    UnorderedMap::apply(
        map.clone(),
        (std::sync::Arc::new(evaluateReplacement)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(map)
}

fn addMutableArgument(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut map: ArgumentMap,
    mut buildArrayBinding: bool,
) -> Result<ArgumentMap> {
    let mut map: ArgumentMap = map;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = getBindingExp(node.clone(), map.clone(), true, buildArrayBinding)?;
    exp = Expression::makeMutable(exp);
    UnorderedMap::add(node, exp, map.clone())?;
    Ok(map)
}

fn addImmutableArgument(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut map: ArgumentMap,
    mut buildArrayBinding: bool,
) -> Result<ArgumentMap> {
    let mut map: ArgumentMap = map;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = getBindingExp(node.clone(), map.clone(), false, buildArrayBinding)?;
    UnorderedMap::add(node, exp, map.clone())?;
    Ok(map)
}

fn getBindingExp(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut map: ArgumentMap,
    mut mutableParams: bool,
    mut buildArrayBinding: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut bindingExp: metamodelica::Ref<Expression::NFExpression>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    comp = InstNode::component(&node)?;
    binding = Component::getBinding(&comp);
    if Binding::isBound(&binding) {
        bindingExp = Expression::clone(Binding::getExp(&binding)?)?;
    } else {
        bindingExp = buildBinding(node, map, mutableParams, buildArrayBinding)?;
    }
    Ok(bindingExp)
}

fn buildBinding(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut map: ArgumentMap,
    mut mutableParams: bool,
    mut buildArrayBinding: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = InstNode::getType(node.clone())?;
    ty = Type::mapDims(
        ty,
        &({
            let __pe_b0 = map.clone();
            move |__pe_a1| applyReplacementsDim(__pe_b0.clone(), __pe_a1)
        }),
    )?;
    result = (match &*ty {
        Type::ARRAY { .. } if (buildArrayBinding) => {
            if (Type::hasKnownSize(ty.clone())?) {
                Expression::fillType(
                    ty.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                        ty: Type::arrayElementType(&ty),
                    }),
                )?
            } else {
                metamodelica::Ref::new(Expression::NFExpression::ARRAY {
                    ty: ty,
                    elements: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                    literal: true,
                })
            }
        }
        Type::COMPLEX { .. } => buildRecordBinding(node, map, mutableParams)?,
        _ => metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: ty }),
    });
    Ok(result)
}

fn applyReplacementsDim(
    mut map: ArgumentMap,
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    dim = (match &*dim {
        Dimension::EXP { exp: __dim_exp, .. } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            exp = Expression::map(
                __dim_exp.clone(),
                (std::sync::Arc::new({
                    let __pe_b0 = map;
                    move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            exp = Ceval::evalExp(exp, &(Ceval::noTarget().clone()))?;
            Dimension::fromExp(exp, Variability::CONSTANT.clone())?
        }
        _ => dim,
    });
    Ok(dim)
}

fn buildRecordBinding(
    mut recordNode: metamodelica::Ref<InstNode::InstNode>,
    mut map: ArgumentMap,
    mut mutableParams: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    '__tco: loop {
        let mut cls_node: metamodelica::Ref<InstNode::InstNode> = InstNode::classScope(recordNode.clone())?;
        let mut cls: metamodelica::Ref<Class::NFClass> = InstNode::getClass(cls_node.clone())?;
        let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
        let mut bindings: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        let mut local_map: ArgumentMap;
        ::match_deref::match_deref! { match &(cls) {
            Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_comps, .. }, ty: __cls_ty, .. } => {
                comps = (*__esc_comps).clone();
                bindings = metamodelica::nil();
                local_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| InstNode::refEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static>), 1);
                let __range0 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut comp in __range0 {
                    exp = getBindingExp(comp.clone(), map.clone(), mutableParams, true)?;
                    if mutableParams {
                        exp = Expression::makeMutable(exp);
                    }
                    UnorderedMap::add(comp, exp, local_map.clone())?;
                }
                UnorderedMap::apply(local_map.clone(), (std::sync::Arc::new({ let __pe_b1 = local_map.clone(); move |__pe_a0| applyBindingReplacement(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                bindings = UnorderedMap::valueList(local_map);
                return Ok(Expression::makeRecord(InstNode::fullPath(cls_node, false)?, __cls_ty.clone(), bindings))
            },
            Deref @ Class::TYPED_DERIVED { baseClass: __cls_baseClass, .. } => { (recordNode, map, mutableParams) = (__cls_baseClass.clone(), map, mutableParams); continue '__tco; },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn applyBindingReplacement(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut map: ArgumentMap,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    outExp = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b0 = map;
            move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(outExp)
}

fn applyReplacements(
    mut map: ArgumentMap,
    mut fnBody: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut fnBody: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = fnBody;
    fnBody = Statement::mapExpList(
        fnBody,
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            > = (std::sync::Arc::new({
                let __pe_b0 = map;
                move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >);
            move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(fnBody)
}

fn applyReplacements2(
    mut map: ArgumentMap,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp.clone() {
        Expression::CREF { cref: __exp_cref, .. } => {
            applyReplacementCref(map, metamodelica::AsArg::as_arg(&__exp_cref), exp)?
        }
        Expression::CALL { call: __exp_call } => applyReplacementCall(map, __exp_call.clone(), exp)?,
        Expression::UNBOX { exp: __exp_exp, .. } => __exp_exp.clone(),
        _ => exp,
    });
    Ok(exp)
}

fn applyReplacementCref(
    mut map: ArgumentMap,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut cref_parts: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut repl_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut parent: metamodelica::Ref<InstNode::InstNode>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    cref_parts = ComponentRef::toListReverse(cref, true, metamodelica::nil());
    if (cref_parts).is_empty() {
        outExp = exp;
    } else {
        parent = ComponentRef::node(&((cref_parts).head().cloned()?))?;
        repl_exp = UnorderedMap::get(parent, map.clone())?;
        if (repl_exp).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(repl_exp) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            outExp = metamodelica::Own::own(__pa0);
        } else {
            outExp = exp;
            return Ok(outExp);
        }
        outExp = Expression::applySubscripts(
            &(ComponentRef::getSubscripts(&((cref_parts).head().cloned()?))),
            outExp,
            false,
        )?;
        cref_parts = (cref_parts).rest()?;
        if !((cref_parts).is_empty()) {
            if '__try1: {
                for mut cr in &*cref_parts {
                    node = unwrap_break_err!(ComponentRef::node(metamodelica::AsArg::as_arg(&cr)), '__try1);
                    outExp = Expression::makeImmutable(outExp.clone());
                    outExp = unwrap_break_err!(Expression::recordElement(&(unwrap_break_err!(InstNode::name(&node), '__try1)), &outExp), '__try1);
                    outExp = unwrap_break_err!(Expression::applySubscripts(&(ComponentRef::getSubscripts(metamodelica::AsArg::as_arg(&cr))), outExp.clone(), false), '__try1);
                }
                Ok::<(), &'static str>(())
            }.is_err() {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.applyReplacementCref")); __mm_s.push_str(&*literal!(" could not find replacement for ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFEvalFunction.mo")))?;
            }
        }
        outExp = Expression::map(
            outExp,
            (std::sync::Arc::new({
                let __pe_b0 = map;
                move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    }
    Ok(outExp)
}

fn applyReplacementCall(
    mut map: ArgumentMap,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut repl_oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut repl_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    outExp = (match &*call {
        Call::TYPED_CALL { r#fn: __call_fn, .. } => {
            repl_oexp = UnorderedMap::get(InstNode::fromHandle(&__call_fn.node)?, map)?;
            if (repl_oexp).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(repl_oexp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                repl_exp = metamodelica::Own::own(__pa0);
                outExp = (::match_deref::match_deref! { match &(repl_exp) {
                    Deref @ Expression::CREF { ty: Deref @ Type::FUNCTION { r#fn: __esc_fn, .. }, .. } => {
                        r#fn = (*__esc_fn).clone();
                        assign_variant_field!(call => Call::NFCall::TYPED_CALL;
                            arguments = mergeFunctionApplicationArgs(&(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone()), var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone(), metamodelica::AsArg::as_arg(&r#fn), metamodelica::nil(), &(metamodelica::nil()))?,
                            r#fn = r#fn.clone()
                        );
                        metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
                    },
                    Deref @ Expression::PARTIAL_FUNCTION_APPLICATION { argNames: __repl_exp_argNames, args: __repl_exp_args, r#fn: __repl_exp_fn, .. } => {
                        r#fn = ((Function::getCachedFuncs(ComponentRef::node(metamodelica::AsArg::as_arg(&__repl_exp_fn))?)?)).head().cloned()?;
                        assign_variant_field!(call => Call::NFCall::TYPED_CALL;
                            arguments = mergeFunctionApplicationArgs(&(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL).clone()), var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone(), &r#fn, __repl_exp_args.clone(), metamodelica::AsArg::as_arg(&__repl_exp_argNames))?,
                            r#fn = r#fn
                        );
                        metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
                    },
                    _ => exp,
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            } else {
                outExp = exp;
            }
            outExp
        }
        _ => exp,
    });
    Ok(outExp)
}

fn evaluateReplacement(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::MUTABLE { .. } => {
            Expression::applyMutable(&exp, &evaluateReplacement2)?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

fn evaluateReplacement2(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::MUTABLE { .. } => {
            Expression::applyMutable(&exp, &evaluateReplacement2)?;
            exp
        }
        Expression::RECORD {
            elements: __exp_elements,
            ..
        } => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut e in (__exp_elements.clone()).into_iter().cloned() {
                    let __x = evaluateReplacement2(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            exp
        }
        _ => {
            if (Expression::contains(exp.clone(), &move |__a0: metamodelica::Ref<
                Expression::NFExpression,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Expression::isEmpty(&__a0))
            })?) {
                exp
            } else {
                Ceval::evalExp(exp, &(Ceval::noTarget().clone()))?
            }
        }
    });
    Ok(exp)
}

fn mergeFunctionApplicationArgs(
    mut oldFn: &metamodelica::Ref<Function::Function>,
    mut oldArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut newFn: &metamodelica::Ref<Function::Function>,
    mut newArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut argNames: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut arg_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Expression::NFExpression>>,
    >;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    arg_map = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut s in &*newFn.slots.clone() {
        if (s.default).is_some() {
            UnorderedMap::add(
                InstNode::name(&s.node)?,
                Expression::unbox(Util::getOption(s.default.clone())?),
                arg_map.clone(),
            )?;
        }
    }
    args = oldArgs;
    for mut i in &*oldFn.inputs.clone() {
        UnorderedMap::add(
            InstNode::name(metamodelica::AsArg::as_arg(&i))?,
            Expression::unbox((args).head().cloned()?),
            arg_map.clone(),
        )?;
        args = (args).rest()?;
    }
    args = newArgs;
    for mut n in &**argNames {
        UnorderedMap::add(n.clone(), Expression::unbox((args).head().cloned()?), arg_map.clone())?;
        args = (args).rest()?;
    }
    for mut i in &*newFn.inputs.clone() {
        outArgs = metamodelica::cons(
            UnorderedMap::getOrFail(InstNode::name(metamodelica::AsArg::as_arg(&i))?, arg_map.clone())?,
            outArgs,
        );
    }
    outArgs = metamodelica::Dangerous::listReverseInPlace(outArgs);
    Ok(outArgs)
}

fn optimizeBody(
    mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = body;
    body = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut s in (body).into_iter().cloned() {
            let __x = Statement::map(s.clone(), &optimizeStatement)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(body)
}

fn optimizeStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    let () = (match &*stmt {
        Statement::FOR {
            body: __stmt_body,
            iterator: __stmt_iterator,
            ..
        } => {
            let mut iter_exp: metamodelica::Ref<Expression::NFExpression>;
            iter_exp = Expression::makeMutable(metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: InstNode::getType(__stmt_iterator.clone())?,
            }));
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                body = Statement::replaceIteratorList(__stmt_body.clone(), metamodelica::AsArg::as_arg(&__stmt_iterator), &iter_exp)?,
                iterator = metamodelica::Ref::new(InstNode::InstNode::ITERATOR_NODE { exp: iter_exp })
            );
            ()
        }
        _ => (),
    });
    Ok(stmt)
}

fn createResult(
    mut map: ArgumentMap,
    mut outputs: &metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut types: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let mut e: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    if ((outputs).len() as i32) == 1 {
        node = InstNode::fromHandle(&((outputs).head().cloned()?))?;
        exp = Ceval::evalExp(
            UnorderedMap::getOrFail(node.clone(), map)?,
            &(Ceval::noTarget().clone()),
        )?;
        exp = assertAssignedOutput(&(list![InstNode::name(&node)?]), exp, &(InstNode::info(&node)), true)?;
    } else {
        expl = metamodelica::nil();
        types = metamodelica::nil();
        for mut h in &**outputs {
            node = InstNode::fromHandle(metamodelica::AsArg::as_arg(&h))?;
            e = Ceval::evalExp(
                UnorderedMap::getOrFail(node.clone(), map.clone())?,
                &(Ceval::noTarget().clone()),
            )?;
            e = assertAssignedOutput(&(list![InstNode::name(&node)?]), e, &(InstNode::info(&node)), true)?;
            expl = metamodelica::cons(e, expl);
        }
        expl = metamodelica::Dangerous::listReverseInPlace(expl);
        types = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
            for mut e in (expl.clone()).into_iter().cloned() {
                let __x = Expression::typeOf(e.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        exp = metamodelica::Ref::new(Expression::NFExpression::TUPLE {
            ty: metamodelica::Ref::new(Type::NFType::TUPLE {
                types: types,
                names: None,
            }),
            elements: expl,
        });
    }
    Ok(exp)
}

fn assertAssignedOutput(
    mut name: &metamodelica::List<ArcStr>,
    mut value: metamodelica::Ref<Expression::NFExpression>,
    mut info: &SourceInfo,
    mut error: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut value: metamodelica::Ref<Expression::NFExpression> = value;
    let mut fields: metamodelica::List<metamodelica::Ref<Record::Field::Field>>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut name_str: ArcStr;
    let mut opt_indices: Option<ArcStr>;
    value = (match &*value {
        Expression::RECORD {
            elements: __value_elements,
            ty: __value_ty,
            ..
        } => {
            fields = Type::recordFields(metamodelica::AsArg::as_arg(&__value_ty));
            expl = metamodelica::nil();
            for mut e in &*__value_elements.clone() {
                let mut e = e.clone();
                e = assertAssignedOutput(
                    &(metamodelica::cons(Record::Field::name(&((fields).head().cloned()?)), name.clone())),
                    e,
                    info,
                    false,
                )?;
                expl = metamodelica::cons(e, expl);
                fields = (fields).rest()?;
            }
            assign_variant_field!(value => Expression::NFExpression::RECORD; elements = metamodelica::Dangerous::listReverseInPlace(expl));
            value
        }
        _ => {
            opt_indices = findUnassignedElement(&value, &(metamodelica::nil()))?;
            if (opt_indices).is_none() {
                return Ok(value);
            }
            name_str = stringDelimitList(name.clone().reverse(), literal!("."));
            name_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name_str);
                __mm_s.push_str(&*Util::getOption(opt_indices)?);
                ArcStr::from(__mm_s)
            };
            if error {
                Error::addSourceMessageAsError(Error::UNASSIGNED_FUNCTION_OUTPUT.clone(), list![name_str], info)?;
                return Err("fail");
            } else {
                Error::addSourceMessage(&(Error::UNASSIGNED_FUNCTION_OUTPUT.clone()), list![name_str], info)?;
            }
            Expression::makeZero(&(Expression::typeOf(value)))?
        }
        _ => value,
    });
    Ok(value)
}

fn findUnassignedElement(
    mut value: &metamodelica::Ref<Expression::NFExpression>,
    mut indices: &metamodelica::List<i32>,
) -> Result<Option<ArcStr>> {
    let mut indicesStr: Option<ArcStr>;
    indicesStr = (match &**value {
        Expression::EMPTY { .. } => Some(List::toStringCustom(
            indices.clone().reverse(),
            &fnptr!(intString, i32),
            literal!(""),
            literal!("["),
            literal!(", "),
            literal!("]"),
            false,
            0,
        )?),
        Expression::ARRAY { .. } => {
            indicesStr = None;
            for mut i in
                1..=metamodelica::arrayLength(var_field!((**value).elements, Expression::NFExpression::ARRAY).clone())
            {
                indicesStr = findUnassignedElement(
                    &(metamodelica::Dangerous::arrayGetNoBoundsChecking(
                        var_field!((**value).elements, Expression::NFExpression::ARRAY).clone(),
                        i,
                    )),
                    &(metamodelica::cons(i, indices.clone())),
                )?;
                if (indicesStr).is_some() {
                    break;
                }
            }
            indicesStr
        }
        _ => None,
    });
    Ok(indicesStr)
}

fn evaluateStatements(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    for mut s in &**stmts {
        ctrl = evaluateStatement(s.clone(), context)?;
        if ctrl != FlowControl::NEXT.clone() {
            if ctrl == FlowControl::CONTINUE.clone() {
                ctrl = FlowControl::NEXT.clone();
            }
            break;
        }
    }
    Ok(ctrl)
}

fn evaluateStatement(mut stmt: metamodelica::Ref<Statement::NFStatement>, mut context: i32) -> Result<FlowControl> {
    let mut ctrl: FlowControl;
    ctrl = (match &*stmt.clone() {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            source: __stmt_source,
            ..
        } => evaluateAssignment(
            metamodelica::AsArg::as_arg(&__stmt_lhs),
            __stmt_rhs.clone(),
            __stmt_source.clone(),
            context,
        )?,
        Statement::FOR {
            body: __stmt_body,
            iterator: __stmt_iterator,
            range: __stmt_range,
            source: __stmt_source,
            ..
        } => evaluateFor(
            metamodelica::AsArg::as_arg(&__stmt_iterator),
            __stmt_range.clone(),
            __stmt_body.clone(),
            __stmt_source.clone(),
            context,
        )?,
        Statement::IF {
            branches: __stmt_branches,
            source: __stmt_source,
        } => evaluateIf(
            metamodelica::AsArg::as_arg(&__stmt_branches),
            __stmt_source.clone(),
            context,
        )?,
        Statement::ASSERT {
            condition: __stmt_condition,
            source: __stmt_source,
            ..
        } => evaluateAssert(__stmt_condition.clone(), &stmt, __stmt_source.clone(), context)?,
        Statement::NORETCALL {
            exp: __stmt_exp,
            source: __stmt_source,
        } => evaluateNoRetCall(__stmt_exp.clone(), __stmt_source.clone(), context)?,
        Statement::WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            source: __stmt_source,
        } => evaluateWhile(
            __stmt_condition.clone(),
            metamodelica::AsArg::as_arg(&__stmt_body),
            __stmt_source.clone(),
            context,
        )?,
        Statement::RETURN { .. } => FlowControl::RETURN.clone(),
        Statement::BREAK { .. } => FlowControl::BREAK.clone(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFEvalFunction.evaluateStatement"));
                    __mm_s.push_str(&*literal!(" failed on "));
                    __mm_s.push_str(&*anyString(stmt));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFEvalFunction.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(ctrl)
}

fn evaluateAssignment(
    mut lhsExp: &metamodelica::Ref<Expression::NFExpression>,
    mut rhsExp: metamodelica::Ref<Expression::NFExpression>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    assignVariable(
        lhsExp,
        &(Ceval::evalExp(
            rhsExp,
            &(evalTargetFromSource(source, STATEMENT_CONTEXT.clone(), context)),
        )?),
    )?;
    Ok(ctrl)
}

pub(crate) fn assignVariable(
    mut variable: &metamodelica::Ref<Expression::NFExpression>,
    mut value: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (variable, value) {
        (Deref @ Expression::MUTABLE { exp: var_ptr }, _) => {
            Mutable::update(var_ptr.clone(), assignExp(Mutable::access(var_ptr.clone()), value.clone())?);
            ()
        },
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::TUPLE { elements: vals, .. }) => {
            let mut var: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
            let mut val: metamodelica::Ref<Expression::NFExpression>;
            let mut vals = (*vals).clone();
            for mut var in &*var_field!((**variable).elements, Expression::NFExpression::TUPLE).clone() {
                let mut var = var.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vals.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                val = metamodelica::Own::own(__pa0);
                vals = metamodelica::Own::own(__pa1);
                assignVariable(&var, &val)?;
            }
            ()
        },
        (Deref @ Expression::SUBSCRIPTED_EXP { exp: Deref @ Expression::MUTABLE { exp: var_ptr }, .. }, _) => {
            assignSubscriptedVariable(var_ptr.clone(), var_field!((**variable).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), value)?;
            ()
        },
        (Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. }, _) => {
            ()
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.assignVariable")); __mm_s.push_str(&*literal!(" failed on ")); __mm_s.push_str(&*Expression::toString(variable.clone())?); __mm_s.push_str(&*literal!(" := ")); __mm_s.push_str(&*Expression::toString(value.clone())?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFEvalFunction.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn assignSubscriptedVariable(
    mut variable: Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>,
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut value: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<()> {
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (subscripts).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), &(Ceval::noTarget().clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Mutable::update(
        variable.clone(),
        assignArrayElement(Mutable::access(variable), &subs, value)?,
    );
    Ok(())
}

fn assignArrayElement(
    mut arrayExp: metamodelica::Ref<Expression::NFExpression>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut value: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut sub: metamodelica::Ref<Expression::NFExpression>;
    let mut val: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut idx: i32;
    let mut subs: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut vals: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    result = (::match_deref::match_deref! { match &((arrayExp.clone(), subscripts.clone())) {
        (Deref @ Expression::ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::INDEX { index: sub }, tail: __esc_rest_subs }) if (Expression::isScalarLiteral(metamodelica::AsArg::as_arg(&sub))) => {
            rest_subs = (*__esc_rest_subs).clone();
            idx = Expression::toInteger(metamodelica::AsArg::as_arg(&sub))?;
            if (rest_subs).is_empty() {
                metamodelica::arrayUpdate(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), idx, value.clone())?;
            } else {
                metamodelica::arrayUpdate(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), idx, assignArrayElement(metamodelica::arrayGet(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), idx)?, metamodelica::AsArg::as_arg(&rest_subs), value)?)?;
            }
            arrayExp
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::SLICE { slice: __esc_sub }, tail: __esc_rest_subs }) => {
            sub = (*__esc_sub).clone();
            rest_subs = (*__esc_rest_subs).clone();
            subs = Expression::arrayElements(metamodelica::AsArg::as_arg(&sub))?;
            vals = Expression::arrayElements(value)?;
            if metamodelica::arrayLength(subs.clone()) > metamodelica::arrayLength(vals.clone()) {
                return Err("fail");
            }
            if (rest_subs).is_empty() {
                for mut i in 1..=metamodelica::arrayLength(subs.clone()) {
                    sub = metamodelica::Dangerous::arrayGetNoBoundsChecking(subs.clone(), i);
                    val = metamodelica::Dangerous::arrayGetNoBoundsChecking(vals.clone(), i);
                    idx = Expression::toInteger(metamodelica::AsArg::as_arg(&sub))?;
                    metamodelica::arrayUpdate(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), idx, val)?;
                }
            } else {
                for mut i in 1..=metamodelica::arrayLength(subs.clone()) {
                    sub = metamodelica::Dangerous::arrayGetNoBoundsChecking(subs.clone(), i);
                    val = metamodelica::Dangerous::arrayGetNoBoundsChecking(vals.clone(), i);
                    idx = Expression::toInteger(metamodelica::AsArg::as_arg(&sub))?;
                    metamodelica::arrayUpdate(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), idx, assignArrayElement(metamodelica::arrayGet(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), idx)?, metamodelica::AsArg::as_arg(&rest_subs), &val)?)?;
                }
            }
            arrayExp
        },
        (Deref @ Expression::ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::WHOLE, tail: __esc_rest_subs }) => {
            rest_subs = (*__esc_rest_subs).clone();
            if (rest_subs).is_empty() {
                assign_variant_field!(arrayExp => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec(Expression::arrayElements(value)?.borrow().clone()));
            } else {
                assign_variant_field!(arrayExp => Expression::NFExpression::ARRAY; elements = metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let __thr_src0 = var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone();
        let __thr_borrow0 = __thr_src0.borrow();
        let mut __thr_it0 = __thr_borrow0.iter().cloned();
        let __thr_src1 = Expression::arrayElements(value)?;
        let __thr_borrow1 = __thr_src1.borrow();
        let mut __thr_it1 = __thr_borrow1.iter().cloned();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e), Some(v)) => {
                    let __x = assignArrayElement(e.clone(), metamodelica::AsArg::as_arg(&rest_subs), &(v.clone()))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    }).into_iter().cloned().collect()));
            }
            arrayExp
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.assignArrayElement")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(arrayExp)?); __mm_s.push_str(&*Subscript::toStringList(subscripts.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*Expression::toString(value.clone())?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFEvalFunction.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn assignExp(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*lhs {
        Expression::RECORD { .. } => assignRecord(lhs, rhs)?,
        _ => rhs,
    });
    Ok(result)
}

fn assignRecord(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &*rhs {
        Expression::RECORD {
            elements: __rhs_elements,
            ..
        } => {
            let mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(lhs.clone()) {
                Deref @ Expression::RECORD { elements: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elems = metamodelica::Own::own(__pa0);
            for mut v in &*__rhs_elements.clone() {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(elems) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa1);
                elems = metamodelica::Own::own(__pa2);
                assignVariable(&e, metamodelica::AsArg::as_arg(&v))?;
            }
            lhs
        }
        Expression::CREF {
            cref: __rhs_cref,
            ty: __rhs_ty,
        } => {
            let mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let mut val: metamodelica::Ref<Expression::NFExpression>;
            let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
            let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let __pa0 = ::match_deref::match_deref! { match &(lhs.clone()) {
                Deref @ Expression::RECORD { elements: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            elems = metamodelica::Own::own(__pa0);
            cls_tree = Class::classTree(InstNode::getClass(ComponentRef::node(metamodelica::AsArg::as_arg(
                &__rhs_cref,
            ))?)?)?;
            comps = ClassTree::getComponents(&cls_tree)?;
            let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range1 {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(elems) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa2);
                elems = metamodelica::Own::own(__pa3);
                ty = InstNode::getType(c.clone())?;
                val = metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: Type::liftArrayLeftList(ty.clone(), &(Type::arrayDims(__rhs_ty.clone()))),
                    cref: ComponentRef::prefixCref(c, ty, metamodelica::nil(), __rhs_cref.clone())?,
                });
                assignVariable(&e, &val)?;
            }
            lhs
        }
        _ => rhs,
    });
    Ok(result)
}

fn evaluateFor(
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut range: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut forBody: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    let mut range_iter: metamodelica::Ref<RangeIterator::NFRangeIterator>;
    let mut iter_exp: Mutable::Mutable<metamodelica::Ref<Expression::NFExpression>>;
    let mut range_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = forBody;
    let mut i: i32 = 0;
    let mut limit: i32 = Flags::getConfigInt(Flags::EVAL_LOOP_LIMIT.clone())?;
    range_exp = Ceval::evalExp(
        Util::getOption(range)?,
        &(evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context)),
    )?;
    range_iter = RangeIterator::fromExp(range_exp)?;
    if RangeIterator::hasNext(&range_iter)? {
        let __pa0 = ::match_deref::match_deref! { match &((*iterator)) {
            Deref @ InstNode::ITERATOR_NODE { exp: Deref @ Expression::MUTABLE { exp: __pa0 } } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        iter_exp = metamodelica::Own::own(__pa0);
        while RangeIterator::hasNext(&range_iter)? {
            (range_iter, value) = RangeIterator::next(range_iter)?;
            Mutable::update(iter_exp.clone(), value);
            ctrl = evaluateStatements(&body, context)?;
            if ctrl != FlowControl::NEXT.clone() {
                if ctrl == FlowControl::BREAK.clone() {
                    ctrl = FlowControl::NEXT.clone();
                }
                break;
            }
            i = i + 1;
            if i > limit {
                Error::addSourceMessage(
                    &(Error::EVAL_LOOP_LIMIT_REACHED.clone()),
                    list![ArcStr::from(::std::format!("{}", limit))],
                    &(ElementSource::getInfo(source.clone())),
                )?;
                return Err("fail");
            }
        }
    }
    Ok(ctrl)
}

fn evaluateIf(
    mut branches: &metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    for mut branch in &**branches {
        (cond, body) = branch.clone();
        if Expression::isTrue(
            &(Ceval::evalExp(
                cond,
                &(evalTargetFromSource(source.clone(), IF_COND_CONTEXT.clone(), context)),
            )?),
        ) {
            ctrl = evaluateStatements(&body, context)?;
            return Ok(ctrl);
        }
    }
    ctrl = FlowControl::NEXT.clone();
    Ok(ctrl)
}

fn evaluateAssert(
    mut condition: metamodelica::Ref<Expression::NFExpression>,
    mut assertStmt: &metamodelica::Ref<Statement::NFStatement>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    let mut msg: metamodelica::Ref<Expression::NFExpression>;
    let mut lvl: metamodelica::Ref<Expression::NFExpression>;
    let mut target: metamodelica::Ref<EvalTarget::EvalTarget> =
        evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context);
    if Expression::isFalse(&(Ceval::evalExp(condition, &target)?)) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*assertStmt)) {
            Deref @ Statement::ASSERT { message: __pa0, level: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        msg = metamodelica::Own::own(__pa0);
        lvl = metamodelica::Own::own(__pa1);
        msg = Ceval::evalExp(msg, &target)?;
        lvl = Ceval::evalExp(lvl, &target)?;
        let () = (::match_deref::match_deref! { match &((msg.clone(), lvl.clone())) {
            (Deref @ Expression::STRING { .. }, Deref @ Expression::ENUM_LITERAL { name: Deref @ "warning", .. }) => {
                Error::addSourceMessage(&(Error::ASSERT_TRIGGERED_WARNING.clone()), list![var_field!((*msg).value, Expression::NFExpression::STRING).clone()], &(Ceval::EvalTarget::getInfo(&target)))?;
                ()
            },
            (Deref @ Expression::STRING { .. }, Deref @ Expression::ENUM_LITERAL { name: Deref @ "error", .. }) => {
                Error::addSourceMessage(&(Error::ASSERT_TRIGGERED_ERROR.clone()), list![var_field!((*msg).value, Expression::NFExpression::STRING).clone()], &(Ceval::EvalTarget::getInfo(&target)))?;
                ctrl = FlowControl::ASSERTION.clone();
                ()
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFEvalFunction.evaluateAssert")); __mm_s.push_str(&*literal!(" failed to evaluate assert(false, ")); __mm_s.push_str(&*Expression::toString(msg)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(lvl)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFEvalFunction.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(ctrl)
}

fn evaluateNoRetCall(
    mut callExp: metamodelica::Ref<Expression::NFExpression>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    Ceval::evalExp(
        callExp,
        &(evalTargetFromSource(source, STATEMENT_CONTEXT.clone(), context)),
    )?;
    Ok(ctrl)
}

fn evaluateWhile(
    mut condition: metamodelica::Ref<Expression::NFExpression>,
    mut body: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
) -> Result<FlowControl> {
    let mut ctrl: FlowControl = FlowControl::NEXT.clone();
    let mut i: i32 = 0;
    let mut limit: i32 = Flags::getConfigInt(Flags::EVAL_LOOP_LIMIT.clone())?;
    let mut target: metamodelica::Ref<EvalTarget::EvalTarget> =
        evalTargetFromSource(source.clone(), STATEMENT_CONTEXT.clone(), context);
    while Expression::isTrue(&(Ceval::evalExp(condition.clone(), &target)?)) {
        ctrl = evaluateStatements(body, context)?;
        if ctrl != FlowControl::NEXT.clone() {
            if ctrl == FlowControl::BREAK.clone() {
                ctrl = FlowControl::NEXT.clone();
            }
            break;
        }
        i = i + 1;
        if i > limit {
            Error::addSourceMessage(
                &(Error::EVAL_LOOP_LIMIT_REACHED.clone()),
                list![ArcStr::from(::std::format!("{}", limit))],
                &(ElementSource::getInfo(source.clone())),
            )?;
            return Err("fail");
        }
    }
    Ok(ctrl)
}

fn evalTargetFromSource(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut context: i32,
    mut currentContext: i32,
) -> metamodelica::Ref<EvalTarget::EvalTarget> {
    let mut target: metamodelica::Ref<EvalTarget::EvalTarget> = Ceval::EvalTarget::new(
        ElementSource::getInfo(source.clone()),
        InstContext::set(context, currentContext),
        None,
    );
    target
}

fn evaluateExternal2(
    mut name: &ArcStr,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut extArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut map: ArgumentMap;
    let mut ext_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    map = createArgumentMap(&r#fn.inputs, &r#fn.outputs, &r#fn.locals, args, true, true)?;
    ext_args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (extArgs).into_iter().cloned() {
            let __x = Expression::map(
                e.clone(),
                (std::sync::Arc::new({
                    let __pe_b0 = map.clone();
                    move |__pe_a1| applyReplacements2(__pe_b0.clone(), __pe_a1)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    evaluateExternal3(name, &ext_args)?;
    result = createResult(map, &r#fn.outputs)?;
    Ok(result)
}

fn evaluateExternal3(
    mut name: &ArcStr,
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "dgeev" => {
            EvalFunctionExt::Lapack_dgeev(args)?;
            ()
        },
        Deref @ "dgegv" => {
            EvalFunctionExt::Lapack_dgegv(args)?;
            ()
        },
        Deref @ "dgels" => {
            EvalFunctionExt::Lapack_dgels(args)?;
            ()
        },
        Deref @ "dgelsx" => {
            EvalFunctionExt::Lapack_dgelsx(args)?;
            ()
        },
        Deref @ "dgelsy" => {
            EvalFunctionExt::Lapack_dgelsy(args)?;
            ()
        },
        Deref @ "dgesv" => {
            EvalFunctionExt::Lapack_dgesv(args)?;
            ()
        },
        Deref @ "dgglse" => {
            EvalFunctionExt::Lapack_dgglse(args)?;
            ()
        },
        Deref @ "dgtsv" => {
            EvalFunctionExt::Lapack_dgtsv(args)?;
            ()
        },
        Deref @ "dgbsv" => {
            EvalFunctionExt::Lapack_dgtsv(args)?;
            ()
        },
        Deref @ "dgesvd" => {
            EvalFunctionExt::Lapack_dgesvd(args)?;
            ()
        },
        Deref @ "dgetrf" => {
            EvalFunctionExt::Lapack_dgetrf(args)?;
            ()
        },
        Deref @ "dgetrs" => {
            EvalFunctionExt::Lapack_dgetrs(args)?;
            ()
        },
        Deref @ "dgetri" => {
            EvalFunctionExt::Lapack_dgetri(args)?;
            ()
        },
        Deref @ "dgeqpf" => {
            EvalFunctionExt::Lapack_dgeqpf(args)?;
            ()
        },
        Deref @ "dorgqr" => {
            EvalFunctionExt::Lapack_dorgqr(args)?;
            ()
        },
        Deref @ "dhseqr" => {
            EvalFunctionExt::Lapack_dhseqr(args)?;
            ()
        },
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn callExternalFunction(
    mut extName: ArcStr,
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut extArgs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut outputRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut extAnnotation: Option<metamodelica::Ref<SCode::Annotation>>,
    mut debug: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut info: SourceInfo;
    let mut pkg_name: ArcStr;
    let mut mapped_args: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut specs: metamodelica::Array<FFI::ArgSpec>;
    let mut ret_ty: metamodelica::Ref<Type::NFType>;
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut output_vals: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut fn_handle: i32;
    info = InstNode::info(&(InstNode::fromHandle(&r#fn.node)?));
    checkExtReturnValue(outputRef, &info)?;
    pkg_name = InstNode::name(&(InstNode::libraryScope(InstNode::fromHandle(&r#fn.node)?)?))?;
    fn_handle = loadLibraryFunction(&pkg_name, extName, extAnnotation, debug, &info)?;
    match '__try0: {
        (mapped_args, specs) = unwrap_break_err!(mapExternalArgs(r#fn, args.clone(), extArgs), '__try0);
        ret_ty = if (ComponentRef::isCref(outputRef)) {
            unwrap_break_err!(ComponentRef::nodeType(outputRef), '__try0)
        } else {
            crate::NFType::interned_NORETCALL()
        };
        (res, output_vals) = unwrap_break_err!(FFI::callFunction(fn_handle, mapped_args.clone(), specs.clone(), ret_ty.clone()), '__try0);
        unwrap_break_err!(freeLibraryFunction(fn_handle, debug), '__try0);
        Ok::<_, &'static str>((
            mapped_args.clone(),
            output_vals.clone(),
            res.clone(),
            ret_ty.clone(),
            specs.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            mapped_args = __try0_o0;
            output_vals = __try0_o1;
            res = __try0_o2;
            ret_ty = __try0_o3;
            specs = __try0_o4;
        }
        Err(__try0_err) => {
            freeLibraryFunction(fn_handle, debug)?;
            return Err(__try0_err);
        }
    }
    if (output_vals).is_empty() {
        result = res;
    } else {
        result = makeExternalResult(
            &(metamodelica::cons(res, output_vals)),
            outputRef,
            extArgs,
            r#fn.outputs.clone(),
        )?;
    }
    Ok(result)
}

fn lookupLibraryInCache(mut libName: &ArcStr) -> i32 {
    let mut libHandle: i32;
    let mut cache: metamodelica::List<(ArcStr, i32)>;
    let mut name: ArcStr;
    cache = openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| __root.borrow().clone());
    for mut l in &*cache {
        (name, libHandle) = l.clone();
        if metamodelica::stringEq(&name, &libName) {
            return libHandle;
        }
    }
    libHandle = -1;
    libHandle
}

fn cacheLibrary(mut libName: ArcStr, mut libHandle: i32) -> () {
    let mut cache: metamodelica::List<(ArcStr, i32)>;
    cache = openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| __root.borrow().clone());
    cache = metamodelica::cons((libName, libHandle), cache);
    {
        let __v = cache;
        openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub(crate) fn clearLibraryCache() -> Result<()> {
    let mut cache: metamodelica::List<(ArcStr, i32)>;
    let mut lib_handle: i32;
    cache = openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| __root.borrow().clone());
    for mut v in &*cache {
        (_, lib_handle) = v.clone();
        System::freeLibrary(lib_handle, false)?;
    }
    {
        let __v = metamodelica::nil();
        openmodelica_util::Globals::sharedLibraryCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

fn loadLibraryFunction(
    mut libName: &ArcStr,
    mut fnName: ArcStr,
    mut extAnnotation: Option<metamodelica::Ref<SCode::Annotation>>,
    mut debug: bool,
    mut info: &SourceInfo,
) -> Result<i32> {
    let mut fnHandle: i32 = -1;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    let mut libs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut dirs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut paths: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut libs2: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut failures: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut found: bool = false;
    let mut installLibDir: ArcStr;
    if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
        installLibDir = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
            __mm_s.push_str(&*literal!("/bin"));
            ArcStr::from(__mm_s)
        };
    } else {
        installLibDir = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
            __mm_s.push_str(&*literal!("/lib/"));
            __mm_s.push_str(&*arcstr::literal!(Autoconf::triple));
            __mm_s.push_str(&*literal!("/omc"));
            ArcStr::from(__mm_s)
        };
    }
    if (extAnnotation).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(extAnnotation) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        ann = metamodelica::Own::own(__pa0);
        libs = parseExternalAnnotation(&(literal!("Library")), &ann)?;
        dirs = parseExternalAnnotation(&(literal!("LibraryDirectory")), &ann)?;
    }
    dirs = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("modelica://"));
            __mm_s.push_str(&*libName);
            __mm_s.push_str(&*literal!("/Resources/Library"));
            ArcStr::from(__mm_s)
        },
        dirs,
    );
    libs = List::unique(&libs);
    dirs = List::unique(&dirs);
    for mut lib in &*libs {
        if !(stringEmpty(&lib)) {
            libs2 = metamodelica::cons(lib.clone(), libs2);
            libs2 = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("lib"));
                    __mm_s.push_str(&*lib);
                    ArcStr::from(__mm_s)
                },
                libs2,
            );
        }
    }
    libs = libs2;
    if !(Autoconf::isWasm.clone()) {
        for mut lib in &*libs {
            paths = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*installLibDir);
                    __mm_s.push_str(&*literal!("/ffi/"));
                    __mm_s.push_str(&*lib);
                    __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt));
                    ArcStr::from(__mm_s)
                },
                paths,
            );
        }
    }
    for mut lib in &*libs {
        let mut lib = lib.clone();
        if stringEmpty(&lib) {
            paths = metamodelica::cons(literal!(""), paths);
            continue;
        }
        lib = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*lib);
            __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt));
            ArcStr::from(__mm_s)
        };
        for mut dir in &*dirs {
            paths = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*dir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*lib);
                    ArcStr::from(__mm_s)
                },
                paths,
            );
            paths = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*dir);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*System::modelicaPlatform());
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*lib);
                    ArcStr::from(__mm_s)
                },
                paths,
            );
            if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))
                || metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("darwin")))
                || Autoconf::isWasm.clone()
            {
                if !(stringEmpty(&(System::openModelicaPlatformAlternative()))) {
                    paths = metamodelica::cons(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*dir);
                            __mm_s.push_str(&*literal!("/"));
                            __mm_s.push_str(&*System::openModelicaPlatformAlternative());
                            __mm_s.push_str(&*literal!("/"));
                            __mm_s.push_str(&*lib);
                            ArcStr::from(__mm_s)
                        },
                        paths,
                    );
                }
                paths = metamodelica::cons(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*dir);
                        __mm_s.push_str(&*literal!("/"));
                        __mm_s.push_str(&*System::openModelicaPlatform());
                        __mm_s.push_str(&*literal!("/"));
                        __mm_s.push_str(&*lib);
                        ArcStr::from(__mm_s)
                    },
                    paths,
                );
            }
        }
        paths = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*installLibDir);
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*lib);
                ArcStr::from(__mm_s)
            },
            paths,
        );
    }
    if (libs).is_empty() || Autoconf::isWasm.clone() {
        paths = metamodelica::cons(literal!(""), paths);
    }
    ErrorExt::setCheckpoint(literal!("NFEvalFunction.loadLibraryFunction"));
    (fnHandle, found, failures) = searchLibraryPaths(&paths, fnName.clone(), false, debug)?;
    if !(found) {
        (fnHandle, found, failures) = searchLibraryPaths(&paths, fnName.clone(), true, debug)?;
    }
    ErrorExt::rollBack(literal!("NFEvalFunction.loadLibraryFunction"));
    if !(found) {
        Error::addSourceMessage(
            &(Error::EXTERNAL_FUNCTION_NOT_FOUND.clone()),
            list![fnName, stringDelimitList(failures, literal!("\n"))],
            info,
        )?;
        return Err("fail");
    }
    Ok(fnHandle)
}

fn searchLibraryPaths(
    mut paths: &metamodelica::List<ArcStr>,
    mut fnName: ArcStr,
    mut lazy: bool,
    mut debug: bool,
) -> Result<(i32, bool, metamodelica::List<ArcStr>)> {
    let mut fnHandle: i32 = -1;
    let mut found: bool = false;
    let mut failures: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut lib_handle: i32;
    let mut file: ArcStr;
    let mut reason: ArcStr;
    let mut resolved: bool;
    for mut path in &**paths {
        reason = literal!("");
        resolved = true;
        match '__try0: {
            file = if (stringEmpty(&path)) {
                literal!("")
            } else {
                unwrap_break_err!(uriToFilename(path.clone()), '__try0)
            };
            Ok::<_, &'static str>((file.clone(),))
        } {
            Ok((__try0_o0,)) => {
                file = __try0_o0;
            }
            Err(_) => {
                file = path.clone();
                resolved = false;
                reason = literal!("not a usable file name");
            }
        }
        if resolved {
            lib_handle = lookupLibraryInCache(&file);
            if lib_handle == -1 {
                match '__try1: {
                    lib_handle = if (lazy) {
                        unwrap_break_err!(System::loadLibraryLazy(file.clone(), false, debug), '__try1)
                    } else {
                        unwrap_break_err!(System::loadLibrary(file.clone(), false, debug), '__try1)
                    };
                    cacheLibrary(file.clone(), lib_handle);
                    Ok::<_, &'static str>((lib_handle.clone(),))
                } {
                    Ok((__try1_o0,)) => {
                        lib_handle = __try1_o0;
                    }
                    Err(_) => {
                        lib_handle = -1;
                        reason = System::getLoadLibraryError();
                        reason = if (stringEmpty(&reason)) {
                            literal!("cannot be loaded")
                        } else {
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("cannot be loaded: "));
                                __mm_s.push_str(&*reason);
                                ArcStr::from(__mm_s)
                            }
                        };
                    }
                }
            }
            if lib_handle != -1 {
                if '__try2: {
                    fnHandle = unwrap_break_err!(System::lookupFunction(lib_handle, fnName.clone()), '__try2);
                    found = true;
                    Ok::<(), &'static str>(())
                }
                .is_err()
                {
                    reason = literal!("loaded, but does not define it");
                }
            }
        }
        if found {
            break;
        }
        if !(stringEmpty(&file)) {
            failures = metamodelica::cons(describeLibraryFailure(file.clone(), reason)?, failures);
        }
    }
    failures = failures.reverse();
    Ok((fnHandle, found, failures))
}

fn describeLibraryFailure(mut file: ArcStr, mut reason: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  "));
        __mm_s.push_str(&*Testsuite::friendly(file.clone())?);
        ArcStr::from(__mm_s)
    };
    if !(System::regularFileExists(file)) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(" (no such file)"));
            ArcStr::from(__mm_s)
        };
    } else if !(stringEmpty(&reason)) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*Testsuite::friendly(reason)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

fn parseExternalAnnotation(
    mut name: &ArcStr,
    mut ann: &metamodelica::Ref<SCode::Annotation>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut strl: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut mods: metamodelica::List<metamodelica::Ref<SCode::Mod>>;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    mods = SCodeUtil::lookupAnnotations(ann, name);
    for mut m in &*mods {
        strl = (::match_deref::match_deref! { match &(m.clone()) {
            Deref @ SCode::Mod::MOD { binding: Some(__esc_exp), .. } => {
                exp = (*__esc_exp).clone();
                parseExternalAnnotationExp(metamodelica::AsArg::as_arg(&exp), strl)?
            },
            _ => strl,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(strl)
}

fn parseExternalAnnotationExp(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
    mut strl: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut strl: metamodelica::List<ArcStr> = strl;
    strl = (match &**exp {
        Absyn::Exp::STRING { value: __exp_value } => metamodelica::cons(__exp_value.clone(), strl),
        Absyn::Exp::ARRAY {
            arrayExp: __exp_arrayExp,
        } => List::fold(
            metamodelica::AsArg::as_arg(&__exp_arrayExp),
            &move |__a0: metamodelica::Ref<Absyn::Exp>, __a1: metamodelica::List<ArcStr>| {
                parseExternalAnnotationExp(&__a0, __a1)
            },
            strl,
        )?,
        _ => strl,
    });
    Ok(strl)
}

fn freeLibraryFunction(mut fnHandle: i32, mut debug: bool) -> Result<()> {
    System::freeFunction(fnHandle, debug)?;
    Ok(())
}

fn mapExternalArgs(
    mut r#fn: &metamodelica::Ref<Function::Function>,
    mut inputArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut extArgs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::Array<FFI::ArgSpec>,
)> {
    let mut mappedArgs: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut argSpecs: metamodelica::Array<FFI::ArgSpec>;
    let mut arg_map: ArgumentMap;
    let mut marg: metamodelica::Ref<Expression::NFExpression>;
    let mut arg_spec: FFI::ArgSpec;
    let mut args_len: i32;
    let mut i: i32 = 1;
    let mut input_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    input_args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (inputArgs).into_iter().cloned() {
            let __x = makeExternalArg(arg.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    arg_map = createArgumentMap(&r#fn.inputs, &r#fn.outputs, &r#fn.locals, input_args, false, false)?;
    args_len = ((extArgs).len() as i32);
    mappedArgs = metamodelica::arrayCreate(
        args_len,
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
    );
    argSpecs = metamodelica::arrayCreate(args_len, FFI::ArgSpec::INPUT.clone());
    for mut ext_arg in &**extArgs {
        (marg, arg_spec) = mapExternalArg(ext_arg.clone(), arg_map.clone(), r#fn)?;
        {
            let __cell0 = marg;
            let __idx0 = i;
            let _ =
                unsafe { metamodelica::Dangerous::arrayInitSlotChecked(mappedArgs.clone().clone(), __idx0, __cell0) }?;
        }
        {
            let __cell1 = arg_spec;
            let __idx1 = i;
            let _ =
                unsafe { metamodelica::Dangerous::arrayInitSlotChecked(argSpecs.clone().clone(), __idx1, __cell1) }?;
        }
        i = i + 1;
    }
    Ok((mappedArgs, argSpecs))
}

fn makeExternalArg(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut extArg: metamodelica::Ref<Expression::NFExpression>;
    extArg = (match &*arg {
        Expression::FILENAME {
            filename: __arg_filename,
        } => metamodelica::Ref::new(Expression::NFExpression::STRING {
            value: __arg_filename.clone(),
        }),
        _ => arg,
    });
    extArg
}

fn mapExternalArg(
    mut extArg: metamodelica::Ref<Expression::NFExpression>,
    mut argMap: ArgumentMap,
    mut r#fn: &metamodelica::Ref<Function::Function>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, FFI::ArgSpec)> {
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut spec: FFI::ArgSpec;
    let mut cr_node: metamodelica::Ref<InstNode::InstNode>;
    arg = applyBindingReplacement(extArg.clone(), argMap)?;
    arg = Ceval::evalExp(arg, &(Ceval::noTarget().clone()))?;
    spec = (match &*extArg {
        Expression::CREF {
            cref: __extArg_cref, ..
        } => {
            cr_node = ComponentRef::node(&(ComponentRef::last(metamodelica::AsArg::as_arg(&__extArg_cref))))?;
            if InstNode::isProtected(&cr_node) {
                spec = FFI::ArgSpec::LOCAL.clone();
            } else if InstNode::isOutput(&cr_node) {
                spec = FFI::ArgSpec::OUTPUT.clone();
            } else {
                spec = FFI::ArgSpec::INPUT.clone();
            }
            spec
        }
        _ => FFI::ArgSpec::INPUT.clone(),
    });
    Ok((arg, spec))
}

fn makeExternalResult(
    mut values: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut outputRef: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut extArgs: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut outputs: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut arg_map: ArgumentMap;
    let mut val: metamodelica::Ref<Expression::NFExpression>;
    let mut vals: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut ret_vals: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    arg_map = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                InstNode::refEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<InstNode::InstNode>,
                        metamodelica::Ref<InstNode::InstNode>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*values)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    val = metamodelica::Own::own(__pa0);
    vals = metamodelica::Own::own(__pa1);
    if ComponentRef::isCref(outputRef) {
        UnorderedMap::addUnique(ComponentRef::node(outputRef)?, val, arg_map.clone())?;
    }
    for mut ext_arg in &**extArgs {
        let () = (match &*ext_arg.clone() {
            Expression::CREF {
                cref: __ext_arg_cref, ..
            } if (InstNode::isOutput(
                &(ComponentRef::node(&(ComponentRef::last(metamodelica::AsArg::as_arg(&__ext_arg_cref))))?),
            )) =>
            {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vals) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                val = metamodelica::Own::own(__pa0);
                vals = metamodelica::Own::own(__pa1);
                UnorderedMap::addUnique(
                    ComponentRef::node(metamodelica::AsArg::as_arg(&__ext_arg_cref))?,
                    val,
                    arg_map.clone(),
                )?;
                ()
            }
            _ => (),
        });
    }
    ret_vals = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut o in (outputs).into_iter().cloned() {
            let __x = getExternalOutputResult(InstNode::fromHandle(&(o.clone()))?, arg_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outExp = Expression::makeTuple(ret_vals)?;
    Ok(outExp)
}

fn getExternalOutputResult(
    mut outputNode: metamodelica::Ref<InstNode::InstNode>,
    mut map: ArgumentMap,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::END);
    let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut cls_node: metamodelica::Ref<InstNode::InstNode>;
    oexp = UnorderedMap::get(outputNode.clone(), map.clone())?;
    if (oexp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(oexp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
    } else if InstNode::isRecord(outputNode.clone())? {
        cls_node = InstNode::classScope(outputNode)?;
        comps = ClassTree::getComponents(&(Class::classTree(InstNode::getClass(cls_node.clone())?)?))?;
        expl = metamodelica::nil();
        let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
        for mut c in __range1 {
            expl = metamodelica::cons(getExternalOutputResult(c, map.clone())?, expl);
        }
        exp = Expression::makeRecord(
            InstNode::fullPath(cls_node.clone(), false)?,
            InstNode::getType(cls_node)?,
            metamodelica::Dangerous::listReverseInPlace(expl),
        );
    } else {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFEvalFunction.getExternalOutputResult"));
                __mm_s.push_str(&*literal!(" failed to find return value for output "));
                __mm_s.push_str(&*InstNode::name(&outputNode)?);
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFEvalFunction.mo")),
        )?;
    }
    Ok(exp)
}

fn checkExtReturnValue(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut info: &SourceInfo,
) -> Result<()> {
    if ComponentRef::isCref(cref) && Type::isRecord(&(ComponentRef::nodeType(cref)?)) {
        Error::addSourceMessage(
            &(Error::UNSUPPORTED_LANGUAGE_FEATURE.clone()),
            list![
                literal!("\"record return value in external function\""),
                literal!("Pass the record as an output parameter")
            ],
            info,
        )?;
        return Err("fail");
    }
    Ok(())
}
