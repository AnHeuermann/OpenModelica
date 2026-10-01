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

use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBReplacements as Replacements;
use crate::NBSolve as Solve;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBResizable.mo
///  package:      NBResizable
///  description:  This file contains util functions for resizable parameters.
pub struct NBResizable;
pub(crate) const debug: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum EvalOrder {
    INDEPENDENT = 1,
    FORWARD = 2,
    BACKWARD = 3,
    FAILED = 4,
}
impl PartialOrd for EvalOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for EvalOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for EvalOrder {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn resize(
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut varData: metamodelica::Ref<VarData::VarData>,
) -> Result<(
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<VarData::VarData>,
)> {
    type applyFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;

    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut min_parameters: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >;
    let mut c2pe: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >;
    let mut p2ci: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >;
    let mut p2ce: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >;
    let mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;
    varData = (match &*varData {
        BVariable::VarData::VAR_DATA_SIM {
            resizables: __varData_resizables,
            variables: __varData_variables,
            ..
        } if (BVariable::VariablePointers::size(metamodelica::AsArg::as_arg(&__varData_resizables)) > 0) => {
            parameters = UnorderedSet::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            min_parameters = UnorderedSet::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            optimal_values = UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            c2pi = UnorderedMap::new(
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
            c2pe = UnorderedMap::new(
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
            EquationPointers::map(
                equations.clone(),
                &({
                    let __pe_b1 = parameters.clone();
                    let __pe_b2 = min_parameters.clone();
                    let __pe_b3 = optimal_values.clone();
                    let __pe_b4 = c2pi.clone();
                    let __pe_b5 = c2pe.clone();
                    move |__pe_a0| {
                        findOptimalResizableValues(
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
            parameters = UnorderedSet::selfMap(
                parameters,
                &({
                    let __pe_b1 = min_parameters;
                    let __pe_b2 = optimal_values.clone();
                    move |__pe_a0| setInitialValues(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
            )?;
            if debug.clone() {
                metamodelica::print(optimalValuesToString(optimal_values.clone(), {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &(literal!("[debug] Initial Resizable Parameter Values:")),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                })?);
                metamodelica::print(StringUtil::headline_2(
                    &(literal!("[debug] Final Inequality Constraints:")),
                )?);
                if UnorderedMap::isEmpty(c2pi.clone()) {
                    metamodelica::print(literal!("  <No Constraints>\n\n"));
                } else {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*List::toStringCustom(
                            UnorderedMap::keyList(c2pi.clone()),
                            &Expression::toString,
                            literal!(""),
                            literal!("  0 >= "),
                            literal!("\n  0 >= "),
                            literal!("\n"),
                            true,
                            0,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                metamodelica::print(StringUtil::headline_2(
                    &(literal!("[debug] Final Equality Constraints:")),
                )?);
                if UnorderedMap::isEmpty(c2pe.clone()) {
                    metamodelica::print(literal!("  <No Constraints>\n\n"));
                } else {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*List::toStringCustom(
                            UnorderedMap::keyList(c2pe.clone()),
                            &Expression::toString,
                            literal!(""),
                            literal!("  0 = "),
                            literal!("\n  0 = "),
                            literal!("\n"),
                            true,
                            0,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            p2ci = invertConstraintParameterMap(c2pi.clone(), parameters.clone())?;
            p2ce = invertConstraintParameterMap(c2pe.clone(), parameters)?;
            computeOptimalValues(optimal_values.clone(), c2pi, p2ci, c2pe, p2ce)?;
            func = (std::sync::Arc::new({
                let __pe_b1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Dimension::NFDimension>,
                        ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
                        + 'static,
                > = (std::sync::Arc::new({
                    let __pe_b1 = optimal_values.clone();
                    move |__pe_a0| updateDimension(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Dimension::NFDimension>,
                            )
                                -> Result<metamodelica::Ref<Dimension::NFDimension>>
                            + 'static,
                    >);
                move |__pe_a0| Type::applyToDims(__pe_a0, &*__pe_b1)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>
                        + 'static,
                >);
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM; variables = BVariable::VariablePointers::map(__varData_variables.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Variable::applyToType(__pe_a0, &*__pe_b1) }))?);
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM; variables = BVariable::VariablePointers::mapPtr(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), &({ let __pe_b1 = optimal_values.clone(); move |__pe_a0| BVariable::updateResizableParameter(__pe_a0, __pe_b1.clone()) }))?);
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM; variables = BVariable::VariablePointers::mapPtr(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), &({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static> = func.clone(); move |__pe_a0| Expression::applyToType(__pe_a0, &*__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); let __pe_b2 = (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| BVariable::mapExp(__pe_a0, __pe_b1.clone(), &*__pe_b2) }))?);
            EquationPointers::mapPtr(
                &equations,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a0| Equation::applyToType(__pe_a0, &*__pe_b1)
                }),
            )?;
            equations = EquationPointers::mapExp(
                equations,
                (std::sync::Arc::new({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a0| Expression::applyToType(__pe_a0, &*__pe_b1)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            EquationPointers::mapRes(
                &equations,
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a0| BVariable::applyToType(__pe_a0, &*__pe_b1)
                }),
            )?;
            if Flags::isSet(Flags::DUMP_RESIZABLE.clone())? || debug.clone() {
                metamodelica::print(optimalValuesToString(optimal_values, {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &(literal!("[dumpResizable] Evaluated Optimal Resizable Parameter Values:")),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                })?);
            }
            varData
        }
        _ => {
            if Flags::isSet(Flags::DUMP_RESIZABLE.clone())? || debug.clone() {
                metamodelica::print(StringUtil::headline_2(
                    &(literal!("[dumpResizable] No resizable parameters were detected in the model.")),
                )?);
            }
            varData
        }
    });
    Ok((equations, varData))
}

pub(crate) fn detect(
    mut eqn: &metamodelica::Ref<Equation::Equation>,
    mut cref_to_solve: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>>> {
    let mut order: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>,
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
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> =
        BVariable::getVarPointer(cref_to_solve, metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"))?;
    let mut var_occurences: metamodelica::Ref<
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
    let mut ite_occurences: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut occ_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut iterators: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
    let mut subs_to_solve: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut local_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut sub_to_solve: metamodelica::Ref<Subscript::NFSubscript>;
    let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut opt_factor: Option<i32>;
    let mut factor: i32;
    let mut shift_value: i32;
    let mut v2: i32;
    let mut eval: EvalOrder;
    let mut stop: bool;
    order = (match &**eqn {
        Equation::FOR_EQUATION { body: __eqn_body, .. } => {
            for mut body in &*__eqn_body.clone() {
                Equation::map(
                    body.clone(),
                    (std::sync::Arc::new({
                        let __pe_b1 = (std::sync::Arc::new({
                            let __pe_b1 = var_ptr.clone();
                            move |__pe_a0| BVariable::equalName(__pe_a0, __pe_b1.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                    ) -> Result<bool>
                                    + 'static,
                            >);
                        let __pe_b2 = var_occurences.clone();
                        move |__pe_a0| collectVars(__pe_a0, &*__pe_b1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                occ_lst = UnorderedSet::toList(var_occurences.clone());
                (iterators, _, _) = Iterator::getFrames(&(Equation::getForIterator(eqn)));
                order = UnorderedMap::fromLists(
                    &(iterators.clone()),
                    ({
                        let mut __acc: metamodelica::List<EvalOrder> = metamodelica::nil();
                        for mut i in (iterators).into_iter().cloned() {
                            let __x = EvalOrder::INDEPENDENT.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                )?;
                if !(List::hasOneElement(&occ_lst)) {
                    subs = ({
                        let mut __acc: metamodelica::List<
                            metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
                        > = metamodelica::nil();
                        for mut cref in (occ_lst).into_iter().cloned() {
                            let __x = ComponentRef::subscriptsAllWithWholeFlat(&(cref.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    subs = List::transposeList(subs)?;
                    subs_to_solve = ComponentRef::subscriptsAllWithWholeFlat(cref_to_solve)?;
                    stop = false;
                    '__loop0: for mut dim in &*List::zip(subs, subs_to_solve) {
                        (local_subs, sub_to_solve) = dim.clone();
                        ite_occurences = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
                        for mut sub in &*local_subs {
                            Subscript::mapExp(
                                sub.clone(),
                                (std::sync::Arc::new({
                                    let __pe_b1 = (std::sync::Arc::new(fnptr!(
                                        BVariable::isIterator,
                                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                                    ))
                                        as std::sync::Arc<
                                            dyn ::std::ops::Fn(
                                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >);
                                    let __pe_b2 = ite_occurences.clone();
                                    move |__pe_a0| collectVars(__pe_a0, &*__pe_b1, __pe_b2.clone())
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
                        iterators = UnorderedSet::toList(ite_occurences);
                        let () = (::match_deref::match_deref! { match &(iterators.clone()) {
                            Deref @ metamodelica::ListNode::Nil => (),
                            Deref @ metamodelica::ListNode::Cons { head: __esc_iter, tail: Deref @ metamodelica::ListNode::Nil } => {
                                iter = (*__esc_iter).clone();
                                eval = UnorderedMap::getSafe(iter.clone(), order.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"))?;
                                if eval < EvalOrder::FAILED.clone() {
                                    args = Differentiate::DifferentiationArguments::simpleCref(iter.clone(), UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>) -> Result<bool> + 'static>), 1));
                                    opt_factor = None;
                                    for mut sub in &*local_subs {
                                        factor = getFactor(Subscript::toExp(metamodelica::AsArg::as_arg(&sub))?, args.clone(), opt_factor)?;
                                        opt_factor = Some(factor);
                                    }
                                    let () = (match opt_factor {
                            Some(mut factor) if (factor != 0) => {
                                if '__try0: {
                                    let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(getShift(unwrap_break_err!(Subscript::toExp(&sub_to_solve), '__try0), iter.clone()), '__try0)) {
                                        Deref @ Expression::INTEGER { value: __pa1 } => __pa1.clone(),
                                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                                    } };
                                    shift_value = metamodelica::Own::own(__pa1);
                                    for mut sub in &*local_subs {
                                        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(getShift(unwrap_break_err!(Subscript::toExp(metamodelica::AsArg::as_arg(&sub)), '__try0), iter.clone()), '__try0)) {
                                            Deref @ Expression::INTEGER { value: __pa2 } => __pa2.clone(),
                                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                                        } };
                                        v2 = metamodelica::Own::own(__pa2);
                                        eval = (match eval {
                            EvalOrder::INDEPENDENT if (shift_value == v2) => EvalOrder::INDEPENDENT.clone(),
                            EvalOrder::INDEPENDENT if (shift_value > v2) => EvalOrder::FORWARD.clone(),
                            EvalOrder::INDEPENDENT if (shift_value < v2) => EvalOrder::BACKWARD.clone(),
                            EvalOrder::FORWARD if (shift_value >= v2) => EvalOrder::FORWARD.clone(),
                            EvalOrder::BACKWARD if (shift_value <= v2) => EvalOrder::BACKWARD.clone(),
                            _ => EvalOrder::FAILED.clone(),
                        });
                                    }
                                    if eval == EvalOrder::FAILED.clone() {
                                        break '__loop0;
                                    }
                                    Ok::<(), &'static str>(())
                                }.is_err() {
                                    eval = EvalOrder::FAILED.clone();
                                }
                                UnorderedMap::add(iter.clone(), eval, order.clone())?;
                                ()
                            },
                            _ => (),
                        });
                                }
                                ()
                            },
                            _ => {
                                for mut it in &*iterators {
                                    UnorderedMap::add(it.clone(), EvalOrder::FAILED.clone(), order.clone())?;
                                }
                                stop = true;
                                ()
                            },
                            _ => unreachable!("match_deref! exhaustiveness placeholder"),
                        } });
                        if stop {
                            break '__loop0;
                        }
                    }
                }
            }
            order
        }
        _ => {
            order = UnorderedMap::fromLists(
                &(list![openmodelica_nf_frontend::NFComponentRef::interned_EMPTY()]),
                list![EvalOrder::FAILED.clone()],
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            )?;
            order
        }
    });
    Ok(order)
}

pub(crate) fn orderFailed(mut eo: EvalOrder) -> bool {
    let mut b: bool = eo == EvalOrder::FAILED.clone();
    b
}

pub(crate) fn orderString(mut eo: EvalOrder) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match eo {
        EvalOrder::INDEPENDENT => literal!("INDEPENDENT"),
        EvalOrder::FORWARD => literal!("FORWARD"),
        EvalOrder::BACKWARD => literal!("BACKWARD"),
        _ => literal!("FAILED"),
    });
    r#str
}

pub type ParameterList = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

pub type ConstraintList = metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;

pub type Occurences = metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>;

thread_local! { static __END_TPL_TLS: (metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>) = (openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(), openmodelica_nf_frontend::NFExpression::interned_END()); }
pub(crate) fn END_TPL() -> (
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Expression::NFExpression>,
) {
    __END_TPL_TLS.with(|__t| __t.clone())
}

fn findOptimalResizableValues(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut min_parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut c2pe: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut resizables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >;
    let mut constrained_vars: metamodelica::Ref<
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
    let mut lhs_dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut rhs_dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut r#const: metamodelica::Ref<Expression::NFExpression>;
    if debug.clone() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[debug] checking equation:\n"));
            __mm_s.push_str(&*Equation::toString(eqn.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    let () = (match &*eqn {
        Equation::FOR_EQUATION {
            body: __eqn_body,
            iter: __eqn_iter,
            ..
        } => {
            resizables = getResizableIterators(metamodelica::AsArg::as_arg(&__eqn_iter))?;
            replacements = getVarReplacements(metamodelica::AsArg::as_arg(&__eqn_iter))?;
            occs = UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            for mut res in &*UnorderedMap::keyList(resizables.clone()) {
                UnorderedMap::add(
                    res.clone(),
                    UnorderedSet::new(
                        (std::sync::Arc::new(Expression::hash)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<i32>
                                    + 'static,
                            >),
                        (std::sync::Arc::new(Expression::isEqual)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                        metamodelica::Ref<Expression::NFExpression>,
                                    ) -> Result<bool>
                                    + 'static,
                            >),
                        13,
                    ),
                    occs.clone(),
                )?;
            }
            for mut body in &*__eqn_body.clone() {
                Equation::map(
                    body.clone(),
                    (std::sync::Arc::new({
                        let __pe_b1 = occs.clone();
                        move |__pe_a0| collectOccurences(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                Equation::map(
                    body.clone(),
                    (std::sync::Arc::new({
                        let __pe_b1 = (std::sync::Arc::new(fnptr!(
                            BVariable::isArray,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                        ))
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                    ) -> Result<bool>
                                    + 'static,
                            >);
                        let __pe_b2 = constrained_vars.clone();
                        move |__pe_a0| collectVars(__pe_a0, &*__pe_b1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
            }
            findOptimalValue(
                &eqn,
                occs,
                resizables,
                parameters,
                min_parameters,
                optimal_values,
                c2pi.clone(),
            )?;
            UnorderedSet::fold(
                constrained_vars,
                &({
                    let __pe_b1 = eqn.clone();
                    let __pe_b2 = Some(replacements);
                    move |__pe_a0, __pe_a3| addVariableConstraint(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
                }),
                c2pi,
            )?;
            ()
        }
        Equation::ARRAY_EQUATION {
            lhs: __eqn_lhs,
            rhs: __eqn_rhs,
            ..
        } => {
            Equation::map(
                eqn.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = (std::sync::Arc::new(BVariable::isResizable)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                ) -> Result<bool>
                                + 'static,
                        >);
                    let __pe_b2 = constrained_vars.clone();
                    move |__pe_a0| collectVars(__pe_a0, &*__pe_b1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            UnorderedSet::fold(
                constrained_vars,
                &({
                    let __pe_b1 = eqn.clone();
                    let __pe_b2 = None;
                    move |__pe_a0, __pe_a3| addVariableConstraint(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
                }),
                c2pi,
            )?;
            for mut tpl in &*List::zip(
                Type::arrayDims(Expression::typeOf(__eqn_lhs.clone())),
                Type::arrayDims(Expression::typeOf(__eqn_rhs.clone())),
            ) {
                (lhs_dim, rhs_dim) = tpl.clone();
                if Dimension::isResizable(&lhs_dim) || Dimension::isResizable(&rhs_dim) {
                    r#const = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![Dimension::sizeExp(&lhs_dim)?],
                        inv_arguments: list![Dimension::sizeExp(&rhs_dim)?],
                        operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()),
                    });
                    match '__try0: {
                        unwrap_break_err!(addConstraint(r#const.clone(), None, c2pe.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isZero(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>), &(literal!("array dimension")), &(literal!("="))), '__try0);
                        unwrap_break_err!(Expression::map(r#const.clone(), (std::sync::Arc::new({ let __pe_b1 = parameters.clone(); move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>)), '__try0);
                        Ok::<(), &'static str>(())
                    } {
                        Ok(()) => {}
                        Err(__try0_err) => {
                            Error::addMessage(
                                Error::INTERNAL_ERROR.clone(),
                                list![{
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("NBResizable.findOptimalResizableValues"));
                                    __mm_s.push_str(&*literal!(" failed.\nViolation of implicit constraint `"));
                                    __mm_s.push_str(&*Dimension::toString(&lhs_dim)?);
                                    __mm_s.push_str(&*literal!(" = "));
                                    __mm_s.push_str(&*Dimension::toString(&rhs_dim)?);
                                    __mm_s.push_str(&*literal!("` for LHS and RHS type dimensions in equation:\n"));
                                    __mm_s.push_str(&*Equation::toString(eqn.clone(), literal!(""))?);
                                    ArcStr::from(__mm_s)
                                }],
                            )?;
                            return Err(__try0_err);
                        }
                    }
                }
            }
            ()
        }
        _ => {
            Equation::map(
                eqn.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = (std::sync::Arc::new(BVariable::isResizable)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                ) -> Result<bool>
                                + 'static,
                        >);
                    let __pe_b2 = constrained_vars.clone();
                    move |__pe_a0| collectVars(__pe_a0, &*__pe_b1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            UnorderedSet::fold(
                constrained_vars,
                &({
                    let __pe_b1 = eqn.clone();
                    let __pe_b2 = None;
                    move |__pe_a0, __pe_a3| addVariableConstraint(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)
                }),
                c2pi,
            )?;
            ()
        }
    });
    if debug.clone() {
        metamodelica::print(literal!("\n"));
    }
    Ok(eqn)
}

fn getResizableIterators(
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
> {
    let mut resizables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
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
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    (names, ranges, _) = Iterator::getFrames(iter);
    for mut tpl in &*List::zip(names, ranges) {
        if iteratorIsResizable(Util::tuple22(tpl.clone()))? {
            UnorderedMap::add(
                Util::tuple21(tpl.clone()),
                Util::tuple22(tpl.clone()),
                resizables.clone(),
            )?;
        }
    }
    Ok(resizables)
}

fn getVarReplacements(
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
> {
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
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
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut max_call: metamodelica::Ref<Expression::NFExpression>;
    (names, ranges, _) = Iterator::getFrames(iter);
    for mut tpl in &*List::zip(names, ranges) {
        (name, range) = tpl.clone();
        let () = (match &*range {
            Expression::RANGE {
                start: __range_start,
                step: __range_step,
                stop: __range_stop,
                ..
            } => {
                if (__range_step).is_some() && Expression::isNegative(&(Util::getOption(__range_step.clone())?))? {
                    UnorderedMap::add(name, __range_start.clone(), replacements.clone())?;
                } else if (__range_step).is_none() || Expression::isPositive(&(Util::getOption(__range_step.clone())?))?
                {
                    UnorderedMap::add(name, __range_stop.clone(), replacements.clone())?;
                } else {
                    max_call = metamodelica::Ref::new(Expression::NFExpression::CALL {
                        call: Call::makeTypedCall(
                            NFBuiltinFuncs::MAX_INT().clone(),
                            list![__range_start.clone(), __range_stop.clone()],
                            Expression::variability(__range_start.clone())?,
                            NFPrefixes::Purity::PURE.clone(),
                            NFBuiltinFuncs::MAX_INT().returnType.clone(),
                        ),
                    });
                    UnorderedMap::add(name, max_call, replacements.clone())?;
                }
                ()
            }
            _ => (),
        });
    }
    Ok(replacements)
}

fn iteratorIsResizable(mut range: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool = Expression::fold(
        range.clone(),
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: bool| {
            expContainsResizable(&__a0, __a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
            >),
        false,
    )?;
    Ok(b)
}

fn expContainsResizable(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut b: bool) -> Result<bool> {
    let mut b: bool = b;
    if !(b) {
        b = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. } => BVariable::checkCref(
                metamodelica::AsArg::as_arg(&__exp_cref),
                &fnptr!(
                    BVariable::isResizableParameter,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ),
                metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
            )?,
            _ => false,
        });
    }
    Ok(b)
}

fn collectResizables(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut collector: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (BVariable::checkCref(
                metamodelica::AsArg::as_arg(&__exp_cref),
                &fnptr!(
                    BVariable::isResizableParameter,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ),
                metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
            )?) =>
        {
            UnorderedSet::add(__exp_cref.clone(), collector)?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

fn collectOccurences(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            ComponentRef::mapSubscripts(
                __exp_cref.clone(),
                &({
                    let __pe_b1 = occs;
                    move |__pe_a0| collectOccurencesSubscript(__pe_a0, __pe_b1.clone())
                }),
                false,
            )?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

fn collectOccurencesSubscript(
    mut sub: metamodelica::Ref<Subscript::NFSubscript>,
    mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >,
) -> Result<metamodelica::Ref<Subscript::NFSubscript>> {
    let mut sub: metamodelica::Ref<Subscript::NFSubscript> = sub;
    let mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut subExp: metamodelica::Ref<Expression::NFExpression>;
    Subscript::mapExp(
        sub.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = occs.clone();
            let __pe_b2 = acc.clone();
            move |__pe_a0| collectOccurencesSubscriptExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    if !(UnorderedSet::isEmpty(acc.clone())) {
        subExp = Subscript::toExp(&sub)?;
        acc = UnorderedSet::selfMap(
            acc,
            &({
                let __pe_b1 = subExp;
                let __pe_b2 = occs;
                move |__pe_a0| addOccurence(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
    }
    Ok(sub)
}

fn collectOccurencesSubscriptExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } if (UnorderedMap::contains(__exp_cref.clone(), occs.clone())?) => {
            UnorderedSet::add(__exp_cref.clone(), acc)?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

fn addOccurence(
    mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut subExp: metamodelica::Ref<Expression::NFExpression>,
    mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef> = iter;
    let mut local_occ: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>> =
        UnorderedMap::getSafe(
            iter.clone(),
            occs.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
        )?;
    UnorderedSet::add(subExp, local_occ)?;
    Ok(iter)
}

fn collectVars(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut func: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
    mut collector: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (func(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
            )?)?) =>
        {
            UnorderedSet::add(__exp_cref.clone(), collector)?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

fn findOptimalValue(
    mut eqn: &metamodelica::Ref<Equation::Equation>,
    mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >,
    mut resizables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut min_parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
) -> Result<()> {
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut target: metamodelica::Ref<Expression::NFExpression>;
    let mut local_parameters: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    for mut cref in &*UnorderedMap::keyList(occs) {
        range = UnorderedMap::getSafe(
            cref.clone(),
            resizables.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
        )?;
        Expression::map(
            range.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = parameters.clone();
                move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        let () = (match &*range.clone() {
            Expression::RANGE {
                start: __range_start,
                step: __range_step,
                stop: __range_stop,
                ..
            } => {
                target = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![__range_stop.clone()],
                    inv_arguments: list![__range_start.clone()],
                    operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()),
                });
                target = SimplifyExp::simplify(target, false)?;
                local_parameters = UnorderedSet::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                Expression::map(
                    range,
                    (std::sync::Arc::new({
                        let __pe_b1 = local_parameters.clone();
                        move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                UnorderedSet::merge(parameters.clone(), local_parameters.clone())?;
                args = Differentiate::DifferentiationArguments::simpleCref(
                    cref.clone(),
                    UnorderedMap::new(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static,
                            >),
                        (std::sync::Arc::new(
                            move |__a0: metamodelica::Ref<Absyn::Path>,
                                  __a1: metamodelica::Ref<Absyn::Path>|
                                  -> metamodelica::Result<_> {
                                ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                            },
                        )
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Absyn::Path>,
                                        metamodelica::Ref<Absyn::Path>,
                                    ) -> Result<bool>
                                    + 'static,
                            >),
                        1,
                    ),
                );
                local_parameters = UnorderedSet::selfMap(
                    local_parameters,
                    &({
                        let __pe_b1 = target;
                        let __pe_b2 = args;
                        let __pe_b3 = min_parameters.clone();
                        let __pe_b4 = optimal_values.clone();
                        move |__pe_a0| {
                            getInitialValues(
                                __pe_a0,
                                __pe_b1.clone(),
                                __pe_b2.clone(),
                                __pe_b3.clone(),
                                __pe_b4.clone(),
                            )
                        }
                    }),
                )?;
                getRangeConstraint(
                    __range_start.clone(),
                    __range_step.clone(),
                    __range_stop.clone(),
                    local_parameters,
                    c2pi.clone(),
                    &(literal!("equation")),
                )?;
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn getRangeConstraint(
    mut start: metamodelica::Ref<Expression::NFExpression>,
    mut step_opt: Option<metamodelica::Ref<Expression::NFExpression>>,
    mut stop: metamodelica::Ref<Expression::NFExpression>,
    mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut const_kind: &ArcStr,
) -> Result<()> {
    let mut step: metamodelica::Ref<Expression::NFExpression>;
    let mut target: metamodelica::Ref<Expression::NFExpression>;
    let mut distance_const: metamodelica::Ref<Expression::NFExpression>;
    step = Util::getOptionOrDefault(
        step_opt,
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
    );
    target = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
        arguments: list![stop],
        inv_arguments: list![start],
        operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()),
    });
    distance_const = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
        arguments: list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 2 })],
        inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::MULTARY {
            arguments: list![target],
            inv_arguments: list![step],
            operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_INTEGER())
        })],
        operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()),
    });
    distance_const = SimplifyExp::simplify(distance_const, false)?;
    distance_const = SimplifyExp::combineBinaries(distance_const)?;
    distance_const = SimplifyExp::simplify(distance_const, false)?;
    UnorderedMap::add(distance_const.clone(), UnorderedSet::toList(parameters), c2pi)?;
    if debug.clone() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[debug] adding "));
            __mm_s.push_str(&*const_kind);
            __mm_s.push_str(&*literal!(" constraint: 0 >= "));
            __mm_s.push_str(&*Expression::toString(distance_const)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn getFactor(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut opt_factor: Option<i32>,
) -> Result<i32> {
    let mut factor: i32;
    let mut diff: metamodelica::Ref<Expression::NFExpression>;
    (diff, _) = Differentiate::differentiateExpression(exp, args)?;
    diff = SimplifyExp::simplify(diff, false)?;
    factor = Expression::integerValueOrDefault(&diff, 0);
    if (opt_factor).is_some() && factor != Util::getOption(opt_factor)? {
        factor = 0;
    }
    Ok(factor)
}

fn getShift(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut shift: metamodelica::Ref<Expression::NFExpression>;
    shift = Replacements::single(
        exp,
        &(metamodelica::Ref::new(Expression::NFExpression::CREF {
            ty: openmodelica_nf_frontend::NFType::interned_INTEGER(),
            cref: cref,
        })),
        &(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })),
    )?;
    shift = SimplifyExp::simplify(shift, false)?;
    Ok(shift)
}

fn getDistance(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut opt_factor: Option<i32>,
    mut min_distance: i32,
    mut max_distance: i32,
) -> Result<(Option<i32>, i32, i32)> {
    let mut opt_factor: Option<i32> = opt_factor;
    let mut min_distance: i32 = min_distance;
    let mut max_distance: i32 = max_distance;
    let mut shift: metamodelica::Ref<Expression::NFExpression>;
    let mut factor: i32;
    let mut distance: i32;
    if (opt_factor).is_none() || Util::getOption(opt_factor.clone())? != 0 {
        factor = getFactor(exp.clone(), args, opt_factor.clone())?;
        if factor != 0 {
            shift = getShift(exp, cref)?;
            match '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(shift.clone()) {
                    Deref @ Expression::INTEGER { value: __pa1 } => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                distance = metamodelica::Own::own(__pa1);
                if (opt_factor).is_none() {
                    min_distance = distance;
                    max_distance = distance;
                    opt_factor = Some(factor);
                } else {
                    min_distance = intMin(distance, min_distance);
                    max_distance = intMax(distance, max_distance);
                }
                Ok::<_, &'static str>((max_distance.clone(), min_distance.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    max_distance = __try0_o0;
                    min_distance = __try0_o1;
                }
                Err(_) => {
                    min_distance = 0;
                    max_distance = 0;
                    opt_factor = Some(0);
                }
            }
        } else {
            min_distance = 0;
            max_distance = 0;
            opt_factor = Some(0);
        }
    }
    Ok((opt_factor, min_distance, max_distance))
}

fn addVariableConstraint(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
    mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
> {
    let mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    > = c2pi;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(BVariable::getVarPointer(
        cref,
        metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
    )?);
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = Type::arrayDims(var.ty.clone());
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
        ComponentRef::subscriptsAllWithWholeFlat(cref)?;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    let mut sub_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut r#const: metamodelica::Ref<Expression::NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator> =
        Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER());
    for mut tpl in &*List::zip(dims, subs) {
        (dim, sub) = tpl.clone();
        sub_exp = Subscript::toExp(&sub)?;
        r#const = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
            arguments: list![sub_exp],
            inv_arguments: list![Dimension::sizeExp(&dim)?],
            operator: op.clone(),
        });
        match '__try0: {
            unwrap_break_err!(addConstraint(r#const.clone(), replacements.clone(), c2pi.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNonPositive(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(cref), '__try0)); __mm_s.push_str(&*literal!(" (variable)")); ArcStr::from(__mm_s) }), &(literal!(">="))), '__try0);
            Ok::<(), &'static str>(())
        } {
            Ok(()) => {}
            Err(__try0_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBResizable.addVariableConstraint"));
                        __mm_s.push_str(&*literal!(" failed.\nViolation of implicit constraint `"));
                        __mm_s.push_str(&*Dimension::toString(&dim)?);
                        __mm_s.push_str(&*literal!(" >= "));
                        __mm_s.push_str(&*Subscript::toString(&sub)?);
                        __mm_s.push_str(&*literal!("` for component reference `"));
                        __mm_s.push_str(&*ComponentRef::toString(cref)?);
                        __mm_s.push_str(&*literal!("` of variable `"));
                        __mm_s.push_str(&*Variable::toString(
                            &(Pointer::access(BVariable::getVarPointer(
                                cref,
                                metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
                            )?)),
                            literal!(""),
                            false,
                        )?);
                        __mm_s.push_str(&*literal!("`\nin equation:\n"));
                        __mm_s.push_str(&*Equation::toString(eqn.clone(), literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err(__try0_err);
            }
        }
        r#const = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
            arguments: list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })],
            inv_arguments: list![Dimension::sizeExp(&dim)?],
            operator: op.clone(),
        });
        match '__try1: {
            unwrap_break_err!(addConstraint(r#const.clone(), replacements.clone(), c2pi.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| Expression::isNonPositive(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>), &({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::toString(cref), '__try1)); __mm_s.push_str(&*literal!(" (variable)")); ArcStr::from(__mm_s) }), &(literal!(">="))), '__try1);
            Ok::<(), &'static str>(())
        } {
            Ok(()) => {}
            Err(__try1_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBResizable.addVariableConstraint"));
                        __mm_s.push_str(&*literal!(" failed.\nViolation of implicit constraint `"));
                        __mm_s.push_str(&*Dimension::toString(&dim)?);
                        __mm_s.push_str(&*literal!(" >= 1"));
                        __mm_s.push_str(&*literal!("` for component reference `"));
                        __mm_s.push_str(&*ComponentRef::toString(cref)?);
                        __mm_s.push_str(&*literal!("` of variable `"));
                        __mm_s.push_str(&*Variable::toString(
                            &(Pointer::access(BVariable::getVarPointer(
                                cref,
                                metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
                            )?)),
                            literal!(""),
                            false,
                        )?);
                        __mm_s.push_str(&*literal!("`\nin equation:\n"));
                        __mm_s.push_str(&*Equation::toString(eqn.clone(), literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err(__try1_err);
            }
        }
    }
    Ok(c2pi)
}

fn addConstraint(
    mut old_const: metamodelica::Ref<Expression::NFExpression>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
    mut c2p: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>,
    mut const_kind: &ArcStr,
    mut eq_kind: &ArcStr,
) -> Result<()> {
    pub type checkFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut r#const: metamodelica::Ref<Expression::NFExpression>;
    let mut diff: metamodelica::Ref<Expression::NFExpression>;
    let mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut params: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut redundant: bool;
    let mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut zero_replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    if (replacements).is_some() {
        r#const = Expression::map(
            old_const.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = Util::getOption(replacements.clone())?;
                move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        r#const = old_const.clone();
    }
    r#const = Expression::map(
        r#const,
        (std::sync::Arc::new({
            let __pe_b1 = replacements;
            let __pe_b2 = c2p.clone();
            let __pe_b3: Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
            > = func.clone();
            let __pe_b4 = const_kind.clone();
            let __pe_b5 = eq_kind.clone();
            move |__pe_a0| addRangeConstraints(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &*__pe_b3, &__pe_b4, &__pe_b5)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    parameters = UnorderedSet::new(
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
    Expression::map(
        r#const.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = parameters.clone();
            move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    params = UnorderedSet::toList(parameters);
    redundant = true;
    for mut p in &*params {
        args = Differentiate::DifferentiationArguments::simpleCref(
            p.clone(),
            UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Absyn::Path>,
                          __a1: metamodelica::Ref<Absyn::Path>|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Path>,
                                metamodelica::Ref<Absyn::Path>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                1,
            ),
        );
        (diff, _) = Differentiate::differentiateExpression(r#const.clone(), args)?;
        diff = SimplifyExp::simplify(diff, false)?;
        if !(Expression::isZero(&diff)?) {
            redundant = false;
            break;
        }
    }
    if !(redundant) {
        UnorderedMap::add(r#const.clone(), params, c2p)?;
    } else {
        zero_replacements = UnorderedMap::new(
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
        for mut p in &*params {
            UnorderedMap::add(
                p.clone(),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                zero_replacements.clone(),
            )?;
        }
        r#const = Expression::map(
            r#const,
            (std::sync::Arc::new({
                let __pe_b1 = zero_replacements;
                move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        r#const = SimplifyExp::simplify(r#const, false)?;
        if !(func(r#const.clone())?) && Expression::isLiteral(&r#const)? {
            return Err("fail");
        }
    }
    if debug.clone() {
        if redundant {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[debug] not adding redundant "));
                __mm_s.push_str(&*const_kind);
                __mm_s.push_str(&*literal!(" constraint: 0 "));
                __mm_s.push_str(&*eq_kind);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*Expression::toString(old_const)?);
                __mm_s.push_str(&*literal!(" simplified to: 0 "));
                __mm_s.push_str(&*eq_kind);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*Expression::toString(r#const)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        } else {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[debug] adding "));
                __mm_s.push_str(&*const_kind);
                __mm_s.push_str(&*literal!(" constraint: 0 "));
                __mm_s.push_str(&*eq_kind);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*Expression::toString(old_const)?);
                __mm_s.push_str(&*literal!(" simplified to: 0 "));
                __mm_s.push_str(&*eq_kind);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*Expression::toString(r#const)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok(())
}

fn addRangeConstraints(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
    mut c2p: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
    mut const_kind: &ArcStr,
    mut eq_kind: &ArcStr,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    pub type checkFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    exp = (match &*exp {
        Expression::RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            parameters = UnorderedSet::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            Expression::map(
                exp.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = parameters.clone();
                    move |__pe_a0| collectResizables(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            getRangeConstraint(
                __exp_start.clone(),
                __exp_step.clone(),
                __exp_stop.clone(),
                parameters,
                c2p,
                &(literal!("variable")),
            )?;
            Expression::rangeSizeExp(exp)?
        }
        _ => exp,
    });
    Ok(exp)
}

fn getInitialValues(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut target: metamodelica::Ref<Expression::NFExpression>,
    mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut min_parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut diff: metamodelica::Ref<Expression::NFExpression>;
    let mut binding: metamodelica::Ref<Expression::NFExpression>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    assign_field!(args.diffCref = cref.clone());
    (diff, _) = Differentiate::differentiateExpression(target, args)?;
    diff = SimplifyExp::simplify(diff, false)?;
    if Expression::isPositive(&diff)? {
        UnorderedSet::add(cref.clone(), min_parameters)?;
    } else if Expression::isNegative(&diff)? {
    } else {
        var = Pointer::access(BVariable::getVarPointer(
            &cref,
            metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
        )?);
        binding = Binding::getExp(&var.binding)?;
        UnorderedMap::add(cref.clone(), binding, optimal_values)?;
    }
    Ok(cref)
}

fn setInitialValues(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut min_parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut attributes: metamodelica::Ref<VariableAttributes::VariableAttributes>;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    if !(UnorderedMap::contains(cref.clone(), optimal_values.clone())?) {
        var = Pointer::access(BVariable::getVarPointer(
            &cref,
            metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
        )?);
        value = (::match_deref::match_deref! { match &(var) {
            Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { attributes: __esc_attributes @ Deref @ VariableAttributes::VAR_ATTR_INT { .. }, .. }, .. } => {
                attributes = (*__esc_attributes).clone();
                if UnorderedSet::contains(cref.clone(), min_parameters)? {
                    if (var_field!((*attributes).min, VariableAttributes::VariableAttributes::VAR_ATTR_INT)).is_some() {
                        value = Binding::getTypedExp(&(Util::getOption(var_field!((*attributes).min, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone())?))?;
                    } else {
                        value = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
                    }
                } else if (var_field!((*attributes).max, VariableAttributes::VariableAttributes::VAR_ATTR_INT)).is_some() {
                    value = Binding::getTypedExp(&(Util::getOption(var_field!((*attributes).max, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone())?))?;
                } else {
                    value = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
                }
                value
            },
            _ => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        UnorderedMap::add(cref.clone(), value, optimal_values)?;
    }
    Ok(cref)
}

fn invertConstraintParameterMap(
    mut c2p: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut parameters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >,
> {
    let mut p2c: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
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
    let mut r#const: metamodelica::Ref<Expression::NFExpression>;
    let mut params: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    for mut param in &*UnorderedSet::toList(parameters) {
        UnorderedMap::add(param.clone(), metamodelica::nil(), p2c.clone())?;
    }
    for mut tpl in &*UnorderedMap::toList(c2p) {
        (r#const, params) = tpl.clone();
        for mut param in &*params {
            UnorderedMap::add(
                param.clone(),
                metamodelica::cons(
                    r#const.clone(),
                    UnorderedMap::getOrDefault(param.clone(), p2c.clone(), metamodelica::nil())?,
                ),
                p2c.clone(),
            )?;
        }
    }
    Ok(p2c)
}

fn computeOptimalValues(
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut c2pi: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut p2ci: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >,
    mut c2pe: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut p2ce: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >,
) -> Result<()> {
    let mut failed_parameters: metamodelica::Ref<
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
    if debug.clone() {
        metamodelica::print(literal!("FIXING CONSTRAINTS\n\n"));
    }
    fixConstraints(
        optimal_values.clone(),
        c2pi,
        p2ci,
        failed_parameters.clone(),
        &({
            let __pe_b1 = 0;
            move |__pe_a0| Ok(intLe(__pe_a0, __pe_b1.clone()))
        }),
    )?;
    fixConstraints(
        optimal_values,
        c2pe,
        p2ce,
        failed_parameters,
        &({
            let __pe_b1 = 0;
            move |__pe_a0| Ok(intEq(__pe_a0, __pe_b1.clone()))
        }),
    )?;
    if debug.clone() {
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

fn fixConstraints(
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut c2p: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut p2c: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >,
    mut failed_parameters: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut func: &dyn ::std::ops::Fn(i32) -> Result<bool>,
) -> Result<()> {
    pub type checkVal = std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>;

    let mut parsed_constraints: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>,
    > = UnorderedSet::new(
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
        13,
    );
    let mut constraint: metamodelica::Ref<Expression::NFExpression>;
    let mut old_optimal_value: metamodelica::Ref<Expression::NFExpression>;
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    let mut solved_eqn: metamodelica::Ref<Equation::Equation>;
    let mut status: Solve::Status;
    let mut value: i32;
    let mut failed: bool;
    for mut tpl in &*UnorderedMap::toList(c2p) {
        (constraint, crefs) = tpl.clone();
        failed = false;
        let () = (match checkConstraint(constraint.clone(), optimal_values.clone())? {
            Some(mut value) if (func(value)?) => {
                if debug.clone() {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*Expression::toString(constraint)?);
                        __mm_s.push_str(&*literal!(" || is not violated "));
                        __mm_s.push_str(&*intString(value));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                ()
            }
            Some(mut __esc_value) => {
                value = __esc_value.clone();
                if debug.clone() {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*Expression::toString(constraint.clone())?);
                        __mm_s.push_str(&*literal!(" || is violated by "));
                        __mm_s.push_str(&*intString(value));
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                eqn = Equation::makeAssignmentEqn(
                    constraint.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                    crate::NBEquation::Iterator::interned_EMPTY(),
                    NBEquation::default(EquationKind::DISCRETE.clone(), false, None, None),
                )?;
                for mut cref in &*crefs {
                    failed = false;
                    (solved_eqn, status, _) = Solve::solveBody(
                        eqn.clone(),
                        cref.clone(),
                        UnorderedMap::new(
                            (std::sync::Arc::new(
                                move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                                },
                            )
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static,
                                >),
                            (std::sync::Arc::new(
                                move |__a0: metamodelica::Ref<Absyn::Path>,
                                      __a1: metamodelica::Ref<Absyn::Path>|
                                      -> metamodelica::Result<_> {
                                    ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                                },
                            )
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Absyn::Path>,
                                            metamodelica::Ref<Absyn::Path>,
                                        ) -> Result<bool>
                                        + 'static,
                                >),
                            1,
                        ),
                    )?;
                    if status == Solve::Status::EXPLICIT.clone() {
                        let () = (match checkConstraint(
                            Util::getOption(Equation::getRHS(solved_eqn)?)?,
                            optimal_values.clone(),
                        )? {
                            Some(mut __esc_value) => {
                                value = __esc_value.clone();
                                old_optimal_value = UnorderedMap::getSafe(
                                    cref.clone(),
                                    optimal_values.clone(),
                                    metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
                                )?;
                                UnorderedMap::add(
                                    cref.clone(),
                                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: value }),
                                    optimal_values.clone(),
                                )?;
                                for mut cons in &*UnorderedMap::getSafe(
                                    cref.clone(),
                                    p2c.clone(),
                                    metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
                                )? {
                                    if UnorderedSet::contains(constraint.clone(), parsed_constraints.clone())?
                                        && !(func(Util::getOptionOrDefault(
                                            checkConstraint(cons.clone(), optimal_values.clone())?,
                                            1,
                                        ))?)
                                    {
                                        failed = true;
                                        break;
                                    }
                                }
                                if failed {
                                    UnorderedMap::add(cref.clone(), old_optimal_value, optimal_values.clone())?;
                                }
                                ()
                            }
                            _ => (),
                        });
                    } else {
                        failed = true;
                    }
                    if !(failed) {
                        UnorderedSet::add(constraint, parsed_constraints.clone())?;
                        break;
                    }
                }
                ()
            }
            _ => {
                for mut cref in &*crefs {
                    UnorderedSet::add(cref.clone(), failed_parameters.clone())?;
                }
                ()
            }
        });
        if failed {
            for mut cref in &*crefs {
                UnorderedSet::add(cref.clone(), failed_parameters.clone())?;
            }
        }
    }
    Ok(())
}

fn checkConstraint(
    mut constraint: metamodelica::Ref<Expression::NFExpression>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<Option<i32>> {
    let mut value: Option<i32>;
    let mut replaced: metamodelica::Ref<Expression::NFExpression>;
    replaced = Expression::map(
        constraint,
        (std::sync::Arc::new({
            let __pe_b1 = optimal_values;
            move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    replaced = SimplifyExp::simplify(replaced, false)?;
    value = (match &*replaced {
        Expression::INTEGER {
            value: __replaced_value,
        } => Some(__replaced_value.clone()),
        _ => None,
    });
    Ok(value)
}

fn updateDimension(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    dim = (match &*dim {
        Dimension::RESIZABLE { exp: __dim_exp, .. } => {
            assign_variant_field!(dim => Dimension::NFDimension::RESIZABLE; opt_size = checkConstraint(__dim_exp.clone(), optimal_values)?);
            dim
        }
        _ => dim,
    });
    Ok(dim)
}

fn optimalValuesToString(
    mut optimal_values: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut r#str: ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut param: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut value: metamodelica::Ref<Expression::NFExpression>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut old_vals: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut new_vals: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut name: ArcStr;
    let mut old: ArcStr;
    let mut new: ArcStr;
    let mut names_len: i32;
    for mut tpl in &*UnorderedMap::toList(optimal_values) {
        (param, value) = tpl.clone();
        var = Pointer::access(BVariable::getVarPointer(
            &param,
            metamodelica::sourceInfo!("NBackEnd/Util/NBResizable.mo"),
        )?);
        names = metamodelica::cons(ComponentRef::toString(&param)?, names);
        new_vals = metamodelica::cons(Expression::toString(value)?, new_vals);
        old_vals = metamodelica::cons(Binding::toString(&var.binding, &(literal!("")))?, old_vals);
    }
    names_len = ({
        let mut __acc: Option<i32> = None;
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = ((n).len() as i32);
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
    while !((names).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(names) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        names = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(new_vals) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        new = metamodelica::Own::own(__pa2);
        new_vals = metamodelica::Own::own(__pa3);
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(old_vals) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        old = metamodelica::Own::own(__pa4);
        old_vals = metamodelica::Own::own(__pa5);
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("  "));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*StringUtil::repeat(
                literal!("."),
                names_len + 5 - ((name).len() as i32),
            )?);
            __mm_s.push_str(&*literal!(" OPTIMAL: "));
            __mm_s.push_str(&*new);
            __mm_s.push_str(&*literal!(" (ORIGINAL: "));
            __mm_s.push_str(&*old);
            __mm_s.push_str(&*literal!(")\n"));
            ArcStr::from(__mm_s)
        };
    }
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn occurencesToString(
    mut occs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
        >,
    >,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    for mut tpl in &*UnorderedMap::toList(occs) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentRef::toString(&(Util::tuple21(tpl.clone())))?);
            __mm_s.push_str(&*literal!(": {"));
            __mm_s.push_str(&*UnorderedSet::toString(
                Util::tuple22(tpl.clone()),
                &Expression::toString,
                literal!(", "),
            )?);
            __mm_s.push_str(&*literal!("}\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

fn distancesToString(mut tpl: (metamodelica::Ref<ComponentRef::NFComponentRef>, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentRef::toString(&(Util::tuple21(tpl.clone())))?);
        __mm_s.push_str(&*literal!(":"));
        __mm_s.push_str(&*intString(Util::tuple22(tpl.clone())));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn parametersToString(
    mut parameters: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = List::toString(
        parameters.clone(),
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
        List::Style::FLAT_CURLY.clone(),
    )?;
    Ok(r#str)
}

fn constraintsToString(
    mut constraints: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = List::toString(
        constraints.clone(),
        &Expression::toString,
        List::Style::FLAT_CURLY.clone(),
    )?;
    Ok(r#str)
}
