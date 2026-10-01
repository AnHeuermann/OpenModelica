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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBModule as Module;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComponent as Component;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFModifier::Modifier;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBInline.mo
///  package:      NBInline
///  description:  This file contains functions for inlining operations.
pub struct NBInline<T>(std::marker::PhantomData<T>);
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
    mut inline_types: metamodelica::List<DAE::InlineType>,
    mut init: bool,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    bdae = (match &*bdae {
        BackendDAE::MAIN {
            eqData: __bdae_eqData,
            funcMap: __bdae_funcMap,
            varData: __bdae_varData,
            ..
        } => {
            let mut eqData: metamodelica::Ref<EqData::EqData>;
            let mut varData: metamodelica::Ref<VarData::VarData>;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(StringUtil::headline_4(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[dumpBackendInline] Inlining operatations for: "));
                        __mm_s.push_str(&*List::toString(
                            inline_types.clone(),
                            &fnptr!(DAEDump::dumpInlineTypeBackendStr, DAE::InlineType),
                            List::Style::FLAT_CURLY.clone(),
                        )?);
                        ArcStr::from(__mm_s)
                    }),
                )?);
            }
            (eqData, varData) = inline(
                __bdae_eqData.clone(),
                __bdae_varData.clone(),
                __bdae_funcMap.clone(),
                inline_types,
                init,
            )?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                eqData = eqData,
                varData = varData
            );
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(literal!("\n"));
            }
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBInline.main"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn inlineForEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: new_eqn, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } if (BEquation::Iterator::size(metamodelica::AsArg::as_arg(&__eqn_iter), false)? == 1 && !(BEquation::Iterator::isResizable(metamodelica::AsArg::as_arg(&__eqn_iter))?)) => {
            let mut replacements: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
            let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut range: metamodelica::Ref<Expression::NFExpression>;
            let mut start: i32;
            let mut new_eqn = (*new_eqn).clone();
            replacements = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            (names, ranges, _) = BEquation::Iterator::getFrames(metamodelica::AsArg::as_arg(&__eqn_iter));
            for mut tpl in &*List::zip(names, ranges) {
                (name, range) = tpl.clone();
                (start, _, _) = Expression::getIntegerRange(range, true)?;
                UnorderedMap::add(name, metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: start }), replacements.clone())?;
            }
            new_eqn = BEquation::Equation::map(new_eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = replacements; move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*literal!("NBInline.inlineForEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::toString(new_eqn.clone(), literal!(""))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            new_eqn.clone()
        },
        _ => {
            eqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

pub(crate) fn functionInlineable(mut r#fn: &metamodelica::Ref<Function::Function>) -> Result<bool> {
    let mut b: bool = false;
    if Function::hasSingleOrEmptyBody(r#fn) {
        b = (::match_deref::match_deref! { match &(Function::getBody(r#fn)?) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Statement::ASSIGNMENT { .. }, tail: Deref @ metamodelica::ListNode::Nil } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(b)
}

pub(crate) fn inlineRecordSliceEquation(
    mut slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
    mut inlineSimple: bool,
) -> Result<
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
> {
    let mut slices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut record_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        Pointer::create(metamodelica::nil());
    inlineRecordTupleArrayEquation(
        Pointer::access(Slice::getT(slice.clone())),
        &(crate::NBEquation::Iterator::interned_EMPTY()),
        variables,
        record_eqns.clone(),
        set,
        index,
        inlineSimple,
    )?;
    slices = ({
        let mut __acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        > = metamodelica::nil();
        for mut eqn in (Pointer::access(record_eqns)).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Slice::NBSlice {
                t: eqn.clone(),
                indices: metamodelica::nil(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (slices).is_empty() {
        slices = list![slice];
    }
    Ok(slices)
}

pub(crate) fn inlineArrayConstructorSingle(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<(metamodelica::Ref<Equation::Equation>, bool)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut changed: bool;
    match '__try0: {
        (eqn, changed) = (::match_deref::match_deref! { match &(eqn.clone()) {
            Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, attr: __eqn_attr, .. } => {
                (unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0), true)
            },
            Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, rhs: rhs @ Deref @ Expression::CREF { .. }, attr: __eqn_attr, .. } => {
                (unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0), true)
            },
            Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
                let mut new_eqn: metamodelica::Ref<Equation::Equation>;
                (new_eqn, changed) = unwrap_break_err!(inlineArrayConstructorSingle(body.clone(), metamodelica::AsArg::as_arg(&__eqn_iter), variables, set.clone(), index.clone(), new_eqns.clone()), '__try0);
                new_eqn = if (changed) {new_eqn.clone()} else {eqn.clone()};
                (new_eqn.clone(), changed)
            },
            _ => {
                (eqn.clone(), false)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        eqn = if (BEquation::Equation::isDummy(&eqn)) {
            Pointer::access(unwrap_break_err!(((Pointer::access(new_eqns.clone()))).head().cloned(), '__try0))
        } else {
            eqn.clone()
        };
        Ok::<_, &'static str>((changed.clone(),))
    } {
        Ok((__try0_o0,)) => {
            changed = __try0_o0;
        }
        Err(_) => {
            changed = false;
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Failed to inline following equation:\n"));
                    __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), literal!(""))?);
                    ArcStr::from(__mm_s)
                })?;
            }
        }
    }
    Ok((eqn, changed))
}

fn inline(
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut inline_types: metamodelica::List<DAE::InlineType>,
    mut init: bool,
) -> Result<(metamodelica::Ref<EqData::EqData>, metamodelica::Ref<VarData::VarData>)> {
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >;
    let mut set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >;
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> =
        BVariable::VarData::getVariables(&varData)?;
    let mut key: metamodelica::Ref<Absyn::Path>;
    let mut value: metamodelica::Ref<Function::Function>;
    let mut func_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<InlineRating::InlineRating>,
        >,
    > = UnorderedMap::new(
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Function::Function>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Function::nameHash(&__a0))
            },
        )
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Function::Function>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Function::Function>,
                  __a1: metamodelica::Ref<Function::Function>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Function::nameEqual(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Function::Function>,
                        metamodelica::Ref<Function::Function>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    replacements = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Absyn::Path>,
                  __a1: metamodelica::Ref<Absyn::Path>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    for mut tpl in &*UnorderedMap::toList(funcMap) {
        (key, value) = tpl.clone();
        if checkInline(value.clone(), &inline_types, func_map.clone())? {
            UnorderedMap::add(key, value, replacements.clone())?;
        }
    }
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())?
        && List::contains(
            &inline_types,
            openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE,
            &fnptr!(DAEUtil::inlineTypeEqual, DAE::InlineType, DAE::InlineType),
        )?
        && !(init)
    {
        metamodelica::print(StringUtil::headline_2(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "Heuristic results for Inline=default functions. Threshold = "
                ));
                __mm_s.push_str(&*intString(HEURISTIC_THRESHOLD.clone()));
                ArcStr::from(__mm_s)
            }),
        )?);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*UnorderedMap::toString(
                func_map,
                &({
                    let __pe_b1 = false;
                    move |__pe_a0| Function::signatureString(&__pe_a0, __pe_b1.clone())
                }),
                &move |__a0: metamodelica::Ref<InlineRating::InlineRating>| InlineRating::toString(&__a0),
                literal!("\n"),
                &(literal!(", ")),
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    eqData = propagateAttributes(eqData, &variables, replacements.clone())?;
    eqData = Replacements::replaceFunctions(eqData, &variables, replacements)?;
    set = UnorderedSet::new(
        (std::sync::Arc::new(BVariable::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(BVariable::equalName)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    if !(List::any(
        &inline_types,
        &({
            let __pe_b1 = openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE;
            move |__pe_a0| Ok(DAEUtil::inlineTypeEqual(__pe_a0, __pe_b1.clone()))
        }),
    )?) {
        eqData = inlineRecordsTuplesArrays(eqData, &variables, set.clone(), init)?;
    }
    eqData = BEquation::EqData::map(
        eqData,
        &({
            let __pe_b1 = variables.clone();
            let __pe_b2 = set.clone();
            move |__pe_a0| BackendDAE::lowerEquationIterators(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    varData = BVariable::VarData::addTypedList(
        varData,
        &(UnorderedSet::toList(set)),
        BVariable::VarData::VarType::ITERATOR.clone(),
    )?;
    eqData = BEquation::EqData::mapExp(
        eqData,
        (std::sync::Arc::new({
            let __pe_b1 = variables.clone();
            let __pe_b2 = true;
            move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        Some(
            (std::sync::Arc::new({
                let __pe_b1 = variables;
                let __pe_b2 = true;
                move |__pe_a0| BackendDAE::lowerComponentReference(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        )
                            -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                        + 'static,
                >),
        ),
    )?;
    eqData = BEquation::EqData::mapExp(
        eqData,
        (std::sync::Arc::new(replaceConstantArguments)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        None,
    )?;
    Ok((eqData, varData))
}

fn replaceConstantArguments(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_CALL; arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*call).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = replaceConstantArgument(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: call.clone() })
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn replaceConstantArgument(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { ty: Deref @ Type::COMPLEX { .. }, cref: __exp_cref } if (Type::isRecord(var_field!((*exp).ty, Expression::NFExpression::CREF)) && isVarCref(metamodelica::AsArg::as_arg(&__exp_cref))? && BVariable::checkCref(metamodelica::AsArg::as_arg(&__exp_cref), &fnptr!(BVariable::isRecord, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBInline.mo"))?) => {
            var_ptr = BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__exp_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBInline.mo"))?;
            children = BVariable::getRecordChildren(var_ptr)?;
            if List::any(&children, &fnptr!(BVariable::isConst, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>))? && List::compareLength(children, Type::recordFields(var_field!((*exp).ty, Expression::NFExpression::CREF)))? == 0 {
                elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut child in (BVariable::getRecordChildrenCref(metamodelica::AsArg::as_arg(&__exp_cref))?).into_iter().cloned() {
            let __x = Replacements::recordChildArg(child.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                new_exp = Expression::makeRecord(InstNode::fullPath(Type::complexNode(var_field!((*exp).ty, Expression::NFExpression::CREF))?, false)?, var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), elements);
            } else {
                new_exp = exp.clone();
            }
            new_exp
        },
        Deref @ Expression::CREF { cref: __exp_cref, .. } if (isVarCref(metamodelica::AsArg::as_arg(&__exp_cref))? && BVariable::checkCref(metamodelica::AsArg::as_arg(&__exp_cref), &fnptr!(BVariable::isConst, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBInline.mo"))?) => Replacements::recordChildArg(__exp_cref.clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn isVarCref(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = ComponentRef::isCref(cref) && InstNode::isVar(&(ComponentRef::node(cref)?));
    Ok(b)
}

fn propagateAttributes(
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<metamodelica::Ref<EqData::EqData>> {
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >;
    if UnorderedMap::isEmpty(replacements.clone()) {
        return Ok(eqData);
    }
    alias_map = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    eqData = BEquation::EqData::map(
        eqData,
        &({
            let __pe_b1 = variables.clone();
            let __pe_b2 = alias_map.clone();
            move |__pe_a0| collectFunctionAlias(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    eqData = BEquation::EqData::map(
        eqData,
        &({
            let __pe_b1 = variables.clone();
            let __pe_b2 = replacements;
            let __pe_b3 = alias_map;
            move |__pe_a0| propagateEquationAttributes(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    Ok(eqData)
}

fn collectFunctionAlias(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let () = (::match_deref::match_deref! { match &(&*eqn) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref: cr1, .. }, rhs: Deref @ Expression::CREF { cref: cr2, .. }, .. } => {
            addFunctionAlias(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2), variables, alias_map)?;
            ()
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref: cr1, .. }, rhs: Deref @ Expression::CREF { cref: cr2, .. }, .. } => {
            addFunctionAlias(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2), variables, alias_map)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn addFunctionAlias(
    mut cr1: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut cr2: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<()> {
    let mut n1: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(cr1);
    let mut n2: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(cr2);
    let mut f1: bool;
    let mut f2: bool;
    if !(BVariable::VariablePointers::containsCref(n1.clone(), variables)?
        && BVariable::VariablePointers::containsCref(n2.clone(), variables)?)
    {
        return Ok(());
    }
    f1 = BVariable::isFunctionAlias(BVariable::VariablePointers::getVarSafe(variables, n1.clone(), None)?)?;
    f2 = BVariable::isFunctionAlias(BVariable::VariablePointers::getVarSafe(variables, n2.clone(), None)?)?;
    if f1 && !(f2) {
        UnorderedMap::add(n1, n2, alias_map)?;
    } else if f2 && !(f1) {
        UnorderedMap::add(n2, n1, alias_map)?;
    }
    Ok(())
}

fn resolveAlias(
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    let mut tortoise: metamodelica::Ref<ComponentRef::NFComponentRef> = name.clone();
    let mut next: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    loop {
        next = UnorderedMap::get(name.clone(), alias_map.clone())?;
        if (next).is_none() {
            return Ok(name);
        }
        let __pa0 = ::match_deref::match_deref! { match &(next) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        next = UnorderedMap::get(name.clone(), alias_map.clone())?;
        if (next).is_none() {
            return Ok(name);
        }
        let __pa1 = ::match_deref::match_deref! { match &(next) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa1);
        tortoise = UnorderedMap::getOrFail(tortoise, alias_map.clone())?;
        if ComponentRef::isEqual(&tortoise, &name)? {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBInline.resolveAlias"));
                    __mm_s.push_str(&*literal!(
                        ": detected a cycle in the function-alias map while resolving '"
                    ));
                    __mm_s.push_str(&*ComponentRef::toString(&name)?);
                    __mm_s.push_str(&*literal!("'."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    }
    Ok(name)
}

fn propagateEquationAttributes(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let () = (match &*eqn {
        BEquation::Equation::SCALAR_EQUATION {
            lhs: __eqn_lhs,
            rhs: __eqn_rhs,
            ..
        } => {
            propagateOutput(
                __eqn_lhs.clone(),
                __eqn_rhs.clone(),
                variables,
                replacements.clone(),
                alias_map.clone(),
            )?;
            ()
        }
        BEquation::Equation::ARRAY_EQUATION {
            lhs: __eqn_lhs,
            rhs: __eqn_rhs,
            ..
        } => {
            propagateOutput(
                __eqn_lhs.clone(),
                __eqn_rhs.clone(),
                variables,
                replacements.clone(),
                alias_map.clone(),
            )?;
            ()
        }
        BEquation::Equation::RECORD_EQUATION {
            lhs: __eqn_lhs,
            rhs: __eqn_rhs,
            ..
        } => {
            propagateOutput(
                __eqn_lhs.clone(),
                __eqn_rhs.clone(),
                variables,
                replacements.clone(),
                alias_map.clone(),
            )?;
            ()
        }
        _ => (),
    });
    eqn = BEquation::Equation::map(
        eqn,
        (std::sync::Arc::new({
            let __pe_b1 = variables.clone();
            let __pe_b2 = replacements;
            let __pe_b3 = alias_map;
            move |__pe_a0| propagateInputExp(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        None,
        (std::sync::Arc::new(Expression::map)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(eqn)
}

fn propagateOutput(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<()> {
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut call_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    if Expression::isCref(&lhs) && isInlinableCall(&rhs, replacements.clone())? {
        cref_exp = lhs;
        call_exp = rhs;
    } else if Expression::isCref(&rhs) && isInlinableCall(&lhs, replacements.clone())? {
        cref_exp = rhs;
        call_exp = lhs;
    } else {
        return Ok(());
    }
    let __pa0 = ::match_deref::match_deref! { match &(call_exp) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    r#fn = Call::typedFunction(&call)?;
    r#fn = UnorderedMap::getOrFail(r#fn.path.clone(), replacements)?;
    if ((r#fn.outputs).len() as i32) == 1 {
        mergeNodeOntoArg(
            &(InstNode::fromHandle(&((r#fn.outputs).head().cloned()?))?),
            &cref_exp,
            variables,
            alias_map,
        )?;
    }
    Ok(())
}

fn propagateInputExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { r#fn, arguments: __esc_args, .. } } if (UnorderedMap::contains(r#fn.path.clone(), replacements.clone())?) => {
            call = (*__esc_call).clone();
            args = (*__esc_args).clone();
            let mut r#fn = (*r#fn).clone();
            r#fn = UnorderedMap::getOrFail(r#fn.path.clone(), replacements.clone())?;
            if ((r#fn.inputs).len() as i32) == ((args).len() as i32) {
                for mut tpl in &*List::zip(r#fn.inputs.clone(), args.clone()) {
                    mergeNodeOntoArg(&(Util::tuple21(tpl.clone())), &(Util::tuple22(tpl.clone())), variables, alias_map.clone())?;
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn isInlinableCall(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn, .. } } => {
            UnorderedMap::contains(r#fn.path.clone(), replacements)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn mergeNodeOntoArg(
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut arg: &metamodelica::Ref<Expression::NFExpression>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<()> {
    let mut node_children: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let () = (match &**arg {
        Expression::CREF { cref: __arg_cref, .. } => {
            mergeNodeOntoCref(
                node.clone(),
                metamodelica::AsArg::as_arg(&__arg_cref),
                variables,
                alias_map,
            )?;
            ()
        }
        _ => {
            node_children = nodeRecordChildren(node.clone())?;
            if !((node_children).is_empty()) {
                match '__try0: {
                    elems = unwrap_break_err!(Expression::getRecordElements(arg.clone()), '__try0);
                    Ok::<_, &'static str>((elems.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        elems = __try0_o0;
                    }
                    Err(_) => {
                        elems = metamodelica::nil();
                    }
                }
                if ((node_children).len() as i32) == ((elems).len() as i32) {
                    for mut tpl in &*List::zip(node_children, elems) {
                        mergeNodeOntoArg(
                            &(Util::tuple21(tpl.clone())),
                            &(Util::tuple22(tpl.clone())),
                            variables,
                            alias_map.clone(),
                        )?;
                    }
                }
            }
            ()
        }
    });
    Ok(())
}

fn mergeNodeOntoCref(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut alias_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<()> {
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut binfo: metamodelica::Ref<BackendExtension::BackendInfo::BackendInfo>;
    let mut rec_children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut node_children: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut cref_children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut src_attrs: metamodelica::Ref<BackendExtension::VariableAttributes::VariableAttributes>;
    name = ComponentRef::stripSubscriptsAll(cref);
    name = resolveAlias(name, alias_map.clone())?;
    if !(BVariable::VariablePointers::containsCref(name.clone(), variables)?) {
        return Ok(());
    }
    var_ptr = BVariable::VariablePointers::getVarSafe(variables, name.clone(), None)?;
    rec_children = BVariable::getRecordChildren(var_ptr.clone())?;
    if !((rec_children).is_empty()) {
        node_children = nodeRecordChildren(node)?;
        cref_children = BVariable::getRecordChildrenCref(&name)?;
        if ((node_children).len() as i32) == ((cref_children).len() as i32) {
            for mut tpl in &*List::zip(node_children, cref_children) {
                mergeNodeOntoCref(
                    Util::tuple21(tpl.clone()),
                    &(Util::tuple22(tpl.clone())),
                    variables,
                    alias_map.clone(),
                )?;
            }
        }
        return Ok(());
    }
    if '__try0: {
        src_attrs = unwrap_break_err!(nodeVariableAttributes(node.clone()), '__try0);
        var = Pointer::access(var_ptr.clone());
        binfo = var.backendinfo.clone();
        assign_field!(binfo.attributes = unwrap_break_err!(BackendExtension::VariableAttributes::merge(binfo.attributes.clone(), &src_attrs), '__try0));
        assign_field!(var.backendinfo = binfo.clone());
        Pointer::update(var_ptr.clone(), var.clone());
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(())
}

fn nodeRecordChildren(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    let mut children: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    children = (::match_deref::match_deref! { match &(Type::arrayElementType(&(InstNode::getType(node)?))) {
        __esc_elem_ty @ Deref @ Type::COMPLEX { .. } => {
            elem_ty = (*__esc_elem_ty).clone();
            Class::getComponents(InstNode::getClass(Type::complexNode(metamodelica::AsArg::as_arg(&elem_ty))?)?)?.borrow().iter().cloned().collect::<metamodelica::List<_>>()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(children)
}

fn nodeVariableAttributes(
    mut node: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<BackendExtension::VariableAttributes::VariableAttributes>> {
    let mut attrs: metamodelica::Ref<BackendExtension::VariableAttributes::VariableAttributes>;
    let mut comp: metamodelica::Ref<Component::NFComponent> = InstNode::component(&node)?;
    let mut ty_attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>;
    ty_attrs = ({
        let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
        for mut m in (Class::getTypeAttributes(InstNode::getClass(Component::classInstance(&comp)?)?))
            .into_iter()
            .cloned()
        {
            let __x = (Modifier::name(&(m.clone()))?, Modifier::binding(&(m.clone())));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    ty_attrs = List::filterOnTrue(
        ty_attrs,
        (std::sync::Arc::new(fnptr!(attrIsConst, (ArcStr, metamodelica::Ref<Binding::NFBinding>)))
            as std::sync::Arc<
                dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<Binding::NFBinding>)) -> Result<bool> + 'static,
            >),
    )?;
    attrs = BackendExtension::VariableAttributes::create(
        &ty_attrs,
        &(InstNode::getType(node)?),
        &(Component::getAttributes(&comp)),
        &(metamodelica::nil()),
        &(Component::comment(&comp)?),
    )?;
    Ok(attrs)
}

fn attrIsConst(mut attr: (ArcStr, metamodelica::Ref<Binding::NFBinding>)) -> bool {
    let mut b: bool;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    match '__try0: {
        exp = unwrap_break_err!(Binding::getTypedExp(&(Util::tuple22(attr.clone()))), '__try0);
        b = !(unwrap_break_err!(Expression::contains(exp.clone(), &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isCref(&__a0)) }), '__try0));
        Ok::<_, &'static str>((b.clone(),))
    } {
        Ok((__try0_o0,)) => {
            b = __try0_o0;
        }
        Err(_) => {
            b = false;
        }
    }
    b
}

fn inlineRecordsTuplesArrays(
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut init: bool,
) -> Result<metamodelica::Ref<EqData::EqData>> {
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut index: Pointer::Pointer<i32> = BEquation::EqData::getUniqueIndex(&eqData)?;
    let mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        Pointer::create(metamodelica::nil());
    if init {
        eqData = (match &*eqData {
            BEquation::EqData::EQ_DATA_SIM {
                initials: __eqData_initials,
                ..
            } => {
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::map(__eqData_initials.clone(), &({ let __pe_b1 = crate::NBEquation::Iterator::interned_EMPTY(); let __pe_b2 = variables.clone(); let __pe_b3 = new_eqns.clone(); let __pe_b4 = set; let __pe_b5 = index; let __pe_b6 = false; move |__pe_a0| inlineRecordTupleArrayEquation(__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }))?);
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::addList(&(Pointer::access(new_eqns)), var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone())?);
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::compress(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone())?);
                eqData
            }
            _ => eqData,
        });
    } else {
        eqData = BEquation::EqData::map(
            eqData,
            &({
                let __pe_b1 = crate::NBEquation::Iterator::interned_EMPTY();
                let __pe_b2 = variables.clone();
                let __pe_b3 = new_eqns.clone();
                let __pe_b4 = set;
                let __pe_b5 = index;
                let __pe_b6 = false;
                move |__pe_a0| {
                    inlineRecordTupleArrayEquation(
                        __pe_a0,
                        &__pe_b1,
                        &__pe_b2,
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_b5.clone(),
                        __pe_b6.clone(),
                    )
                }
            }),
        )?;
        eqData = BEquation::EqData::addUntypedList(eqData, &(Pointer::access(new_eqns)), false)?;
        eqData = BEquation::EqData::compress(eqData)?;
    }
    Ok(eqData)
}

pub(crate) fn inlineRecordTupleArrayEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
    mut inlineSimple: bool,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    if '__try0: {
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CREF { .. }, .. } if (!(inlineSimple)) => {
            eqn.clone()
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CREF { .. }, .. } if (!(inlineSimple)) => {
            eqn.clone()
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { ty: Deref @ Type::COMPLEX { .. }, attr: __eqn_attr, lhs: __eqn_lhs, recordSize: __eqn_recordSize, rhs: __eqn_rhs, .. } => {
            unwrap_break_err!(inlineRecordEquation(eqn.clone(), __eqn_lhs.clone(), __eqn_rhs.clone(), iter.clone(), __eqn_attr.clone(), __eqn_recordSize.clone(), variables, new_eqns.clone(), set.clone(), index.clone(), inlineSimple), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { recordSize: Some(size), attr: __eqn_attr, lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            unwrap_break_err!(inlineRecordEquation(eqn.clone(), __eqn_lhs.clone(), __eqn_rhs.clone(), iter.clone(), __eqn_attr.clone(), size.clone(), variables, new_eqns.clone(), set.clone(), index.clone(), inlineSimple), '__try0)
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { attr: __eqn_attr, lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            unwrap_break_err!(inlineTupleEquation(eqn.clone(), __eqn_lhs.clone(), __eqn_rhs.clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::ARRAY { .. }, rhs: rhs @ Deref @ Expression::ARRAY { .. }, attr: __eqn_attr, .. } => {
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: rhs @ Deref @ Expression::ARRAY { .. }, attr: __eqn_attr, .. } => {
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            dim = unwrap_break_err!(((Type::arrayDims(var_field!((**lhs).ty, Expression::NFExpression::CREF).clone()))).head().cloned(), '__try0);
            elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone())).into_iter() {
            let __x = unwrap_break_err!(Expression::applySubscripts(&(list![unwrap_break_err!(Subscript::nth(&dim, i.clone()), '__try0)]), lhs.clone(), true), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()), var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::ARRAY { .. }, rhs: rhs @ Deref @ Expression::CREF { .. }, attr: __eqn_attr, .. } => {
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            dim = unwrap_break_err!(((Type::arrayDims(var_field!((**rhs).ty, Expression::NFExpression::CREF).clone()))).head().cloned(), '__try0);
            elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone())).into_iter() {
            let __x = unwrap_break_err!(Expression::applySubscripts(&(list![unwrap_break_err!(Subscript::nth(&dim, i.clone()), '__try0)]), rhs.clone(), true), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone(), metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, attr: __eqn_attr, .. } => {
            unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, rhs: rhs @ Deref @ Expression::CREF { .. }, attr: __eqn_attr, .. } => {
            unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call }, attr: __eqn_attr, .. } if (metamodelica::stringEq(&(unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(&(unwrap_break_err!(Call::typedFunction(metamodelica::AsArg::as_arg(&call)), '__try0))), literal!("."), true, false), '__try0)), &(literal!("cat")))) => {
            unwrap_break_err!(inlineCatCall(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), &(unwrap_break_err!(Call::arguments(metamodelica::AsArg::as_arg(&call)), '__try0)), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call }, rhs: rhs @ Deref @ Expression::CREF { .. }, attr: __eqn_attr, .. } if (metamodelica::stringEq(&(unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(&(unwrap_break_err!(Call::typedFunction(metamodelica::AsArg::as_arg(&call)), '__try0))), literal!("."), true, false), '__try0)), &(literal!("cat")))) => {
            unwrap_break_err!(inlineCatCall(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), &(unwrap_break_err!(Call::arguments(metamodelica::AsArg::as_arg(&call)), '__try0)), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call }, attr: __eqn_attr, .. } if (metamodelica::stringEq(&(unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(&(unwrap_break_err!(Call::typedFunction(metamodelica::AsArg::as_arg(&call)), '__try0))), literal!("."), true, false), '__try0)), &(literal!("promote")))) => {
            unwrap_break_err!(inlinePromoteCall(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), &(unwrap_break_err!(Call::arguments(metamodelica::AsArg::as_arg(&call)), '__try0)), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call }, rhs: rhs @ Deref @ Expression::CREF { .. }, attr: __eqn_attr, .. } if (metamodelica::stringEq(&(unwrap_break_err!(AbsynUtil::pathString(Function::nameConsiderBuiltin(&(unwrap_break_err!(Call::typedFunction(metamodelica::AsArg::as_arg(&call)), '__try0))), literal!("."), true, false), '__try0)), &(literal!("promote")))) => {
            unwrap_break_err!(inlinePromoteCall(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), &(unwrap_break_err!(Call::arguments(metamodelica::AsArg::as_arg(&call)), '__try0)), __eqn_attr.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            let mut new_eqn: metamodelica::Ref<Equation::Equation>;
            new_eqn = unwrap_break_err!(inlineRecordTupleArrayEquation(body.clone(), metamodelica::AsArg::as_arg(&__eqn_iter), variables, new_eqns.clone(), set.clone(), index.clone(), true), '__try0);
            new_eqn = if (BEquation::Equation::isDummy(&new_eqn)) {new_eqn.clone()} else {eqn.clone()};
            new_eqn.clone()
        },
        Deref @ BEquation::Equation::IF_EQUATION { body: __eqn_body, .. } if (unwrap_break_err!(BEquation::IfEquationBody::isRecordOrTupleEquation(metamodelica::AsArg::as_arg(&__eqn_body)), '__try0)) => {
            let mut new_eqn: metamodelica::Ref<Equation::Equation>;
            new_eqn = unwrap_break_err!(inlineRecordTupleArrayIfEquation(eqn.clone(), __eqn_body.clone(), iter.clone(), variables, new_eqns.clone(), set.clone(), index.clone(), inlineSimple), '__try0);
            new_eqn = if (BEquation::Equation::isDummy(&new_eqn)) {new_eqn.clone()} else {eqn.clone()};
            new_eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<(), &'static str>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Error::addCompilerWarning({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to inline following equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), literal!(""))?); ArcStr::from(__mm_s) })?;
        }
    }
    Ok(eqn)
}

fn inlineRecordTupleArrayIfEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
    mut inlineSimple: bool,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut new_body: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    eqns = Pointer::access(new_eqns.clone());
    new_body = inlineRecordTupleArrayIfBody(body, &iter, variables, set, index.clone(), inlineSimple)?;
    for mut b in &*BEquation::IfEquationBody::split(new_body)? {
        new_eqn = BEquation::IfEquationBody::makeIfEquation(
            b.clone(),
            index.clone(),
            &(arcstr::literal!(BEquation::SIMULATION_STR)),
            iter.clone(),
            BEquation::Equation::getSource(eqn.clone()),
            BEquation::Equation::getAttributes(eqn.clone()),
        )?;
        eqns = metamodelica::cons(new_eqn, eqns);
    }
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlineRecordTupleArrayIfBody(
    mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
    mut inlineSimple: bool,
) -> Result<metamodelica::Ref<IfEquationBody::IfEquationBody>> {
    let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody> = body;
    let mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        Pointer::create(metamodelica::nil());
    assign_field!(
        body.then_eqns = List::flatten(
            ({
                let mut __acc: metamodelica::List<
                    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                > = metamodelica::nil();
                for mut e in (body.then_eqns.clone()).into_iter().cloned() {
                    let __x = (match &*(inlineRecordTupleArrayEquation(
                        Pointer::access(e.clone()),
                        iter,
                        variables,
                        new_eqns.clone(),
                        set.clone(),
                        index.clone(),
                        inlineSimple,
                    )?) {
                        BEquation::Equation::DUMMY_EQUATION => Pointer::access(new_eqns.clone()),
                        _ => list![e.clone()],
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        )?,
        body.else_if = Util::applyOption(
            body.else_if.clone(),
            &({
                let __pe_b1 = iter.clone();
                let __pe_b2 = variables.clone();
                let __pe_b3 = set;
                let __pe_b4 = index;
                let __pe_b5 = inlineSimple;
                move |__pe_a0| {
                    inlineRecordTupleArrayIfBody(
                        __pe_a0,
                        &__pe_b1,
                        &__pe_b2,
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_b5.clone(),
                    )
                }
            })
        )?
    );
    Ok(body)
}

fn inlineRecordEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut recordSize: i32,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
    mut inlineSimple: bool,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut new_lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut new_rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*literal!("NBInline.inlineRecordEquation"));
            __mm_s.push_str(&*literal!("] Inlining: "));
            ArcStr::from(__mm_s)
        });
        if !(BEquation::Iterator::isEmpty(&iter)) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                __mm_s.push_str(&*literal!("} "));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    eqns = Pointer::access(new_eqns.clone());
    for mut i in 1..=recordSize {
        new_lhs = inlineRecordConstructorExp(lhs.clone(), i, variables)?;
        new_rhs = inlineRecordConstructorExp(rhs.clone(), i, variables)?;
        eqns = createInlinedEquation(
            eqns,
            new_lhs,
            new_rhs,
            attr.clone(),
            iter.clone(),
            variables,
            set.clone(),
            index.clone(),
        )?;
    }
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlineTupleEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut LHS: metamodelica::Ref<Expression::NFExpression>,
    mut RHS: metamodelica::Ref<Expression::NFExpression>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut lhs_elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut rhs_elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    lhs_elems = getElementList(LHS)?;
    rhs_elems = getElementList(RHS)?;
    if !((lhs_elems).is_empty()) && List::compareLength(lhs_elems.clone(), rhs_elems.clone())? == 0 {
        if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n["));
                __mm_s.push_str(&*literal!("NBInline.inlineTupleEquation"));
                __mm_s.push_str(&*literal!("] Inlining: "));
                ArcStr::from(__mm_s)
            });
            if !(BEquation::Iterator::isEmpty(&iter)) {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("{"));
                    __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                    __mm_s.push_str(&*literal!("} "));
                    ArcStr::from(__mm_s)
                });
            }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        eqns = Pointer::access(new_eqns.clone());
        for mut tpl in &*List::zip(lhs_elems, rhs_elems) {
            (lhs, rhs) = tpl.clone();
            if !(Expression::isWildCref(&lhs) || Expression::isWildCref(&rhs)) {
                eqns = createInlinedEquation(
                    eqns,
                    lhs,
                    rhs,
                    attr.clone(),
                    iter.clone(),
                    variables,
                    set.clone(),
                    index.clone(),
                )?;
            }
        }
        Pointer::update(new_eqns, eqns);
        eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    }
    Ok(eqn)
}

fn inlineArrayEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut lhs_elements: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    mut rhs_elements: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let __ab_rhs_elements = rhs_elements.borrow();
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*literal!("NBInline.inlineArrayEquation"));
            __mm_s.push_str(&*literal!("] Inlining: "));
            ArcStr::from(__mm_s)
        });
        if !(BEquation::Iterator::isEmpty(&iter)) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                __mm_s.push_str(&*literal!("} "));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    eqns = Pointer::access(new_eqns.clone());
    for mut i in 1..=metamodelica::arrayLength(lhs_elements.clone()) {
        eqns = createInlinedEquation(
            eqns,
            ({
                let __elt = (*metamodelica::index_checked(&lhs_elements.borrow(), i)?).clone();
                __elt
            }),
            (*metamodelica::index_checked(&__ab_rhs_elements, i)?).clone(),
            attr.clone(),
            iter.clone(),
            variables,
            set.clone(),
            index.clone(),
        )?;
    }
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlineArrayConstructor(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut new_rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut local_set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = UnorderedSet::new(
        (std::sync::Arc::new(BVariable::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(BVariable::equalName)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    let mut local_it: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*literal!("NBInline.inlineArrayConstructor"));
            __mm_s.push_str(&*literal!("] Inlining: "));
            ArcStr::from(__mm_s)
        });
        if !(BEquation::Iterator::isEmpty(&iter)) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                __mm_s.push_str(&*literal!("} "));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    eqns = Pointer::access(new_eqns.clone());
    frames = ({
        let mut __acc: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        )> = metamodelica::nil();
        for mut iter in (iters).into_iter().cloned() {
            let __x = BEquation::Iterator::createFrame(iter.clone(), local_set.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    UnorderedSet::merge(set.clone(), local_set.clone())?;
    subs = BEquation::Iterator::normalizedSubscripts(
        &(BEquation::Iterator::fromFrames(frames.clone())),
        UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
    )?;
    cref_exp = Expression::fromCref(ComponentRef::mergeSubscripts(subs, cref, true, false, false)?, false)?;
    local_it = BVariable::VariablePointers::fromList(&(UnorderedSet::toList(local_set)), false)?;
    cref_exp = Expression::map(
        cref_exp,
        (std::sync::Arc::new({
            let __pe_b1 = local_it.clone();
            let __pe_b2 = false;
            move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    new_rhs = Expression::map(
        rhs,
        (std::sync::Arc::new({
            let __pe_b1 = local_it;
            let __pe_b2 = false;
            move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    eqns = createInlinedEquation(
        eqns,
        cref_exp,
        new_rhs,
        attr,
        BEquation::Iterator::addFrames(iter, frames),
        variables,
        set,
        index,
    )?;
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlinePromoteCall(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut n: i32;
    let mut dim_count: i32;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*literal!("NBInline.inlinePromoteCall"));
            __mm_s.push_str(&*literal!("] Inlining: "));
            ArcStr::from(__mm_s)
        });
        if !(BEquation::Iterator::isEmpty(&iter)) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                __mm_s.push_str(&*literal!("} "));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    arg = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    eqn = (match &*arg {
        Expression::CREF { cref: __arg_cref, .. } => {
            dim_count = Type::dimensionCount(ComponentRef::getSubscriptedType(
                metamodelica::AsArg::as_arg(&__arg_cref),
                false,
            )?);
            if n == dim_count {
                lhs = Expression::fromCref(cref, false)?;
            } else {
                subs = Subscript::fillWithWholeLeft(
                    List::fill(
                        metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                        }),
                        n - dim_count,
                    ),
                    n,
                );
                lhs = Expression::fromCref(ComponentRef::mergeSubscripts(subs, cref, false, false, false)?, false)?;
            }
            new_eqn = BEquation::Equation::makeAssignment(
                lhs,
                arg,
                index,
                &(arcstr::literal!(BEquation::SIMULATION_STR)),
                iter,
                attr,
            )?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-- Result: "));
                    __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), literal!(""))?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            Pointer::access(new_eqn)
        }
        _ => eqn,
    });
    Ok(eqn)
}

fn inlineCatCall(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut n: i32;
    let mut sz: i32;
    let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut iterator_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut iterator_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut update_vars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut subscript_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut lhs_sub: metamodelica::Ref<Expression::NFExpression>;
    let mut lhs_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut shift: metamodelica::Ref<Expression::NFExpression>;
    let mut new_size: metamodelica::Ref<Expression::NFExpression>;
    let mut local_iter: metamodelica::Ref<Iterator::Iterator>;
    let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut failed: bool = false;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n["));
            __mm_s.push_str(&*literal!("NBInline.inlineCatCall"));
            __mm_s.push_str(&*literal!("] Inlining: "));
            ArcStr::from(__mm_s)
        });
        if !(BEquation::Iterator::isEmpty(&iter)) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("{"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                __mm_s.push_str(&*literal!("} "));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    eqns = Pointer::access(new_eqns.clone());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: __pa0 }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    iterator_name = ComponentRef::makeIterator(
        InstNode::newUniqueIterator(
            Absyn::dummyInfo.clone(),
            openmodelica_nf_frontend::NFType::interned_INTEGER(),
        ),
        openmodelica_nf_frontend::NFType::interned_INTEGER(),
    )?;
    iterator_var = BackendDAE::lowerIterator(iterator_name)?;
    iterator_name = BVariable::getVarName(iterator_var.clone());
    update_vars = BVariable::VariablePointers::fromList(&(list![iterator_var.clone()]), false)?;
    UnorderedSet::add(iterator_var, set)?;
    subscript_exp = Expression::fromCref(iterator_name.clone(), false)?;
    shift = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
    for mut arg in &*rest {
        failed = (match &*arg.clone() {
            Expression::CREF { cref: __esc_rhs, .. } if (!(failed)) => {
                rhs = (*__esc_rhs).clone();
                ty = Expression::typeOf(arg.clone());
                if Type::isArray(&ty) {
                    dim = Type::nthDimension(ty, n)?;
                    sz = Dimension::size(&dim, false)?;
                    if sz != 1 || Dimension::isResizable(&dim) {
                        new_size = Dimension::sizeExp(&dim)?;
                        range = Expression::makeRange(
                            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                            None,
                            new_size.clone(),
                        )?;
                        local_iter =
                            BEquation::Iterator::addFrames(iter.clone(), list![(iterator_name.clone(), range, None)]);
                        lhs_sub = if (Expression::isZero(&shift)?) {
                            subscript_exp.clone()
                        } else {
                            metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                arguments: list![shift.clone(), subscript_exp.clone()],
                                inv_arguments: metamodelica::nil(),
                                operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()),
                            })
                        };
                        lhs = ComponentRef::mergeSubscripts(
                            Subscript::fillWithWholeLeft(
                                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: lhs_sub })],
                                n,
                            ),
                            cref.clone(),
                            false,
                            false,
                            false,
                        )?;
                        rhs = ComponentRef::mergeSubscripts(
                            Subscript::fillWithWholeLeft(
                                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                                    index: subscript_exp.clone()
                                })],
                                n,
                            ),
                            rhs.clone(),
                            false,
                            false,
                            false,
                        )?;
                        lhs_exp = Expression::map(
                            Expression::fromCref(lhs, false)?,
                            (std::sync::Arc::new({
                                let __pe_b1 = update_vars.clone();
                                let __pe_b2 = false;
                                move |__pe_a0| {
                                    BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
                                }
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Expression::NFExpression>,
                                        )
                                            -> Result<metamodelica::Ref<Expression::NFExpression>>
                                        + 'static,
                                >),
                        )?;
                        rhs_exp = Expression::map(
                            Expression::fromCref(rhs.clone(), false)?,
                            (std::sync::Arc::new({
                                let __pe_b1 = update_vars.clone();
                                let __pe_b2 = false;
                                move |__pe_a0| {
                                    BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
                                }
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Expression::NFExpression>,
                                        )
                                            -> Result<metamodelica::Ref<Expression::NFExpression>>
                                        + 'static,
                                >),
                        )?;
                    } else {
                        new_size = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 });
                        lhs_sub = bumpShift(shift.clone(), new_size.clone())?;
                        lhs = ComponentRef::mergeSubscripts(
                            Subscript::fillWithWholeLeft(
                                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: lhs_sub })],
                                n,
                            ),
                            cref.clone(),
                            false,
                            false,
                            false,
                        )?;
                        lhs_exp = Expression::fromCref(lhs, false)?;
                        rhs = ComponentRef::mergeSubscripts(
                            Subscript::fillWithWholeLeft(
                                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })
                                })],
                                n,
                            ),
                            rhs.clone(),
                            false,
                            false,
                            false,
                        )?;
                        rhs_exp = Expression::fromCref(rhs.clone(), false)?;
                        local_iter = iter.clone();
                    }
                } else {
                    new_size = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 });
                    lhs_sub = bumpShift(shift.clone(), new_size.clone())?;
                    lhs = ComponentRef::mergeSubscripts(
                        Subscript::fillWithWholeLeft(
                            list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: lhs_sub })],
                            n,
                        ),
                        cref.clone(),
                        false,
                        false,
                        false,
                    )?;
                    lhs_exp = Expression::fromCref(lhs, false)?;
                    rhs_exp = Expression::fromCref(rhs.clone(), false)?;
                    local_iter = iter.clone();
                }
                new_eqn = BEquation::Equation::makeAssignment(
                    lhs_exp,
                    rhs_exp,
                    index.clone(),
                    &(arcstr::literal!(BEquation::SIMULATION_STR)),
                    local_iter,
                    attr.clone(),
                )?;
                shift = bumpShift(shift, new_size)?;
                eqns = metamodelica::cons(new_eqn.clone(), eqns);
                if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("-- Result: "));
                        __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn, literal!(""))?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                false
            }
            Expression::ARRAY { .. } if (!(failed) && Expression::isLiteral(metamodelica::AsArg::as_arg(&arg))?) => {
                (eqns, shift) = inlineCatCallLiterals(
                    metamodelica::AsArg::as_arg(&arg),
                    &cref,
                    &iter,
                    &attr,
                    n,
                    index.clone(),
                    eqns,
                    shift,
                    &(metamodelica::nil()),
                )?;
                false
            }
            _ => true,
        });
    }
    if !(failed) {
        Pointer::update(new_eqns, eqns);
        eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    }
    Ok(eqn)
}

fn inlineCatCallLiterals(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut attr: &metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut n: i32,
    mut index: Pointer::Pointer<i32>,
    mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut shift: metamodelica::Ref<Expression::NFExpression>,
    mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = eqns;
    let mut shift: metamodelica::Ref<Expression::NFExpression> = shift;
    let () = (match &**exp {
        Expression::ARRAY { .. } => {
            let mut sub_idx: metamodelica::Ref<Expression::NFExpression>;
            let mut is_cat_dim: bool;
            let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
            is_cat_dim = n == ((subs).len() as i32) + 1;
            sub_idx = if (is_cat_dim) {
                bumpShift(
                    shift.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                )?
            } else {
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })
            };
            let __range0 = var_field!((**exp).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut elem in __range0 {
                sub = metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: sub_idx.clone() });
                (eqns, shift) = inlineCatCallLiterals(
                    &elem,
                    cref,
                    iter,
                    attr,
                    n,
                    index.clone(),
                    eqns,
                    shift,
                    &(metamodelica::cons(sub, subs.clone())),
                )?;
                sub_idx = bumpShift(
                    sub_idx,
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                )?;
            }
            if is_cat_dim {
                shift = bumpShift(
                    shift,
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                        value: metamodelica::arrayLength(
                            var_field!((**exp).elements, Expression::NFExpression::ARRAY).clone(),
                        ),
                    }),
                )?;
            }
            ()
        }
        _ => {
            let mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut lhs_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
            lhs = ComponentRef::mergeSubscripts(subs.clone().reverse(), cref.clone(), false, false, false)?;
            lhs_exp = Expression::fromCref(lhs, false)?;
            new_eqn = BEquation::Equation::makeAssignment(
                lhs_exp,
                exp.clone(),
                index,
                &(arcstr::literal!(BEquation::SIMULATION_STR)),
                iter.clone(),
                attr.clone(),
            )?;
            eqns = metamodelica::cons(new_eqn.clone(), eqns);
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-- Result: "));
                    __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn, literal!(""))?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            ()
        }
    });
    Ok((eqns, shift))
}

fn bumpShift(
    mut shift: metamodelica::Ref<Expression::NFExpression>,
    mut new_size: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut shift: metamodelica::Ref<Expression::NFExpression> = shift;
    shift = (::match_deref::match_deref! { match &((shift.clone(), new_size.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: var_field!((*shift).value, Expression::NFExpression::INTEGER).clone() + var_field!((*new_size).value, Expression::NFExpression::INTEGER).clone() })
        },
        (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value }, tail: args }, .. }, Deref @ Expression::INTEGER { .. }) if (Operator::getMathClassification(var_field!((*shift).operator, Expression::NFExpression::MULTARY))? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(shift => Expression::NFExpression::MULTARY; arguments = metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: value.clone() + var_field!((*new_size).value, Expression::NFExpression::INTEGER).clone() }), args.clone()));
            shift.clone()
        },
        (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: args }, .. }, _) if (Operator::getMathClassification(var_field!((*shift).operator, Expression::NFExpression::MULTARY))? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(shift => Expression::NFExpression::MULTARY; arguments = metamodelica::cons(arg.clone(), metamodelica::cons(new_size, args.clone())));
            shift.clone()
        },
        _ => {
            metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![shift.clone(), new_size], inv_arguments: metamodelica::nil(), operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(shift)
}

fn createInlinedEquation(
    mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = eqns;
    let mut tmp_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        Pointer::create(metamodelica::nil());
    let mut inlined: metamodelica::Ref<Equation::Equation>;
    let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    new_eqn = BEquation::Equation::makeAssignment(
        lhs,
        rhs,
        index.clone(),
        &(arcstr::literal!(BEquation::SIMULATION_STR)),
        iter.clone(),
        attr,
    )?;
    inlined = inlineRecordTupleArrayEquation(
        Pointer::access(new_eqn.clone()),
        &iter,
        variables,
        tmp_eqns.clone(),
        set,
        index,
        false,
    )?;
    eqns = (match &*inlined {
        BEquation::Equation::DUMMY_EQUATION => listAppend(eqns, Pointer::access(tmp_eqns)),
        _ => {
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-- Result: "));
                    __mm_s.push_str(&*BEquation::Equation::toString(inlined, literal!(""))?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            metamodelica::cons(new_eqn, eqns)
        }
    });
    Ok(eqns)
}

fn inlineRecordConstructorExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut index: i32,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::nthRecordElement(index, &exp)?;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(inlineRecordConstructorElements)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = variables.clone();
            let __pe_b2 = true;
            move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

fn inlineRecordConstructorElements(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn, .. } }, index: __exp_index, .. } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            if Function::isDefaultRecordConstructor(metamodelica::AsArg::as_arg(&r#fn))? {
                new_exp = (var_field!((**call).arguments, Call::NFCall::TYPED_CALL)).get(__exp_index.clone())?;
            } else if Function::isNonDefaultRecordConstructor(metamodelica::AsArg::as_arg(&r#fn)) {
                new_exp = (var_field!((**call).arguments, Call::NFCall::TYPED_CALL)).get(__exp_index.clone())?;
            } else {
                new_exp = exp;
            }
            new_exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn getElementList(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    elements = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TUPLE { elements: __exp_elements, .. } => {
            __exp_elements.clone()
        },
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: sub_exp @ Deref @ Expression::TUPLE { .. }, index: __exp_index, .. } => {
            let mut elem: metamodelica::Ref<Expression::NFExpression>;
            if __exp_index.clone() > ((var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE)).len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.getElementList")); __mm_s.push_str(&*literal!(" failed to get subscripted tuple element: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                return Err("fail");
            } else {
                elem = (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE)).get(__exp_index.clone())?;
            }
            list![elem]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn checkInline(
    mut func: metamodelica::Ref<Function::Function>,
    mut inline_types: &metamodelica::List<DAE::InlineType>,
    mut func_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<InlineRating::InlineRating>,
        >,
    >,
) -> Result<bool> {
    let mut b: bool;
    let mut it: DAE::InlineType = Function::inlineBuiltin(&func);
    b = List::contains(
        inline_types,
        it,
        &fnptr!(DAEUtil::inlineTypeEqual, DAE::InlineType, DAE::InlineType),
    )? && functionInlineable(&func)?;
    if b && DAEUtil::inlineTypeEqual(it, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE) {
        b = defaultHeuristic(func, func_map)?;
    }
    Ok(b)
}

pub(crate) const HEURISTIC_THRESHOLD: i32 = 10;

fn defaultHeuristic(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut func_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<InlineRating::InlineRating>,
        >,
    >,
) -> Result<bool> {
    let mut b: bool;
    b = InlineRating::resolve(&(InlineRating::fromFunction(r#fn, func_map)?))?
        < metamodelica::OrderedFloat((HEURISTIC_THRESHOLD.clone()) as f64);
    Ok(b)
}

pub mod InlineRating {
    use super::*;
    /// used to rate a function by how much it grows when inlining.
    ///    collects data about how often the inputs will occur and how much constant bloating inlining would cause.
    /// factors for each input with an additional constant overhead.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct InlineRating {
        pub input_rating: metamodelica::Array<i32>,
        pub constant_rating: i32,
    }

    impl metamodelica::gc::MMTrace for InlineRating {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.input_rating, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.constant_rating, __mmv)?;
            Ok(())
        }
    }
    impl Default for InlineRating {
        fn default() -> Self {
            Self {
                input_rating: Default::default(),
                constant_rating: Default::default(),
            }
        }
    }

    pub type INLINE_RATING = InlineRating;

    pub(crate) fn toString(mut ir: &metamodelica::Ref<InlineRating>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("{resolved: "));
            __mm_s.push_str(&*realString(resolve(ir)?));
            __mm_s.push_str(&*literal!(" | input: "));
            __mm_s.push_str(&*Array::toString(
                ir.input_rating.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!(" | constant: "));
            __mm_s.push_str(&*intString(ir.constant_rating.clone()));
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn resolve(mut ir: &metamodelica::Ref<InlineRating>) -> Result<metamodelica::Real> {
        let mut r: metamodelica::Real = metamodelica::real_div_checked(
            metamodelica::OrderedFloat(
                ({
                    let mut __acc: i32 = 0;
                    for mut v in (ir.input_rating.clone()).borrow().iter() {
                        let __x = v.clone();
                        __acc += __x;
                    }
                    __acc
                }) as f64,
            ),
            metamodelica::OrderedFloat((metamodelica::arrayLength(ir.input_rating.clone())) as f64),
        )? + intReal(ir.constant_rating.clone());
        Ok(r)
    }

    pub(crate) fn add(
        mut dst: metamodelica::Ref<InlineRating>,
        mut src: &metamodelica::Ref<InlineRating>,
    ) -> Result<metamodelica::Ref<InlineRating>> {
        let mut dst: metamodelica::Ref<InlineRating> = dst;
        if metamodelica::arrayLength(dst.input_rating.clone()) == metamodelica::arrayLength(src.input_rating.clone()) {
            for mut i in 1..=metamodelica::arrayLength(dst.input_rating.clone()) {
                {
                    let __cell0 = ({
                        let __elt = (*metamodelica::index_checked(&dst.input_rating.borrow(), i)?).clone();
                        __elt
                    }) + ({
                        let __elt = (*metamodelica::index_checked(&src.input_rating.borrow(), i)?).clone();
                        __elt
                    });
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut dst.input_rating.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
            assign_field!(dst.constant_rating = dst.constant_rating.clone() + src.constant_rating.clone());
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBInline.InlineRating.add"));
                    __mm_s.push_str(&*literal!(
                        " failed because dst and src input arrays are of different length.\n"
                    ));
                    __mm_s.push_str(&*literal!("dst: "));
                    __mm_s.push_str(&*toString(&dst)?);
                    __mm_s.push_str(&*literal!("\nsrc: "));
                    __mm_s.push_str(&*toString(src)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(dst)
    }

    pub(crate) fn multiply(
        mut ir: metamodelica::Ref<InlineRating>,
        mut i: i32,
    ) -> Result<metamodelica::Ref<InlineRating>> {
        let mut ir: metamodelica::Ref<InlineRating> = ir;
        for mut i in 1..=metamodelica::arrayLength(ir.input_rating.clone()) {
            {
                let __cell0 = i
                    * ({
                        let __elt = (*metamodelica::index_checked(&ir.input_rating.borrow(), i)?).clone();
                        __elt
                    });
                let __idx0 = i;
                *metamodelica::index_mut_checked(&mut ir.input_rating.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
        assign_field!(ir.constant_rating = i * ir.constant_rating.clone());
        Ok(ir)
    }

    pub(crate) fn addConst(mut ir: metamodelica::Ref<InlineRating>) -> metamodelica::Ref<InlineRating> {
        let mut ir: metamodelica::Ref<InlineRating> = ir;
        assign_field!(ir.constant_rating = ir.constant_rating.clone() + 1);
        ir
    }

    pub(crate) fn addMapped(
        mut dst: metamodelica::Ref<InlineRating>,
        mut src: &metamodelica::Ref<InlineRating>,
        mut args: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        mut local_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<InlineRating>,
            >,
        >,
    ) -> Result<metamodelica::Ref<InlineRating>> {
        let mut dst: metamodelica::Ref<InlineRating> = dst;
        let mut irp: Pointer::Pointer<metamodelica::Ref<InlineRating>> =
            Pointer::create(metamodelica::Ref::new(InlineRating {
                input_rating: arrayCreate(metamodelica::arrayLength(dst.input_rating.clone()), 0),
                constant_rating: src.constant_rating.clone(),
            }));
        if metamodelica::arrayLength(src.input_rating.clone()) == metamodelica::arrayLength(args.clone()) {
            for mut i in 1..=metamodelica::arrayLength(src.input_rating.clone()) {
                if ({
                    let __elt = (*metamodelica::index_checked(&src.input_rating.borrow(), i)?).clone();
                    __elt
                }) != 0
                {
                    Expression::map(
                        ({
                            let __elt = (*metamodelica::index_checked(&args.borrow(), i)?).clone();
                            __elt
                        }),
                        (std::sync::Arc::new({
                            let __pe_b1 = ({
                                let __elt = (*metamodelica::index_checked(&src.input_rating.borrow(), i)?).clone();
                                __elt
                            });
                            let __pe_b2 = irp.clone();
                            let __pe_b3 = local_map.clone();
                            move |__pe_a0| addMappedExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                    )?;
                }
            }
            dst = add(dst, &(Pointer::access(irp)))?;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBInline.InlineRating.addMapped"));
                    __mm_s.push_str(&*literal!(
                        " failed because src input array and arguments are of different length.\n"
                    ));
                    __mm_s.push_str(&*literal!("src: "));
                    __mm_s.push_str(&*toString(src)?);
                    __mm_s.push_str(&*literal!("\nargs: "));
                    __mm_s.push_str(&*Array::toString(
                        args.clone(),
                        &Expression::toString,
                        literal!(""),
                        literal!("["),
                        literal!(", "),
                        literal!("]"),
                        true,
                        0,
                    )?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(dst)
    }

    pub(crate) fn addMappedExp(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut i: i32,
        mut irp: Pointer::Pointer<metamodelica::Ref<InlineRating>>,
        mut local_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<InlineRating>,
            >,
        >,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let () = (match &*exp {
            Expression::CREF { cref: __exp_cref, .. } => {
                let mut iro: Option<metamodelica::Ref<InlineRating>>;
                iro = UnorderedMap::get(__exp_cref.clone(), local_map)?;
                if (iro).is_some() {
                    Pointer::update(
                        irp.clone(),
                        add(Pointer::access(irp), &(multiply(iro.ok_or("pattern mismatch")?, i)?))?,
                    );
                }
                ()
            }
            _ => (),
        });
        Ok(exp)
    }

    pub(crate) fn fromFunction(
        mut r#fn: metamodelica::Ref<Function::Function>,
        mut func_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Function::Function>, metamodelica::Ref<InlineRating>>,
        >,
    ) -> Result<metamodelica::Ref<InlineRating>> {
        let mut ir: metamodelica::Ref<InlineRating>;
        let mut irp: Pointer::Pointer<metamodelica::Ref<InlineRating>>;
        let mut lir: metamodelica::Ref<InlineRating>;
        let mut idx: i32 = 1;
        let mut num_inp: i32 = ((r#fn.inputs).len() as i32);
        let mut tmp: metamodelica::Ref<InlineRating>;
        let mut local_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<InlineRating>,
            >,
        > = UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        for mut inp in &*r#fn.inputs.clone() {
            tmp = metamodelica::Ref::new(InlineRating {
                input_rating: arrayCreate(num_inp, 0),
                constant_rating: 0,
            });
            {
                let __cell0 = 1;
                let __idx0 = idx;
                *metamodelica::index_mut_checked(&mut tmp.input_rating.clone().borrow_mut(), __idx0)? = __cell0;
            }
            idx = idx + 1;
            UnorderedMap::add(
                ComponentRef::fromNode(
                    inp.clone(),
                    InstNode::getType(inp.clone())?,
                    metamodelica::nil(),
                    ComponentRef::Origin::CREF.clone(),
                )?,
                tmp,
                local_map.clone(),
            )?;
        }
        for mut loc in &*r#fn.locals.clone() {
            irp = Pointer::create(metamodelica::Ref::new(InlineRating {
                input_rating: arrayCreate(num_inp, 0),
                constant_rating: 0,
            }));
            lir = (::match_deref::match_deref! { match &(InstNode::getBindingExpOpt(metamodelica::AsArg::as_arg(&loc))?) {
                Some(bind) => {
                    Expression::fakeMap(bind.clone(), &({ let __pe_b1 = func_map.clone(); let __pe_b2 = local_map.clone(); let __pe_b3 = irp.clone(); move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
                    Pointer::access(irp)
                },
                _ => {
                    Pointer::access(irp)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            UnorderedMap::add(
                ComponentRef::fromNode(
                    loc.clone(),
                    InstNode::getType(loc.clone())?,
                    metamodelica::nil(),
                    ComponentRef::Origin::CREF.clone(),
                )?,
                lir,
                local_map.clone(),
            )?;
        }
        irp = Pointer::create(metamodelica::Ref::new(InlineRating {
            input_rating: arrayCreate(num_inp, 0),
            constant_rating: 0,
        }));
        Expression::fakeMap(
            Function::getSingleBodyExp(&r#fn)?,
            &({
                let __pe_b1 = func_map.clone();
                let __pe_b2 = local_map;
                let __pe_b3 = irp.clone();
                move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
            }),
        )?;
        ir = Pointer::access(irp);
        UnorderedMap::add(r#fn, ir.clone(), func_map)?;
        Ok(ir)
    }

    pub(crate) fn rateExpression(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut func_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Function::Function>, metamodelica::Ref<InlineRating>>,
        >,
        mut local_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<InlineRating>,
            >,
        >,
        mut irp: Pointer::Pointer<metamodelica::Ref<InlineRating>>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut cont: bool;
        if Expression::isLiteral(&exp)? {
            Pointer::update(irp.clone(), addConst(Pointer::access(irp)));
        } else {
            cont = (match &*exp {
                Expression::CALL { call: __exp_call }
                    if (functionInlineable(&(Call::typedFunction(metamodelica::AsArg::as_arg(&__exp_call))?))?) =>
                {
                    let mut r#fn: metamodelica::Ref<Function::Function>;
                    let mut lir: Option<metamodelica::Ref<InlineRating>>;
                    r#fn = Call::typedFunction(metamodelica::AsArg::as_arg(&__exp_call))?;
                    lir = UnorderedMap::get(r#fn.clone(), func_map.clone())?;
                    if (lir).is_some() {
                        Pointer::update(
                            irp.clone(),
                            addMapped(
                                Pointer::access(irp.clone()),
                                &(lir.ok_or("pattern mismatch")?),
                                metamodelica::arrayFromVec(
                                    Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?
                                        .into_iter()
                                        .cloned()
                                        .collect(),
                                ),
                                local_map.clone(),
                            )?,
                        );
                        cont = false;
                    } else if DAEUtil::inlineTypeEqual(
                        Function::inlineBuiltin(&r#fn),
                        openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE,
                    ) {
                        Pointer::update(
                            irp.clone(),
                            addMapped(
                                Pointer::access(irp.clone()),
                                &(fromFunction(r#fn, func_map.clone())?),
                                metamodelica::arrayFromVec(
                                    Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?
                                        .into_iter()
                                        .cloned()
                                        .collect(),
                                ),
                                local_map.clone(),
                            )?,
                        );
                        cont = false;
                    } else {
                        cont = true;
                    }
                    cont
                }
                Expression::CREF { cref: __exp_cref, .. } => {
                    let mut lir: Option<metamodelica::Ref<InlineRating>>;
                    (::match_deref::match_deref! { match &(UnorderedMap::get(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref)), local_map.clone())?) {
                        __esc_lir @ Some(_) => {
                            lir = (*__esc_lir).clone();
                            Pointer::update(irp.clone(), add(Pointer::access(irp.clone()), &(lir.clone().ok_or("pattern mismatch")?))?);
                            false
                        },
                        _ => {
                            Pointer::update(irp.clone(), addConst(Pointer::access(irp.clone())));
                            false
                        },
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } })
                }
                _ => true,
            });
            if cont {
                exp = Expression::mapShallow(
                    exp,
                    (std::sync::Arc::new({
                        let __pe_b1 = func_map;
                        let __pe_b2 = local_map;
                        let __pe_b3 = irp;
                        move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
            }
        }
        Ok(exp)
    }
}
