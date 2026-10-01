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
use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBFunctionAlias::Call_Aux;
use crate::NBInitialization as Initialization;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_nf_frontend::NFPrefixes;
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
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// NF imports
// NB imports
// util imports
pub(crate) fn indexReduction(
    mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: NBPartition::Kind,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<NFFunction::Function::Function>>,
    >,
    mut matching: metamodelica::Ref<Matching::NBMatching>,
    mut mapping_opt: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>,
) -> Result<(
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
    bool,
)> {
    pub(crate) type SliceSet = metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;

    let mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix> = adj;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut changed: bool;
    let mut mapping: metamodelica::Ref<Adjacency::Mapping::Mapping>;
    let mut excluded_eqns: metamodelica::Array<bool>;
    let mut msss: metamodelica::Array<metamodelica::List<i32>>;
    let mut marked_eqns: metamodelica::List<i32>;
    let mut constraint: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut diffed_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut states: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut dummy_states: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut sliced_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut sliced_dummy_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut state_derivatives: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut dummy_derivatives: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut dummy_slice_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut current_candidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut rest_candidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut constraint_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut matched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut unmatched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut new_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
    let mut der_alias_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut der_aliases: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut diffArguments: metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>;
    let mut diffArguments_ptr: Pointer::Pointer<
        metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>,
    >;
    let mut candidate_ptrs: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut constraint_ptrs: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut set_adj: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut full_local: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut set_matching: metamodelica::Ref<Matching::NBMatching>;
    let mut vo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut vn: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut eo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut en: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut stages: metamodelica::List<(ArcStr, BVariable::checkVar)>;
    let mut stageFunc: BVariable::checkVar;
    let mut stageStr: ArcStr;
    let mut slice_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
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
    let mut dummy_slice_set: metamodelica::Ref<
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
    let mut alias_subst: metamodelica::Ref<
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
    let mut alias_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut debug: bool = false;
    mapping = (::match_deref::match_deref! { match &(mapping_opt) {
        Some(__esc_mapping) => {
            mapping = (*__esc_mapping).clone();
            mapping.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction")); __mm_s.push_str(&*literal!(" failed because no mapping was provided.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    excluded_eqns = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut eqn in (EquationPointers::toList(&equations)?).into_iter().cloned() {
                let __x = Equation::isDiscrete(eqn.clone()) || Equation::hasDerivative(eqn.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    msss = (match &*adj {
        Adjacency::Matrix::FINAL {
            m: __adj_m,
            mT: __adj_mT,
            ..
        } => getMSSS(
            metamodelica::AsArg::as_arg(&__adj_m),
            metamodelica::AsArg::as_arg(&__adj_mT),
            &matching,
            excluded_eqns.clone(),
            &mapping,
        )?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction"));
                    __mm_s.push_str(&*literal!(" expected final matrix as adj input but got :\n"));
                    __mm_s.push_str(&*Adjacency::Matrix::toString(&adj, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    if !(metamodelica::arrayLength(msss.clone()) == 0) {
        changed = true;
        marked_eqns = UnorderedSet::unique_list(
            List::flatten(msss.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?,
            std::sync::Arc::new(fnptr!(Util::id, _)),
            (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        )?;
        (constraint_ptrs, candidate_ptrs, constraint_eqns) =
            getConstraintsAndCandidates(&equations, &marked_eqns, &mapping)?;
        for mut eq in &*constraint_eqns {
            UnorderedMap::add(
                Equation::getEqnName(Slice::getT(eq.clone()))?,
                UnorderedSet::fromList(
                    &eq.indices,
                    std::sync::Arc::new(fnptr!(Util::id, _)),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                )?,
                slice_map.clone(),
            )?;
        }
        (candidate_ptrs, der_aliases, der_alias_eqns) = aliasStateDerivatives(
            candidate_ptrs,
            &constraint_ptrs,
            BVariable::VarData::getUniqueIndex(&varData)?,
        )?;
        if !((der_aliases).is_empty()) {
            varData = BVariable::VarData::addTypedList(
                varData,
                &der_aliases,
                BVariable::VarData::VarType::ALGEBRAIC.clone(),
            )?;
            variables = BVariable::VariablePointers::addList(&der_aliases, variables)?;
            new_eqns = listAppend(der_alias_eqns, new_eqns);
        }
        if BVariable::VariablePointers::scalarSize(&candidate_ptrs, false)?
            < ({
                let mut __acc: i32 = 0;
                for mut eq in (constraint_eqns).into_iter().cloned() {
                    let __x = Slice::size(
                        eq.clone(),
                        &({
                            let __pe_b1 = true;
                            move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
                        }),
                    )?;
                    __acc += __x;
                }
                __acc
            })
        {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction"));
                    __mm_s.push_str(&*literal!(" failed because there was not enough state candidates to balance out the constraint equations.\n"));
                    __mm_s.push_str(&*EquationPointers::toString(
                        &constraint_ptrs,
                        literal!("Constraint"),
                        None,
                        true,
                        None,
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*BVariable::VariablePointers::toString(
                        &candidate_ptrs,
                        literal!("State Candidate"),
                        None,
                        true,
                    )?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_1(&(literal!("Index Reduction")))?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*BVariable::VariablePointers::toString(
                    &candidate_ptrs,
                    literal!("State Candidate"),
                    None,
                    true,
                )?);
                __mm_s.push_str(&*EquationPointers::toString(
                    &constraint_ptrs,
                    literal!("Constraint"),
                    None,
                    true,
                    None,
                )?);
                ArcStr::from(__mm_s)
            });
        }
        full_local = Adjacency::Matrix::createFull(&candidate_ptrs, &constraint_ptrs, kind)?;
        set_adj = metamodelica::Ref::new(Adjacency::Matrix::Matrix::EMPTY {
            st: Adjacency::MatrixStrictness::LINEAR.clone(),
        });
        rest_candidates = BVariable::VariablePointers::toList(&candidate_ptrs)?;
        eo = constraint_ptrs.map.clone();
        en = UnorderedMap::new(
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
        vo = UnorderedMap::new(
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
        vn = UnorderedMap::new(
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
        set_matching = Matching::EMPTY_MATCHING().clone();
        stages = list![
            (
                literal!("1. StateSelect.NEVER"),
                (std::sync::Arc::new({
                    let __pe_b1 = StateSelect::NEVER.clone();
                    move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone()))
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >)
            ),
            (
                literal!("2. StateSelect.AVOID"),
                (std::sync::Arc::new({
                    let __pe_b1 = StateSelect::AVOID.clone();
                    move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone()))
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >)
            ),
            (
                literal!("3. Artificial Variables"),
                (std::sync::Arc::new(BVariable::isArtificial)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >)
            ),
            (
                literal!("4. StateSelect.DEFAULT"),
                (std::sync::Arc::new({
                    let __pe_b1 = StateSelect::DEFAULT.clone();
                    move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone()))
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >)
            ),
            (
                literal!("5. StateSelect.PREFER"),
                (std::sync::Arc::new({
                    let __pe_b1 = StateSelect::PREFER.clone();
                    move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone()))
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >)
            )
        ];
        for mut stage in &*stages {
            (stageStr, stageFunc) = stage.clone();
            (current_candidates, rest_candidates) = List::splitOnTrue(&rest_candidates, &*(stageFunc.clone()))?;
            if (current_candidates).is_empty() {
                if debug {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("Nothing done for ("));
                                __mm_s.push_str(&*stageStr);
                                __mm_s.push_str(&*literal!(") Index Reduction"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
            } else {
                vo = UnorderedMap::merge(
                    vo,
                    UnorderedMap::copy(vn),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
                )?;
                vn = UnorderedMap::subMap(
                    candidate_ptrs.map.clone(),
                    &({
                        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                            metamodelica::nil();
                        for mut var in (current_candidates).into_iter().cloned() {
                            let __x = BVariable::getVarName(var.clone());
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?;
                (set_adj, full_local) = Adjacency::Matrix::expand(
                    set_adj,
                    full_local,
                    vo.clone(),
                    vn.clone(),
                    eo.clone(),
                    en.clone(),
                    &candidate_ptrs,
                    &constraint_ptrs,
                    kind,
                )?;
                set_matching = Matching::regular(set_matching, &set_adj, false, true, false)?;
                if debug {
                    metamodelica::print(Adjacency::Matrix::toString(&set_adj, {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("("));
                        __mm_s.push_str(&*stageStr);
                        __mm_s.push_str(&*literal!(") Index Reduction"));
                        ArcStr::from(__mm_s)
                    })?);
                    metamodelica::print(Matching::toString(&set_matching, {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("("));
                        __mm_s.push_str(&*stageStr);
                        __mm_s.push_str(&*literal!(") Index Reduction"));
                        ArcStr::from(__mm_s)
                    })?);
                }
                if Matching::isEmpty(&set_matching) && Matching::isPerfect(&set_matching)? {
                    if debug {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*StringUtil::headline_2(
                                &({
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("Finished with perfect matching in stage "));
                                    __mm_s.push_str(&*stageStr);
                                    __mm_s.push_str(&*literal!("."));
                                    ArcStr::from(__mm_s)
                                }),
                            )?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    break;
                }
            }
        }
        (dummy_states, states, matched_eqns, unmatched_eqns) = Matching::getMatches(
            &set_matching,
            Adjacency::Matrix::getMappingOpt(&set_adj),
            &candidate_ptrs,
            &constraint_ptrs,
        )?;
        unmatched_eqns = resolveSlicedUnmatched(&unmatched_eqns, slice_map.clone())?;
        dummy_states = resolveSlicedDummyStates(dummy_states, states.clone())?;
        (states, alias_eqns) = resolveSlicedCandidates(
            states,
            alias_subst.clone(),
            BVariable::VarData::getUniqueIndex(&varData)?,
            BVariable::VarData::getUniqueIndex(&varData)?,
        )?;
        if !(UnorderedMap::isEmpty(alias_subst.clone())) {
            for mut constraint in &*EquationPointers::toList(&constraint_ptrs)? {
                let mut constraint = constraint.clone();
                substituteSlicedDummyEqn(constraint, alias_subst.clone())?;
            }
        }
        new_eqns = listAppend(alias_eqns.clone(), new_eqns);
        for mut eqn in &*alias_eqns {
            UnorderedMap::add(
                Equation::getEqnName(eqn.clone())?,
                UnorderedSet::new(
                    std::sync::Arc::new(fnptr!(Util::id, _)),
                    (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    13,
                ),
                slice_map.clone(),
            )?;
        }
        constraint_ptrs = EquationPointers::addList(&alias_eqns, constraint_ptrs)?;
        diffArguments =
            Differentiate::DifferentiationArguments::default(Differentiate::DifferentiationType::TIME.clone(), funcMap);
        assign_field!(diffArguments.diff_map = Some(BVariable::VarData::getStateOrder(&varData)?));
        diffArguments_ptr = Pointer::create(diffArguments);
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            metamodelica::print(StringUtil::headline_3(
                &(literal!("[dummyselect] 1. Differentiate the constraint equations")),
            )?);
        }
        for mut constraint in &*EquationPointers::toList(&constraint_ptrs)? {
            let mut constraint = constraint.clone();
            diffed_eqn = Differentiate::differentiateEquationPointer(
                constraint.clone(),
                diffArguments_ptr.clone(),
                &(literal!("")),
            )?;
            diffed_eqn = removeSlicedDerivatives(
                diffed_eqn,
                UnorderedMap::getSafe(
                    Equation::getEqnName(constraint.clone())?,
                    slice_map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
                )?,
                dummy_slice_set.clone(),
                BVariable::VarData::getUniqueIndex(&varData)?,
            )?;
            new_eqns = metamodelica::cons(diffed_eqn.clone(), new_eqns);
            if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[dummyselect] constraint eqn:\t\t"));
                    __mm_s.push_str(&*Equation::toString(Pointer::access(constraint), literal!(""))?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[dummyselect] differentiated eqn:\t"));
                    __mm_s.push_str(&*Equation::toString(Pointer::access(diffed_eqn), literal!(""))?);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
        diffArguments = Pointer::access(diffArguments_ptr);
        for mut dummy in &*dummy_states {
            if (dummy.indices).is_empty() {
                dummy_derivatives = metamodelica::cons(
                    BVariable::makeDummyState(Slice::getT(dummy.clone()))?,
                    dummy_derivatives,
                );
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction"));
                        __mm_s.push_str(&*literal!(
                            " failed because slicing during index reduction is not yet supported.\n"
                        ));
                        __mm_s.push_str(&*Slice::toString(dummy.clone(), &BVariable::pointerToString, 10)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        }
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            metamodelica::print(StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[dummyselect] ("));
                    __mm_s.push_str(&*intString(((states).len() as i32)));
                    __mm_s.push_str(&*literal!(") Selected States"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*Slice::lstToString(
                    states.clone(),
                    (std::sync::Arc::new(BVariable::pointerToString)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                ) -> Result<ArcStr>
                                + 'static,
                        >),
                    literal!(""),
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
            metamodelica::print(StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[stateselection] ("));
                    __mm_s.push_str(&*intString(((diffArguments.new_vars).len() as i32)));
                    __mm_s.push_str(&*literal!(") State Derivatives Created by Differentiation"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    diffArguments.new_vars.clone(),
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[stateselection] ("));
                    __mm_s.push_str(&*intString(((dummy_states).len() as i32)));
                    __mm_s.push_str(&*literal!(") Selected Dummy States"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*Slice::lstToString(
                    dummy_states.clone(),
                    (std::sync::Arc::new(BVariable::pointerToString)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                ) -> Result<ArcStr>
                                + 'static,
                        >),
                    literal!(""),
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        if (unmatched_eqns).is_empty() {
            if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &(literal!("\t STATIC STATE SELECTION\n\t(no unmatched equations)")),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        } else {
            if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
                metamodelica::print(toStringDynamicSelect(dummy_states.clone(), unmatched_eqns)?);
            }
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction"));
                    __mm_s.push_str(&*literal!(
                        " failed because dynamic state selection is not yet supported."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        (state_derivatives, _) = List::extractOnTrue(
            &diffArguments.new_vars,
            &fnptr!(
                BVariable::isStateDerivative,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ),
        )?;
        sliced_states = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut slice in (states).into_iter().cloned() {
                let __x = Slice::getT(slice.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        varData =
            BVariable::VarData::addTypedList(varData, &sliced_states, BVariable::VarData::VarType::STATE.clone())?;
        varData = BVariable::VarData::addTypedList(
            varData,
            &state_derivatives,
            BVariable::VarData::VarType::STATE_DER.clone(),
        )?;
        sliced_dummy_states = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut slice in (dummy_states).into_iter().cloned() {
                let __x = Slice::getT(slice.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        varData = BVariable::VarData::addTypedList(
            varData,
            &sliced_dummy_states,
            BVariable::VarData::VarType::ALGEBRAIC.clone(),
        )?;
        varData = BVariable::VarData::addTypedList(
            varData,
            &dummy_derivatives,
            BVariable::VarData::VarType::ALGEBRAIC.clone(),
        )?;
        eqData = EqData::addTypedList(eqData, &new_eqns, EqData::EqType::CONTINUOUS.clone(), true)?;
        variables = BVariable::VariablePointers::addList(&diffArguments.new_vars, variables)?;
        variables = BVariable::VariablePointers::addList(&sliced_dummy_states, variables)?;
        variables = BVariable::VariablePointers::removeList(&sliced_states, variables)?;
        equations = EquationPointers::addList(&new_eqns, equations)?;
        dummy_slice_vars = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut cref in (UnorderedSet::toList(dummy_slice_set)).into_iter().cloned() {
                let __x = BVariable::getVarPointer(
                    &(cref.clone()),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        varData = BVariable::VarData::addTypedList(
            varData,
            &dummy_slice_vars,
            BVariable::VarData::VarType::ALGEBRAIC.clone(),
        )?;
        variables = BVariable::VariablePointers::addList(&dummy_slice_vars, variables)?;
    } else {
        changed = false;
    }
    Ok((adj, full, variables, equations, varData, eqData, changed))
}

pub(crate) fn balanceInitialization(
    mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: NBPartition::Kind,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<NFFunction::Function::Function>>,
    >,
    mut matching: metamodelica::Ref<Matching::NBMatching>,
    mut mapping_opt: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>,
) -> Result<(
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
    bool,
)> {
    let mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix> = adj;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut changed: bool;
    let mut unmatched_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut unmatched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut start_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut failed_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut sliced_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut start_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut kept_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut remaining: metamodelica::List<i32>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut ptr_start_vars: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut ptr_start_eqns: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    > = Pointer::create(metamodelica::nil());
    let mut idx: Pointer::Pointer<i32>;
    let mut error_msg: ArcStr;
    let mut vo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut vn: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut eo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    let mut en: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
    (_, unmatched_vars, _, unmatched_eqns) =
        Matching::getMatches(&matching, mapping_opt.clone(), &variables, &equations)?;
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        metamodelica::print(toStringUnmatched(unmatched_vars.clone(), unmatched_eqns.clone())?);
    }
    if !((unmatched_vars).is_empty() && (unmatched_eqns).is_empty()) {
        changed = true;
        if !((unmatched_eqns).is_empty()) {
            Error::addMessage(
                Error::COMPILER_WARNING.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBResolveSingularities.balanceInitialization"));
                    __mm_s.push_str(&*literal!(" reports an overdetermined initialization!\nChecking for consistency is not yet supported, following equations had to be removed:\n"));
                    __mm_s.push_str(&*Slice::lstToString(
                        unmatched_eqns.clone(),
                        (std::sync::Arc::new({
                            let __pe_b1 = literal!("");
                            move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                                    ) -> Result<ArcStr>
                                    + 'static,
                            >),
                        literal!(""),
                        10,
                    )?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            eo = UnorderedMap::copy(equations.map.clone());
            sliced_eqns = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                    metamodelica::nil();
                for mut eqn in (unmatched_eqns.clone()).into_iter().cloned() {
                    let __x = Slice::getT(eqn.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            equations = EquationPointers::removeList(&sliced_eqns, equations)?;
            kept_eqns = metamodelica::nil();
            for mut eqn_slice in &*unmatched_eqns {
                if !((eqn_slice.indices).is_empty()) && Equation::isForEquation(Slice::getT(eqn_slice.clone())) {
                    remaining = ({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut i in (0..=Equation::size(Slice::getT(eqn_slice.clone()), false)? - 1).into_iter() {
                            if !(!(List::contains(&eqn_slice.indices, i.clone(), &fnptr!(intEq, i32, i32))?)) {
                                continue;
                            }
                            let __x = i.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    (sliced_eqns, _) = Equation::slice(Slice::getT(eqn_slice.clone()), remaining)?;
                    kept_eqns = listAppend(sliced_eqns, kept_eqns);
                }
            }
            if (kept_eqns).is_empty() {
                (adj, full) = Adjacency::Matrix::compress(adj, full, &equations, &variables, eo)?;
            } else {
                equations = EquationPointers::addList(&kept_eqns, equations)?;
                full = Adjacency::Matrix::createFull(&variables, &equations, kind)?;
                adj = Adjacency::Matrix::fullToFinal(
                    &full,
                    variables.map.clone(),
                    equations.map.clone(),
                    &(equations.clone()),
                    Adjacency::MatrixStrictness::MATCHING.clone(),
                    &(crate::NBEquation::Iterator::interned_EMPTY()),
                )?;
            }
        }
        idx = EqData::getUniqueIndex(&eqData)?;
        for mut var in &*unmatched_vars {
            var_ptr = Slice::getT(var.clone());
            if BVariable::isFixable(var_ptr.clone())? {
                Initialization::createStartEquationSlice(
                    var.clone(),
                    ptr_start_vars.clone(),
                    ptr_start_eqns.clone(),
                    idx.clone(),
                    true,
                )?;
            } else {
                failed_vars = metamodelica::cons(var_ptr, failed_vars);
            }
        }
        if (failed_vars).is_empty() {
            start_vars = Pointer::access(ptr_start_vars);
            start_eqns = Pointer::access(ptr_start_eqns);
            vo = variables.map.clone();
            eo = UnorderedMap::copy(equations.map.clone());
            varData = BVariable::VarData::addTypedList(varData, &start_vars, VarData::VarType::START.clone())?;
            eqData = EqData::addTypedList(eqData, &start_eqns, EqData::EqType::INITIAL.clone(), true)?;
            equations = EquationPointers::addList(&start_eqns, equations)?;
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
                equations.map.clone(),
                &({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut eqn in (start_eqns.clone()).into_iter().cloned() {
                        let __x = Equation::getEqnName(eqn.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            (adj, full) = Adjacency::Matrix::expand(adj, full, vo, vn, eo, en, &variables, &equations, kind)?;
            if Flags::isSet(Flags::INITIALIZATION.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*List::toStringCustom(
                        start_eqns.clone(),
                        &({
                            let __pe_b1 = literal!("");
                            move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                        }),
                        StringUtil::headline_4(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s
                                    .push_str(&*literal!("Created Start Equations for balancing the Initialization ("));
                                __mm_s.push_str(&*intString(((start_eqns).len() as i32)));
                                __mm_s.push_str(&*literal!("):"));
                                ArcStr::from(__mm_s)
                            }),
                        )?,
                        literal!("\t"),
                        literal!("\n\t"),
                        literal!(""),
                        false,
                        0,
                    )?);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
        } else {
            error_msg = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBResolveSingularities.balanceInitialization"));
                __mm_s.push_str(&*literal!(
                    " failed because following non-fixable variables could not be solved:\n"
                ));
                __mm_s.push_str(&*List::toString(
                    failed_vars,
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            if Flags::isSet(Flags::INITIALIZATION.clone())? {
                error_msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*error_msg);
                    __mm_s.push_str(&*literal!("\nFollowing equations were created by fixing variables:\n"));
                    __mm_s.push_str(&*List::toString(
                        Pointer::access(ptr_start_eqns),
                        &({
                            let __pe_b1 = literal!("\t");
                            move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                        }),
                        List::Style::NEWLINE_TAB.clone(),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            } else {
                error_msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*error_msg);
                    __mm_s.push_str(&*literal!("\nUse -d=initialization for more debug output."));
                    ArcStr::from(__mm_s)
                };
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                error_msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*error_msg);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*BVariable::VariablePointers::toString(
                        &variables,
                        literal!("All"),
                        None,
                        true,
                    )?);
                    __mm_s.push_str(&*EquationPointers::toString(
                        &equations,
                        literal!("All"),
                        None,
                        true,
                        None,
                    )?);
                    __mm_s.push_str(&*Adjacency::Mapping::toString(
                        &(Util::getOptionOrDefault(mapping_opt, Adjacency::Mapping::empty())),
                    )?);
                    __mm_s.push_str(&*Adjacency::Matrix::toString(&adj, literal!(""))?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*Matching::toString(&matching, literal!(""))?);
                    ArcStr::from(__mm_s)
                };
            } else {
                error_msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*error_msg);
                    __mm_s.push_str(&*literal!("\nUse -d=bltdump for more verbose debug output."));
                    ArcStr::from(__mm_s)
                };
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![error_msg])?;
            return Err("fail");
        }
    } else {
        changed = false;
    }
    Ok((adj, full, variables, equations, varData, eqData, changed))
}

fn getMSSS(
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut mT: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut matching: &metamodelica::Ref<Matching::NBMatching>,
    mut excluded_eqns: metamodelica::Array<bool>,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let __ab_excluded_eqns = excluded_eqns.borrow();
    let mut msss: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqn_candidates: metamodelica::List<i32> = metamodelica::nil();
    let mut color_clustering: metamodelica::Array<i32>;
    let mut eqn_coloring: metamodelica::Array<i32> = arrayCreate(Adjacency::IntMatrix::rows(m), -1);
    let mut var_coloring: metamodelica::Array<i32> = arrayCreate(Adjacency::IntMatrix::rows(mT), -1);
    let mut color: i32 = 0;
    for mut eqn in 1..=metamodelica::arrayLength(matching.eqn_to_var.clone()) {
        if ({
            let __elt = (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), eqn)?).clone();
            __elt
        }) == -1
        {
            eqn_candidates = metamodelica::cons(eqn, eqn_candidates);
        }
    }
    color_clustering = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (1..=((eqn_candidates).len() as i32)).into_iter() {
                let __x = i.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    for mut eqn in &*eqn_candidates {
        if ({
            let __elt = (*metamodelica::index_checked(&eqn_coloring.borrow(), eqn.clone())?).clone();
            __elt
        }) == -1
        {
            color = color + 1;
            fillColorEqn(
                eqn.clone(),
                color,
                eqn_coloring.clone(),
                var_coloring.clone(),
                color_clustering.clone(),
                m,
                mT,
                matching,
                mapping,
            )?;
        }
    }
    resolveClustering(color_clustering.clone())?;
    msss = arrayCreate(color, metamodelica::nil());
    for mut eqn in 1..=metamodelica::arrayLength(eqn_coloring.clone()) {
        if ({
            let __elt = (*metamodelica::index_checked(&eqn_coloring.borrow(), eqn)?).clone();
            __elt
        }) != -1
            && !((*metamodelica::index_checked(
                &__ab_excluded_eqns,
                ({
                    let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn)?).clone();
                    __elt
                }),
            )?)
            .clone())
        {
            color = ({
                let __elt = (*metamodelica::index_checked(
                    &color_clustering.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(&eqn_coloring.borrow(), eqn)?).clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            });
            {
                let __cell0 = metamodelica::cons(
                    eqn,
                    ({
                        let __elt = (*metamodelica::index_checked(&msss.borrow(), color)?).clone();
                        __elt
                    }),
                );
                let __idx0 = color;
                *metamodelica::index_mut_checked(&mut msss.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
    }
    msss = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
            for mut ms in (msss.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())
                .into_iter()
                .cloned()
            {
                if !(!((ms).is_empty())) {
                    continue;
                }
                let __x = ms.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    Ok(msss)
}

fn fillColorEqn(
    mut eqn: i32,
    mut color: i32,
    mut eqn_coloring: metamodelica::Array<i32>,
    mut var_coloring: metamodelica::Array<i32>,
    mut color_clustering: metamodelica::Array<i32>,
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut mT: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut matching: &metamodelica::Ref<Matching::NBMatching>,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
) -> Result<()> {
    let mut data: metamodelica::Array<i32> = Adjacency::IntMatrix::entries(m);
    let mut first: i32 = ({
        let __elt = (*metamodelica::index_checked(&m.start.borrow(), eqn)?).clone();
        __elt
    });
    metamodelica::arrayUpdate(eqn_coloring.clone(), eqn, color)?;
    let __range0 = first
        ..=first
            + ({
                let __elt = (*metamodelica::index_checked(&m.len.borrow(), eqn)?).clone();
                __elt
            })
            - 1;
    for mut k in __range0 {
        fillColorVar(
            ({
                let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                __elt
            }),
            color,
            eqn_coloring.clone(),
            var_coloring.clone(),
            color_clustering.clone(),
            m,
            mT,
            matching,
            mapping,
        )?;
    }
    Ok(())
}

fn fillColorVar(
    mut var: i32,
    mut color: i32,
    mut eqn_coloring: metamodelica::Array<i32>,
    mut var_coloring: metamodelica::Array<i32>,
    mut color_clustering: metamodelica::Array<i32>,
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut mT: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut matching: &metamodelica::Ref<Matching::NBMatching>,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
) -> Result<()> {
    let mut eqn: i32 = ({
        let __elt = (*metamodelica::index_checked(&matching.var_to_eqn.borrow(), var)?).clone();
        __elt
    });
    if ({
        let __elt = (*metamodelica::index_checked(&var_coloring.borrow(), var)?).clone();
        __elt
    }) == -1
    {
        metamodelica::arrayUpdate(var_coloring.clone(), var, color)?;
        if eqn != -1 {
            if ({
                let __elt = (*metamodelica::index_checked(&eqn_coloring.borrow(), eqn)?).clone();
                __elt
            }) == -1
            {
                fillColorEqn(
                    eqn,
                    color,
                    eqn_coloring.clone(),
                    var_coloring.clone(),
                    color_clustering.clone(),
                    m,
                    mT,
                    matching,
                    mapping,
                )?;
            }
        }
    } else {
        colorClustering(
            ({
                let __elt = (*metamodelica::index_checked(&var_coloring.borrow(), var)?).clone();
                __elt
            }),
            color,
            color_clustering.clone(),
        )?;
    }
    Ok(())
}

fn colorClustering(
    mut old_color: i32,
    mut new_color: i32,
    mut color_clustering: metamodelica::Array<i32>,
) -> Result<()> {
    if ({
        let __elt = (*metamodelica::index_checked(&color_clustering.borrow(), old_color)?).clone();
        __elt
    }) != old_color
    {
        colorClustering(
            ({
                let __elt = (*metamodelica::index_checked(&color_clustering.borrow(), old_color)?).clone();
                __elt
            }),
            new_color,
            color_clustering.clone(),
        )?;
    }
    metamodelica::arrayUpdate(color_clustering.clone(), old_color, new_color)?;
    Ok(())
}

fn resolveClustering(mut color_clustering: metamodelica::Array<i32>) -> Result<()> {
    let mut color: i32;
    for mut i in 1..=metamodelica::arrayLength(color_clustering.clone()) {
        color = i;
        while ({
            let __elt = (*metamodelica::index_checked(&color_clustering.borrow(), color)?).clone();
            __elt
        }) != color
        {
            color = ({
                let __elt = (*metamodelica::index_checked(&color_clustering.borrow(), color)?).clone();
                __elt
            });
        }
        metamodelica::arrayUpdate(color_clustering.clone(), i, color)?;
    }
    Ok(())
}

fn aliasStateDerivatives(
    mut candidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut constraints: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut uniqueIndex: Pointer::Pointer<i32>,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut candidates: metamodelica::Ref<VariablePointers::VariablePointers> = candidates;
    let mut aliases: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut alias_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut subst: metamodelica::Ref<
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
    let mut ders: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut alias_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut alias_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    ders = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut v in (BVariable::VariablePointers::toList(&candidates)?).into_iter().cloned() {
            if !(BVariable::isStateDerivative(v.clone())) {
                continue;
            }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (ders).is_empty() {
        return Ok((candidates, aliases, alias_eqns));
    }
    for mut der_var in &*ders {
        der_cref = BVariable::getVarName(der_var.clone());
        (alias_var, alias_cref) = BVariable::makeAuxVar(
            &(arcstr::literal!(BVariable::DUMMY_ALIAS_STR)),
            Pointer::access(uniqueIndex.clone()),
            Variable::typeOf(&(Pointer::access(der_var.clone()))),
            false,
        )?;
        Pointer::update(uniqueIndex.clone(), Pointer::access(uniqueIndex.clone()) + 1);
        alias_eqns = metamodelica::cons(
            Equation::makeAssignment(
                Expression::fromCref(alias_cref.clone(), false)?,
                Expression::fromCref(der_cref.clone(), false)?,
                uniqueIndex.clone(),
                &(literal!("DUM")),
                crate::NBEquation::Iterator::interned_EMPTY(),
                NBEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None),
            )?,
            alias_eqns,
        );
        UnorderedMap::add(der_cref, alias_cref, subst.clone())?;
        aliases = metamodelica::cons(alias_var, aliases);
    }
    candidates = BVariable::VariablePointers::compress(BVariable::VariablePointers::addList(
        &aliases,
        BVariable::VariablePointers::removeList(&ders, candidates)?,
    )?)?;
    for mut constraint in &*EquationPointers::toList(constraints)? {
        Pointer::update(
            constraint.clone(),
            Equation::map(
                Pointer::access(constraint.clone()),
                (std::sync::Arc::new({
                    let __pe_b1 = subst.clone();
                    move |__pe_a0| substituteDerivativeAlias(__pe_a0, __pe_b1.clone())
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
            )?,
        );
    }
    Ok((candidates, aliases, alias_eqns))
}

fn substituteDerivativeAlias(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut subst: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (UnorderedMap::contains(
                ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref)),
                subst.clone(),
            )?) =>
        {
            let mut alias_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            alias_cref = UnorderedMap::getSafe(
                ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref)),
                subst.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
            )?;
            Expression::fromCref(
                ComponentRef::copySubscripts(metamodelica::AsArg::as_arg(&__exp_cref), alias_cref)?,
                false,
            )?
        }
        _ => exp,
    });
    Ok(exp)
}

fn getConstraintsAndCandidates(
    mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut marked_eqns: &metamodelica::List<i32>,
    mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
) -> Result<(
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
)> {
    let mut constr: metamodelica::Ref<EquationPointers::EquationPointers> =
        EquationPointers::empty(BaseHashTable::bigBucketSize.clone());
    let mut states: metamodelica::Ref<VariablePointers::VariablePointers> =
        BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false);
    let mut sliced_constr: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    > = metamodelica::nil();
    let mut eqn_indices: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        13,
    );
    let mut eqn_slices: metamodelica::Array<metamodelica::List<i32>> =
        arrayCreate(EquationPointers::size(equations), metamodelica::nil());
    let mut state_candidates: metamodelica::Ref<
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
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    for mut eqn in &**marked_eqns {
        UnorderedSet::add(
            ({
                let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn.clone())?).clone();
                __elt
            }),
            eqn_indices.clone(),
        )?;
        {
            let __cell0 = metamodelica::cons(
                eqn.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(
                        &eqn_slices.borrow(),
                        ({
                            let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn.clone())?).clone();
                            __elt
                        }),
                    )?)
                    .clone();
                    __elt
                }),
            );
            let __idx0 = ({
                let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn.clone())?).clone();
                __elt
            });
            *metamodelica::index_mut_checked(&mut eqn_slices.clone().borrow_mut(), __idx0)? = __cell0;
        }
    }
    for mut eqn in &*UnorderedSet::toList(eqn_indices) {
        eqn_ptr = EquationPointers::getEqnAt(equations, eqn.clone())?;
        constr = EquationPointers::add(eqn_ptr.clone(), constr)?;
        sliced_constr = metamodelica::cons(
            metamodelica::Ref::new(Slice::NBSlice {
                t: eqn_ptr.clone(),
                indices: ({
                    let __elt = (*metamodelica::index_checked(&eqn_slices.borrow(), eqn.clone())?).clone();
                    __elt
                }),
            }),
            sliced_constr,
        );
        for mut candidate in &*Equation::collectCrefs(
            Pointer::access(eqn_ptr),
            (std::sync::Arc::new(getStateCandidate)
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
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )? {
            UnorderedSet::add(candidate.clone(), state_candidates.clone())?;
        }
    }
    for mut candidate in &*UnorderedSet::toList(state_candidates) {
        var_ptr = BVariable::getVarPointer(
            metamodelica::AsArg::as_arg(&candidate),
            metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
        )?;
        states = BVariable::VariablePointers::add(var_ptr, states)?;
    }
    Ok((constr, states, sliced_constr))
}

fn getStateCandidate(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    fn getStateCandidateVar(
        mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut acc: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<()> {
        if BVariable::isContinuous(var.clone(), false)?
            && !(BVariable::isTime(var.clone())
                || BVariable::isDummyVariable(var.clone())
                || BVariable::isDummyState(var.clone())
                || BVariable::isForcedState(var.clone())
                    && !(BVariable::isStateSelect(var.clone(), StateSelect::PREFER.clone())))
        {
            UnorderedSet::add(BVariable::getVarName(var), acc)?;
        }
        Ok(())
    }

    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    var = BVariable::getVarPointer(
        &cref,
        metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
    )?;
    if BVariable::isRecord(var.clone()) {
        for mut child in &*BVariable::getRecordChildrenCells(var) {
            getStateCandidateVar(PointerWeak::upgrade(child.clone())?, acc.clone())?;
        }
    } else {
        getStateCandidateVar(var, acc)?;
    }
    Ok(cref)
}

fn candidatePriority(mut candidate: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> {
    let mut prio: i32;
    prio = (::match_deref::match_deref! { match &(Pointer::access(candidate)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { attributes, .. }, .. } => {
            (match VariableAttributes::getStateSelect(metamodelica::AsArg::as_arg(&attributes)) {
        StateSelect::NEVER => -200,
        StateSelect::AVOID => -100,
        StateSelect::DEFAULT => 0,
        StateSelect::PREFER => 100,
        StateSelect::ALWAYS => 200,
        _ => 0,
    })
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(prio)
}

fn sortCandidates(
    mut candidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut candidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = candidates;
    let mut priorities: metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)> =
        metamodelica::nil();
    for mut candidate in &*candidates {
        priorities = metamodelica::cons((candidatePriority(candidate.clone())?, candidate.clone()), priorities);
    }
    priorities = List::sort(priorities, std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)))?;
    candidates = List::unzipSecond(&priorities);
    Ok(candidates)
}

fn resolveSlicedDummyStates(
    mut dummy_states: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut states: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
) -> Result<
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>,
> {
    pub(crate) type SliceSet = metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;

    let mut dummy_states: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = dummy_states;
    let mut covered: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
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
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cover_set: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;
    let mut full_size: i32;
    let mut resolved: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = metamodelica::nil();
    for mut cand in &*listAppend(states, dummy_states.clone()) {
        if !((cand.indices).is_empty()) {
            cref = BVariable::getVarName(Slice::getT(cand.clone()));
            if UnorderedMap::contains(cref.clone(), covered.clone())? {
                cover_set = UnorderedMap::getSafe(
                    cref,
                    covered.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
                )?;
                for mut idx in &*cand.indices.clone() {
                    UnorderedSet::add(idx.clone(), cover_set.clone())?;
                }
            } else {
                UnorderedMap::add(
                    cref,
                    UnorderedSet::fromList(
                        &cand.indices,
                        std::sync::Arc::new(fnptr!(Util::id, _)),
                        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    )?,
                    covered.clone(),
                )?;
            }
        }
    }
    for mut dummy in &*dummy_states {
        if (dummy.indices).is_empty() {
            resolved = metamodelica::cons(dummy.clone(), resolved);
        } else {
            cref = BVariable::getVarName(Slice::getT(dummy.clone()));
            cover_set = UnorderedMap::getSafe(
                cref.clone(),
                covered.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
            )?;
            full_size = BVariable::size(Slice::getT(dummy.clone()), false)?;
            if UnorderedSet::size(cover_set.clone()) == full_size {
                resolved = metamodelica::cons(
                    metamodelica::Ref::new(Slice::NBSlice {
                        t: Slice::getT(dummy.clone()),
                        indices: metamodelica::nil(),
                    }),
                    resolved,
                );
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBResolveSingularities.resolveSlicedDummyStates"));
                        __mm_s.push_str(&*literal!(" failed because the partially matched array variable "));
                        __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                        __mm_s.push_str(&*literal!(" could not be fully accounted for during index reduction ("));
                        __mm_s.push_str(&*intString(UnorderedSet::size(cover_set)));
                        __mm_s.push_str(&*literal!(" of "));
                        __mm_s.push_str(&*intString(full_size));
                        __mm_s.push_str(&*literal!(" elements matched as state or dummy state) -- the remainder belongs to a different, currently unresolved part of the system.\n"));
                        __mm_s.push_str(&*Slice::toString(dummy.clone(), &BVariable::pointerToString, 10)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        }
    }
    dummy_states = resolved.reverse();
    Ok(dummy_states)
}

fn resolveSlicedCandidates(
    mut candidates: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut subst: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut aux_index: Pointer::Pointer<i32>,
    mut eq_index: Pointer::Pointer<i32>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut candidates: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = candidates;
    let mut alias_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut resolved: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = metamodelica::nil();
    let mut alias_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut alias_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    for mut cand in &*candidates {
        if (cand.indices).is_empty() {
            resolved = metamodelica::cons(cand.clone(), resolved);
        } else {
            (alias_var, alias_eqn) =
                resolveSlicedDummy(cand.clone(), subst.clone(), aux_index.clone(), eq_index.clone())?;
            alias_eqns = metamodelica::cons(alias_eqn, alias_eqns);
            resolved = metamodelica::cons(
                metamodelica::Ref::new(Slice::NBSlice {
                    t: alias_var,
                    indices: metamodelica::nil(),
                }),
                resolved,
            );
        }
    }
    candidates = resolved.reverse();
    Ok((candidates, alias_eqns))
}

fn resolveSlicedDummy(
    mut dummy: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut subst: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut aux_index: Pointer::Pointer<i32>,
    mut eq_index: Pointer::Pointer<i32>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
)> {
    let mut alias_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut alias_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(Slice::getT(dummy.clone()));
    let mut orig_cref: metamodelica::Ref<ComponentRef::NFComponentRef> =
        BVariable::getVarName(Slice::getT(dummy.clone()));
    let mut elem_ty: metamodelica::Ref<Type::NFType> = Type::arrayElementType(&(Variable::typeOf(&var)));
    let mut sizes: metamodelica::List<i32> = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut d in (Type::arrayDims(Variable::typeOf(&var))).into_iter().cloned() {
            let __x = Dimension::size(&(d.clone()), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let mut n: i32 = ((dummy.indices).len() as i32);
    let mut alias_ty: metamodelica::Ref<Type::NFType>;
    let mut alias_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut elem_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut alias_elem_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut elems: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut loc: metamodelica::List<i32>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut i: i32 = 1;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    alias_ty = if (n == 1) {
        elem_ty
    } else {
        metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: elem_ty,
            dimensions: list![Dimension::fromInteger(n, NFPrefixes::Variability::CONSTANT.clone())],
        })
    };
    (alias_var, alias_cref) = BVariable::makeAuxVar(
        &(arcstr::literal!(BVariable::DUMMY_ALIAS_STR)),
        Pointer::access(aux_index.clone()),
        alias_ty.clone(),
        false,
    )?;
    Pointer::update(aux_index.clone(), Pointer::access(aux_index) + 1);
    for mut idx in &*dummy.indices.clone() {
        loc = Slice::indexToLocation(idx.clone(), sizes.clone());
        subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut l in (loc).into_iter().cloned() {
                let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: l.clone() + 1 }),
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        elem_cref = ComponentRef::mergeSubscripts(subs, orig_cref.clone(), true, true, true)?;
        elems = metamodelica::cons(Expression::fromCref(elem_cref.clone(), false)?, elems);
        alias_elem_cref = if (n == 1) {
            alias_cref.clone()
        } else {
            ComponentRef::setSubscripts(
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                alias_cref.clone(),
            )?
        };
        UnorderedMap::add(elem_cref, Expression::fromCref(alias_elem_cref, false)?, subst.clone())?;
        i = i + 1;
    }
    elems = elems.reverse();
    rhs = if (n == 1) {
        (elems).head().cloned()?
    } else {
        Expression::makeArray(
            alias_ty,
            metamodelica::arrayFromVec(elems.into_iter().cloned().collect()),
            false,
        )
    };
    alias_eqn = Equation::makeAssignment(
        Expression::fromCref(alias_cref, false)?,
        rhs,
        eq_index,
        &(literal!("DUM")),
        crate::NBEquation::Iterator::interned_EMPTY(),
        NBEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None),
    )?;
    Ok((alias_var, alias_eqn))
}

fn substituteSlicedDummyEqn(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut subst: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<()> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    eqn = Equation::map(
        eqn,
        (std::sync::Arc::new({
            let __pe_b1 = subst;
            move |__pe_a0| substituteSlicedDummyExp(__pe_a0, __pe_b1.clone())
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
    Pointer::update(eqn_ptr, eqn);
    Ok(())
}

fn substituteSlicedDummyExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut subst: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } if (UnorderedMap::contains(__exp_cref.clone(), subst.clone())?) => {
            UnorderedMap::getSafe(
                __exp_cref.clone(),
                subst.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
            )?
        }
        _ => exp,
    });
    Ok(exp)
}

fn resolveSlicedUnmatched(
    mut old_unmatched: &metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
    mut slice_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
        >,
    >,
) -> Result<
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
> {
    pub(crate) fn resolveSlicedUnmatchedSingle(
        mut eq: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        mut acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        >,
        mut slice_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
            >,
        >,
    ) -> Result<
        metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
    > {
        let mut acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        > = acc;
        let mut relevant_indices: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>;
        relevant_indices = UnorderedMap::getSafe(
            Equation::getEqnName(Slice::getT(eq.clone()))?,
            slice_map,
            metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBResolveSingularities.mo"),
        )?;
        if UnorderedSet::isEmpty(relevant_indices.clone()) {
            acc = metamodelica::cons(eq, acc);
        } else {
            assign_field!(
                eq.indices = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut ind in (eq.indices.clone()).into_iter().cloned() {
                        if !(UnorderedSet::contains(ind.clone(), relevant_indices.clone())?) {
                            continue;
                        }
                        let __x = ind.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            );
            if !((eq.indices).is_empty()) {
                acc = metamodelica::cons(eq, acc);
            }
        }
        Ok(acc)
    }

    let mut filtered_unmatched: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    > = metamodelica::nil();
    for mut eq in &**old_unmatched {
        filtered_unmatched = resolveSlicedUnmatchedSingle(eq.clone(), filtered_unmatched, slice_map.clone())?;
    }
    Ok(filtered_unmatched)
}

fn removeSlicedDerivatives(
    mut derivative: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut slice_set: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
    mut dummy_slice_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut aux_index: Pointer::Pointer<i32>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut derivative: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = derivative;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    if !(UnorderedSet::isEmpty(slice_set)) {
        eqn = removeSlicedDerivateEqn(
            Pointer::access(derivative.clone()),
            &(crate::NBEquation::Iterator::interned_EMPTY()),
            dummy_slice_set,
            aux_index,
        )?;
        Pointer::update(derivative.clone(), eqn);
    }
    Ok(derivative)
}

fn removeSlicedDerivateEqn(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut dummy_slice_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut aux_index: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    fn replaceTupleLiterals(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
        mut dummy_slice_set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
        mut aux_index: Pointer::Pointer<i32>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut aux: metamodelica::Ref<ComponentRef::NFComponentRef>;
        if Expression::isLiteral(&exp)? {
            aux = Call_Aux::createName(
                Expression::typeOf(exp),
                iter,
                aux_index,
                &(arcstr::literal!(BVariable::DERIVATIVE_STR)),
                false,
            )?;
            exp = metamodelica::Ref::new(Expression::NFExpression::CREF {
                ty: ComponentRef::getSubscriptedType(&aux, false)?,
                cref: aux.clone(),
            });
            UnorderedSet::add(aux, dummy_slice_set)?;
        }
        Ok(exp)
    }

    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { body: __eqn_body, iter: __eqn_iter, .. } => {
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::Equation>> = metamodelica::nil();
        for mut b in (__eqn_body.clone()).into_iter().cloned() {
            let __x = removeSlicedDerivateEqn(b.clone(), metamodelica::AsArg::as_arg(&__eqn_iter), dummy_slice_set.clone(), aux_index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eqn
        },
        Deref @ Equation::RECORD_EQUATION { lhs: lhs @ Deref @ Expression::TUPLE { .. }, .. } => {
            let mut lhs = (*lhs).clone();
            assign_variant_field!(lhs => Expression::NFExpression::TUPLE; elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (var_field!((*lhs).elements, Expression::NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = replaceTupleLiterals(e.clone(), iter, dummy_slice_set.clone(), aux_index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(eqn => Equation::Equation::RECORD_EQUATION; lhs = lhs.clone());
            eqn
        },
        _ => {
            eqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn toStringCandidatesConstraints(
    mut state_candidates: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut constraint_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*StringUtil::headline_1(&(literal!("Index Reduction")))?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(((state_candidates).len() as i32)));
                __mm_s.push_str(&*literal!(") Sorted State Candidates"));
                ArcStr::from(__mm_s)
            }),
        )?);
        __mm_s.push_str(&*Slice::lstToString(
            state_candidates,
            (std::sync::Arc::new(BVariable::pointerToString)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr>
                        + 'static,
                >),
            literal!(""),
            10,
        )?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(((constraint_eqns).len() as i32)));
                __mm_s.push_str(&*literal!(") Constraint Equations"));
                ArcStr::from(__mm_s)
            }),
        )?);
        __mm_s.push_str(&*Slice::lstToString(
            constraint_eqns,
            (std::sync::Arc::new({
                let __pe_b1 = literal!("");
                move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<ArcStr>
                        + 'static,
                >),
            literal!(""),
            10,
        )?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn toStringDynamicSelect(
    mut dummy_states: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut unmatched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*StringUtil::headline_2(
            &(literal!("\t  DYNAMIC STATE SELECTION\n\t(some unmatched equations)")),
        )?);
        __mm_s.push_str(&*StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(((dummy_states).len() as i32)));
                __mm_s.push_str(&*literal!(") Remaining State Candidates"));
                ArcStr::from(__mm_s)
            }),
        )?);
        __mm_s.push_str(&*Slice::lstToString(
            dummy_states,
            (std::sync::Arc::new(BVariable::pointerToString)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr>
                        + 'static,
                >),
            literal!(""),
            10,
        )?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(((unmatched_eqns).len() as i32)));
                __mm_s.push_str(&*literal!(") Remaining Equations"));
                ArcStr::from(__mm_s)
            }),
        )?);
        __mm_s.push_str(&*Slice::lstToString(
            unmatched_eqns,
            (std::sync::Arc::new({
                let __pe_b1 = literal!("");
                move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<ArcStr>
                        + 'static,
                >),
            literal!(""),
            10,
        )?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn toStringUnmatched(
    mut unmatched_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut unmatched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    let mut s4: ArcStr;
    if (unmatched_vars).is_empty() {
        s1 = StringUtil::headline_4(&(literal!("Not underdetermined.")))?;
        s3 = literal!("");
    } else {
        s1 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Stage "));
            __mm_s.push_str(&*intString(((unmatched_vars).len() as i32)));
            __mm_s.push_str(&*literal!(" underdetermined.\n"));
            ArcStr::from(__mm_s)
        };
        s3 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*intString(((unmatched_vars).len() as i32)));
                    __mm_s.push_str(&*literal!(") Unmatched variables:"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            __mm_s.push_str(&*Slice::lstToString(
                unmatched_vars,
                (std::sync::Arc::new(BVariable::pointerToString)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<ArcStr>
                            + 'static,
                    >),
                literal!(""),
                10,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    if (unmatched_eqns).is_empty() {
        s2 = StringUtil::headline_4(&(literal!("Not overdetermined.")))?;
        s4 = literal!("");
    } else {
        s2 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Stage "));
            __mm_s.push_str(&*intString(((unmatched_eqns).len() as i32)));
            __mm_s.push_str(&*literal!(" overdetermined.\n"));
            ArcStr::from(__mm_s)
        };
        s4 = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*intString(((unmatched_eqns).len() as i32)));
                    __mm_s.push_str(&*literal!(") Unmatched equations:"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            __mm_s.push_str(&*Slice::lstToString(
                unmatched_eqns,
                (std::sync::Arc::new({
                    let __pe_b1 = literal!("");
                    move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<ArcStr>
                            + 'static,
                    >),
                literal!(""),
                10,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s1);
        __mm_s.push_str(&*s2);
        __mm_s.push_str(&*s3);
        __mm_s.push_str(&*s4);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}
