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

use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedSet;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFAlgorithm {
    pub statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    pub inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    pub outputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    pub stmtDiffInfo: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Statement::NFStatement>>>>,
    /// Weakly: that scope's sections hold
    ///      the algorithm.
    pub scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    pub source: metamodelica::Ref<DAE::ElementSource>,
}

impl metamodelica::gc::MMTrace for NFAlgorithm {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.statements, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inputs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outputs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stmtDiffInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        Ok(())
    }
}
impl Default for NFAlgorithm {
    fn default() -> Self {
        Self {
            statements: Default::default(),
            inputs: Default::default(),
            outputs: Default::default(),
            stmtDiffInfo: Default::default(),
            scope: Default::default(),
            source: Default::default(),
        }
    }
}

pub type ALGORITHM = NFAlgorithm;

pub type ApplyFn =
    std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<()> + 'static>;

pub(crate) fn applyList(
    mut algs: &metamodelica::List<metamodelica::Ref<NFAlgorithm>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<()>,
) -> Result<()> {
    for mut alg in &**algs {
        for mut s in &*alg.statements.clone() {
            Statement::apply(s.clone(), func)?;
        }
    }
    Ok(())
}

pub(crate) fn apply(
    mut alg: &metamodelica::Ref<NFAlgorithm>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<()>,
) -> Result<()> {
    for mut s in &*alg.statements.clone() {
        Statement::apply(s.clone(), func)?;
    }
    Ok(())
}

pub(crate) fn applyExp(
    mut alg: &metamodelica::Ref<NFAlgorithm>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut s in &*alg.statements.clone() {
        Statement::applyExp(metamodelica::AsArg::as_arg(&s), func)?;
    }
    Ok(())
}

pub(crate) fn applyExpList(
    mut algs: &metamodelica::List<metamodelica::Ref<NFAlgorithm>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut alg in &**algs {
        applyExp(metamodelica::AsArg::as_arg(&alg), func)?;
    }
    Ok(())
}

pub(crate) fn map(
    mut alg: metamodelica::Ref<NFAlgorithm>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::Ref<NFAlgorithm>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Statement::NFStatement>,
            ) -> Result<metamodelica::Ref<Statement::NFStatement>>
            + 'static,
    >;

    let mut alg: metamodelica::Ref<NFAlgorithm> = alg;
    assign_field!(
        alg.statements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
            for mut s in (alg.statements.clone()).into_iter().cloned() {
                let __x = Statement::map(s.clone(), r#fn)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(alg)
}

pub fn mapExp(
    mut alg: metamodelica::Ref<NFAlgorithm>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFAlgorithm>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut alg: metamodelica::Ref<NFAlgorithm> = alg;
    assign_field!(alg.statements = Statement::mapExpList(alg.statements.clone(), func)?);
    Ok(alg)
}

pub fn mapExpList(
    mut algs: metamodelica::List<metamodelica::Ref<NFAlgorithm>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFAlgorithm>>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut algs: metamodelica::List<metamodelica::Ref<NFAlgorithm>> = algs;
    algs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFAlgorithm>> = metamodelica::nil();
        for mut alg in (algs).into_iter().cloned() {
            let __x = mapExp(alg.clone(), func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(algs)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut alg: &metamodelica::Ref<NFAlgorithm>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut s in &*alg.statements.clone() {
        arg = Statement::foldExp(metamodelica::AsArg::as_arg(&s), func, arg)?;
    }
    Ok(arg)
}

pub(crate) fn foldExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut algs: &metamodelica::List<metamodelica::Ref<NFAlgorithm>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut alg in &**algs {
        arg = foldExp(metamodelica::AsArg::as_arg(&alg), func, arg)?;
    }
    Ok(arg)
}

pub fn toString(mut alg: &metamodelica::Ref<NFAlgorithm>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = Statement::toStringList(&alg.statements, indent)?;
    Ok(r#str)
}

pub fn setInputsOutputs(mut alg: metamodelica::Ref<NFAlgorithm>) -> Result<metamodelica::Ref<NFAlgorithm>> {
    let mut alg: metamodelica::Ref<NFAlgorithm> = alg;
    let mut inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut outputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    (inputs, outputs) = getInputsOutputs(&alg.statements)?;
    assign_field!(alg.inputs = inputs, alg.outputs = outputs);
    Ok(alg)
}

pub fn getInputsOutputs(
    mut statements: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
)> {
    let mut inputs_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut outputs_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut inputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
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
    let mut outputs_set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = UnorderedSet::new(
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
    match '__try0: {
        for mut statement in &**statements {
            unwrap_break_err!(statementInputsOutputs(metamodelica::AsArg::as_arg(&statement), inputs_set.clone(), outputs_set.clone()), '__try0);
        }
        inputs_lst = UnorderedSet::toList(inputs_set.clone());
        outputs_lst = UnorderedSet::toList(outputs_set.clone());
        Ok::<_, &'static str>((inputs_lst.clone(), outputs_lst.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            inputs_lst = __try0_o0;
            outputs_lst = __try0_o1;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFAlgorithm.getInputsOutputs"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok((inputs_lst, outputs_lst))
}

pub fn isEqual(mut alg1: &metamodelica::Ref<NFAlgorithm>, mut alg2: &metamodelica::Ref<NFAlgorithm>) -> Result<bool> {
    let mut b: bool;
    b = List::isEqualOnTrue(
        alg1.inputs.clone(),
        alg2.inputs.clone(),
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
               __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1),
    )? && List::isEqualOnTrue(
        alg1.outputs.clone(),
        alg2.outputs.clone(),
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
               __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1),
    )? && List::isEqualOnTrue(
        alg1.statements.clone(),
        alg2.statements.clone(),
        &move |__a0: metamodelica::Ref<Statement::NFStatement>, __a1: metamodelica::Ref<Statement::NFStatement>| {
            Statement::isEqual(&__a0, &__a1)
        },
    )?;
    Ok(b)
}

pub fn isEmpty(mut alg: &metamodelica::Ref<NFAlgorithm>) -> bool {
    let mut b: bool = (alg.statements).is_empty();
    b
}

pub fn isDiscrete(mut alg: &metamodelica::Ref<NFAlgorithm>) -> Result<bool> {
    let mut b: bool;
    b = List::any(&alg.outputs, &move |__a0: metamodelica::Ref<
        ComponentRef::NFComponentRef,
    >| ComponentRef::isDiscrete(&__a0))?;
    b = if (b) {
        b
    } else {
        List::any(&alg.statements, &move |__a0: metamodelica::Ref<
            Statement::NFStatement,
        >| Statement::isDiscrete(&__a0))?
    };
    Ok(b)
}

fn statementInputsOutputs(
    mut statement: &metamodelica::Ref<Statement::NFStatement>,
    mut inputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut outputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match statement {
        Deref @ Statement::ASSIGNMENT { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs, .. } => {
            Expression::apply(rhs.clone(), &({ let __pe_b1 = inputs_set.clone(); let __pe_b2 = outputs_set.clone(); move |__pe_a0| expressionInputs(&__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            expressionOutput(lhs.clone(), inputs_set, outputs_set)?;
            ()
        },
        Deref @ Statement::ASSIGNMENT { lhs: Deref @ Expression::TUPLE { elements, .. }, rhs, .. } => {
            Expression::apply(rhs.clone(), &({ let __pe_b1 = inputs_set.clone(); let __pe_b2 = outputs_set.clone(); move |__pe_a0| expressionInputs(&__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            for mut exp in &*elements.clone() {
                expressionOutput(exp.clone(), inputs_set.clone(), outputs_set.clone())?;
            }
            ()
        },
        Deref @ Statement::FOR { body: stmts, .. } => {
            for mut stmt in &*stmts.clone() {
                statementInputsOutputs(metamodelica::AsArg::as_arg(&stmt), inputs_set.clone(), outputs_set.clone())?;
            }
            ()
        },
        Deref @ Statement::IF { branches, .. } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            for mut branch in &*branches.clone() {
                (_, stmts) = branch.clone();
                for mut stmt in &*stmts {
                    statementInputsOutputs(metamodelica::AsArg::as_arg(&stmt), inputs_set.clone(), outputs_set.clone())?;
                }
            }
            ()
        },
        Deref @ Statement::WHEN { branches, .. } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            for mut branch in &*branches.clone() {
                (_, stmts) = branch.clone();
                for mut stmt in &*stmts {
                    statementInputsOutputs(metamodelica::AsArg::as_arg(&stmt), inputs_set.clone(), outputs_set.clone())?;
                }
            }
            ()
        },
        Deref @ Statement::WHILE { body: stmts, .. } => {
            for mut stmt in &*stmts.clone() {
                statementInputsOutputs(metamodelica::AsArg::as_arg(&stmt), inputs_set.clone(), outputs_set.clone())?;
            }
            ()
        },
        Deref @ Statement::ASSERT { .. } => {
            ()
        },
        Deref @ Statement::TERMINATE { .. } => {
            ()
        },
        Deref @ Statement::REINIT { .. } => {
            ()
        },
        Deref @ Statement::NORETCALL { .. } => {
            ()
        },
        Deref @ Statement::RETURN { .. } => {
            ()
        },
        Deref @ Statement::BREAK { .. } => {
            ()
        },
        Deref @ Statement::FAILURE { .. } => {
            ()
        },
        Deref @ Statement::FUNCTION_ARRAY_INIT { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.statementInputsOutputs")); __mm_s.push_str(&*literal!(" failed due to wrong Statement Type: FUNCTION_ARRAY_INIT.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.statementInputsOutputs")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*Statement::toString(statement, literal!(""))?); ArcStr::from(__mm_s) }])?;
            }
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn expressionInputs(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut inputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut outputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let () = (match &**exp {
        Expression::CREF { cref: cr, .. } if (ComponentRef::isTime(cr)?) => (),
        Expression::CREF { cref: cr, .. } if (ComponentRef::isIterator(cr)) => (),
        Expression::CREF { ty, .. } if (Type::isExternalObject(ty)) => (),
        Expression::CREF { cref: cr, .. } => {
            let mut cr = (*cr).clone();
            cr = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cr));
            if !(UnorderedSet::contains(cr.clone(), outputs_set)?) {
                UnorderedSet::add(cr.clone(), inputs_set)?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn expressionOutput(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut inputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut outputs_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => {
            ()
        },
        Deref @ Expression::CREF { cref: cr, .. } if (ComponentRef::isTime(metamodelica::AsArg::as_arg(&cr))?) => {
            Error::addMessage(Error::COMPILER_ERROR.clone(), list![literal!("Trying to assign to time.")])?;
            return Err("fail")
        },
        Deref @ Expression::CREF { cref: cr, .. } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&cr))) => {
            Error::addMessage(Error::COMPILER_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Trying to assign to iterator ")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        Deref @ Expression::CREF { ty, .. } if (Type::isExternalObject(metamodelica::AsArg::as_arg(&ty))) => {
            ()
        },
        Deref @ Expression::CREF { cref: cr, .. } => {
            let mut cr = (*cr).clone();
            cr = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cr));
            if UnorderedSet::remove(cr.clone(), inputs_set)? {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Using output variable in RHS before it is assigned (former occurences will be set to initial value): ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                }
            }
            UnorderedSet::add(cr.clone(), outputs_set)?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFAlgorithm.expressionOutput")); __mm_s.push_str(&*literal!(" failed due to wrong expression type in LHS of algorithm statement: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
