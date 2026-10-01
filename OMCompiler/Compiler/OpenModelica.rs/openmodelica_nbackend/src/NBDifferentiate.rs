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

use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointer;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::BaseModelica;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs as BuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFClassTree::ClassTree;
use openmodelica_nf_frontend::NFComponent as Component;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFFunction::Slot;
use openmodelica_nf_frontend::NFFunctionDerivative as FunctionDerivative;
use openmodelica_nf_frontend::NFInstContext as InstContext;
use openmodelica_nf_frontend::NFInstNode;
use openmodelica_nf_frontend::NFInstNode::CachedData;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFRestriction as Restriction;
use openmodelica_nf_frontend::NFSections as Sections;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Util imports
// ================================
//        TYPES AND UNIONTYPES
// ================================
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum DifferentiationType {
    TIME = 1,
    SIMPLE = 2,
    FUNCTION = 3,
    JACOBIAN = 4,
}
impl PartialOrd for DifferentiationType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DifferentiationType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for DifferentiationType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for DifferentiationType {
    fn default() -> Self {
        Self::TIME
    }
}

pub mod DifferentiationArguments {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct DifferentiationArguments {
        /// The input will be differentiated w.r.t. this cref (only SIMPLE).
        pub diffCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        /// contains all new variables that need to be added to the system
        pub new_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        /// seed and temporary cref map x --> $SEED.MATRIX.x, y --> $pDer.MATRIX.y. Can be used for any differentiation rules
        pub diff_map: Option<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                >,
            >,
        >,
        /// Differentiation use case (time, simple, function, jacobian)
        pub diffType: DifferentiationType,
        /// Function tree containing all functions and their known derivatives
        pub funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
        /// true if the variables are scalarized
        pub scalarized: bool,
        /// map for accumulating adjoint gradients for component refs
        pub adjoint_map: Option<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
                >,
            >,
        >,
        /// current gradient expression, used in reverse mode
        pub current_grad: metamodelica::Ref<Expression::NFExpression>,
        /// If false, skip writing into adjoint_map (used for LHS traversal in reverse/Jacobian).
        pub collectAdjoints: bool,
    }

    impl metamodelica::gc::MMTrace for DifferentiationArguments {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.diffCref, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.new_vars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.diff_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.diffType, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.funcMap, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.scalarized, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.adjoint_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.current_grad, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.collectAdjoints, __mmv)?;
            Ok(())
        }
    }
    impl Default for DifferentiationArguments {
        fn default() -> Self {
            Self {
                diffCref: Default::default(),
                new_vars: Default::default(),
                diff_map: Default::default(),
                diffType: Default::default(),
                funcMap: Default::default(),
                scalarized: Default::default(),
                adjoint_map: Default::default(),
                current_grad: Default::default(),
                collectAdjoints: Default::default(),
            }
        }
    }

    pub type DIFFERENTIATION_ARGUMENTS = DifferentiationArguments;

    pub(crate) fn default(
        mut ty: DifferentiationType,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
    ) -> metamodelica::Ref<DifferentiationArguments> {
        let mut diffArgs: metamodelica::Ref<DifferentiationArguments> =
            metamodelica::Ref::new(DifferentiationArguments {
                diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
                new_vars: metamodelica::nil(),
                diff_map: None,
                diffType: ty,
                funcMap: funcMap.clone(),
                scalarized: false,
                adjoint_map: None,
                current_grad: metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                    ty: openmodelica_nf_frontend::NFType::interned_REAL(),
                }),
                collectAdjoints: false,
            });
        diffArgs
    }

    pub(crate) fn simpleCref(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
    ) -> metamodelica::Ref<DifferentiationArguments> {
        let mut diffArgs: metamodelica::Ref<DifferentiationArguments> =
            metamodelica::Ref::new(DifferentiationArguments {
                diffCref: cref.clone(),
                new_vars: metamodelica::nil(),
                diff_map: None,
                diffType: DifferentiationType::SIMPLE.clone(),
                funcMap: funcMap.clone(),
                scalarized: false,
                adjoint_map: None,
                current_grad: metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                    ty: openmodelica_nf_frontend::NFType::interned_REAL(),
                }),
                collectAdjoints: false,
            });
        diffArgs
    }

    pub(crate) fn toString(mut diffArgs: &metamodelica::Ref<DifferentiationArguments>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("["));
            __mm_s.push_str(&*diffTypeStr(diffArgs.diffType.clone()));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        if diffArgs.diffType.clone() == DifferentiationType::SIMPLE.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*ComponentRef::toString(&diffArgs.diffCref)?);
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn diffTypeStr(mut diffType: DifferentiationType) -> ArcStr {
        let mut r#str: ArcStr;
        r#str = (match diffType {
            DifferentiationType::TIME => literal!("TIME"),
            DifferentiationType::SIMPLE => literal!("SIMPLE"),
            DifferentiationType::FUNCTION { .. } => literal!("FUNCTION"),
            DifferentiationType::JACOBIAN => literal!("JACOBIAN"),
            _ => literal!("FAIL"),
        });
        r#str
    }
}

// ================================
//             FUNCTIONS
// ================================
pub(crate) fn differentiateStrongComponentList(
    mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut idx: Pointer::Pointer<i32>,
    mut context: &ArcStr,
    mut name: &ArcStr,
) -> Result<(
    metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> = comps;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>> =
        Pointer::create(diffArguments.clone());
    comps = List::map(
        comps,
        &({
            let __pe_b1 = diffArguments_ptr.clone();
            let __pe_b2 = idx;
            let __pe_b3 = context.clone();
            let __pe_b4 = name.clone();
            move |__pe_a0| differentiateStrongComponent(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3, &__pe_b4)
        }),
    )?;
    diffArguments = Pointer::access(diffArguments_ptr);
    Ok((comps, diffArguments))
}

pub(crate) fn differentiateStrongComponent<'__b>(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>>,
    mut idx: Pointer::Pointer<i32>,
    mut context: &'__b ArcStr,
    mut name: &'__b ArcStr,
) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>> {
    '__tco: loop {
        match &*comp {
            StrongComponent::SINGLE_COMPONENT {
                eqn: __comp_eqn,
                status: __comp_status,
                var: __comp_var,
            } => {
                let mut new_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                new_var = differentiateVariablePointer(__comp_var.clone(), diffArguments_ptr.clone())?;
                new_eqn = differentiateEquationPointer(__comp_eqn.clone(), diffArguments_ptr, name)?;
                Equation::createName(new_eqn.clone(), idx, context)?;
                return Ok(metamodelica::Ref::new(
                    StrongComponent::NBStrongComponent::SINGLE_COMPONENT {
                        var: new_var,
                        eqn: new_eqn,
                        status: __comp_status.clone(),
                    },
                ));
            }
            StrongComponent::MULTI_COMPONENT {
                eqn: __comp_eqn,
                status: __comp_status,
                vars: __comp_vars,
            } => {
                let mut new_var_slices: metamodelica::List<
                    metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
                >;
                let mut new_eqn_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                >;
                new_var_slices = ({
                    let mut __acc: metamodelica::List<
                        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
                    > = metamodelica::nil();
                    for mut var in (__comp_vars.clone()).into_iter().cloned() {
                        let __x = Slice::apply(
                            var.clone(),
                            &({
                                let __pe_b1 = diffArguments_ptr.clone();
                                move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone())
                            }),
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                new_eqn_slice = Slice::apply(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr;
                        let __pe_b2 = name.clone();
                        move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                Equation::createName(Slice::getT(new_eqn_slice.clone()), idx, context)?;
                return Ok(metamodelica::Ref::new(
                    StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                        vars: new_var_slices,
                        eqn: new_eqn_slice,
                        status: __comp_status.clone(),
                    },
                ));
            }
            StrongComponent::SLICED_COMPONENT {
                eqn: __comp_eqn,
                status: __comp_status,
                var: __comp_var,
                var_cref: __comp_var_cref,
            } => {
                let mut new_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut new_var_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                >;
                let mut new_eqn_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                >;
                let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateComponentRefNoCollect(Expression::fromCref(__comp_var_cref.clone(), false)?, Pointer::access(diffArguments_ptr.clone()))?) {
                    (Deref @ Expression::CREF { cref: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                new_cref = metamodelica::Own::own(__pa0);
                diffArguments = metamodelica::Own::own(__pa1);
                Pointer::update(diffArguments_ptr.clone(), diffArguments);
                new_var_slice = Slice::apply(
                    __comp_var.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr.clone();
                        move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                new_eqn_slice = Slice::apply(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr;
                        let __pe_b2 = name.clone();
                        move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                Slice::applyMutable(
                    new_eqn_slice.clone(),
                    &({
                        let __pe_b1 = idx;
                        let __pe_b2 = context.clone();
                        move |__pe_a0| Equation::createName(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                return Ok(metamodelica::Ref::new(
                    StrongComponent::NBStrongComponent::SLICED_COMPONENT {
                        var_cref: new_cref,
                        var: new_var_slice,
                        eqn: new_eqn_slice,
                        status: __comp_status.clone(),
                    },
                ));
            }
            StrongComponent::RESIZABLE_COMPONENT {
                eqn: __comp_eqn,
                order: __comp_order,
                status: __comp_status,
                var: __comp_var,
                var_cref: __comp_var_cref,
            } => {
                let mut new_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut new_var_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                >;
                let mut new_eqn_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                >;
                let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateComponentRef(Expression::fromCref(__comp_var_cref.clone(), false)?, Pointer::access(diffArguments_ptr.clone()))?) {
                    (Deref @ Expression::CREF { cref: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                new_cref = metamodelica::Own::own(__pa0);
                diffArguments = metamodelica::Own::own(__pa1);
                Pointer::update(diffArguments_ptr.clone(), diffArguments);
                new_var_slice = Slice::apply(
                    __comp_var.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr.clone();
                        move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                new_eqn_slice = Slice::apply(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr;
                        let __pe_b2 = name.clone();
                        move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                Slice::applyMutable(
                    new_eqn_slice.clone(),
                    &({
                        let __pe_b1 = idx;
                        let __pe_b2 = context.clone();
                        move |__pe_a0| Equation::createName(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                return Ok(metamodelica::Ref::new(
                    StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT {
                        var_cref: new_cref,
                        var: new_var_slice,
                        eqn: new_eqn_slice,
                        order: __comp_order.clone(),
                        status: __comp_status.clone(),
                    },
                ));
            }
            StrongComponent::GENERIC_COMPONENT {
                eqn: __comp_eqn,
                var: __comp_var,
                var_cref: __comp_var_cref,
            } => {
                let mut new_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut new_var_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                >;
                let mut new_eqn_slice: metamodelica::Ref<
                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                >;
                let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateComponentRef(Expression::fromCref(__comp_var_cref.clone(), false)?, Pointer::access(diffArguments_ptr.clone()))?) {
                    (Deref @ Expression::CREF { cref: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                new_cref = metamodelica::Own::own(__pa0);
                diffArguments = metamodelica::Own::own(__pa1);
                Pointer::update(diffArguments_ptr.clone(), diffArguments);
                new_var_slice = Slice::apply(
                    __comp_var.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr.clone();
                        move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                new_eqn_slice = Slice::apply(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr;
                        let __pe_b2 = name.clone();
                        move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                Slice::applyMutable(
                    new_eqn_slice.clone(),
                    &({
                        let __pe_b1 = idx;
                        let __pe_b2 = context.clone();
                        move |__pe_a0| Equation::createName(__pe_a0, __pe_b1.clone(), &__pe_b2)
                    }),
                )?;
                return Ok(metamodelica::Ref::new(
                    StrongComponent::NBStrongComponent::GENERIC_COMPONENT {
                        var_cref: new_cref,
                        var: new_var_slice,
                        eqn: new_eqn_slice,
                    },
                ));
            }
            StrongComponent::ALGEBRAIC_LOOP {
                casual: __comp_casual,
                homotopy: __comp_homotopy,
                implicitlyCreated: __comp_implicitlyCreated,
                linear: __comp_linear,
                status: __comp_status,
                strict: __comp_strict,
                ..
            } => {
                let mut strict: metamodelica::Ref<Tearing::NBTearing>;
                let mut casual: Option<metamodelica::Ref<Tearing::NBTearing>>;
                let mut linear: bool;
                strict = differentiateTearing(
                    metamodelica::AsArg::as_arg(&__comp_strict),
                    diffArguments_ptr.clone(),
                    idx.clone(),
                    context,
                    name,
                )?;
                casual = Util::applyOption(
                    __comp_casual.clone(),
                    &({
                        let __pe_b1 = diffArguments_ptr.clone();
                        let __pe_b2 = idx;
                        let __pe_b3 = context.clone();
                        let __pe_b4 = name.clone();
                        move |__pe_a0| {
                            differentiateTearing(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3, &__pe_b4)
                        }
                    }),
                )?;
                linear = (match &*(Pointer::access(diffArguments_ptr)) {
                    DifferentiationArguments::DIFFERENTIATION_ARGUMENTS {
                        diffType: DifferentiationType::JACOBIAN,
                        ..
                    } => true,
                    _ => __comp_linear.clone(),
                });
                return Ok(metamodelica::Ref::new(
                    StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP {
                        idx: -1,
                        strict: strict,
                        casual: casual,
                        linear: linear,
                        mixed: false,
                        homotopy: __comp_homotopy.clone(),
                        status: __comp_status.clone(),
                        implicitlyCreated: __comp_implicitlyCreated.clone(),
                    },
                ));
            }
            StrongComponent::ENTWINED_COMPONENT { .. } => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBDifferentiate.differentiateStrongComponent"));
                        __mm_s.push_str(&*literal!(" not implemented for entwined equation:\n"));
                        __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
            StrongComponent::ALIAS {
                original: __comp_original,
                ..
            } => {
                (comp, diffArguments_ptr, idx, context, name) =
                    (__comp_original.clone(), diffArguments_ptr, idx, context, name);
                continue '__tco;
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBDifferentiate.differentiateStrongComponent"));
                        __mm_s.push_str(&*literal!(" not implemented for unknown strong component:\n"));
                        __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn differentiateTearing(
    mut tearing: &metamodelica::Ref<Tearing::NBTearing>,
    mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>>,
    mut idx: Pointer::Pointer<i32>,
    mut context: &ArcStr,
    mut name: &ArcStr,
) -> Result<metamodelica::Ref<Tearing::NBTearing>> {
    let mut diff_tearing: metamodelica::Ref<Tearing::NBTearing>;
    let mut ite_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut res_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut inner_eqns: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    ite_vars = ({
        let mut __acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        > = metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::apply(
                var.clone(),
                &({
                    let __pe_b1 = diffArguments_ptr.clone();
                    move |__pe_a0| differentiateVariablePointer(__pe_a0, __pe_b1.clone())
                }),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res_eqns = ({
        let mut __acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        > = metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::apply(
                eqn.clone(),
                &({
                    let __pe_b1 = diffArguments_ptr.clone();
                    let __pe_b2 = name.clone();
                    move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
                }),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    inner_eqns = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            for mut ie in (tearing
                .innerEquations
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>())
            .into_iter()
            .cloned()
            {
                if !(!(StrongComponent::isDiscrete(&(ie.clone()))?)) {
                    continue;
                }
                let __x =
                    differentiateStrongComponent(ie.clone(), diffArguments_ptr.clone(), idx.clone(), context, name)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    diff_tearing = metamodelica::Ref::new(Tearing::NBTearing {
        iteration_vars: ite_vars,
        residual_eqns: res_eqns,
        innerEquations: inner_eqns.clone(),
        jac: None,
    });
    Ok(diff_tearing)
}

pub(crate) fn differentiateEquationPointerList(
    mut equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut idx: Pointer::Pointer<i32>,
    mut context: &ArcStr,
    mut name: &ArcStr,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = equations;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>> =
        Pointer::create(diffArguments.clone());
    equations = List::map(
        equations,
        &({
            let __pe_b1 = diffArguments_ptr.clone();
            let __pe_b2 = name.clone();
            move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
        }),
    )?;
    for mut eqn in &*equations {
        Equation::createName(eqn.clone(), idx.clone(), context)?;
    }
    diffArguments = Pointer::access(diffArguments_ptr);
    Ok((equations, diffArguments))
}

pub(crate) fn differentiateEquationPointer(
    mut eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>>,
    mut name: &ArcStr,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut derivative_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut eq: metamodelica::Ref<Equation::Equation>;
    let mut diffedEq: metamodelica::Ref<Equation::Equation>;
    let mut old_diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut new_diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    eq = Pointer::access(eq_ptr.clone());
    old_diffArguments = Pointer::access(diffArguments_ptr.clone());
    derivative_ptr = (match &*(Equation::getAttributes(eq.clone())) {
        EquationAttributes::EQUATION_ATTRIBUTES {
            derivative: Some(__esc_derivative_ptr),
            ..
        } if (old_diffArguments.diffType.clone() == DifferentiationType::TIME.clone()) => {
            derivative_ptr = (*__esc_derivative_ptr).clone();
            derivative_ptr.clone()
        }
        _ => {
            (diffedEq, new_diffArguments) = differentiateEquation(eq.clone(), old_diffArguments.clone(), name)?;
            derivative_ptr = Pointer::create(diffedEq);
            if new_diffArguments.diffType.clone() == DifferentiationType::TIME.clone() {
                Pointer::update(eq_ptr, Equation::setDerivative(eq, derivative_ptr.clone())?);
            }
            if !(referenceEq(&*(&*new_diffArguments), &*(&*old_diffArguments))) {
                Pointer::update(diffArguments_ptr, new_diffArguments);
            }
            derivative_ptr
        }
    });
    Ok(derivative_ptr)
}

pub(crate) fn differentiateEquation(
    mut eq: metamodelica::Ref<Equation::Equation>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut name: &ArcStr,
) -> Result<(
    metamodelica::Ref<Equation::Equation>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut eq: metamodelica::Ref<Equation::Equation> = eq;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? && !(stringEqual(&name, &(literal!("")))) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("### debugDifferentiation | "));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" ###\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[BEFORE] "));
            __mm_s.push_str(&*Equation::toString(eq.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (eq, diffArguments) = ({
        let mut forBody: metamodelica::List<metamodelica::Ref<Equation::Equation>> = metamodelica::nil();
        (match &*eq {
            Equation::SCALAR_EQUATION {
                attr: __eq_attr,
                lhs: __eq_lhs,
                rhs: __eq_rhs,
                source: __eq_source,
                ty: __eq_ty,
            } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                (lhs, diffArguments) = differentiateExpressionNoCollect(__eq_lhs.clone(), diffArguments)?;
                (rhs, diffArguments) = differentiateExpression(__eq_rhs.clone(), diffArguments)?;
                attr = differentiateEquationAttributes(__eq_attr.clone(), &diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::SCALAR_EQUATION {
                        ty: __eq_ty.clone(),
                        lhs: lhs,
                        rhs: rhs,
                        source: __eq_source.clone(),
                        attr: attr,
                    }),
                    diffArguments,
                )
            }
            Equation::ARRAY_EQUATION {
                attr: __eq_attr,
                lhs: __eq_lhs,
                recordSize: __eq_recordSize,
                rhs: __eq_rhs,
                source: __eq_source,
                ty: __eq_ty,
            } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                (lhs, diffArguments) = differentiateExpressionNoCollect(__eq_lhs.clone(), diffArguments)?;
                (rhs, diffArguments) = differentiateExpression(__eq_rhs.clone(), diffArguments)?;
                attr = differentiateEquationAttributes(__eq_attr.clone(), &diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::ARRAY_EQUATION {
                        ty: __eq_ty.clone(),
                        lhs: lhs,
                        rhs: rhs,
                        source: __eq_source.clone(),
                        attr: attr,
                        recordSize: __eq_recordSize.clone(),
                    }),
                    diffArguments,
                )
            }
            Equation::RECORD_EQUATION {
                attr: __eq_attr,
                lhs: __eq_lhs,
                recordSize: __eq_recordSize,
                rhs: __eq_rhs,
                source: __eq_source,
                ty: __eq_ty,
            } => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                (lhs, diffArguments) = differentiateExpressionNoCollect(__eq_lhs.clone(), diffArguments)?;
                (rhs, diffArguments) = differentiateExpression(__eq_rhs.clone(), diffArguments)?;
                attr = differentiateEquationAttributes(__eq_attr.clone(), &diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::RECORD_EQUATION {
                        ty: __eq_ty.clone(),
                        lhs: lhs,
                        rhs: rhs,
                        source: __eq_source.clone(),
                        attr: attr,
                        recordSize: __eq_recordSize.clone(),
                    }),
                    diffArguments,
                )
            }
            Equation::IF_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                size: __eq_size,
                source: __eq_source,
            } => {
                let mut ifBody: metamodelica::Ref<IfEquationBody::IfEquationBody>;
                let mut diffArguments_ptr: Pointer::Pointer<
                    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
                >;
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                (ifBody, diffArguments_ptr) =
                    differentiateIfEquationBody(__eq_body.clone(), Pointer::create(diffArguments.clone()))?;
                attr = differentiateEquationAttributes(__eq_attr.clone(), &diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::IF_EQUATION {
                        size: __eq_size.clone(),
                        body: ifBody,
                        source: __eq_source.clone(),
                        attr: attr,
                    }),
                    Pointer::access(diffArguments_ptr),
                )
            }
            Equation::FOR_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                iter: __eq_iter,
                size: __eq_size,
                source: __eq_source,
            } => {
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                for mut body_eqn in &*__eq_body.clone() {
                    let mut body_eqn = body_eqn.clone();
                    (body_eqn, diffArguments) = differentiateEquation(body_eqn, diffArguments, &(literal!("")))?;
                    forBody = metamodelica::cons(body_eqn, forBody);
                }
                attr = differentiateEquationAttributes(__eq_attr.clone(), &diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::FOR_EQUATION {
                        size: __eq_size.clone(),
                        iter: __eq_iter.clone(),
                        body: forBody.reverse(),
                        source: __eq_source.clone(),
                        attr: attr,
                    }),
                    diffArguments,
                )
            }
            Equation::WHEN_EQUATION {
                attr: __eq_attr,
                body: __eq_body,
                size: __eq_size,
                source: __eq_source,
            } => {
                let mut whenBody: metamodelica::Ref<WhenEquationBody::WhenEquationBody>;
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                (whenBody, diffArguments) = differentiateWhenEquationBody(__eq_body.clone(), diffArguments)?;
                attr = differentiateEquationAttributes(__eq_attr.clone(), &diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::WHEN_EQUATION {
                        size: __eq_size.clone(),
                        body: whenBody,
                        source: __eq_source.clone(),
                        attr: attr,
                    }),
                    diffArguments,
                )
            }
            Equation::ALGORITHM {
                alg: __eq_alg,
                attr: __eq_attr,
                expand: __eq_expand,
                size: __eq_size,
                source: __eq_source,
            } => {
                let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
                (alg, diffArguments) = differentiateAlgorithm(__eq_alg.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Equation::Equation::ALGORITHM {
                        size: __eq_size.clone(),
                        alg: alg,
                        source: __eq_source.clone(),
                        expand: __eq_expand.clone(),
                        attr: __eq_attr.clone(),
                    }),
                    diffArguments,
                )
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBDifferentiate.differentiateEquation"));
                        __mm_s.push_str(&*literal!(" failed for: "));
                        __mm_s.push_str(&*Equation::toString(eq, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? && !(stringEqual(&name, &(literal!("")))) {
        eq = Equation::simplify(
            eq,
            name,
            &(literal!("\t")),
            Pointer::create(metamodelica::nil()),
            Pointer::create(metamodelica::nil()),
            (std::sync::Arc::new({
                let __pe_b1 = true;
                let __pe_b2 = name.clone();
                let __pe_b3 = literal!("\t");
                move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[AFTER ] "));
            __mm_s.push_str(&*Equation::toString(eq.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    } else {
        eq = Equation::simplify(
            eq,
            name,
            &(literal!("")),
            Pointer::create(metamodelica::nil()),
            Pointer::create(metamodelica::nil()),
            (std::sync::Arc::new({
                let __pe_b1 = true;
                let __pe_b2 = name.clone();
                let __pe_b3 = literal!("");
                move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    }
    Ok((eq, diffArguments))
}

pub(crate) fn differentiateEquationAdjoint(
    mut eq: &metamodelica::Ref<Equation::Equation>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
)> {
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut adjointStatements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    (diffArguments, adjointStatements) = (match &**eq {
        Equation::SCALAR_EQUATION {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            let mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut seedCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut dm: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                >,
            >;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let __pa0 = ::match_deref::match_deref! { match &(diffArguments.diff_map.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dm = metamodelica::Own::own(__pa0);
            lhsCref = Expression::toCref(metamodelica::AsArg::as_arg(&__eq_lhs))?;
            if !(ComponentRef::isEmpty(&lhsCref))
                && UnorderedMap::contains(ComponentRef::stripSubscriptsAll(&lhsCref), dm.clone())?
            {
                seedCref = UnorderedMap::getOrFail(ComponentRef::stripSubscriptsAll(&lhsCref), dm)?;
                if !(diffArguments.scalarized.clone()) {
                    seedCref = ComponentRef::copySubscripts(&lhsCref, ComponentRef::stripSubscriptsAll(&seedCref))?;
                }
                assign_field!(
                    diffArguments.current_grad = Expression::fromCref(seedCref, false)?,
                    diffArguments.collectAdjoints = true
                );
                (_, diffArguments) = differentiateExpression(__eq_rhs.clone(), diffArguments)?;
                (diffArguments, stmts) = makeAdjointAccumulationStatements(diffArguments)?;
            } else {
                stmts = metamodelica::nil();
            }
            (diffArguments, stmts)
        }
        Equation::ARRAY_EQUATION {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            let mut dm: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                >,
            >;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut lhs_base: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut seed_base: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut seed_subscripted: metamodelica::Ref<Expression::NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(diffArguments.diff_map.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dm = metamodelica::Own::own(__pa0);
            lhs_base = Expression::toCref(metamodelica::AsArg::as_arg(&__eq_lhs))?;
            if !(ComponentRef::isEmpty(&lhs_base))
                && UnorderedMap::contains(ComponentRef::stripSubscriptsAll(&lhs_base), dm.clone())?
            {
                seed_base = UnorderedMap::getOrFail(ComponentRef::stripSubscriptsAll(&lhs_base), dm)?;
                seed_subscripted = Expression::applySubscripts(
                    &(ComponentRef::subscriptsAllFlat(&lhs_base)?),
                    Expression::fromCref(ComponentRef::stripSubscriptsAll(&seed_base), false)?,
                    true,
                )?;
                assign_field!(
                    diffArguments.current_grad = seed_subscripted,
                    diffArguments.collectAdjoints = true
                );
                (_, diffArguments) = differentiateExpression(__eq_rhs.clone(), diffArguments)?;
                (diffArguments, stmts) = makeAdjointAccumulationStatements(diffArguments)?;
            } else {
                stmts = metamodelica::nil();
            }
            (diffArguments, stmts)
        }
        Equation::RECORD_EQUATION {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } => {
            let mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut seedCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut dm: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                >,
            >;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let __pa0 = ::match_deref::match_deref! { match &(diffArguments.diff_map.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dm = metamodelica::Own::own(__pa0);
            lhsCref = Expression::toCref(metamodelica::AsArg::as_arg(&__eq_lhs))?;
            if !(ComponentRef::isEmpty(&lhsCref))
                && UnorderedMap::contains(ComponentRef::stripSubscriptsAll(&lhsCref), dm.clone())?
            {
                seedCref = UnorderedMap::getOrFail(ComponentRef::stripSubscriptsAll(&lhsCref), dm)?;
                if !(diffArguments.scalarized.clone()) {
                    seedCref = ComponentRef::copySubscripts(&lhsCref, ComponentRef::stripSubscriptsAll(&seedCref))?;
                }
                assign_field!(
                    diffArguments.current_grad = Expression::fromCref(seedCref, false)?,
                    diffArguments.collectAdjoints = true
                );
                (_, diffArguments) = differentiateExpression(__eq_rhs.clone(), diffArguments)?;
                (diffArguments, stmts) = makeAdjointAccumulationStatements(diffArguments)?;
            } else {
                stmts = metamodelica::nil();
            }
            (diffArguments, stmts)
        }
        Equation::IF_EQUATION { body: __eq_body, .. } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut ifBranches: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            )>;
            let mut elseIfBranches: Option<
                metamodelica::List<(
                    metamodelica::Ref<Expression::NFExpression>,
                    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
                )>,
            >;
            (diffArguments, ifBranches, elseIfBranches) =
                differentiateIfEquationBodyAdjoint(metamodelica::AsArg::as_arg(&__eq_body), diffArguments)?;
            stmts = list![metamodelica::Ref::new(Statement::NFStatement::IF {
                branches: ifBranches,
                source: DAE::emptyElementSource().clone()
            })];
            (diffArguments, stmts)
        }
        Equation::FOR_EQUATION {
            body: __eq_body,
            iter: __eq_iter,
            ..
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut bodyStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut iterNames: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut iterRanges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut iterMaps: metamodelica::List<Option<metamodelica::Ref<NBEquation::Iterator::Iterator>>>;
            let mut iterName: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut iterRange: metamodelica::Ref<Expression::NFExpression>;
            let mut iterMap: Option<metamodelica::Ref<NBEquation::Iterator::Iterator>>;
            let mut sub_iters_stmt: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
            )>;
            let mut revIter: metamodelica::Ref<NBEquation::Iterator::Iterator>;
            let mut iter_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut iter_elems: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
            stmts = metamodelica::nil();
            for mut bodyEqn in &*__eq_body.clone() {
                (diffArguments, bodyStmts) =
                    differentiateEquationAdjoint(metamodelica::AsArg::as_arg(&bodyEqn), diffArguments)?;
                stmts = listAppend(bodyStmts, stmts);
            }
            revIter = reverseEquationIterator(metamodelica::AsArg::as_arg(&__eq_iter))?;
            (iterNames, iterRanges, iterMaps) = NBEquation::Iterator::getFrames(&revIter);
            for mut tpl in &*List::zip3(iterNames, iterRanges, iterMaps).reverse() {
                (iterName, iterRange, iterMap) = tpl.clone();
                sub_iters_stmt = (::match_deref::match_deref! { match &(iterMap) {
                    Some(Deref @ NBEquation::Iterator::SINGLE { name: __esc_iter_name, range: Deref @ Expression::ARRAY { elements: __esc_iter_elems, .. }, map: None }) => {
                        iter_name = (*__esc_iter_name).clone();
                        iter_elems = (*__esc_iter_elems).clone();
                        list![(iter_name.clone(), iter_elems.clone())]
                    },
                    _ => metamodelica::nil(),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                stmts = list![metamodelica::Ref::new(Statement::NFStatement::FOR {
                    iterator: ComponentRef::node(&iterName)?,
                    range: Some(iterRange),
                    body: stmts,
                    forType: openmodelica_nf_frontend::NFStatement::ForType::NORMAL,
                    source: DAE::emptyElementSource().clone(),
                    sub_iters: sub_iters_stmt
                })];
            }
            (diffArguments, stmts)
        }
        Equation::ALGORITHM { alg: __eq_alg, .. } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut bodyStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut allStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            allStmts = metamodelica::nil();
            for mut s in &*__eq_alg.statements.clone() {
                (diffArguments, bodyStmts) =
                    differentiateStatementAdjoint(metamodelica::AsArg::as_arg(&s), diffArguments)?;
                for mut bs in &*bodyStmts {
                    allStmts = metamodelica::cons(bs.clone(), allStmts);
                }
            }
            stmts = allStmts;
            (diffArguments, stmts)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBDifferentiate.differentiateEquationAdjoint"));
                    __mm_s.push_str(&*literal!(" failed for: "));
                    __mm_s.push_str(&*Equation::toString(eq.clone(), literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((diffArguments, adjointStatements))
}

pub(crate) fn differentiateStatementAdjoint(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
)> {
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut adjointStatements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    (diffArguments, adjointStatements) = (match &**stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } if (Type::isReal(&(Type::arrayElementType(&(Expression::typeOf(__stmt_lhs.clone())))))?) => {
            let mut lhs: metamodelica::Ref<Expression::NFExpression>;
            let mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            (lhs, diffArguments) = differentiateExpressionNoCollect(__stmt_lhs.clone(), diffArguments)?;
            lhsCref = (match &*lhs {
                Expression::CREF { cref: __lhs_cref, .. } => __lhs_cref.clone(),
                _ => openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
            });
            if !(ComponentRef::isEmpty(&lhsCref)) {
                assign_field!(diffArguments.current_grad = lhs, diffArguments.collectAdjoints = true);
                (_, diffArguments) = differentiateExpression(__stmt_rhs.clone(), diffArguments)?;
                (diffArguments, stmts) = makeAdjointAccumulationStatements(diffArguments)?;
            } else {
                stmts = metamodelica::nil();
            }
            (diffArguments, stmts)
        }
        Statement::FOR {
            body: __stmt_body,
            forType: __stmt_forType,
            iterator: __stmt_iterator,
            range: __stmt_range,
            source: __stmt_source,
            sub_iters: __stmt_sub_iters,
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut bodyStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut allStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            allStmts = metamodelica::nil();
            for mut s in &*__stmt_body.clone() {
                (diffArguments, bodyStmts) =
                    differentiateStatementAdjoint(metamodelica::AsArg::as_arg(&s), diffArguments)?;
                for mut bs in &*bodyStmts {
                    allStmts = metamodelica::cons(bs.clone(), allStmts);
                }
            }
            stmts = list![metamodelica::Ref::new(Statement::NFStatement::FOR {
                iterator: __stmt_iterator.clone(),
                range: reverseForRange(__stmt_range.clone())?,
                body: allStmts,
                forType: __stmt_forType.clone(),
                source: __stmt_source.clone(),
                sub_iters: __stmt_sub_iters.clone()
            })];
            (diffArguments, stmts)
        }
        Statement::IF {
            branches: __stmt_branches,
            source: __stmt_source,
        } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut bodyStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut allStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut adjBranches: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            )>;
            let mut cond: metamodelica::Ref<Expression::NFExpression>;
            adjBranches = metamodelica::nil();
            for mut branch in &*__stmt_branches.clone() {
                (cond, bodyStmts) = branch.clone();
                allStmts = metamodelica::nil();
                for mut s in &*bodyStmts {
                    (diffArguments, stmts) =
                        differentiateStatementAdjoint(metamodelica::AsArg::as_arg(&s), diffArguments)?;
                    for mut bs in &*stmts {
                        allStmts = metamodelica::cons(bs.clone(), allStmts);
                    }
                }
                adjBranches = metamodelica::cons((cond, allStmts), adjBranches);
            }
            stmts = list![metamodelica::Ref::new(Statement::NFStatement::IF {
                branches: adjBranches.reverse(),
                source: __stmt_source.clone()
            })];
            (diffArguments, stmts)
        }
        Statement::ASSIGNMENT { .. } => (diffArguments, list![stmt.clone()]),
        _ => (diffArguments, list![stmt.clone()]),
    });
    Ok((diffArguments, adjointStatements))
}

pub(crate) fn differentiateIfEquationBodyAdjoint(
    mut body: &metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
    Option<
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
    >,
)> {
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>;
    let mut elseIfBranches: Option<
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
    >;
    let mut allStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut bodyStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut bodyEqn: metamodelica::Ref<Equation::Equation>;
    let mut elseBody: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    let mut elseBranches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>;
    let mut nestedElse: Option<
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )>,
    >;
    allStmts = metamodelica::nil();
    for mut eqPtr in &*body.then_eqns.clone() {
        bodyEqn = Pointer::access(eqPtr.clone());
        (diffArguments, bodyStmts) = differentiateEquationAdjoint(&bodyEqn, diffArguments)?;
        for mut s in &*bodyStmts {
            allStmts = metamodelica::cons(s.clone(), allStmts);
        }
    }
    branches = list![(body.condition.clone(), allStmts)];
    if (body.else_if).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(body.else_if.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        elseBody = metamodelica::Own::own(__pa0);
        (diffArguments, elseBranches, nestedElse) = differentiateIfEquationBodyAdjoint(&elseBody, diffArguments)?;
        for mut b in &*elseBranches {
            branches = metamodelica::cons(b.clone(), branches);
        }
        elseIfBranches = nestedElse;
    } else {
        elseIfBranches = None;
    }
    branches = branches.reverse();
    Ok((diffArguments, branches, elseIfBranches))
}

pub(crate) fn makeAdjointAccumulationStatements(
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
)> {
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut amap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >;
    let mut keys: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut taggedTerms: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut accRhs: metamodelica::Ref<Expression::NFExpression>;
    let mut vty: metamodelica::Ref<Type::NFType>;
    let mut sc: Operator::SizeClassification;
    let mut addOp: metamodelica::Ref<Operator::NFOperator>;
    let mut key: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    stmts = metamodelica::nil();
    if (diffArguments.adjoint_map).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(diffArguments.adjoint_map.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        amap = metamodelica::Own::own(__pa0);
        keys = UnorderedMap::keyList(amap.clone());
        for mut key in &*keys {
            let mut key = key.clone();
            taggedTerms = UnorderedMap::getOrFail(key.clone(), amap.clone())?;
            if !((taggedTerms).is_empty()) {
                vty = ComponentRef::getSubscriptedType(&key, true)?;
                sc = sizeClassificationFromType(vty.clone());
                addOp =
                    Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sc), vty.clone())?;
                if List::hasOneElement(&taggedTerms) {
                    accRhs = (taggedTerms).head().cloned()?;
                } else {
                    accRhs = SimplifyExp::simplify(
                        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: taggedTerms,
                            inv_arguments: metamodelica::nil(),
                            operator: addOp.clone(),
                        }),
                        false,
                    )?;
                }
                accRhs = SimplifyExp::simplify(
                    metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![Expression::fromCref(key.clone(), false)?, accRhs],
                        inv_arguments: metamodelica::nil(),
                        operator: addOp,
                    }),
                    false,
                )?;
                accRhs = Expression::map(
                    accRhs,
                    (std::sync::Arc::new(Expression::repairOperator)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                stmts = metamodelica::cons(
                    metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                        lhs: Expression::fromCref(key, false)?,
                        rhs: accRhs,
                        ty: vty,
                        source: DAE::emptyElementSource().clone(),
                    }),
                    stmts,
                );
            }
        }
        UnorderedMap::clear(amap.clone());
        assign_field!(diffArguments.adjoint_map = Some(amap));
    }
    Ok((diffArguments, stmts))
}

pub(crate) fn differentiateIfEquationBody(
    mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>>,
) -> Result<(
    metamodelica::Ref<IfEquationBody::IfEquationBody>,
    Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>>,
)> {
    let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody> = body;
    let mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>> =
        diffArguments_ptr;
    let mut then_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut else_if: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    then_eqns = List::map(
        body.then_eqns.clone(),
        &({
            let __pe_b1 = diffArguments_ptr.clone();
            let __pe_b2 = literal!("");
            move |__pe_a0| differentiateEquationPointer(__pe_a0, __pe_b1.clone(), &__pe_b2)
        }),
    )?;
    if (body.else_if).is_some() {
        (else_if, diffArguments_ptr) =
            differentiateIfEquationBody(body.else_if.clone().ok_or("pattern mismatch")?, diffArguments_ptr)?;
        body = metamodelica::Ref::new(IfEquationBody::IfEquationBody {
            condition: body.condition.clone(),
            then_eqns: then_eqns,
            else_if: Some(else_if),
        });
    } else {
        body = metamodelica::Ref::new(IfEquationBody::IfEquationBody {
            condition: body.condition.clone(),
            then_eqns: then_eqns,
            else_if: None,
        });
    }
    Ok((body, diffArguments_ptr))
}

pub(crate) fn differentiateWhenEquationBody(
    mut body: metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut body: metamodelica::Ref<WhenEquationBody::WhenEquationBody> = body;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut when_stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>;
    let mut else_when: metamodelica::Ref<WhenEquationBody::WhenEquationBody>;
    (when_stmts, diffArguments) = List::mapFold(
        &body.when_stmts,
        &(move |__pe_a0, __pe_a1| differentiateWhenStatement(__pe_a0, __pe_a1)),
        diffArguments,
    )?;
    if (body.else_when).is_some() {
        (else_when, diffArguments) =
            differentiateWhenEquationBody(body.else_when.clone().ok_or("pattern mismatch")?, diffArguments)?;
        body = metamodelica::Ref::new(WhenEquationBody::WhenEquationBody {
            condition: body.condition.clone(),
            when_stmts: when_stmts,
            else_when: Some(else_when),
        });
    } else {
        body = metamodelica::Ref::new(WhenEquationBody::WhenEquationBody {
            condition: body.condition.clone(),
            when_stmts: when_stmts,
            else_when: None,
        });
    }
    Ok((body, diffArguments))
}

pub(crate) fn differentiateWhenStatement(
    mut stmt: metamodelica::Ref<WhenStatement::WhenStatement>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<WhenStatement::WhenStatement>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut stmt: metamodelica::Ref<WhenStatement::WhenStatement> = stmt;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    (stmt, diffArguments) = (match &*stmt {
        WhenStatement::ASSIGN {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            source: __stmt_source,
        } => {
            let mut lhs: metamodelica::Ref<Expression::NFExpression>;
            let mut rhs: metamodelica::Ref<Expression::NFExpression>;
            (lhs, diffArguments) = differentiateExpression(__stmt_lhs.clone(), diffArguments)?;
            (rhs, diffArguments) = differentiateExpression(__stmt_rhs.clone(), diffArguments)?;
            (
                metamodelica::Ref::new(WhenStatement::WhenStatement::ASSIGN {
                    lhs: lhs,
                    rhs: rhs,
                    source: __stmt_source.clone(),
                }),
                diffArguments,
            )
        }
        _ => (stmt, diffArguments),
    });
    Ok((stmt, diffArguments))
}

pub(crate) fn differentiateExpressionDump(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut name: &ArcStr,
    mut indent: &ArcStr,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("### debugDifferentiation | "));
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*literal!(" ###\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("[BEFORE] "));
            __mm_s.push_str(&*Expression::toString(exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        (exp, diffArguments) = differentiateExpression(exp, diffArguments)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*indent);
            __mm_s.push_str(&*literal!("[AFTER ] "));
            __mm_s.push_str(&*Expression::toString(exp.clone())?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    } else {
        (exp, diffArguments) = differentiateExpression(exp, diffArguments)?;
    }
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateExpression(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    (exp, diffArguments) = ({
        let mut new_elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut new_matrix_elements: metamodelica::List<
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        > = metamodelica::nil();
        let mut isReverse: bool = (diffArguments.adjoint_map).is_some();
        (match &*exp.clone() {
            Expression::INTEGER { .. } => (
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                diffArguments,
            ),
            Expression::REAL { .. } => (
                metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: metamodelica::OrderedFloat(0.0_f64),
                }),
                diffArguments,
            ),
            Expression::STRING { .. } => (exp, diffArguments),
            Expression::BOOLEAN { .. } => (exp, diffArguments),
            Expression::CREF { .. } => differentiateComponentRef(exp, diffArguments)?,
            Expression::ARRAY { .. } => {
                let mut arr: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
                (arr, diffArguments) = Array::mapFold(
                    var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(),
                    &differentiateExpression,
                    diffArguments,
                )?;
                assign_variant_field!(exp => Expression::NFExpression::ARRAY; elements = arr.clone());
                (exp, diffArguments)
            }
            Expression::MATRIX {
                elements: __exp_elements,
            } => {
                for mut element_lst in &*__exp_elements.clone() {
                    new_elements = metamodelica::nil();
                    for mut element in &*element_lst.clone() {
                        let mut element = element.clone();
                        (element, diffArguments) = differentiateExpression(element, diffArguments)?;
                        new_elements = metamodelica::cons(element, new_elements);
                    }
                    new_matrix_elements = metamodelica::cons(new_elements.reverse(), new_matrix_elements);
                }
                (
                    metamodelica::Ref::new(Expression::NFExpression::MATRIX {
                        elements: new_matrix_elements.reverse(),
                    }),
                    diffArguments,
                )
            }
            Expression::TUPLE {
                elements: __exp_elements,
                ty: __exp_ty,
            } => {
                for mut element in &*__exp_elements.clone() {
                    let mut element = element.clone();
                    (element, diffArguments) = differentiateExpression(element, diffArguments)?;
                    new_elements = metamodelica::cons(element, new_elements);
                }
                (
                    metamodelica::Ref::new(Expression::NFExpression::TUPLE {
                        ty: __exp_ty.clone(),
                        elements: new_elements.reverse(),
                    }),
                    diffArguments,
                )
            }
            Expression::RECORD {
                elements: __exp_elements,
                path: __exp_path,
                ty: __exp_ty,
            } => {
                for mut element in &*__exp_elements.clone() {
                    let mut element = element.clone();
                    (element, diffArguments) = differentiateExpression(element, diffArguments)?;
                    new_elements = metamodelica::cons(element, new_elements);
                }
                (
                    metamodelica::Ref::new(Expression::NFExpression::RECORD {
                        path: __exp_path.clone(),
                        ty: __exp_ty.clone(),
                        elements: new_elements.reverse(),
                    }),
                    diffArguments,
                )
            }
            Expression::CALL { .. } => differentiateCall(exp, diffArguments)?,
            Expression::IF {
                condition: __exp_condition,
                falseBranch: __exp_falseBranch,
                trueBranch: __exp_trueBranch,
                ty: __exp_ty,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                let mut elem2: metamodelica::Ref<Expression::NFExpression>;
                let mut current_grad: metamodelica::Ref<Expression::NFExpression>;
                let mut gradTrue: metamodelica::Ref<Expression::NFExpression>;
                let mut gradFalse: metamodelica::Ref<Expression::NFExpression>;
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    gradTrue = metamodelica::Ref::new(Expression::NFExpression::IF {
                        ty: Expression::typeOf(current_grad.clone()),
                        condition: __exp_condition.clone(),
                        trueBranch: current_grad.clone(),
                        falseBranch: Expression::makeZero(&(Expression::typeOf(current_grad.clone())))?,
                    });
                    gradFalse = metamodelica::Ref::new(Expression::NFExpression::IF {
                        ty: Expression::typeOf(current_grad.clone()),
                        condition: __exp_condition.clone(),
                        trueBranch: Expression::makeZero(&(Expression::typeOf(current_grad.clone())))?,
                        falseBranch: current_grad.clone(),
                    });
                    assign_field!(diffArguments.current_grad = gradTrue);
                    (elem1, diffArguments) = differentiateExpression(__exp_trueBranch.clone(), diffArguments)?;
                    assign_field!(diffArguments.current_grad = gradFalse);
                    (elem2, diffArguments) = differentiateExpression(__exp_falseBranch.clone(), diffArguments)?;
                    assign_field!(diffArguments.current_grad = current_grad);
                } else {
                    (elem1, diffArguments) = differentiateExpression(__exp_trueBranch.clone(), diffArguments)?;
                    (elem2, diffArguments) = differentiateExpression(__exp_falseBranch.clone(), diffArguments)?;
                }
                (
                    metamodelica::Ref::new(Expression::NFExpression::IF {
                        ty: __exp_ty.clone(),
                        condition: __exp_condition.clone(),
                        trueBranch: elem1,
                        falseBranch: elem2,
                    }),
                    diffArguments,
                )
            }
            Expression::BINARY { .. } => differentiateBinary(exp, diffArguments)?,
            Expression::MULTARY { .. } => differentiateMultary(exp, diffArguments)?,
            Expression::UNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                let mut current_grad: metamodelica::Ref<Expression::NFExpression>;
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    assign_field!(
                        diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::UNARY {
                            operator: __exp_operator.clone(),
                            exp: current_grad.clone()
                        })
                    );
                    (elem1, diffArguments) = differentiateExpression(__exp_exp.clone(), diffArguments)?;
                    assign_field!(diffArguments.current_grad = current_grad);
                } else {
                    (elem1, diffArguments) = differentiateExpression(__exp_exp.clone(), diffArguments)?;
                }
                (
                    metamodelica::Ref::new(Expression::NFExpression::UNARY {
                        operator: __exp_operator.clone(),
                        exp: elem1,
                    }),
                    diffArguments,
                )
            }
            Expression::CAST {
                exp: __exp_exp,
                ty: __exp_ty,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                (elem1, diffArguments) = differentiateExpression(__exp_exp.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::CAST {
                        ty: __exp_ty.clone(),
                        exp: elem1,
                    }),
                    diffArguments,
                )
            }
            Expression::BOX { exp: __exp_exp } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                (elem1, diffArguments) = differentiateExpression(__exp_exp.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::BOX { exp: elem1 }),
                    diffArguments,
                )
            }
            Expression::UNBOX {
                exp: __exp_exp,
                ty: __exp_ty,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                (elem1, diffArguments) = differentiateExpression(__exp_exp.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::UNBOX {
                        exp: elem1,
                        ty: __exp_ty.clone(),
                    }),
                    diffArguments,
                )
            }
            Expression::SUBSCRIPTED_EXP {
                exp: __exp_exp,
                split: __exp_split,
                subscripts: __exp_subscripts,
                ty: __exp_ty,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                (elem1, diffArguments) = differentiateExpression(__exp_exp.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::SUBSCRIPTED_EXP {
                        exp: elem1,
                        subscripts: __exp_subscripts.clone(),
                        ty: __exp_ty.clone(),
                        split: __exp_split.clone(),
                    }),
                    diffArguments,
                )
            }
            Expression::TUPLE_ELEMENT {
                index: __exp_index,
                tupleExp: __exp_tupleExp,
                ty: __exp_ty,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                (elem1, diffArguments) = differentiateExpression(__exp_tupleExp.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::TUPLE_ELEMENT {
                        tupleExp: elem1,
                        index: __exp_index.clone(),
                        ty: __exp_ty.clone(),
                    }),
                    diffArguments,
                )
            }
            Expression::RECORD_ELEMENT {
                fieldName: __exp_fieldName,
                index: __exp_index,
                recordExp: __exp_recordExp,
                ty: __exp_ty,
            } => {
                let mut elem1: metamodelica::Ref<Expression::NFExpression>;
                if diffArguments.diffType.clone() == DifferentiationType::SIMPLE.clone()
                    && !(Expression::containsCref(__exp_recordExp.clone(), &diffArguments.diffCref)?)
                {
                    elem1 = Expression::makeZero(&(Expression::typeOf(exp)))?;
                } else {
                    (elem1, diffArguments) = differentiateExpression(__exp_recordExp.clone(), diffArguments)?;
                    elem1 = metamodelica::Ref::new(Expression::NFExpression::RECORD_ELEMENT {
                        recordExp: elem1,
                        index: __exp_index.clone(),
                        fieldName: __exp_fieldName.clone(),
                        ty: __exp_ty.clone(),
                    });
                }
                (elem1, diffArguments)
            }
            Expression::PARTIAL_FUNCTION_APPLICATION {
                argNames: __exp_argNames,
                args: __exp_args,
                r#fn: __exp_fn,
                ty: __exp_ty,
            } => {
                let mut d_fn: metamodelica::Ref<ComponentRef::NFComponentRef>;
                d_fn = BVariable::makeFDerVar(__exp_fn.clone())?;
                for mut element in &*__exp_args.clone() {
                    let mut element = element.clone();
                    (element, diffArguments) = differentiateExpression(element, diffArguments)?;
                    new_elements = metamodelica::cons(element, new_elements);
                }
                (
                    metamodelica::Ref::new(Expression::NFExpression::PARTIAL_FUNCTION_APPLICATION {
                        r#fn: d_fn,
                        args: listAppend(__exp_args.clone(), new_elements.reverse()),
                        argNames: listAppend(
                            __exp_argNames.clone(),
                            ({
                                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                                for mut name in (__exp_argNames.clone()).into_iter().cloned() {
                                    let __x = BackendUtil::makeFDerString(name.clone(), None)?;
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                        ),
                        ty: __exp_ty.clone(),
                    }),
                    diffArguments,
                )
            }
            Expression::LBINARY { .. } => (exp, diffArguments),
            Expression::LUNARY { .. } => (exp, diffArguments),
            Expression::RELATION { .. } => (exp, diffArguments),
            Expression::SIZE { .. } => (exp, diffArguments),
            Expression::RANGE { .. } => (exp, diffArguments),
            Expression::END => (exp, diffArguments),
            Expression::EMPTY { .. } => (exp, diffArguments),
            Expression::ENUM_LITERAL { .. } => (exp, diffArguments),
            Expression::TYPENAME { .. } => (exp, diffArguments),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBDifferentiate.differentiateExpression"));
                        __mm_s.push_str(&*literal!(" failed for: "));
                        __mm_s.push_str(&*Expression::toString(exp)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateExpressionNoCollect(
    mut expr: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut expr: metamodelica::Ref<Expression::NFExpression> = expr;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut oldCollect: bool;
    if (diffArguments.adjoint_map).is_some() {
        oldCollect = diffArguments.collectAdjoints.clone();
        assign_field!(diffArguments.collectAdjoints = false);
        (expr, diffArguments) = differentiateExpression(expr, diffArguments)?;
        assign_field!(diffArguments.collectAdjoints = oldCollect);
    } else {
        (expr, diffArguments) = differentiateExpression(expr, diffArguments)?;
    }
    Ok((expr, diffArguments))
}

pub(crate) fn differentiateComponentRef(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut der_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut derCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut strippedCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    var_ptr = (::match_deref::match_deref! { match &(&*exp) {
        _ if (diffArguments.diffType.clone() == DifferentiationType::FUNCTION.clone()) => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::EMPTY, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: __exp_cref, .. } => BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__exp_cref), metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"))?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateComponentRef")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dbg(&({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("[dCREF] exp="));
        __mm_s.push_str(&*Expression::toString(exp.clone())?);
        __mm_s.push_str(&*literal!(" | diffType="));
        __mm_s.push_str(&*DifferentiationArguments::diffTypeStr(diffArguments.diffType.clone()));
        __mm_s.push_str(&*literal!(" | scalarized="));
        __mm_s.push_str(&*boolString(diffArguments.scalarized.clone()));
        __mm_s.push_str(&*literal!(" | collectAdjoints="));
        __mm_s.push_str(&*boolString(diffArguments.collectAdjoints.clone()));
        ArcStr::from(__mm_s)
    }))?;
    if (diffArguments.adjoint_map).is_some() {
        dbg(&({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[dCREF] current_grad="));
            __mm_s.push_str(&*Expression::toString(diffArguments.current_grad.clone())?);
            ArcStr::from(__mm_s)
        }))?;
    }
    (exp, diffArguments) = (::match_deref::match_deref! { match &((exp.clone(), diffArguments.diffType.clone(), diffArguments.diff_map.clone())) {
        (Deref @ Expression::CREF { cref: Deref @ ComponentRef::EMPTY, .. }, _, _) => {
            (exp.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. }, _, _) => {
            (exp.clone(), diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::FUNCTION { .. }, Some(diff_map)) => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            strippedCref = ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF));
            if !(Type::isDiscrete(Type::arrayElementType(var_field!((*exp).ty, Expression::NFExpression::CREF)))?) && UnorderedMap::contains(strippedCref.clone(), diff_map.clone())? {
                derCref = UnorderedMap::getOrFail(strippedCref, diff_map.clone())?;
                derCref = ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF), derCref)?;
                res = Expression::fromCref(derCref, false)?;
            } else if !(Type::isDiscrete(Type::arrayElementType(var_field!((*exp).ty, Expression::NFExpression::CREF)))?) && ((derivativeOfPrefix(&strippedCref, diff_map.clone())?)).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(derivativeOfPrefix(&strippedCref, diff_map.clone())?) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                derCref = metamodelica::Own::own(__pa0);
                derCref = ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF), derCref)?;
                res = Expression::fromCref(derCref, false)?;
            } else {
                res = makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF))?;
            }
            (res, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if ((diffArguments.diffType.clone() == DifferentiationType::SIMPLE.clone() || diffArguments.diffType.clone() == DifferentiationType::TIME.clone()) && Type::isRecord(var_field!((*exp).ty, Expression::NFExpression::CREF)) && !(ComponentRef::isEqual(var_field!((*exp).cref, Expression::NFExpression::CREF), &diffArguments.diffCref)?) && BVariable::checkCref(var_field!((*exp).cref, Expression::NFExpression::CREF), &fnptr!(BVariable::isRecord, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"))?) => {
            differentiateRecordCref(exp.clone(), diffArguments.clone())?
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF))?) => {
            (Expression::makeOne(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if (ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF))?) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if (BVariable::isStart(var_ptr.clone())) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::SIMPLE, _) if (ComponentRef::isEqual(var_field!((*exp).cref, Expression::NFExpression::CREF), &diffArguments.diffCref)?) => {
            (makeOne(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::SIMPLE, _) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, _, _) if (BVariable::isParamOrConst(var_ptr.clone()) && !(ComponentRef::isTopLevel(var_field!((*exp).cref, Expression::NFExpression::CREF)) && BVariable::isInput(var_ptr.clone())) && !(BVariable::isOptimizable(var_ptr.clone()))) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isDiscrete(var_ptr.clone()) || BVariable::isDiscreteState(var_ptr.clone())) => {
            (Expression::makeZero(var_field!((*exp).ty, Expression::NFExpression::CREF))?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, Some(diff_map)) if (UnorderedMap::contains(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF)), diff_map.clone())?) => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            derCref = UnorderedMap::getOrFail(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF)), diff_map.clone())?;
            derCref = ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF), derCref)?;
            res = Expression::fromCref(derCref, false)?;
            (res, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isDummyState(var_ptr.clone())) => {
            (Expression::fromCref(BVariable::getPartnerCref(var_field!((*exp).cref, Expression::NFExpression::CREF), &BVariable::getVarDummyDer, false)?, false)?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isState(var_ptr.clone())) => {
            (Expression::fromCref(BVariable::getPartnerCref(var_field!((*exp).cref, Expression::NFExpression::CREF), &fnptr!(BVariable::getVarDer, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), false)?, false)?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::TIME, _) if (BVariable::isContinuous(var_ptr.clone(), false)?) => {
            (derCref, der_ptr) = BVariable::makeDerVar(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), false)?;
            assign_field!(diffArguments.new_vars = metamodelica::cons(der_ptr.clone(), diffArguments.new_vars.clone()));
            BVariable::setStateDerivativeVar(var_ptr.clone(), der_ptr);
            (Expression::fromCref(derCref, false)?, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::JACOBIAN, Some(diff_map)) if (diffArguments.scalarized.clone()) => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            let mut elem_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut elem_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut elem_res: metamodelica::Ref<Expression::NFExpression>;
            let mut hasSetSub: bool;
            if Type::isRecord(var_field!((*exp).ty, Expression::NFExpression::CREF)) && isMixedRecordDerivative(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF)), diff_map.clone())? {
                (res, diffArguments) = differentiateRecordCref(exp.clone(), diffArguments)?;
            } else if UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diff_map.clone())? {
                res = Expression::fromCref(UnorderedMap::getOrFail(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diff_map.clone())?, false)?;
                if diffArguments.collectAdjoints.clone() {
                    UnorderedMap::tryAddUpdate(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), &({ let __pe_b1 = diffArguments.current_grad.clone(); move |__pe_a0| Ok(updateAdjointList(__pe_a0, __pe_b1.clone())) }), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?;
                }
            } else {
                hasSetSub = false;
                elem_crefs = metamodelica::nil();
                if Type::isArray(var_field!((*exp).ty, Expression::NFExpression::CREF)) && Type::sizeOf(var_field!((*exp).ty, Expression::NFExpression::CREF), false)? <= 256 {
                    elem_crefs = ComponentRef::scalarizeAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), false)?.reverse();
                    for mut c in &*elem_crefs {
                        if UnorderedMap::contains(c.clone(), diff_map.clone())? {
                            hasSetSub = true;
                            break;
                        }
                    }
                }
                if hasSetSub {
                    elem_exps = metamodelica::nil();
                    for mut c in &*elem_crefs {
                        (elem_res, diffArguments) = differentiateComponentRef(Expression::fromCref(c.clone(), false)?, diffArguments)?;
                        elem_exps = metamodelica::cons(elem_res, elem_exps);
                    }
                    res = makeShapedArray(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), elem_exps.reverse())?;
                } else if Type::isRecord(var_field!((*exp).ty, Expression::NFExpression::CREF)) {
                    (res, diffArguments) = differentiateRecordCref(exp.clone(), diffArguments)?;
                } else {
                    res = differentiateIteratorElement(exp.clone(), diffArguments.clone(), diff_map.clone())?;
                }
            }
            (res, diffArguments.clone())
        },
        (Deref @ Expression::CREF { .. }, DifferentiationType::JACOBIAN, Some(diff_map)) if (!(diffArguments.scalarized.clone())) => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            let mut expCrefSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut adjointKey: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut elem_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut elem_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut elem_res: metamodelica::Ref<Expression::NFExpression>;
            let mut hasSetSub: bool;
            strippedCref = ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF));
            expCrefSubscripts = ComponentRef::subscriptsAllFlat(var_field!((*exp).cref, Expression::NFExpression::CREF))?;
            dbg(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] cref=")); __mm_s.push_str(&*ComponentRef::toString(var_field!((*exp).cref, Expression::NFExpression::CREF))?); __mm_s.push_str(&*literal!(" | stripped=")); __mm_s.push_str(&*ComponentRef::toString(&strippedCref)?); __mm_s.push_str(&*literal!(" | subs=")); __mm_s.push_str(&*Subscript::toStringList(expCrefSubscripts)?); ArcStr::from(__mm_s) }))?;
            if Type::isRecord(var_field!((*exp).ty, Expression::NFExpression::CREF)) && isMixedRecordDerivative(strippedCref.clone(), diff_map.clone())? {
                (res, diffArguments) = differentiateRecordCref(exp.clone(), diffArguments)?;
            } else if UnorderedMap::contains(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diff_map.clone())? {
                derCref = UnorderedMap::getOrFail(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), diff_map.clone())?;
                dbg(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] exact match -> ")); __mm_s.push_str(&*ComponentRef::toString(&derCref)?); ArcStr::from(__mm_s) }))?;
                res = Expression::fromCref(derCref.clone(), false)?;
                if diffArguments.collectAdjoints.clone() {
                    if !(UnorderedMap::contains(derCref.clone(), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?) {
                        UnorderedMap::tryAdd(derCref.clone(), metamodelica::nil(), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?;
                    }
                    UnorderedMap::tryAddUpdate(derCref, &({ let __pe_b1 = diffArguments.current_grad.clone(); move |__pe_a0| Ok(updateAdjointList(__pe_a0, __pe_b1.clone())) }), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?;
                }
            } else if UnorderedMap::contains(strippedCref.clone(), diff_map.clone())? {
                derCref = UnorderedMap::getOrFail(strippedCref, diff_map.clone())?;
                dbg(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] mapped -> ")); __mm_s.push_str(&*ComponentRef::toString(&derCref)?); ArcStr::from(__mm_s) }))?;
                res = Expression::fromCref(ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF), ComponentRef::stripSubscriptsAll(&derCref))?, false)?;
                dbg(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dCREF:JAC] get variable for derivative cref: ")); __mm_s.push_str(&*BVariable::pointerToString(BVariable::getVarPointer(&derCref, metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"))?)?); ArcStr::from(__mm_s) }))?;
                if diffArguments.collectAdjoints.clone() {
                    adjointKey = ComponentRef::copySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF), ComponentRef::stripSubscriptsAll(&derCref))?;
                    if !(UnorderedMap::contains(adjointKey.clone(), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?) {
                        UnorderedMap::tryAdd(adjointKey.clone(), metamodelica::nil(), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?;
                    }
                    UnorderedMap::tryAddUpdate(adjointKey, &({ let __pe_b1 = diffArguments.current_grad.clone(); move |__pe_a0| Ok(updateAdjointList(__pe_a0, __pe_b1.clone())) }), diffArguments.adjoint_map.clone().ok_or("pattern mismatch")?)?;
                } else {
                    dbg(&(literal!("[dCREF:JAC] collectAdjoints=false, skip append")))?;
                }
            } else {
                hasSetSub = false;
                for mut s in &*ComponentRef::subscriptsAllFlat(var_field!((*exp).cref, Expression::NFExpression::CREF))? {
                    if !(Subscript::isScalar(metamodelica::AsArg::as_arg(&s))?) && !(Subscript::isSliced(metamodelica::AsArg::as_arg(&s))) {
                        hasSetSub = true;
                    }
                }
                if !(hasSetSub) && Type::isArray(var_field!((*exp).ty, Expression::NFExpression::CREF)) && Type::sizeOf(var_field!((*exp).ty, Expression::NFExpression::CREF), false)? <= 256 {
                    for mut c in &*ComponentRef::scalarizeAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), false)?.reverse() {
                        if UnorderedMap::contains(c.clone(), diff_map.clone())? {
                            hasSetSub = true;
                            break;
                        }
                    }
                }
                if !(hasSetSub) {
                    if Type::isRecord(var_field!((*exp).ty, Expression::NFExpression::CREF)) {
                        (res, diffArguments) = differentiateRecordCref(exp.clone(), diffArguments)?;
                    } else {
                        res = differentiateIteratorElement(exp.clone(), diffArguments.clone(), diff_map.clone())?;
                    }
                } else {
                    elem_crefs = ComponentRef::scalarizeAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), false)?.reverse();
                    elem_exps = metamodelica::nil();
                    for mut c in &*elem_crefs {
                        (elem_res, diffArguments) = differentiateComponentRef(Expression::fromCref(c.clone(), false)?, diffArguments)?;
                        elem_exps = metamodelica::cons(elem_res, elem_exps);
                    }
                    res = makeShapedArray(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), elem_exps.reverse())?;
                }
            }
            (res, diffArguments.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateComponentRef")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, diffArguments))
}

pub(crate) fn makeShapedArray(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = Type::arrayDims(ty.clone());
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut row_size: i32;
    let mut n_rows: i32;
    let mut rows: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut row: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut remaining: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = elems.clone();
    if ((dims).len() as i32) < 2 {
        res = Expression::makeArray(
            ty,
            metamodelica::arrayFromVec(elems.into_iter().cloned().collect()),
            false,
        );
    } else {
        row_ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: Type::arrayElementType(&ty),
            dimensions: (dims).rest()?,
        });
        row_size = Type::sizeOf(&row_ty, false)?;
        if row_size < 1 || ((elems).len() as i32) != row_size * Dimension::size(&((dims).head().cloned()?), false)? {
            res = Expression::makeArray(
                ty,
                metamodelica::arrayFromVec(elems.into_iter().cloned().collect()),
                false,
            );
        } else {
            n_rows = Dimension::size(&((dims).head().cloned()?), false)?;
            for mut i in 1..=n_rows {
                (row, remaining) = List::split(remaining, row_size)?;
                rows = metamodelica::cons(makeShapedArray(row_ty.clone(), row)?, rows);
            }
            res = Expression::makeArray(
                ty,
                metamodelica::arrayFromVec(rows.reverse().into_iter().cloned().collect()),
                false,
            );
        }
    }
    Ok(res)
}

pub(crate) fn derivativeOfPrefix(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut derCref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    derCref = (::match_deref::match_deref! { match cref {
        Deref @ ComponentRef::CREF { restCref: rest @ Deref @ ComponentRef::CREF { .. }, .. } => {
            let mut der_rest: metamodelica::Ref<ComponentRef::NFComponentRef>;
            if UnorderedMap::contains(rest.clone(), diff_map.clone())? {
                derCref = Some(ComponentRef::prepend(UnorderedMap::getOrFail(rest.clone(), diff_map)?, cref.clone())?);
            } else {
                derCref = (::match_deref::match_deref! { match &(derivativeOfPrefix(metamodelica::AsArg::as_arg(&rest), diff_map)?) {
        Some(__esc_der_rest) => {
            der_rest = (*__esc_der_rest).clone();
            Some(ComponentRef::prepend(der_rest.clone(), cref.clone())?)
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            derCref
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(derCref)
}

pub(crate) fn isMixedRecordDerivative(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<bool> {
    let mut b: bool = false;
    let mut der_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        UnorderedMap::get(cref.clone(), diff_map.clone())?;
    let mut root: ArcStr;
    if (der_opt).is_some()
        && BVariable::checkCref(
            &cref,
            &fnptr!(
                BVariable::isRecord,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ),
            metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"),
        )?
    {
        root = crefRoot(&(der_opt.ok_or("pattern mismatch")?))?;
        for mut child in &*BVariable::getRecordChildrenCref(&cref)? {
            b = (::match_deref::match_deref! { match &(UnorderedMap::get(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&child)), diff_map.clone())?) {
                Some(child_der) => {
                    !metamodelica::stringEq(&(crefRoot(metamodelica::AsArg::as_arg(&child_der))?), &root)
                },
                _ => {
                    true
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            if b {
                break;
            }
        }
    }
    Ok(b)
}

pub(crate) fn crefRoot(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<ArcStr> {
    let mut root: ArcStr = (Util::stringSplitAtChar(ComponentRef::toString(cref)?, literal!("."))?)
        .head()
        .cloned()?;
    Ok(root)
}

pub(crate) fn makeZero(
    mut ty: &metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut zero: metamodelica::Ref<Expression::NFExpression>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut fields: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    zero = (match &**ty {
        Type::COMPLEX { .. }
            if (Type::isRecord(ty)
                && !(Restriction::isOperatorRecord(
                    &(Class::restriction(&(InstNode::getClass(Type::complexNode(ty)?)?))),
                ))) =>
        {
            node = Type::complexNode(ty)?;
            let __range0 = Class::getComponents(InstNode::getClass(node.clone())?)?
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut comp in __range0 {
                fields = metamodelica::cons(makeZero(&(InstNode::getType(comp)?))?, fields);
            }
            Expression::makeRecord(InstNode::fullPath(node, false)?, ty.clone(), fields.reverse())
        }
        Type::ARRAY { .. } if (Type::isRecord(&(Type::arrayElementType(ty)))) => {
            Expression::fillType(ty.clone(), makeZero(&(Type::arrayElementType(ty)))?)?
        }
        Type::STRING => metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") }),
        _ => Expression::makeZero(ty)?,
    });
    Ok(zero)
}

pub(crate) fn makeOne(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut one: metamodelica::Ref<Expression::NFExpression>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut fields: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    one = (match &**ty {
        Type::COMPLEX { .. }
            if (Type::isRecord(ty)
                && !(Restriction::isOperatorRecord(
                    &(Class::restriction(&(InstNode::getClass(Type::complexNode(ty)?)?))),
                ))) =>
        {
            node = Type::complexNode(ty)?;
            let __range0 = Class::getComponents(InstNode::getClass(node.clone())?)?
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut comp in __range0 {
                fields = metamodelica::cons(makeOne(&(InstNode::getType(comp)?))?, fields);
            }
            Expression::makeRecord(InstNode::fullPath(node, false)?, ty.clone(), fields.reverse())
        }
        Type::ARRAY { .. } if (Type::isRecord(&(Type::arrayElementType(ty)))) => {
            Expression::fillType(ty.clone(), makeOne(&(Type::arrayElementType(ty)))?)?
        }
        Type::STRING => metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") }),
        _ => Expression::makeOne(ty)?,
    });
    Ok(one)
}

pub(crate) fn differentiateRecordCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut elem: metamodelica::Ref<Expression::NFExpression>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    (cref, ty) = (match &*exp {
        Expression::CREF {
            cref: __exp_cref,
            ty: __exp_ty,
        } => (__exp_cref.clone(), __exp_ty.clone()),
        _ => (
            openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
            openmodelica_nf_frontend::NFType::interned_UNKNOWN(),
        ),
    });
    if Type::isRecord(&ty)
        && BVariable::checkCref(
            &cref,
            &fnptr!(
                BVariable::isRecord,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ),
            metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"),
        )?
    {
        children = BVariable::getRecordChildrenCref(&cref)?;
        if List::compareLength(children.clone(), Type::recordFields(&ty))? == 0 {
            for mut child in &*children {
                if Type::isString(&(ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&child), false)?))? {
                    elem = Expression::fromCref(child.clone(), false)?;
                } else {
                    (elem, diffArguments) =
                        differentiateComponentRef(Expression::fromCref(child.clone(), false)?, diffArguments)?;
                }
                elements = metamodelica::cons(elem, elements);
            }
        }
    }
    if (elements).is_empty() {
        exp = Expression::makeZero(&(Expression::typeOf(exp)))?;
    } else {
        exp = Expression::makeRecord(
            InstNode::fullPath(Type::complexNode(&ty)?, false)?,
            ty,
            elements.reverse(),
        );
    }
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateIteratorElement(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut base: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut elem_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut elem_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut elem_res: metamodelica::Ref<Expression::NFExpression>;
    let mut base_ty: metamodelica::Ref<Type::NFType>;
    let mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut found: bool;
    res = Expression::makeZero(&(Expression::typeOf(exp.clone())))?;
    (base, subs) = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => (
            ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref)),
            ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&__exp_cref))?,
        ),
        _ => (
            openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
            metamodelica::nil(),
        ),
    });
    if (subs).is_empty()
        || List::all(&subs, &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| {
            Subscript::isLiteral(&__a0)
        })?
    {
        return Ok(res);
    }
    base_ty = ComponentRef::getSubscriptedType(&base, true)?;
    if !(Type::isArray(&base_ty)) || Type::sizeOf(&base_ty, false)? > 256 {
        return Ok(res);
    }
    elem_crefs = ComponentRef::scalarizeAll(base, false)?.reverse();
    found = false;
    for mut c in &*elem_crefs {
        if UnorderedMap::contains(c.clone(), diff_map.clone())? {
            found = true;
            break;
        }
    }
    if !(found) {
        return Ok(res);
    }
    for mut c in &*elem_crefs {
        (elem_res, args) = differentiateComponentRef(Expression::fromCref(c.clone(), false)?, args)?;
        elem_exps = metamodelica::cons(elem_res, elem_exps);
    }
    res = Expression::applySubscripts(&subs, makeShapedArray(base_ty, elem_exps.reverse())?, false)?;
    Ok(res)
}

pub(crate) fn differentiateComponentRefNoCollect(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut oldCollect: bool;
    if (diffArguments.adjoint_map).is_some() {
        oldCollect = diffArguments.collectAdjoints.clone();
        assign_field!(diffArguments.collectAdjoints = false);
        (exp, diffArguments) = differentiateComponentRef(exp, diffArguments)?;
        assign_field!(diffArguments.collectAdjoints = oldCollect);
    } else {
        (exp, diffArguments) = differentiateComponentRef(exp, diffArguments)?;
    }
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateVariablePointer(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut diffArguments_ptr: Pointer::Pointer<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut diff_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> =
        Pointer::access(diffArguments_ptr.clone());
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut crefExp: metamodelica::Ref<Expression::NFExpression>;
    (crefExp, diffArguments) =
        differentiateComponentRefNoCollect(Expression::fromCref(var.name.clone(), false)?, diffArguments)?;
    diff_ptr = (::match_deref::match_deref! { match &(crefExp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::EMPTY, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::WILD, .. } => Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
        Deref @ Expression::CREF { cref: __crefExp_cref, .. } => BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__crefExp_cref), metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"))?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateVariablePointer")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*Variable::toString(&var, literal!(""), false)?); __mm_s.push_str(&*literal!(" because the result is expected to be a variable but turned out to be ")); __mm_s.push_str(&*Expression::toString(crefExp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(diffArguments_ptr, diffArguments);
    Ok(diff_ptr)
}

pub(crate) fn differentiateCall(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let debug: bool = false;
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nDifferentiate Exp-Call: "));
            __mm_s.push_str(&*Expression::toString(exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (exp, diffArguments) = ({
        let mut arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut skippedVarying: bool = false;
        (::match_deref::match_deref! { match &(exp.clone()) {
            ret @ Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
                let mut arg: metamodelica::Ref<Expression::NFExpression>;
                let mut ret = (*ret).clone();
                let mut call = (*call).clone();
                (arg, diffArguments) = differentiateExpression(var_field!((*call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), diffArguments)?;
                assign_variant_field!(call => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR; exp = arg);
                assign_variant_field!(ret => Expression::NFExpression::CALL; call = call.clone());
                (ret.clone(), diffArguments)
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                (ret, diffArguments) = differentiateReduction(&(AbsynUtil::pathString(NFFunction::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_REDUCTION)), literal!("."), true, false)?), exp, diffArguments)?;
                (ret, diffArguments)
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (NFFunction::Function::isBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL))) => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                (ret, diffArguments) = differentiateBuiltinCall(AbsynUtil::pathString(NFFunction::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?, exp, diffArguments)?;
                (ret, diffArguments)
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut arg: metamodelica::Ref<Expression::NFExpression>;
                let mut func_opt: Option<metamodelica::Ref<Function::Function>>;
                let mut der_func_opt: Option<metamodelica::Ref<Function::Function>>;
                let mut func: metamodelica::Ref<Function::Function>;
                let mut der_func: metamodelica::Ref<Function::Function>;
                let mut arguments_inputs: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<InstNode::InstNode>)>;
                let mut inp: metamodelica::Ref<InstNode::InstNode>;
                let mut isCont: bool;
                let mut isReal: bool;
                let mut isFunc: bool;
                let mut isSkipped: bool;
                let mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>;
                func_opt = UnorderedMap::get(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL).path.clone(), diffArguments.funcMap.clone())?;
                if (func_opt).is_some() {
                    let __pa0 = ::match_deref::match_deref! { match &(func_opt) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    func = metamodelica::Own::own(__pa0);
                    interface_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
                    arguments_inputs = List::zip(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone(), func.inputs.clone());
                    for mut tpl in &*arguments_inputs {
                        (arg, inp) = tpl.clone();
                        isCont = diffArguments.diffType.clone() == DifferentiationType::FUNCTION.clone() || BackendUtil::containsContinuousVar(arg.clone())?;
                        isReal = Type::isReal(&(Type::arrayElementType(&(Expression::typeOf(arg.clone())))))? || Type::isRecord(&(Type::arrayElementType(&(Expression::typeOf(arg)))));
                        isFunc = InstNode::isFunction(inp.clone())?;
                        isSkipped = Util::applyOptionOrDefault(func.interfaceDiffInfo.clone(), &({ let __pe_b0 = inp.clone(); move |__pe_a1| UnorderedSet::contains(__pe_b0.clone(), __pe_a1) }), false)?;
                        if isSkipped || !(isFunc || isCont && isReal) {
                            UnorderedMap::add(InstNode::name(&inp)?, !(isFunc || isReal), interface_map.clone())?;
                        }
                    }
                    der_func_opt = NFFunction::Function::getDerivative(&func, interface_map.clone())?;
                    if (der_func_opt).is_some() {
                        let __pa1 = ::match_deref::match_deref! { match &(der_func_opt) {
                            Some(__pa1) => __pa1.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        der_func = metamodelica::Own::own(__pa1);
                        (der_func, _) = addDiffInfo(&func, der_func, diffArguments.clone())?;
                    } else if List::any(&func.inputs, &InstNode::isFunction)? {
                        return Err("fail");
                    } else {
                        (der_func, diffArguments) = differentiateFunction(func.clone(), interface_map.clone(), diffArguments)?;
                    }
                    for mut tpl in &*arguments_inputs.reverse() {
                        (arg, inp) = tpl.clone();
                        isSkipped = Util::applyOptionOrDefault(func.interfaceDiffInfo.clone(), &({ let __pe_b0 = inp.clone(); move |__pe_a1| UnorderedSet::contains(__pe_b0.clone(), __pe_a1) }), false)?;
                        if !(isSkipped || UnorderedMap::getOrDefault(InstNode::name(&inp)?, interface_map.clone(), false)?) {
                            arguments = metamodelica::cons(arg, arguments);
                        } else if isSkipped && diffArguments.diffType.clone() != DifferentiationType::FUNCTION.clone() && BackendUtil::containsContinuousVar(arg.clone())? {
                            skippedVarying = true;
                        }
                    }
                    (arguments, diffArguments) = List::mapFold(&arguments, &differentiateExpression, diffArguments)?;
                    if diffArguments.diffType.clone() != DifferentiationType::FUNCTION.clone() && !(skippedVarying) && List::all(&arguments, &isZeroDerivative)? && !(Type::isTuple(&(Expression::typeOf(exp.clone())))) && !(Type::isComplex(&(Type::arrayElementType(&(Expression::typeOf(exp.clone())))))) && (!(Type::isArray(&(Expression::typeOf(exp.clone())))) || Type::hasKnownSize(Expression::typeOf(exp.clone()))?) {
                        ret = Expression::makeZero(&(Expression::typeOf(exp)))?;
                    } else {
                        arguments = listAppend(var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone(), arguments);
                        ret = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(der_func.clone(), arguments, var_field!((**call).var, Call::NFCall::TYPED_CALL).clone(), var_field!((**call).purity, Call::NFCall::TYPED_CALL).clone(), der_func.returnType.clone()) });
                    }
                } else {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateCall")); __mm_s.push_str(&*literal!(" failed because the function is not a builtin function and could not be found in the function tree: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                    return Err("fail");
                }
                (ret, diffArguments)
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    if debug {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Differentiate-ExpCall-result: "));
            __mm_s.push_str(&*Expression::toString(exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateReduction(
    mut name: &ArcStr,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } if (metamodelica::stringEq(&name, &(literal!("sum")))) => {
            let mut arg: metamodelica::Ref<Expression::NFExpression>;
            let mut call = (*call).clone();
            (arg, diffArguments) = differentiateExpression(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), diffArguments)?;
            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION; exp = arg);
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateReduction")); __mm_s.push_str(&*literal!(" failed because of non-call expression: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateBuiltinCall(
    mut name: ArcStr,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR.clone();
    let mut addOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::ADDITION.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    let mut mulOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::MULTIPLICATION.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    exp = ({
        let mut current_grad: metamodelica::Ref<Expression::NFExpression> = diffArguments.current_grad.clone();
        let mut isReverse: bool = (diffArguments.adjoint_map).is_some();
        (match &*exp.clone() {
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("delay")))) => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut ret2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg3: metamodelica::Ref<Expression::NFExpression>;
                let mut diffType: DifferentiationType;
                (arg1, arg2, arg3) = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg3, tail: Deref @ metamodelica::ListNode::Nil } } } => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        arg3 = (*__esc_arg3).clone();
                        (arg1.clone(), arg2.clone(), arg3.clone())
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ret1 = metamodelica::Ref::new(Expression::NFExpression::REAL {
                    value: if (diffArguments.diffType.clone() == DifferentiationType::TIME.clone()) {
                        metamodelica::OrderedFloat(1.0_f64)
                    } else {
                        metamodelica::OrderedFloat(0.0_f64)
                    },
                });
                (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments)?;
                ret2 = SimplifyExp::simplifyDump(
                    metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![ret1],
                        inv_arguments: list![ret2],
                        operator: addOp,
                    }),
                    true,
                    &(literal!("NBDifferentiate.differentiateBuiltinCall")),
                    &(literal!("")),
                )?;
                if Expression::isZero(&ret2)? {
                    ret = Expression::makeZero(&(Expression::typeOf(arg1)))?;
                } else {
                    diffType = diffArguments.diffType.clone();
                    assign_field!(diffArguments.diffType = DifferentiationType::TIME.clone());
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                    assign_field!(diffArguments.diffType = diffType);
                    assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![ret1, arg2, arg3])?);
                    ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![ret2, exp],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp,
                    });
                }
                ret
            }
            Expression::CALL { .. } if (metamodelica::stringEq(&name, &(literal!("smooth")))) => {
                let mut i: i32;
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut ret2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                ret = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1 @ Deref @ Expression::INTEGER { value: i }, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } if (i.clone() > 0) => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments)?;
                        assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() - 1 }), ret2])?);
                        exp
                    },
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1 @ Deref @ Expression::INTEGER { value: __esc_i }, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                        arg1 = (*__esc_arg1).clone();
                        i = (*__esc_i).clone();
                        arg2 = (*__esc_arg2).clone();
                        (ret2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments)?;
                        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::NO_EVENT().clone(), list![ret2.clone()], Expression::variability(ret2)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::NO_EVENT().returnType.clone()) });
                        exp
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ret
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("sum")))) => {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    assign_field!(diffArguments.current_grad = current_grad.clone());
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                    assign_field!(diffArguments.current_grad = current_grad);
                } else {
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1])?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("symmetric")))) => {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut ty: metamodelica::Ref<Type::NFType>;
                let mut elTy: metamodelica::Ref<Type::NFType>;
                let mut addM: metamodelica::Ref<Operator::NFOperator>;
                let mut subM: metamodelica::Ref<Operator::NFOperator>;
                let mut sumG: metamodelica::Ref<Expression::NFExpression>;
                let mut triuG: metamodelica::Ref<Expression::NFExpression>;
                let mut nExp: i32;
                let mut eyeNN: metamodelica::Ref<Expression::NFExpression>;
                let mut mulEW: metamodelica::Ref<Operator::NFOperator>;
                let mut diagG: metamodelica::Ref<Expression::NFExpression>;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    ty = Expression::typeOf(current_grad.clone());
                    elTy = if (Type::isArray(&ty)) {
                        Type::arrayElementType(&ty)
                    } else {
                        ty.clone()
                    };
                    nExp = Dimension::size(
                        &((Type::arrayDims(Expression::typeOf(arg1.clone()))).head().cloned()?),
                        false,
                    )?;
                    addM = Operator::fromClassification(
                        (
                            Operator::MathClassification::ADDITION.clone(),
                            Operator::SizeClassification::ELEMENT_WISE.clone(),
                        ),
                        ty.clone(),
                    )?;
                    subM = Operator::fromClassification(
                        (
                            Operator::MathClassification::SUBTRACTION.clone(),
                            Operator::SizeClassification::ELEMENT_WISE.clone(),
                        ),
                        ty.clone(),
                    )?;
                    mulEW = Operator::fromClassification(
                        (
                            Operator::MathClassification::MULTIPLICATION.clone(),
                            Operator::SizeClassification::ELEMENT_WISE.clone(),
                        ),
                        ty,
                    )?;
                    sumG = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: current_grad.clone(),
                        operator: addM,
                        exp2: typeTransposeCall(current_grad.clone())?,
                    });
                    triuG = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: sumG,
                        operator: mulEW.clone(),
                        exp2: Expression::makeTriuMask(nExp, elTy.clone())?,
                    });
                    eyeNN = Expression::makeIdentityMatrix(nExp, elTy)?;
                    diagG = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: current_grad.clone(),
                        operator: mulEW,
                        exp2: eyeNN,
                    });
                    assign_field!(
                        diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: triuG,
                            operator: subM,
                            exp2: diagG
                        })
                    );
                }
                (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                if isReverse {
                    assign_field!(diffArguments.current_grad = current_grad);
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1])?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("diagonal")))) => {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut nExp: i32;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    nExp = Dimension::size(
                        &((Type::arrayDims(Expression::typeOf(arg1.clone()))).head().cloned()?),
                        false,
                    )?;
                    assign_field!(
                        diffArguments.current_grad =
                            extractDiagonalVector(current_grad.clone(), nExp, Expression::typeOf(arg1.clone()))?
                    );
                }
                (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                if isReverse {
                    assign_field!(diffArguments.current_grad = current_grad);
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1])?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("matrix")))) => {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut grad_x: metamodelica::Ref<Expression::NFExpression>;
                let mut ty: metamodelica::Ref<Type::NFType>;
                let mut rX: i32;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    ty = Expression::typeOf(arg1.clone());
                    rX = if (Type::isArray(&ty)) {
                        Type::dimensionCount(ty)
                    } else {
                        0
                    };
                    grad_x = current_grad.clone();
                    if rX < 2 {
                        for mut i in 1..=2 - rX {
                            grad_x = dropLastDimIndex1(grad_x)?;
                        }
                    } else if rX > 2 {
                        grad_x = typePromoteCall(grad_x, rX)?;
                    }
                    assign_field!(diffArguments.current_grad = grad_x);
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                    assign_field!(diffArguments.current_grad = current_grad);
                } else {
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1])?);
                exp
            }
            Expression::CALL { call: __exp_call }
                if (List::contains(
                    &(list![
                        literal!("pre"),
                        literal!("noEvent"),
                        literal!("scalar"),
                        literal!("vector"),
                        literal!("transpose"),
                        literal!("skew")
                    ]),
                    name.clone(),
                    &fnptr!(stringEqual, ArcStr, ArcStr),
                )?) =>
            {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1])?);
                exp
            }
            Expression::CALL { call: __exp_call }
                if (List::contains(
                    &(list![literal!("homotopy"), literal!("$OMC$inStreamDiv")]),
                    name.clone(),
                    &fnptr!(stringEqual, ArcStr, ArcStr),
                )?) =>
            {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut ret2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                (arg1, arg2) = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        (arg1.clone(), arg2.clone())
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                (ret2, diffArguments) = differentiateExpression(arg2, diffArguments)?;
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1, ret2])?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("cat")))) => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut diffRest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                if isReverse {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall"));
                            __mm_s.push_str(&*literal!(" failed for: "));
                            __mm_s.push_str(&*Expression::toString(exp.clone())?);
                            __mm_s.push_str(&*literal!("\nReverse Mode not implemented for `cat()`."));
                            ArcStr::from(__mm_s)
                        },
                        metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"),
                    )?;
                    return Err("fail");
                }
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                arg1 = metamodelica::Own::own(__pa0);
                rest = metamodelica::Own::own(__pa1);
                diffRest = metamodelica::nil();
                for mut arg in &*rest.reverse() {
                    (ret, diffArguments) = differentiateExpression(arg.clone(), diffArguments)?;
                    diffRest = metamodelica::cons(ret, diffRest);
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), metamodelica::cons(arg1, diffRest))?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("promote")))) => {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                let mut old_grad: metamodelica::Ref<Expression::NFExpression>;
                let mut rY: i32;
                let mut rX: i32;
                (arg1, arg2) = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        (arg1.clone(), arg2.clone())
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                if isReverse {
                    rY = if (Type::isArray(&(Expression::typeOf(exp.clone())))) {
                        Type::dimensionCount(Expression::typeOf(exp.clone()))
                    } else {
                        0
                    };
                    rX = if (Type::isArray(&(Expression::typeOf(arg1.clone())))) {
                        Type::dimensionCount(Expression::typeOf(arg1.clone()))
                    } else {
                        0
                    };
                    current_grad = diffArguments.current_grad.clone();
                    old_grad = current_grad.clone();
                    for mut i in 1..=std::cmp::max(0, rY - rX) {
                        current_grad = dropLastDimIndex1(current_grad)?;
                    }
                    assign_field!(diffArguments.current_grad = current_grad);
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                    assign_field!(diffArguments.current_grad = old_grad);
                } else {
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![ret1, arg2])?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("identity")))) => {
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                metamodelica::Ref::new(Expression::NFExpression::CALL {
                    call: Call::makeTypedCall(
                        BuiltinFuncs::FILL_FUNC().clone(),
                        list![
                            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                            arg1.clone(),
                            arg1
                        ],
                        Variability::CONSTANT.clone(),
                        Prefixes::Purity::PURE.clone(),
                        BuiltinFuncs::FILL_FUNC().returnType.clone(),
                    ),
                })
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("fill")))) => {
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut old_grad: metamodelica::Ref<Expression::NFExpression>;
                let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut rY: i32;
                let mut rX: i32;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                arg1 = metamodelica::Own::own(__pa0);
                rest = metamodelica::Own::own(__pa1);
                if isReverse {
                    rY = if (Type::isArray(&(Expression::typeOf(exp.clone())))) {
                        Type::dimensionCount(Expression::typeOf(exp.clone()))
                    } else {
                        0
                    };
                    rX = if (Type::isArray(&(Expression::typeOf(arg1.clone())))) {
                        Type::dimensionCount(Expression::typeOf(arg1.clone()))
                    } else {
                        0
                    };
                    current_grad = diffArguments.current_grad.clone();
                    old_grad = current_grad.clone();
                    for mut i in 1..=std::cmp::max(0, rY - rX) {
                        current_grad = typeSumCall(current_grad)?;
                    }
                    assign_field!(diffArguments.current_grad = current_grad);
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                    assign_field!(diffArguments.current_grad = old_grad);
                } else {
                    (ret1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                }
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), metamodelica::cons(ret1, rest))?);
                exp
            }
            Expression::CALL { call: __exp_call } if (metamodelica::stringEq(&name, &(literal!("semiLinear")))) => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut ret2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg3: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg2: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg3: metamodelica::Ref<Expression::NFExpression>;
                let mut cond: metamodelica::Ref<Expression::NFExpression>;
                let mut grad_x: metamodelica::Ref<Expression::NFExpression>;
                let mut ty: metamodelica::Ref<Type::NFType>;
                (arg1, arg2, arg3) = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg3, tail: Deref @ metamodelica::ListNode::Nil } } } => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        arg3 = (*__esc_arg3).clone();
                        (arg1.clone(), arg2.clone(), arg3.clone())
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                current_grad = diffArguments.current_grad.clone();
                if isReverse {
                    cond = metamodelica::Ref::new(Expression::NFExpression::RELATION {
                        exp1: arg1.clone(),
                        operator: Operator::makeGreaterEq(Expression::typeOf(arg1.clone())),
                        exp2: Expression::makeZero(&(Expression::typeOf(arg1.clone())))?,
                        index: -1,
                    });
                    grad_x = metamodelica::Ref::new(Expression::NFExpression::IF {
                        ty: Expression::typeOf(arg1.clone()),
                        condition: cond,
                        trueBranch: metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![arg2.clone(), current_grad.clone()],
                            inv_arguments: metamodelica::nil(),
                            operator: mulOp.clone(),
                        }),
                        falseBranch: metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![arg3.clone(), current_grad.clone()],
                            inv_arguments: metamodelica::nil(),
                            operator: mulOp.clone(),
                        }),
                    });
                    assign_field!(diffArguments.current_grad = grad_x);
                }
                (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments)?;
                assign_field!(diffArguments.current_grad = current_grad);
                (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments)?;
                (diffArg3, diffArguments) = differentiateExpression(arg3.clone(), diffArguments)?;
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(__exp_call.clone(), list![arg1.clone(), diffArg2, diffArg3])?);
                ret = exp;
                if !(Expression::isZero(&diffArg1)?) {
                    ty = Expression::typeOf(diffArg1.clone());
                    ret1 = metamodelica::Ref::new(Expression::NFExpression::RELATION {
                        exp1: arg1,
                        operator: Operator::makeGreaterEq(ty.clone()),
                        exp2: Expression::makeZero(&ty)?,
                        index: -1,
                    });
                    ret1 = metamodelica::Ref::new(Expression::NFExpression::IF {
                        ty: ty,
                        condition: ret1,
                        trueBranch: arg2,
                        falseBranch: arg3,
                    });
                    ret2 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![diffArg1, ret1],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp,
                    });
                    ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![ret, ret2],
                        inv_arguments: metamodelica::nil(),
                        operator: addOp,
                    });
                }
                ret
            }
            Expression::CALL { call: __exp_call }
                if (metamodelica::stringEq(&name, &(literal!("min")))
                    || metamodelica::stringEq(&name, &(literal!("max")))) =>
            {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg2: metamodelica::Ref<Expression::NFExpression>;
                let mut cond1: metamodelica::Ref<Expression::NFExpression>;
                let mut cond2: metamodelica::Ref<Expression::NFExpression>;
                let mut zero1: metamodelica::Ref<Expression::NFExpression>;
                let mut zero2: metamodelica::Ref<Expression::NFExpression>;
                let mut grad_x: metamodelica::Ref<Expression::NFExpression>;
                let mut grad_y: metamodelica::Ref<Expression::NFExpression>;
                let mut old_grad: metamodelica::Ref<Expression::NFExpression>;
                let mut ty: metamodelica::Ref<Type::NFType>;
                ret = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments)?;
                        ty = Expression::typeOf(diffArg1.clone());
                        if Expression::isZero(&diffArg1)? {
                            ret = Expression::makeZero(&(Type::arrayElementType(&ty)))?;
                        } else {
                            ret1 = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(if (metamodelica::stringEq(&name, &(literal!("min")))) {BuiltinFuncs::ARG_MIN_ARR_REAL().clone()} else {BuiltinFuncs::ARG_MAX_ARR_REAL().clone()}, list![arg1.clone()], Expression::variability(arg1.clone())?, Prefixes::Purity::PURE.clone(), if (metamodelica::stringEq(&name, &(literal!("min")))) {BuiltinFuncs::ARG_MIN_ARR_REAL().returnType.clone()} else {BuiltinFuncs::ARG_MAX_ARR_REAL().returnType.clone()}) });
                            ret = Expression::applySubscripts(&(list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: ret1 })]), diffArg1, true)?;
                        }
                        ret
                    },
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        if isReverse {
                            current_grad = diffArguments.current_grad.clone();
                            cond1 = metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: arg1.clone(), operator: if (metamodelica::stringEq(&name, &(literal!("min")))) {Operator::makeLess(Expression::typeOf(arg1.clone()))} else {Operator::makeGreater(Expression::typeOf(arg1.clone()))}, exp2: arg2.clone(), index: -1 });
                            cond2 = metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: arg2.clone(), operator: if (metamodelica::stringEq(&name, &(literal!("min")))) {Operator::makeLess(Expression::typeOf(arg2.clone()))} else {Operator::makeGreater(Expression::typeOf(arg2.clone()))}, exp2: arg1.clone(), index: -1 });
                            zero1 = Expression::makeZero(&(Expression::typeOf(arg1.clone())))?;
                            zero2 = Expression::makeZero(&(Expression::typeOf(arg2.clone())))?;
                            grad_x = metamodelica::Ref::new(Expression::NFExpression::IF { ty: Expression::typeOf(arg1.clone()), condition: cond1, trueBranch: current_grad.clone(), falseBranch: zero1 });
                            grad_y = metamodelica::Ref::new(Expression::NFExpression::IF { ty: Expression::typeOf(arg2.clone()), condition: cond2, trueBranch: current_grad, falseBranch: zero2 });
                            old_grad = diffArguments.current_grad.clone();
                            assign_field!(diffArguments.current_grad = grad_x);
                            (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments)?;
                            assign_field!(diffArguments.current_grad = grad_y);
                            (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments)?;
                            assign_field!(diffArguments.current_grad = old_grad);
                        } else {
                            (diffArg1, diffArguments) = differentiateExpression(arg1.clone(), diffArguments)?;
                            (diffArg2, diffArguments) = differentiateExpression(arg2.clone(), diffArguments)?;
                        }
                        ty = Expression::typeOf(diffArg1.clone());
                        if Expression::isZero(&diffArg1)? && Expression::isZero(&diffArg2)? {
                            ret = Expression::makeZero(&ty)?;
                        } else {
                            ret1 = metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: arg1.clone(), operator: if (metamodelica::stringEq(&name, &(literal!("min")))) {Operator::makeLess(ty.clone())} else {Operator::makeGreater(ty.clone())}, exp2: arg2.clone(), index: -1 });
                            ret = metamodelica::Ref::new(Expression::NFExpression::IF { ty: ty, condition: ret1, trueBranch: diffArg1, falseBranch: diffArg2 });
                        }
                        ret
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ret
            }
            Expression::CALL { call: __exp_call }
                if (List::hasOneElement(&(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?))) =>
            {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg1: metamodelica::Ref<Expression::NFExpression>;
                arg1 = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Nil } => {
                        arg1 = (*__esc_arg1).clone();
                        arg1.clone()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ret = differentiateBuiltinCall1Arg(&name, arg1.clone())?;
                if !(Expression::isZero(&ret)?) {
                    current_grad = diffArguments.current_grad.clone();
                    assign_field!(
                        diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![current_grad.clone(), ret.clone()],
                            inv_arguments: metamodelica::nil(),
                            operator: mulOp.clone()
                        })
                    );
                    (diffArg1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                    assign_field!(diffArguments.current_grad = current_grad);
                    ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![ret, diffArg1],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp,
                    });
                }
                ret
            }
            Expression::CALL { call: __exp_call }
                if (((Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?).len() as i32) == 2) =>
            {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                let mut ret1: metamodelica::Ref<Expression::NFExpression>;
                let mut ret2: metamodelica::Ref<Expression::NFExpression>;
                let mut arg1: metamodelica::Ref<Expression::NFExpression>;
                let mut arg2: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffArg2: metamodelica::Ref<Expression::NFExpression>;
                (arg1, arg2) = (::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                        arg1 = (*__esc_arg1).clone();
                        arg2 = (*__esc_arg2).clone();
                        (arg1.clone(), arg2.clone())
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                (ret1, ret2) = differentiateBuiltinCall2Arg(&name, arg1.clone(), arg2.clone())?;
                current_grad = diffArguments.current_grad.clone();
                assign_field!(
                    diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![current_grad.clone(), ret1.clone()],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp.clone()
                    })
                );
                (diffArg1, diffArguments) = differentiateExpression(arg1, diffArguments)?;
                assign_field!(
                    diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![current_grad.clone(), ret2.clone()],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp.clone()
                    })
                );
                (diffArg2, diffArguments) = differentiateExpression(arg2, diffArguments)?;
                assign_field!(diffArguments.current_grad = current_grad);
                ret1 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![ret1, diffArg1],
                    inv_arguments: metamodelica::nil(),
                    operator: mulOp.clone(),
                });
                ret2 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![ret2, diffArg2],
                    inv_arguments: metamodelica::nil(),
                    operator: mulOp,
                });
                ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![ret1, ret2],
                    inv_arguments: metamodelica::nil(),
                    operator: addOp,
                });
                ret
            }
            Expression::CALL { call: __exp_call } => {
                let mut ret: metamodelica::Ref<Expression::NFExpression>;
                ret = (::match_deref::match_deref! { match &(Call::functionNameLast(metamodelica::AsArg::as_arg(&__exp_call))?) {
                    Deref @ "sample" => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ret
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall"));
                        __mm_s.push_str(&*literal!(" failed because of non-call expression: "));
                        __mm_s.push_str(&*Expression::toString(exp)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateBuiltinCall1Arg(
    mut name: &ArcStr,
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut derFuncCall: metamodelica::Ref<Expression::NFExpression>;
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR.clone();
    let mut powOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::POWER.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    let mut addOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::ADDITION.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    let mut mulOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::MULTIPLICATION.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    derFuncCall = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "sign" => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })
        },
        Deref @ "ceil" => {
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ "floor" => {
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) })
        },
        Deref @ "integer" => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })
        },
        Deref @ "abs" => {
            metamodelica::Ref::new(Expression::NFExpression::CAST { ty: Expression::typeOf(arg.clone()), exp: metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::SIGN().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::SIGN().returnType.clone()) }) })
        },
        Deref @ "sqrt" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "sin" => {
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::COS_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::COS_REAL().returnType.clone()) })
        },
        Deref @ "cos" => {
            Expression::negate(metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::SIN_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::SIN_REAL().returnType.clone()) }))
        },
        Deref @ "tan" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::COS_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::COS_REAL().returnType.clone()) });
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: ret, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "asin" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp.clone(), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: addOp });
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: ret, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "acos" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp.clone(), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: addOp });
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: ret, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(-1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "atan" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }), ret], inv_arguments: metamodelica::nil(), operator: addOp });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "sinh" => {
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::COSH_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::COSH_REAL().returnType.clone()) })
        },
        Deref @ "cosh" => {
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::SINH_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::SINH_REAL().returnType.clone()) })
        },
        Deref @ "tanh" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::TANH_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::TANH_REAL().returnType.clone()) });
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: ret, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: addOp });
            ret
        },
        Deref @ "acosh" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp.clone(), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![ret], inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], operator: addOp });
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: ret, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "asinh" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp.clone(), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![ret, metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: metamodelica::nil(), operator: addOp });
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: ret, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.5_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "atanh" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: addOp });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![ret], operator: mulOp });
            ret
        },
        Deref @ "exp" => {
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::EXP_REAL().clone(), list![arg.clone()], Expression::variability(arg)?, Prefixes::Purity::PURE.clone(), BuiltinFuncs::EXP_REAL().returnType.clone()) })
        },
        Deref @ "log" => {
            metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![arg], operator: mulOp })
        },
        Deref @ "log10" => {
            let mut ret: metamodelica::Ref<Expression::NFExpression>;
            ret = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::LOG_REAL().clone(), list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(10.0_f64) })], Variability::CONSTANT.clone(), Prefixes::Purity::PURE.clone(), BuiltinFuncs::LOG_REAL().returnType.clone()) });
            ret = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) })], inv_arguments: list![arg, ret], operator: mulOp });
            ret
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall1Arg")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(derFuncCall)
}

pub(crate) fn differentiateBuiltinCall2Arg(
    mut name: &ArcStr,
    mut arg1: metamodelica::Ref<Expression::NFExpression>,
    mut arg2: metamodelica::Ref<Expression::NFExpression>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut derFuncCall1: metamodelica::Ref<Expression::NFExpression>;
    let mut derFuncCall2: metamodelica::Ref<Expression::NFExpression>;
    let mut sizeClass: Operator::SizeClassification = Operator::SizeClassification::SCALAR.clone();
    let mut powOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::POWER.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    let mut addOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::ADDITION.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    let mut mulOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (Operator::MathClassification::MULTIPLICATION.clone(), sizeClass),
        openmodelica_nf_frontend::NFType::interned_REAL(),
    )?;
    (derFuncCall1, derFuncCall2) = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "div" => {
            (metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }), metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }))
        },
        Deref @ "mod" => {
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            let mut ret2: metamodelica::Ref<Expression::NFExpression>;
            exp2 = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::FLOOR().clone(), list![metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![arg1.clone()], inv_arguments: list![arg2.clone()], operator: mulOp })], Prefixes::variabilityMax(Expression::variability(arg1)?, Expression::variability(arg2)?), Prefixes::Purity::PURE.clone(), BuiltinFuncs::FLOOR().returnType.clone()) });
            ret2 = Expression::negate(exp2);
            (metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((1) as f64) }), ret2)
        },
        Deref @ "rem" => {
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            let mut ret2: metamodelica::Ref<Expression::NFExpression>;
            exp2 = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::DIV_REAL().clone(), list![arg1.clone(), arg2.clone()], Prefixes::variabilityMax(Expression::variability(arg1)?, Expression::variability(arg2)?), Prefixes::Purity::PURE.clone(), BuiltinFuncs::DIV_REAL().returnType.clone()) });
            ret2 = Expression::negate(exp2);
            (metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((1) as f64) }), ret2)
        },
        Deref @ "atan2" => {
            let mut exp1: metamodelica::Ref<Expression::NFExpression>;
            let mut exp2: metamodelica::Ref<Expression::NFExpression>;
            let mut ret1: metamodelica::Ref<Expression::NFExpression>;
            let mut ret2: metamodelica::Ref<Expression::NFExpression>;
            exp1 = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg1.clone(), operator: powOp.clone(), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            exp2 = metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: arg2.clone(), operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) });
            exp1 = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![exp1, exp2], inv_arguments: metamodelica::nil(), operator: addOp });
            ret1 = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![Expression::negate(arg2)], inv_arguments: list![exp1.clone()], operator: mulOp.clone() });
            ret2 = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![arg1], inv_arguments: list![exp1], operator: mulOp });
            (ret1, ret2)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBuiltinCall2Arg")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((derFuncCall1, derFuncCall2))
}

pub(crate) fn addDiffInfo(
    mut func: &metamodelica::Ref<Function::Function>,
    mut der_func: metamodelica::Ref<Function::Function>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Function::Function>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut der_func: metamodelica::Ref<Function::Function> = der_func;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diffInfo: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>;
    diffInfo = (::match_deref::match_deref! { match &(func.interfaceDiffInfo.clone()) {
        Some(__esc_diffInfo) => {
            diffInfo = (*__esc_diffInfo).clone();
            UnorderedSet::copy(diffInfo.clone())
        },
        _ => UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| InstNode::nameEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut node in &*func.inputs.clone() {
        UnorderedSet::add(node.clone(), diffInfo.clone())?;
    }
    for mut node in &*func.locals.clone() {
        UnorderedSet::add(node.clone(), diffInfo.clone())?;
    }
    for mut o in &*func.outputs.clone() {
        UnorderedSet::add(InstNode::fromHandle(metamodelica::AsArg::as_arg(&o))?, diffInfo.clone())?;
    }
    assign_field!(der_func.interfaceDiffInfo = Some(diffInfo));
    UnorderedMap::add(der_func.path.clone(), der_func.clone(), diffArguments.funcMap.clone())?;
    Ok((der_func, diffArguments))
}

pub(crate) fn differentiateFunction(
    mut func: metamodelica::Ref<Function::Function>,
    mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Function::Function>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut der_func: metamodelica::Ref<Function::Function>;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    der_func = ({
        let mut diff_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
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
        (::match_deref::match_deref! { match &(func.clone()) {
            __esc_der_func @ Deref @ Function::FUNCTION { .. } => {
                der_func = (*__esc_der_func).clone();
                let mut node: metamodelica::Ref<InstNode::InstNode>;
                let mut cls: Pointer::Pointer<metamodelica::Ref<Class::NFClass>>;
                let mut new_cls: metamodelica::Ref<Class::NFClass>;
                let mut funcDiffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
                let mut diffInfo: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>;
                let mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
                let mut funcDer: metamodelica::Ref<FunctionDerivative::NFFunctionDerivative>;
                let mut dummy_func: metamodelica::Ref<Function::Function>;
                let mut cachedData: metamodelica::Ref<CachedData::CachedData>;
                let mut der_func_name: ArcStr;
                let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
                let mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
                let mut outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
                let mut local_outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
                let mut uninitialized: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
                let mut slots: metamodelica::List<metamodelica::Ref<Slot::Slot>>;
                node = InstNode::fromHandle(&der_func.node)?;
                let __pa0 = ::match_deref::match_deref! { match &(node.clone()) {
                    Deref @ InstNode::CLASS_NODE { cls: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                cls = metamodelica::Own::own(__pa0);
                new_cls = (::match_deref::match_deref! { match &(Pointer::access(cls.clone())) {
            __esc_new_cls @ Deref @ Class::INSTANCED_CLASS { .. } => {
                new_cls = (*__esc_new_cls).clone();
                local_outputs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut lout in (der_func.outputs.clone()).into_iter().cloned() {
                let __x = InstNode::setComponentDirection(Prefixes::Direction::NONE.clone(), InstNode::fromHandle(&(lout.clone()))?)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                local_outputs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut lout in (local_outputs).into_iter().cloned() {
                let __x = InstNode::protect(lout.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                funcDiffArgs = DifferentiationArguments::default(DifferentiationType::TIME.clone(), UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Path>, __a1: metamodelica::Ref<Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Path>) -> Result<bool> + 'static>), 1));
                assign_field!(
                    funcDiffArgs.diffType = DifferentiationType::FUNCTION.clone(),
                    funcDiffArgs.funcMap = diffArguments.funcMap.clone()
                );
                diffInfo = (::match_deref::match_deref! { match &(der_func.interfaceDiffInfo.clone()) {
            Some(__esc_diffInfo) => {
                diffInfo = (*__esc_diffInfo).clone();
                UnorderedSet::copy(diffInfo.clone())
            },
            _ => UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| InstNode::nameEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static>), 13),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                createInterfaceDerivatives(&der_func.inputs, interface_map.clone(), diff_map.clone())?;
                createInterfaceDerivatives(&der_func.locals, interface_map.clone(), diff_map.clone())?;
                createInterfaceDerivatives(&(({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut o in (der_func.outputs.clone()).into_iter().cloned() {
                let __x = InstNode::fromHandle(&(o.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })), interface_map.clone(), diff_map.clone())?;
                assign_field!(funcDiffArgs.diff_map = Some(diff_map.clone()));
                (inputs, funcDiffArgs) = differentiateFunctionInterfaceNodes(der_func.inputs.clone(), interface_map.clone(), diff_map.clone(), funcDiffArgs, diffInfo.clone(), true)?;
                (locals, funcDiffArgs) = differentiateFunctionInterfaceNodes(der_func.locals.clone(), interface_map.clone(), diff_map.clone(), funcDiffArgs, diffInfo.clone(), false)?;
                (outputs, funcDiffArgs) = differentiateFunctionInterfaceNodes(({
            let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
            for mut o in (der_func.outputs.clone()).into_iter().cloned() {
                let __x = InstNode::fromHandle(&(o.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), interface_map.clone(), diff_map.clone(), funcDiffArgs, diffInfo.clone(), false)?;
                assign_field!(
                    der_func.inputs = inputs,
                    der_func.locals = List::flatten(list![der_func.locals.clone(), locals.clone(), local_outputs])?,
                    der_func.outputs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>> = metamodelica::nil();
            for mut o in (outputs).into_iter().cloned() {
                let __x = metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: o.clone() });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
                );
                assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS; elements = ClassTree::appendComponentsToFlatTree(locals, var_field!((*new_cls).elements, Class::NFClass::INSTANCED_CLASS).clone())?);
                (slots, funcDiffArgs) = createSlotDerivatives(&der_func.slots, interface_map.clone(), diff_map, funcDiffArgs)?;
                assign_field!(der_func.slots = listAppend(der_func.slots.clone(), slots));
                dummy_func = func.clone();
                node = InstNode::replaceClass(new_cls.clone(), node)?;
                der_func_name = { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BVariable::FUNCTION_DERIVATIVE_STR)); __mm_s.push_str(&*intString(((func.derivatives).len() as i32))); ArcStr::from(__mm_s) };
                node = InstNode::rename({ let mut __mm_s = String::new(); __mm_s.push_str(&*der_func_name); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*InstNode::name(&node)?); ArcStr::from(__mm_s) }, node)?;
                node = InstNode::setDefinition(SCodeUtil::setElementName(InstNode::definition(node.clone())?, InstNode::name(&node)?), node)?;
                assign_field!(
                    der_func.path = AbsynUtil::prefixPath(der_func_name, der_func.path.clone()),
                    der_func.derivatives = metamodelica::nil(),
                    der_func.derivedInputs = metamodelica::nil(),
                    der_func.interfaceDiffInfo = Some(diffInfo.clone())
                );
                cachedData = metamodelica::Ref::new(CachedData::CachedData::FUNCTION { funcs: list![der_func.clone()], typed: true, specialBuiltin: false });
                assign_field!(der_func.node = metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: InstNode::newFuncCache(node.clone(), cachedData)? }));
                funcDer = metamodelica::Ref::new(FunctionDerivative::NFFunctionDerivative { derivativeFn: InstNode::identityCell(InstNode::fromHandle(&der_func.node)?), derivedFn: InstNode::identityCell(InstNode::fromHandle(&dummy_func.node)?), order: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), conditions: FunctionDerivative::conditionsFromMap(interface_map.clone()), lowerOrderDerivatives: metamodelica::nil() });
                assign_field!(dummy_func.derivatives = metamodelica::cons(funcDer, dummy_func.derivatives.clone()));
                UnorderedMap::add(dummy_func.path.clone(), dummy_func, funcDiffArgs.funcMap.clone())?;
                funcDiffArgs = (::match_deref::match_deref! { match &(var_field!((*new_cls).sections, Class::NFClass::INSTANCED_CLASS).clone()) {
            sections @ Deref @ Sections::SECTIONS { .. } => {
                let mut sections = (*sections).clone();
                (algorithms, funcDiffArgs) = List::mapFold(var_field!((*sections).algorithms, Sections::NFSections::SECTIONS), &differentiateAlgorithm, funcDiffArgs)?;
                assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = algorithms);
                assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS; sections = sections.clone());
                funcDiffArgs
            },
            _ => {
                funcDiffArgs
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                let __pa0 = ::match_deref::match_deref! { match &(node.clone()) {
                    Deref @ InstNode::CLASS_NODE { cls: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                cls = metamodelica::Own::own(__pa0);
                Pointer::update(cls, new_cls.clone());
                assign_field!(
                    der_func.derivatives = metamodelica::nil(),
                    der_func.derivedInputs = metamodelica::nil(),
                    der_func.interfaceDiffInfo = Some(diffInfo)
                );
                cachedData = metamodelica::Ref::new(CachedData::CachedData::FUNCTION { funcs: list![der_func.clone()], typed: true, specialBuiltin: false });
                assign_field!(der_func.node = metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: InstNode::newFuncCache(node.clone(), cachedData)? }));
                uninitialized = NFFunction::Function::checkUseBeforeAssignGenerated(metamodelica::AsArg::as_arg(&der_func))?;
                if !((uninitialized).is_empty()) {
                    assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS; sections = NFFunction::Function::initializeUninitialized(var_field!((*new_cls).sections, Class::NFClass::INSTANCED_CLASS).clone(), uninitialized, AbsynUtil::pathString(der_func.path.clone(), literal!("."), true, false)?)?);
                    let __pa1 = ::match_deref::match_deref! { match &(node) {
                        Deref @ InstNode::CLASS_NODE { cls: __pa1, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cls = metamodelica::Own::own(__pa1);
                    Pointer::update(cls, new_cls.clone());
                }
                assign_field!(diffArguments.funcMap = funcDiffArgs.funcMap.clone());
                new_cls.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateFunction")); __mm_s.push_str(&*literal!(" failed for class ")); __mm_s.push_str(&*Class::toFlatString(&(Pointer::access(cls)), InstNode::fromHandle(&func.node)?, BaseModelica::defaultFormat.clone(), literal!(""))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                UnorderedMap::add(der_func.path.clone(), der_func.clone(), diffArguments.funcMap.clone())?;
                funcDer = metamodelica::Ref::new(FunctionDerivative::NFFunctionDerivative { derivativeFn: InstNode::identityCell(InstNode::fromHandle(&der_func.node)?), derivedFn: InstNode::identityCell(InstNode::fromHandle(&func.node)?), order: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), conditions: FunctionDerivative::conditionsFromMap(interface_map), lowerOrderDerivatives: metamodelica::nil() });
                assign_field!(func.derivatives = List::appendElt(funcDer, func.derivatives.clone()));
                UnorderedMap::add(func.path.clone(), func.clone(), diffArguments.funcMap.clone())?;
                der_func.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateFunction")); __mm_s.push_str(&*literal!(" failed for uninstantiated function ")); __mm_s.push_str(&*NFFunction::Function::signatureString(&func, true)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n[BEFORE] "));
            __mm_s.push_str(&*NFFunction::Function::toFlatString(
                &func,
                BaseModelica::defaultFormat.clone(),
                literal!(""),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n[AFTER ] "));
            __mm_s.push_str(&*NFFunction::Function::toFlatString(
                &der_func,
                BaseModelica::defaultFormat.clone(),
                literal!(""),
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((der_func, diffArguments))
}

pub(crate) fn differentiateFunctionInterfaceNodes(
    mut interface_nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut diffInfo: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>,
    mut keepOld: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut interface_nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = interface_nodes;
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut new_nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut d_node: metamodelica::Ref<InstNode::InstNode>;
    new_nodes = if (keepOld) {
        interface_nodes.clone().reverse()
    } else {
        metamodelica::nil()
    };
    for mut node in &*interface_nodes {
        if !(UnorderedMap::contains(
            InstNode::name(metamodelica::AsArg::as_arg(&node))?,
            interface_map.clone(),
        )?) {
            if !(UnorderedSet::contains(node.clone(), diffInfo.clone())?) {
                (d_node, diffArgs) = differentiateFunctionInterfaceNode(node.clone(), diff_map.clone(), diffArgs)?;
                new_nodes = metamodelica::cons(d_node, new_nodes);
                UnorderedSet::add(node.clone(), diffInfo.clone())?;
            }
        }
    }
    interface_nodes = new_nodes.reverse();
    Ok((interface_nodes, diffArgs))
}

pub(crate) fn differentiateFunctionInterfaceNode(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<InstNode::InstNode>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut d_node: metamodelica::Ref<InstNode::InstNode>;
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut diff_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut func: metamodelica::Ref<Function::Function>;
    let mut d_func: metamodelica::Ref<Function::Function>;
    cref = ComponentRef::fromNode(
        node.clone(),
        InstNode::getType(node.clone())?,
        metamodelica::nil(),
        ComponentRef::Origin::CREF.clone(),
    )?;
    diff_cref = UnorderedMap::getSafe(
        cref,
        diff_map,
        metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"),
    )?;
    diff_cref = (match &*diff_cref {
        ComponentRef::CREF { .. } if (InstNode::isComponent(&(ComponentRef::node(&diff_cref)?))?) => {
            d_node = ComponentRef::node(&diff_cref)?;
            comp = InstNode::component(&d_node)?;
            comp = (::match_deref::match_deref! { match &(comp.clone()) {
                __esc_comp @ Deref @ Component::COMPONENT { .. } => {
                    comp = (*__esc_comp).clone();
                    (binding, diffArgs) = differentiateBinding(var_field!((*comp).binding, Component::NFComponent::COMPONENT).clone(), diffArgs)?;
                    assign_variant_field!(comp => Component::NFComponent::COMPONENT; binding = binding);
                    comp.clone()
                },
                _ => comp,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            d_node = InstNode::replaceComponent(comp, d_node)?;
            assign_variant_field!(diff_cref => ComponentRef::NFComponentRef::CREF; node = ComponentRef::storeNode(d_node, true)?);
            diff_cref.clone()
        }
        _ => diff_cref.clone(),
    });
    if InstNode::isFunction(node.clone())? {
        func = (NFFunction::Function::getCachedFuncs(node)?).head().cloned()?;
        (d_func, diffArgs) = differentiateFunction(
            func,
            UnorderedMap::new(
                (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
                (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
                1,
            ),
            diffArgs,
        )?;
    }
    d_node = ComponentRef::node(&diff_cref)?;
    Ok((d_node, diffArgs))
}

pub(crate) fn createInterfaceDerivatives(
    mut interface_nodes: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<()> {
    fn addCref(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut diff_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    ) -> Result<()> {
        let mut diff_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        diff_cref = BVariable::makeFDerVar(cref.clone())?;
        UnorderedMap::add(cref.clone(), diff_cref, diff_map.clone())?;
        children = ComponentRef::getRecordChildren(cref)?;
        for mut child in &*children {
            addCref(child.clone(), diff_map.clone())?;
        }
        Ok(())
    }

    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    for mut node in &**interface_nodes {
        if !(UnorderedMap::contains(
            InstNode::name(metamodelica::AsArg::as_arg(&node))?,
            interface_map.clone(),
        )?) {
            cref = ComponentRef::fromNode(
                node.clone(),
                InstNode::getType(node.clone())?,
                metamodelica::nil(),
                ComponentRef::Origin::CREF.clone(),
            )?;
            addCref(cref, diff_map.clone())?;
        }
    }
    Ok(())
}

pub(crate) fn createSlotDerivatives(
    mut slots: &metamodelica::List<metamodelica::Ref<Slot::Slot>>,
    mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Slot::Slot>>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut new_slots: metamodelica::List<metamodelica::Ref<Slot::Slot>> = metamodelica::nil();
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut d_node: metamodelica::Ref<InstNode::InstNode>;
    let mut local_index: i32 = ((slots).len() as i32) + 1;
    for mut slot in &**slots {
        let mut slot = slot.clone();
        if !(UnorderedMap::contains(InstNode::name(&slot.node)?, interface_map.clone())?) {
            (d_node, diffArgs) = differentiateFunctionInterfaceNode(slot.node.clone(), diff_map.clone(), diffArgs)?;
            assign_field!(slot.node = d_node, slot.index = local_index);
            new_slots = metamodelica::cons(slot, new_slots);
            local_index = local_index + 1;
        }
    }
    new_slots = new_slots.reverse();
    Ok((new_slots, diffArgs))
}

pub(crate) fn resolvePartialDerivatives(
    mut func: metamodelica::Ref<Function::Function>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<metamodelica::Ref<Function::Function>> {
    let mut func: metamodelica::Ref<Function::Function> = func;
    let mut der_func: metamodelica::Ref<Function::Function>;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cls: Pointer::Pointer<metamodelica::Ref<Class::NFClass>>;
    let mut tmp_cls: Pointer::Pointer<metamodelica::Ref<Class::NFClass>>;
    let mut new_cls: metamodelica::Ref<Class::NFClass>;
    let mut wrap_cls: metamodelica::Ref<Class::NFClass>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    let mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
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
    let mut interface_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, bool>>;
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> =
        DifferentiationArguments::default(
            DifferentiationType::TIME.clone(),
            UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Path>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Path>) -> Result<i32> + 'static>),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Path>, __a1: metamodelica::Ref<Path>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Path>) -> Result<bool> + 'static,
                    >),
                1,
            ),
        );
    let mut diffInfo: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<InstNode::InstNode>>>;
    let mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut cachedData: metamodelica::Ref<CachedData::CachedData>;
    let mut diffCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut local_outputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut changed: bool = false;
    func = (::match_deref::match_deref! { match &(func.clone()) {
        __esc_der_func @ Deref @ Function::FUNCTION { .. } => {
            der_func = (*__esc_der_func).clone();
            let __pa0 = ::match_deref::match_deref! { match &(InstNode::fromHandle(&der_func.node)?) {
                Deref @ InstNode::CLASS_NODE { cls: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cls = metamodelica::Own::own(__pa0);
            wrap_cls = Pointer::access(cls);
            new_cls = (::match_deref::match_deref! { match &(wrap_cls.clone()) {
        __esc_wrap_cls @ Deref @ Class::TYPED_DERIVED { baseClass: __esc_node @ Deref @ InstNode::CLASS_NODE { cls: __esc_tmp_cls, .. }, .. } => {
            wrap_cls = (*__esc_wrap_cls).clone();
            node = (*__esc_node).clone();
            tmp_cls = (*__esc_tmp_cls).clone();
            new_cls = (::match_deref::match_deref! { match &(Pointer::access(tmp_cls.clone())) {
        __esc_new_cls @ Deref @ Class::INSTANCED_CLASS { sections: __esc_sections @ Deref @ Sections::SECTIONS { algorithms: __esc_algorithms, .. }, .. } => {
            new_cls = (*__esc_new_cls).clone();
            sections = (*__esc_sections).clone();
            algorithms = (*__esc_algorithms).clone();
            assign_field!(
                diffArgs.diffType = DifferentiationType::FUNCTION.clone(),
                diffArgs.funcMap = funcMap.clone()
            );
            diffInfo = (::match_deref::match_deref! { match &(der_func.interfaceDiffInfo.clone()) {
        Some(__esc_diffInfo) => {
            diffInfo = (*__esc_diffInfo).clone();
            UnorderedSet::copy(diffInfo.clone())
        },
        _ => UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| InstNode::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| InstNode::nameEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<InstNode::InstNode>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            interface_map = UnorderedMap::fromLists(&(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut var in (der_func.inputs.clone()).into_iter().cloned() {
            let __x = InstNode::name(&(var.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), List::fill(false, ((der_func.inputs).len() as i32)), (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
            for mut var in &*List::getAtIndexLst(der_func.inputs.clone(), der_func.derivedInputs.clone(), false)? {
                UnorderedMap::remove(InstNode::name(metamodelica::AsArg::as_arg(&var))?, interface_map.clone())?;
                local_outputs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        for mut node in (der_func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::setComponentDirection(Prefixes::Direction::NONE.clone(), InstNode::fromHandle(metamodelica::AsArg::as_arg(&node))?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                local_outputs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        for mut node in (local_outputs).into_iter().cloned() {
            let __x = InstNode::protect(node.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                createInterfaceDerivatives(&(list![var.clone()]), interface_map.clone(), diff_map.clone())?;
                createInterfaceDerivatives(&der_func.locals, interface_map.clone(), diff_map.clone())?;
                createInterfaceDerivatives(&(({
        let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        for mut o in (der_func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::fromHandle(&(o.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), interface_map.clone(), diff_map.clone())?;
                assign_field!(diffArgs.diff_map = Some(diff_map.clone()));
                (locals, diffArgs) = differentiateFunctionInterfaceNodes(der_func.locals.clone(), interface_map.clone(), diff_map.clone(), diffArgs, diffInfo.clone(), true)?;
                (outputs, diffArgs) = differentiateFunctionInterfaceNodes(({
        let mut __acc: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
        for mut o in (der_func.outputs.clone()).into_iter().cloned() {
            let __x = InstNode::fromHandle(&(o.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), interface_map.clone(), diff_map.clone(), diffArgs, diffInfo.clone(), false)?;
                diffCref = UnorderedMap::getSafe(ComponentRef::fromNode(var.clone(), InstNode::getType(var.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?, diff_map.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"))?;
                assign_field!(
                    der_func.locals = listAppend(locals, local_outputs),
                    der_func.outputs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>> = metamodelica::nil();
        for mut o in (outputs).into_iter().cloned() {
            let __x = metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: o.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                    der_func.interfaceDiffInfo = Some(diffInfo.clone())
                );
                (algorithms, diffArgs) = List::mapFold(metamodelica::AsArg::as_arg(&algorithms), &differentiateAlgorithm, diffArgs)?;
                algorithms = Algorithm::mapExpList(algorithms.clone(), &({ let __pe_b1 = Expression::fromCref(diffCref.clone(), false)?; let __pe_b2 = Expression::makeOne(&(ComponentRef::getSubscriptedType(&diffCref, false)?))?; move |__pe_a0| Replacements::single(__pe_a0, &__pe_b1, &__pe_b2) }))?;
                UnorderedMap::add(InstNode::name(metamodelica::AsArg::as_arg(&var))?, false, interface_map.clone())?;
            }
            assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = algorithms.clone());
            assign_variant_field!(new_cls => Class::NFClass::INSTANCED_CLASS;
                sections = sections.clone(),
                ty = var_field!((*wrap_cls).ty, Class::NFClass::TYPED_DERIVED).clone(),
                restriction = var_field!((*wrap_cls).restriction, Class::NFClass::TYPED_DERIVED).clone()
            );
            assign_variant_field!(node => InstNode::InstNode::CLASS_NODE; cls = Pointer::create(new_cls.clone()));
            assign_field!(
                der_func.derivatives = metamodelica::nil(),
                der_func.derivedInputs = metamodelica::nil(),
                der_func.interfaceDiffInfo = Some(diffInfo)
            );
            cachedData = metamodelica::Ref::new(CachedData::CachedData::FUNCTION { funcs: list![der_func.clone()], typed: true, specialBuiltin: false });
            assign_field!(der_func.node = metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: InstNode::newFuncCache(node.clone(), cachedData)? }));
            changed = true;
            new_cls.clone()
        },
        _ => wrap_cls.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_cls
        },
        _ => wrap_cls,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if changed {
                if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[BEFORE] ")); __mm_s.push_str(&*NFFunction::Function::toFlatString(&func, BaseModelica::defaultFormat.clone(), literal!(""))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[AFTER ] ")); __mm_s.push_str(&*NFFunction::Function::toFlatString(metamodelica::AsArg::as_arg(&der_func), BaseModelica::defaultFormat.clone(), literal!(""))?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                }
                UnorderedMap::add(der_func.path.clone(), der_func.clone(), funcMap)?;
            }
            der_func.clone()
        },
        _ => func,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub(crate) fn differentiateAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Algorithm::NFAlgorithm>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut statements: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>;
    let mut statements_flat: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut outputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut diffInfo: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Statement::NFStatement>>>;
    diffInfo = (::match_deref::match_deref! { match &(alg.stmtDiffInfo.clone()) {
        Some(__esc_diffInfo) => {
            diffInfo = (*__esc_diffInfo).clone();
            UnorderedSet::copy(diffInfo.clone())
        },
        _ => UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<Statement::NFStatement>| Statement::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Statement::NFStatement>, __a1: metamodelica::Ref<Statement::NFStatement>| Statement::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Statement::NFStatement>, metamodelica::Ref<Statement::NFStatement>) -> Result<bool> + 'static>), 13),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (statements, diffArguments) = List::mapFold(
        &alg.statements,
        &({
            let __pe_b1 = diffInfo.clone();
            move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        diffArguments,
    )?;
    for mut stmt in &*alg.statements.clone() {
        UnorderedSet::add(stmt.clone(), diffInfo.clone())?;
    }
    statements_flat = List::flatten(statements)?;
    (inputs, outputs) = Algorithm::getInputsOutputs(&statements_flat)?;
    alg = metamodelica::Ref::new(Algorithm::NFAlgorithm {
        statements: statements_flat,
        inputs: inputs,
        outputs: outputs,
        stmtDiffInfo: Some(diffInfo),
        scope: alg.scope.clone(),
        source: alg.source.clone(),
    });
    Ok((alg, diffArguments))
}

pub(crate) fn wildIfNotCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::TUPLE {
            elements: __exp_elements,
            ty: __exp_ty,
        } => metamodelica::Ref::new(Expression::NFExpression::TUPLE {
            ty: __exp_ty.clone(),
            elements: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut e in (__exp_elements.clone()).into_iter().cloned() {
                    let __x = if (Expression::isCref(&(e.clone()))) {
                        e.clone()
                    } else {
                        metamodelica::Ref::new(Expression::NFExpression::CREF {
                            ty: Expression::typeOf(e.clone()),
                            cref: openmodelica_nf_frontend::NFComponentRef::interned_WILD(),
                        })
                    };
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        _ => exp,
    });
    exp
}

pub(crate) fn differentiateStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut diffInfo: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Statement::NFStatement>>>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut diff_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    diff_stmts = ({
        let mut branches: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )> = metamodelica::nil();
        let mut isReverse: bool = (diffArguments.adjoint_map).is_some();
        (::match_deref::match_deref! { match &(stmt.clone()) {
            _ if (UnorderedSet::contains(stmt.clone(), diffInfo.clone())?) => {
                list![stmt.clone()]
            },
            diff_stmt @ Deref @ Statement::ASSIGNMENT { .. } if (Type::isReal(&(Type::arrayElementType(&(Expression::typeOf(var_field!((**diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone())))))?) => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut diff_stmt = (*diff_stmt).clone();
                (lhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments)?;
                (rhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments)?;
                assign_variant_field!(diff_stmt => Statement::NFStatement::ASSIGNMENT;
                    lhs = lhs,
                    rhs = SimplifyExp::simplifyDump(rhs, true, &(literal!("NBDifferentiate.differentiateStatement")), &(literal!("")))?
                );
                if (isReverse) {list![diff_stmt.clone()]} else {list![diff_stmt.clone(), stmt.clone()]}
            },
            diff_stmt @ Deref @ Statement::ASSIGNMENT { .. } if (Type::isComplex(&(Expression::typeOf(var_field!((**diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone()))) && Expression::isCall(var_field!((**diff_stmt).rhs, Statement::NFStatement::ASSIGNMENT))) => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut diff_stmt = (*diff_stmt).clone();
                (lhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments)?;
                (rhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments)?;
                assign_variant_field!(diff_stmt => Statement::NFStatement::ASSIGNMENT;
                    lhs = lhs,
                    rhs = SimplifyExp::simplifyDump(rhs, true, &(literal!("NBDifferentiate.differentiateStatement")), &(literal!("")))?
                );
                if (isReverse) {list![diff_stmt.clone()]} else {list![diff_stmt.clone(), stmt.clone()]}
            },
            diff_stmt @ Deref @ Statement::ASSIGNMENT { lhs: Deref @ Expression::TUPLE { .. }, .. } if (Expression::isCall(var_field!((**diff_stmt).rhs, Statement::NFStatement::ASSIGNMENT))) => {
                let mut lhs: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut diff_stmt = (*diff_stmt).clone();
                (lhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments)?;
                (rhs, diffArguments) = differentiateExpression(var_field!((*diff_stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), diffArguments)?;
                assign_variant_field!(diff_stmt => Statement::NFStatement::ASSIGNMENT;
                    lhs = wildIfNotCref(lhs),
                    rhs = SimplifyExp::simplifyDump(rhs, true, &(literal!("NBDifferentiate.differentiateStatement")), &(literal!("")))?
                );
                if (isReverse) {list![diff_stmt.clone()]} else {list![diff_stmt.clone(), stmt.clone()]}
            },
            diff_stmt @ Deref @ Statement::FOR { .. } => {
                let mut branch_stmts: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>;
                let mut diff_stmt = (*diff_stmt).clone();
                (branch_stmts, diffArguments) = List::mapFold(var_field!((*diff_stmt).body, Statement::NFStatement::FOR), &({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments)?;
                assign_variant_field!(diff_stmt => Statement::NFStatement::FOR; body = List::flatten(branch_stmts)?);
                list![diff_stmt.clone()]
            },
            diff_stmt @ Deref @ Statement::WHILE { .. } => {
                let mut branch_stmts: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>;
                let mut diff_stmt = (*diff_stmt).clone();
                (branch_stmts, diffArguments) = List::mapFold(var_field!((*diff_stmt).body, Statement::NFStatement::WHILE), &({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments)?;
                assign_variant_field!(diff_stmt => Statement::NFStatement::WHILE; body = List::flatten(branch_stmts)?);
                list![diff_stmt.clone()]
            },
            diff_stmt @ Deref @ Statement::FAILURE { .. } => {
                let mut branch_stmts: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>;
                let mut diff_stmt = (*diff_stmt).clone();
                (branch_stmts, diffArguments) = List::mapFold(var_field!((*diff_stmt).body, Statement::NFStatement::FAILURE), &({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments)?;
                assign_variant_field!(diff_stmt => Statement::NFStatement::FAILURE; body = List::flatten(branch_stmts)?);
                list![diff_stmt.clone()]
            },
            diff_stmt @ Deref @ Statement::IF { .. } => {
                let mut exp: metamodelica::Ref<Expression::NFExpression>;
                let mut branch_stmts_flat: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                let mut branch_stmts: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>;
                let mut diff_stmt = (*diff_stmt).clone();
                for mut branch in &*var_field!((*diff_stmt).branches, Statement::NFStatement::IF).clone() {
                    (exp, branch_stmts_flat) = branch.clone();
                    (branch_stmts, diffArguments) = List::mapFold(&branch_stmts_flat, &({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments)?;
                    branches = metamodelica::cons((exp, List::flatten(branch_stmts)?), branches);
                }
                assign_variant_field!(diff_stmt => Statement::NFStatement::IF; branches = branches.reverse());
                list![diff_stmt.clone()]
            },
            diff_stmt @ Deref @ Statement::WHEN { .. } => {
                let mut exp: metamodelica::Ref<Expression::NFExpression>;
                let mut branch_stmts_flat: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                let mut branch_stmts: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>;
                let mut diff_stmt = (*diff_stmt).clone();
                for mut branch in &*var_field!((*diff_stmt).branches, Statement::NFStatement::WHEN).clone() {
                    (exp, branch_stmts_flat) = branch.clone();
                    (branch_stmts, diffArguments) = List::mapFold(&branch_stmts_flat, &({ let __pe_b1 = diffInfo.clone(); move |__pe_a0, __pe_a2| differentiateStatement(__pe_a0, __pe_b1.clone(), __pe_a2) }), diffArguments)?;
                    branches = metamodelica::cons((exp, List::flatten(branch_stmts)?), branches);
                }
                assign_variant_field!(diff_stmt => Statement::NFStatement::WHEN; branches = branches.reverse());
                list![diff_stmt.clone()]
            },
            Deref @ Statement::ASSIGNMENT { .. } => {
                list![stmt.clone()]
            },
            Deref @ Statement::FUNCTION_ARRAY_INIT { .. } => {
                list![stmt.clone()]
            },
            Deref @ Statement::ASSERT { .. } => {
                list![stmt.clone()]
            },
            Deref @ Statement::TERMINATE { .. } => {
                list![stmt.clone()]
            },
            Deref @ Statement::NORETCALL { .. } => {
                list![stmt.clone()]
            },
            Deref @ Statement::RETURN { .. } => {
                list![stmt.clone()]
            },
            Deref @ Statement::BREAK { .. } => {
                list![stmt.clone()]
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateStatement")); __mm_s.push_str(&*literal!(" failed for:")); __mm_s.push_str(&*Statement::toString(&stmt, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok((diff_stmts, diffArguments))
}

pub(crate) fn reverseForRange(
    mut rangeIn: Option<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut rangeOut: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut startExp: metamodelica::Ref<Expression::NFExpression>;
    let mut stopExp: metamodelica::Ref<Expression::NFExpression>;
    let mut stepExp: metamodelica::Ref<Expression::NFExpression>;
    rangeOut = (::match_deref::match_deref! { match &(rangeIn.clone()) {
        Some(Deref @ Expression::RANGE { start: __esc_startExp, step: Some(__esc_stepExp), stop: __esc_stopExp, .. }) => {
            startExp = (*__esc_startExp).clone();
            stepExp = (*__esc_stepExp).clone();
            stopExp = (*__esc_stopExp).clone();
            Some(Expression::makeRange(stopExp.clone(), Some(Expression::negate(stepExp.clone())), startExp.clone())?)
        },
        Some(Deref @ Expression::RANGE { start: __esc_startExp, step: None, stop: __esc_stopExp, .. }) => {
            startExp = (*__esc_startExp).clone();
            stopExp = (*__esc_stopExp).clone();
            Some(Expression::makeRange(stopExp.clone(), Some(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: -1 })), startExp.clone())?)
        },
        _ => rangeIn,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rangeOut)
}

pub(crate) fn reverseEquationIterator(
    mut iterIn: &metamodelica::Ref<NBEquation::Iterator::Iterator>,
) -> Result<metamodelica::Ref<NBEquation::Iterator::Iterator>> {
    let mut iterOut: metamodelica::Ref<NBEquation::Iterator::Iterator>;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<NBEquation::Iterator::Iterator>>>;
    let mut revRanges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut o_range: Option<metamodelica::Ref<Expression::NFExpression>>;
    (names, ranges, maps) = NBEquation::Iterator::getFrames(iterIn);
    for mut range in &*ranges {
        o_range = reverseForRange(Some(range.clone()))?;
        revRanges = metamodelica::cons(o_range.ok_or("pattern mismatch")?, revRanges);
    }
    iterOut = NBEquation::Iterator::fromFrames(List::zip3(names, revRanges.reverse(), maps));
    Ok(iterOut)
}

pub(crate) fn bothZero(
    mut diffExp1: metamodelica::Ref<Expression::NFExpression>,
    mut diffExp2: metamodelica::Ref<Expression::NFExpression>,
    mut operator: &metamodelica::Ref<Operator::NFOperator>,
) -> Result<bool> {
    let mut b: bool = isZeroDerivative(diffExp1.clone())?
        && isZeroDerivative(diffExp2.clone())?
        && (!(Type::isArray(&(Operator::typeOf(operator)))) || Type::hasKnownSize(Operator::typeOf(operator))?);
    Ok(b)
}

pub(crate) fn isZeroDerivative(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool = Expression::isZero(&exp)? || Expression::isZero(&(SimplifyExp::simplify(exp.clone(), false)?))?;
    Ok(b)
}

pub(crate) fn differentiateBinary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("differentiateBinary: "));
            __mm_s.push_str(&*Expression::toString(exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (exp, diffArguments) = ({
        let mut current_grad: metamodelica::Ref<Expression::NFExpression> = diffArguments.current_grad.clone();
        let mut isReverse: bool = (diffArguments.adjoint_map).is_some();
        (match &*exp.clone() {
            Expression::BINARY { exp1, operator, exp2 }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::ADDITION.clone()) =>
            {
                let mut diffExp1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffExp2: metamodelica::Ref<Expression::NFExpression>;
                (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments)?;
                (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments)?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![diffExp1, diffExp2],
                        inv_arguments: metamodelica::nil(),
                        operator: operator.clone(),
                    }),
                    diffArguments,
                )
            }
            Expression::BINARY { exp1, operator, exp2 }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::SUBTRACTION.clone()) =>
            {
                let mut diffExp1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffExp2: metamodelica::Ref<Expression::NFExpression>;
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                current_grad = diffArguments.current_grad.clone();
                (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments)?;
                assign_field!(diffArguments.current_grad = Expression::negate(current_grad.clone()));
                (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments)?;
                assign_field!(diffArguments.current_grad = current_grad);
                (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
                addOp = Operator::fromClassification(
                    (Operator::MathClassification::ADDITION.clone(), sizeClass),
                    operator.ty.clone(),
                )?;
                (
                    metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![diffExp1],
                        inv_arguments: list![diffExp2],
                        operator: addOp,
                    }),
                    diffArguments,
                )
            }
            Expression::BINARY { exp1, operator, exp2 }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::MULTIPLICATION.clone()) =>
            {
                let mut diffExp1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffExp2: metamodelica::Ref<Expression::NFExpression>;
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                let mut grad_exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut grad_exp2: metamodelica::Ref<Expression::NFExpression>;
                let mut isVec1: bool;
                let mut isVec2: bool;
                let mut isMat1: bool;
                let mut isMat2: bool;
                let mut ty1: metamodelica::Ref<Type::NFType>;
                let mut ty2: metamodelica::Ref<Type::NFType>;
                let mut r1: i32;
                let mut r2: i32;
                let mut dim1: metamodelica::List<i32>;
                let mut dim2: metamodelica::List<i32>;
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    ty1 = Expression::typeOf(exp1.clone());
                    ty2 = Expression::typeOf(exp2.clone());
                    r1 = if (Type::isArray(&ty1)) {
                        Type::dimensionCount(ty1.clone())
                    } else {
                        0
                    };
                    r2 = if (Type::isArray(&ty2)) {
                        Type::dimensionCount(ty2.clone())
                    } else {
                        0
                    };
                    dim1 = if (r1 > 0) {
                        Dimension::sizes(Type::arrayDims(ty1), false)?
                    } else {
                        metamodelica::nil()
                    };
                    dim2 = if (r2 > 0) {
                        Dimension::sizes(Type::arrayDims(ty2), false)?
                    } else {
                        metamodelica::nil()
                    };
                    isVec1 = r1 == 1;
                    isVec2 = r2 == 1;
                    isMat1 = r1 == 2;
                    isMat2 = r2 == 2;
                    (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
                    if isVec1 && isVec2 && sizeClass == Operator::SizeClassification::SCALAR.clone() {
                        grad_exp1 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: current_grad.clone(),
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::SCALAR_ARRAY.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: exp2.clone(),
                        });
                        grad_exp2 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: current_grad.clone(),
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::SCALAR_ARRAY.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: exp1.clone(),
                        });
                    } else if isMat1
                        && isMat2
                        && sizeClass == Operator::SizeClassification::MATRIX.clone()
                        && (dim1).get(1)? > 1
                        && (dim1).get(2)? == 1
                        && (dim2).get(1)? == 1
                        && (dim2).get(2)? > 1
                    {
                        grad_exp1 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: current_grad.clone(),
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: exp2.clone(),
                        });
                        grad_exp2 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: typeTransposeCall(current_grad.clone())?,
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: exp1.clone(),
                        });
                    } else if isMat1 && isVec2 {
                        grad_exp1 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: current_grad.clone(),
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: typeTransposeCall(exp2.clone())?,
                        });
                        grad_exp2 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: typeTransposeCall(exp1.clone())?,
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX_VECTOR.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: current_grad.clone(),
                        });
                    } else if isVec1 && isMat2 {
                        grad_exp1 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: exp2.clone(),
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX_VECTOR.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: typeTransposeCall(current_grad.clone())?,
                        });
                        grad_exp2 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: typeTransposeCall(exp1.clone())?,
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: current_grad.clone(),
                        });
                    } else if isMat1 && isMat2 {
                        grad_exp1 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: current_grad.clone(),
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: typeTransposeCall(exp2.clone())?,
                        });
                        grad_exp2 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: typeTransposeCall(exp1.clone())?,
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::MATRIX.clone(),
                                ),
                                operator.ty.clone(),
                            )?,
                            exp2: current_grad.clone(),
                        });
                    } else {
                        grad_exp1 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![current_grad.clone(), exp2.clone()],
                            inv_arguments: metamodelica::nil(),
                            operator: makeMulFromOperator(metamodelica::AsArg::as_arg(&operator))?,
                        });
                        grad_exp2 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![current_grad.clone(), exp1.clone()],
                            inv_arguments: metamodelica::nil(),
                            operator: makeMulFromOperator(metamodelica::AsArg::as_arg(&operator))?,
                        });
                    }
                    assign_field!(diffArguments.current_grad = grad_exp1);
                    (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments)?;
                    assign_field!(diffArguments.current_grad = grad_exp2);
                    (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments)?;
                    assign_field!(diffArguments.current_grad = current_grad);
                } else {
                    (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments)?;
                    (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments)?;
                }
                sizeClass = Operator::classifyAddition(metamodelica::AsArg::as_arg(&operator));
                addOp = Operator::fromClassification(
                    (Operator::MathClassification::ADDITION.clone(), sizeClass),
                    operator.ty.clone(),
                )?;
                (
                    if (!(isReverse)
                        && bothZero(
                            diffExp1.clone(),
                            diffExp2.clone(),
                            metamodelica::AsArg::as_arg(&operator),
                        )?)
                    {
                        Expression::makeZero(&(Operator::typeOf(metamodelica::AsArg::as_arg(&operator))))?
                    } else {
                        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![
                                metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                    exp1: diffExp1,
                                    operator: operator.clone(),
                                    exp2: exp2.clone()
                                }),
                                metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                    exp1: exp1.clone(),
                                    operator: operator.clone(),
                                    exp2: diffExp2
                                })
                            ],
                            inv_arguments: metamodelica::nil(),
                            operator: addOp,
                        })
                    },
                    diffArguments,
                )
            }
            Expression::BINARY { exp1, operator, exp2 }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::DIVISION.clone()) =>
            {
                let mut diffExp1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffExp2: metamodelica::Ref<Expression::NFExpression>;
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut mulOp: metamodelica::Ref<Operator::NFOperator>;
                let mut powOp: metamodelica::Ref<Operator::NFOperator>;
                let mut divOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                let mut powSizeClass: Operator::SizeClassification;
                let mut denom2: metamodelica::Ref<Expression::NFExpression>;
                let mut numUF: metamodelica::Ref<Expression::NFExpression>;
                powSizeClass = Operator::SizeClassification::SCALAR.clone();
                powOp = Operator::fromClassification(
                    (Operator::MathClassification::POWER.clone(), powSizeClass),
                    openmodelica_nf_frontend::NFType::interned_REAL(),
                )?;
                if isReverse {
                    current_grad = diffArguments.current_grad.clone();
                    assign_field!(
                        diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![current_grad.clone()],
                            inv_arguments: list![exp2.clone()],
                            operator: Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    if (Type::isArray(&(Expression::typeOf(current_grad.clone())))) {
                                        Operator::SizeClassification::ARRAY_SCALAR.clone()
                                    } else {
                                        Operator::SizeClassification::SCALAR.clone()
                                    }
                                ),
                                operator.ty.clone()
                            )?
                        })
                    );
                }
                (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments)?;
                if isReverse {
                    denom2 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: exp2.clone(),
                        operator: powOp.clone(),
                        exp2: metamodelica::Ref::new(Expression::NFExpression::REAL {
                            value: metamodelica::OrderedFloat(2.0_f64),
                        }),
                    });
                    numUF = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                        exp1: current_grad.clone(),
                        operator: if (Type::isArray(&(Expression::typeOf(exp1.clone())))) {
                            Operator::makeScalarProduct(operator.ty.clone())
                        } else {
                            Operator::fromClassification(
                                (
                                    Operator::MathClassification::MULTIPLICATION.clone(),
                                    Operator::SizeClassification::SCALAR.clone(),
                                ),
                                openmodelica_nf_frontend::NFType::interned_REAL(),
                            )?
                        },
                        exp2: exp1.clone(),
                    });
                    divOp = Operator::fromClassification(
                        (
                            Operator::MathClassification::DIVISION.clone(),
                            Operator::SizeClassification::SCALAR.clone(),
                        ),
                        openmodelica_nf_frontend::NFType::interned_REAL(),
                    )?;
                    assign_field!(
                        diffArguments.current_grad =
                            Expression::negate(metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                exp1: numUF,
                                operator: divOp,
                                exp2: denom2
                            }))
                    );
                }
                (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments)?;
                if isReverse {
                    assign_field!(diffArguments.current_grad = current_grad);
                }
                (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
                addOp = Operator::fromClassification(
                    (
                        Operator::MathClassification::ADDITION.clone(),
                        Operator::classifyAddition(metamodelica::AsArg::as_arg(&operator)),
                    ),
                    operator.ty.clone(),
                )?;
                mulOp = Operator::fromClassification(
                    (Operator::MathClassification::MULTIPLICATION.clone(), sizeClass),
                    operator.ty.clone(),
                )?;
                (
                    if (!(isReverse)
                        && bothZero(
                            diffExp1.clone(),
                            diffExp2.clone(),
                            metamodelica::AsArg::as_arg(&operator),
                        )?)
                    {
                        Expression::makeZero(&(Operator::typeOf(metamodelica::AsArg::as_arg(&operator))))?
                    } else {
                        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                arguments: list![metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                    exp1: diffExp1,
                                    operator: mulOp.clone(),
                                    exp2: exp2.clone()
                                })],
                                inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                    exp1: exp1.clone(),
                                    operator: mulOp.clone(),
                                    exp2: diffExp2
                                })],
                                operator: addOp
                            })],
                            inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                exp1: exp2.clone(),
                                operator: powOp,
                                exp2: metamodelica::Ref::new(Expression::NFExpression::REAL {
                                    value: metamodelica::OrderedFloat(2.0_f64)
                                })
                            })],
                            operator: mulOp,
                        })
                    },
                    diffArguments,
                )
            }
            Expression::BINARY { exp1, operator, .. }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::POWER.clone()
                    && Expression::isZero(metamodelica::AsArg::as_arg(&exp1))?) =>
            {
                (Expression::makeZero(&operator.ty)?, diffArguments)
            }
            Expression::BINARY { exp1, operator, exp2 }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::POWER.clone()) =>
            {
                let mut diffExp1: metamodelica::Ref<Expression::NFExpression>;
                let mut diffExp2: metamodelica::Ref<Expression::NFExpression>;
                let mut e1: metamodelica::Ref<Expression::NFExpression>;
                let mut e2: metamodelica::Ref<Expression::NFExpression>;
                let mut e3: metamodelica::Ref<Expression::NFExpression>;
                let mut res: metamodelica::Ref<Expression::NFExpression>;
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut mulOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
                addOp = Operator::fromClassification(
                    (Operator::MathClassification::ADDITION.clone(), sizeClass),
                    operator.ty.clone(),
                )?;
                current_grad = diffArguments.current_grad.clone();
                assign_field!(
                    diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![
                            current_grad.clone(),
                            exp2.clone(),
                            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                exp1: exp1.clone(),
                                operator: operator.clone(),
                                exp2: minusOne(exp2.clone(), addOp.clone())?
                            })
                        ],
                        inv_arguments: metamodelica::nil(),
                        operator: makeMulFromOperator(metamodelica::AsArg::as_arg(&operator))?
                    })
                );
                (diffExp1, diffArguments) = differentiateExpression(exp1.clone(), diffArguments)?;
                assign_field!(
                    diffArguments.current_grad = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![current_grad.clone(), exp.clone(), expLog(exp1.clone())?],
                        inv_arguments: metamodelica::nil(),
                        operator: makeMulFromOperator(metamodelica::AsArg::as_arg(&operator))?
                    })
                );
                (diffExp2, diffArguments) = differentiateExpression(exp2.clone(), diffArguments)?;
                assign_field!(diffArguments.current_grad = current_grad);
                diffExp1 = SimplifyExp::simplifyDump(
                    diffExp1,
                    true,
                    &(literal!("NBDifferentiate.differentiateBinary")),
                    &(literal!("")),
                )?;
                diffExp2 = SimplifyExp::simplifyDump(
                    diffExp2,
                    true,
                    &(literal!("NBDifferentiate.differentiateBinary")),
                    &(literal!("")),
                )?;
                mulOp = Operator::fromClassification(
                    (Operator::MathClassification::MULTIPLICATION.clone(), sizeClass),
                    operator.ty.clone(),
                )?;
                res = (match (Expression::isZero(&diffExp1)?, Expression::isZero(&diffExp2)?) {
                    (true, true) => Expression::makeZero(&operator.ty)?,
                    (false, true) => metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![
                            exp2.clone(),
                            metamodelica::Ref::new(Expression::NFExpression::BINARY {
                                exp1: exp1.clone(),
                                operator: operator.clone(),
                                exp2: minusOne(exp2.clone(), addOp)?
                            }),
                            diffExp1
                        ],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp,
                    }),
                    (true, false) => metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![exp, expLog(exp1.clone())?, diffExp2],
                        inv_arguments: metamodelica::nil(),
                        operator: mulOp,
                    }),
                    _ => {
                        e1 = metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: exp1.clone(),
                            operator: operator.clone(),
                            exp2: minusOne(exp2.clone(), addOp.clone())?,
                        });
                        e2 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![exp1.clone(), expLog(exp1.clone())?, diffExp2],
                            inv_arguments: metamodelica::nil(),
                            operator: mulOp.clone(),
                        });
                        e3 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![exp2.clone(), diffExp1],
                            inv_arguments: metamodelica::nil(),
                            operator: mulOp.clone(),
                        });
                        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: list![
                                e1,
                                metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                    arguments: list![e2, e3],
                                    inv_arguments: metamodelica::nil(),
                                    operator: addOp
                                })
                            ],
                            inv_arguments: metamodelica::nil(),
                            operator: mulOp,
                        })
                    }
                });
                (res, diffArguments)
            }
            Expression::BINARY { operator, .. }
                if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                    == Operator::MathClassification::LOGICAL.clone()
                    || Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))?
                        == Operator::MathClassification::RELATION.clone()) =>
            {
                (exp, diffArguments)
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBDifferentiate.differentiateBinary"));
                        __mm_s.push_str(&*literal!(" failed for: "));
                        __mm_s.push_str(&*Expression::toString(exp)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateMultary(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut isReverse: bool = (diffArguments.adjoint_map).is_some();
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("differentiateMultary: "));
            __mm_s.push_str(&*Expression::toString(exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    exp = ({
        let mut new_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut new_inv_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
            metamodelica::nil();
        let mut current_grad: metamodelica::Ref<Expression::NFExpression> = diffArguments.current_grad.clone();
        let mut hasArray: bool = false;
        (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::MULTARY { arguments, inv_arguments, operator } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))? == Operator::MathClassification::ADDITION.clone()) => {
                let mut diff_arg: metamodelica::Ref<Expression::NFExpression>;
                let mut local_grad: metamodelica::Ref<Expression::NFExpression>;
                if isReverse {
                    hasArray = List::any(metamodelica::AsArg::as_arg(&arguments), &fnptr!(Expression::hasArrayType, metamodelica::Ref<Expression::NFExpression>))? || List::any(metamodelica::AsArg::as_arg(&inv_arguments), &fnptr!(Expression::hasArrayType, metamodelica::Ref<Expression::NFExpression>))?;
                }
                for mut arg in &*arguments.clone().reverse() {
                    if isReverse {
                        current_grad = diffArguments.current_grad.clone();
                        if Expression::isScalar(arg.clone()) && hasArray {
                            assign_field!(diffArguments.current_grad = typeSumCall(current_grad.clone())?);
                        } else {
                            assign_field!(diffArguments.current_grad = current_grad.clone());
                        }
                    }
                    (diff_arg, diffArguments) = differentiateExpression(arg.clone(), diffArguments)?;
                    if isReverse {
                        assign_field!(diffArguments.current_grad = current_grad.clone());
                    } else {
                        new_arguments = metamodelica::cons(diff_arg, new_arguments);
                    }
                }
                for mut arg in &*inv_arguments.clone().reverse() {
                    if isReverse {
                        current_grad = diffArguments.current_grad.clone();
                        local_grad = Expression::negate(current_grad.clone());
                        if Expression::isScalar(arg.clone()) && hasArray {
                            local_grad = typeSumCall(local_grad)?;
                        }
                        assign_field!(diffArguments.current_grad = local_grad);
                    }
                    (diff_arg, diffArguments) = differentiateExpression(arg.clone(), diffArguments)?;
                    if isReverse {
                        assign_field!(diffArguments.current_grad = current_grad.clone());
                    } else {
                        new_inv_arguments = metamodelica::cons(diff_arg, new_inv_arguments);
                    }
                }
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: new_arguments, inv_arguments: new_inv_arguments, operator: operator.clone() })
            },
            Deref @ Expression::MULTARY { arguments, inv_arguments: Deref @ metamodelica::ListNode::Nil, operator } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))? == Operator::MathClassification::MULTIPLICATION.clone()) => {
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                sizeClass = Operator::classifyAddition(metamodelica::AsArg::as_arg(&operator));
                addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass), operator.ty.clone())?;
                (new_arguments, diffArguments) = differentiateMultaryMultiplicationArgs(metamodelica::AsArg::as_arg(&arguments), diffArguments, operator.clone())?;
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: new_arguments, inv_arguments: metamodelica::nil(), operator: addOp })
            },
            Deref @ Expression::MULTARY { arguments, inv_arguments, operator } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))? == Operator::MathClassification::MULTIPLICATION.clone() && !((inv_arguments).is_empty()) && isReverse) => {
                let mut diff_arg: metamodelica::Ref<Expression::NFExpression>;
                let mut mulEWOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                let mut powSizeClass: Operator::SizeClassification;
                let mut upstream: metamodelica::Ref<Expression::NFExpression>;
                let mut e_over_f: metamodelica::Ref<Expression::NFExpression>;
                let mut e_over_g: metamodelica::Ref<Expression::NFExpression>;
                let mut numProd: metamodelica::Ref<Expression::NFExpression>;
                let mut denomProd: metamodelica::Ref<Expression::NFExpression>;
                let mut arg_rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut hasArrayNum: bool;
                let mut localUpF: metamodelica::Ref<Expression::NFExpression>;
                let mut localUpG: metamodelica::Ref<Expression::NFExpression>;
                let mut i: i32;
                (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
                Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass), operator.ty.clone())?;
                makeMulFromOperator(metamodelica::AsArg::as_arg(&operator))?;
                mulEWOp = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), operator.ty.clone())?;
                Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), Operator::SizeClassification::ELEMENT_WISE.clone()), operator.ty.clone())?;
                hasArrayNum = List::any(metamodelica::AsArg::as_arg(&arguments), &fnptr!(Expression::hasArrayType, metamodelica::Ref<Expression::NFExpression>))?;
                numProd = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: arguments.clone(), inv_arguments: metamodelica::nil(), operator: operator.clone() });
                denomProd = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: inv_arguments.clone(), inv_arguments: metamodelica::nil(), operator: operator.clone() });
                upstream = diffArguments.current_grad.clone();
                i = 1;
                for mut f in &*arguments.clone() {
                    arg_rest = listDelete(arguments.clone(), i)?;
                    e_over_f = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: arg_rest, inv_arguments: list![denomProd.clone()], operator: operator.clone() });
                    localUpF = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![upstream.clone(), e_over_f], inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() });
                    if Expression::isScalar(f.clone()) && hasArrayNum {
                        localUpF = typeSumCall(localUpF)?;
                    }
                    assign_field!(diffArguments.current_grad = localUpF);
                    (diff_arg, diffArguments) = differentiateExpression(f.clone(), diffArguments)?;
                    i = i + 1;
                }
                i = 1;
                powSizeClass = if (Expression::hasArrayType((inv_arguments).head().cloned()?)) {Operator::SizeClassification::ARRAY_SCALAR.clone()} else {Operator::SizeClassification::SCALAR.clone()};
                Operator::fromClassification((Operator::MathClassification::POWER.clone(), powSizeClass), openmodelica_nf_frontend::NFType::interned_REAL())?;
                for mut g in &*inv_arguments.clone() {
                    listDelete(inv_arguments.clone(), i)?;
                    e_over_g = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![numProd.clone()], inv_arguments: metamodelica::cons(g.clone(), inv_arguments.clone()), operator: operator.clone() });
                    localUpG = Expression::negate(metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![upstream.clone(), e_over_g.clone()], inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() }));
                    if hasArrayNum {
                        localUpG = typeSumCall(localUpG)?;
                    }
                    assign_field!(diffArguments.current_grad = localUpG);
                    (diff_arg, diffArguments) = differentiateExpression(g.clone(), diffArguments)?;
                    Expression::negate(metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![diff_arg, e_over_g], inv_arguments: metamodelica::nil(), operator: mulEWOp.clone() }));
                    i = i + 1;
                }
                assign_field!(diffArguments.current_grad = upstream);
                openmodelica_nf_frontend::NFExpression::interned_END()
            },
            Deref @ Expression::MULTARY { arguments, inv_arguments, operator } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))? == Operator::MathClassification::MULTIPLICATION.clone() && !((inv_arguments).is_empty())) => {
                let mut divisor: metamodelica::Ref<Expression::NFExpression>;
                let mut diff_enumerator: metamodelica::Ref<Expression::NFExpression>;
                let mut diff_divisor: metamodelica::Ref<Expression::NFExpression>;
                let mut diff_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut diff_inv_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut addOp: metamodelica::Ref<Operator::NFOperator>;
                let mut powOp: metamodelica::Ref<Operator::NFOperator>;
                let mut sizeClass: Operator::SizeClassification;
                let mut powSizeClass: Operator::SizeClassification;
                let mut powTy: metamodelica::Ref<Type::NFType>;
                if !((inv_arguments).is_empty()) && Type::isArray(&(Expression::typeOf((inv_arguments).head().cloned()?))) {
                    powSizeClass = Operator::SizeClassification::ARRAY_SCALAR.clone();
                    powTy = operator.ty.clone();
                } else {
                    powSizeClass = Operator::SizeClassification::SCALAR.clone();
                    powTy = openmodelica_nf_frontend::NFType::interned_REAL();
                }
                if !((arguments).is_empty()) && Type::isArray(&(Expression::typeOf((arguments).head().cloned()?))) {
                    sizeClass = Operator::SizeClassification::ELEMENT_WISE.clone();
                } else {
                    (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
                }
                addOp = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass), operator.ty.clone())?;
                powOp = Operator::fromClassification((Operator::MathClassification::POWER.clone(), powSizeClass), powTy)?;
                (diff_arguments, diffArguments) = differentiateMultaryMultiplicationArgs(metamodelica::AsArg::as_arg(&arguments), diffArguments, operator.clone())?;
                diff_enumerator = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: diff_arguments, inv_arguments: metamodelica::nil(), operator: addOp.clone() });
                (diff_inv_arguments, diffArguments) = differentiateMultaryMultiplicationArgs(metamodelica::AsArg::as_arg(&inv_arguments), diffArguments, operator.clone())?;
                diff_divisor = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: diff_inv_arguments, inv_arguments: metamodelica::nil(), operator: addOp.clone() });
                divisor = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: inv_arguments.clone(), inv_arguments: metamodelica::nil(), operator: operator.clone() });
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: metamodelica::cons(diff_enumerator, inv_arguments.clone()), inv_arguments: metamodelica::nil(), operator: operator.clone() })], inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: metamodelica::cons(diff_divisor, arguments.clone()), inv_arguments: metamodelica::nil(), operator: operator.clone() })], operator: addOp })], inv_arguments: list![metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: divisor, operator: powOp, exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(2.0_f64) }) })], operator: operator.clone() })
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDifferentiate.differentiateMultary")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok((exp, diffArguments))
}

pub(crate) fn differentiateMultaryMultiplicationArgs(
    mut arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
    mut operator: metamodelica::Ref<Operator::NFOperator>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut new_arguments: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArguments;
    let mut diff_arg: metamodelica::Ref<Expression::NFExpression>;
    let mut current_grad: metamodelica::Ref<Expression::NFExpression> = diffArguments.current_grad.clone();
    let mut localUp: metamodelica::Ref<Expression::NFExpression>;
    let mut restProd: metamodelica::Ref<Expression::NFExpression>;
    let mut diff_lists: metamodelica::Array<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    let mut arg_products: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut restArgs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut idx: i32 = 1;
    let mut isReverse: bool = (diffArguments.adjoint_map).is_some();
    let mut mulEWOp: metamodelica::Ref<Operator::NFOperator> = Operator::fromClassification(
        (
            Operator::MathClassification::MULTIPLICATION.clone(),
            Operator::SizeClassification::ELEMENT_WISE.clone(),
        ),
        operator.ty.clone(),
    )?;
    if isReverse {
        arg_products = Expression::productOfListExceptSelf(arguments, &(makeMulFromOperator(&operator)?))?;
    } else {
        diff_lists = arrayCreate(((arguments).len() as i32), metamodelica::nil());
    }
    for mut arg in &**arguments {
        if isReverse {
            current_grad = diffArguments.current_grad.clone();
            restProd = (arg_products).get(idx)?;
            restArgs = (match &*restProd {
                Expression::MULTARY {
                    operator: mOp,
                    arguments: rA,
                    ..
                } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&mOp))?
                    == Operator::MathClassification::MULTIPLICATION.clone()) =>
                {
                    rA.clone()
                }
                _ => {
                    list![restProd.clone()]
                }
            });
            localUp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                arguments: listAppend(list![current_grad.clone()], restArgs),
                inv_arguments: metamodelica::nil(),
                operator: mulEWOp.clone(),
            });
            if Expression::isScalar(arg.clone()) && Expression::hasArrayType(restProd) {
                localUp = typeSumCall(localUp)?;
            }
            assign_field!(diffArguments.current_grad = localUp);
        }
        (diff_arg, diffArguments) = differentiateExpression(arg.clone(), diffArguments)?;
        if isReverse {
            assign_field!(diffArguments.current_grad = current_grad.clone());
        } else {
            for mut i in 1..=metamodelica::arrayLength(diff_lists.clone()) {
                {
                    let __cell0 = if (i == idx) {
                        metamodelica::cons(
                            diff_arg.clone(),
                            ({
                                let __elt = (*metamodelica::index_checked(&diff_lists.borrow(), i)?).clone();
                                __elt
                            }),
                        )
                    } else {
                        metamodelica::cons(
                            arg.clone(),
                            ({
                                let __elt = (*metamodelica::index_checked(&diff_lists.borrow(), i)?).clone();
                                __elt
                            }),
                        )
                    };
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut diff_lists.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
        }
        idx = idx + 1;
    }
    if !(isReverse) {
        for mut i in ({
            let __s = metamodelica::arrayLength(diff_lists.clone());
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            new_arguments = metamodelica::cons(
                metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: ({
                        let __elt = (*metamodelica::index_checked(&diff_lists.borrow(), i)?).clone();
                        __elt
                    })
                    .reverse(),
                    inv_arguments: metamodelica::nil(),
                    operator: operator.clone(),
                }),
                new_arguments,
            );
        }
    }
    Ok((new_arguments, diffArguments))
}

pub(crate) fn differentiateEquationAttributes(
    mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
    mut diffArguments: &metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<metamodelica::Ref<EquationAttributes::EquationAttributes>> {
    let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes> = attr;
    attr = (::match_deref::match_deref! { match &((attr.clone(), diffArguments.clone())) {
        (Deref @ EquationAttributes::EQUATION_ATTRIBUTES { residualVar: Some(residualVar), .. }, Deref @ DifferentiationArguments::DIFFERENTIATION_ARGUMENTS { diff_map: Some(diff_map), diffType: DifferentiationType::JACOBIAN, .. }) if (UnorderedMap::contains(BVariable::getVarName(residualVar.clone()), diff_map.clone())?) => {
            let mut diffedResidualVar: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            diffedResidualVar = BVariable::getVarPointer(&(UnorderedMap::getOrFail(BVariable::getVarName(residualVar.clone()), diff_map.clone())?), metamodelica::sourceInfo!("NBackEnd/Util/NBDifferentiate.mo"))?;
            assign_field!(attr.residualVar = Some(diffedResidualVar));
            attr
        },
        _ => {
            attr
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(attr)
}

pub(crate) fn differentiateBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
) -> Result<(
    metamodelica::Ref<Binding::NFBinding>,
    metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>,
)> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> = diffArgs;
    let mut opt_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    opt_exp = Binding::getExpOpt(&binding);
    if (opt_exp).is_some() {
        (exp, diffArgs) = differentiateExpression(opt_exp.ok_or("pattern mismatch")?, diffArgs)?;
        binding = Binding::setExp(exp, binding)?;
    }
    Ok((binding, diffArgs))
}

fn sizeClassificationFromType(mut ty: metamodelica::Ref<Type::NFType>) -> Operator::SizeClassification {
    let mut sc: Operator::SizeClassification;
    sc = (match Type::dimensionCount(ty) {
        0 => Operator::SizeClassification::SCALAR.clone(),
        1 => Operator::SizeClassification::ELEMENT_WISE.clone(),
        2 => Operator::SizeClassification::MATRIX.clone(),
        _ => Operator::SizeClassification::ELEMENT_WISE.clone(),
    });
    sc
}

fn minusOne(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut op: metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::REAL { value: r } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: r.clone() - metamodelica::OrderedFloat(1.0_f64),
        }),
        Expression::INTEGER { value: i } => {
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i.clone() - 1 })
        }
        _ => metamodelica::Ref::new(Expression::NFExpression::MULTARY {
            arguments: list![exp],
            inv_arguments: list![Expression::makeOne(&op.ty)?],
            operator: op,
        }),
    });
    Ok(exp)
}

fn expLog(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::REAL { value: r } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (r.clone()).ln(),
        }),
        Expression::INTEGER { value: i } => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: (metamodelica::OrderedFloat((i.clone()) as f64)).ln(),
        }),
        _ => metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                BuiltinFuncs::LOG_REAL().clone(),
                list![exp.clone()],
                Expression::variability(exp)?,
                Prefixes::Purity::PURE.clone(),
                BuiltinFuncs::LOG_REAL().returnType.clone(),
            ),
        }),
    });
    Ok(exp)
}

fn makeMulFromOperator(
    mut operator: &metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<Operator::NFOperator>> {
    let mut mulOp: metamodelica::Ref<Operator::NFOperator>;
    mulOp = Operator::fromClassification(
        (
            Operator::MathClassification::MULTIPLICATION.clone(),
            Operator::getSizeClassification(operator)?,
        ),
        operator.ty.clone(),
    )?;
    Ok(mulOp)
}

fn typeTransposeCall(
    mut mat: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut tr: metamodelica::Ref<Expression::NFExpression>;
    let mut inTy: metamodelica::Ref<Type::NFType> = Expression::typeOf(mat.clone());
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut elTy: metamodelica::Ref<Type::NFType>;
    let mut resTy: metamodelica::Ref<Type::NFType>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut var: Variability = Expression::variability(mat.clone())?;
    let mut pur: Prefixes::Purity = Expression::purity(mat.clone())?;
    if !(Type::isArray(&inTy)) {
        tr = mat;
        return Ok(tr);
    }
    elTy = Type::arrayElementType(&inTy);
    dims = Type::arrayDims(inTy);
    if ((dims).len() as i32) < 2 {
        tr = mat;
        return Ok(tr);
    }
    resTy = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: elTy,
        dimensions: listAppend(list![(dims).get(2)?, (dims).get(1)?], ((dims).rest()?).rest()?),
    });
    call = Call::makeTypedCall(BuiltinFuncs::TRANSPOSE().clone(), list![mat], var, pur, resTy);
    tr = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    Ok(tr)
}

// Helper: build a typed builtin promote(A, n) call that appends (n - ndims(A)) singleton dims.
fn typePromoteCall(
    mut arr: metamodelica::Ref<Expression::NFExpression>,
    mut n: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut promoted: metamodelica::Ref<Expression::NFExpression>;
    let mut inTy: metamodelica::Ref<Type::NFType> = Expression::typeOf(arr.clone());
    let mut elTy: metamodelica::Ref<Type::NFType>;
    let mut inDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut m: i32;
    let mut k: i32;
    let mut ones: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut resDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut resTy: metamodelica::Ref<Type::NFType>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut var: Variability = Expression::variability(arr.clone())?;
    let mut pur: Prefixes::Purity = Expression::purity(arr.clone())?;
    elTy = if (Type::isArray(&inTy)) {
        Type::arrayElementType(&inTy)
    } else {
        inTy.clone()
    };
    inDims = if (Type::isArray(&inTy)) {
        Type::arrayDims(inTy)
    } else {
        metamodelica::nil()
    };
    m = ((inDims).len() as i32);
    for mut k in 1..=std::cmp::max(0, n - m) {
        ones = metamodelica::cons(Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone()), ones);
    }
    resDims = List::append_reverse(&ones, inDims);
    resTy = if (n > 0) {
        metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: elTy,
            dimensions: resDims,
        })
    } else {
        elTy
    };
    call = Call::makeTypedCall(
        BuiltinFuncs::PROMOTE().clone(),
        list![
            arr,
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: n })
        ],
        var,
        pur,
        resTy,
    );
    promoted = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    Ok(promoted)
}

fn typeSumCall(
    mut arr: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut s: metamodelica::Ref<Expression::NFExpression>;
    let mut inTy: metamodelica::Ref<Type::NFType> = Expression::typeOf(arr.clone());
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut elTy: metamodelica::Ref<Type::NFType>;
    let mut resTy: metamodelica::Ref<Type::NFType>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut var: Variability = Expression::variability(arr.clone())?;
    let mut pur: Prefixes::Purity = Expression::purity(arr.clone())?;
    if !(Type::isArray(&inTy)) {
        s = arr;
        return Ok(s);
    }
    elTy = Type::arrayElementType(&inTy);
    dims = Type::arrayDims(inTy);
    resTy = elTy;
    call = Call::makeTypedCall(BuiltinFuncs::SUM().clone(), list![arr], var, pur, resTy);
    s = metamodelica::Ref::new(Expression::NFExpression::CALL { call: call });
    Ok(s)
}

// Helper: build matrix * vector (or matrix * matrix) MULTARY with a proper mul operator
fn makeMul(
    mut a: metamodelica::Ref<Expression::NFExpression>,
    mut b: metamodelica::Ref<Expression::NFExpression>,
    mut sc: Operator::SizeClassification,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    res = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: a,
        operator: Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sc), ty)?,
        exp2: b,
    });
    Ok(res)
}

// Drop the last array dimension by indexing it with 1:
// arr[..., 1]. If arr is not an array, return it unchanged.
fn dropLastDimIndex1(
    mut arr: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = Expression::typeOf(arr.clone());
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut m: i32;
    let mut i: i32;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
    if !(Type::isArray(&ty)) {
        res = arr;
        return Ok(res);
    }
    dims = Type::arrayDims(ty);
    m = ((dims).len() as i32);
    if m <= 0 {
        res = arr;
        return Ok(res);
    }
    for mut i in 1..=m - 1 {
        subs = metamodelica::cons(openmodelica_nf_frontend::NFSubscript::interned_WHOLE(), subs);
    }
    subs = metamodelica::cons(
        metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
        }),
        subs,
    );
    subs = subs.reverse();
    res = Expression::applySubscripts(&subs, arr, true)?;
    Ok(res)
}

// Build vector[n] with elements A[i,i], i=1..n (literal array).
fn extractDiagonalVector(
    mut A: metamodelica::Ref<Expression::NFExpression>,
    mut n: i32,
    mut vecTy: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut v: metamodelica::Ref<Expression::NFExpression>;
    let mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut i: i32 = 0;
    for mut i in 1..=n {
        elems = metamodelica::cons(
            Expression::applySubscripts(
                &(list![
                    metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                        index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                    }),
                    metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                        index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                    })
                ]),
                A.clone(),
                true,
            )?,
            elems,
        );
    }
    v = metamodelica::Ref::new(Expression::NFExpression::ARRAY {
        ty: vecTy,
        elements: metamodelica::arrayFromVec(elems.reverse().into_iter().cloned().collect()),
        literal: false,
    });
    Ok(v)
}

fn dbg(mut s: &ArcStr) -> Result<()> {
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*s);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn expressionHasIteratorCref(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    pub(crate) fn foldIter(mut e: &metamodelica::Ref<Expression::NFExpression>, mut b: bool) -> bool {
        let mut b: bool = b;
        b = (match &**e {
            Expression::CREF { cref: __e_cref, .. } => {
                b || ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__e_cref))
            }
            _ => b,
        });
        b
    }

    let mut hasIter: bool;
    hasIter = Expression::fold(
        exp,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: bool| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(foldIter(&__a0, __a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
            >),
        false,
    )?;
    Ok(hasIter)
}

fn subscriptHasIterator(mut sub: &metamodelica::Ref<Subscript::NFSubscript>) -> Result<bool> {
    let mut hasIter: bool;
    hasIter = (match &**sub {
        Subscript::INDEX { index: __sub_index } => expressionHasIteratorCref(__sub_index.clone())?,
        Subscript::SLICE { slice: __sub_slice } => expressionHasIteratorCref(__sub_slice.clone())?,
        _ => false,
    });
    Ok(hasIter)
}

fn subscriptsHaveIterator(mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>) -> Result<bool> {
    let mut hasIter: bool = false;
    for mut sub in &**subs {
        if subscriptHasIterator(metamodelica::AsArg::as_arg(&sub))? {
            hasIter = true;
            break;
        }
    }
    Ok(hasIter)
}

fn updateAdjointList(
    mut oldOpt: Option<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>>,
    mut current_grad: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::List<metamodelica::Ref<Expression::NFExpression>> {
    let mut newList: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut oldList: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    newList = (::match_deref::match_deref! { match &(oldOpt) {
        Some(__esc_oldList) => {
            oldList = (*__esc_oldList).clone();
            metamodelica::cons(current_grad, oldList.clone())
        },
        _ => list![current_grad],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    newList
}
