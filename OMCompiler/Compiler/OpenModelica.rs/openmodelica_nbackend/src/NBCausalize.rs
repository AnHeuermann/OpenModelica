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

use crate::NBASSC as ASSC;
use crate::NBAdjacency as Adjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_error::ErrorExt;
use openmodelica_nf_frontend::NFArrayConnections::NameVertexTable;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTypeCheck as TypeCheck;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// util imports
// ############################################################
//                      Main Functions
// ############################################################
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
    mut kind: BPartition::Kind,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::causalizeInterface = getModule()?;
    bdae = (::match_deref::match_deref! { match &((kind, bdae.clone())) {
        (BPartition::Kind::ODE, Deref @ BackendDAE::MAIN { ode: partitions, clocked, varData, eqData, .. }) => {
            let mut partitions = (*partitions).clone();
            let mut clocked = (*clocked).clone();
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (partitions, varData, eqData, _) = applyModule(metamodelica::AsArg::as_arg(&partitions), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), &*(func.clone()), metamodelica::nil())?;
            (clocked, varData, eqData, _) = applyModule(metamodelica::AsArg::as_arg(&clocked), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), &*(func.clone()), metamodelica::nil())?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                ode = partitions.clone(),
                clocked = clocked.clone(),
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        },
        (_, Deref @ BackendDAE::MAIN { init: partitions, varData, eqData, .. }) if (BPartition::kindIsInitial(kind)) => {
            let mut twins: metamodelica::List<metamodelica::Ref<Partition::Partition>>;
            let mut partitions = (*partitions).clone();
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            if Flags::isSet(Flags::INITIALIZATION.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(&(literal!("Balance Initialization")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            (partitions, varData, eqData, twins) = applyModule(metamodelica::AsArg::as_arg(&partitions), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), &*(func.clone()), var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN).clone().unwrap_or(metamodelica::nil()))?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = partitions.clone());
            if (var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init_0 = Some(twins));
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        },
        (BPartition::Kind::DAE, Deref @ BackendDAE::MAIN { dae: Some(partitions), varData, eqData, .. }) => {
            let mut partitions = (*partitions).clone();
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (partitions, varData, eqData, _) = applyModule(metamodelica::AsArg::as_arg(&partitions), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), &fnptr!(causalizeDAEMode, metamodelica::Ref<Partition::Partition>, metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, metamodelica::List<metamodelica::Ref<Partition::Partition>>), metamodelica::nil())?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                dae = Some(partitions.clone()),
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.main")); __mm_s.push_str(&*literal!(" failed with partition type ")); __mm_s.push_str(&*BPartition::Partition::kindToString(kind)?); __mm_s.push_str(&*literal!("!")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn applyModule(
    mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition>>,
    mut kind: BPartition::Kind,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Partition::Partition>,
        metamodelica::Ref<VarData::VarData>,
        metamodelica::Ref<EqData::EqData>,
        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>,
        metamodelica::List<metamodelica::Ref<Partition::Partition>>,
    ) -> Result<(
        metamodelica::Ref<Partition::Partition>,
        metamodelica::Ref<VarData::VarData>,
        metamodelica::Ref<EqData::EqData>,
        metamodelica::List<metamodelica::Ref<Partition::Partition>>,
    )>,
    mut twins: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Partition::Partition>>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
    metamodelica::List<metamodelica::Ref<Partition::Partition>>,
)> {
    let mut new_partitions: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut new_twins: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
    let mut new_partition: metamodelica::Ref<Partition::Partition>;
    let mut paired: metamodelica::List<metamodelica::Ref<Partition::Partition>>;
    let mut unpaired: metamodelica::List<metamodelica::Ref<Partition::Partition>> = twins;
    let mut violated: bool = false;
    for mut partition in &**partitions {
        (paired, unpaired) = List::splitOnTrue(
            &unpaired,
            &({
                let __pe_b1 = partition.index.clone();
                move |__pe_a0| Ok(BPartition::Partition::hasIndex(&__pe_a0, __pe_b1.clone()))
            }),
        )?;
        (new_partition, varData, eqData, paired) = func(partition.clone(), varData, eqData, funcMap.clone(), paired)?;
        new_partitions = if (BPartition::Partition::isEmpty(&new_partition)?) {
            new_partitions
        } else {
            metamodelica::cons(new_partition, new_partitions)
        };
        new_twins = List::append_reverse(
            &({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut twin in (paired).into_iter().cloned() {
                    if !(!(BPartition::Partition::isEmpty(&(twin.clone()))?)) {
                        continue;
                    }
                    let __x = twin.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            new_twins,
        );
    }
    for mut twin in &*unpaired {
        (new_partition, varData, eqData, _) =
            func(twin.clone(), varData, eqData, funcMap.clone(), metamodelica::nil())?;
        new_twins = if (BPartition::Partition::isEmpty(&new_partition)?) {
            new_twins
        } else {
            metamodelica::cons(new_partition, new_twins)
        };
    }
    new_partitions = new_partitions.reverse();
    new_twins = new_twins.reverse();
    if !(BPartition::kindIsInitial(kind)) {
        for mut partition in &*new_partitions {
            violated = checkSystemVariabilities(metamodelica::AsArg::as_arg(&partition))? || violated;
        }
        if violated {
            return Err("fail");
        }
    }
    Ok((new_partitions, varData, eqData, new_twins))
}

pub(crate) fn checkSystemVariabilities(mut partition: &metamodelica::Ref<Partition::Partition>) -> Result<bool> {
    let mut violated: bool = false;
    let mut err: ArcStr;
    if (partition.strongComponents).is_some() {
        let __range0 = partition
            .strongComponents
            .clone()
            .ok_or("pattern mismatch")?
            .borrow()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        for mut scc in __range0 {
            let () = (match &*scc {
                StrongComponent::SINGLE_COMPONENT {
                    eqn: __scc_eqn,
                    var: __scc_var,
                    ..
                } => {
                    let mut ty1: metamodelica::Ref<Type::NFType>;
                    let mut ty2: metamodelica::Ref<Type::NFType>;
                    let mut kind: TypeCheck::MatchKind;
                    ty1 = Type::removeSizeOneArraysAndRecords(Variable::typeOf(&(Pointer::access(__scc_var.clone()))))?;
                    ty2 = Type::removeSizeOneArraysAndRecords(BEquation::Equation::getType(
                        &(Pointer::access(__scc_eqn.clone())),
                        false,
                    )?)?;
                    (_, _, kind) = TypeCheck::matchTypes(
                        ty1.clone(),
                        ty2.clone(),
                        Expression::fromCref(BVariable::getVarName(__scc_var.clone()), false)?,
                        TypeCheck::DEFAULT_OPTIONS.clone(),
                    )?;
                    if kind != TypeCheck::MatchKind::EXACT.clone() {
                        err = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBCausalize.checkSystemVariabilities"));
                            __mm_s.push_str(&*literal!(
                                " failed. The following strong component has conflicting types: "
                            ));
                            __mm_s.push_str(&*Type::toString(&ty1)?);
                            __mm_s.push_str(&*literal!(" != "));
                            __mm_s.push_str(&*Type::toString(&ty2)?);
                            __mm_s.push_str(&*literal!("\n"));
                            __mm_s.push_str(&*StrongComponent::toString(&scc, -1)?);
                            ArcStr::from(__mm_s)
                        };
                        if Flags::isSet(Flags::BLT_DUMP.clone())? {
                            err = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*err);
                                __mm_s.push_str(&*literal!("\n"));
                                __mm_s.push_str(&*BPartition::Partition::toString(partition, 0)?);
                                ArcStr::from(__mm_s)
                            };
                        }
                        Error::addMessage(Error::COMPILER_ERROR.clone(), list![err])?;
                        violated = true;
                    }
                    ()
                }
                _ => (),
            });
        }
    }
    Ok(violated)
}

pub(crate) fn simple(
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut kind: BPartition::Kind,
    mut st: Adjacency::MatrixStrictness,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<(
    metamodelica::Ref<Matching::NBMatching>,
    metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
)> {
    let mut matching: metamodelica::Ref<Matching::NBMatching>;
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    full = Adjacency::Matrix::createFull(vars, eqns, kind)?;
    adj = Adjacency::Matrix::fullToFinal(&full, vars.map.clone(), eqns.map.clone(), eqns, st, iter)?;
    matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), &adj, false, false, true)?;
    adj = Adjacency::Matrix::upgrade(
        adj,
        &full,
        vars.map.clone(),
        eqns.map.clone(),
        eqns,
        Adjacency::MatrixStrictness::SORTING.clone(),
        &(crate::NBEquation::Iterator::interned_EMPTY()),
    )?;
    comps = Sorting::tarjan(adj, matching.clone(), vars, eqns)?;
    Ok((matching, comps))
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Partition::Partition>,
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                >,
                metamodelica::List<metamodelica::Ref<Partition::Partition>>,
            ) -> Result<(
                metamodelica::Ref<Partition::Partition>,
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                metamodelica::List<metamodelica::Ref<Partition::Partition>>,
            )> + 'static,
    >,
> {
    let mut func: Module::causalizeInterface;
    let mut flag: ArcStr = Flags::getConfigString(Flags::MATCHING_ALGORITHM.clone())?;
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "PFPlusExt" => (std::sync::Arc::new(causalizePseudoArray) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Partition::Partition>, metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, metamodelica::List<metamodelica::Ref<Partition::Partition>>) -> Result<(metamodelica::Ref<Partition::Partition>, metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::List<metamodelica::Ref<Partition::Partition>>)> + 'static>),
        Deref @ "pseudo" => (std::sync::Arc::new(causalizePseudoArray) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Partition::Partition>, metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, metamodelica::List<metamodelica::Ref<Partition::Partition>>) -> Result<(metamodelica::Ref<Partition::Partition>, metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::List<metamodelica::Ref<Partition::Partition>>)> + 'static>),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.getModule")); __mm_s.push_str(&*literal!(" failed for unknown option: ")); __mm_s.push_str(&*flag); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

// ############################################################
//                Protected Functions and Types
// ############################################################
fn causalizePseudoArray(
    mut partition: metamodelica::Ref<Partition::Partition>,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut twins: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
) -> Result<(
    metamodelica::Ref<Partition::Partition>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
    metamodelica::List<metamodelica::Ref<Partition::Partition>>,
)> {
    let mut partition: metamodelica::Ref<Partition::Partition> = partition;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut twins: metamodelica::List<metamodelica::Ref<Partition::Partition>> = twins;
    let mut kind: BPartition::Kind = BPartition::Partition::getKind(&partition);
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut adj_matching: metamodelica::Ref<Adjacency::Matrix::Matrix> =
        <metamodelica::Ref<Adjacency::Matrix::Matrix> as ::std::default::Default>::default();
    let mut adj_sorting: metamodelica::Ref<Adjacency::Matrix::Matrix> =
        <metamodelica::Ref<Adjacency::Matrix::Matrix> as ::std::default::Default>::default();
    let mut matching: metamodelica::Ref<Matching::NBMatching>;
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut new_twins: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
    (variables, equations, full, matching, comps) = (match kind {
        mut kind if (BPartition::kindIsInitial(kind)) => {
            let mut fixable: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            let mut unfixable: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            let mut initials: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
            let mut simulation: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
            let mut vo: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
            >;
            let mut vn: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
            >;
            let mut eo: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
            >;
            let mut en: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
            >;
            assign_field!(
                partition.unknowns = BVariable::VariablePointers::compress(partition.unknowns.clone())?,
                partition.equations = BEquation::EquationPointers::compress(partition.equations.clone())?
            );
            (fixable, unfixable) = List::splitOnTrue(
                &(BVariable::VariablePointers::toList(&partition.unknowns)?),
                &BVariable::isFixable,
            )?;
            (initials, simulation) = List::splitOnTrue(
                &(BEquation::EquationPointers::toList(&partition.equations)?),
                &fnptr!(
                    BEquation::Equation::isInitial,
                    Pointer::Pointer<metamodelica::Ref<Equation::Equation>>
                ),
            )?;
            full = Adjacency::Matrix::createFull(&partition.unknowns, &partition.equations, kind)?;
            vn = UnorderedMap::subMap(
                partition.unknowns.map.clone(),
                &({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut var in (unfixable).into_iter().cloned() {
                        let __x = BVariable::getVarName(var.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            en = UnorderedMap::subMap(
                partition.equations.map.clone(),
                &({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut eqn in (initials).into_iter().cloned() {
                        let __x = BEquation::Equation::getEqnName(eqn.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            adj_matching = Adjacency::Matrix::fullToFinal(
                &full,
                vn.clone(),
                en.clone(),
                &partition.equations,
                Adjacency::MatrixStrictness::MATCHING.clone(),
                &(crate::NBEquation::Iterator::interned_EMPTY()),
            )?;
            matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), &adj_matching, true, true, true)?;
            vo = vn;
            eo = en;
            vn = UnorderedMap::new(
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
            en = UnorderedMap::subMap(
                partition.equations.map.clone(),
                &({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut eqn in (simulation).into_iter().cloned() {
                        let __x = BEquation::Equation::getEqnName(eqn.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            (adj_matching, full) = Adjacency::Matrix::expand(
                adj_matching,
                full,
                vo.clone(),
                vn.clone(),
                eo.clone(),
                en.clone(),
                &(partition.unknowns.clone()),
                &(partition.equations.clone()),
                BPartition::Partition::getKind(&partition),
            )?;
            matching = Matching::regular(matching, &adj_matching, true, true, true)?;
            vo = UnorderedMap::merge(
                vo,
                vn,
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBCausalize.mo"),
            )?;
            eo = UnorderedMap::merge(
                eo,
                en,
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBCausalize.mo"),
            )?;
            vn = UnorderedMap::subMap(
                partition.unknowns.map.clone(),
                &({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut var in (fixable).into_iter().cloned() {
                        let __x = BVariable::getVarName(var.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            en = UnorderedMap::new(
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
            (adj_matching, full) = Adjacency::Matrix::expand(
                adj_matching,
                full,
                vo,
                vn,
                eo,
                en,
                &(partition.unknowns.clone()),
                &(partition.equations.clone()),
                BPartition::Partition::getKind(&partition),
            )?;
            (matching, adj_matching, full, variables, equations, varData, eqData) = Matching::singular(
                matching,
                adj_matching,
                full,
                partition.unknowns.clone(),
                partition.equations.clone(),
                funcMap.clone(),
                varData,
                eqData,
                kind,
                false,
                false,
                0,
            )?;
            adj_sorting = Adjacency::Matrix::upgrade(
                adj_matching.clone(),
                &full,
                variables.map.clone(),
                equations.map.clone(),
                &(equations.clone()),
                Adjacency::MatrixStrictness::SORTING.clone(),
                &(crate::NBEquation::Iterator::interned_EMPTY()),
            )?;
            comps = Sorting::tarjan(adj_sorting.clone(), matching.clone(), &variables, &equations)?;
            (variables, equations, full, matching, comps)
        }
        _ => {
            variables = BVariable::VariablePointers::compress(partition.unknowns.clone())?;
            equations = BEquation::EquationPointers::compress(partition.equations.clone())?;
            full = Adjacency::Matrix::createFull(&variables, &equations, kind)?;
            adj_matching = Adjacency::Matrix::fullToFinal(
                &full,
                variables.map.clone(),
                equations.map.clone(),
                &(equations.clone()),
                Adjacency::MatrixStrictness::MATCHING.clone(),
                &(crate::NBEquation::Iterator::interned_EMPTY()),
            )?;
            (matching, adj_matching, full, variables, equations, varData, eqData) = Matching::singular(
                Matching::EMPTY_MATCHING().clone(),
                adj_matching,
                full,
                variables,
                equations,
                funcMap.clone(),
                varData,
                eqData,
                kind,
                false,
                true,
                0,
            )?;
            adj_sorting = Adjacency::Matrix::upgrade(
                adj_matching.clone(),
                &full,
                variables.map.clone(),
                equations.map.clone(),
                &(equations.clone()),
                Adjacency::MatrixStrictness::SORTING.clone(),
                &(crate::NBEquation::Iterator::interned_EMPTY()),
            )?;
            comps = Sorting::tarjan(adj_sorting.clone(), matching.clone(), &variables, &equations)?;
            (variables, equations, full, matching, comps)
        }
    });
    assign_field!(
        partition.unknowns = variables,
        partition.equations = equations,
        partition.adjacencyMatrix = Some(full),
        partition.matching = Some(matching),
        partition.strongComponents = Some(metamodelica::arrayFromVec(comps.into_iter().cloned().collect()))
    );
    for mut twin in &*twins {
        let mut twin = twin.clone();
        (twin, varData, eqData) = causalizeTwin(
            twin,
            &partition,
            &adj_matching,
            &adj_sorting,
            funcMap.clone(),
            varData,
            eqData,
        )?;
        new_twins = metamodelica::cons(twin, new_twins);
    }
    twins = new_twins.reverse();
    Ok((partition, varData, eqData, twins))
}

fn causalizeTwin(
    mut twin: metamodelica::Ref<Partition::Partition>,
    mut seed: &metamodelica::Ref<Partition::Partition>,
    mut seed_matching: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut seed_sorting: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
) -> Result<(
    metamodelica::Ref<Partition::Partition>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
)> {
    let mut twin: metamodelica::Ref<Partition::Partition> = twin;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut no_twins: metamodelica::List<metamodelica::Ref<Partition::Partition>>;
    ErrorExt::setCheckpoint(literal!("NBCausalize.causalizeTwin"));
    match '__try0: {
        (twin, varData, eqData) = unwrap_break_err!(causalizeTwinSeeded(twin.clone(), seed, seed_matching, seed_sorting, funcMap.clone(), varData.clone(), eqData.clone()), '__try0);
        ErrorExt::delCheckpoint(literal!("NBCausalize.causalizeTwin"));
        Ok::<_, &'static str>((eqData.clone(), twin.clone(), varData.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            eqData = __try0_o0;
            twin = __try0_o1;
            varData = __try0_o2;
        }
        Err(_) => {
            ErrorExt::rollBack(literal!("NBCausalize.causalizeTwin"));
            (twin, varData, eqData, no_twins) = causalizePseudoArray(
                twin.clone(),
                varData.clone(),
                eqData.clone(),
                funcMap.clone(),
                metamodelica::nil(),
            )?;
        }
    }
    Ok((twin, varData, eqData))
}

fn causalizeTwinSeeded(
    mut twin: metamodelica::Ref<Partition::Partition>,
    mut seed: &metamodelica::Ref<Partition::Partition>,
    mut seed_matching: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut seed_sorting: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
) -> Result<(
    metamodelica::Ref<Partition::Partition>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
)> {
    let mut twin: metamodelica::Ref<Partition::Partition> = twin;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut kind: BPartition::Kind = BPartition::Partition::getKind(&twin);
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut balanced: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut adj_matching: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut adj_sorting: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut matching: metamodelica::Ref<Matching::NBMatching>;
    let mut seed_index: metamodelica::Array<i32>;
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    assign_field!(
        twin.unknowns = BVariable::VariablePointers::compress(twin.unknowns.clone())?,
        twin.equations = BEquation::EquationPointers::compress(twin.equations.clone())?
    );
    full = Adjacency::Matrix::createFull(&twin.unknowns, &twin.equations, kind)?;
    seed_index = Adjacency::Matrix::equalRows(&seed.equations, &seed.unknowns, &twin.equations, &twin.unknowns)?;
    adj_matching = Adjacency::Matrix::upgradeFrom(
        metamodelica::Ref::new(Adjacency::Matrix::Matrix::EMPTY {
            st: Adjacency::MatrixStrictness::FULL.clone(),
        }),
        &full,
        twin.unknowns.map.clone(),
        twin.equations.map.clone(),
        &(twin.equations.clone()),
        Adjacency::MatrixStrictness::MATCHING.clone(),
        seed_matching,
        seed_index.clone(),
    )?;
    matching = Matching::fromSeed(seed, &adj_matching, &twin.unknowns, &twin.equations)?;
    (matching, adj_matching, balanced, variables, equations, varData, eqData) = Matching::singular(
        matching,
        adj_matching,
        full.clone(),
        twin.unknowns.clone(),
        twin.equations.clone(),
        funcMap,
        varData,
        eqData,
        kind,
        false,
        false,
        0,
    )?;
    if referenceEq(&*(&*balanced), &*(&*full)) {
        adj_sorting = Adjacency::Matrix::upgradeFrom(
            adj_matching,
            &full,
            variables.map.clone(),
            equations.map.clone(),
            &(equations.clone()),
            Adjacency::MatrixStrictness::SORTING.clone(),
            seed_sorting,
            seed_index.clone(),
        )?;
    } else {
        full = balanced;
        adj_sorting = Adjacency::Matrix::upgrade(
            adj_matching,
            &full,
            variables.map.clone(),
            equations.map.clone(),
            &(equations.clone()),
            Adjacency::MatrixStrictness::SORTING.clone(),
            &(crate::NBEquation::Iterator::interned_EMPTY()),
        )?;
    }
    comps = Sorting::tarjan(adj_sorting, matching.clone(), &variables, &equations)?;
    assign_field!(
        twin.unknowns = variables,
        twin.equations = equations,
        twin.adjacencyMatrix = Some(full),
        twin.matching = Some(matching),
        twin.strongComponents = Some(metamodelica::arrayFromVec(comps.into_iter().cloned().collect()))
    );
    Ok((twin, varData, eqData))
}

fn causalizeDAEMode(
    mut partition: metamodelica::Ref<Partition::Partition>,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut twins: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
) -> (
    metamodelica::Ref<Partition::Partition>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
    metamodelica::List<metamodelica::Ref<Partition::Partition>>,
) {
    let mut partition: metamodelica::Ref<Partition::Partition> = partition;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut twins: metamodelica::List<metamodelica::Ref<Partition::Partition>> = twins;
    (partition, varData, eqData, twins)
}
