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
use crate::NFCardinalityTable as CardinalityTable;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnector as Connector;
use crate::NFConnector::Face;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatten as Flatten;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStreamFlowAlias as StreamFlowAlias;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

thread_local! { static __EQ_ASSERT_STR_TLS: metamodelica::Ref<Expression::NFExpression> = metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("Connected constants/parameters must be equal") }); }
pub(crate) fn EQ_ASSERT_STR() -> metamodelica::Ref<Expression::NFExpression> {
    __EQ_ASSERT_STR_TLS.with(|__t| __t.clone())
}

pub(crate) fn generateEquations(
    mut sets: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
)> {
    type potFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            ) -> Result<(
                metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            )> + 'static,
    >;

    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut connectedLocalIOs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut unhandledStreamSets: metamodelica::List<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>> =
        metamodelica::nil();
    let mut set_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut potfunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            ) -> Result<(
                metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            )> + 'static,
    >;
    let mut flowThreshold: metamodelica::Ref<Expression::NFExpression>;
    let mut cty: i32;
    let mut flow_alias_elim: bool = Flags::isSet(Flags::FLOW_ALIAS_ELIMINATION.clone())?;
    {
        let __v = None;
        openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = __v)
    };
    connectedLocalIOs = UnorderedSet::new(
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
        13,
    );
    potfunc =
        (std::sync::Arc::new(
            move |__a0: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
                  __a1: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >| generatePotentialEquations(&__a0, __a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
                        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
                    ) -> Result<(
                        metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
                        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
                    )> + 'static,
            >);
    flowThreshold = metamodelica::Ref::new(Expression::NFExpression::REAL {
        value: Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?,
    });
    let __range0 = sets.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut set in __range0 {
        cty = getSetType(&set)?;
        if Prefixes::ConnectorType::isPotential(cty) {
            (set_eql, connectedLocalIOs) = potfunc(set, connectedLocalIOs)?;
        } else if Prefixes::ConnectorType::isFlow(cty) {
            set_eql = generateFlowEquations(set)?;
        } else if Prefixes::ConnectorType::isStream(cty) {
            if flow_alias_elim {
                unhandledStreamSets = metamodelica::cons(set, unhandledStreamSets);
                set_eql = metamodelica::nil();
            } else {
                set_eql = generateStreamEquations(&set, flowThreshold.clone(), variables.clone(), None)?;
            }
        } else {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFConnectEquations.generateEquations"));
                    __mm_s.push_str(&*literal!(" got connection set with invalid type '"));
                    __mm_s.push_str(&*Prefixes::ConnectorType::toDebugString(cty));
                    __mm_s.push_str(&*literal!("': "));
                    __mm_s.push_str(&*List::toString(
                        set,
                        &move |__a0: metamodelica::Ref<Connector::NFConnector>| Connector::toString(&__a0),
                        List::Style::FLAT_CURLY.clone(),
                    )?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFConnectEquations.mo"),
            )?;
            return Err("fail");
        }
        equations = listAppend(set_eql.clone(), equations);
    }
    unhandledStreamSets = metamodelica::Dangerous::listReverseInPlace(unhandledStreamSets);
    Ok((equations, connectedLocalIOs, unhandledStreamSets))
}

pub(crate) fn generateStreamEquationsList(
    mut sets: &metamodelica::List<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut set_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut flow_threshold: metamodelica::Ref<Expression::NFExpression>;
    flow_threshold = metamodelica::Ref::new(Expression::NFExpression::REAL {
        value: Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?,
    });
    for mut set in &**sets {
        set_eql = generateStreamEquations(
            metamodelica::AsArg::as_arg(&set),
            flow_threshold.clone(),
            variables.clone(),
            Some(replacements.clone()),
        )?;
        equations = listAppend(set_eql, equations);
    }
    Ok(equations)
}

pub(crate) fn evaluateOperators(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    use crate::NFOperator::Op;
    let mut evalExp: metamodelica::Ref<Expression::NFExpression>;
    evalExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call } => {
            (match &*call.clone() {
        Call::TYPED_CALL { arguments: __call_arguments, r#fn: __call_fn, .. } => (::match_deref::match_deref! { match &(Function::name(metamodelica::AsArg::as_arg(&__call_fn))) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" } => evaluateInStream(Expression::toCref(&((__call_arguments).head().cloned()?))?, sets, setsArray.clone(), variables, ctable, replacements)?,
        Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" } => {
            (evalExp, _) = evaluateActualStream(Expression::toCref(&((__call_arguments).head().cloned()?))?, sets, setsArray.clone(), variables, ctable, replacements)?;
            evalExp
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "cardinality" } => CardinalityTable::evaluateCardinality((__call_arguments).head().cloned()?, ctable)?,
        _ => evaluateOperatorsShallow(exp, sets, setsArray.clone(), variables, ctable, replacements)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        Call::TYPED_REDUCTION { exp: __call_exp, .. } if (Expression::contains(__call_exp.clone(), &move |__a0: metamodelica::Ref<Expression::NFExpression>| isStreamCall(&__a0))?) => evaluateOperatorReductionExp(&exp, sets, setsArray.clone(), variables, ctable, replacements)?,
        Call::TYPED_ARRAY_CONSTRUCTOR { exp: __call_exp, .. } if (Expression::contains(__call_exp.clone(), &move |__a0: metamodelica::Ref<Expression::NFExpression>| isStreamCall(&__a0))?) => evaluateOperatorArrayConstructorExp(exp, sets, setsArray.clone(), variables, ctable, replacements)?,
        _ => evaluateOperatorsShallow(exp, sets, setsArray.clone(), variables, ctable, replacements)?,
    })
        },
        Deref @ Expression::BINARY { exp1: Deref @ Expression::CREF { .. }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } } if (AbsynUtil::isNamedPathIdent(&(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL))), &(literal!("actualStream")))) => {
            evaluateActualStreamMul(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), &((var_field!((**call).arguments, Call::NFCall::TYPED_CALL)).head().cloned()?), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), sets, setsArray.clone(), variables, ctable, replacements)?
        },
        Deref @ Expression::BINARY { exp1: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::MUL, .. }, exp2: Deref @ Expression::CREF { .. } } if (AbsynUtil::isNamedPathIdent(&(Function::name(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL))), &(literal!("actualStream")))) => {
            evaluateActualStreamMul(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), &((var_field!((**call).arguments, Call::NFCall::TYPED_CALL)).head().cloned()?), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), sets, setsArray.clone(), variables, ctable, replacements)?
        },
        _ => {
            evaluateOperatorsShallow(exp, sets, setsArray.clone(), variables, ctable, replacements)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(evalExp)
}

fn evaluateOperatorsShallow(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut evalExp: metamodelica::Ref<Expression::NFExpression>;
    evalExp = Expression::mapShallow(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = sets.clone();
            let __pe_b2 = setsArray.clone();
            let __pe_b3 = variables;
            let __pe_b4 = ctable;
            let __pe_b5 = replacements;
            move |__pe_a0| {
                evaluateOperators(
                    __pe_a0,
                    &__pe_b1,
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                )
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(evalExp)
}

fn getSetType(mut set: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>) -> Result<i32> {
    let mut cty: i32;
    let __pa0 = ::match_deref::match_deref! { match &((*set)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { cty: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cty = metamodelica::Own::own(__pa0);
    Ok(cty)
}

fn generatePotentialEquations(
    mut elements: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut connectedLocalIOs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
)> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut connectedLocalIOs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = connectedLocalIOs;
    let mut c1: metamodelica::Ref<Connector::NFConnector>;
    c1 = (elements).head().cloned()?;
    if Connector::variability(&c1)? > Variability::PARAMETER.clone() {
        equations = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut c2 in ((elements).rest()?).into_iter().cloned() {
                let __x = makeEqualityEquation(
                    c1.name.clone(),
                    &(c1.source.clone()),
                    c2.name.clone(),
                    &(c2.source.clone()),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if Flags::getConfigInt(Flags::EXPOSE_LOCAL_IOS.clone())? > 0 {
            for mut c in &**elements {
                if Connector::isInside(metamodelica::AsArg::as_arg(&c))
                    && (ComponentRef::isInput(&c.name)? || ComponentRef::isOutput(&c.name)?)
                {
                    UnorderedSet::add(
                        (ComponentRef::stripSubscripts(c.name.clone())).0,
                        connectedLocalIOs.clone(),
                    )?;
                }
            }
        }
    } else {
        if Type::isEmptyArray(&c1.ty)? {
            equations = metamodelica::nil();
        } else {
            equations = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut c2 in ((elements).rest()?).into_iter().cloned() {
                    let __x = makeEqualityAssert(
                        c1.name.clone(),
                        &(c1.source.clone()),
                        c2.name.clone(),
                        &(c2.source.clone()),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        }
    }
    Ok((equations, connectedLocalIOs))
}

//function generatePotentialEquationsOrdered
//  "Like generatePotentialEquations, but orders the connectors with
//   shouldFlipPotentialEquation."
//  input list<Connector> elements;
//  output list<Equation> equations = {};
//protected
//  partial function eqFunc
//    input ComponentRef lhsCref;
//    input DAE.ElementSource lhsSource;
//    input ComponentRef rhsCref;
//    input DAE.ElementSource rhsSource;
//    output Equation eq;
//  end eqFunc;
//
//  Connector c1;
//  ComponentRef cr1, cr2;
//  DAE.ElementSource source;
//  eqFunc eqfunc;
//algorithm
//  if listEmpty(elements) then
//    return;
//  end if;
//
//  c1 := listHead(elements);
//  eqfunc := if Connector.variability(c1) > Variability.PARAMETER then
//    makeEqualityEquation else makeEqualityAssert;
//
//  cr1 := c1.name;
//
//  for c2 in listRest(elements) loop
//    cr2 := c2.name;
//    (cr1, cr2) := Util.swap(shouldFlipPotentialEquation(cr1, c1.source), cr1, cr2);
//    equations := eqfunc(cr1, c2.source, cr2, c2.source) :: equations;
//    c1 := c2;
//    cr1 := cr2;
//  end for;
//end generatePotentialEquationsOrdered;
fn makeEqualityEquation(
    mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut lhsSource: &metamodelica::Ref<DAE::ElementSource>,
    mut rhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhsSource: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut equalityEq: metamodelica::Ref<Equation::NFEquation>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = ElementSource::mergeSources(lhsSource, rhsSource)?;
    equalityEq = Equation::makeCrefEquality(
        lhsCref,
        rhsCref,
        crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        source,
    )?;
    Ok(equalityEq)
}

fn makeEqualityAssert(
    mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut lhsSource: &metamodelica::Ref<DAE::ElementSource>,
    mut rhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhsSource: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut equalityAssert: metamodelica::Ref<Equation::NFEquation>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut lhs_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut iterators: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    source = ElementSource::mergeSources(lhsSource, rhsSource)?;
    ty = ComponentRef::getSubscriptedType(&lhsCref, false)?;
    if Type::isArray(&ty) {
        (iterators, ranges, subs) = Flatten::makeIterators(&lhsCref, &(Type::arrayDims(ty.clone())))?;
        subs = metamodelica::Dangerous::listReverseInPlace(subs);
        lhs_exp = Expression::fromCref(
            ComponentRef::mergeSubscripts(subs.clone(), lhsCref, false, false, false)?,
            false,
        )?;
        rhs_exp = Expression::fromCref(
            ComponentRef::mergeSubscripts(subs, rhsCref, false, false, false)?,
            false,
        )?;
    } else {
        lhs_exp = Expression::fromCref(lhsCref, false)?;
        rhs_exp = Expression::fromCref(rhsCref, false)?;
    }
    elem_ty = Type::arrayElementType(&ty);
    if Type::isReal(&elem_ty)? {
        exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: lhs_exp,
            operator: Operator::makeSub(elem_ty.clone()),
            exp2: rhs_exp,
        });
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::ABS_REAL().clone(),
                list![exp.clone()],
                Expression::variability(exp)?,
                Purity::PURE.clone(),
                NFBuiltinFuncs::ABS_REAL().returnType.clone(),
            ),
        });
        exp = metamodelica::Ref::new(Expression::NFExpression::RELATION {
            exp1: exp,
            operator: Operator::makeLessEq(elem_ty),
            exp2: metamodelica::Ref::new(Expression::NFExpression::REAL {
                value: metamodelica::OrderedFloat(0.0_f64),
            }),
            index: -1,
        });
    } else {
        exp = metamodelica::Ref::new(Expression::NFExpression::RELATION {
            exp1: lhs_exp,
            operator: Operator::makeEqual(elem_ty),
            exp2: rhs_exp,
            index: -1,
        });
    }
    equalityAssert = metamodelica::Ref::new(Equation::NFEquation::ASSERT {
        condition: exp,
        message: EQ_ASSERT_STR().clone(),
        level: NFBuiltin::ASSERTIONLEVEL_ERROR().clone(),
        scope: NFInstNode::NO_SCOPE().clone(),
        source: source.clone(),
    });
    while !((iterators).is_empty()) {
        equalityAssert = metamodelica::Ref::new(Equation::NFEquation::FOR {
            iterator: (iterators).head().cloned()?,
            range: Some((ranges).head().cloned()?),
            body: list![equalityAssert],
            scope: NFInstNode::NO_SCOPE().clone(),
            source: source.clone(),
        });
        iterators = (iterators).rest()?;
        ranges = (ranges).rest()?;
    }
    Ok(equalityAssert)
}

//protected function shouldFlipPotentialEquation
//  "If the flag +orderConnections=false is used, then we should keep the order of
//   the connector elements as they occur in the connection (if possible). In that
//   case we check if the cref of the first argument to the first connection
//   stored in the element source is a prefix of the connector element cref. If
//   it isn't, indicate that we should flip the generated equation."
//  input DAE.ComponentRef lhsCref;
//  input DAE.ElementSource lhsSource;
//  output Boolean shouldFlip;
//algorithm
//  shouldFlip := match lhsSource
//    local
//      DAE.ComponentRef lhs;
//
//    case DAE.SOURCE(connectEquationOptLst = (lhs, _) :: _)
//      then not ComponentReferenceBasics.crefPrefixOf(lhs, lhsCref);
//
//    else false;
//  end match;
//end shouldFlipPotentialEquation;
fn generateFlowEquations(
    mut elements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut c: metamodelica::Ref<Connector::NFConnector>;
    let mut c_rest: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut sum: metamodelica::Ref<Expression::NFExpression>;
    let mut terms: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut iterators: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elements.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    c = metamodelica::Own::own(__pa0);
    c_rest = metamodelica::Own::own(__pa1);
    src = c.source.clone();
    if Connector::isArray(&c) {
        (iterators, ranges, subs) = Flatten::makeIterators(&(c.name.clone()), &(Type::arrayDims(c.ty.clone())))?;
        subs = metamodelica::Dangerous::listReverseInPlace(subs);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(({
            let mut __acc: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
            for mut e in (elements).into_iter().cloned() {
                let __x = Connector::addSubscripts(subs.clone(), e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
        c = metamodelica::Own::own(__pa2);
        c_rest = metamodelica::Own::own(__pa3);
    }
    if (c_rest).is_empty() {
        sum = Expression::fromCref(c.name.clone(), false)?;
    } else {
        terms = list![makeFlowExp(&c)?];
        for mut e in &*c_rest {
            terms = metamodelica::cons(makeFlowExp(metamodelica::AsArg::as_arg(&e))?, terms);
            src = ElementSource::mergeSources(&src, &e.source)?;
        }
        sum = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
            arguments: metamodelica::Dangerous::listReverseInPlace(terms),
            inv_arguments: metamodelica::nil(),
            operator: Operator::makeAdd(crate::NFType::interned_REAL()),
        });
    }
    equations = list![Equation::makeEquality(
        sum,
        metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64)
        }),
        Type::arrayElementType(&c.ty),
        src.clone(),
        crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        Equation::ScalarizeMode::NO_PREFERENCE.clone()
    )];
    while !((iterators).is_empty()) {
        equations = list![metamodelica::Ref::new(Equation::NFEquation::FOR {
            iterator: (iterators).head().cloned()?,
            range: Some((ranges).head().cloned()?),
            body: equations,
            scope: NFInstNode::NO_SCOPE().clone(),
            source: src.clone()
        })];
        iterators = (iterators).rest()?;
        ranges = (ranges).rest()?;
    }
    Ok(equations)
}

fn makeFlowExp(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut face: Face;
    exp = Expression::fromCref(element.name.clone(), false)?;
    face = element.face.clone();
    if face == Face::OUTSIDE.clone() {
        exp = metamodelica::Ref::new(Expression::NFExpression::UNARY {
            operator: Operator::makeUMinus(crate::NFType::interned_REAL()),
            exp: exp,
        });
    }
    Ok(exp)
}

fn generateStreamEquations(
    mut elements: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut cr1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cr2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut src1: metamodelica::Ref<DAE::ElementSource>;
    let mut src2: metamodelica::Ref<DAE::ElementSource>;
    let mut cref1: metamodelica::Ref<Expression::NFExpression>;
    let mut cref2: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut inside: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut outside: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    (outside, inside) = List::splitOnTrue(
        elements,
        &move |__a0: metamodelica::Ref<Connector::NFConnector>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Connector::isOutside(&__a0))
        },
    )?;
    inside = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
        for mut s in (inside).into_iter().cloned() {
            if !(!(isNoFlowInside(&(s.clone()), variables.clone(), None)?)) {
                continue;
            }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    equations = (::match_deref::match_deref! { match &((inside.clone(), outside.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => metamodelica::nil(),
        (Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => metamodelica::nil(),
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { name: __esc_cr1, source: __esc_src1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { name: __esc_cr2, source: __esc_src2, .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            cr1 = (*__esc_cr1).clone();
            src1 = (*__esc_src1).clone();
            cr2 = (*__esc_cr2).clone();
            src2 = (*__esc_src2).clone();
            cref1 = Expression::fromCref(cr1.clone(), false)?;
            cref2 = Expression::fromCref(cr2.clone(), false)?;
            e1 = makeInStreamCall(cref2.clone())?;
            e2 = makeInStreamCall(cref1.clone())?;
            src = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&src1), metamodelica::AsArg::as_arg(&src2))?;
            list![Equation::makeEquality(cref1, e1, crate::NFType::interned_REAL(), src.clone(), crate::NFInstNode::InstNode::interned_EMPTY_NODE(), Equation::ScalarizeMode::NO_PREFERENCE.clone()), Equation::makeEquality(cref2, e2, crate::NFType::interned_REAL(), src, crate::NFInstNode::InstNode::interned_EMPTY_NODE(), Equation::ScalarizeMode::NO_PREFERENCE.clone())]
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { name: __esc_cr1, source: __esc_src1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { name: __esc_cr2, source: __esc_src2, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            cr1 = (*__esc_cr1).clone();
            src1 = (*__esc_src1).clone();
            cr2 = (*__esc_cr2).clone();
            src2 = (*__esc_src2).clone();
            src = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&src1), metamodelica::AsArg::as_arg(&src2))?;
            list![Equation::makeCrefEquality(cr1.clone(), cr2.clone(), crate::NFInstNode::InstNode::interned_EMPTY_NODE(), src)?]
        },
        _ => streamEquationGeneral(outside, inside, flowThreshold, variables, replacements)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

fn streamEquationGeneral(
    mut outsideElements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut insideElements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut reduced_outside: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut outside: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    reduced_outside = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
        for mut s in (outsideElements.clone()).into_iter().cloned() {
            if !(!(isNoFlowOutside(&(s.clone()), variables.clone(), None)?)) {
                continue;
            }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut e in &*outsideElements {
        cref_exp = Expression::fromCref(e.name.clone(), false)?;
        outside = removeStreamSetElement(e.name.clone(), reduced_outside.clone())?;
        res = streamSumEquationExp(
            outside,
            insideElements.clone(),
            flowThreshold.clone(),
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
            variables.clone(),
            replacements.clone(),
        )?;
        src = ElementSource::addAdditionalComment(&e.source, literal!(" equation generated from stream connection"));
        equations = metamodelica::cons(
            Equation::makeEquality(
                cref_exp,
                res,
                crate::NFType::interned_REAL(),
                src,
                crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                Equation::ScalarizeMode::NO_PREFERENCE.clone(),
            ),
            equations,
        );
    }
    Ok(equations)
}

fn streamSumEquationExp(
    mut outsideElements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut insideElements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut fallback: metamodelica::Ref<Expression::NFExpression>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut sumExp: metamodelica::Ref<Expression::NFExpression>;
    let mut outside_sum1: metamodelica::Ref<Expression::NFExpression>;
    let mut outside_sum2: metamodelica::Ref<Expression::NFExpression>;
    let mut inside_sum1: metamodelica::Ref<Expression::NFExpression>;
    let mut inside_sum2: metamodelica::Ref<Expression::NFExpression>;
    let mut outside_non_negative: metamodelica::List<bool>;
    let mut inside_non_negative: metamodelica::List<bool>;
    (outside_non_negative, inside_non_negative) = setRequiresPositiveMax(
        outsideElements.clone(),
        insideElements.clone(),
        variables.clone(),
        replacements,
    )?;
    sumExp = (match ((outsideElements).is_empty(), (insideElements).is_empty()) {
        (true, true) => fallback,
        (true, false) => {
            inside_sum1 = sumMap(
                &insideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumInside1(&__a0, __a1, __a2, __a3),
                flowThreshold.clone(),
                &inside_non_negative,
                variables.clone(),
            )?;
            inside_sum2 = sumMap(
                &insideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumInside2(&__a0, __a1, __a2, __a3),
                flowThreshold,
                &inside_non_negative,
                variables,
            )?;
            sumExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: inside_sum1,
                operator: Operator::makeDiv(crate::NFType::interned_REAL()),
                exp2: inside_sum2,
            });
            makeInStreamDivCall(sumExp, fallback)?
        }
        (false, true) => {
            outside_sum1 = sumMap(
                &outsideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumOutside1(&__a0, __a1, __a2, __a3),
                flowThreshold.clone(),
                &outside_non_negative,
                variables.clone(),
            )?;
            outside_sum2 = sumMap(
                &outsideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumOutside2(&__a0, __a1, __a2, __a3),
                flowThreshold,
                &outside_non_negative,
                variables,
            )?;
            sumExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: outside_sum1,
                operator: Operator::makeDiv(crate::NFType::interned_REAL()),
                exp2: outside_sum2,
            });
            makeInStreamDivCall(sumExp, fallback)?
        }
        (false, false) => {
            outside_sum1 = sumMap(
                &outsideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumOutside1(&__a0, __a1, __a2, __a3),
                flowThreshold.clone(),
                &outside_non_negative,
                variables.clone(),
            )?;
            outside_sum2 = sumMap(
                &outsideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumOutside2(&__a0, __a1, __a2, __a3),
                flowThreshold.clone(),
                &outside_non_negative,
                variables.clone(),
            )?;
            inside_sum1 = sumMap(
                &insideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumInside1(&__a0, __a1, __a2, __a3),
                flowThreshold.clone(),
                &inside_non_negative,
                variables.clone(),
            )?;
            inside_sum2 = sumMap(
                &insideElements,
                &move |__a0: metamodelica::Ref<Connector::NFConnector>,
                       __a1: metamodelica::Ref<Expression::NFExpression>,
                       __a2: bool,
                       __a3: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >| sumInside2(&__a0, __a1, __a2, __a3),
                flowThreshold,
                &inside_non_negative,
                variables,
            )?;
            sumExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                exp1: metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: outside_sum1,
                    operator: Operator::makeAdd(crate::NFType::interned_REAL()),
                    exp2: inside_sum1,
                }),
                operator: Operator::makeDiv(crate::NFType::interned_REAL()),
                exp2: metamodelica::Ref::new(Expression::NFExpression::BINARY {
                    exp1: outside_sum2,
                    operator: Operator::makeAdd(crate::NFType::interned_REAL()),
                    exp2: inside_sum2,
                }),
            });
            makeInStreamDivCall(sumExp, fallback)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(sumExp)
}

fn setRequiresPositiveMax(
    mut outsideElements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut insideElements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<(metamodelica::List<bool>, metamodelica::List<bool>)> {
    let mut outsidePositiveMax: metamodelica::List<bool> = metamodelica::nil();
    let mut insidePositiveMax: metamodelica::List<bool> = metamodelica::nil();
    let mut positive_flow: bool;
    let mut non_negative_flow: bool;
    let mut any_positive_flow: bool = false;
    for mut c in &*outsideElements {
        (positive_flow, non_negative_flow) =
            hasFlowOutside(metamodelica::AsArg::as_arg(&c), variables.clone(), replacements.clone())?;
        any_positive_flow = any_positive_flow || positive_flow;
        outsidePositiveMax = metamodelica::cons(!(non_negative_flow), outsidePositiveMax);
    }
    for mut c in &*insideElements {
        (positive_flow, non_negative_flow) =
            hasFlowInside(metamodelica::AsArg::as_arg(&c), variables.clone(), replacements.clone())?;
        any_positive_flow = any_positive_flow || positive_flow;
        insidePositiveMax = metamodelica::cons(!(non_negative_flow), insidePositiveMax);
    }
    if any_positive_flow {
        outsidePositiveMax = metamodelica::Dangerous::listReverseInPlace(outsidePositiveMax);
        insidePositiveMax = metamodelica::Dangerous::listReverseInPlace(insidePositiveMax);
    } else {
        outsidePositiveMax = ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut c in (outsideElements).into_iter().cloned() {
                let __x = true;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        insidePositiveMax = ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut c in (insideElements).into_iter().cloned() {
                let __x = true;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    Ok((outsidePositiveMax, insidePositiveMax))
}

fn hasFlowOutside(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<(bool, bool)> {
    let mut positiveFlow: bool;
    let mut nonNegativeFlow: bool;
    (positiveFlow, nonNegativeFlow) = hasFlow(conn, true, variables, replacements)?;
    Ok((positiveFlow, nonNegativeFlow))
}

fn hasFlowInside(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<(bool, bool)> {
    let mut positiveFlow: bool;
    let mut nonNegativeFlow: bool;
    (positiveFlow, nonNegativeFlow) = hasFlow(conn, true, variables, replacements)?;
    Ok((positiveFlow, nonNegativeFlow))
}

fn hasFlow(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut isInside: bool,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<(bool, bool)> {
    let mut positiveFlow: bool = false;
    let mut nonNegativeFlow: bool = false;
    let mut oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut v: metamodelica::Ref<Variable::NFVariable>;
    let mut is_inside: bool = isInside;
    let mut negated: bool;
    (v, negated) = lookupFlowVarInConnector(conn, variables, replacements)?;
    if negated {
        is_inside = !(is_inside);
    }
    oexp = lookupAttrInVar(&(if (is_inside) { literal!("max") } else { literal!("min") }), &v);
    if (oexp).is_some() {
        exp = evaluateAttribute(oexp.ok_or("pattern mismatch")?)?;
        if is_inside {
            positiveFlow = Expression::isNegative(&exp)?;
            nonNegativeFlow = Expression::isNonPositive(&exp)?;
        } else {
            positiveFlow = Expression::isPositive(&exp)?;
            nonNegativeFlow = Expression::isNonNegative(&exp)?;
        }
    }
    if !(positiveFlow) {
        oexp = Binding::getExpOpt(&v.binding);
        positiveFlow = Util::applyOptionOrDefault(
            oexp,
            &*(if (is_inside) {
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| {
                    Expression::isNegative(&__a0)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                    >)
            } else {
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| {
                    Expression::isPositive(&__a0)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                    >)
            }),
            false,
        )?;
        nonNegativeFlow = nonNegativeFlow || positiveFlow;
    }
    Ok((positiveFlow, nonNegativeFlow))
}

fn sumMap(
    mut elements: &metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Connector::NFConnector>,
        metamodelica::Ref<Expression::NFExpression>,
        bool,
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Variable::NFVariable>,
            >,
        >,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut needsPositiveMax: &metamodelica::List<bool>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Connector::NFConnector>,
                metamodelica::Ref<Expression::NFExpression>,
                bool,
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Variable::NFVariable>,
                    >,
                >,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut needs_positive_max: bool;
    let mut needs_positive_max_rest: metamodelica::List<bool>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*needsPositiveMax)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    needs_positive_max = metamodelica::Own::own(__pa0);
    needs_positive_max_rest = metamodelica::Own::own(__pa1);
    exp = func(
        (elements).head().cloned()?,
        flowThreshold.clone(),
        needs_positive_max,
        variables.clone(),
    )?;
    for mut e in &*(elements).rest()? {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(needs_positive_max_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        needs_positive_max = metamodelica::Own::own(__pa2);
        needs_positive_max_rest = metamodelica::Own::own(__pa3);
        exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: func(e.clone(), flowThreshold.clone(), needs_positive_max, variables.clone())?,
            operator: Operator::makeAdd(crate::NFType::interned_REAL()),
            exp2: exp,
        });
    }
    Ok(exp)
}

fn streamFlowExp(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut streamExp: metamodelica::Ref<Expression::NFExpression>;
    let mut flowExp: metamodelica::Ref<Expression::NFExpression>;
    let mut stream_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    stream_cr = Connector::name(element);
    streamExp = Expression::fromCref(stream_cr.clone(), false)?;
    flowExp = Expression::fromCref(associatedFlowCref(stream_cr)?, false)?;
    Ok((streamExp, flowExp))
}

fn flowExp(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut flowExp: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    flow_cr = associatedFlowCref(Connector::name(element))?;
    flowExp = Expression::fromCref(flow_cr, false)?;
    Ok(flowExp)
}

fn sumOutside1(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut needsPositiveMax: bool,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stream_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_exp: metamodelica::Ref<Expression::NFExpression>;
    (stream_exp, flow_exp) = streamFlowExp(element)?;
    if needsPositiveMax {
        flow_exp = makePositiveMaxCall(flow_exp, element, flowThreshold, variables)?;
    }
    exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: flow_exp,
        operator: Operator::makeMul(crate::NFType::interned_REAL()),
        exp2: makeInStreamCall(stream_exp)?,
    });
    Ok(exp)
}

fn sumInside1(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut needsPositiveMax: bool,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stream_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_exp: metamodelica::Ref<Expression::NFExpression>;
    (stream_exp, flow_exp) = streamFlowExp(element)?;
    flow_exp = metamodelica::Ref::new(Expression::NFExpression::UNARY {
        operator: Operator::makeUMinus(crate::NFType::interned_REAL()),
        exp: flow_exp,
    });
    if needsPositiveMax {
        flow_exp = makePositiveMaxCall(flow_exp, element, flowThreshold, variables)?;
    }
    exp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: flow_exp,
        operator: Operator::makeMul(crate::NFType::interned_REAL()),
        exp2: stream_exp,
    });
    Ok(exp)
}

fn sumOutside2(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut needsPositiveMax: bool,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = flowExp(element)?;
    if needsPositiveMax {
        exp = makePositiveMaxCall(exp, element, flowThreshold, variables)?;
    }
    Ok(exp)
}

fn sumInside2(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut needsPositiveMax: bool,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = flowExp(element)?;
    exp = metamodelica::Ref::new(Expression::NFExpression::UNARY {
        operator: Operator::makeUMinus(crate::NFType::interned_REAL()),
        exp: exp,
    });
    if needsPositiveMax {
        exp = makePositiveMaxCall(exp, element, flowThreshold, variables)?;
    }
    Ok(exp)
}

fn makeInStreamCall(
    mut streamExp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut inStreamCall: metamodelica::Ref<Expression::NFExpression>;
    inStreamCall = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::IN_STREAM().clone(),
            list![streamExp.clone()],
            Expression::variability(streamExp)?,
            Purity::PURE.clone(),
            NFBuiltinFuncs::IN_STREAM().returnType.clone(),
        ),
    });
    Ok(inStreamCall)
}

fn makePositiveMaxCall(
    mut flowExp: metamodelica::Ref<Expression::NFExpression>,
    mut element: &metamodelica::Ref<Connector::NFConnector>,
    mut flowThreshold: metamodelica::Ref<Expression::NFExpression>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut positiveMaxCall: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nominal_oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut nominal_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_threshold: metamodelica::Ref<Expression::NFExpression>;
    let mut fn_node: metamodelica::Ref<InstNode::InstNode>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    flow_name = associatedFlowCref(Connector::name(element))?;
    nominal_oexp = lookupVarAttr(flow_name.clone(), &(literal!("nominal")), variables)?;
    if (nominal_oexp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(nominal_oexp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        nominal_exp = metamodelica::Own::own(__pa0);
        flow_threshold = metamodelica::Ref::new(Expression::NFExpression::BINARY {
            exp1: flowThreshold,
            operator: Operator::makeMul(crate::NFType::interned_REAL()),
            exp2: nominal_exp,
        });
    } else {
        flow_threshold = flowThreshold;
    }
    if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
        (fn_node, _) = Class::lookupElement(
            literal!("$OMC$PositiveMax"),
            NFInstNode::InstNode::getClass(NFInstNode::InstNode::topScope(ComponentRef::node(&flow_name)?)?)?,
        )?;
        fn_node = Function::instFunctionNode(fn_node, NFInstContext::NO_CONTEXT.clone(), Absyn::dummyInfo.clone())?;
        let __pa1 = ::match_deref::match_deref! { match &(Function::typeNodeCache(fn_node, NFInstContext::FUNCTION.clone())?) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        r#fn = metamodelica::Own::own(__pa1);
        positiveMaxCall = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                r#fn.clone(),
                list![flowExp, flow_threshold],
                Connector::variability(element)?,
                Purity::PURE.clone(),
                r#fn.returnType.clone(),
            ),
        });
    } else {
        positiveMaxCall = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::POSITIVE_MAX_REAL().clone(),
                list![flowExp, flow_threshold],
                Connector::variability(element)?,
                Purity::PURE.clone(),
                NFBuiltinFuncs::POSITIVE_MAX_REAL().returnType.clone(),
            ),
        });
    }
    {
        let __v = Some(true);
        openmodelica_util::Globals::isInStream.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(positiveMaxCall)
}

fn makeInStreamDivCall(
    mut sum_exp: metamodelica::Ref<Expression::NFExpression>,
    mut fallback: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut inStreamDivCall: metamodelica::Ref<Expression::NFExpression>;
    if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
        inStreamDivCall = sum_exp;
    } else {
        inStreamDivCall = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::INSTREAM_DIV_REAL().clone(),
                list![sum_exp, fallback.clone()],
                Expression::variability(fallback)?,
                Purity::PURE.clone(),
                NFBuiltinFuncs::INSTREAM_DIV_REAL().returnType.clone(),
            ),
        });
    }
    Ok(inStreamDivCall)
}

fn isStreamCall(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut streamCall: bool;
    streamCall = (match &**exp {
        Expression::CALL { call: __exp_call } => {
            (::match_deref::match_deref! { match &(Function::name(&(Call::typedFunction(metamodelica::AsArg::as_arg(&__exp_call))?))) {
                Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" } => true,
                Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" } => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(streamCall)
}

fn evaluateOperatorReductionExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut evalExp: metamodelica::Ref<Expression::NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut iter_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut iter_node: metamodelica::Ref<InstNode::InstNode>;
    evalExp = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            call = (*__esc_call).clone();
            ty = Expression::typeOf(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone());
            for mut iter in &*var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                (iter_node, iter_exp) = iter.clone();
                if Component::variability(&(NFInstNode::InstNode::component(&iter_node)?))? > Variability::PARAMETER.clone() {
                    metamodelica::print(literal!("Iteration range in reduction containing connector operator calls must be a parameter expression."));
                    return Err("fail");
                }
                iter_exp = Ceval::evalExp(iter_exp, &(Ceval::noTarget().clone()))?;
                ty = Type::liftArrayLeftList(ty, &(Type::arrayDims(Expression::typeOf(iter_exp.clone()))));
                iters = metamodelica::cons((iter_node, iter_exp), iters);
            }
            iters = metamodelica::Dangerous::listReverseInPlace(iters);
            (arg, _) = ExpandExp::expandArrayConstructor(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), ty, &iters)?;
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(var_field!((*call).r#fn, Call::NFCall::TYPED_REDUCTION).clone(), list![arg], var_field!((*call).var, Call::NFCall::TYPED_REDUCTION).clone(), Purity::PURE.clone(), var_field!((*call).ty, Call::NFCall::TYPED_REDUCTION).clone()) })
        },
        _ => return Err("match: no arm matched"),
    } });
    evalExp = evaluateOperators(evalExp, sets, setsArray.clone(), variables, ctable, replacements)?;
    Ok(evalExp)
}

fn evaluateOperatorArrayConstructorExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut evalExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expanded: bool;
    (evalExp, expanded) = ExpandExp::expand(exp.clone(), false, false)?;
    if !(expanded) {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFConnectEquations.evaluateOperatorArrayConstructorExp"));
                __mm_s.push_str(&*literal!(" failed to expand call containing stream operator: "));
                __mm_s.push_str(&*Expression::toString(exp)?);
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("NFFrontEnd/NFConnectEquations.mo"),
        )?;
    }
    evalExp = evaluateOperators(evalExp, sets, setsArray.clone(), variables, ctable, replacements)?;
    Ok(evalExp)
}

fn evaluateInStream(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut c: metamodelica::Ref<Connector::NFConnector>;
    let mut sl: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut set: i32;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    cr = ComponentRef::evaluateSubscripts(cref)?;
    c = metamodelica::Ref::new(Connector::NFConnector {
        name: cr.clone(),
        ty: crate::NFType::interned_UNKNOWN(),
        face: Face::INSIDE.clone(),
        cty: ConnectorType::STREAM.clone(),
        source: DAE::emptyElementSource().clone(),
    });
    match '__try0: {
        set = unwrap_break_err!(ConnectionSets::findSetArrayIndex(c.clone(), sets), '__try0);
        sl = unwrap_break_err!(metamodelica::arrayGet(setsArray.clone(), set), '__try0);
        Ok::<_, &'static str>((sl.clone(),))
    } {
        Ok((__try0_o0,)) => {
            sl = __try0_o0;
        }
        Err(_) => {
            sl = list![c.clone()];
        }
    }
    exp = generateInStreamExp(
        cr,
        sl,
        sets,
        setsArray.clone(),
        variables,
        ctable,
        replacements.clone(),
        Flags::getConfigReal(Flags::FLOW_THRESHOLD.clone())?,
    )?;
    if (replacements).is_some() {
        exp = StreamFlowAlias::applyReplacementsInExp(replacements.ok_or("pattern mismatch")?, exp)?;
    }
    Ok(exp)
}

fn generateInStreamExp(
    mut streamCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut streams: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
    mut flowThreshold: metamodelica::Real,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut reducedStreams: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut inside: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut outside: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut f1: Face;
    let mut f2: Face;
    reducedStreams = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = metamodelica::nil();
        for mut s in (streams).into_iter().cloned() {
            if !(!(isNoFlowMinMax(&(s.clone()), &streamCref, variables.clone(), replacements.clone())?)) {
                continue;
            }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    exp = (::match_deref::match_deref! { match &(reducedStreams.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { face: Connector::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Nil } => Expression::fromCref(streamCref, false)?,
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { face: Connector::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { face: Connector::Face::INSIDE, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref, reducedStreams)?) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { name: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            Expression::fromCref(cr, false)?
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { face: f1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { face: f2, .. }, tail: Deref @ metamodelica::ListNode::Nil } } if (f1.clone() != f2.clone()) => {
            let __pa0 = ::match_deref::match_deref! { match &(removeStreamSetElement(streamCref, reducedStreams)?) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Connector::CONNECTOR { name: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            evaluateInStream(cr, sets, setsArray.clone(), variables, ctable, replacements)?
        },
        _ => {
            (outside, inside) = List::splitOnTrue(&reducedStreams, &move |__a0: metamodelica::Ref<Connector::NFConnector>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Connector::isOutside(&__a0)) })?;
            inside = removeStreamSetElement(streamCref.clone(), inside)?;
            exp = streamSumEquationExp(outside, inside, metamodelica::Ref::new(Expression::NFExpression::REAL { value: flowThreshold }), Expression::fromCref(streamCref, false)?, variables.clone(), replacements.clone())?;
            exp = evaluateOperators(exp, sets, setsArray.clone(), variables, ctable, replacements)?;
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn isNoFlowMinMax(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut streamCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<bool> {
    let mut noFlow: bool;
    if ComponentRef::isEqual(streamCref, &conn.name)? {
        noFlow = false;
    } else if Connector::isOutside(conn) {
        noFlow = isNoFlowOutside(conn, variables, replacements)?;
    } else {
        noFlow = isNoFlowInside(conn, variables, replacements)?;
    }
    Ok(noFlow)
}

fn isNoFlowOutside(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<bool> {
    let mut noFlow: bool;
    noFlow = isNoFlow(conn, false, variables, replacements)?;
    Ok(noFlow)
}

fn isNoFlowInside(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<bool> {
    let mut noFlow: bool;
    noFlow = isNoFlow(conn, true, variables, replacements)?;
    Ok(noFlow)
}

fn isNoFlow(
    mut element: &metamodelica::Ref<Connector::NFConnector>,
    mut isInside: bool,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<bool> {
    let mut noFlow: bool;
    let mut attr_oexp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut attr_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut v: metamodelica::Ref<Variable::NFVariable>;
    let mut is_inside: bool = isInside;
    let mut negated: bool;
    (v, negated) = lookupFlowVarInConnector(element, variables, replacements)?;
    if negated {
        is_inside = !(is_inside);
    }
    attr_oexp = lookupAttrInVar(&(if (is_inside) { literal!("min") } else { literal!("max") }), &v);
    if (attr_oexp).is_some() {
        attr_exp = evaluateAttribute(attr_oexp.ok_or("pattern mismatch")?)?;
        noFlow = if (is_inside) {
            Expression::isNonNegative(&attr_exp)?
        } else {
            Expression::isNonPositive(&attr_exp)?
        };
    } else {
        noFlow = Util::applyOptionOrDefault(
            Binding::getExpOpt(&v.binding),
            &move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isZero(&__a0),
            false,
        )?;
    }
    Ok(noFlow)
}

fn evaluateAttribute(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut var: Variability;
    var = Expression::variability(exp.clone())?;
    if var == Variability::PARAMETER.clone() && !(Structural::isExpressionNotFixed(&exp, false, 4)?) {
        Structural::markExp(&exp)?;
        var = Variability::STRUCTURAL_PARAMETER.clone();
    }
    if var <= Variability::STRUCTURAL_PARAMETER.clone() {
        exp = Ceval::evalExp(exp, &(Ceval::noTarget().clone()))?;
    }
    Ok(exp)
}

fn evaluateActualStream(
    mut streamCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut flowCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stream_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut flow_dir: i32;
    let mut flow_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut stream_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut instream_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    stream_cref = ComponentRef::evaluateSubscripts(streamCref)?;
    flowCref = associatedFlowCref(stream_cref.clone())?;
    flow_dir = evaluateFlowDirection(flowCref.clone(), variables.clone())?;
    if flow_dir == 1 {
        exp = evaluateInStream(
            stream_cref,
            sets,
            setsArray.clone(),
            variables,
            ctable,
            replacements.clone(),
        )?;
    } else if flow_dir == -1 {
        exp = Expression::fromCref(stream_cref, false)?;
    } else {
        flow_exp = Expression::fromCref(flowCref.clone(), false)?;
        stream_exp = Expression::fromCref(stream_cref.clone(), false)?;
        instream_exp = evaluateInStream(
            stream_cref,
            sets,
            setsArray.clone(),
            variables,
            ctable,
            replacements.clone(),
        )?;
        op = Operator::makeGreater(ComponentRef::nodeType(&flowCref)?);
        exp = metamodelica::Ref::new(Expression::NFExpression::IF {
            ty: crate::NFType::interned_REAL(),
            condition: metamodelica::Ref::new(Expression::NFExpression::RELATION {
                exp1: flow_exp,
                operator: op,
                exp2: metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: metamodelica::OrderedFloat(0.0_f64),
                }),
                index: -1,
            }),
            trueBranch: instream_exp,
            falseBranch: stream_exp,
        });
    }
    if (replacements).is_some() {
        exp = StreamFlowAlias::applyReplacementsInExp(replacements.ok_or("pattern mismatch")?, exp)?;
    }
    Ok((exp, flowCref))
}

fn evaluateActualStreamMul(
    mut crefExp: metamodelica::Ref<Expression::NFExpression>,
    mut actualStreamArg: &metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut e1: metamodelica::Ref<Expression::NFExpression>;
    let mut e2: metamodelica::Ref<Expression::NFExpression>;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut flow_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(evaluateOperators(crefExp, sets, setsArray.clone(), variables.clone(), ctable.clone(), replacements.clone())?) {
        __pa1 @ Deref @ Expression::CREF { cref: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    e1 = metamodelica::Own::own(__pa1);
    (e2, flow_cr) = evaluateActualStream(
        Expression::toCref(actualStreamArg)?,
        sets,
        setsArray.clone(),
        variables,
        ctable,
        replacements,
    )?;
    outExp = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2.clone(),
    });
    outExp = (match &*e2 {
        Expression::IF { .. } if (ComponentRef::isEqual(&cr, &flow_cr)?) => makeSmoothCall(outExp, 0)?,
        _ => outExp,
    });
    Ok(outExp)
}

fn evaluateFlowDirection(
    mut flowCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<i32> {
    let mut direction: i32 = 0;
    let mut omin: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut omax: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut min_val: metamodelica::Real;
    let mut max_val: metamodelica::Real;
    omin = lookupVarAttr(flowCref.clone(), &(literal!("min")), variables.clone())?;
    omin = Util::applyOption(
        omin,
        &({
            let __pe_b1 = false;
            move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone())
        }),
    )?;
    omax = lookupVarAttr(flowCref, &(literal!("max")), variables)?;
    omax = Util::applyOption(
        omax,
        &({
            let __pe_b1 = false;
            move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone())
        }),
    )?;
    direction = (::match_deref::match_deref! { match &((omin, omax)) {
        (None, None) => 0,
        (Some(Deref @ Expression::REAL { value: __esc_min_val }), None) => {
            min_val = (*__esc_min_val).clone();
            if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64)) {1} else {0}
        },
        (None, Some(Deref @ Expression::REAL { value: __esc_max_val })) => {
            max_val = (*__esc_max_val).clone();
            if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64)) {-1} else {0}
        },
        (Some(Deref @ Expression::REAL { value: __esc_min_val }), Some(Deref @ Expression::REAL { value: __esc_max_val })) => {
            min_val = (*__esc_min_val).clone();
            max_val = (*__esc_max_val).clone();
            if (min_val.clone() >= metamodelica::OrderedFloat((0) as f64) && max_val.clone() >= min_val.clone()) {1} else if (max_val.clone() <= metamodelica::OrderedFloat((0) as f64) && min_val.clone() <= max_val.clone()) {-1} else {0}
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(direction)
}

fn makeSmoothCall(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut order: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut callExp: metamodelica::Ref<Expression::NFExpression>;
    callExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::SMOOTH().clone(),
            list![
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: order }),
                arg.clone()
            ],
            Expression::variability(arg.clone())?,
            Purity::PURE.clone(),
            Expression::typeOf(arg),
        ),
    });
    Ok(callExp)
}

fn removeStreamSetElement(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut elements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
) -> Result<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<Connector::NFConnector>> = elements;
    (elements, _) = List::deleteMemberOnTrue(cref, elements, &move |__a0: metamodelica::Ref<
        ComponentRef::NFComponentRef,
    >,
                                                                    __a1: metamodelica::Ref<
        Connector::NFConnector,
    >| compareCrefStreamSet(&__a0, &__a1))?;
    Ok(elements)
}

fn compareCrefStreamSet(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut element: &metamodelica::Ref<Connector::NFConnector>,
) -> Result<bool> {
    let mut matches: bool;
    matches = ComponentRef::isEqual(cref, &element.name)?;
    Ok(matches)
}

fn associatedFlowCref(
    mut streamCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    '__tco: loop {
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut rest_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut flow_node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(streamCref.clone()) {
            Deref @ ComponentRef::CREF { ty: __pa0, restCref: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa0);
        rest_cr = metamodelica::Own::own(__pa1);
        ::match_deref::match_deref! { match &(Type::arrayElementType(&ty)) {
            Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::CONNECTOR { flows: Deref @ metamodelica::ListNode::Cons { head: __esc_flow_node, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
                flow_node = (*__esc_flow_node).clone();
                return Ok(ComponentRef::prefixCref(NFInstNode::InstNode::borrow(flow_node.clone())?, NFInstNode::InstNode::getType(NFInstNode::InstNode::borrow(flow_node.clone())?)?, metamodelica::nil(), streamCref)?)
            },
            _ => { streamCref = rest_cr; continue '__tco; },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lookupVar(
    mut varName: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut ovar: Option<metamodelica::Ref<Variable::NFVariable>>;
    ovar = UnorderedMap::get(varName.clone(), variables.clone())?;
    if (ovar).is_none() {
        ovar = UnorderedMap::get(ComponentRef::stripSubscriptsAll(&varName), variables)?;
    }
    if (ovar).is_none() {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFConnectEquations.lookupVar"));
                __mm_s.push_str(&*literal!(" could not find the variable "));
                __mm_s.push_str(&*ComponentRef::toString(&varName)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("NFFrontEnd/NFConnectEquations.mo"),
        )?;
    }
    let __pa0 = ::match_deref::match_deref! { match &(ovar) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    var = metamodelica::Own::own(__pa0);
    Ok(var)
}

fn lookupVarAttr(
    mut varName: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut attrName: &ArcStr,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut attrValue: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    var = lookupVar(varName, variables)?;
    binding = Variable::lookupTypeAttribute(attrName, &var);
    attrValue = Binding::typedExp(&binding);
    Ok(attrValue)
}

fn lookupAttrInVar(
    mut attrName: &ArcStr,
    mut var: &metamodelica::Ref<Variable::NFVariable>,
) -> Option<metamodelica::Ref<Expression::NFExpression>> {
    let mut attrValue: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    binding = Variable::lookupTypeAttribute(attrName, var);
    attrValue = Binding::typedExp(&binding);
    attrValue
}

fn lookupFlowVarInConnector(
    mut conn: &metamodelica::Ref<Connector::NFConnector>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<(metamodelica::Ref<Variable::NFVariable>, bool)> {
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut negated: bool;
    let mut flow_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut flow_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    flow_exp = flowExp(conn)?;
    if (replacements).is_some() {
        flow_exp = StreamFlowAlias::applyReplacementsInExp(replacements.ok_or("pattern mismatch")?, flow_exp)?;
    }
    (flow_name, negated) = expFlowName(&flow_exp)?;
    var = lookupVar(flow_name, variables)?;
    Ok((var, negated))
}

fn expFlowName(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, bool)> {
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut negated: bool;
    (name, negated) = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } => (__exp_cref.clone(), false),
        Expression::UNARY { exp: __exp_exp, .. } => {
            (name, negated) = expFlowName(metamodelica::AsArg::as_arg(&__exp_exp))?;
            (name, !(negated))
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((name, negated))
}
