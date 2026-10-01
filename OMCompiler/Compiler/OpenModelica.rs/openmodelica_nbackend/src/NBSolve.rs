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

use crate::NBAdjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBInline as Inline;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpandExp as ExpandExp;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// backend imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Status {
    UNPROCESSED = 1,
    EXPLICIT = 2,
    IMPLICIT = 3,
    UNSOLVABLE = 4,
}
impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Status {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

// TRUE -> relation must be inverted, FALSE -> relation must not be inverted, UNKNOWN -> TODO: make relation depend on derivative of the expr
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum RelationInversion {
    TRUE = 1,
    FALSE = 2,
    UNKNOWN = 3,
}
impl PartialOrd for RelationInversion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RelationInversion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for RelationInversion {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn statusString(mut status: Status) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match status {
        Status::UNPROCESSED => literal!("Solve.UNPROCESSED"),
        Status::EXPLICIT => literal!("Solve.EXPLICIT"),
        Status::IMPLICIT => literal!("Solve.IMPLICIT"),
        Status::UNSOLVABLE => literal!("Solve.UNSOLVABLE"),
        _ => literal!("Solve.FAILED"),
    });
    r#str
}

pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    pub(crate) type StrongComponentLst = metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;

    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut implicit_index_ptr: Pointer::Pointer<i32> = Pointer::create(1);
    let mut duplicate_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<StrongComponent::NBStrongComponent>,
            metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
        >,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<StrongComponent::NBStrongComponent>| {
            StrongComponent::hash(&__a0)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<StrongComponent::NBStrongComponent>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<StrongComponent::NBStrongComponent>,
                  __a1: metamodelica::Ref<StrongComponent::NBStrongComponent>| {
                StrongComponent::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<StrongComponent::NBStrongComponent>,
                        metamodelica::Ref<StrongComponent::NBStrongComponent>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    bdae = (match &*bdae {
        BackendDAE::MAIN {
            eqData: __bdae_eqData,
            funcMap: __bdae_funcMap,
            init: __bdae_init,
            varData: __bdae_varData,
            ..
        } => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (__bdae_init.clone()).into_iter().cloned() {
                    let __x = solvePartition(par.clone(), __bdae_funcMap.clone(), implicit_index_ptr.clone(), duplicate_map.clone(), metamodelica::AsArg::as_arg(&__bdae_varData), metamodelica::AsArg::as_arg(&__bdae_eqData))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            if (var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init_0 = Some(({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (Util::getOption(var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN).clone())?).into_iter().cloned() {
                        let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })));
            }
            if (var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; dae = Some(({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (Util::getOption(var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN).clone())?).into_iter().cloned() {
                        let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })));
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                        ode = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        algebraic = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).algebraic, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ode_event = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).ode_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        alg_event = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).alg_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        clocked = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).clocked, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = solvePartition(par.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), implicit_index_ptr.clone(), duplicate_map.clone(), &(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone()), &(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.main"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn solvePartition(
    mut partition: metamodelica::Ref<Partition::Partition>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut implicit_index_ptr: Pointer::Pointer<i32>,
    mut duplicate_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<StrongComponent::NBStrongComponent>,
            metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<metamodelica::Ref<Partition::Partition>> {
    pub(crate) type EquationPointerList = metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;

    let mut partition: metamodelica::Ref<Partition::Partition> = partition;
    let mut kind: BPartition::Kind = BPartition::Partition::getKind(&partition);
    let mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
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
    let mut solved_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut implicit_index: i32 = Pointer::access(implicit_index_ptr.clone());
    let mut sliced_idx: Pointer::Pointer<i32>;
    let mut comp_idx: Pointer::Pointer<i32> = Pointer::create(1);
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut sliced_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    if (partition.strongComponents).is_some() {
        let __range0 = Util::getOption(partition.strongComponents.clone())?
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut comp in __range0 {
            solved_comps = (::match_deref::match_deref! { match &(UnorderedMap::get(comp.clone(), duplicate_map.clone())?) {
                Some(alias_comps) => {
                    listAppend(alias_comps.clone(), solved_comps)
                },
                _ => {
                    let mut alias_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                    (alias_comps, implicit_index) = solveStrongComponent(comp.clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData)?;
                    UnorderedMap::add(comp, ({
                let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> = metamodelica::nil();
                for mut c in (alias_comps.clone()).into_iter().cloned() {
                    let __x = StrongComponent::createAlias(kind, partition.index.clone(), comp_idx.clone(), c.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), duplicate_map.clone())?;
                    listAppend(alias_comps.clone(), solved_comps)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        assign_field!(
            partition.strongComponents = Some(metamodelica::arrayFromVec(
                solved_comps.reverse().into_iter().cloned().collect()
            ))
        );
        for mut tpl in &*UnorderedMap::toList(slicing_map) {
            (name, sliced_eqns) = tpl.clone();
            if !((sliced_eqns).is_empty()) {
                sliced_idx = Pointer::create(1);
                for mut eqn_ptr in &*sliced_eqns {
                    Equation::subIdxName(eqn_ptr.clone(), sliced_idx.clone())?;
                }
            }
        }
        Pointer::update(implicit_index_ptr, implicit_index);
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBSolve.solvePartition"));
                __mm_s.push_str(&*literal!(" cannot solve partition without strong components: "));
                __mm_s.push_str(&*BPartition::Partition::toString(&partition, 0)?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(partition)
}

pub(crate) fn solveStrongComponent(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    i32,
)> {
    let mut solved_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut implicit_index: i32 = implicit_index;
    let mut solve_status: Status;
    let mut implicit_comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    match '__try0: {
        (solved_comps, solve_status) = ({
            let mut entwined_slices: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            let mut inner_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            let mut failed_inner: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            (::match_deref::match_deref! { match &(comp.clone()) {
                Deref @ StrongComponent::SINGLE_COMPONENT { eqn: __comp_eqn, var: __comp_var, .. } => {
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    (eqn, solve_status, implicit_index) = unwrap_break_err!(solveSingleStrongComponent(Pointer::access(__comp_eqn.clone()), &(Pointer::access(__comp_var.clone())), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                    (list![metamodelica::Ref::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: __comp_var.clone(), eqn: Pointer::create(eqn.clone()), status: solve_status })], solve_status)
                },
                Deref @ StrongComponent::MULTI_COMPONENT { vars: Deref @ metamodelica::ListNode::Cons { head: var_slice, tail: Deref @ metamodelica::ListNode::Nil }, eqn: __comp_eqn, .. } if (!(Equation::isCompound(Slice::getT(__comp_eqn.clone())))) => {
                    let mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    var_cref = if (Slice::isFull(var_slice.clone())) {BVariable::getVarName(Slice::getT(var_slice.clone()))} else {unwrap_break_err!(Slice::resolveSlicedCref(BVariable::getVarName(Slice::getT(var_slice.clone())), Pointer::access(Slice::getT(__comp_eqn.clone())), unwrap_break_err!(Slice::size(var_slice.clone(), &({ let __pe_b1 = false; move |__pe_a0| BVariable::size(__pe_a0, __pe_b1.clone()) })), '__try0)), '__try0)};
                    (solved_comps, implicit_index) = unwrap_break_err!(solveStrongComponent(unwrap_break_err!(StrongComponent::createSliceOrSingle(var_cref.clone(), var_slice.clone(), __comp_eqn.clone()), '__try0), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                    (solved_comps.clone(), Status::UNPROCESSED.clone())
                },
                Deref @ StrongComponent::MULTI_COMPONENT { eqn: __comp_eqn, .. } => {
                    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
                    let mut solved_comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
                    let mut strict: metamodelica::Ref<Tearing::NBTearing>;
                    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
                    let mut solved_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut outputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut output_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                    let mut input_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                    let mut solved_inputs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                    let mut tmp_crefs: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>)>;
                    let mut tmp_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                    let mut is_mixed: bool;
                    let mut tmp_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                    let mut idx: Pointer::Pointer<i32>;
                    let mut cref_repl: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                    let mut exp_repl: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
                    eqn_ptr = Slice::getT(__comp_eqn.clone());
                    eqn = Pointer::access(eqn_ptr.clone());
                    (solved_comp, solve_status) = (match &*eqn {
                Equation::ALGORITHM { alg: __esc_alg, .. } => {
                    alg = (*__esc_alg).clone();
                    solved_crefs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
                for mut var in (var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone()).into_iter().cloned() {
                    let __x = BVariable::getVarName(Slice::getT(var.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    inputs = unwrap_break_err!(List::flatten(({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                for mut i in (alg.inputs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(BVariable::getRecordChildrenCrefOrSelf(i.clone()), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), '__try0);
                    input_crefs = unwrap_break_err!(UnorderedSet::fromList(&inputs, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>)), '__try0);
                    solved_inputs = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
                    for mut solved_cref in &*solved_crefs {
                        if unwrap_break_err!(UnorderedSet::contains(solved_cref.clone(), input_crefs.clone()), '__try0) {
                            unwrap_break_err!(UnorderedSet::add(solved_cref.clone(), solved_inputs.clone()), '__try0);
                        }
                    }
                    outputs = unwrap_break_err!(List::flatten(({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                for mut o in (alg.outputs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(BVariable::getRecordChildrenCrefOrSelf(o.clone()), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), '__try0);
                    output_crefs = unwrap_break_err!(UnorderedSet::fromList(&outputs, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>)), '__try0);
                    for mut solved_cref in &*solved_crefs {
                        unwrap_break_err!(UnorderedSet::remove(solved_cref.clone(), output_crefs.clone()), '__try0);
                    }
                    if UnorderedSet::isEmpty(output_crefs.clone()) {
                        (eqn_slice, solve_status, implicit_index) = unwrap_break_err!(solveMultiStrongComponent(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), &(crate::NBEquation::Iterator::interned_EMPTY()), varData, eqData), '__try0);
                        solved_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT { vars: var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), eqn: eqn_slice.clone(), status: solve_status });
                    } else {
                        solve_status = Status::IMPLICIT.clone();
                        idx = Pointer::create(0);
                        tmp_crefs = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>)> = metamodelica::nil();
                for mut cref in (UnorderedSet::toList(output_crefs.clone())).into_iter().cloned() {
                    let __x = (cref.clone(), unwrap_break_err!(BVariable::makeTmpVar(cref.clone()), '__try0));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                        tmp_vars = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
                for mut tpl in (tmp_crefs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(BVariable::getVarPointer(&(Util::tuple22(tpl.clone())), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo")), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                        tmp_eqns = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
                for mut tpl in (tmp_crefs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(Equation::makeAssignment(unwrap_break_err!(Expression::fromCref(Util::tuple21(tpl.clone()), false), '__try0), unwrap_break_err!(Expression::fromCref(Util::tuple22(tpl.clone()), false), '__try0), idx.clone(), &(literal!("TMP")), crate::NBEquation::Iterator::interned_EMPTY(), NBEquation::default(NBEquation::EquationKind::CONTINUOUS.clone(), false, None, None)), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                        tmp_eqns = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
                for mut e in (tmp_eqns.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(Equation::createResidual(e.clone(), None, false, false), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                        cref_repl = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                        exp_repl = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                        for mut tpl in &*tmp_crefs {
                            unwrap_break_err!(UnorderedMap::add(Util::tuple21(tpl.clone()), Util::tuple22(tpl.clone()), cref_repl.clone()), '__try0);
                            unwrap_break_err!(UnorderedMap::add(Util::tuple21(tpl.clone()), unwrap_break_err!(Expression::fromCref(Util::tuple22(tpl.clone()), false), '__try0), exp_repl.clone()), '__try0);
                        }
                        assign_field!(alg.outputs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
                for mut c in (var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(UnorderedMap::getOrDefault(c.clone(), cref_repl.clone(), c.clone()), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
                        assign_variant_field!(comp => StrongComponent::NBStrongComponent::MULTI_COMPONENT;
                            vars = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>> = metamodelica::nil();
                for mut c in (alg.outputs.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Slice::NBSlice { t: unwrap_break_err!(BVariable::getVarPointer(&(c.clone()), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo")), '__try0), indices: metamodelica::nil() });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                            status = Status::EXPLICIT.clone()
                        );
                        assign_variant_field!(eqn => Equation::Equation::ALGORITHM; alg = alg.clone());
                        Pointer::update(eqn_ptr.clone(), unwrap_break_err!(Equation::map(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = exp_repl.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>)), '__try0));
                        strict = metamodelica::Ref::new(Tearing::NBTearing { iteration_vars: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>> = metamodelica::nil();
                for mut c in (UnorderedSet::toList(solved_inputs.clone())).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Slice::NBSlice { t: unwrap_break_err!(BVariable::getVarPointer(&(c.clone()), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo")), '__try0), indices: metamodelica::nil() });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), residual_eqns: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>> = metamodelica::nil();
                for mut e in (tmp_eqns.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Slice::NBSlice { t: e.clone(), indices: metamodelica::nil() });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), innerEquations: metamodelica::arrayFromVec(list![comp.clone()].into_iter().cloned().collect()), jac: None });
                        is_mixed = unwrap_break_err!(List::any(&(({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
                for mut v in (strict.iteration_vars.clone()).into_iter().cloned() {
                    let __x = Slice::getT(v.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), &({ let __pe_b1 = BPartition::kindIsInitial(kind); move |__pe_a0| BVariable::isDiscontinuous(__pe_a0, __pe_b1.clone()) })), '__try0);
                        solved_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: implicit_index, strict: strict.clone(), casual: None, linear: false, mixed: is_mixed, homotopy: false, status: solve_status, implicitlyCreated: true });
                        unwrap_break_err!(EqData::addTypedList(eqData.clone(), &tmp_eqns, EqData::EqType::CONTINUOUS.clone(), true), '__try0);
                        unwrap_break_err!(BVariable::VarData::addTypedList(varData.clone(), &tmp_vars, BVariable::VarData::VarType::ALGEBRAIC.clone()), '__try0);
                    }
                    (solved_comp.clone(), solve_status)
                },
                _ => {
                    (eqn_slice, solve_status, implicit_index) = unwrap_break_err!(solveMultiStrongComponent(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), &(crate::NBEquation::Iterator::interned_EMPTY()), varData, eqData), '__try0);
                    (metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT { vars: var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone(), eqn: eqn_slice.clone(), status: solve_status }), solve_status)
                },
            });
                    (list![solved_comp.clone()], solve_status)
                },
                Deref @ StrongComponent::ALGEBRAIC_LOOP { strict, .. } => {
                    let mut tmp: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                    let mut err_str: ArcStr;
                    let mut strict = (*strict).clone();
                    for mut index in ({let __s=metamodelica::arrayLength(strict.innerEquations.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                        (tmp, implicit_index) = unwrap_break_err!(solveStrongComponent(({let __elt = (*unwrap_break_err!(metamodelica::index_checked(&strict.innerEquations.borrow(), index), '__try0)).clone(); __elt}), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                        inner_comps = listAppend(tmp.clone(), inner_comps.clone());
                        for mut elem in &*tmp {
                            if unwrap_break_err!(StrongComponent::getSolveStatus(metamodelica::AsArg::as_arg(&elem)), '__try0) != Status::EXPLICIT.clone() {
                                failed_inner = metamodelica::cons(elem.clone(), failed_inner.clone());
                            }
                        }
                    }
                    if !((failed_inner).is_empty()) {
                        if unwrap_break_err!(Flags::isSet(Flags::TEARING_DUMP.clone()), '__try0) {
                            err_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" Following inner equations could not be solved explicitely:\n")); __mm_s.push_str(&*unwrap_break_err!(List::toString(failed_inner.clone(), &({ let __pe_b1 = -1; move |__pe_a0| StrongComponent::toString(&__pe_a0, __pe_b1.clone()) }), List::Style::NEWLINE.clone()), '__try0)); ArcStr::from(__mm_s) };
                        } else {
                            err_str = literal!(" Use -d=tearingdump for more information.");
                        }
                        unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveStrongComponent")); __mm_s.push_str(&*literal!(" failed. ")); __mm_s.push_str(&*err_str); ArcStr::from(__mm_s) }]), '__try0);
                        break '__try0 Err::<_, _>("fail");
                    }
                    assign_field!(strict.innerEquations = metamodelica::arrayFromVec(inner_comps.clone().into_iter().cloned().collect()));
                    assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP;
                        strict = strict.clone(),
                        status = Status::IMPLICIT.clone()
                    );
                    (list![comp.clone()], Status::IMPLICIT.clone())
                },
                Deref @ StrongComponent::SLICED_COMPONENT { eqn: __comp_eqn, .. } if (Equation::isForEquation(Slice::getT(__comp_eqn.clone()))) => {
                    let mut generic_comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
                    (generic_comp, solve_status, implicit_index) = unwrap_break_err!(solveGenericEquation(comp.clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                    (list![generic_comp.clone()], solve_status)
                },
                Deref @ StrongComponent::SLICED_COMPONENT { eqn: __comp_eqn, var: __comp_var, var_cref: __comp_var_cref, .. } if (Equation::isArrayEquation(Slice::getT(__comp_eqn.clone()))) => {
                    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
                    (eqn_slice, implicit_index, solve_status) = unwrap_break_err!(solveForVarSlice(__comp_eqn.clone(), __comp_var.clone(), __comp_var_cref.clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                    assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT;
                        eqn = eqn_slice.clone(),
                        status = solve_status
                    );
                    (list![comp.clone()], solve_status)
                },
                Deref @ StrongComponent::SLICED_COMPONENT { eqn: __comp_eqn, var_cref: __comp_var_cref, .. } => {
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
                    (eqn, solve_status, implicit_index) = unwrap_break_err!(solveSingleStrongComponent(Pointer::access(Slice::getT(__comp_eqn.clone())), &(unwrap_break_err!(Variable::fromCref(__comp_var_cref.clone()), '__try0)), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                    if solve_status == Status::EXPLICIT.clone() {
                        assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; eqn = metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: metamodelica::nil() }));
                    } else {
                        (eqn_slice, implicit_index, solve_status) = unwrap_break_err!(solveForVarSlice(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), var_field!((*comp).var, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), var_field!((*comp).var_cref, StrongComponent::NBStrongComponent::SLICED_COMPONENT).clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                        assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; eqn = eqn_slice.clone());
                    }
                    assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; status = solve_status);
                    (list![comp.clone()], solve_status)
                },
                Deref @ StrongComponent::RESIZABLE_COMPONENT { eqn: __comp_eqn, order: __comp_order, var_cref: __comp_var_cref, .. } => {
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    (eqn, solve_status, implicit_index, _) = unwrap_break_err!(solveEquation(Pointer::access(Slice::getT(__comp_eqn.clone())), __comp_var_cref.clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                    eqn = unwrap_break_err!(Equation::applyForOrder(eqn.clone(), __comp_order.clone()), '__try0);
                    assign_variant_field!(comp => StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT;
                        eqn = metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(eqn.clone()), indices: __comp_eqn.indices.clone() }),
                        status = solve_status
                    );
                    (list![comp.clone()], solve_status)
                },
                Deref @ StrongComponent::ENTWINED_COMPONENT { entwined_slices: __comp_entwined_slices, .. } => {
                    let mut generic_comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
                    for mut slice in &*__comp_entwined_slices.clone() {
                        (generic_comp, solve_status, implicit_index) = unwrap_break_err!(solveGenericEquation(slice.clone(), funcMap.clone(), kind, implicit_index, slicing_map.clone(), varData, eqData), '__try0);
                        entwined_slices = metamodelica::cons(generic_comp.clone(), entwined_slices.clone());
                    }
                    assign_variant_field!(comp => StrongComponent::NBStrongComponent::ENTWINED_COMPONENT; entwined_slices = entwined_slices.clone().reverse());
                    (list![comp.clone()], Status::EXPLICIT.clone())
                },
                _ => {
                    (list![comp.clone()], Status::UNSOLVABLE.clone())
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        Ok::<_, &'static str>((solve_status.clone(), solved_comps.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            solve_status = __try0_o0;
            solved_comps = __try0_o1;
        }
        Err(_) => {
            (solved_comps, solve_status) = (list![comp.clone()], Status::UNSOLVABLE.clone());
        }
    }
    if solve_status == Status::IMPLICIT.clone() && List::hasOneElement(&solved_comps) {
        (implicit_comp, implicit_index) =
            Tearing::implicit((solved_comps).head().cloned()?, funcMap, implicit_index, kind)?;
        solved_comps = list![implicit_comp];
    } else if solve_status > Status::EXPLICIT.clone() {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBSolve.solveStrongComponent"));
                __mm_s.push_str(&*literal!(" failed with status = "));
                __mm_s.push_str(&*statusString(solve_status));
                __mm_s.push_str(&*literal!(" while trying to solve following strong component:\n"));
                __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok((solved_comps, implicit_index))
}

pub(crate) fn solveGenericEquation(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, Status, i32)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut solve_status: Status;
    let mut implicit_index: i32 = implicit_index;
    (comp, solve_status) = (match &*comp {
        StrongComponent::SLICED_COMPONENT {
            var: var_slice,
            eqn: eqn_slice,
            var_cref: __comp_var_cref,
            ..
        } if (Equation::isForEquation(Slice::getT(eqn_slice.clone()))) => {
            (comp, solve_status, implicit_index) = solveGenericEquationSlice(
                var_slice.clone(),
                eqn_slice.clone(),
                __comp_var_cref.clone(),
                funcMap,
                kind,
                implicit_index,
                slicing_map,
                varData,
                eqData,
            )?;
            (comp, solve_status)
        }
        StrongComponent::RESIZABLE_COMPONENT {
            var: var_slice,
            eqn: eqn_slice,
            order: __comp_order,
            var_cref: __comp_var_cref,
            ..
        } if (Equation::isForEquation(Slice::getT(eqn_slice.clone()))) => {
            let mut eqn_slice = (*eqn_slice).clone();
            eqn_slice = Slice::apply(
                eqn_slice.clone(),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static> = (std::sync::Arc::new({
                        let __pe_b1 = __comp_order.clone();
                        move |__pe_a0| Equation::applyForOrder(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Equation::Equation>,
                                )
                                    -> Result<metamodelica::Ref<Equation::Equation>>
                                + 'static,
                        >);
                    move |__pe_a0| Pointer::apply(__pe_a0, __pe_b1.clone())
                }),
            )?;
            (comp, solve_status, implicit_index) = solveGenericEquationSlice(
                var_slice.clone(),
                eqn_slice.clone(),
                __comp_var_cref.clone(),
                funcMap,
                kind,
                implicit_index,
                slicing_map,
                varData,
                eqData,
            )?;
            (comp, solve_status)
        }
        StrongComponent::SINGLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            let mut eqn: metamodelica::Ref<Equation::Equation>;
            (eqn, solve_status, implicit_index) = solveSingleStrongComponent(
                Pointer::access(__comp_eqn.clone()),
                &(Pointer::access(__comp_var.clone())),
                funcMap,
                kind,
                implicit_index,
                slicing_map,
                varData,
                eqData,
            )?;
            comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT {
                var: __comp_var.clone(),
                eqn: Pointer::create(eqn),
                status: solve_status,
            });
            (comp, solve_status)
        }
        StrongComponent::SLICED_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } => {
            let mut eqn: metamodelica::Ref<Equation::Equation>;
            (eqn, solve_status, implicit_index) = solveSingleStrongComponent(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                &(Variable::fromCref(__comp_var_cref.clone())?),
                funcMap,
                kind,
                implicit_index,
                slicing_map,
                varData,
                eqData,
            )?;
            if solve_status < Status::UNSOLVABLE.clone() {
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; eqn = metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(eqn), indices: var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SLICED_COMPONENT).indices.clone() }));
            }
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::SLICED_COMPONENT; status = solve_status);
            (comp, solve_status)
        }
        StrongComponent::MULTI_COMPONENT {
            eqn: __comp_eqn,
            vars: __comp_vars,
            ..
        } => {
            let mut eqn_slice: metamodelica::Ref<
                Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
            >;
            (eqn_slice, solve_status, implicit_index) = solveMultiStrongComponent(
                __comp_eqn.clone(),
                __comp_vars.clone(),
                funcMap,
                kind,
                implicit_index,
                slicing_map,
                &(crate::NBEquation::Iterator::interned_EMPTY()),
                varData,
                eqData,
            )?;
            comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                vars: __comp_vars.clone(),
                eqn: eqn_slice.clone(),
                status: solve_status,
            });
            (comp, solve_status)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.solveGenericEquation"));
                    __mm_s.push_str(&*literal!(" failed for:\n"));
                    __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((comp, solve_status, implicit_index))
}

pub(crate) fn solveGenericEquationSlice(
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut functions: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, Status, i32)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut solve_status: Status;
    let mut implicit_index: i32 = implicit_index;
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = Slice::getT(eqn_slice.clone());
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    let mut solved_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    if List::hasOneElement(&eqn_slice.indices) {
        replacements = UnorderedMap::new(
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
        (eqn, solve_status) = Equation::singleSlice(
            eqn_ptr.clone(),
            (eqn_slice.indices).head().cloned()?,
            Equation::sizes(eqn_ptr, false)?,
            cref.clone(),
            replacements,
            functions.clone(),
        )?;
    } else {
        (eqn, solve_status, implicit_index, _) = solveEquation(
            Pointer::access(eqn_ptr),
            cref.clone(),
            functions.clone(),
            kind,
            implicit_index,
            slicing_map.clone(),
            varData,
            eqData,
        )?;
    }
    if solve_status < Status::UNSOLVABLE.clone() {
        solved_slice = metamodelica::Ref::new(Slice::NBSlice {
            t: Pointer::create(eqn),
            indices: eqn_slice.indices.clone(),
        });
    } else {
        (solved_slice, implicit_index, solve_status) = solveForVarSlice(
            eqn_slice,
            var_slice.clone(),
            cref.clone(),
            functions,
            kind,
            implicit_index,
            slicing_map,
            varData,
            eqData,
        )?;
    }
    if Equation::isForEquation(Slice::getT(solved_slice.clone())) {
        comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::GENERIC_COMPONENT {
            var_cref: cref,
            var: var_slice,
            eqn: solved_slice,
        });
    } else {
        comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::SLICED_COMPONENT {
            var_cref: cref,
            var: var_slice,
            eqn: solved_slice,
            status: solve_status,
        });
    }
    Ok((comp, solve_status, implicit_index))
}

pub(crate) fn solveSingleStrongComponent(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status, i32)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status;
    let mut implicit_index: i32 = implicit_index;
    let mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if ComponentRef::isEmpty(&var.name) {
        (eqn, status) = (eqn, Status::EXPLICIT.clone());
    } else {
        (var_cref, status) = getVarSlice(var.name.clone(), Some(var.name.clone()), eqn.clone())?;
        var_cref = if (status < Status::UNSOLVABLE.clone()) {
            var_cref
        } else {
            var.name.clone()
        };
        (eqn, status, implicit_index, _) = solveEquation(
            eqn,
            var_cref,
            funcMap,
            kind,
            implicit_index,
            slicing_map,
            varData,
            eqData,
        )?;
    }
    Ok((eqn, status, implicit_index))
}

pub(crate) fn solveMultiStrongComponent(
    mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut var_slices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(
    metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    Status,
    i32,
)> {
    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        eqn_slice;
    let mut status: Status;
    let mut implicit_index: i32 = implicit_index;
    let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(Slice::getT(eqn_slice.clone()));
    (eqn_slice, status) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::IF_EQUATION { body: __eqn_body, .. } => {
            let mut if_body: metamodelica::Ref<IfEquationBody::IfEquationBody>;
            (if_body, status, implicit_index) = solveIfBody(__eqn_body.clone(), &(BVariable::VariablePointers::fromList(&(({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (var_slices).into_iter().cloned() {
            let __x = Slice::getT(v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), false)?), funcMap, kind, implicit_index, slicing_map, iter, varData, eqData)?;
            if status == Status::EXPLICIT.clone() {
                assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = if_body);
                eqn_slice = metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(eqn), indices: eqn_slice.indices.clone() });
            }
            (eqn_slice, status)
        },
        Deref @ Equation::ALGORITHM { .. } => {
            (metamodelica::Ref::new(Slice::NBSlice { t: Pointer::clone(Slice::getT(eqn_slice.clone())), indices: eqn_slice.indices.clone() }), Status::EXPLICIT.clone())
        },
        Deref @ Equation::WHEN_EQUATION { .. } => {
            (metamodelica::Ref::new(Slice::NBSlice { t: Pointer::clone(Slice::getT(eqn_slice.clone())), indices: eqn_slice.indices.clone() }), Status::EXPLICIT.clone())
        },
        Deref @ Equation::RECORD_EQUATION { .. } => {
            let mut solved_eqn: metamodelica::Ref<Equation::Equation>;
            (solved_eqn, status) = solveMultiRecordStrongComponent(eqn, var_slices, funcMap, false)?;
            (metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(solved_eqn), indices: eqn_slice.indices.clone() }), status)
        },
        Deref @ Equation::ARRAY_EQUATION { .. } => {
            let mut solved_eqn: metamodelica::Ref<Equation::Equation>;
            (solved_eqn, status) = solveMultiRecordStrongComponent(eqn, var_slices, funcMap, false)?;
            (metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(solved_eqn), indices: eqn_slice.indices.clone() }), status)
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: solved_eqn @ Deref @ Equation::RECORD_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut solved_eqn = (*solved_eqn).clone();
            (solved_eqn, status) = solveMultiRecordStrongComponent(solved_eqn.clone(), var_slices, funcMap, true)?;
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![solved_eqn.clone()]);
            (metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(eqn), indices: eqn_slice.indices.clone() }), status)
        },
        Deref @ Equation::DUMMY_EQUATION => {
            (eqn_slice, Status::EXPLICIT.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveMultiStrongComponent")); __mm_s.push_str(&*literal!(" failed for equation:\n")); __mm_s.push_str(&*Slice::toString(eqn_slice, &({ let __pe_b1 = literal!(""); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn_slice, status, implicit_index))
}

pub(crate) fn solveMultiRecordStrongComponent(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut var_slices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut inFor: bool,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status)> {
    let mut solved_eqn: metamodelica::Ref<Equation::Equation> = eqn.clone();
    let mut status: Status = Status::UNPROCESSED.clone();
    let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut v in (var_slices.clone()).into_iter().cloned() {
            let __x = Slice::getT(v.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut lhs: metamodelica::Ref<Expression::NFExpression> = Util::getOption(Equation::getLHS(eqn.clone())?)?;
    let mut rhs: metamodelica::Ref<Expression::NFExpression> = Util::getOption(Equation::getRHS(eqn.clone())?)?;
    let mut record_crefs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    (solved_eqn, status) = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (exp @ Deref @ Expression::TUPLE { .. }, _) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), vars.clone(), inFor)?) => {
            (solved_eqn, Status::EXPLICIT.clone())
        },
        (_, exp @ Deref @ Expression::TUPLE { .. }) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), vars.clone(), inFor)?) => {
            solved_eqn = Equation::setRHS(solved_eqn, &lhs)?;
            solved_eqn = Equation::setLHS(solved_eqn, &rhs)?;
            (solved_eqn, Status::EXPLICIT.clone())
        },
        _ => {
            record_crefs = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            for mut var_slice in &*var_slices {
                var_cref = BVariable::getVarName(Slice::getT(var_slice.clone()));
                (var_cref, status) = getVarSlice(var_cref.clone(), Some(var_cref), eqn.clone())?;
                UnorderedSet::add(var_cref.clone(), record_crefs.clone())?;
                if status == Status::UNSOLVABLE.clone() {
                    break;
                }
            }
            solved_eqn = (::match_deref::match_deref! { match &((UnorderedSet::toList(record_crefs), status)) {
        (Deref @ metamodelica::ListNode::Cons { head: __esc_var_cref, tail: Deref @ metamodelica::ListNode::Nil }, Status::UNPROCESSED) => {
            var_cref = (*__esc_var_cref).clone();
            (solved_eqn, status, _) = solveBody(eqn, var_cref.clone(), funcMap)?;
            solved_eqn
        },
        _ => eqn,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (solved_eqn, status)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((solved_eqn, status))
}

pub(crate) fn solveEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status, i32, RelationInversion)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status;
    let mut implicit_index: i32 = implicit_index;
    let mut invertRelation: RelationInversion;
    (eqn, status, invertRelation) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body @ Deref @ Equation::IF_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            let mut body_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
            let mut indexed_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut dummy: metamodelica::Ref<Iterator::Iterator>;
            (indexed_var, _) = BVariable::makeVarPtr(BVariable::getVar(&cref, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo"))?, cref)?;
            dummy = Iterator::dummy(__eqn_iter.clone())?;
            (body_slice, status, implicit_index) = solveMultiStrongComponent(metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(body.clone()), indices: metamodelica::nil() }), list![metamodelica::Ref::new(Slice::NBSlice { t: indexed_var, indices: metamodelica::nil() })], funcMap, kind, implicit_index, slicing_map, &dummy, varData, eqData)?;
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![Pointer::access(Slice::getT(body_slice))]);
            (eqn, status, RelationInversion::FALSE.clone())
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut body = (*body).clone();
            (body, status, invertRelation) = solveBody(body.clone(), cref, funcMap)?;
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body.clone()]);
            (eqn, status, invertRelation)
        },
        Deref @ Equation::FOR_EQUATION { .. } => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveEquation")); __mm_s.push_str(&*literal!(" failed to solve a for-equation with multiple body eqns for a single cref. Please iterate over body elements individually.\n")); __mm_s.push_str(&*literal!("cref: ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); __mm_s.push_str(&*literal!(" in equation:\n")); __mm_s.push_str(&*Equation::toString(eqn, literal!(""))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        Deref @ Equation::DUMMY_EQUATION => {
            (eqn, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        _ => {
            solveBody(eqn, cref, funcMap)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, status, implicit_index, invertRelation))
}

pub(crate) fn singleElement(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut element: Option<metamodelica::Ref<Expression::NFExpression>> = None;
    let mut expanded: metamodelica::Ref<Expression::NFExpression>;
    let mut success: bool;
    if !(Type::isArray(&(Expression::typeOf(exp.clone())))) {
        element = Some(exp);
        return Ok(element);
    }
    (expanded, success) = ExpandExp::expand(exp, true, false)?;
    if success {
        element = (match &*expanded {
            Expression::ARRAY { .. }
                if (metamodelica::arrayLength(
                    var_field!((*expanded).elements, Expression::NFExpression::ARRAY).clone(),
                ) == 1) =>
            {
                singleElement(
                    ({
                        let __elt = (*metamodelica::index_checked(
                            &var_field!((*expanded).elements, Expression::NFExpression::ARRAY).borrow(),
                            1,
                        )?)
                        .clone();
                        __elt
                    }),
                )?
            }
            _ => None,
        });
    }
    Ok(element)
}

pub(crate) fn scalarElementEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut result: metamodelica::Ref<Equation::Equation> = eqn.clone();
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut success_lhs: bool;
    let mut success_rhs: bool;
    let mut idx: i32 = 0;
    let () = (match &*eqn {
        Equation::ARRAY_EQUATION {
            attr: __eqn_attr,
            lhs: __eqn_lhs,
            rhs: __eqn_rhs,
            source: __eqn_source,
            ty: __eqn_ty,
            ..
        } => {
            (lhs, success_lhs) = ExpandExp::expand(__eqn_lhs.clone(), true, false)?;
            (rhs, success_rhs) = ExpandExp::expand(__eqn_rhs.clone(), true, false)?;
            if success_lhs && success_rhs {
                let () = (::match_deref::match_deref! { match &((&*lhs, &*rhs)) {
                    (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*lhs).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*rhs).elements, Expression::NFExpression::ARRAY).clone())) => {
                        for mut i in 1..=metamodelica::arrayLength(var_field!((*lhs).elements, Expression::NFExpression::ARRAY).clone()) {
                            if Expression::containsCref(({let __elt = (*metamodelica::index_checked(&var_field!((*lhs).elements, Expression::NFExpression::ARRAY).borrow(), i)?).clone(); __elt}), cref)? || Expression::containsCref(({let __elt = (*metamodelica::index_checked(&var_field!((*rhs).elements, Expression::NFExpression::ARRAY).borrow(), i)?).clone(); __elt}), cref)? {
                                idx = if (idx == 0) {i} else {-1};
                            }
                        }
                        if idx > 0 {
                            result = metamodelica::Ref::new(Equation::Equation::SCALAR_EQUATION { ty: Type::arrayElementType(metamodelica::AsArg::as_arg(&__eqn_ty)), lhs: ({let __elt = (*metamodelica::index_checked(&var_field!((*lhs).elements, Expression::NFExpression::ARRAY).borrow(), idx)?).clone(); __elt}), rhs: ({let __elt = (*metamodelica::index_checked(&var_field!((*rhs).elements, Expression::NFExpression::ARRAY).borrow(), idx)?).clone(); __elt}), source: __eqn_source.clone(), attr: __eqn_attr.clone() });
                        }
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            ()
        }
        _ => (),
    });
    Ok(result)
}

pub(crate) fn arrayElementEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut result: metamodelica::Ref<Equation::Equation> = eqn.clone();
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(&cref);
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
        ComponentRef::subscriptsAllWithWholeFlat(&cref)?;
    result = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref: side, .. }, attr: __eqn_attr, rhs: __eqn_rhs, source: __eqn_source, ty: __eqn_ty, .. } if (ComponentRef::isEqual(metamodelica::AsArg::as_arg(&side), &name)?) => {
            metamodelica::Ref::new(Equation::Equation::SCALAR_EQUATION { ty: Type::arrayElementType(metamodelica::AsArg::as_arg(&__eqn_ty)), lhs: Expression::fromCref(cref, false)?, rhs: Expression::applySubscripts(&subs, __eqn_rhs.clone(), true)?, source: __eqn_source.clone(), attr: __eqn_attr.clone() })
        },
        Deref @ Equation::ARRAY_EQUATION { rhs: Deref @ Expression::CREF { cref: side, .. }, attr: __eqn_attr, lhs: __eqn_lhs, source: __eqn_source, ty: __eqn_ty, .. } if (ComponentRef::isEqual(metamodelica::AsArg::as_arg(&side), &name)?) => {
            metamodelica::Ref::new(Equation::Equation::SCALAR_EQUATION { ty: Type::arrayElementType(metamodelica::AsArg::as_arg(&__eqn_ty)), lhs: Expression::applySubscripts(&subs, __eqn_lhs.clone(), true)?, rhs: Expression::fromCref(cref, false)?, source: __eqn_source.clone(), attr: __eqn_attr.clone() })
        },
        Deref @ Equation::ARRAY_EQUATION { ty: __eqn_ty, .. } if (Type::sizeOf(metamodelica::AsArg::as_arg(&__eqn_ty), false)? <= 64) => {
            scalarElementEquation(eqn, &cref)?
        },
        _ => {
            eqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn solveBody(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status, RelationInversion)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status;
    let mut invertRelation: RelationInversion;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut fixed_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut residual: metamodelica::Ref<Expression::NFExpression>;
    let mut derivative: metamodelica::Ref<Expression::NFExpression>;
    let mut diffArgs: metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>;
    fixed_cref = ComponentRef::stripSubscriptsAll(&cref);
    ty = ComponentRef::getSubscriptedType(&fixed_cref, true)?;
    if Type::isArray(&ty) && Type::sizeOf(&ty, false)? == 1 {
        (fixed_cref, _) = getVarSlice(fixed_cref, Some(cref), eqn.clone())?;
    } else {
        fixed_cref = cref.clone();
        eqn = (match &*eqn {
            Equation::ARRAY_EQUATION {
                recordSize: None,
                attr: __eqn_attr,
                lhs: __eqn_lhs,
                rhs: __eqn_rhs,
                source: __eqn_source,
                ty: __eqn_ty,
            } if (!(Type::isArray(&ty)) && Type::sizeOf(metamodelica::AsArg::as_arg(&__eqn_ty), false)? == 1) => {
                let mut lhs: Option<metamodelica::Ref<Expression::NFExpression>>;
                let mut rhs: Option<metamodelica::Ref<Expression::NFExpression>>;
                lhs = singleElement(__eqn_lhs.clone())?;
                rhs = singleElement(__eqn_rhs.clone())?;
                if ((lhs).is_some() && (rhs).is_some()) {
                    metamodelica::Ref::new(Equation::Equation::SCALAR_EQUATION {
                        ty: Type::arrayElementType(metamodelica::AsArg::as_arg(&__eqn_ty)),
                        lhs: Util::getOption(lhs)?,
                        rhs: Util::getOption(rhs)?,
                        source: __eqn_source.clone(),
                        attr: __eqn_attr.clone(),
                    })
                } else {
                    eqn
                }
            }
            Equation::ARRAY_EQUATION {
                recordSize: None,
                ty: __eqn_ty,
                ..
            } if (!(Type::isArray(&ty)) && Type::sizeOf(metamodelica::AsArg::as_arg(&__eqn_ty), false)? > 1) => {
                scalarElementEquation(eqn, &cref)?
            }
            Equation::ARRAY_EQUATION { recordSize: None, .. }
                if (!(Type::isArray(&(ComponentRef::getSubscriptedType(&cref, true)?)))) =>
            {
                arrayElementEquation(eqn, cref.clone())?
            }
            _ => eqn,
        });
    }
    if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
        solvePrintInput(eqn.clone(), &fixed_cref)?;
    }
    (eqn, status, invertRelation) = solveSimple(eqn, &fixed_cref)?;
    if status == Status::UNPROCESSED.clone() {
        residual = Equation::getResidualExp(&eqn, true)?;
        (eqn, status) = solveUnique(eqn, residual.clone(), fixed_cref.clone())?;
        if status == Status::EXPLICIT.clone() {
            invertRelation = RelationInversion::UNKNOWN.clone();
        } else {
            diffArgs = Differentiate::DifferentiationArguments::simpleCref(fixed_cref.clone(), funcMap);
            match '__try0: {
                (derivative, diffArgs) = unwrap_break_err!(Differentiate::differentiateExpressionDump(residual.clone(), diffArgs.clone(), &(literal!("NBSolve.solveBody")), &(literal!(""))), '__try0);
                derivative = unwrap_break_err!(SimplifyExp::simplifyDump(derivative.clone(), true, &(literal!("NBSolve.solveBody")), &(literal!(""))), '__try0);
                Ok::<_, &'static str>((derivative.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    derivative = __try0_o0;
                }
                Err(_) => {
                    derivative = Expression::fromCref(fixed_cref.clone(), false)?;
                }
            }
            if Expression::isZero(&derivative)? {
                invertRelation = RelationInversion::FALSE.clone();
                status = if (Type::isArray(&(ComponentRef::getSubscriptedType(&fixed_cref, true)?))
                    && !(Expression::containsCref(residual, &fixed_cref)?)
                    && !((Equation::collectCrefs(
                        eqn.clone(),
                        (std::sync::Arc::new({
                            let __pe_b2 = ComponentRef::stripSubscriptsAll(&fixed_cref);
                            move |__pe_a0, __pe_a1| Slice::getSliceCandidates(__pe_a0, __pe_a1, __pe_b2.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                                        metamodelica::Ref<
                                            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                        >,
                                    )
                                        -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                                    + 'static,
                            >),
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
                    )?)
                    .is_empty()))
                {
                    Status::IMPLICIT.clone()
                } else {
                    Status::UNSOLVABLE.clone()
                };
            } else if !(Expression::containsCref(derivative.clone(), &fixed_cref)?) {
                eqn = solveLinear(eqn, residual, derivative.clone(), &diffArgs, fixed_cref)?;
                invertRelation = if (Expression::isPositive(&derivative)?) {
                    RelationInversion::FALSE.clone()
                } else {
                    if (Expression::isNegative(&derivative)?) {
                        RelationInversion::TRUE.clone()
                    } else {
                        RelationInversion::UNKNOWN.clone()
                    }
                };
                status = Status::EXPLICIT.clone();
            } else {
                invertRelation = RelationInversion::FALSE.clone();
                if Flags::isSet(Flags::FAILTRACE.clone())? && status != Status::EXPLICIT.clone() {
                    Error::addCompilerWarning({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBSolve.solveBody"));
                        __mm_s.push_str(&*literal!(" cref: "));
                        __mm_s.push_str(&*ComponentRef::toString(&fixed_cref)?);
                        __mm_s.push_str(&*literal!(" has to be solved implicitely in equation:\n"));
                        __mm_s.push_str(&*Equation::toString(eqn.clone(), literal!(""))?);
                        ArcStr::from(__mm_s)
                    })?;
                }
            }
        }
    }
    eqn = Equation::simplify(
        eqn,
        &(literal!("NBSolve.solveBody")),
        &(literal!("")),
        Pointer::create(metamodelica::nil()),
        Pointer::create(metamodelica::nil()),
        (std::sync::Arc::new({
            let __pe_b1 = true;
            let __pe_b2 = literal!("NBSolve.solveBody");
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
    if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
        solvePrintOutput(eqn.clone(), status)?;
    }
    Ok((eqn, status, invertRelation))
}

pub(crate) fn solveIfBody(
    mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(metamodelica::Ref<IfEquationBody::IfEquationBody>, Status, i32)> {
    let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody> = body;
    let mut status: Status;
    let mut implicit_index: i32 = implicit_index;
    let mut else_if: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut solved_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut new_then_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut explicit: bool = true;
    (_, comps) = Causalize::simple(
        vars,
        &(EquationPointers::fromList(&body.then_eqns)?),
        kind,
        NBAdjacency::MatrixStrictness::MATCHING.clone(),
        iter,
    )?;
    for mut comp in &*comps {
        (solved_comps, implicit_index) = solveStrongComponent(
            comp.clone(),
            funcMap.clone(),
            kind,
            implicit_index,
            slicing_map.clone(),
            varData,
            eqData,
        )?;
        for mut solved_comp in &*solved_comps {
            if StrongComponent::getSolveStatus(metamodelica::AsArg::as_arg(&solved_comp))? == Status::EXPLICIT.clone() {
                new_then_eqns = metamodelica::cons(
                    StrongComponent::toSolvedEquation(metamodelica::AsArg::as_arg(&solved_comp))?,
                    new_then_eqns,
                );
            } else {
                explicit = false;
            }
        }
    }
    if !(explicit) {
        status = Status::IMPLICIT.clone();
        return Ok((body, status, implicit_index));
    }
    assign_field!(body.then_eqns = new_then_eqns.reverse());
    if (body.else_if).is_some() {
        (else_if, status, implicit_index) = solveIfBody(
            Util::getOption(body.else_if.clone())?,
            vars,
            funcMap,
            kind,
            implicit_index,
            slicing_map,
            iter,
            varData,
            eqData,
        )?;
        assign_field!(body.else_if = Some(else_if));
    } else {
        status = Status::EXPLICIT.clone();
    }
    Ok((body, status, implicit_index))
}

pub(crate) fn solveSimple(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status, RelationInversion)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status;
    let mut invertRelation: RelationInversion;
    (eqn, status, invertRelation) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::SCALAR_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            solveSimpleLhsRhs(__eqn_lhs.clone(), __eqn_rhs.clone(), cref, eqn)?
        },
        Deref @ Equation::ARRAY_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            solveSimpleLhsRhs(__eqn_lhs.clone(), __eqn_rhs.clone(), cref, eqn)?
        },
        Deref @ Equation::RECORD_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            solveSimpleLhsRhs(__eqn_lhs.clone(), __eqn_rhs.clone(), cref, eqn)?
        },
        Deref @ Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            solveSimpleWhen(metamodelica::AsArg::as_arg(&__eqn_body), cref, eqn)?
        },
        Deref @ Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut body = (*body).clone();
            (body, status, invertRelation) = solveSimple(body.clone(), cref)?;
            if status == Status::EXPLICIT.clone() {
                assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body.clone()]);
            } else {
                status = Status::UNPROCESSED.clone();
            }
            (eqn, status, invertRelation)
        },
        Deref @ Equation::IF_EQUATION { body: __eqn_body, .. } => {
            let mut if_body: metamodelica::Ref<IfEquationBody::IfEquationBody>;
            (if_body, status, invertRelation) = solveSimpleIf(__eqn_body.clone(), cref)?;
            if status == Status::EXPLICIT.clone() {
                assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = if_body);
            } else {
                status = Status::UNPROCESSED.clone();
            }
            (eqn, status, invertRelation)
        },
        _ => {
            (eqn, Status::UNPROCESSED.clone(), RelationInversion::FALSE.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, status, invertRelation))
}

fn solveSimpleLhsRhs(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn: metamodelica::Ref<Equation::Equation>,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status, RelationInversion)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status;
    let mut invertRelation: RelationInversion;
    (eqn, status, invertRelation) = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ Expression::CREF { cref: checkCref, .. }, exp) if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(exp.clone(), cref)?)) => {
            (eqn, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp, Deref @ Expression::CREF { cref: checkCref, .. }) if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(exp.clone(), cref)?)) => {
            (Equation::swapLHSandRHS(eqn)?, Status::EXPLICIT.clone(), RelationInversion::TRUE.clone())
        },
        (Deref @ Expression::UNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }, exp) if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(exp.clone(), cref)?)) => {
            (Equation::updateLHSandRHS(eqn, Expression::negate(lhs), Expression::negate(rhs))?, Status::EXPLICIT.clone(), RelationInversion::TRUE.clone())
        },
        (Deref @ Expression::LUNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }, exp) if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(exp.clone(), cref)?)) => {
            (Equation::updateLHSandRHS(eqn, Expression::logicNegate(lhs), Expression::logicNegate(rhs))?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp, Deref @ Expression::UNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }) if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(exp.clone(), cref)?)) => {
            (Equation::updateLHSandRHS(eqn, Expression::negate(rhs), Expression::negate(lhs))?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp, Deref @ Expression::LUNARY { exp: Deref @ Expression::CREF { cref: checkCref, .. }, .. }) if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(exp.clone(), cref)?)) => {
            (Equation::updateLHSandRHS(eqn, Expression::logicNegate(rhs), Expression::logicNegate(lhs))?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (exp @ Deref @ Expression::TUPLE { .. }, _) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), list![BVariable::getVarPointer(cref, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo"))?], false)?) => {
            (eqn, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        (_, exp @ Deref @ Expression::TUPLE { .. }) if (tupleSolvable(var_field!((**exp).elements, Expression::NFExpression::TUPLE).clone(), list![BVariable::getVarPointer(cref, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo"))?], false)?) => {
            (Equation::swapLHSandRHS(eqn)?, Status::EXPLICIT.clone(), RelationInversion::FALSE.clone())
        },
        _ => {
            (eqn, Status::UNPROCESSED.clone(), RelationInversion::FALSE.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqn, status, invertRelation))
}

fn solveSimpleWhen(
    mut body: &metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn: metamodelica::Ref<Equation::Equation>,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status, RelationInversion)> {
    let mut eqnOut: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status = Status::UNSOLVABLE.clone();
    let mut invertRelation: RelationInversion = RelationInversion::FALSE.clone();
    for mut stmt in &*body.when_stmts.clone() {
        status = (::match_deref::match_deref! { match &(stmt.clone()) {
            Deref @ WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: checkCref, .. }, rhs: __stmt_rhs, .. } if (ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&checkCref))? && !(Expression::containsCref(__stmt_rhs.clone(), cref)?)) => {
                Status::EXPLICIT.clone()
            },
            _ => {
                Status::UNSOLVABLE.clone()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if status == Status::EXPLICIT.clone() {
            break;
        }
    }
    Ok((eqnOut, status, invertRelation))
}

fn solveSimpleIf(
    mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<IfEquationBody::IfEquationBody>,
    Status,
    RelationInversion,
)> {
    let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody> = body;
    let mut status: Status = Status::EXPLICIT.clone();
    let mut invertRelation: RelationInversion = RelationInversion::FALSE.clone();
    let mut else_if: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    if (body.else_if).is_some() {
        (else_if, status, _) = solveSimpleIf(Util::getOption(body.else_if.clone())?, cref)?;
        if status == Status::EXPLICIT.clone() {
            assign_field!(body.else_if = Some(else_if));
        }
    }
    if status == Status::EXPLICIT.clone() && List::hasOneElement(&body.then_eqns) {
        eqn = Pointer::access((body.then_eqns).head().cloned()?);
        (eqn, status, _) = solveSimple(eqn, cref)?;
        if status == Status::EXPLICIT.clone() {
            Pointer::update((body.then_eqns).head().cloned()?, eqn);
        }
    } else {
        status = Status::UNPROCESSED.clone();
    }
    Ok((body, status, invertRelation))
}

fn solveLinear(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut residual: metamodelica::Ref<Expression::NFExpression>,
    mut derivative: metamodelica::Ref<Expression::NFExpression>,
    mut diffArgs: &metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut crefExp: metamodelica::Ref<Expression::NFExpression>;
    let mut numerator: metamodelica::Ref<Expression::NFExpression>;
    let mut mulOp: metamodelica::Ref<Operator::NFOperator>;
    let mut uminOp: metamodelica::Ref<Operator::NFOperator>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    crefExp = Expression::fromCref(cref.clone(), false)?;
    ty = ComponentRef::getSubscriptedType(&cref, true)?;
    numerator = Replacements::single(residual, &crefExp, &(Expression::makeZero(&ty)?))?;
    mulOp = metamodelica::Ref::new(Operator::NFOperator {
        ty: ty.clone(),
        op: Operator::Op::MUL.clone(),
    });
    uminOp = metamodelica::Ref::new(Operator::NFOperator {
        ty: ty,
        op: Operator::Op::UMINUS.clone(),
    });
    eqn = Equation::setLHS(eqn, &crefExp)?;
    eqn = Equation::setRHS(
        eqn,
        &(metamodelica::Ref::new(Expression::NFExpression::UNARY {
            operator: uminOp,
            exp: metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                arguments: list![numerator],
                inv_arguments: list![derivative],
                operator: mulOp,
            }),
        })),
    )?;
    Ok(eqn)
}

fn solveUnique(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut residual: metamodelica::Ref<Expression::NFExpression>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(metamodelica::Ref<Equation::Equation>, Status)> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut status: Status;
    let mut crefExp: metamodelica::Ref<Expression::NFExpression> = Expression::fromCref(cref.clone(), false)?;
    let mut solvedRHS: metamodelica::Ref<Expression::NFExpression>;
    let mut crefFound: bool;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<Type::NFType> = ComponentRef::getSubscriptedType(&cref, true)?;
    (crefFound, inverseInstructions, status) =
        solveUniqueFindInstructions(residual, &cref, false, inverseInstructions)?;
    if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
        solveUniquePrintInstructions(&inverseInstructions, status)?;
    }
    eqn = (match status {
        Status::IMPLICIT => eqn,
        _ => {
            if !(crefFound) {
                status = Status::IMPLICIT.clone();
            } else {
                status = Status::EXPLICIT.clone();
                solvedRHS = Expression::makeZero(&ty)?;
                for mut instruction in &*inverseInstructions {
                    solvedRHS = applyInstruction(solvedRHS, instruction.clone())?;
                }
                eqn = Equation::setLHS(eqn, &crefExp)?;
                eqn = Equation::setRHS(eqn, &solvedRHS)?;
            }
            eqn
        }
    });
    Ok((eqn, status))
}

fn solveUniqueFindInstructions(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::EXPLICIT.clone();
    let mut substExp: metamodelica::Ref<Expression::NFExpression> =
        BVariable::toExpression(Pointer::create(BVariable::SUBST_VARIABLE().clone()))?;
    let mut ty: metamodelica::Ref<Type::NFType> = ComponentRef::getSubscriptedType(cref, true)?;
    let mut call: metamodelica::Ref<Call::NFCall>;
    if crefFound {
        if Expression::containsCref(exp, cref)? {
            status = Status::IMPLICIT.clone();
        } else {
            crefFound = false;
        }
        return Ok((crefFound, inverseInstructions, status));
    }
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::REAL { .. } => (),
        Deref @ Expression::INTEGER { .. } => (),
        Deref @ Expression::CREF { cref: __exp_cref, .. } => {
            if ComponentRef::isEqual(cref, metamodelica::AsArg::as_arg(&__exp_cref))? {
                crefFound = true;
            }
            ()
        },
        Deref @ Expression::CAST { .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsCast(&substExp, &exp, cref, crefFound, inverseInstructions)?;
            ()
        },
        Deref @ Expression::MULTARY { .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsMultary(substExp, &exp, cref, crefFound, inverseInstructions)?;
            ()
        },
        Deref @ Expression::BINARY { operator: __exp_operator, .. } => {
            let () = (match &*__exp_operator.clone() {
        Operator::OPERATOR { op: Operator::Op::POW, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsBinaryPow(ty, substExp, &exp, cref, crefFound, inverseInstructions)?;
            ()
        },
        Operator::OPERATOR { op: Operator::Op::ADD, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsBinaryComOp(substExp, &exp, cref, crefFound, inverseInstructions)?;
            ()
        },
        Operator::OPERATOR { op: Operator::Op::MUL, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsBinaryComOp(substExp, &exp, cref, crefFound, inverseInstructions)?;
            ()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            ()
        },
    });
            ()
        },
        Deref @ Expression::UNARY { operator: __exp_operator, .. } => {
            let () = (match &*__exp_operator.clone() {
        Operator::OPERATOR { op: Operator::Op::UMINUS, .. } => {
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsUnaryUminus(ty, substExp, &exp, cref, crefFound, inverseInstructions)?;
            ()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            ()
        },
    });
            ()
        },
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { .. } } if (List::none(&(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?), &({ let __pe_b1 = cref.clone(); move |__pe_a0| solveUniqueExpressionNoCref(__pe_a0, &__pe_b1) }))?) => {
            call = (*__esc_call).clone();
            ()
        },
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } if (List::none(&(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?), &({ let __pe_b1 = cref.clone(); move |__pe_a0| solveUniqueExpressionNoCref(__pe_a0, &__pe_b1) }))?) => {
            call = (*__esc_call).clone();
            ()
        },
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_REDUCTION { .. } } if (List::none(&(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?), &({ let __pe_b1 = cref.clone(); move |__pe_a0| solveUniqueExpressionNoCref(__pe_a0, &__pe_b1) }))?) => {
            call = (*__esc_call).clone();
            ()
        },
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { .. } } if (List::hasOneElement(&(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?))) => {
            call = (*__esc_call).clone();
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsCallOneArg(ty, substExp, exp.clone(), cref, crefFound, inverseInstructions)?;
            ()
        },
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { .. } } if ((((Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?)).len() as i32) == 2) => {
            call = (*__esc_call).clone();
            (crefFound, inverseInstructions, status) = solveUniqueFindInstructionsCallTwoArgs(ty, substExp, exp.clone(), cref, crefFound, inverseInstructions)?;
            ()
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsMultary(
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::EXPLICIT.clone();
    let mut argList: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut invargList: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut crefFoundInRecursion: bool;
    let () = (match &**exp {
        Expression::MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            operator: __exp_operator,
        } => {
            for mut arg in &*__exp_arguments.clone() {
                (crefFoundInRecursion, inverseInstructions, status) =
                    solveUniqueFindInstructions(arg.clone(), cref, crefFound, inverseInstructions)?;
                if status == Status::IMPLICIT.clone() {
                    return Ok((crefFound, inverseInstructions, status));
                }
                if !(crefFoundInRecursion) {
                    argList = metamodelica::cons(arg.clone(), argList);
                } else {
                    crefFound = true;
                }
            }
            if crefFound {
                if List::any(
                    metamodelica::AsArg::as_arg(&__exp_inv_arguments),
                    &({
                        let __pe_b1 = cref.clone();
                        move |__pe_a0| Expression::containsCref(__pe_a0, &__pe_b1)
                    }),
                )? {
                    status = Status::IMPLICIT.clone();
                    return Ok((crefFound, inverseInstructions, status));
                } else {
                    inverseInstructions = metamodelica::cons(
                        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: metamodelica::cons(substExp, __exp_inv_arguments.clone()),
                            inv_arguments: argList,
                            operator: __exp_operator.clone(),
                        }),
                        inverseInstructions,
                    );
                }
            } else {
                for mut invarg in &*__exp_inv_arguments.clone() {
                    (crefFoundInRecursion, inverseInstructions, status) =
                        solveUniqueFindInstructions(invarg.clone(), cref, crefFound, inverseInstructions)?;
                    if status == Status::IMPLICIT.clone() {
                        return Ok((crefFound, inverseInstructions, status));
                    }
                    if !(crefFoundInRecursion) {
                        invargList = metamodelica::cons(invarg.clone(), invargList);
                    } else {
                        crefFound = true;
                    }
                }
                if crefFound {
                    inverseInstructions = metamodelica::cons(
                        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                            arguments: argList,
                            inv_arguments: metamodelica::cons(substExp, invargList),
                            operator: __exp_operator.clone(),
                        }),
                        inverseInstructions,
                    );
                }
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsMultary"));
                    __mm_s.push_str(&*literal!(" can only be called for Expression.MULTARY."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsBinaryPow(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool;
    let mut local_exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut local_exp2: metamodelica::Ref<Expression::NFExpression>;
    let () = (match &**exp {
        Expression::BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            (crefFoundInRecursion, inverseInstructions, status) =
                solveUniqueFindInstructions(__exp_exp1.clone(), cref, crefFound, inverseInstructions)?;
            if status == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion {
                crefFound = true;
                if Expression::containsCref(__exp_exp2.clone(), cref)? {
                    status = Status::IMPLICIT.clone();
                } else {
                    inverseInstructions = metamodelica::cons(
                        metamodelica::Ref::new(Expression::NFExpression::BINARY {
                            exp1: substExp,
                            operator: __exp_operator.clone(),
                            exp2: metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                arguments: metamodelica::nil(),
                                inv_arguments: list![__exp_exp2.clone()],
                                operator: metamodelica::Ref::new(Operator::NFOperator {
                                    ty: ty,
                                    op: Operator::Op::MUL.clone(),
                                }),
                            }),
                        }),
                        inverseInstructions,
                    );
                }
            } else {
                (crefFoundInRecursion, inverseInstructions, status) =
                    solveUniqueFindInstructions(__exp_exp2.clone(), cref, crefFound, inverseInstructions)?;
                if status == Status::IMPLICIT.clone() {
                    return Ok((crefFound, inverseInstructions, status));
                }
                if crefFoundInRecursion {
                    crefFound = true;
                    local_exp1 = metamodelica::Ref::new(Expression::NFExpression::CALL {
                        call: Call::makeTypedCall(
                            NFBuiltinFuncs::LOG_REAL().clone(),
                            list![substExp.clone()],
                            Expression::variability(substExp.clone())?,
                            NFPrefixes::Purity::PURE.clone(),
                            NFBuiltinFuncs::LOG_REAL().returnType.clone(),
                        ),
                    });
                    local_exp2 = metamodelica::Ref::new(Expression::NFExpression::CALL {
                        call: Call::makeTypedCall(
                            NFBuiltinFuncs::LOG_REAL().clone(),
                            list![__exp_exp1.clone()],
                            Expression::variability(__exp_exp1.clone())?,
                            NFPrefixes::Purity::PURE.clone(),
                            NFBuiltinFuncs::LOG_REAL().returnType.clone(),
                        ),
                    });
                    inverseInstructions = metamodelica::cons(
                        local_exp1,
                        metamodelica::cons(
                            metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                arguments: list![substExp],
                                inv_arguments: list![local_exp2],
                                operator: metamodelica::Ref::new(Operator::NFOperator {
                                    ty: ty,
                                    op: Operator::Op::MUL.clone(),
                                }),
                            }),
                            inverseInstructions,
                        ),
                    );
                }
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsBinaryPow"));
                    __mm_s.push_str(&*literal!(
                        " can only be called for Expression.BINARY with operator POW."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsBinaryComOp(
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool;
    let () = (match &**exp {
        Expression::BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructionsMultary(
                substExp,
                &(metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![__exp_exp1.clone(), __exp_exp2.clone()],
                    inv_arguments: metamodelica::nil(),
                    operator: __exp_operator.clone(),
                })),
                cref,
                crefFound,
                inverseInstructions,
            )?;
            if status == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion {
                crefFound = true;
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsBinaryComOp"));
                    __mm_s.push_str(&*literal!(
                        " can only be called for Expression.BINARY with commutative operator."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsCast(
    mut substExp: &metamodelica::Ref<Expression::NFExpression>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool;
    let () = (match &**exp {
        Expression::CAST { exp: __exp_exp, .. } => {
            (crefFoundInRecursion, inverseInstructions, status) =
                solveUniqueFindInstructions(__exp_exp.clone(), cref, crefFound, inverseInstructions)?;
            if status == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion {
                crefFound = true;
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsCast"));
                    __mm_s.push_str(&*literal!(" can only be called for Expression.CAST."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsUnaryUminus(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool;
    let () = (match &**exp {
        Expression::UNARY { exp: __exp_exp, .. } => {
            (crefFoundInRecursion, inverseInstructions, status) =
                solveUniqueFindInstructions(__exp_exp.clone(), cref, crefFound, inverseInstructions)?;
            if status == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion {
                crefFound = true;
                inverseInstructions = metamodelica::cons(
                    metamodelica::Ref::new(Expression::NFExpression::UNARY {
                        operator: metamodelica::Ref::new(Operator::NFOperator {
                            ty: ty,
                            op: Operator::Op::UMINUS.clone(),
                        }),
                        exp: substExp,
                    }),
                    inverseInstructions,
                );
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsUnaryUminus"));
                    __mm_s.push_str(&*literal!(
                        " can only be called for Expression.BINARY with commutative operator."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsCallOneArg(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool;
    let mut argExp: metamodelica::Ref<Expression::NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut name: ArcStr;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { .. } } if (List::hasOneElement(&(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?))) => {
            call = (*__esc_call).clone();
            name = AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?;
            argExp = ((Call::arguments(metamodelica::AsArg::as_arg(&call))?)).head().cloned()?;
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(argExp, cref, crefFound, inverseInstructions)?;
            if status == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion {
                crefFound = true;
                inverseInstructions = (::match_deref::match_deref! { match &(name) {
        Deref @ "sqrt" => metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: substExp, operator: metamodelica::Ref::new(Operator::NFOperator { ty: ty, op: Operator::Op::POW.clone() }), exp2: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((2) as f64) }) }), inverseInstructions),
        Deref @ "cos" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ACOS_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "sin" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ASIN_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "tan" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ATAN_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "acos" => solveUniqueCreateSubstCall(NFBuiltinFuncs::COS_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "asin" => solveUniqueCreateSubstCall(NFBuiltinFuncs::SIN_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "atan" => solveUniqueCreateSubstCall(NFBuiltinFuncs::TAN_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "cosh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ACOSH_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "sinh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ASINH_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "tanh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::ATANH_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "acosh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::COSH_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "asinh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::SINH_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "atanh" => solveUniqueCreateSubstCall(NFBuiltinFuncs::TANH_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "exp" => solveUniqueCreateSubstCall(NFBuiltinFuncs::LOG_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "log" => solveUniqueCreateSubstCall(NFBuiltinFuncs::EXP_REAL().clone(), substExp, inverseInstructions)?,
        Deref @ "log10" => metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((10) as f64) }), operator: metamodelica::Ref::new(Operator::NFOperator { ty: ty, op: Operator::Op::POW.clone() }), exp2: substExp }), inverseInstructions),
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            inverseInstructions
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsCallOneArg")); __mm_s.push_str(&*literal!(" can only be called for Expression.CALL with one argument.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueFindInstructionsCallTwoArgs(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut crefFound: bool,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<(
    bool,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    Status,
)> {
    let mut crefFound: bool = crefFound;
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    let mut status: Status = Status::UNPROCESSED;
    let mut crefFoundInRecursion: bool;
    let mut argExp1: metamodelica::Ref<Expression::NFExpression>;
    let mut argExp2: metamodelica::Ref<Expression::NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut name: ArcStr;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_CALL { .. } } if ((((Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?)).len() as i32) == 2) => {
            call = (*__esc_call).clone();
            name = AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((*call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&call))?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            argExp1 = metamodelica::Own::own(__pa0);
            argExp2 = metamodelica::Own::own(__pa1);
            (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(argExp1.clone(), cref, crefFound, inverseInstructions)?;
            if status == Status::IMPLICIT.clone() {
                return Ok((crefFound, inverseInstructions, status));
            }
            if crefFoundInRecursion {
                crefFound = true;
                if Expression::containsCref(argExp2.clone(), cref)? {
                    status = Status::IMPLICIT.clone();
                } else {
                    inverseInstructions = (::match_deref::match_deref! { match &(name) {
        Deref @ "atan2" => {
            inverseInstructions = metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![substExp.clone(), argExp2], inv_arguments: metamodelica::nil(), operator: metamodelica::Ref::new(Operator::NFOperator { ty: ty, op: Operator::Op::MUL.clone() }) }), inverseInstructions);
            solveUniqueCreateSubstCall(NFBuiltinFuncs::TAN_REAL().clone(), substExp, inverseInstructions)?
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            inverseInstructions
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                }
            } else {
                (crefFoundInRecursion, inverseInstructions, status) = solveUniqueFindInstructions(argExp2, cref, crefFound, inverseInstructions)?;
                if status == Status::IMPLICIT.clone() {
                    return Ok((crefFound, inverseInstructions, status));
                }
                if crefFoundInRecursion {
                    crefFound = true;
                    inverseInstructions = (::match_deref::match_deref! { match &(name) {
        Deref @ "atan2" => {
            inverseInstructions = metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: list![argExp1], inv_arguments: list![substExp.clone()], operator: metamodelica::Ref::new(Operator::NFOperator { ty: ty, op: Operator::Op::MUL.clone() }) }), inverseInstructions);
            solveUniqueCreateSubstCall(NFBuiltinFuncs::TAN_REAL().clone(), substExp, inverseInstructions)?
        },
        _ => {
            if Flags::isSet(Flags::DUMP_SOLVE.clone())? {
                solveUniquePrintImplicitFallback(exp.clone())?;
            }
            status = Status::IMPLICIT.clone();
            inverseInstructions
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                }
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.solveUniqueFindInstructionsCallTwoArgs")); __mm_s.push_str(&*literal!(" can only be called for Expression.CALL with two arguments.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((crefFound, inverseInstructions, status))
}

fn solveUniqueCreateSubstCall(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut inverseInstructions: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = inverseInstructions;
    inverseInstructions = metamodelica::cons(
        metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                r#fn.clone(),
                list![exp.clone()],
                Expression::variability(exp)?,
                NFPrefixes::Purity::PURE.clone(),
                r#fn.returnType.clone(),
            ),
        }),
        inverseInstructions,
    );
    Ok(inverseInstructions)
}

fn solveUniqueExpressionNoCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    fn solveUniqueExpressionNoCrefTraverse(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut res: Pointer::Pointer<bool>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        if !(Pointer::access(res.clone())) {
            exp = (match &*exp {
                Expression::CREF { cref: __exp_cref, .. } => {
                    Pointer::update(
                        res,
                        ComponentRef::isEqual(metamodelica::AsArg::as_arg(&__exp_cref), cref)?,
                    );
                    exp
                }
                _ => Expression::mapShallow(
                    exp,
                    (std::sync::Arc::new({
                        let __pe_b1 = cref.clone();
                        let __pe_b2 = res;
                        move |__pe_a0| solveUniqueExpressionNoCrefTraverse(__pe_a0, &__pe_b1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?,
            });
        }
        Ok(exp)
    }

    let mut b: bool;
    let mut res: Pointer::Pointer<bool> = Pointer::create(false);
    Expression::fakeMap(
        exp,
        &({
            let __pe_b1 = cref.clone();
            let __pe_b2 = res.clone();
            move |__pe_a0| solveUniqueExpressionNoCrefTraverse(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    b = Pointer::access(res);
    Ok(b)
}

fn solvePrintInput(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut crefExp: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<()> {
    metamodelica::print(literal!(
        "\n##########################################\nSTART - Solve\n\n"
    ));
    metamodelica::print(literal!("Solve Input:\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("### Variable:\n\t"));
        __mm_s.push_str(&*ComponentRef::toString(crefExp)?);
        __mm_s.push_str(&*literal!("\n### Equation:\n\t"));
        __mm_s.push_str(&*Equation::toString(eqn, literal!(""))?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

fn solvePrintOutput(mut eqn: metamodelica::Ref<Equation::Equation>, mut status: Status) -> Result<()> {
    metamodelica::print(literal!("Solve Output:\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("### Status:\n\t"));
        __mm_s.push_str(&*statusString(status));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("### Equation:\n\t"));
        __mm_s.push_str(&*Equation::toString(eqn, literal!(""))?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!(
        "\nEND - Solve\n##########################################\n\n"
    ));
    Ok(())
}

fn solveUniquePrintInstructions(
    mut inverseInstructions: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut status: Status,
) -> Result<()> {
    metamodelica::print(literal!("SolveUnique Instructions (substitute from top to bottom):\n"));
    metamodelica::print(literal!("\t0 (is initial)\n"));
    for mut instruction in &**inverseInstructions {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\t"));
            __mm_s.push_str(&*Expression::toString(instruction.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("### Status:\n\t"));
        __mm_s.push_str(&*statusString(status));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn solveUniquePrintImplicitFallback(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<()> {
    metamodelica::print(literal!("Setting Status.Implicit (fallback) due to:\n"));
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("### Expression:\n\t"));
        __mm_s.push_str(&*Expression::toString(exp)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn applyInstruction(
    mut insertExp: metamodelica::Ref<Expression::NFExpression>,
    mut instruction: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut insertExp: metamodelica::Ref<Expression::NFExpression> = insertExp;
    insertExp = ({
        let mut argList: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let mut invargList: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(instruction.clone()) {
            Deref @ Expression::MULTARY { arguments: __instruction_arguments, inv_arguments: __instruction_inv_arguments, operator: __instruction_operator } => {
                for mut arg in &*__instruction_arguments.clone() {
                    if !(Expression::isSubstitute(metamodelica::AsArg::as_arg(&arg))?) {
                        argList = metamodelica::cons(arg.clone(), argList);
                    } else {
                        argList = metamodelica::cons(insertExp.clone(), argList);
                    }
                }
                for mut invarg in &*__instruction_inv_arguments.clone() {
                    if !(Expression::isSubstitute(metamodelica::AsArg::as_arg(&invarg))?) {
                        invargList = metamodelica::cons(invarg.clone(), invargList);
                    } else {
                        invargList = metamodelica::cons(insertExp.clone(), invargList);
                    }
                }
                metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: argList, inv_arguments: invargList, operator: __instruction_operator.clone() })
            },
            Deref @ Expression::BINARY { .. } => {
                if Expression::isSubstitute(var_field!((*instruction).exp1, Expression::NFExpression::BINARY))? {
                    assign_variant_field!(instruction => Expression::NFExpression::BINARY; exp1 = insertExp.clone());
                }
                if Expression::isSubstitute(var_field!((*instruction).exp2, Expression::NFExpression::BINARY))? {
                    assign_variant_field!(instruction => Expression::NFExpression::BINARY; exp2 = insertExp);
                }
                instruction
            },
            Deref @ Expression::UNARY { .. } => {
                if Expression::isSubstitute(var_field!((*instruction).exp, Expression::NFExpression::UNARY))? {
                    assign_variant_field!(instruction => Expression::NFExpression::UNARY; exp = insertExp);
                }
                instruction
            },
            exp @ Deref @ Expression::CALL { .. } => {
                let mut exp = (*exp).clone();
                let () = (::match_deref::match_deref! { match &(var_field!((*instruction).call, Expression::NFExpression::CALL).clone()) {
            local_call @ Deref @ Call::TYPED_CALL { .. } => {
                let mut local_call = (*local_call).clone();
                for mut arg in &*var_field!((*local_call).arguments, Call::NFCall::TYPED_CALL).clone() {
                    if !(Expression::isSubstitute(metamodelica::AsArg::as_arg(&arg))?) {
                        argList = metamodelica::cons(arg.clone(), argList);
                    } else {
                        argList = metamodelica::cons(insertExp.clone(), argList);
                    }
                }
                assign_variant_field!(local_call => Call::NFCall::TYPED_CALL; arguments = argList.reverse());
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = local_call.clone());
                ()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.applyInstruction")); __mm_s.push_str(&*literal!(" can only handle TYPED_CALL.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                exp.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSolve.applyInstruction")); __mm_s.push_str(&*literal!(" failed for instruction: ")); __mm_s.push_str(&*Expression::toString(instruction)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(insertExp)
}

fn tupleSolvable(
    mut tuple_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut inFor: bool,
) -> Result<bool> {
    let mut b: bool = false;
    let mut filtered_exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (tuple_exps.clone()).into_iter().cloned() {
            if !(!(Expression::isWildCref(&(e.clone())))) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, bool>>;
    let mut sizes: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if List::compareLength(filtered_exps.clone(), vars.clone())? == 0 {
        map = UnorderedMap::new(
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
        sizes = UnorderedMap::new(
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
        for mut var in &*vars {
            UnorderedMap::add(BVariable::getVarName(var.clone()), false, map.clone())?;
            UnorderedMap::add(
                BVariable::getVarName(var.clone()),
                BVariable::size(var.clone(), false)?,
                sizes.clone(),
            )?;
        }
        for mut exp in &*filtered_exps {
            let () = (match &*exp.clone() {
                Expression::CREF { cref: __exp_cref, .. }
                    if (UnorderedMap::contains(__exp_cref.clone(), map.clone())?) =>
                {
                    UnorderedMap::add(__exp_cref.clone(), true, map.clone())?;
                    ()
                }
                Expression::CREF { cref: __exp_cref, .. } => {
                    stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref));
                    if UnorderedMap::contains(stripped.clone(), map.clone())?
                        && (inFor
                            || UnorderedMap::getSafe(
                                stripped.clone(),
                                sizes.clone(),
                                metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo"),
                            )? == Type::sizeOf(&(Expression::typeOf(exp.clone())), false)?)
                    {
                        UnorderedMap::add(stripped, true, map.clone())?;
                    } else {
                        return Ok(b);
                    }
                    ()
                }
                _ => {
                    return Ok(b);
                    ()
                }
            });
        }
        b = List::all(&(UnorderedMap::valueList(map)), &fnptr!(Util::id, _))?;
    }
    Ok(b)
}

fn expandArraySumExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut arrayCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut arrTy: metamodelica::Ref<Type::NFType>;
    let mut elemTy: metamodelica::Ref<Type::NFType>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut sizes: metamodelica::List<i32>;
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut arg_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut elemCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: arg_cref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } if (metamodelica::stringEq(&(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?), &(literal!("sum"))) && ComponentRef::isEqual(metamodelica::AsArg::as_arg(&arg_cref), arrayCref)?) => {
            arrTy = ComponentRef::getSubscriptedType(arrayCref, true)?;
            elemTy = Type::arrayElementType(&arrTy);
            dims = Type::arrayDims(arrTy);
            sizes = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut dim in (dims).into_iter().cloned() {
            let __x = Dimension::size(&(dim.clone()), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            elements = expandArraySumExpDim(&sizes, arrayCref, &elemTy, &(metamodelica::nil()), metamodelica::nil())?;
            new_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY { arguments: elements.reverse(), inv_arguments: metamodelica::nil(), operator: Operator::makeAdd(elemTy) });
            new_exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn expandArraySumExpDim(
    mut sizes: &metamodelica::List<i32>,
    mut arrayCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut elemTy: &metamodelica::Ref<Type::NFType>,
    mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = elements;
    elements = (::match_deref::match_deref! { match sizes {
        Deref @ metamodelica::ListNode::Cons { head: n, tail: rest } => {
            for mut i in 1..=n.clone() {
                elements = expandArraySumExpDim(rest, arrayCref, elemTy, &(metamodelica::cons(metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i }) }), subs.clone())), elements)?;
            }
            elements
        },
        _ => {
            let mut elemCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            elemCref = ComponentRef::mergeSubscripts(subs.clone().reverse(), arrayCref.clone(), false, false, false)?;
            metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::CREF { ty: elemTy.clone(), cref: elemCref }), elements)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn getVarSlice(
    mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut reference: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut eqn: metamodelica::Ref<Equation::Equation>,
) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, Status)> {
    fn checkReference(
        mut var_cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut reference_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    ) -> Result<bool> {
        let mut fit: bool;
        fit = (::match_deref::match_deref! { match &(reference_opt) {
            Some(reference) => {
                Type::sizeOf(&(ComponentRef::getSubscriptedType(var_cref, false)?), true)? == Type::sizeOf(&(ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&reference), false)?), true)?
            },
            _ => {
                true
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(fit)
    }

    let mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = var_cref;
    let mut solve_status: Status;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = BVariable::getVarPointer(
        &var_cref,
        metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo"),
    )?;
    let mut slices_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut filtered: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut record_parent: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    slices_lst = Equation::collectCrefs(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b2 = var_cref.clone();
            move |__pe_a0, __pe_a1| Slice::getSliceCandidates(__pe_a0, __pe_a1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >),
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
    if List::hasOneElement(&slices_lst) {
        var_cref = (slices_lst).head().cloned()?;
        if checkReference(&var_cref, reference)? {
            solve_status = Status::UNPROCESSED.clone();
        } else {
            solve_status = Status::IMPLICIT.clone();
        }
    } else {
        record_parent = BVariable::getParent(BVariable::getVarPointer(
            &var_cref,
            metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBSolve.mo"),
        )?);
        if (record_parent).is_some() {
            (var_cref, solve_status) = getVarSlice(BVariable::getVarName(Util::getOption(record_parent)?), None, eqn)?;
        } else if (slices_lst).is_empty() {
            solve_status = Status::UNSOLVABLE.clone();
        } else {
            filtered = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut c in (slices_lst).into_iter().cloned() {
                    if !(checkReference(&(c.clone()), reference.clone())?) {
                        continue;
                    }
                    let __x = c.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if List::hasOneElement(&filtered) {
                var_cref = (filtered).head().cloned()?;
                solve_status = Status::UNPROCESSED.clone();
            } else {
                solve_status = Status::IMPLICIT.clone();
            }
        }
    }
    Ok((var_cref, solve_status))
}

fn solveForVarSlice(
    mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: BPartition::Kind,
    mut implicit_index: i32,
    mut slicing_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        >,
    >,
    mut varData: &metamodelica::Ref<VarData::VarData>,
    mut eqData: &metamodelica::Ref<EqData::EqData>,
) -> Result<(
    metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    i32,
    Status,
)> {
    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        eqn_slice;
    let mut implicit_index: i32 = implicit_index;
    let mut solve_status: Status;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    let mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    eqn = Pointer::access(Slice::getT(eqn_slice.clone()));
    (var_cref, solve_status) = getVarSlice(
        BVariable::getVarName(Slice::getT(var_slice)),
        Some(cref.clone()),
        eqn.clone(),
    )?;
    if solve_status < Status::IMPLICIT.clone() {
        (eqn, solve_status, implicit_index, _) = solveEquation(
            eqn,
            var_cref,
            funcMap,
            kind,
            implicit_index,
            slicing_map,
            varData,
            eqData,
        )?;
        eqn_slice = metamodelica::Ref::new(Slice::NBSlice {
            t: Pointer::create(eqn),
            indices: metamodelica::nil(),
        });
    } else if solve_status == Status::IMPLICIT.clone() {
        eqn = Equation::map(
            eqn,
            (std::sync::Arc::new({
                let __pe_b1 = var_cref;
                move |__pe_a0| expandArraySumExp(__pe_a0, &__pe_b1)
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
        (eqn, solve_status, implicit_index, _) =
            solveEquation(eqn, cref, funcMap, kind, implicit_index, slicing_map, varData, eqData)?;
        if solve_status < Status::UNSOLVABLE.clone() {
            eqn_slice = metamodelica::Ref::new(Slice::NBSlice {
                t: Pointer::create(eqn),
                indices: metamodelica::nil(),
            });
        } else {
            solve_status = Status::IMPLICIT.clone();
        }
    }
    Ok((eqn_slice, implicit_index, solve_status))
}
