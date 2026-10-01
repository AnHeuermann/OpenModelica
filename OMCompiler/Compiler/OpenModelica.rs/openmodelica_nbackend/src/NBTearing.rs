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

use crate::NBAdjacency as Adjacency;
use crate::NBAdjacency::Solvability;
use crate::NBBackendUtil as BackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::EqnSlice;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian as BJacobian;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VarSlice;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_error::ErrorExt;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBTearing.mo
/// package:     NBTearing
/// description: This file contains the data-types used for tearing. It is a
///              uniontype and therefore also contains some structures for tearing.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NBTearing {
    /// the variables used for iteration
    pub iteration_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    /// implicitely solved residual equations
    pub residual_eqns:
        metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
    /// array of matched equations and variables
    pub innerEquations: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    /// optional jacobian
    pub jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
}

impl metamodelica::gc::MMTrace for NBTearing {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.iteration_vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.residual_eqns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jac, __mmv)?;
        Ok(())
    }
}
impl Default for NBTearing {
    fn default() -> Self {
        Self {
            iteration_vars: Default::default(),
            residual_eqns: Default::default(),
            innerEquations: Default::default(),
            jac: Default::default(),
        }
    }
}

pub type TEARING_SET = NBTearing;

pub(crate) fn hash(mut set: &metamodelica::Ref<NBTearing>) -> Result<i32> {
    let mut h: i32 = ({
        let mut __acc: i32 = 0;
        for mut var in (set.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::hash(var.clone(), &BVariable::hash)?;
            __acc += __x;
        }
        __acc
    });
    Ok(h)
}

pub(crate) fn isEqual(
    mut set1: &metamodelica::Ref<NBTearing>,
    mut set2: &metamodelica::Ref<NBTearing>,
) -> Result<bool> {
    let mut b: bool;
    b = UnorderedSet::equal_list(
        &set1.residual_eqns,
        &set2.residual_eqns,
        (std::sync::Arc::new({
            let __pe_b1 = (std::sync::Arc::new(BEquation::Equation::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32>
                        + 'static,
                >);
            move |__pe_a0| Slice::hash(__pe_a0, &*__pe_b1)
        }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<i32> + 'static>),
        (std::sync::Arc::new({
            let __pe_b2 = (std::sync::Arc::new(BEquation::Equation::isEqualPtr)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        ) -> Result<bool>
                        + 'static,
                >);
            move |__pe_a0, __pe_a1| Slice::isEqual(__pe_a0, __pe_a1, &*__pe_b2)
        }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
    )?;
    b = if (b) {
        Array::isEqualOnTrue(
            set1.innerEquations.clone(),
            set2.innerEquations.clone(),
            &move |__a0: metamodelica::Ref<StrongComponent::NBStrongComponent>,
                   __a1: metamodelica::Ref<StrongComponent::NBStrongComponent>| {
                StrongComponent::isEqual(&__a0, &__a1)
            },
        )?
    } else {
        b
    };
    b = if (b) {
        UnorderedSet::equal_list(
            &set1.iteration_vars,
            &set2.iteration_vars,
            (std::sync::Arc::new({
                let __pe_b1 = (std::sync::Arc::new(BVariable::hash)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                            + 'static,
                    >);
                move |__pe_a0| Slice::hash(__pe_a0, &*__pe_b1)
            }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<i32> + 'static>),
            (std::sync::Arc::new({
                let __pe_b2 = (std::sync::Arc::new(BVariable::equalName)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                            ) -> Result<bool>
                            + 'static,
                    >);
                move |__pe_a0, __pe_a1| Slice::isEqual(__pe_a0, __pe_a1, &*__pe_b2)
            }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
        )?
    } else {
        b
    };
    Ok(b)
}

pub(crate) fn size(mut set: &metamodelica::Ref<NBTearing>, mut resize: bool) -> Result<i32> {
    let mut s: i32;
    s = ({
        let mut __acc: i32 = 0;
        for mut eq in (set.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::size(
                eq.clone(),
                &({
                    let __pe_b1 = resize;
                    move |__pe_a0| BEquation::Equation::size(__pe_a0, __pe_b1.clone())
                }),
            )?;
            __acc += __x;
        }
        __acc
    });
    s = s
        + ({
            let mut __acc: i32 = 0;
            for mut eq in (set.innerEquations.clone()).borrow().iter() {
                let __x = StrongComponent::size(&(eq.clone()), resize)?;
                __acc += __x;
            }
            __acc
        });
    Ok(s)
}

pub(crate) fn toString(mut set: &metamodelica::Ref<NBTearing>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = StringUtil::headline_4(&r#str)?;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("### Iteration Variables:\n"));
        __mm_s.push_str(&*Slice::lstToString(
            set.iteration_vars.clone(),
            (std::sync::Arc::new(BVariable::pointerToString)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr>
                        + 'static,
                >),
            literal!("    "),
            10,
        )?);
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n### Residual Equations:\n"));
        __mm_s.push_str(&*Slice::lstToString(
            set.residual_eqns.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = literal!("    ");
                move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<ArcStr>
                        + 'static,
                >),
            literal!(""),
            10,
        )?);
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("\n### Inner Equations:\n"));
        __mm_s.push_str(&*Array::toString(
            set.innerEquations.clone(),
            &({
                let __pe_b1 = -1;
                move |__pe_a0| StrongComponent::toString(&__pe_a0, __pe_b1.clone())
            }),
            literal!(""),
            literal!("    "),
            literal!("\n  "),
            literal!(""),
            true,
            0,
        )?);
        ArcStr::from(__mm_s)
    };
    if (set.jac).is_some() {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*BJacobian::toString(
                &(set.jac.clone().ok_or("pattern mismatch")?),
                literal!("NLS"),
            )?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn main(
    mut bdae: metamodelica::Ref<Jacobian::NBackendDAE>,
    mut kind: Partition::Kind,
) -> Result<metamodelica::Ref<Jacobian::NBackendDAE>> {
    '__tco: loop {
        let funcs: metamodelica::List<Module::tearingInterface> = getModule()?;
        if Flags::isSet(Flags::TEARING_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_1(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("["));
                        __mm_s.push_str(&*Partition::Partition::kindToString(kind)?);
                        __mm_s.push_str(&*literal!("] Tearing"));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        ::match_deref::match_deref! { match &((kind, bdae.clone())) {
            (Partition::Kind::ODE, Deref @ Jacobian::MAIN { eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { uniqueIndex: eq_index, .. }, .. }) => {
                assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; ode = tearingTraverser(&(var_field!((*bdae).ode, Jacobian::NBackendDAE::MAIN).clone()), &funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?);
                return Ok(bdae)
            },
            (_, Deref @ Jacobian::MAIN { eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { uniqueIndex: eq_index, .. }, .. }) if (Partition::kindIsInitial(kind)) => {
                assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; init = tearingTraverser(&(var_field!((*bdae).init, Jacobian::NBackendDAE::MAIN).clone()), &funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?);
                if (var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN)).is_some() {
                    assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; init_0 = Some(tearingTraverser(&(var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN).clone().ok_or("pattern mismatch")?), &funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?));
                }
                return Ok(bdae)
            },
            (Partition::Kind::DAE, Deref @ Jacobian::MAIN { dae: Some(partitions), eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { uniqueIndex: eq_index, .. }, .. }) => {
                assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; dae = Some(tearingTraverser(metamodelica::AsArg::as_arg(&partitions), &funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?));
                { (bdae, kind) = (bdae, Partition::Kind::ODE.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn implicit(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut index: i32,
    mut kind: Partition::Kind,
) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, i32)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut index: i32 = index;
    let mut dummy: metamodelica::Ref<Adjacency::Matrix::Matrix> =
        metamodelica::Ref::new(Adjacency::Matrix::Matrix::EMPTY {
            st: Adjacency::MatrixStrictness::FULL.clone(),
        });
    let mut new_comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut homotopy: Pointer::Pointer<bool> = Pointer::create(false);
    (comp, dummy, index) = (match &*comp {
        StrongComponent::SINGLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            BEquation::Equation::map(
                Pointer::access(__comp_eqn.clone()),
                (std::sync::Arc::new({
                    let __pe_b1 = homotopy.clone();
                    move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone())
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
            new_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP {
                idx: index,
                strict: singleImplicit(__comp_var.clone(), __comp_eqn.clone()),
                casual: None,
                linear: isLinearSlice(
                    __comp_eqn.clone(),
                    ComponentRef::scalarize(BVariable::getVarName(__comp_var.clone()), false)?,
                    funcMap.clone(),
                )?,
                mixed: false,
                homotopy: Pointer::access(homotopy),
                status: Solve::Status::IMPLICIT.clone(),
                implicitlyCreated: true,
            });
            index = index + 1;
            finalize(
                new_comp,
                dummy,
                funcMap,
                index,
                BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false),
                BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()),
                Pointer::create(0),
                kind,
            )?
        }
        StrongComponent::MULTI_COMPONENT {
            eqn: __comp_eqn,
            vars: __comp_vars,
            ..
        } => {
            BEquation::Equation::map(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                (std::sync::Arc::new({
                    let __pe_b1 = homotopy.clone();
                    move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone())
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
            new_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP {
                idx: index,
                strict: singleImplicit(
                    Slice::getT((__comp_vars).head().cloned()?),
                    Slice::getT(__comp_eqn.clone()),
                ),
                casual: None,
                linear: false,
                mixed: false,
                homotopy: Pointer::access(homotopy),
                status: Solve::Status::IMPLICIT.clone(),
                implicitlyCreated: true,
            });
            index = index + 1;
            finalize(
                new_comp,
                dummy,
                funcMap,
                index,
                BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false),
                BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()),
                Pointer::create(0),
                kind,
            )?
        }
        StrongComponent::RESIZABLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            var_cref: __comp_var_cref,
            ..
        } => {
            BEquation::Equation::map(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                (std::sync::Arc::new({
                    let __pe_b1 = homotopy.clone();
                    move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone())
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
            new_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP {
                idx: index,
                strict: singleImplicit(Slice::getT(__comp_var.clone()), Slice::getT(__comp_eqn.clone())),
                casual: None,
                linear: isLinearSlice(
                    Slice::getT(__comp_eqn.clone()),
                    ComponentRef::scalarize(__comp_var_cref.clone(), false)?,
                    funcMap.clone(),
                )?,
                mixed: false,
                homotopy: Pointer::access(homotopy),
                status: Solve::Status::IMPLICIT.clone(),
                implicitlyCreated: true,
            });
            index = index + 1;
            finalize(
                new_comp,
                dummy,
                funcMap,
                index,
                BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false),
                BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()),
                Pointer::create(0),
                kind,
            )?
        }
        StrongComponent::SLICED_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            var_cref: __comp_var_cref,
            ..
        } => {
            BEquation::Equation::map(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                (std::sync::Arc::new({
                    let __pe_b1 = homotopy.clone();
                    move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone())
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
            new_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP {
                idx: index,
                strict: slicedImplicit(__comp_var.clone(), __comp_eqn.clone())?,
                casual: None,
                linear: isLinearSlice(
                    Slice::getT(__comp_eqn.clone()),
                    ComponentRef::scalarize(__comp_var_cref.clone(), false)?,
                    funcMap.clone(),
                )?,
                mixed: false,
                homotopy: Pointer::access(homotopy),
                status: Solve::Status::IMPLICIT.clone(),
                implicitlyCreated: true,
            });
            index = index + 1;
            finalize(
                new_comp,
                dummy,
                funcMap,
                index,
                BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false),
                BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()),
                Pointer::create(0),
                kind,
            )?
        }
        _ => (comp, dummy, index),
    });
    Ok((comp, index))
}

pub(crate) fn singleImplicit(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
) -> metamodelica::Ref<NBTearing> {
    let mut tearingSet: metamodelica::Ref<NBTearing> = metamodelica::Ref::new(NBTearing {
        iteration_vars: list![metamodelica::Ref::new(Slice::NBSlice {
            t: var.clone(),
            indices: metamodelica::nil()
        })],
        residual_eqns: list![metamodelica::Ref::new(Slice::NBSlice {
            t: eqn.clone(),
            indices: metamodelica::nil()
        })],
        innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        jac: None,
    });
    tearingSet
}

pub(crate) fn slicedImplicit(
    mut var: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<metamodelica::Ref<NBTearing>> {
    let mut tearingSet: metamodelica::Ref<NBTearing> = metamodelica::Ref::new(NBTearing {
        iteration_vars: list![var.clone()],
        residual_eqns: scalarSlices(eqn.clone())?,
        innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        jac: None,
    });
    Ok(tearingSet)
}

pub(crate) fn scalarSlices(
    mut eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
> {
    let mut slices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = Slice::getT(eqn.clone());
    let mut e: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    let mut indices: metamodelica::List<i32> = eqn.indices.clone();
    let mut base_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut residual: metamodelica::Ref<Expression::NFExpression>;
    let mut elem_ty: metamodelica::Ref<Type::NFType>;
    let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut row_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut row_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut row_var_data: metamodelica::Ref<Variable::NFVariable>;
    let mut row_residual: metamodelica::Ref<Expression::NFExpression>;
    let mut row_eqn: metamodelica::Ref<Equation::Equation>;
    let mut is_for: bool;
    if (indices).is_empty() {
        indices = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (0..=BEquation::Equation::size(eqn_ptr.clone(), false)? - 1).into_iter() {
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    if List::hasOneElement(&indices) && BEquation::Equation::size(eqn_ptr.clone(), false)? == 1 {
        slices = list![eqn];
    } else {
        base_cref = BEquation::Equation::getEqnName(eqn_ptr.clone())?;
        is_for = BEquation::Equation::isSingleBodyFor(&e);
        residual = if (is_for) {
            metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                ty: openmodelica_nf_frontend::NFType::interned_UNKNOWN(),
            })
        } else {
            BEquation::Equation::getResidualExp(&e, true)?
        };
        elem_ty = Type::arrayElementType(&(Expression::typeOf(residual.clone())));
        attr = BEquation::Equation::getAttributes(e.clone());
        assign_field!(attr.residual = true);
        slices = metamodelica::nil();
        for mut i in &*indices {
            if is_for {
                row_residual = BEquation::Equation::forArrayBodyRowResidual(&e, i.clone())?;
                elem_ty = Type::arrayElementType(&(Expression::typeOf(row_residual.clone())));
            } else {
                subs = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                    for mut l in
                        (Slice::indexToLocation(i.clone(), BEquation::Equation::sizes(eqn_ptr.clone(), false)?))
                            .into_iter()
                            .cloned()
                    {
                        let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: l.clone() + 1 }),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                row_residual = Expression::applySubscripts(&subs, residual.clone(), false)?;
            }
            (row_var, row_cref) = BVariable::makeAuxVar(
                &(ComponentRef::toString(&base_cref)?),
                i.clone(),
                elem_ty.clone(),
                false,
            )?;
            row_var_data = Pointer::access(row_var.clone());
            assign_field!(
                row_var_data.backendinfo = BackendInfo::setVarKind(
                    row_var_data.backendinfo.clone(),
                    openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_RESIDUAL_VAR()
                )
            );
            Pointer::update(row_var.clone(), row_var_data);
            assign_field!(attr.residualVar = Some(row_var));
            row_eqn = metamodelica::Ref::new(Equation::Equation::SCALAR_EQUATION {
                ty: elem_ty.clone(),
                lhs: Expression::fromCref(row_cref, false)?,
                rhs: row_residual,
                source: BEquation::Equation::getSource(e.clone()),
                attr: attr.clone(),
            });
            slices = metamodelica::cons(
                metamodelica::Ref::new(Slice::NBSlice {
                    t: Pointer::create(row_eqn),
                    indices: metamodelica::nil(),
                }),
                slices,
            );
        }
        slices = slices.reverse();
    }
    Ok(slices)
}

pub(crate) fn containsNamedCref(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut names: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut b: bool,
) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b, exp.clone())) {
        (false, Deref @ Expression::CREF { .. }) => UnorderedSet::contains(ComponentRef::stripSubscriptsAll(var_field!((**exp).cref, Expression::NFExpression::CREF)), names)?,
        _ => b,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isLinearSlice(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<bool> {
    let mut linear: bool = true;
    let mut residual_opt: Option<metamodelica::Ref<Expression::NFExpression>> =
        BEquation::Equation::tryGetResidualExp(eqn_ptr.clone());
    let mut residual: metamodelica::Ref<Expression::NFExpression>;
    let mut diffArgs: metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>;
    let mut derivative: metamodelica::Ref<Expression::NFExpression>;
    let mut names: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::fromList(
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut c in (crefs.clone()).into_iter().cloned() {
                    let __x = ComponentRef::stripSubscriptsAll(&(c.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
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
        )?;
    let mut occurrences: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    if (residual_opt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(residual_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        residual = metamodelica::Own::own(__pa0);
        occurrences = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut c in (UnorderedSet::toList(Expression::extractCrefs(residual.clone())?))
                .into_iter()
                .cloned()
            {
                if !(UnorderedSet::contains(ComponentRef::stripSubscriptsAll(&(c.clone())), names.clone())?) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if (occurrences).is_empty() {
            occurrences = crefs;
        }
        if '__try1: {
            for mut cref in &*occurrences {
                diffArgs = Differentiate::DifferentiationArguments::simpleCref(cref.clone(), funcMap.clone());
                (derivative, diffArgs) = unwrap_break_err!(Differentiate::differentiateExpressionDump(residual.clone(), diffArgs.clone(), &(literal!("NBTearing.isLinearSlice")), &(literal!(""))), '__try1);
                if unwrap_break_err!(Expression::fold(derivative.clone(), (std::sync::Arc::new({ let __pe_b1 = names.clone(); move |__pe_a0, __pe_a2| containsNamedCref(&__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static>), false), '__try1) {
                    linear = false;
                }
            }
            Ok::<(), &'static str>(())
        }.is_err() {
            linear = false;
        }
    } else {
        linear = false;
    }
    Ok(linear)
}

pub(crate) fn getModule() -> Result<
    metamodelica::List<
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<StrongComponent::NBStrongComponent>,
                    metamodelica::Ref<Adjacency::Matrix::Matrix>,
                    metamodelica::Ref<
                        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                    >,
                    i32,
                    metamodelica::Ref<VariablePointers::VariablePointers>,
                    metamodelica::Ref<EquationPointers::EquationPointers>,
                    Pointer::Pointer<i32>,
                    Partition::Kind,
                ) -> Result<(
                    metamodelica::Ref<StrongComponent::NBStrongComponent>,
                    metamodelica::Ref<Adjacency::Matrix::Matrix>,
                    i32,
                )> + 'static,
        >,
    >,
> {
    fn isNotGuruVar(
        mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut init: bool,
    ) -> Result<bool> {
        let mut b: bool;
        let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
        b = BVariable::hasTearingSelect(
            var_ptr,
            NFBackendExtension::TearingSelect::PREFER.clone(),
            &fnptr!(intLt, i32, i32),
        )?;
        Ok(b)
    }

    let mut funcs: metamodelica::List<Module::tearingInterface>;
    let mut flag: ArcStr = Flags::getConfigString(Flags::TEARING_METHOD.clone())?;
    funcs = (::match_deref::match_deref! { match &(flag) {
        Deref @ "minimalTearing" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(BVariable::isDiscontinuous) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(BEquation::Equation::isDiscontinuous, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(minimal) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        Deref @ "cellier" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(BVariable::isDiscontinuous) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(BEquation::Equation::isDiscontinuous, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(minimal) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(cellier) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        Deref @ "omcTearing" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(BVariable::isDiscontinuous) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(BEquation::Equation::isDiscontinuous, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(minimal) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        Deref @ "guruTearing" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(isNotGuruVar) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(noFilterEqn, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(guru) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, i32, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, metamodelica::Ref<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub(crate) fn getVariables(
    mut tearing: &metamodelica::Ref<NBTearing>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut variables: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    variables = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut var in (metamodelica::cons(
            ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
                    let __x = Slice::getT(var.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            ({
                let mut __acc: metamodelica::List<
                    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                > = metamodelica::nil();
                for mut comp in (tearing.innerEquations.clone()).borrow().iter() {
                    let __x = StrongComponent::getVariables(comp.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        ))
        .into_iter()
        .cloned()
        {
            let __x = var.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    });
    Ok(variables)
}

pub(crate) fn getResidualVars(
    mut tearing: &metamodelica::Ref<NBTearing>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut residuals: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::getResidualVar(Slice::getT(eqn.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(residuals)
}

pub(crate) fn getIterationVars(
    mut tearing: &metamodelica::Ref<NBTearing>,
) -> metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut iterationVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    iterationVars
}

pub(crate) fn getResidualEqns(
    mut tearing: &metamodelica::Ref<NBTearing>,
) -> metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut residuals: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    residuals
}

pub(crate) fn setResidualEqns(
    mut tearing: metamodelica::Ref<NBTearing>,
    mut residuals: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
) -> metamodelica::Ref<NBTearing> {
    let mut tearing: metamodelica::Ref<NBTearing> = tearing;
    assign_field!(tearing.residual_eqns = residuals);
    tearing
}

fn tearingTraverser(
    mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
    mut funcs: &metamodelica::List<
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<StrongComponent::NBStrongComponent>,
                    metamodelica::Ref<Adjacency::Matrix::Matrix>,
                    metamodelica::Ref<
                        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                    >,
                    i32,
                    metamodelica::Ref<VariablePointers::VariablePointers>,
                    metamodelica::Ref<EquationPointers::EquationPointers>,
                    Pointer::Pointer<i32>,
                    Partition::Kind,
                ) -> Result<(
                    metamodelica::Ref<StrongComponent::NBStrongComponent>,
                    metamodelica::Ref<Adjacency::Matrix::Matrix>,
                    i32,
                )> + 'static,
        >,
    >,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut eq_index: Pointer::Pointer<i32>,
    mut kind: Partition::Kind,
) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> {
    let mut new_partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> =
        metamodelica::nil();
    let mut strongComponents: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut tmp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut idx: i32 = 0;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    for mut part in &**partitions {
        let mut part = part.clone();
        if (part.strongComponents).is_some() && (part.adjacencyMatrix).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(part.strongComponents.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            strongComponents = metamodelica::Own::own(__pa0);
            let __pa1 = ::match_deref::match_deref! { match &(part.adjacencyMatrix.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            full = metamodelica::Own::own(__pa1);
            for mut i in 1..=metamodelica::arrayLength(strongComponents.clone()) {
                tmp = ({
                    let __elt = (*metamodelica::index_checked(&strongComponents.borrow(), i)?).clone();
                    __elt
                });
                for mut func in &**funcs {
                    (tmp, full, idx) = func(
                        tmp,
                        full,
                        funcMap.clone(),
                        idx,
                        part.unknowns.clone(),
                        part.equations.clone(),
                        eq_index.clone(),
                        kind,
                    )?;
                }
                if !(referenceEq(
                    &*(&*tmp),
                    &*({
                        let __elt = (*metamodelica::index_checked(&strongComponents.borrow(), i)?).clone();
                        __elt
                    }),
                )) {
                    metamodelica::arrayUpdate(strongComponents.clone(), i, tmp)?;
                }
            }
            assign_field!(
                part.strongComponents = Some(strongComponents.clone()),
                part.adjacencyMatrix = Some(full)
            );
        }
        new_partitions = metamodelica::cons(part, new_partitions);
    }
    new_partitions = new_partitions.reverse();
    Ok(new_partitions)
}

fn noFilterVar(mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, mut init: bool) -> bool {
    let mut b: bool;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = true;
    b
}

fn noFilterEqn(mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> bool {
    let mut b: bool;
    b = true;
    b
}

fn tolerantSubMap(
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>> {
    let mut sub_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
    > = UnorderedMap::subMap(
        map.clone(),
        &({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut k in (lst.clone()).into_iter().cloned() {
                if !(UnorderedMap::contains(k.clone(), map.clone())?) {
                    continue;
                }
                let __x = k.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    Ok(sub_map)
}

fn initialize(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut index: i32,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eq_index: Pointer::Pointer<i32>,
    mut kind: Partition::Kind,
    mut varFunc: Arc<
        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static,
    >,
    mut eqnFunc: Arc<
        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static,
    >,
) -> Result<(
    metamodelica::Ref<StrongComponent::NBStrongComponent>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    i32,
)> {
    pub type checkVarInit = std::sync::Arc<
        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, bool) -> Result<bool> + 'static,
    >;

    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: metamodelica::Ref<NBTearing>;
    let mut all_vars_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut all_eqns_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut vars_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut v_all: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut e_all: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    (comp, full, index) = (match &*comp.clone() {
        StrongComponent::ALGEBRAIC_LOOP {
            strict: __esc_strict, ..
        } => {
            strict = (*__esc_strict).clone();
            index = index + 1;
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; idx = index);
            all_vars_lst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
                    let __x = BVariable::getVarName(Slice::getT(var.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            all_eqns_lst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                    let __x = BEquation::Equation::getEqnName(Slice::getT(eqn.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            vars_set = UnorderedSet::fromList(
                &all_vars_lst,
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
            v_all = tolerantSubMap(variables.map.clone(), all_vars_lst)?;
            e_all = tolerantSubMap(equations.map.clone(), all_eqns_lst)?;
            full = Adjacency::Matrix::refine(
                full,
                funcMap,
                v_all.clone(),
                e_all.clone(),
                &variables,
                &equations,
                vars_set,
                Partition::kindIsInitial(kind),
            )?;
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; linear = checkLinearity(&full, v_all, e_all)?);
            (comp, full, index)
        }
        _ => (comp, full, index),
    });
    Ok((comp, full, index))
}

fn isPartialVarSlice(
    mut var: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<bool> {
    let mut b: bool =
        !((var.indices).is_empty()) && BVariable::size(Slice::getT(var.clone()), false)? > ((var.indices).len() as i32);
    Ok(b)
}

fn isPartialArraySlice(
    mut eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<bool> {
    let mut b: bool = !((eqn.indices).is_empty())
        && (BEquation::Equation::isArrayEquation(Slice::getT(eqn.clone()))
            || BEquation::Equation::isSingleBodyFor(&(Pointer::access(Slice::getT(eqn.clone())))))
        && BEquation::Equation::size(Slice::getT(eqn.clone()), false)? > ((eqn.indices).len() as i32);
    Ok(b)
}

fn isArrayBodyFor(
    mut eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(Pointer::access(Slice::getT(eqn))) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            Type::isArray(&(BEquation::Equation::getType(metamodelica::AsArg::as_arg(&body), false)?))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn finalize(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut index: i32,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eq_index: Pointer::Pointer<i32>,
    mut kind: Partition::Kind,
) -> Result<(
    metamodelica::Ref<StrongComponent::NBStrongComponent>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    i32,
)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: metamodelica::Ref<NBTearing>;
    let mut partial_vars: bool;
    let mut acc: metamodelica::List<
        metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
    >;
    let mut dummy_set: metamodelica::Ref<
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
    comp = (match &*comp {
        StrongComponent::ALGEBRAIC_LOOP {
            strict: __esc_strict, ..
        } => {
            strict = (*__esc_strict).clone();
            acc = ({
                let mut __acc: metamodelica::List<
                    metamodelica::List<
                        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
                    >,
                > = metamodelica::nil();
                for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                    let __x = Inline::inlineRecordSliceEquation(
                        eqn.clone(),
                        &variables,
                        dummy_set.clone(),
                        eq_index.clone(),
                        true,
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            partial_vars = List::any(&strict.iteration_vars, &isPartialVarSlice)?;
            assign_field!(
                strict.residual_eqns = ({
                    let mut __acc: metamodelica::List<
                        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
                    > = metamodelica::nil();
                    for mut eqn in (List::flatten(
                        ({
                            let mut __acc: metamodelica::List<
                                metamodelica::List<
                                    metamodelica::Ref<
                                        Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                                    >,
                                >,
                            > = metamodelica::nil();
                            for mut eqn in (List::flatten(acc)?).into_iter().cloned() {
                                let __x = if (isPartialArraySlice(eqn.clone())?
                                    || isArrayBodyFor(eqn.clone())?
                                    || partial_vars
                                        && BEquation::Equation::isSingleBodyFor(
                                            &(Pointer::access(Slice::getT(eqn.clone()))),
                                        )) {
                                    scalarSlices(eqn.clone())?
                                } else {
                                    list![eqn.clone()]
                                };
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?)
                    .into_iter()
                    .cloned()
                    {
                        let __x = Slice::apply(
                            eqn.clone(),
                            &({
                                let __pe_b1 = None;
                                let __pe_b2 = true;
                                let __pe_b3 = false;
                                move |__pe_a0| {
                                    BEquation::Equation::createResidual(
                                        __pe_a0,
                                        __pe_b1.clone(),
                                        __pe_b2.clone(),
                                        __pe_b3.clone(),
                                    )
                                }
                            }),
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            );
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            if Flags::isSet(Flags::TEARING_DUMP.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("["));
                            __mm_s.push_str(&*Partition::Partition::kindToString(kind)?);
                            __mm_s.push_str(&*literal!("] Tearing Result "));
                            __mm_s.push_str(&*intString(
                                var_field!((*comp).idx, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(),
                            ));
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            comp
        }
        _ => comp,
    });
    Ok((comp, full, index))
}

fn minimal(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut index: i32,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eq_index: Pointer::Pointer<i32>,
    mut kind: Partition::Kind,
) -> Result<(
    metamodelica::Ref<StrongComponent::NBStrongComponent>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    i32,
)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: metamodelica::Ref<NBTearing>;
    let mut innerStrict: metamodelica::Ref<NBTearing>;
    let mut vars_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut cont_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut disc_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut implied_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut alg_implied: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut eqns_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut cont_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut disc_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut alg_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut num_vars: i32;
    let mut num_eqns: i32;
    let mut iteration_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = metamodelica::nil();
    let mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut sub: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut matching: metamodelica::Ref<Matching::NBMatching>;
    let mut inner_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut disc_variables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut disc_equations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut matched_set: metamodelica::Ref<
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
    comp = (match &*comp {
        StrongComponent::ALGEBRAIC_LOOP {
            strict: __esc_strict, ..
        } => {
            strict = (*__esc_strict).clone();
            vars_lst = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
                    let __x = Slice::getT(var.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            eqns_lst = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                    metamodelica::nil();
                for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                    let __x = Slice::getT(eqn.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            (cont_vars, disc_vars) = filterDiscreteVariables(&vars_lst, Partition::kindIsInitial(kind))?;
            (cont_eqns, disc_eqns) = List::splitOnTrue(&eqns_lst, &BEquation::Equation::isContinousRecordAware)?;
            (alg_eqns, cont_eqns) = List::splitOnTrue(
                &cont_eqns,
                &fnptr!(
                    BEquation::Equation::isAlgorithm,
                    Pointer::Pointer<metamodelica::Ref<Equation::Equation>>
                ),
            )?;
            implied_vars = List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                    > = metamodelica::nil();
                    for mut eqn in (listAppend(alg_eqns.clone(), disc_eqns.clone())).into_iter().cloned() {
                        let __x = getImpliedInnerVars(eqn.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            disc_vars = UnorderedSet::unique_list(
                listAppend(disc_vars, implied_vars.clone()),
                (std::sync::Arc::new(BVariable::hash)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                            + 'static,
                    >),
                (std::sync::Arc::new(BVariable::equalName)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                            ) -> Result<bool>
                            + 'static,
                    >),
            )?;
            cont_vars = UnorderedSet::difference_list(
                cont_vars,
                implied_vars,
                (std::sync::Arc::new(BVariable::hash)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                            + 'static,
                    >),
                (std::sync::Arc::new(BVariable::equalName)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                            ) -> Result<bool>
                            + 'static,
                    >),
            )?;
            num_vars = ({
                let mut __acc: i32 = 0;
                for mut var in (disc_vars.clone()).into_iter().cloned() {
                    let __x = BVariable::size(var.clone(), false)?;
                    __acc += __x;
                }
                __acc
            });
            num_eqns = ({
                let mut __acc: i32 = 0;
                for mut eqn in (disc_eqns.clone()).into_iter().cloned() {
                    let __x = BEquation::Equation::size(eqn.clone(), false)?;
                    __acc += __x;
                }
                __acc
            }) + ({
                let mut __acc: i32 = 0;
                for mut eqn in (alg_eqns.clone()).into_iter().cloned() {
                    let __x = BEquation::Equation::size(eqn.clone(), false)?;
                    __acc += __x;
                }
                __acc
            });
            if !((disc_eqns).is_empty() && (alg_eqns).is_empty()) {
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; mixed = true);
                inner_comps = metamodelica::nil();
                for mut alg_eqn in &*alg_eqns {
                    alg_implied = getImpliedInnerVars(alg_eqn.clone())?;
                    for mut var in &*alg_implied {
                        UnorderedSet::add(BVariable::getVarName(var.clone()), matched_set.clone())?;
                    }
                    inner_comps = metamodelica::cons(
                        metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                            vars: ({
                                let mut __acc: metamodelica::List<
                                    metamodelica::Ref<
                                        Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                                    >,
                                > = metamodelica::nil();
                                for mut var in (alg_implied).into_iter().cloned() {
                                    let __x = metamodelica::Ref::new(Slice::NBSlice {
                                        t: var.clone(),
                                        indices: metamodelica::nil(),
                                    });
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                            eqn: metamodelica::Ref::new(Slice::NBSlice {
                                t: alg_eqn.clone(),
                                indices: metamodelica::nil(),
                            }),
                            status: Solve::Status::UNPROCESSED.clone(),
                        }),
                        inner_comps,
                    );
                }
                if !((disc_eqns).is_empty()) {
                    disc_vars = sortByIndex(
                        ({
                            let mut __acc: metamodelica::List<
                                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                            > = metamodelica::nil();
                            for mut var in (disc_vars).into_iter().cloned() {
                                if !(!(UnorderedSet::contains(
                                    BVariable::getVarName(var.clone()),
                                    matched_set.clone(),
                                )?)) {
                                    continue;
                                }
                                let __x = var.clone();
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        &fnptr!(
                            BVariable::getVarName,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                        ),
                        variables.map.clone(),
                    )?;
                    disc_eqns = sortByIndex(disc_eqns, &BEquation::Equation::getEqnName, equations.map.clone())?;
                    disc_variables = BVariable::VariablePointers::fromList(&disc_vars, false)?;
                    disc_equations = BEquation::EquationPointers::fromList(&disc_eqns)?;
                    sub = Adjacency::Matrix::subFull(
                        &full,
                        &({
                            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                            for mut eqn in (disc_eqns).into_iter().cloned() {
                                let __x = UnorderedMap::getSafe(
                                    BEquation::Equation::getEqnName(eqn.clone())?,
                                    equations.map.clone(),
                                    metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                                )?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        &disc_equations,
                        &disc_variables,
                    )?;
                    adj = Adjacency::Matrix::fullToFinal(
                        &sub,
                        disc_variables.map.clone(),
                        disc_equations.map.clone(),
                        &(disc_equations.clone()),
                        Adjacency::MatrixStrictness::MATCHING.clone(),
                        &(crate::NBEquation::Iterator::interned_EMPTY()),
                    )?;
                    matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), &adj, true, true, true)?;
                    for mut var in &*Matching::getMatchedVars(
                        &matching,
                        Adjacency::Matrix::getMappingOpt(&adj),
                        disc_variables.map.clone(),
                        &(disc_variables.clone()),
                    )? {
                        UnorderedSet::add(BVariable::getVarName(var.clone()), matched_set.clone())?;
                    }
                    adj = Adjacency::Matrix::upgrade(
                        adj,
                        &sub,
                        disc_variables.map.clone(),
                        disc_equations.map.clone(),
                        &(disc_equations.clone()),
                        Adjacency::MatrixStrictness::SORTING.clone(),
                        &(crate::NBEquation::Iterator::interned_EMPTY()),
                    )?;
                    inner_comps = listAppend(
                        Sorting::tarjan(adj, matching, &disc_variables, &disc_equations)?,
                        inner_comps,
                    );
                }
                for mut var in &*strict.iteration_vars.clone() {
                    if !(UnorderedSet::contains(BVariable::getVarName(Slice::getT(var.clone())), matched_set.clone())?)
                    {
                        iteration_vars = metamodelica::cons(var.clone(), iteration_vars);
                    }
                }
                assign_field!(
                    strict.innerEquations = metamodelica::arrayFromVec(inner_comps.into_iter().cloned().collect()),
                    strict.residual_eqns = ({
                        let mut __acc: metamodelica::List<
                            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
                        > = metamodelica::nil();
                        for mut eqn in (cont_eqns).into_iter().cloned() {
                            let __x = metamodelica::Ref::new(Slice::NBSlice {
                                t: eqn.clone(),
                                indices: metamodelica::nil(),
                            });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    strict.iteration_vars = iteration_vars.reverse()
                );
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            }
            comp
        }
        _ => comp,
    });
    Ok((comp, full, index))
}

fn sortByIndex<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lst: metamodelica::List<T>,
    mut func: &dyn ::std::ops::Fn(T) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<metamodelica::List<T>> {
    pub type getName<T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> + 'static>;

    let mut sorted: metamodelica::List<T>;
    let mut indexed: metamodelica::List<(i32, T)> = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut t in (lst.clone()).into_iter().cloned() {
            let __x = (
                UnorderedMap::getSafe(
                    func(t.clone())?,
                    map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                )?,
                t.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    indexed = List::sort(
        indexed,
        (std::sync::Arc::new(move |__pe_a0, __pe_a1| Ok(Util::compareTupleIntGt(__pe_a0, __pe_a1)))
            as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
    )?;
    sorted = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut tpl in (indexed).into_iter().cloned() {
            let __x = Util::tuple22(tpl.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(sorted)
}

fn guru(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut index: i32,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eq_index: Pointer::Pointer<i32>,
    mut kind: Partition::Kind,
) -> Result<(
    metamodelica::Ref<StrongComponent::NBStrongComponent>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    i32,
)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut inner_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut residuals: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    > = metamodelica::nil();
    let mut strict: metamodelica::Ref<NBTearing>;
    let mut nEqn: i32;
    let mut inner_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut guru_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut failed_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut unsolved_inner_vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        >,
    >;
    let mut unsolved_equations: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        >,
    >;
    let mut solve_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut solve_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut solve_var: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;
    let mut solve_eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
    let mut success: bool;
    let mut var_assigned: bool = false;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let staticAsContinuous: bool = Partition::kindIsInitial(kind);
    comp = (::match_deref::match_deref! { match &((comp.clone(), full.clone())) {
        (Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: __esc_strict, .. }, Deref @ Adjacency::Matrix::FULL { .. }) => {
            strict = (*__esc_strict).clone();
            nEqn = metamodelica::arrayLength(var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).clone());
            (inner_vars, guru_vars) = List::splitOnTrue(&strict.iteration_vars, &({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1 = NFBackendExtension::TearingSelect::PREFER.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0| BVariable::hasTearingSelect(__pe_a0, __pe_b1.clone(), &*__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0| Slice::check(__pe_a0, &*__pe_b1) }))?;
            if (guru_vars).is_empty() {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. No guru variables provided for strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?); ArcStr::from(__mm_s) }])?;
                return Err("fail");
            } else {
                failed_vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>> = metamodelica::nil();
        for mut var in (guru_vars.clone()).into_iter().cloned() {
            if !(Slice::check(var.clone(), &({ let __pe_b1 = staticAsContinuous; move |__pe_a0| BVariable::isDiscontinuous(__pe_a0, __pe_b1.clone()) }))?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                if !((failed_vars).is_empty()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. Following variables cannot be chosen as iteration variables because they are discontinuous:\n")); __mm_s.push_str(&*List::toString(failed_vars, &({ let __pe_b1 = (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr> + 'static>); let __pe_b2 = 10; move |__pe_a0| Slice::toString(__pe_a0, &*__pe_b1, __pe_b2.clone()) }), List::Style::NEWLINE_TAB.clone())?); ArcStr::from(__mm_s) }])?;
                    return Err("fail");
                }
                unsolved_inner_vars = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                for mut var in &*inner_vars {
                    UnorderedMap::add(BVariable::getVarName(Slice::getT(var.clone())), var.clone(), unsolved_inner_vars.clone())?;
                }
                unsolved_equations = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                for mut eqn in &*strict.residual_eqns.clone() {
                    UnorderedMap::add(BEquation::Equation::getEqnName(Slice::getT(eqn.clone()))?, eqn.clone(), unsolved_equations.clone())?;
                }
                while !(UnorderedMap::isEmpty(unsolved_inner_vars.clone())) {
                    for mut i in 1..=nEqn {
                        var_assigned = false;
                        if UnorderedMap::contains(({let __elt = (*metamodelica::index_checked(&var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}), unsolved_equations.clone())? {
                            solve_opt = None;
                            success = false;
                            let __range0 = &*UnorderedSet::toList(({let __elt = (*metamodelica::index_checked(&var_field!((*full).occurrences, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}));
                            for mut cref in __range0 {
                                stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref));
                                if UnorderedMap::contains(stripped, unsolved_inner_vars.clone())? {
                                    if (solve_opt).is_none() {
                                        success = true;
                                        solve_opt = Some(cref.clone());
                                    } else {
                                        success = false;
                                        break;
                                    }
                                }
                            }
                            let () = (::match_deref::match_deref! { match &((solve_opt, success)) {
        (Some(__esc_solve_cref), true) => {
            solve_cref = (*__esc_solve_cref).clone();
            stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&solve_cref));
            solve_var = UnorderedMap::getSafe(stripped.clone(), unsolved_inner_vars.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            solve_eqn = UnorderedMap::getSafe(({let __elt = (*metamodelica::index_checked(&var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}), unsolved_equations.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            inner_comps = metamodelica::cons(StrongComponent::createSliceOrSingle(solve_cref.clone(), solve_var, solve_eqn)?, inner_comps);
            UnorderedMap::remove(stripped, unsolved_inner_vars.clone())?;
            UnorderedMap::remove(({let __elt = (*metamodelica::index_checked(&var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}), unsolved_equations.clone())?;
            var_assigned = true;
            ()
        },
        (Some(__esc_solve_cref), false) => {
            solve_cref = (*__esc_solve_cref).clone();
            ()
        },
        (None, false) => {
            residuals = metamodelica::cons(UnorderedMap::getSafe(({let __elt = (*metamodelica::index_checked(&var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}), unsolved_equations.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?, residuals);
            UnorderedMap::remove(({let __elt = (*metamodelica::index_checked(&var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}), unsolved_equations.clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. Impossible result for equation representative: ")); __mm_s.push_str(&*ComponentRef::toString(&({let __elt = (*metamodelica::index_checked(&var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow(), i)?).clone(); __elt}))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        }
                        if var_assigned {
                            break;
                        }
                    }
                    if !(var_assigned) {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. Following variables could not be solved as inner variables:\n")); __mm_s.push_str(&*List::toString(UnorderedMap::valueList(unsolved_inner_vars.clone()), &({ let __pe_b1 = (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr> + 'static>); let __pe_b2 = 10; move |__pe_a0| Slice::toString(__pe_a0, &*__pe_b1, __pe_b2.clone()) }), List::Style::NEWLINE_TAB.clone())?); ArcStr::from(__mm_s) }])?;
                        return Err("fail");
                    }
                }
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; mixed = List::any(&inner_vars, &({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| BVariable::isDiscontinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0| Slice::check(__pe_a0, &*__pe_b1) }))?);
                assign_field!(
                    strict.innerEquations = metamodelica::arrayFromVec(inner_comps.reverse().into_iter().cloned().collect()),
                    strict.residual_eqns = listAppend(UnorderedMap::valueList(unsolved_equations), residuals),
                    strict.iteration_vars = guru_vars
                );
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            }
            comp
        },
        _ => comp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, full, index))
}

fn cellier(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut index: i32,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eq_index: Pointer::Pointer<i32>,
    mut kind: Partition::Kind,
) -> Result<(
    metamodelica::Ref<StrongComponent::NBStrongComponent>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    i32,
)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: metamodelica::Ref<NBTearing>;
    let mut cellier_strict: metamodelica::Ref<NBTearing>;
    comp = (::match_deref::match_deref! { match &((comp.clone(), full.clone())) {
        (Deref @ StrongComponent::ALGEBRAIC_LOOP { strict, .. }, Deref @ Adjacency::Matrix::FULL { .. }) if (!((strict.iteration_vars).is_empty()) && !((strict.residual_eqns).is_empty())) => {
            cellier_strict = cellierTearingSet(strict.clone(), &full, &equations, funcMap)?;
            if !(var_field!((*comp).linear, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone() && linearSystemCapable(metamodelica::AsArg::as_arg(&strict))? && !(linearSystemCapable(&cellier_strict)?)) {
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = cellier_strict);
            }
            comp
        },
        _ => comp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, full, index))
}

fn linearSystemCapable(mut strict: &metamodelica::Ref<NBTearing>) -> Result<bool> {
    let mut b: bool;
    let mut partial_vars: bool = List::any(&strict.iteration_vars, &isPartialVarSlice)?;
    b = !(Array::any(strict.innerEquations.clone(), &isForComponent)?)
        && (partial_vars
            || !(List::any(
                &({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                        metamodelica::nil();
                    for mut e in (strict.residual_eqns.clone()).into_iter().cloned() {
                        let __x = Slice::getT(e.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                &fnptr!(
                    BEquation::Equation::isForEquation,
                    Pointer::Pointer<metamodelica::Ref<Equation::Equation>>
                ),
            )?))
        && (!(partial_vars) || Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())?);
    Ok(b)
}

fn isForComponent(mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>) -> Result<bool> {
    let mut b: bool = List::any(
        &(StrongComponent::getEquations(comp.clone())?),
        &fnptr!(
            BEquation::Equation::isForEquation,
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>
        ),
    )?;
    Ok(b)
}

fn cellierHasRow(
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<bool> {
    let mut b: bool = UnorderedMap::contains(BEquation::Equation::getEqnName(eqn.clone())?, map.clone())?;
    Ok(b)
}

fn cellierTearingSet(
    mut strict: metamodelica::Ref<NBTearing>,
    mut full: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<metamodelica::Ref<NBTearing>> {
    let mut strict: metamodelica::Ref<NBTearing> = strict;
    let mut probes: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, bool>> = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        1,
    );
    let mut loop_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut block_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut loop_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut vars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut eqns: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut mapping: metamodelica::Ref<Adjacency::Mapping::Mapping>;
    let mut nv: i32;
    let mut ne: i32;
    let mut nb: i32;
    let mut idx: i32;
    let mut start: i32;
    let mut size: i32;
    let mut unit_kind: metamodelica::Array<i32>;
    let mut eqn_glob: metamodelica::Array<i32>;
    let mut owner: metamodelica::Array<i32>;
    let mut eqn_step: metamodelica::Array<i32>;
    let mut row_var: metamodelica::Array<i32>;
    let mut block_step: metamodelica::Array<i32>;
    let mut var_slices: metamodelica::Array<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut eqn_slices: metamodelica::Array<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut block_in: metamodelica::Array<metamodelica::List<i32>>;
    let mut block_out: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqn_rows: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqn_units: metamodelica::Array<metamodelica::List<i32>>;
    let mut blocks: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut tears: metamodelica::List<i32>;
    let mut fixed: metamodelica::List<i32> = metamodelica::nil();
    let mut trial: metamodelica::List<i32>;
    let mut opt_idx: Option<i32>;
    let mut complete: bool;
    let mut ordered: metamodelica::List<(i32, metamodelica::Ref<StrongComponent::NBStrongComponent>)> =
        metamodelica::nil();
    let mut tear_set: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;
    let mut occurrences: metamodelica::Array<
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    > = Default::default();
    let mut solvabilities: metamodelica::Array<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Solvability::Solvability>,
            >,
        >,
    > = Default::default();
    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
    let () = (match &**full {
        Adjacency::Matrix::FULL { .. } => {
            occurrences = var_field!((**full).occurrences, Adjacency::Matrix::Matrix::FULL).clone();
            solvabilities = var_field!((**full).solvabilities, Adjacency::Matrix::Matrix::FULL).clone();
            ()
        }
        _ => return Err("fail"),
    });
    blocks = strict.innerEquations.clone();
    nb = metamodelica::arrayLength(blocks.clone());
    loop_vars = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut b in ({
        let __s = nb;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        block_vars = listAppend(
            StrongComponent::getVariables(
                ({
                    let __elt = (*metamodelica::index_checked(&blocks.borrow(), b)?).clone();
                    __elt
                }),
            )?,
            block_vars,
        );
    }
    loop_eqns = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    vars = BVariable::VariablePointers::fromList(&(listAppend(loop_vars.clone(), block_vars.clone())), false)?;
    eqns = BEquation::EquationPointers::fromList(&loop_eqns)?;
    nv = BVariable::VariablePointers::size(&vars);
    ne = BEquation::EquationPointers::size(&eqns);
    if nv != ((loop_vars).len() as i32) + ((block_vars).len() as i32) || ne != ((loop_eqns).len() as i32) {
        return Ok(strict);
    }
    for mut b in 1..=nb {
        loop_eqns = listAppend(
            StrongComponent::getEquations(
                ({
                    let __elt = (*metamodelica::index_checked(&blocks.borrow(), b)?).clone();
                    __elt
                }),
            )?,
            loop_eqns,
        );
    }
    if !(List::all(
        &loop_eqns,
        &({
            let __pe_b1 = equations.map.clone();
            move |__pe_a0| cellierHasRow(__pe_a0, __pe_b1.clone())
        }),
    )?) {
        return Ok(strict);
    }
    loop_eqns = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unit_kind = arrayCreate(nv, 3);
    var_slices = arrayCreate(nv, (strict.iteration_vars).head().cloned()?);
    for mut var in &*strict.iteration_vars.clone() {
        idx = UnorderedMap::getSafe(
            BVariable::getVarName(Slice::getT(var.clone())),
            vars.map.clone(),
            metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
        )?;
        {
            let __cell0 = 1;
            let __idx0 = idx;
            *metamodelica::index_mut_checked(&mut unit_kind.clone().borrow_mut(), __idx0)? = __cell0;
        }
        {
            let __cell1 = var.clone();
            let __idx1 = idx;
            *metamodelica::index_mut_checked(&mut var_slices.clone().borrow_mut(), __idx1)? = __cell1;
        }
        if ((BVariable::getTearingSelect(Slice::getT(var.clone()))?) as i32)
            == ((NFBackendExtension::TearingSelect::ALWAYS.clone()) as i32)
        {
            fixed = metamodelica::cons(idx, fixed);
        }
    }
    eqn_glob = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut eqn in (loop_eqns).into_iter().cloned() {
                let __x = UnorderedMap::getSafe(
                    BEquation::Equation::getEqnName(eqn.clone())?,
                    equations.map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    adj = Adjacency::Matrix::subFull(
        full,
        &(eqn_glob
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>()),
        &eqns,
        &vars,
    )?;
    adj = Adjacency::Matrix::fullToFinal(
        &adj,
        vars.map.clone(),
        eqns.map.clone(),
        &(eqns.clone()),
        Adjacency::MatrixStrictness::SORTING.clone(),
        &(crate::NBEquation::Iterator::interned_EMPTY()),
    )?;
    mapping = Adjacency::Matrix::getMappingOpt(&adj).ok_or("pattern mismatch")?;
    eqn_slices = metamodelica::arrayFromVec(strict.residual_eqns.clone().into_iter().cloned().collect());
    eqn_rows = arrayCreate(ne, metamodelica::nil());
    for mut i in 1..=ne {
        eqn_slice = ({
            let __elt = (*metamodelica::index_checked(&eqn_slices.borrow(), i)?).clone();
            __elt
        });
        (start, size) = ({
            let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), i)?).clone();
            __elt
        });
        {
            let __cell2 = if ((eqn_slice.indices).is_empty()) {
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut k in (0..=size - 1).into_iter() {
                        let __x = start + k.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            } else {
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut k in (List::sort(
                        eqn_slice.indices.clone(),
                        (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    )?)
                    .into_iter()
                    .cloned()
                    {
                        let __x = start + k.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            };
            let __idx2 = i;
            *metamodelica::index_mut_checked(&mut eqn_rows.clone().borrow_mut(), __idx2)? = __cell2;
        }
    }
    owner = arrayCreate(nv, 0);
    block_in = arrayCreate(nb, metamodelica::nil());
    block_out = arrayCreate(nb, metamodelica::nil());
    for mut b in 1..=nb {
        {
            let __cell3 = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut v in (StrongComponent::getVariables(
                    ({
                        let __elt = (*metamodelica::index_checked(&blocks.borrow(), b)?).clone();
                        __elt
                    }),
                )?)
                .into_iter()
                .cloned()
                {
                    let __x = UnorderedMap::getSafe(
                        BVariable::getVarName(v.clone()),
                        vars.map.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            let __idx3 = b;
            *metamodelica::index_mut_checked(&mut block_out.clone().borrow_mut(), __idx3)? = __cell3;
        }
        let __range4 = &*({
            let __elt = (*metamodelica::index_checked(&block_out.borrow(), b)?).clone();
            __elt
        });
        for mut o in __range4 {
            {
                let __cell5 = b;
                let __idx5 = o.clone();
                *metamodelica::index_mut_checked(&mut owner.clone().borrow_mut(), __idx5)? = __cell5;
            }
        }
    }
    for mut b in 1..=nb {
        let __range6 = &*StrongComponent::getEquations(
            ({
                let __elt = (*metamodelica::index_checked(&blocks.borrow(), b)?).clone();
                __elt
            }),
        )?;
        for mut eqn in __range6 {
            let __range7 = &*UnorderedSet::toList(
                ({
                    let __elt = (*metamodelica::index_checked(
                        &occurrences.borrow(),
                        UnorderedMap::getSafe(
                            BEquation::Equation::getEqnName(eqn.clone())?,
                            equations.map.clone(),
                            metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                        )?,
                    )?)
                    .clone();
                    __elt
                }),
            );
            for mut cref in __range7 {
                opt_idx = UnorderedMap::get(
                    ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref)),
                    vars.map.clone(),
                )?;
                if (opt_idx).is_some() {
                    let __pa8 = ::match_deref::match_deref! { match &(opt_idx) {
                        Some(__pa8) => __pa8.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    idx = metamodelica::Own::own(__pa8);
                    if ({
                        let __elt = (*metamodelica::index_checked(&owner.borrow(), idx)?).clone();
                        __elt
                    }) < b
                        && !(listMember(
                            idx,
                            ({
                                let __elt = (*metamodelica::index_checked(&block_in.borrow(), b)?).clone();
                                __elt
                            }),
                        ))
                    {
                        {
                            let __cell9 = metamodelica::cons(
                                idx,
                                ({
                                    let __elt = (*metamodelica::index_checked(&block_in.borrow(), b)?).clone();
                                    __elt
                                }),
                            );
                            let __idx9 = b;
                            *metamodelica::index_mut_checked(&mut block_in.clone().borrow_mut(), __idx9)? = __cell9;
                        }
                    }
                }
            }
        }
    }
    (tears, _, _, _, _, _) = cellierCausalize(
        fixed.clone(),
        true,
        &adj,
        &vars,
        &eqns,
        eqn_glob.clone(),
        eqn_rows.clone(),
        solvabilities.clone(),
        unit_kind.clone(),
        var_slices.clone(),
        block_in.clone(),
        block_out.clone(),
        funcMap.clone(),
        probes.clone(),
    )?;
    for mut t in &*List::sort(
        tears.clone(),
        (std::sync::Arc::new({
            let __pe_b2 = var_slices.clone();
            move |__pe_a0, __pe_a1| cellierSizeLess(__pe_a0, __pe_a1, __pe_b2.clone())
        }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )? {
        if !(listMember(t.clone(), fixed.clone())) {
            trial = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (tears.clone()).into_iter().cloned() {
                    if !(i.clone() != t.clone()) {
                        continue;
                    }
                    let __x = i.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            (_, _, _, _, _, complete) = cellierCausalize(
                trial.clone(),
                false,
                &adj,
                &vars,
                &eqns,
                eqn_glob.clone(),
                eqn_rows.clone(),
                solvabilities.clone(),
                unit_kind.clone(),
                var_slices.clone(),
                block_in.clone(),
                block_out.clone(),
                funcMap.clone(),
                probes.clone(),
            )?;
            if complete {
                tears = trial;
            }
        }
    }
    (tears, eqn_step, eqn_units, row_var, block_step, complete) = cellierCausalize(
        tears,
        false,
        &adj,
        &vars,
        &eqns,
        eqn_glob.clone(),
        eqn_rows.clone(),
        solvabilities.clone(),
        unit_kind.clone(),
        var_slices.clone(),
        block_in.clone(),
        block_out.clone(),
        funcMap,
        probes,
    )?;
    if !(complete)
        || Array::all(
            eqn_step.clone(),
            &({
                let __pe_b1 = 0;
                move |__pe_a0| Ok(intEq(__pe_a0, __pe_b1.clone()))
            }),
        )?
    {
        return Ok(strict);
    }
    for mut i in 1..=ne {
        if ({
            let __elt = (*metamodelica::index_checked(&eqn_step.borrow(), i)?).clone();
            __elt
        }) > 0
        {
            ordered = metamodelica::cons(
                (
                    ({
                        let __elt = (*metamodelica::index_checked(&eqn_step.borrow(), i)?).clone();
                        __elt
                    }),
                    cellierComponent(
                        i,
                        ({
                            let __elt = (*metamodelica::index_checked(&eqn_units.borrow(), i)?).clone();
                            __elt
                        }),
                        ({
                            let __elt = (*metamodelica::index_checked(&eqn_rows.borrow(), i)?).clone();
                            __elt
                        }),
                        row_var.clone(),
                        &mapping,
                        &vars,
                        &eqns,
                        var_slices.clone(),
                        eqn_slices.clone(),
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &solvabilities.borrow(),
                                ({
                                    let __elt = (*metamodelica::index_checked(&eqn_glob.borrow(), i)?).clone();
                                    __elt
                                }),
                            )?)
                            .clone();
                            __elt
                        }),
                    )?,
                ),
                ordered,
            );
        }
    }
    for mut b in 1..=nb {
        ordered = metamodelica::cons(
            (
                ({
                    let __elt = (*metamodelica::index_checked(&block_step.borrow(), b)?).clone();
                    __elt
                }),
                ({
                    let __elt = (*metamodelica::index_checked(&blocks.borrow(), b)?).clone();
                    __elt
                }),
            ),
            ordered,
        );
    }
    ordered = List::sort(
        ordered,
        (std::sync::Arc::new(move |__pe_a0, __pe_a1| Ok(Util::compareTupleIntGt(__pe_a0, __pe_a1)))
            as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>),
    )?;
    tear_set = UnorderedSet::fromList(
        &tears,
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    assign_field!(
        strict.innerEquations = metamodelica::arrayFromVec(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                    metamodelica::nil();
                for mut tpl in (ordered).into_iter().cloned() {
                    let __x = Util::tuple22(tpl.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
            .into_iter()
            .cloned()
            .collect()
        ),
        strict.iteration_vars = ({
            let mut __acc: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
            > = metamodelica::nil();
            for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
                if !(UnorderedSet::contains(
                    UnorderedMap::getSafe(
                        BVariable::getVarName(Slice::getT(var.clone())),
                        vars.map.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                    )?,
                    tear_set.clone(),
                )?) {
                    continue;
                }
                let __x = var.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        strict.residual_eqns = ({
            let mut __acc: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
            > = metamodelica::nil();
            for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                if !(({
                    let __elt = (*metamodelica::index_checked(
                        &eqn_step.borrow(),
                        UnorderedMap::getSafe(
                            BEquation::Equation::getEqnName(Slice::getT(eqn.clone()))?,
                            eqns.map.clone(),
                            metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                        )?,
                    )?)
                    .clone();
                    __elt
                }) == 0)
                {
                    continue;
                }
                let __x = eqn.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(strict)
}

fn cellierComponent(
    mut e: i32,
    mut units: metamodelica::List<i32>,
    mut rows: metamodelica::List<i32>,
    mut row_var: metamodelica::Array<i32>,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut var_slices: metamodelica::Array<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut eqn_slices: metamodelica::Array<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
    mut solvabilities: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>> {
    let __ab_eqn_slices = eqn_slices.borrow();
    let __ab_var_slices = var_slices.borrow();
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut u: i32;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if !(List::hasOneElement(&units)) {
        comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT {
            vars: ({
                let mut __acc: metamodelica::List<
                    metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
                > = metamodelica::nil();
                for mut i in (units).into_iter().cloned() {
                    let __x = (*metamodelica::index_checked(&__ab_var_slices, i.clone())?).clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            eqn: (*metamodelica::index_checked(&__ab_eqn_slices, e)?).clone(),
            status: Solve::Status::UNPROCESSED.clone(),
        });
    } else if List::hasOneElement(&rows) {
        comp = StrongComponent::createPseudoScalar(&rows, row_var.clone(), mapping, vars, eqns)?;
    } else {
        u = (units).head().cloned()?;
        name = BVariable::getVarName(BVariable::VariablePointers::getVarAt(vars, u)?);
        comp = StrongComponent::createPseudoSlice(
            u,
            e,
            (cellierOccurrences(&name, solvabilities)?).head().cloned()?,
            rows,
            row_var.clone(),
            eqns,
            mapping,
            BEquation::Equation::isArrayEquation(BEquation::EquationPointers::getEqnAt(eqns, e)?),
        )?;
    }
    Ok(comp)
}

fn cellierSizeLess(
    mut u1: i32,
    mut u2: i32,
    mut var_slices: metamodelica::Array<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
) -> Result<bool> {
    let __ab_var_slices = var_slices.borrow();
    let mut b: bool = Slice::size(
        (*metamodelica::index_checked(&__ab_var_slices, u1)?).clone(),
        &({
            let __pe_b1 = false;
            move |__pe_a0| BVariable::size(__pe_a0, __pe_b1.clone())
        }),
    )? < Slice::size(
        (*metamodelica::index_checked(&__ab_var_slices, u2)?).clone(),
        &({
            let __pe_b1 = false;
            move |__pe_a0| BVariable::size(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(b)
}

fn cellierCausalize(
    mut fixed_tears: metamodelica::List<i32>,
    mut greedy: bool,
    mut adj: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut eqn_glob: metamodelica::Array<i32>,
    mut eqn_rows: metamodelica::Array<metamodelica::List<i32>>,
    mut solvabilities: metamodelica::Array<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Solvability::Solvability>,
            >,
        >,
    >,
    mut unit_kind: metamodelica::Array<i32>,
    mut var_slices: metamodelica::Array<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut block_in: metamodelica::Array<metamodelica::List<i32>>,
    mut block_out: metamodelica::Array<metamodelica::List<i32>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut probes: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, bool>>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    bool,
)> {
    let __ab_block_out = block_out.borrow();
    let __ab_eqn_rows = eqn_rows.borrow();
    let __ab_var_slices = var_slices.borrow();
    let mut tears: metamodelica::List<i32> = fixed_tears.clone();
    let mut eqn_step: metamodelica::Array<i32>;
    let mut eqn_units: metamodelica::Array<metamodelica::List<i32>>;
    let mut row_var: metamodelica::Array<i32>;
    let mut block_step: metamodelica::Array<i32>;
    let mut complete: bool = true;
    let mut m: metamodelica::Ref<Adjacency::IntMatrix::IntMatrix> =
        <metamodelica::Ref<Adjacency::IntMatrix::IntMatrix> as ::std::default::Default>::default();
    let mut mT: metamodelica::Ref<Adjacency::IntMatrix::IntMatrix> =
        <metamodelica::Ref<Adjacency::IntMatrix::IntMatrix> as ::std::default::Default>::default();
    let mut mapping: metamodelica::Ref<Adjacency::Mapping::Mapping> =
        <metamodelica::Ref<Adjacency::Mapping::Mapping> as ::std::default::Default>::default();
    let mut m_data: metamodelica::Array<i32>;
    let mut mT_data: metamodelica::Array<i32>;
    let mut unknown_rem: metamodelica::Array<i32>;
    let mut eqn_unknown: metamodelica::Array<i32>;
    let mut eqn_size: metamodelica::Array<i32>;
    let mut stamp: metamodelica::Array<i32>;
    let mut known_scal: metamodelica::Array<bool>;
    let mut row_in_loop: metamodelica::Array<bool>;
    let mut ready: metamodelica::Array<metamodelica::List<(i32, metamodelica::List<i32>)>> =
        arrayCreate(4, metamodelica::nil());
    let mut partial_eqns: metamodelica::Array<metamodelica::List<i32>>;
    let mut partial_cov: metamodelica::Array<i32>;
    let mut partial_rank: metamodelica::Array<i32>;
    let mut cover_owner: metamodelica::Array<i32>;
    let mut is_partial: bool;
    let mut overlap: bool;
    let mut nv: i32;
    let mut ne: i32;
    let mut nb: i32 = metamodelica::arrayLength(block_in.clone());
    let mut start: i32;
    let mut size: i32;
    let mut open_units: i32 = 0;
    let mut step: i32 = 0;
    let mut next_block: i32 = 1;
    let mut stamp_id: i32 = 0;
    let mut rank: i32;
    let mut best: i32 = 0;
    let mut u: i32;
    let mut fresh: metamodelica::List<i32> = metamodelica::nil();
    let mut units: metamodelica::List<i32> = metamodelica::nil();
    let mut found: bool;
    let mut slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;
    let () = (match &**adj {
        Adjacency::Matrix::FINAL {
            m: __adj_m,
            mT: __adj_mT,
            mapping: __adj_mapping,
            ..
        } => {
            m = __adj_m.clone();
            mT = __adj_mT.clone();
            mapping = __adj_mapping.clone();
            ()
        }
        _ => return Err("fail"),
    });
    m_data = Adjacency::IntMatrix::entries(&m);
    mT_data = Adjacency::IntMatrix::entries(&mT);
    nv = metamodelica::arrayLength(mapping.var_AtS.clone());
    ne = metamodelica::arrayLength(mapping.eqn_AtS.clone());
    eqn_step = arrayCreate(ne, 0);
    eqn_units = arrayCreate(ne, metamodelica::nil());
    row_var = arrayCreate(metamodelica::arrayLength(mapping.eqn_StA.clone()), -1);
    block_step = arrayCreate(nb, 0);
    stamp = arrayCreate(metamodelica::arrayLength(mapping.var_StA.clone()), 0);
    partial_eqns = arrayCreate(nv, metamodelica::nil());
    partial_cov = arrayCreate(nv, 0);
    partial_rank = arrayCreate(nv, 0);
    cover_owner = arrayCreate(metamodelica::arrayLength(mapping.var_StA.clone()), 0);
    known_scal = arrayCreate(metamodelica::arrayLength(mapping.var_StA.clone()), false);
    unknown_rem = arrayCreate(nv, 0);
    for mut i in 1..=nv {
        (start, size) = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), i)?).clone();
            __elt
        });
        slice = (*metamodelica::index_checked(&__ab_var_slices, i)?).clone();
        if ({
            let __elt = (*metamodelica::index_checked(&unit_kind.borrow(), i)?).clone();
            __elt
        }) == 1
            && !((slice.indices).is_empty())
        {
            for mut k in 0..=size - 1 {
                {
                    let __cell0 = true;
                    let __idx0 = start + k;
                    *metamodelica::index_mut_checked(&mut known_scal.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
            for mut k in &*slice.indices.clone() {
                {
                    let __cell1 = false;
                    let __idx1 = start + k.clone();
                    *metamodelica::index_mut_checked(&mut known_scal.clone().borrow_mut(), __idx1)? = __cell1;
                }
            }
            {
                let __cell2 = ((slice.indices).len() as i32);
                let __idx2 = i;
                *metamodelica::index_mut_checked(&mut unknown_rem.clone().borrow_mut(), __idx2)? = __cell2;
            }
        } else {
            {
                let __cell3 = size;
                let __idx3 = i;
                *metamodelica::index_mut_checked(&mut unknown_rem.clone().borrow_mut(), __idx3)? = __cell3;
            }
        }
        if ({
            let __elt = (*metamodelica::index_checked(&unit_kind.borrow(), i)?).clone();
            __elt
        }) != 3
            && ({
                let __elt = (*metamodelica::index_checked(&unknown_rem.borrow(), i)?).clone();
                __elt
            }) > 0
        {
            open_units = open_units + 1;
        }
    }
    row_in_loop = arrayCreate(metamodelica::arrayLength(mapping.eqn_StA.clone()), false);
    eqn_unknown = arrayCreate(ne, 0);
    eqn_size = arrayCreate(ne, 0);
    for mut e in 1..=ne {
        {
            let __cell4 = ((*metamodelica::index_checked(&__ab_eqn_rows, e)?).len() as i32);
            let __idx4 = e;
            *metamodelica::index_mut_checked(&mut eqn_size.clone().borrow_mut(), __idx4)? = __cell4;
        }
        for mut r in &*(*metamodelica::index_checked(&__ab_eqn_rows, e)?).clone() {
            {
                let __cell5 = true;
                let __idx5 = r.clone();
                *metamodelica::index_mut_checked(&mut row_in_loop.clone().borrow_mut(), __idx5)? = __cell5;
            }
            let __range6 = ({
                let __elt = (*metamodelica::index_checked(&m.start.borrow(), r.clone())?).clone();
                __elt
            })..=({
                let __elt = (*metamodelica::index_checked(&m.start.borrow(), r.clone())?).clone();
                __elt
            }) + ({
                let __elt = (*metamodelica::index_checked(&m.len.borrow(), r.clone())?).clone();
                __elt
            }) - 1;
            for mut p in __range6 {
                if !({
                    let __elt = (*metamodelica::index_checked(
                        &known_scal.borrow(),
                        ({
                            let __elt = (*metamodelica::index_checked(&m_data.borrow(), p)?).clone();
                            __elt
                        }),
                    )?)
                    .clone();
                    __elt
                }) {
                    {
                        let __cell7 = ({
                            let __elt = (*metamodelica::index_checked(&eqn_unknown.borrow(), e)?).clone();
                            __elt
                        }) + 1;
                        let __idx7 = e;
                        *metamodelica::index_mut_checked(&mut eqn_unknown.clone().borrow_mut(), __idx7)? = __cell7;
                    }
                }
            }
        }
    }
    for mut e in ({
        let __s = ne;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if ({
            let __elt = (*metamodelica::index_checked(&eqn_unknown.borrow(), e)?).clone();
            __elt
        }) == ({
            let __elt = (*metamodelica::index_checked(&eqn_size.borrow(), e)?).clone();
            __elt
        }) {
            fresh = metamodelica::cons(e, fresh);
        }
    }
    for mut t in &*fixed_tears {
        fresh = cellierMarkKnown(
            t.clone(),
            &mapping,
            &mT,
            mT_data.clone(),
            row_in_loop.clone(),
            known_scal.clone(),
            unknown_rem.clone(),
            eqn_unknown.clone(),
            eqn_size.clone(),
            eqn_step.clone(),
            fresh,
        )?;
        open_units = open_units - 1;
    }
    loop {
        while next_block <= nb
            && List::all(
                &({
                    let __elt = (*metamodelica::index_checked(&block_in.borrow(), next_block)?).clone();
                    __elt
                }),
                &({
                    let __pe_b1 = unknown_rem.clone();
                    move |__pe_a0| cellierIsKnown(__pe_a0, __pe_b1.clone())
                }),
            )?
        {
            step = step + 1;
            {
                let __cell8 = step;
                let __idx8 = next_block;
                *metamodelica::index_mut_checked(&mut block_step.clone().borrow_mut(), __idx8)? = __cell8;
            }
            for mut o in &*(*metamodelica::index_checked(&__ab_block_out, next_block)?).clone() {
                fresh = cellierMarkKnown(
                    o.clone(),
                    &mapping,
                    &mT,
                    mT_data.clone(),
                    row_in_loop.clone(),
                    known_scal.clone(),
                    unknown_rem.clone(),
                    eqn_unknown.clone(),
                    eqn_size.clone(),
                    eqn_step.clone(),
                    fresh,
                )?;
            }
            next_block = next_block + 1;
        }
        for mut e in &*fresh {
            stamp_id = stamp_id + 1;
            (rank, units, is_partial) = cellierCanAssign(
                e.clone(),
                &(*metamodelica::index_checked(&__ab_eqn_rows, e.clone())?),
                ({
                    let __elt = (*metamodelica::index_checked(&eqn_size.borrow(), e.clone())?).clone();
                    __elt
                }),
                &mapping,
                &m,
                m_data.clone(),
                known_scal.clone(),
                unknown_rem.clone(),
                eqn_unknown.clone(),
                unit_kind.clone(),
                stamp.clone(),
                stamp_id,
                row_var.clone(),
                vars,
                eqns,
                solvabilities.clone(),
                eqn_glob.clone(),
                funcMap.clone(),
                probes.clone(),
            )?;
            if rank > 0 && !(is_partial) {
                {
                    let __cell9 = metamodelica::cons(
                        (e.clone(), units.clone()),
                        ({
                            let __elt = (*metamodelica::index_checked(&ready.borrow(), rank)?).clone();
                            __elt
                        }),
                    );
                    let __idx9 = rank;
                    *metamodelica::index_mut_checked(&mut ready.clone().borrow_mut(), __idx9)? = __cell9;
                }
            } else if rank > 0 {
                u = (units).head().cloned()?;
                overlap = List::any(
                    &(*metamodelica::index_checked(&__ab_eqn_rows, e.clone())?),
                    &({
                        let __pe_b1 = row_var.clone();
                        let __pe_b2 = cover_owner.clone();
                        move |__pe_a0| cellierIsCovered(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
                if !(overlap) {
                    for mut r in &*(*metamodelica::index_checked(&__ab_eqn_rows, e.clone())?).clone() {
                        {
                            let __cell10 = e.clone();
                            let __idx10 = ({
                                let __elt = (*metamodelica::index_checked(&row_var.borrow(), r.clone())?).clone();
                                __elt
                            });
                            *metamodelica::index_mut_checked(&mut cover_owner.clone().borrow_mut(), __idx10)? =
                                __cell10;
                        }
                    }
                    {
                        let __cell11 = metamodelica::cons(
                            e.clone(),
                            ({
                                let __elt = (*metamodelica::index_checked(&partial_eqns.borrow(), u)?).clone();
                                __elt
                            }),
                        );
                        let __idx11 = u;
                        *metamodelica::index_mut_checked(&mut partial_eqns.clone().borrow_mut(), __idx11)? = __cell11;
                    }
                    {
                        let __cell12 = ({
                            let __elt = (*metamodelica::index_checked(&partial_cov.borrow(), u)?).clone();
                            __elt
                        }) + ({
                            let __elt = (*metamodelica::index_checked(&eqn_size.borrow(), e.clone())?).clone();
                            __elt
                        });
                        let __idx12 = u;
                        *metamodelica::index_mut_checked(&mut partial_cov.clone().borrow_mut(), __idx12)? = __cell12;
                    }
                    {
                        let __cell13 = std::cmp::max(
                            ({
                                let __elt = (*metamodelica::index_checked(&partial_rank.borrow(), u)?).clone();
                                __elt
                            }),
                            rank,
                        );
                        let __idx13 = u;
                        *metamodelica::index_mut_checked(&mut partial_rank.clone().borrow_mut(), __idx13)? = __cell13;
                    }
                    if ({
                        let __elt = (*metamodelica::index_checked(&partial_cov.borrow(), u)?).clone();
                        __elt
                    }) == ({
                        let __elt = (*metamodelica::index_checked(&unknown_rem.borrow(), u)?).clone();
                        __elt
                    }) {
                        {
                            let __cell14 = metamodelica::cons(
                                (-(u), units.clone()),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &ready.borrow(),
                                        ({
                                            let __elt =
                                                (*metamodelica::index_checked(&partial_rank.borrow(), u)?).clone();
                                            __elt
                                        }),
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            );
                            let __idx14 = ({
                                let __elt = (*metamodelica::index_checked(&partial_rank.borrow(), u)?).clone();
                                __elt
                            });
                            *metamodelica::index_mut_checked(&mut ready.clone().borrow_mut(), __idx14)? = __cell14;
                        }
                    }
                }
            }
        }
        fresh = metamodelica::nil();
        found = false;
        for mut r in 1..=4 {
            while !(found)
                && !(({
                    let __elt = (*metamodelica::index_checked(&ready.borrow(), r)?).clone();
                    __elt
                })
                .is_empty())
            {
                (best, units) = ({
                    let __elt = (*metamodelica::index_checked(&ready.borrow(), r)?).clone();
                    __elt
                })
                .head()
                .cloned()?;
                {
                    let __cell15 = ({
                        let __elt = (*metamodelica::index_checked(&ready.borrow(), r)?).clone();
                        __elt
                    })
                    .rest()?;
                    let __idx15 = r;
                    *metamodelica::index_mut_checked(&mut ready.clone().borrow_mut(), __idx15)? = __cell15;
                }
                if best < 0 {
                    found = ({
                        let __elt = (*metamodelica::index_checked(&unknown_rem.borrow(), -(best))?).clone();
                        __elt
                    }) > 0;
                } else if ({
                    let __elt = (*metamodelica::index_checked(&eqn_step.borrow(), best)?).clone();
                    __elt
                }) == 0
                    && ({
                        let __elt = (*metamodelica::index_checked(&eqn_unknown.borrow(), best)?).clone();
                        __elt
                    }) == ({
                        let __elt = (*metamodelica::index_checked(&eqn_size.borrow(), best)?).clone();
                        __elt
                    })
                {
                    found = true;
                }
            }
            if found {
                break;
            }
        }
        if found {
            step = step + 1;
            let __range16 = &*if (best < 0) {
                ({
                    let __elt = (*metamodelica::index_checked(&partial_eqns.borrow(), -(best))?).clone();
                    __elt
                })
            } else {
                list![best]
            };
            for mut e in __range16 {
                {
                    let __cell17 = step;
                    let __idx17 = e.clone();
                    *metamodelica::index_mut_checked(&mut eqn_step.clone().borrow_mut(), __idx17)? = __cell17;
                }
                {
                    let __cell18 = units.clone();
                    let __idx18 = e.clone();
                    *metamodelica::index_mut_checked(&mut eqn_units.clone().borrow_mut(), __idx18)? = __cell18;
                }
            }
            for mut un in &*units {
                fresh = cellierMarkKnown(
                    un.clone(),
                    &mapping,
                    &mT,
                    mT_data.clone(),
                    row_in_loop.clone(),
                    known_scal.clone(),
                    unknown_rem.clone(),
                    eqn_unknown.clone(),
                    eqn_size.clone(),
                    eqn_step.clone(),
                    fresh,
                )?;
                open_units = open_units - 1;
            }
        } else if open_units == 0 {
            break;
        } else if greedy {
            u = cellierSelectTear(
                &mapping,
                &mT,
                mT_data.clone(),
                row_in_loop.clone(),
                known_scal.clone(),
                unknown_rem.clone(),
                unit_kind.clone(),
                eqn_step.clone(),
                vars,
            )?;
            tears = metamodelica::cons(u, tears);
            fresh = cellierMarkKnown(
                u,
                &mapping,
                &mT,
                mT_data.clone(),
                row_in_loop.clone(),
                known_scal.clone(),
                unknown_rem.clone(),
                eqn_unknown.clone(),
                eqn_size.clone(),
                eqn_step.clone(),
                fresh,
            )?;
            open_units = open_units - 1;
        } else {
            complete = false;
            break;
        }
    }
    Ok((tears, eqn_step, eqn_units, row_var, block_step, complete))
}

fn cellierIsCovered(
    mut r: i32,
    mut row_var: metamodelica::Array<i32>,
    mut cover_owner: metamodelica::Array<i32>,
) -> Result<bool> {
    let __ab_cover_owner = cover_owner.borrow();
    let __ab_row_var = row_var.borrow();
    let mut b: bool = (*metamodelica::index_checked(
        &__ab_cover_owner,
        (*metamodelica::index_checked(&__ab_row_var, r)?).clone(),
    )?)
    .clone()
        != 0;
    Ok(b)
}

fn cellierIsKnown(mut u: i32, mut unknown_rem: metamodelica::Array<i32>) -> Result<bool> {
    let __ab_unknown_rem = unknown_rem.borrow();
    let mut b: bool = (*metamodelica::index_checked(&__ab_unknown_rem, u)?).clone() == 0;
    Ok(b)
}

fn cellierMarkKnown(
    mut u: i32,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
    mut mT: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut mT_data: metamodelica::Array<i32>,
    mut row_in_loop: metamodelica::Array<bool>,
    mut known_scal: metamodelica::Array<bool>,
    mut unknown_rem: metamodelica::Array<i32>,
    mut eqn_unknown: metamodelica::Array<i32>,
    mut eqn_size: metamodelica::Array<i32>,
    mut eqn_step: metamodelica::Array<i32>,
    mut fresh: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let __ab_eqn_size = eqn_size.borrow();
    let __ab_eqn_step = eqn_step.borrow();
    let __ab_mT_data = mT_data.borrow();
    let __ab_row_in_loop = row_in_loop.borrow();
    let mut fresh: metamodelica::List<i32> = fresh;
    let mut start: i32;
    let mut size: i32;
    let mut e: i32;
    let mut r: i32;
    (start, size) = ({
        let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), u)?).clone();
        __elt
    });
    for mut s in start..=start + size - 1 {
        if !({
            let __elt = (*metamodelica::index_checked(&known_scal.borrow(), s)?).clone();
            __elt
        }) {
            metamodelica::arrayUpdate(known_scal.clone(), s, true)?;
            let __range0 = ({
                let __elt = (*metamodelica::index_checked(&mT.start.borrow(), s)?).clone();
                __elt
            })..=({
                let __elt = (*metamodelica::index_checked(&mT.start.borrow(), s)?).clone();
                __elt
            }) + ({
                let __elt = (*metamodelica::index_checked(&mT.len.borrow(), s)?).clone();
                __elt
            }) - 1;
            for mut p in __range0 {
                r = (*metamodelica::index_checked(&__ab_mT_data, p)?).clone();
                if (*metamodelica::index_checked(&__ab_row_in_loop, r)?).clone() {
                    e = ({
                        let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), r)?).clone();
                        __elt
                    });
                    metamodelica::arrayUpdate(
                        eqn_unknown.clone(),
                        e,
                        ({
                            let __elt = (*metamodelica::index_checked(&eqn_unknown.borrow(), e)?).clone();
                            __elt
                        }) - 1,
                    )?;
                    if ({
                        let __elt = (*metamodelica::index_checked(&eqn_unknown.borrow(), e)?).clone();
                        __elt
                    }) == (*metamodelica::index_checked(&__ab_eqn_size, e)?).clone()
                        && (*metamodelica::index_checked(&__ab_eqn_step, e)?).clone() == 0
                    {
                        fresh = metamodelica::cons(e, fresh);
                    }
                }
            }
        }
    }
    metamodelica::arrayUpdate(unknown_rem.clone(), u, 0)?;
    Ok(fresh)
}

fn cellierCanAssign(
    mut e: i32,
    mut rows: &metamodelica::List<i32>,
    mut size: i32,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut m_data: metamodelica::Array<i32>,
    mut known_scal: metamodelica::Array<bool>,
    mut unknown_rem: metamodelica::Array<i32>,
    mut eqn_unknown: metamodelica::Array<i32>,
    mut unit_kind: metamodelica::Array<i32>,
    mut stamp: metamodelica::Array<i32>,
    mut stamp_id: i32,
    mut row_var: metamodelica::Array<i32>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut solvabilities: metamodelica::Array<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Solvability::Solvability>,
            >,
        >,
    >,
    mut eqn_glob: metamodelica::Array<i32>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut probes: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, bool>>,
) -> Result<(i32, metamodelica::List<i32>, bool)> {
    let __ab_eqn_glob = eqn_glob.borrow();
    let __ab_eqn_unknown = eqn_unknown.borrow();
    let __ab_known_scal = known_scal.borrow();
    let __ab_m_data = m_data.borrow();
    let __ab_solvabilities = solvabilities.borrow();
    let __ab_unknown_rem = unknown_rem.borrow();
    let mut rank: i32 = 0;
    let mut units: metamodelica::List<i32> = metamodelica::nil();
    let mut is_partial: bool = false;
    let mut cnt: i32;
    let mut sv: i32 = 0;
    let mut u: i32;
    let mut covered: i32 = 0;
    let mut key: i32;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if (*metamodelica::index_checked(&__ab_eqn_unknown, e)?).clone() != size {
        return Ok((rank, units, is_partial));
    }
    for mut r in &**rows {
        cnt = 0;
        let __range0 = ({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), r.clone())?).clone();
            __elt
        })..=({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), r.clone())?).clone();
            __elt
        }) + ({
            let __elt = (*metamodelica::index_checked(&m.len.borrow(), r.clone())?).clone();
            __elt
        }) - 1;
        for mut p in __range0 {
            if !((*metamodelica::index_checked(
                &__ab_known_scal,
                (*metamodelica::index_checked(&__ab_m_data, p)?).clone(),
            )?)
            .clone())
            {
                cnt = cnt + 1;
                sv = (*metamodelica::index_checked(&__ab_m_data, p)?).clone();
            }
        }
        if cnt != 1 {
            units = metamodelica::nil();
            return Ok((rank, units, is_partial));
        } else if ({
            let __elt = (*metamodelica::index_checked(&stamp.borrow(), sv)?).clone();
            __elt
        }) == stamp_id
        {
            units = metamodelica::nil();
            return Ok((rank, units, is_partial));
        }
        metamodelica::arrayUpdate(stamp.clone(), sv, stamp_id)?;
        metamodelica::arrayUpdate(row_var.clone(), r.clone(), sv)?;
        u = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), sv)?).clone();
            __elt
        });
        if !(listMember(u, units.clone())) {
            if ({
                let __elt = (*metamodelica::index_checked(&unit_kind.borrow(), u)?).clone();
                __elt
            }) != 1
            {
                units = metamodelica::nil();
                return Ok((rank, units, is_partial));
            }
            units = metamodelica::cons(u, units);
            covered = covered + (*metamodelica::index_checked(&__ab_unknown_rem, u)?).clone();
        }
    }
    is_partial = covered > size && List::hasOneElement(&units);
    if covered != size && !(is_partial) {
        units = metamodelica::nil();
        return Ok((rank, units, is_partial));
    }
    if List::hasOneElement(&units) {
        name = BVariable::getVarName(BVariable::VariablePointers::getVarAt(vars, (units).head().cloned()?)?);
        rank = cellierRank(
            &name,
            (*metamodelica::index_checked(
                &__ab_solvabilities,
                (*metamodelica::index_checked(&__ab_eqn_glob, e)?).clone(),
            )?)
            .clone(),
        )?;
        if rank < 1 || rank > 4 {
            rank = 0;
        } else {
            key = e * (metamodelica::arrayLength(unit_kind.clone()) + 1) + (units).head().cloned()?;
            if !(UnorderedMap::contains(key, probes.clone())?) {
                UnorderedMap::add(
                    key,
                    cellierSolvable(
                        BEquation::EquationPointers::getEqnAt(eqns, e)?,
                        &name,
                        (*metamodelica::index_checked(
                            &__ab_solvabilities,
                            (*metamodelica::index_checked(&__ab_eqn_glob, e)?).clone(),
                        )?)
                        .clone(),
                        funcMap,
                    )?,
                    probes.clone(),
                )?;
            }
            if !(UnorderedMap::getSafe(
                key,
                probes,
                metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
            )?) {
                rank = 0;
            }
        }
    } else if isRecordAssignment(
        BEquation::EquationPointers::getEqnAt(eqns, e)?,
        &({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut i in (units.clone()).into_iter().cloned() {
                let __x = BVariable::VariablePointers::getVarAt(vars, i.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )? {
        rank = 1;
    }
    if rank == 0 {
        units = metamodelica::nil();
    }
    Ok((rank, units, is_partial))
}

fn cellierOccurrences(
    mut name: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut solvabilities: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut c in (UnorderedMap::keyList(solvabilities.clone())).into_iter().cloned() {
            if !(ComponentRef::isEqual(&(ComponentRef::stripSubscriptsAll(&(c.clone()))), name)?) {
                continue;
            }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(crefs)
}

fn cellierSolvable(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut name: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut solvabilities: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<bool> {
    let mut b: bool = false;
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        cellierOccurrences(name, solvabilities.clone())?;
    let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    let mut status: Solve::Status;
    let mut single: bool;
    (eqn, single) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            (body.clone(), true)
        },
        Deref @ BEquation::Equation::FOR_EQUATION { .. } => {
            (eqn, false)
        },
        _ => {
            (eqn, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if single && List::hasOneElement(&crefs) {
        ErrorExt::setCheckpoint(literal!("NBTearing.cellierSolvable"));
        match '__try0: {
            (_, status, _) = unwrap_break_err!(Solve::solveBody(eqn.clone(), unwrap_break_err!((crefs).head().cloned(), '__try0), funcMap.clone()), '__try0);
            b = status == Solve::Status::EXPLICIT.clone();
            Ok::<_, &'static str>((b.clone(),))
        } {
            Ok((__try0_o0,)) => {
                b = __try0_o0;
            }
            Err(_) => {
                b = false;
            }
        }
        ErrorExt::rollBack(literal!("NBTearing.cellierSolvable"));
    }
    Ok(b)
}

fn cellierRank(
    mut name: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut solvabilities: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
) -> Result<i32> {
    let mut rank: i32 = 0;
    let mut r: i32;
    for mut tpl in &*UnorderedMap::toList(solvabilities) {
        if ComponentRef::isEqual(&(ComponentRef::stripSubscriptsAll(&(Util::tuple21(tpl.clone())))), name)? {
            r = Adjacency::Solvability::rank(&(Util::tuple22(tpl.clone())))?;
            if r == 0 {
                rank = 0;
                return Ok(rank);
            }
            rank = std::cmp::max(rank, r);
        }
    }
    Ok(rank)
}

fn isRecordAssignment(
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut vars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<bool> {
    let mut b: bool = true;
    let mut lhs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut rhs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let () = (::match_deref::match_deref! { match &(Pointer::access(eqn)) {
        e @ Deref @ BEquation::Equation::RECORD_EQUATION { .. } => {
            lhs = UnorderedSet::fromList(&(({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut c in (UnorderedSet::toList(Expression::extractCrefs(var_field!((**e).lhs, Equation::Equation::RECORD_EQUATION).clone())?)).into_iter().cloned() {
            let __x = ComponentRef::stripSubscriptsAll(&(c.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            rhs = UnorderedSet::fromList(&(({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut c in (UnorderedSet::toList(Expression::extractCrefs(var_field!((**e).rhs, Equation::Equation::RECORD_EQUATION).clone())?)).into_iter().cloned() {
            let __x = ComponentRef::stripSubscriptsAll(&(c.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            for mut var in &**vars {
                names = recordNames(var.clone());
                if !(List::any(&names, &({ let __pe_b1 = lhs.clone(); move |__pe_a0| UnorderedSet::contains(__pe_a0, __pe_b1.clone()) }))?) || List::any(&names, &({ let __pe_b1 = rhs.clone(); move |__pe_a0| UnorderedSet::contains(__pe_a0, __pe_b1.clone()) }))? {
                    b = false;
                }
            }
            ()
        },
        _ => {
            b = false;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn recordNames(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        list![BVariable::getVarName(var.clone())];
    names = (match BVariable::getParent(var) {
        Some(mut parent) => listAppend(names, recordNames(parent)),
        _ => names,
    });
    names
}

fn cellierSelectTear(
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
    mut mT: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut mT_data: metamodelica::Array<i32>,
    mut row_in_loop: metamodelica::Array<bool>,
    mut known_scal: metamodelica::Array<bool>,
    mut unknown_rem: metamodelica::Array<i32>,
    mut unit_kind: metamodelica::Array<i32>,
    mut eqn_step: metamodelica::Array<i32>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<i32> {
    let __ab_known_scal = known_scal.borrow();
    let __ab_mT_data = mT_data.borrow();
    let __ab_row_in_loop = row_in_loop.borrow();
    let __ab_unit_kind = unit_kind.borrow();
    let mut best: i32 = 0;
    let mut start: i32;
    let mut size: i32;
    let mut cls: i32;
    let mut occ: i32;
    let mut e: i32;
    let mut best_cls: i32 = -1;
    let mut best_occ: i32 = -1;
    let mut best_size: i32 = 0;
    let mut has_start: bool;
    let mut best_start: bool = false;
    let mut seen: metamodelica::Array<i32> = arrayCreate(metamodelica::arrayLength(eqn_step.clone()), 0);
    for mut u in 1..=metamodelica::arrayLength(unknown_rem.clone()) {
        if (*metamodelica::index_checked(&__ab_unit_kind, u)?).clone() != 3
            && ({
                let __elt = (*metamodelica::index_checked(&unknown_rem.borrow(), u)?).clone();
                __elt
            }) > 0
        {
            cls = ((BVariable::getTearingSelect(BVariable::VariablePointers::getVarAt(vars, u)?)?) as i32);
            has_start = (BVariable::getStartAttribute(BVariable::VariablePointers::getVarAt(vars, u)?)?).is_some();
            occ = 0;
            (start, size) = ({
                let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), u)?).clone();
                __elt
            });
            for mut s in start..=start + size - 1 {
                if !((*metamodelica::index_checked(&__ab_known_scal, s)?).clone()) {
                    let __range0 = ({
                        let __elt = (*metamodelica::index_checked(&mT.start.borrow(), s)?).clone();
                        __elt
                    })..=({
                        let __elt = (*metamodelica::index_checked(&mT.start.borrow(), s)?).clone();
                        __elt
                    }) + ({
                        let __elt = (*metamodelica::index_checked(&mT.len.borrow(), s)?).clone();
                        __elt
                    }) - 1;
                    for mut p in __range0 {
                        if (*metamodelica::index_checked(
                            &__ab_row_in_loop,
                            (*metamodelica::index_checked(&__ab_mT_data, p)?).clone(),
                        )?)
                        .clone()
                        {
                            e = ({
                                let __elt = (*metamodelica::index_checked(
                                    &mapping.eqn_StA.borrow(),
                                    (*metamodelica::index_checked(&__ab_mT_data, p)?).clone(),
                                )?)
                                .clone();
                                __elt
                            });
                            if ({
                                let __elt = (*metamodelica::index_checked(&eqn_step.borrow(), e)?).clone();
                                __elt
                            }) == 0
                                && ({
                                    let __elt = (*metamodelica::index_checked(&seen.borrow(), e)?).clone();
                                    __elt
                                }) != u
                            {
                                {
                                    let __cell1 = u;
                                    let __idx1 = e;
                                    *metamodelica::index_mut_checked(&mut seen.clone().borrow_mut(), __idx1)? = __cell1;
                                }
                                occ = occ + 1;
                            }
                        }
                    }
                }
            }
            if cls > best_cls
                || cls == best_cls
                    && (has_start && !(best_start)
                        || has_start == best_start
                            && (occ > best_occ
                                || occ == best_occ
                                    && ({
                                        let __elt = (*metamodelica::index_checked(&unknown_rem.borrow(), u)?).clone();
                                        __elt
                                    }) < best_size))
            {
                best = u;
                best_cls = cls;
                best_start = has_start;
                best_occ = occ;
                best_size = ({
                    let __elt = (*metamodelica::index_checked(&unknown_rem.borrow(), u)?).clone();
                    __elt
                });
            }
        }
    }
    Ok(best)
}

fn checkLinearity(
    mut full: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut v: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut e: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<bool> {
    fn varIsLinear(
        mut var: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut v: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut sol: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Solvability::Solvability>,
            >,
        >,
    ) -> Result<bool> {
        let mut b: bool = !(UnorderedMap::contains(var.clone(), v.clone())?
            && Adjacency::Solvability::isNonlinearOrImplicit(
                &(UnorderedMap::getSafe(
                    var.clone(),
                    sol.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"),
                )?),
            ));
        Ok(b)
    }

    fn eqnIsLinear(
        mut i: i32,
        mut occ: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >,
        mut sol: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Solvability::Solvability>,
                >,
            >,
        >,
        mut v: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    ) -> Result<bool> {
        let __ab_occ = occ.borrow();
        let __ab_sol = sol.borrow();
        let mut b: bool = UnorderedSet::all(
            (*metamodelica::index_checked(&__ab_occ, i)?).clone(),
            &({
                let __pe_b1 = v.clone();
                let __pe_b2 = (*metamodelica::index_checked(&__ab_sol, i)?).clone();
                move |__pe_a0| varIsLinear(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            }),
        )?;
        Ok(b)
    }

    let mut linear: bool;
    linear = (match &**full {
        Adjacency::Matrix::FULL { .. } => UnorderedMap::all(
            e,
            (std::sync::Arc::new({
                let __pe_b1 = var_field!((**full).occurrences, Adjacency::Matrix::Matrix::FULL).clone();
                let __pe_b2 = var_field!((**full).solvabilities, Adjacency::Matrix::Matrix::FULL).clone();
                let __pe_b3 = v;
                move |__pe_a0| eqnIsLinear(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
            }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>),
        )?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBTearing.checkLinearity"));
                    __mm_s.push_str(&*literal!(" expected type full, got type "));
                    __mm_s.push_str(&*Adjacency::strictnessString(Adjacency::Matrix::getStrictness(full)?));
                    __mm_s.push_str(&*literal!("."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(linear)
}

fn filterDiscreteVariables(
    mut vars_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut staticAsContinuous: bool,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
)> {
    fn addDiscreteRecord(
        mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut discrete_records: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    ) -> Result<()> {
        let () = (match BVariable::getParent(var) {
            Some(mut parent) => {
                UnorderedSet::add(BVariable::getVarName(parent.clone()), discrete_records.clone())?;
                addDiscreteRecord(parent, discrete_records)?;
                ()
            }
            _ => (),
        });
        Ok(())
    }

    fn checkDiscreteRecord(
        mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut discrete_records: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
        mut is_parent: bool,
    ) -> Result<bool> {
        '__tco: loop {
            match BVariable::getParent(var.clone()) {
                Some(mut parent) => {
                    (var, discrete_records, is_parent) = (parent, discrete_records, true);
                    continue '__tco;
                }
                _ => return Ok(is_parent && UnorderedSet::contains(BVariable::getVarName(var), discrete_records)?),
            }
        }
    }

    let mut cont_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut disc_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut discrete_records: metamodelica::Ref<
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
    let mut rec_disc_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    (cont_vars, disc_vars) = List::splitOnTrue(
        vars_lst,
        &({
            let __pe_b1 = staticAsContinuous;
            move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone())
        }),
    )?;
    for mut var in &*disc_vars {
        addDiscreteRecord(var.clone(), discrete_records.clone())?;
    }
    (rec_disc_vars, cont_vars) = List::splitOnTrue(
        &cont_vars,
        &({
            let __pe_b1 = discrete_records;
            let __pe_b2 = false;
            move |__pe_a0| checkDiscreteRecord(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }),
    )?;
    disc_vars = listAppend(rec_disc_vars, disc_vars).reverse();
    Ok((cont_vars, disc_vars))
}

fn getImpliedInnerVars(
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    vars = (::match_deref::match_deref! { match &(Pointer::access(eqn)) {
        Deref @ BEquation::Equation::ALGORITHM { alg, .. } => {
            vars = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut out_cr in (alg.outputs.clone()).into_iter().cloned() {
            let __x = BVariable::getVarPointer(&(out_cr.clone()), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            vars
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } => {
            ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut tpl_cr in (UnorderedSet::toList(Expression::extractCrefs(tpl.clone())?)).into_iter().cloned() {
            if !(!(ComponentRef::isWild(&(tpl_cr.clone())) || ComponentRef::isEmpty(&(tpl_cr.clone())))) { continue; }
            let __x = BVariable::getVarPointer(&(tpl_cr.clone()), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { .. }, .. } => {
            metamodelica::nil()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}
