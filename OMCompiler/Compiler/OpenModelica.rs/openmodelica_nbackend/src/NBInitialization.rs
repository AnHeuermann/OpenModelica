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

use crate::NBCausalize as Causalize;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBInline as Inline;
use crate::NBJacobian as Jacobian;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten as Flatten;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// NF imports
// Backend imports
// Util imports
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut initialVars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut initialEqs: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut modules: metamodelica::List<(Module::wrapper, ArcStr)>;
    let mut clocks: metamodelica::List<(ArcStr, metamodelica::Real)>;
    let mut followEquations: metamodelica::List<ArcStr> =
        Flags::getConfigStringList(Flags::DEBUG_FOLLOW_EQUATIONS.clone())?;
    let mut eq_filter_opt: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>;
    match '__try0: {
        bdae = ({
            let mut algorithm_outputs: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            > = UnorderedSet::new(
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
            let mut new_iters: metamodelica::Ref<
                UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
            > = UnorderedSet::new(
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
                13,
            );
            let mut cref_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Iterator::Iterator>,
                >,
            > = UnorderedMap::new(
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
            (::match_deref::match_deref! { match &(bdae.clone()) {
                Deref @ BackendDAE::MAIN { varData: varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { variables: __esc_variables, initials: __esc_initialVars, .. }, eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { equations: __esc_equations, initials: __esc_initialEqs, .. }, .. } => {
                    variables = (*__esc_variables).clone();
                    initialVars = (*__esc_initialVars).clone();
                    equations = (*__esc_equations).clone();
                    initialEqs = (*__esc_initialEqs).clone();
                    let mut clonedEqns: metamodelica::Ref<EquationPointers::EquationPointers>;
                    let mut clonedVars: metamodelica::Ref<VariablePointers::VariablePointers>;
                    let mut parameter_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                    let mut secondary_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                    let mut primary_aux_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                    let mut parameter_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                    let mut secondary_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                    let mut primary_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                    let mut varData = (*varData).clone();
                    let mut eqData = (*eqData).clone();
                    clonedEqns = unwrap_break_err!(BEquation::EquationPointers::clone(metamodelica::AsArg::as_arg(&equations), false), '__try0);
                    initialEqs = unwrap_break_err!(BEquation::EquationPointers::addList(&(unwrap_break_err!(BEquation::EquationPointers::toList(metamodelica::AsArg::as_arg(&initialEqs)), '__try0)), clonedEqns.clone()), '__try0);
                    unwrap_break_err!(BEquation::EquationPointers::mapRemovePtr(initialEqs.clone(), &fnptr!(BEquation::Equation::isClocked, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>)), '__try0);
                    unwrap_break_err!(BEquation::EquationPointers::mapPtr(metamodelica::AsArg::as_arg(&initialEqs), &replaceClockedFunctionsEqn), '__try0);
                    initialEqs = unwrap_break_err!(BEquation::EquationPointers::map(initialEqs.clone(), &({ let __pe_b1 = crate::NBEquation::Iterator::interned_EMPTY(); let __pe_b2 = cref_map.clone(); move |__pe_a0| removeWhenEquation(__pe_a0, &__pe_b1, __pe_b2.clone()) })), '__try0);
                    (equations, initialEqs) = unwrap_break_err!(createWhenReplacementEquations(cref_map.clone(), equations.clone(), initialEqs.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone()), '__try0);
                    unwrap_break_err!(BEquation::EquationPointers::map(initialEqs.clone(), &({ let __pe_b1 = algorithm_outputs.clone(); move |__pe_a0| collectAlgorithmOutputs(__pe_a0, __pe_b1.clone()) })), '__try0);
                    (variables, initialVars, equations, initialEqs) = unwrap_break_err!(createStartEquations(var_field!((*varData).states, VarData::VarData::VAR_DATA_SIM).clone(), variables.clone(), initialVars.clone(), equations.clone(), initialEqs.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), algorithm_outputs.clone(), var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM), &(literal!("State"))), '__try0);
                    (variables, initialVars, equations, initialEqs) = unwrap_break_err!(createStartEquations(var_field!((*varData).algebraics, VarData::VarData::VAR_DATA_SIM).clone(), variables.clone(), initialVars.clone(), equations.clone(), initialEqs.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), algorithm_outputs.clone(), var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM), &(literal!("Algebraic"))), '__try0);
                    (variables, initialVars, equations, initialEqs) = unwrap_break_err!(createStartEquations(var_field!((*varData).discretes, VarData::VarData::VAR_DATA_SIM).clone(), variables.clone(), initialVars.clone(), equations.clone(), initialEqs.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), algorithm_outputs.clone(), var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM), &(literal!("Discrete"))), '__try0);
                    (variables, initialVars, equations, initialEqs) = unwrap_break_err!(createStartEquations(var_field!((*varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone(), variables.clone(), initialVars.clone(), equations.clone(), initialEqs.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), algorithm_outputs.clone(), var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM), &(literal!("Discrete State"))), '__try0);
                    (variables, initialVars, equations, initialEqs) = unwrap_break_err!(createStartEquations(var_field!((*varData).clocked_states, VarData::VarData::VAR_DATA_SIM).clone(), variables.clone(), initialVars.clone(), equations.clone(), initialEqs.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), algorithm_outputs.clone(), var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM), &(literal!("Clocked State"))), '__try0);
                    (parameter_eqs, parameter_vars) = unwrap_break_err!(createParameterEquations(var_field!((*varData).parameters, VarData::VarData::VAR_DATA_SIM), new_iters.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), metamodelica::nil(), metamodelica::nil()), '__try0);
                    (parameter_eqs, parameter_vars) = unwrap_break_err!(createParameterEquations(var_field!((*varData).resizables, VarData::VarData::VAR_DATA_SIM), new_iters.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), parameter_eqs.clone(), parameter_vars.clone()), '__try0);
                    (parameter_eqs, parameter_vars) = unwrap_break_err!(createParameterEquations(var_field!((*varData).records, VarData::VarData::VAR_DATA_SIM), new_iters.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), parameter_eqs.clone(), parameter_vars.clone()), '__try0);
                    (parameter_eqs, parameter_vars) = unwrap_break_err!(createParameterEquations(var_field!((*varData).external_objects, VarData::VarData::VAR_DATA_SIM), new_iters.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), parameter_eqs.clone(), parameter_vars.clone()), '__try0);
                    (primary_comps, secondary_eqs, secondary_vars, primary_aux_eqs) = unwrap_break_err!(selectPrimaryParameters(parameter_eqs.clone(), parameter_vars.clone(), ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
                for mut eqn_ptr in (unwrap_break_err!(BEquation::EquationPointers::toList(metamodelica::AsArg::as_arg(&initialEqs)), '__try0)).into_iter().cloned() {
                    if !(unwrap_break_err!(isFunctionAliasBinding(eqn_ptr.clone()), '__try0)) { continue; }
                    let __x = eqn_ptr.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), &(unwrap_break_err!(BEquation::EquationPointers::toList(metamodelica::AsArg::as_arg(&initialEqs)), '__try0))), '__try0);
                    equations = unwrap_break_err!(BEquation::EquationPointers::addList(&parameter_eqs, equations.clone()), '__try0);
                    initialEqs = unwrap_break_err!(BEquation::EquationPointers::removeList(&primary_aux_eqs, initialEqs.clone()), '__try0);
                    initialEqs = unwrap_break_err!(BEquation::EquationPointers::addList(&secondary_eqs, initialEqs.clone()), '__try0);
                    initialVars = unwrap_break_err!(BVariable::VariablePointers::removeList(&(({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
                for mut eqn_ptr in (primary_aux_eqs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(BVariable::getVarPointer(&(unwrap_break_err!(Expression::toCref(&(unwrap_break_err!(unwrap_break_err!(BEquation::Equation::getLHS(Pointer::access(eqn_ptr.clone())), '__try0).ok_or("pattern mismatch"), '__try0))), '__try0)), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo")), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), initialVars.clone()), '__try0);
                    initialVars = unwrap_break_err!(BVariable::VariablePointers::addList(&secondary_vars, initialVars.clone()), '__try0);
                    if unwrap_break_err!(Flags::isSet(Flags::INITIALIZATION.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_BINDINGS.clone()), '__try0) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(List::toStringCustom(secondary_eqs.clone(), &({ let __pe_b1 = literal!("\t"); move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), unwrap_break_err!(StringUtil::headline_4(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Secondary Parameter Binding Equations (")); __mm_s.push_str(&*intString(((secondary_eqs).len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) })), '__try0), literal!(""), literal!("\n"), literal!(""), false, 0), '__try0)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(List::toStringCustom(primary_comps.clone(), &({ let __pe_b1 = -1; move |__pe_a0| StrongComponent::toString(&__pe_a0, __pe_b1.clone()) }), unwrap_break_err!(StringUtil::headline_4(&({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Primary Parameter Binding Equations (")); __mm_s.push_str(&*intString(((primary_comps).len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) })), '__try0), literal!(""), literal!("\n"), literal!(""), false, 0), '__try0)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
                    }
                    clonedVars = unwrap_break_err!(BVariable::VariablePointers::clone(metamodelica::AsArg::as_arg(&initialVars), true), '__try0);
                    unwrap_break_err!(BVariable::VariablePointers::mapRemovePtr(clonedVars.clone(), &fnptr!(BVariable::isClocked, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)), '__try0);
                    assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                        variables = variables.clone(),
                        initials = unwrap_break_err!(BVariable::VariablePointers::compress(clonedVars.clone()), '__try0)
                    );
                    assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM;
                        equations = equations.clone(),
                        initials = unwrap_break_err!(BEquation::EquationPointers::compress(initialEqs.clone()), '__try0)
                    );
                    assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                        eqData = eqData.clone(),
                        parameters = primary_comps.clone()
                    );
                    unwrap_break_err!(BackendDAE::setVarData(bdae.clone(), unwrap_break_err!(BVariable::VarData::addTypedList(varData.clone(), &(UnorderedSet::toList(new_iters.clone())), BVariable::VarData::VarType::ITERATOR.clone()), '__try0)), '__try0)
                },
                _ => {
                    unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInitialization.main")); __mm_s.push_str(&*literal!(" failed to create initial partition!")); ArcStr::from(__mm_s) }]), '__try0);
                    break '__try0 Err::<_, _>("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        if (followEquations).is_empty() {
            eq_filter_opt = None;
        } else {
            eq_filter_opt = Some(
                unwrap_break_err!(UnorderedSet::fromList(&followEquations, (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0),
            );
        }
        modules = list![
            (
                (std::sync::Arc::new({
                    let __pe_b1 = true;
                    move |__pe_a0| BackendDAE::simplify(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::NBackendDAE>,
                            )
                                -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>>
                            + 'static,
                    >),
                literal!("Simplify")
            ),
            (
                (std::sync::Arc::new({
                    let __pe_b1 = list![
                        openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
                        openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE,
                        openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE,
                        openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE
                    ];
                    let __pe_b2 = true;
                    move |__pe_a0| Inline::main(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::NBackendDAE>,
                            )
                                -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>>
                            + 'static,
                    >),
                literal!("Inline")
            ),
            (
                (std::sync::Arc::new({
                    let __pe_b1 = BPartition::Kind::INI.clone();
                    move |__pe_a0| Partitioning::main(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::NBackendDAE>,
                            )
                                -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>>
                            + 'static,
                    >),
                literal!("Partitioning")
            ),
            (
                (std::sync::Arc::new(cleanup)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::NBackendDAE>,
                            )
                                -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>>
                            + 'static,
                    >),
                literal!("Cleanup")
            ),
            (
                (std::sync::Arc::new({
                    let __pe_b1 = BPartition::Kind::INI.clone();
                    move |__pe_a0| Causalize::main(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::NBackendDAE>,
                            )
                                -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>>
                            + 'static,
                    >),
                literal!("Causalize")
            ),
            (
                (std::sync::Arc::new({
                    let __pe_b1 = BPartition::Kind::INI.clone();
                    move |__pe_a0| Tearing::main(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::NBackendDAE>,
                            )
                                -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>>
                            + 'static,
                    >),
                literal!("Tearing")
            )
        ];
        (bdae, clocks) = unwrap_break_err!(BackendDAE::applyModules(bdae.clone(), &modules, eq_filter_opt.clone(), ClockIndexes::RT_CLOCK_NEW_BACKEND_INITIALIZATION.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_BACKEND_CLOCKS.clone()), '__try0) {
            if !((clocks).is_empty()) {
                metamodelica::print(
                    unwrap_break_err!(StringUtil::headline_4(&(literal!("Initialization Backend Clocks:"))), '__try0),
                );
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut clck in (clocks.clone()).into_iter().cloned() {
                                let __x = unwrap_break_err!(Module::moduleClockString(&(clck.clone())), '__try0);
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!("\n"),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
        Ok::<_, &'static str>((bdae.clone(), clocks.clone(), eq_filter_opt.clone(), modules.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            bdae = __try0_o0;
            clocks = __try0_o1;
            eq_filter_opt = __try0_o2;
            modules = __try0_o3;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBInitialization.main"));
                    __mm_s.push_str(&*literal!(" failed to apply modules!"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(bdae)
}

pub(crate) fn createStartEquations(
    mut states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut initialVars: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut initialEqs: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut idx: Pointer::Pointer<i32>,
    mut algorithm_outputs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut aliasVars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut r#str: &ArcStr,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
)> {
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut initialVars: metamodelica::Ref<VariablePointers::VariablePointers> = initialVars;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    let mut initialEqs: metamodelica::Ref<EquationPointers::EquationPointers> = initialEqs;
    let mut ptr_start_vars: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut ptr_start_vars_init: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut ptr_start_eqs: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    > = Pointer::create(metamodelica::nil());
    let mut start_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    BVariable::VariablePointers::mapPtr(
        states,
        &({
            let __pe_b1 = ptr_start_vars.clone();
            let __pe_b2 = ptr_start_vars_init.clone();
            let __pe_b3 = ptr_start_eqs.clone();
            let __pe_b4 = idx;
            let __pe_b5 = algorithm_outputs;
            let __pe_b6 = aliasVars.clone();
            move |__pe_a0| {
                createStartEquation(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    &__pe_b6,
                )
            }
        }),
    )?;
    start_eqs = Pointer::access(ptr_start_eqs);
    variables = BVariable::VariablePointers::addList(&(Pointer::access(ptr_start_vars)), variables)?;
    initialVars = BVariable::VariablePointers::addList(&(Pointer::access(ptr_start_vars_init)), initialVars)?;
    equations = BEquation::EquationPointers::addList(&start_eqs, equations)?;
    initialEqs = BEquation::EquationPointers::addList(&start_eqs, initialEqs)?;
    if Flags::isSet(Flags::INITIALIZATION.clone())? && !((start_eqs).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*List::toStringCustom(
                start_eqs.clone(),
                &({
                    let __pe_b1 = literal!("\t");
                    move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone())
                }),
                StringUtil::headline_4(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Created "));
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!(" Start Equations ("));
                        __mm_s.push_str(&*intString(((start_eqs).len() as i32)));
                        __mm_s.push_str(&*literal!("):"));
                        ArcStr::from(__mm_s)
                    }),
                )?,
                literal!(""),
                literal!("\n"),
                literal!(""),
                false,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((variables, initialVars, equations, initialEqs))
}

pub(crate) fn createStartEquation(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut ptr_start_vars: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut ptr_start_vars_init: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut ptr_start_eqs: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut idx: Pointer::Pointer<i32>,
    mut algorithm_outputs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut aliasVars: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<()> {
    if !(UnorderedSet::contains(BVariable::getVarName(var.clone()), algorithm_outputs)?) {
        let () = (match &*(Pointer::access(var.clone())) {
            Variable::VARIABLE { .. } if (BVariable::isArray(var.clone())) => {
                let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
                let mut start_ok: Pointer::Pointer<bool>;
                if !(BVariable::isFixed(var.clone())?) && isFunctionAliasOrElement(var.clone())? {
                    start_ok = Pointer::create(true);
                    let () = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var.clone())?) {
                        Some(start_exp) if (!(Expression::isLiteralXML(start_exp.clone())?)) => {
                            let mut start_exp = (*start_exp).clone();
                            start_exp = resolveStartCrefs(start_exp.clone(), ptr_start_vars.clone(), aliasVars, start_ok.clone(), 0)?;
                            if Pointer::access(start_ok.clone()) {
                                Pointer::update(var.clone(), BVariable::setStartAttribute(Pointer::access(var.clone()), start_exp.clone(), true)?);
                            }
                            ()
                        },
                        _ => (),
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } });
                    if !(Pointer::access(start_ok)) {
                        return Ok(());
                    }
                }
                if BVariable::isFixed(var.clone())? {
                    createStartEquationSlice(
                        metamodelica::Ref::new(Slice::NBSlice {
                            t: var.clone(),
                            indices: metamodelica::nil(),
                        }),
                        ptr_start_vars,
                        ptr_start_eqs,
                        idx,
                        BVariable::isFixed(var.clone())?,
                    )?;
                } else {
                    createStartEquationSlice(
                        metamodelica::Ref::new(Slice::NBSlice {
                            t: var.clone(),
                            indices: metamodelica::nil(),
                        }),
                        ptr_start_vars_init,
                        ptr_start_eqs,
                        idx,
                        BVariable::isFixed(var.clone())?,
                    )?;
                }
                ()
            }
            Variable::VARIABLE { .. } if (BVariable::isFixed(var.clone())?) => {
                let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut start_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut start_eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                let mut kind: EquationKind;
                let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
                name = BVariable::getVarName(var.clone());
                start_exp = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var.clone())?) {
                    Some(e) if (!(Expression::isLiteralXML(e.clone())?)) => {
                        e.clone()
                    },
                    _ => {
                        (_, name, start_var, start_name) = createStartVar(var.clone(), name, metamodelica::nil())?;
                        Pointer::update(ptr_start_vars.clone(), metamodelica::cons(start_var, Pointer::access(ptr_start_vars)));
                        Expression::fromCref(start_name, false)?
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                kind = if (BVariable::isContinuous(var.clone(), true)?) {
                    EquationKind::CONTINUOUS.clone()
                } else {
                    EquationKind::DISCRETE.clone()
                };
                start_eq = BEquation::Equation::makeAssignment(
                    Expression::fromCref(name, false)?,
                    start_exp,
                    idx,
                    &(arcstr::literal!(BEquation::START_STR)),
                    crate::NBEquation::Iterator::interned_EMPTY(),
                    BEquation::default(kind, true, None, None),
                )?;
                Pointer::update(
                    ptr_start_eqs.clone(),
                    metamodelica::cons(start_eq, Pointer::access(ptr_start_eqs)),
                );
                ()
            }
            Variable::VARIABLE { .. } => {
                let mut start_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut start_eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                let mut kind: EquationKind;
                let mut start_ok: Pointer::Pointer<bool>;
                let () = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var.clone())?) {
                    Some(e) if (!(Expression::isLiteralXML(e.clone())?)) => {
                        let mut e = (*e).clone();
                        start_ok = Pointer::create(true);
                        if isFunctionAliasOrElement(var.clone())? {
                            e = resolveStartCrefs(e.clone(), ptr_start_vars, aliasVars, start_ok.clone(), 0)?;
                        }
                        if Pointer::access(start_ok) {
                            (_, _, start_var, start_name) = createStartVar(var.clone(), BVariable::getVarName(var.clone()), metamodelica::nil())?;
                            kind = if (BVariable::isContinuous(var.clone(), true)?) {EquationKind::CONTINUOUS.clone()} else {EquationKind::DISCRETE.clone()};
                            start_eq = BEquation::Equation::makeAssignment(Expression::fromCref(start_name, false)?, e.clone(), idx, &(arcstr::literal!(BEquation::START_STR)), crate::NBEquation::Iterator::interned_EMPTY(), BEquation::default(kind, true, None, None))?;
                            Pointer::update(ptr_start_eqs.clone(), metamodelica::cons(start_eq, Pointer::access(ptr_start_eqs)));
                            Pointer::update(ptr_start_vars_init.clone(), metamodelica::cons(start_var, Pointer::access(ptr_start_vars_init)));
                        }
                        ()
                    },
                    _ => {
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

pub(crate) fn createWhenReplacementEquations(
    mut cref_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Iterator::Iterator>,
        >,
    >,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut initialEqs: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut idx: Pointer::Pointer<i32>,
) -> Result<(
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
)> {
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    let mut initialEqs: metamodelica::Ref<EquationPointers::EquationPointers> = initialEqs;
    let mut ptr_start_eqs: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    > = Pointer::create(metamodelica::nil());
    let mut start_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    for mut tpl in &*UnorderedMap::toList(cref_map) {
        createWhenReplacementEquation(&(tpl.clone()), ptr_start_eqs.clone(), idx.clone())?;
    }
    start_eqs = Pointer::access(ptr_start_eqs);
    equations = BEquation::EquationPointers::addList(&start_eqs, equations)?;
    initialEqs = BEquation::EquationPointers::addList(&start_eqs, initialEqs)?;
    if Flags::isSet(Flags::INITIALIZATION.clone())? && !((start_eqs).is_empty()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*List::toStringCustom(
                start_eqs.clone(),
                &({
                    let __pe_b1 = literal!("\t");
                    move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone())
                }),
                StringUtil::headline_4(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Created When Replacement Equations ("));
                        __mm_s.push_str(&*intString(((start_eqs).len() as i32)));
                        __mm_s.push_str(&*literal!("):"));
                        ArcStr::from(__mm_s)
                    }),
                )?,
                literal!(""),
                literal!("\n"),
                literal!(""),
                false,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((equations, initialEqs))
}

pub(crate) fn createWhenReplacementEquation(
    mut tpl: &(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Iterator::Iterator>,
    ),
    mut ptr_start_eqs: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut idx: Pointer::Pointer<i32>,
) -> Result<()> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut iter: metamodelica::Ref<Iterator::Iterator>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var_pre: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut pre: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut kind: EquationKind;
    let mut eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    (cref, iter) = tpl.clone();
    var_ptr = BVariable::getVarPointer(
        &cref,
        metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"),
    )?;
    (var_pre, _) = BVariable::getVarPre(var_ptr.clone());
    if (var_pre).is_some() {
        pre = BVariable::getVarName(var_pre.ok_or("pattern mismatch")?);
        pre = ComponentRef::copySubscripts(&cref, pre)?;
        kind = if (BVariable::isContinuous(var_ptr, true)?) {
            EquationKind::CONTINUOUS.clone()
        } else {
            EquationKind::DISCRETE.clone()
        };
        eq = BEquation::Equation::makeAssignment(
            Expression::fromCref(cref, true)?,
            Expression::fromCref(pre, true)?,
            idx,
            &(arcstr::literal!(BEquation::START_STR)),
            iter,
            BEquation::default(kind, true, None, None),
        )?;
        Pointer::update(
            ptr_start_eqs.clone(),
            metamodelica::cons(eq, Pointer::access(ptr_start_eqs)),
        );
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBInitialization.createWhenReplacementEquation"));
                __mm_s.push_str(&*literal!(" could not replace when-replacement for "));
                __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                __mm_s.push_str(&*literal!(" because it has no pre-variable."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn createStartVar(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut start_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let (mut var_pre, _): (
        Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        ArcStr,
    ) = BVariable::getVarPre(var_ptr.clone());
    let mut merged_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if BVariable::isPrevious(var_ptr.clone()) && (var_pre).is_some() {
        merged_name = BVariable::getVarName(var_pre.ok_or("pattern mismatch")?);
        merged_name = ComponentRef::mergeSubscripts(subscripts, merged_name, true, true, true)?;
    } else if (var_pre).is_some() {
        merged_name = ComponentRef::mergeSubscripts(subscripts.clone(), name, true, true, true)?;
        var_ptr = var_pre.ok_or("pattern mismatch")?;
        name = BVariable::getVarName(var_ptr.clone());
        name = ComponentRef::mergeSubscripts(subscripts, name, true, true, true)?;
    } else {
        name = ComponentRef::mergeSubscripts(subscripts, name, true, true, true)?;
        merged_name = name.clone();
    }
    (start_name, start_var) = BVariable::makeStartVar(&merged_name)?;
    start_var = (match BVariable::getParent(var_ptr.clone()) {
        Some(mut parent) => {
            let mut start_parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            start_parent = (match (BVariable::getVarStart(parent.clone())).0 {
                Some(mut __esc_start_parent) => {
                    start_parent = __esc_start_parent.clone();
                    start_parent
                }
                _ => {
                    (_, _, start_parent, _) =
                        createStartVar(parent.clone(), BVariable::getVarName(parent), metamodelica::nil())?;
                    start_parent
                }
            });
            BVariable::addRecordChild(start_parent.clone(), start_var.clone())?;
            start_var = BVariable::setParent(start_var, start_parent);
            start_var
        }
        _ => start_var,
    });
    Ok((var_ptr, name, start_var, start_name))
}

pub(crate) fn createParameterEquations(
    mut parameters: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut new_iters: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut idx: Pointer::Pointer<i32>,
    mut parameter_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut initial_param_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
)> {
    let mut parameter_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = parameter_eqs;
    let mut initial_param_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        initial_param_vars;
    for mut var in &*BVariable::VariablePointers::toList(parameters)? {
        (parameter_eqs, initial_param_vars) = createParameterEquation(
            var.clone(),
            new_iters.clone(),
            idx.clone(),
            parameter_eqs,
            initial_param_vars,
        )?;
    }
    Ok((parameter_eqs, initial_param_vars))
}

pub(crate) fn selectPrimaryParameters(
    mut parameter_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut initial_param_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut aux_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut initial_eqs: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut primary_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut secondary_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut secondary_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut primary_aux_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut primary: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut unresolved: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut duplicates: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut remaining: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut next: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut sorted: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
    let mut name_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut progress: bool = true;
    let mut ready: bool;
    for mut eqn_ptr in &*listAppend(parameter_eqs.clone(), aux_eqs.clone()) {
        name_opt = explicitBindingName(eqn_ptr.clone())?;
        let () = (::match_deref::match_deref! { match &(name_opt) {
            Some(__esc_name) => {
                name = (*__esc_name).clone();
                if UnorderedSet::contains(name.clone(), unresolved.clone())? {
                    UnorderedSet::add(name.clone(), duplicates.clone())?;
                }
                UnorderedSet::add(name.clone(), unresolved.clone())?;
                remaining = metamodelica::cons(eqn_ptr.clone(), remaining);
                ()
            },
            _ => {
                let () = (::match_deref::match_deref! { match &(BEquation::Equation::getLHS(Pointer::access(eqn_ptr.clone()))?) {
            Some(Deref @ Expression::CREF { cref: __esc_name, .. }) => {
                name = (*__esc_name).clone();
                UnorderedSet::add(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&name)), unresolved.clone())?;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    remaining = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        for mut eqn_ptr in (remaining.reverse()).into_iter().cloned() {
            if !(!(isDuplicateBinding(eqn_ptr.clone(), duplicates.clone())?)) {
                continue;
            }
            let __x = eqn_ptr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut eqn_ptr in &**initial_eqs {
        let () = (::match_deref::match_deref! { match &(BEquation::Equation::getLHS(Pointer::access(eqn_ptr.clone()))?) {
            Some(Deref @ Expression::CREF { cref: __esc_name, .. }) => {
                name = (*__esc_name).clone();
                UnorderedSet::add(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&name)), unresolved.clone())?;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    while progress && !((remaining).is_empty()) {
        progress = false;
        next = metamodelica::nil();
        for mut eqn_ptr in &*remaining {
            let __pa0 = ::match_deref::match_deref! { match &(explicitBindingName(eqn_ptr.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            ready = true;
            for mut dep in &*UnorderedSet::toList(Expression::extractCrefs(
                (BEquation::Equation::getRHS(Pointer::access(eqn_ptr.clone()))?).ok_or("pattern mismatch")?,
            )?) {
                if !(isPrimaryCref(metamodelica::AsArg::as_arg(&dep), primary.clone(), unresolved.clone())?) {
                    ready = false;
                    break;
                }
            }
            if ready {
                UnorderedSet::add(name, primary.clone())?;
                sorted = metamodelica::cons(eqn_ptr.clone(), sorted);
                progress = true;
            } else {
                next = metamodelica::cons(eqn_ptr.clone(), next);
            }
        }
        remaining = next.reverse();
    }
    sorted = sorted.reverse();
    primary_comps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> = metamodelica::nil();
        for mut eqn_ptr in (sorted).into_iter().cloned() {
            let __x = StrongComponent::fromSolvedEquationSlice(metamodelica::Ref::new(Slice::NBSlice {
                t: eqn_ptr.clone(),
                indices: metamodelica::nil(),
            }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    secondary_eqs = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        for mut eqn_ptr in (parameter_eqs).into_iter().cloned() {
            if !(!(isPrimaryBinding(eqn_ptr.clone(), primary.clone())?)) {
                continue;
            }
            let __x = eqn_ptr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    secondary_vars = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut var in (initial_param_vars).into_iter().cloned() {
            if !(!(isSolved(
                ComponentRef::stripSubscriptsAll(&(BVariable::getVarName(var.clone()))),
                primary.clone(),
            )?)) {
                continue;
            }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    primary_aux_eqs = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        for mut eqn_ptr in (aux_eqs).into_iter().cloned() {
            if !(isPrimaryBinding(eqn_ptr.clone(), primary.clone())?) {
                continue;
            }
            let __x = eqn_ptr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((primary_comps, secondary_eqs, secondary_vars, primary_aux_eqs))
}

pub(crate) fn isFunctionAliasBinding(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr)) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
            isFunctionAliasCref(metamodelica::AsArg::as_arg(&cref))?
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, recordSize: None, .. } => {
            isFunctionAliasCref(metamodelica::AsArg::as_arg(&cref))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isFunctionAliasCref(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool;
    b = (match &**cref {
        ComponentRef::CREF { .. } if (InstNode::isVar(&(ComponentRef::node(cref)?))) => {
            BVariable::isFunctionAlias(BVariable::getVarPointer(
                cref,
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"),
            )?)?
        }
        _ => false,
    });
    Ok(b)
}

pub(crate) fn explicitBindingName(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
) -> Result<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut name: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    name = (::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } if (!(hasSubscripts(metamodelica::AsArg::as_arg(&cref))?)) => {
            Some(cref.clone())
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { ty, lhs: Deref @ Expression::CREF { cref, .. }, recordSize: None, .. } if (!(hasSubscripts(metamodelica::AsArg::as_arg(&cref))?) && !(Type::isComplex(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&ty)))))) => {
            Some(cref.clone())
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } if (!(hasSubscripts(metamodelica::AsArg::as_arg(&cref))?)) => {
            Some(cref.clone())
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } if (coversVariable(metamodelica::AsArg::as_arg(&cref), Pointer::access(eqn_ptr.clone()))?) => {
            Some(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref)))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(name)
}

pub(crate) fn coversVariable(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn: metamodelica::Ref<Equation::Equation>,
) -> Result<bool> {
    let mut b: bool;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
        ComponentRef::subscriptsAllFlat(cref)?;
    let mut iters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    var_ptr = BVariable::getVarPointer(
        &(ComponentRef::stripSubscriptsAll(cref)),
        metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"),
    )?;
    if (BVariable::getParent(var_ptr)).is_some() {
        b = false;
        return Ok(b);
    }
    b = !((subs).is_empty());
    for mut sub in &*subs {
        if !(Subscript::isIterator(metamodelica::AsArg::as_arg(&sub))) {
            b = false;
            break;
        }
        iter = Expression::toCref(&(Subscript::toExp(metamodelica::AsArg::as_arg(&sub))?))?;
        if UnorderedSet::contains(iter.clone(), iters.clone())? {
            b = false;
            break;
        }
        UnorderedSet::add(iter, iters.clone())?;
    }
    if b {
        b = BEquation::Equation::size(Pointer::create(eqn), false)?
            == BVariable::size(
                BVariable::getVarPointer(
                    &(ComponentRef::stripSubscriptsAll(cref)),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"),
                )?,
                false,
            )?;
    }
    Ok(b)
}

pub(crate) fn hasSubscripts(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut b: bool = !(ComponentRef::isEqual(cref, &(ComponentRef::stripSubscriptsAll(cref)))?);
    Ok(b)
}

pub(crate) fn isDuplicateBinding(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut duplicates: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(explicitBindingName(eqn_ptr)?) {
        Some(name) => {
            UnorderedSet::contains(name.clone(), duplicates)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isPrimaryBinding(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut primary: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(explicitBindingName(eqn_ptr)?) {
        Some(name) => {
            UnorderedSet::contains(name.clone(), primary)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isSolved(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut primary: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<bool> {
    let mut b: bool = false;
    let mut parent: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    while !(ComponentRef::isEmpty(&parent)) {
        if UnorderedSet::contains(parent.clone(), primary.clone())? {
            b = true;
            break;
        }
        parent = ComponentRef::rest(&parent)?;
    }
    Ok(b)
}

pub(crate) fn isPrimaryCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut primary: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut unresolved: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<bool> {
    let mut b: bool;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(cref);
    let mut parent: metamodelica::Ref<ComponentRef::NFComponentRef> = name.clone();
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    if ComponentRef::isIterator(cref) || isSolved(name, primary)? {
        b = true;
    } else {
        b = true;
        while !(ComponentRef::isEmpty(&parent)) {
            if UnorderedSet::contains(parent.clone(), unresolved.clone())? {
                b = false;
                break;
            }
            parent = ComponentRef::rest(&parent)?;
        }
        if b {
            b = (match &**cref {
                ComponentRef::CREF { .. } if (InstNode::isVar(&(ComponentRef::node(cref)?))) => {
                    var_ptr = BVariable::getVarPointer(
                        cref,
                        metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"),
                    )?;
                    BVariable::isConst(var_ptr.clone())
                        || BVariable::isParamOrConst(var_ptr.clone()) && BVariable::isFixed(var_ptr)?
                }
                _ => false,
            });
        }
    }
    Ok(b)
}

pub(crate) fn createParameterEquation(
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut new_iters: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut idx: Pointer::Pointer<i32>,
    mut parameter_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut initial_param_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
)> {
    let mut parameter_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = parameter_eqs;
    let mut initial_param_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        initial_param_vars;
    let mut parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut c_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut c_cell: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>;
    let mut skip: bool;
    if BVariable::isConst(var.clone()) {
        skip = true;
    } else {
        skip = (match BVariable::getParent(var.clone()) {
            Some(mut __esc_parent) => {
                parent = __esc_parent.clone();
                BVariable::isBound(parent.clone()) && BVariable::isKnownRecord(parent)
            }
            _ => BVariable::isRecord(var.clone()) && !(BVariable::isBound(var.clone())),
        });
    }
    if skip {
        return Ok((parameter_eqs, initial_param_vars));
    }
    if BVariable::isKnownRecord(var.clone()) {
        if !(BVariable::hasEvaluableBinding(var.clone())?)
            && (BVariable::isBound(var.clone()) || BVariable::hasStartAttr(var.clone())?)
        {
            initial_param_vars = listAppend(BVariable::getRecordChildren(var.clone())?, initial_param_vars);
            parameter_eqs = metamodelica::cons(
                BEquation::Equation::generateBindingEquation(var, idx, true, new_iters)?,
                parameter_eqs,
            );
        } else {
            for mut c_cell in &*BVariable::getRecordChildrenCells(var) {
                let mut c_cell = c_cell.clone();
                c_var = PointerWeak::upgrade(c_cell)?;
                if BVariable::isBound(c_var.clone()) {
                    BVariable::setBindingAsStart(c_var.clone(), true)?;
                }
                if BVariable::isRecord(c_var.clone()) {
                    (parameter_eqs, initial_param_vars) = createParameterEquation(
                        c_var,
                        new_iters.clone(),
                        idx.clone(),
                        parameter_eqs,
                        initial_param_vars,
                    )?;
                }
            }
        }
    } else if !(BVariable::isRecord(var.clone())) {
        if !(BVariable::hasEvaluableBinding(var.clone())?) {
            initial_param_vars = metamodelica::cons(var.clone(), initial_param_vars);
            if BVariable::isFixed(var.clone())? {
                parameter_eqs = metamodelica::cons(
                    BEquation::Equation::generateBindingEquation(var, idx, true, new_iters)?,
                    parameter_eqs,
                );
            }
        } else if BVariable::isBound(var.clone()) {
            BVariable::setBindingAsStart(var, true)?;
        }
    }
    Ok((parameter_eqs, initial_param_vars))
}

pub(crate) fn isAliasVar(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut aliasVars: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<bool> {
    let mut b: bool = BVariable::VariablePointers::containsCref(BVariable::getVarName(var_ptr.clone()), aliasVars)?;
    Ok(b)
}

pub(crate) fn isFunctionAliasOrElement(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<bool> {
    '__tco: loop {
        match BVariable::getParent(var_ptr.clone()) {
            Some(mut parent) => {
                var_ptr = parent;
                continue '__tco;
            }
            _ => return Ok(BVariable::isFunctionAlias(var_ptr)?),
        }
    }
}

pub(crate) fn resolveStartCrefs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ptr_start_vars: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut aliasVars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut ok: Pointer::Pointer<bool>,
    mut depth: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    res = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = ptr_start_vars;
            let __pe_b2 = aliasVars.clone();
            let __pe_b3 = ok;
            let __pe_b4 = depth;
            move |__pe_a0| resolveStartCref(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone(), __pe_b4.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(res)
}

pub(crate) fn resolveStartCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ptr_start_vars: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut aliasVars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut ok: Pointer::Pointer<bool>,
    mut depth: i32,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression> = exp.clone();
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var: metamodelica::Ref<Variable::NFVariable>;
    let mut start_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut existed: bool;
    let mut start_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
    res = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CALL { call: __exp_call } if (StringUtil::startsWith(AbsynUtil::pathFirstIdent(&(Call::functionName(metamodelica::AsArg::as_arg(&__exp_call))?)), literal!("$OMC$"))) => {
            Pointer::update(ok, false);
            res
        },
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } if (!(ComponentRef::isTime(var_field!((*exp).cref, Expression::NFExpression::CREF))?)) => {
            var_ptr = BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"))?;
            var = Pointer::access(var_ptr.clone());
            if ComponentRef::isEmpty(&var.name) {
                Pointer::update(ok, false);
            } else if BVariable::VariablePointers::containsCref(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF)), aliasVars)? {
                if depth < 10 && Binding::isBound(&var.binding) {
                    res = Binding::getExp(&var.binding)?;
                    if ComponentRef::hasSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF))? && Type::isArray(&(Expression::typeOf(res.clone()))) {
                        res = Expression::applySubscripts(&(ComponentRef::subscriptsAllWithWholeFlat(var_field!((*exp).cref, Expression::NFExpression::CREF))?), res, true)?;
                    }
                    res = resolveStartCrefs(res, ptr_start_vars, aliasVars, ok, depth + 1)?;
                } else {
                    Pointer::update(ok, false);
                }
            } else if List::any(&(BVariable::getRecordChildren(var_ptr.clone())?), &({ let __pe_b1 = aliasVars.clone(); move |__pe_a0| isAliasVar(__pe_a0, &__pe_b1) }))? {
                Pointer::update(ok, false);
            } else if BVariable::isParamOrConst(var_ptr.clone()) || BVariable::isStart(var_ptr.clone()) || BVariable::isIterator(var_ptr.clone()) || BVariable::isExtObj(var_ptr.clone()) {
            } else if BVariable::isRecord(var_ptr.clone()) || ((BVariable::getParent(var_ptr.clone()))).is_some() {
                Pointer::update(ok, false);
            } else if Type::isReal(&(Type::arrayElementType(&(Variable::typeOf(&var)))))? && !(Type::isArray(&(Expression::typeOf(exp.clone())))) && BVariable::isContinuous(var_ptr.clone(), true)? {
                start_opt = BVariable::getStartAttribute(var_ptr.clone())?;
                existed = (((BVariable::getVarStart(var_ptr.clone())).0)).is_some();
                if BVariable::isFixed(var_ptr.clone())? && (start_opt).is_some() && !(Expression::isLiteralXML(start_opt.clone().ok_or("pattern mismatch")?)?) {
                    if depth < 10 {
                        res = start_opt.ok_or("pattern mismatch")?;
                        if ComponentRef::hasSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF))? && Type::isArray(&(Expression::typeOf(res.clone()))) {
                            res = Expression::applySubscripts(&(ComponentRef::subscriptsAllWithWholeFlat(var_field!((*exp).cref, Expression::NFExpression::CREF))?), res, true)?;
                        }
                        res = resolveStartCrefs(res, ptr_start_vars, aliasVars, ok, depth + 1)?;
                    } else {
                        Pointer::update(ok, false);
                    }
                } else if (start_opt).is_none() && !(existed) {
                    Pointer::update(ok, false);
                } else {
                    (start_name, start_var) = BVariable::makeStartVar(var_field!((*exp).cref, Expression::NFExpression::CREF))?;
                    res = Expression::fromCref(start_name, false)?;
                    if !(existed) && !(BVariable::isFixed(var_ptr)?) && ((start_opt).is_none() || Expression::isLiteralXML(start_opt.ok_or("pattern mismatch")?)?) {
                        Pointer::update(ptr_start_vars.clone(), metamodelica::cons(start_var, Pointer::access(ptr_start_vars)));
                    }
                }
            } else {
                Pointer::update(ok, false);
            }
            res
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn createStartEquationSlice(
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut ptr_start_vars: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut ptr_start_eqs: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut idx: Pointer::Pointer<i32>,
    mut fixed: bool,
) -> Result<()> {
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut start_var_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut e: metamodelica::Ref<Expression::NFExpression>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut start_eq: Option<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = None;
    let mut kind: EquationKind;
    let mut iterator: metamodelica::Ref<Iterator::Iterator> = metamodelica::Ref::new(Iterator::EMPTY);
    let mut sliced_eqn: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    var_ptr = Slice::getT(var_slice.clone());
    name = BVariable::getVarName(var_ptr.clone());
    kind = if (BVariable::isContinuous(var_ptr.clone(), true)?) {
        EquationKind::CONTINUOUS.clone()
    } else {
        EquationKind::DISCRETE.clone()
    };
    if fixed {
        start_exp = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var_ptr.clone())?) {
            Some(e) if (!(Expression::isLiteralXML(e.clone())?)) => {
                (start_exp, var_ptr, name, _, _, iterator) = createStartExpressionSlice(e.clone(), var_slice.clone(), var_ptr, name)?;
                start_exp
            },
            _ => {
                (start_var_exp, var_ptr, name, iterator) = createStartVariableSlice(var_slice.clone(), var_ptr, name, ptr_start_vars)?;
                start_var_exp
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        start_eq = Some(BEquation::Equation::makeAssignment(
            Expression::fromCref(name, true)?,
            start_exp,
            idx,
            &(arcstr::literal!(BEquation::START_STR)),
            iterator,
            BEquation::default(kind, true, None, None),
        )?);
    } else {
        start_eq = (::match_deref::match_deref! { match &(BVariable::getStartAttribute(var_ptr.clone())?) {
            Some(e) if (!(Expression::isLiteralXML(e.clone())?)) => {
                (start_exp, var_ptr, _, start_var, name, iterator) = createStartExpressionSlice(e.clone(), var_slice.clone(), var_ptr, name)?;
                start_eq = Some(BEquation::Equation::makeAssignment(Expression::fromCref(name, true)?, start_exp, idx, &(arcstr::literal!(BEquation::START_STR)), iterator, BEquation::default(kind, true, None, None))?);
                Pointer::update(ptr_start_vars.clone(), metamodelica::cons(start_var, Pointer::access(ptr_start_vars)));
                start_eq
            },
            _ => None,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if (start_eq).is_some() {
        if !((var_slice.indices).is_empty()) {
            (sliced_eqn, _) =
                BEquation::Equation::slice(start_eq.ok_or("pattern mismatch")?, var_slice.indices.clone())?;
            Pointer::update(
                ptr_start_eqs.clone(),
                listAppend(Pointer::access(ptr_start_eqs), sliced_eqn),
            );
        } else {
            Pointer::update(
                ptr_start_eqs.clone(),
                metamodelica::cons(start_eq.ok_or("pattern mismatch")?, Pointer::access(ptr_start_eqs)),
            );
        }
    }
    Ok(())
}

pub(crate) fn createStartExpressionSlice(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Iterator::Iterator>,
)> {
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut start_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut iterator: metamodelica::Ref<Iterator::Iterator>;
    (start_exp, iterator) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: array_constructor @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut frames: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>, Option<metamodelica::Ref<Iterator::Iterator>>)>;
            let mut replacements: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
            let mut old_iter: metamodelica::Ref<InstNode::InstNode>;
            let mut new_iter: metamodelica::Ref<ComponentRef::NFComponentRef>;
            (var_ptr, name, start_var, start_cref, _, frames, iterator) = createIteratedStartCref(var_ptr, name, ((var_field!((**array_constructor).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR)).len() as i32))?;
            replacements = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            for mut tpl in &*List::zip(var_field!((**array_constructor).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), frames) {
                let ((__pa0, _), (__pa1, _, _)) = tpl.clone();
                old_iter = metamodelica::Own::own(__pa0);
                new_iter = metamodelica::Own::own(__pa1);
                UnorderedMap::add(ComponentRef::fromNode(old_iter.clone(), InstNode::getType(old_iter)?, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?, Expression::fromCref(new_iter, false)?, replacements.clone())?;
            }
            (Expression::map(var_field!((**array_constructor).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), (std::sync::Arc::new({ let __pe_b1 = replacements; move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?, iterator)
        },
        _ => {
            let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            if Slice::isFull(var_slice) {
                (var_ptr, name, start_var, start_cref) = createStartVar(var_ptr, name, metamodelica::nil())?;
                iterator = crate::NBEquation::Iterator::interned_EMPTY();
                start_exp = exp;
            } else {
                (var_ptr, name, start_var, start_cref, subscripts, _, iterator) = createIteratedStartCref(var_ptr, name, 0)?;
                start_exp = Expression::applySubscripts(&subscripts, exp, true)?;
            }
            (start_exp, iterator)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((start_exp, var_ptr, name, start_var, start_cref, iterator))
}

pub(crate) fn createStartVariableSlice(
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ptr_start_vars: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::Ref<Iterator::Iterator>,
)> {
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    let mut iterator: metamodelica::Ref<Iterator::Iterator>;
    let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut start_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    if Slice::isFull(var_slice) {
        (var_ptr, name, start_var, start_name) = createStartVar(var_ptr, name, metamodelica::nil())?;
        iterator = crate::NBEquation::Iterator::interned_EMPTY();
    } else {
        (var_ptr, name, start_var, start_name, subscripts, _, iterator) = createIteratedStartCref(var_ptr, name, 0)?;
    }
    Pointer::update(
        ptr_start_vars.clone(),
        metamodelica::cons(start_var, Pointer::access(ptr_start_vars)),
    );
    start_exp = Expression::fromCref(start_name, false)?;
    Ok((start_exp, var_ptr, name, iterator))
}

fn createIteratedStartCref(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut num_dim: i32,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>,
    metamodelica::Ref<Iterator::Iterator>,
)> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = var_ptr;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = name;
    let mut start_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut start_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut iterator: metamodelica::Ref<Iterator::Iterator>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut iterators: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut iter_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    dims = Type::arrayDims(ComponentRef::getSubscriptedType(&name, false)?);
    dims = if (num_dim == 0) {
        dims
    } else {
        List::firstN(dims, num_dim)?
    };
    (iterators, ranges, subscripts) = Flatten::makeIterators(&name, &dims)?;
    iter_crefs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut iter in (iterators).into_iter().cloned() {
            let __x = ComponentRef::makeIterator(iter.clone(), openmodelica_nf_frontend::NFType::interned_INTEGER())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    iter_crefs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut iter in (iter_crefs).into_iter().cloned() {
            let __x = BackendDAE::lowerIteratorCref(iter.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    subscripts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut sub in (subscripts).into_iter().cloned() {
            let __x = Subscript::mapExp(
                sub.clone(),
                (std::sync::Arc::new(BackendDAE::lowerIteratorExp)
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
    frames = List::zip3(
        iter_crefs.clone(),
        ranges,
        List::fill(None, ((iter_crefs).len() as i32)),
    );
    iterator = BEquation::Iterator::fromFrames(frames.clone());
    (var_ptr, name, start_var, start_cref) = createStartVar(var_ptr, name, subscripts.clone())?;
    Ok((var_ptr, name, start_var, start_cref, subscripts, frames, iterator))
}

pub(crate) fn createPreEquation(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut ptr_pre_eqs: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut idx: Pointer::Pointer<i32>,
) -> Result<()> {
    let mut pre: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut pre_eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut kind: EquationKind;
    if !(BVariable::isPrevious(var_ptr.clone())) {
        (pre, _) = BVariable::getVarPre(var_ptr.clone());
        if (pre).is_some() {
            kind = if (BVariable::isContinuous(var_ptr.clone(), true)?) {
                EquationKind::CONTINUOUS.clone()
            } else {
                EquationKind::DISCRETE.clone()
            };
            pre_eq = BEquation::Equation::makeAssignment(
                Expression::fromCref(BVariable::getVarName(var_ptr), false)?,
                Expression::fromCref(BVariable::getVarName(pre.ok_or("pattern mismatch")?), false)?,
                idx,
                &(arcstr::literal!(BEquation::PRE_STR)),
                crate::NBEquation::Iterator::interned_EMPTY(),
                BEquation::default(kind, true, None, None),
            )?;
            Pointer::update(
                ptr_pre_eqs.clone(),
                metamodelica::cons(pre_eq, Pointer::access(ptr_pre_eqs)),
            );
        }
    }
    Ok(())
}

pub(crate) fn createPreEquationSlice(
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut ptr_pre_eqs: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    mut idx: Pointer::Pointer<i32>,
) -> Result<()> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut pre: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut pre_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut iterators: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut frames: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
        Option<metamodelica::Ref<Iterator::Iterator>>,
    )>;
    let mut pre_eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut kind: EquationKind;
    let mut sliced_eqn: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    var_ptr = Slice::getT(var_slice.clone());
    if !(BVariable::isPrevious(var_ptr.clone())) {
        (pre, _) = BVariable::getVarPre(var_ptr.clone());
        if (pre).is_some() {
            name = BVariable::getVarName(var_ptr.clone());
            dims = Type::arrayDims(ComponentRef::getSubscriptedType(&name, false)?);
            (iterators, ranges, subscripts) = Flatten::makeIterators(&name, &dims)?;
            frames = List::zip3(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut iter in (iterators).into_iter().cloned() {
                        let __x = ComponentRef::makeIterator(
                            iter.clone(),
                            openmodelica_nf_frontend::NFType::interned_INTEGER(),
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                ranges.clone(),
                List::fill(None, ((ranges).len() as i32)),
            );
            pre_name = BVariable::getVarName(pre.ok_or("pattern mismatch")?);
            pre_name = ComponentRef::mergeSubscripts(subscripts.clone(), pre_name, true, true, false)?;
            name = ComponentRef::mergeSubscripts(subscripts, name, true, true, false)?;
            kind = if (BVariable::isContinuous(var_ptr, true)?) {
                EquationKind::CONTINUOUS.clone()
            } else {
                EquationKind::DISCRETE.clone()
            };
            pre_eq = BEquation::Equation::makeAssignment(
                Expression::fromCref(name, true)?,
                Expression::fromCref(pre_name, false)?,
                idx,
                &(arcstr::literal!(BEquation::PRE_STR)),
                BEquation::Iterator::fromFrames(frames),
                BEquation::default(kind, true, None, None),
            )?;
            if !((var_slice.indices).is_empty()) {
                (sliced_eqn, _) = BEquation::Equation::slice(pre_eq, var_slice.indices.clone())?;
                Pointer::update(
                    ptr_pre_eqs.clone(),
                    listAppend(Pointer::access(ptr_pre_eqs), sliced_eqn),
                );
            } else {
                Pointer::update(
                    ptr_pre_eqs.clone(),
                    metamodelica::cons(pre_eq, Pointer::access(ptr_pre_eqs)),
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn cleanup(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut hasHom: Pointer::Pointer<bool> = Pointer::create(false);
    let mut init_0: metamodelica::List<metamodelica::Ref<Partition::Partition>>;
    bdae = (match &*bdae {
        BackendDAE::MAIN { ode: __bdae_ode, .. } => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                        ode = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (__bdae_ode.clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapEqn(par.clone(), &({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone()) }))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        algebraic = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).algebraic, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapEqn(par.clone(), &({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone()) }))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ode_event = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).ode_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapEqn(par.clone(), &({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone()) }))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        alg_event = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).alg_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapEqn(par.clone(), &({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone()) }))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            if (var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; dae = Some(({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN).clone().ok_or("pattern mismatch")?).into_iter().cloned() {
                        let __x = BPartition::Partition::mapEqn(par.clone(), &({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone()) }))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })));
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                        ode = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapExp(par.clone(), (std::sync::Arc::new({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupHomotopy(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        algebraic = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).algebraic, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapExp(par.clone(), (std::sync::Arc::new({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupHomotopy(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ode_event = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).ode_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapExp(par.clone(), (std::sync::Arc::new({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupHomotopy(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        alg_event = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).alg_event, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapExp(par.clone(), (std::sync::Arc::new({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupHomotopy(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            if (var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; dae = Some(({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (var_field!((*bdae).dae, BackendDAE::NBackendDAE::MAIN).clone().ok_or("pattern mismatch")?).into_iter().cloned() {
                        let __x = BPartition::Partition::mapExp(par.clone(), (std::sync::Arc::new({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupHomotopy(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })));
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapExp(par.clone(), (std::sync::Arc::new({ let __pe_b1 = hasHom.clone(); move |__pe_a0| containsLambda0(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            if Pointer::access(hasHom) {
                init_0 = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone())
                        .into_iter()
                        .cloned()
                    {
                        let __x = BPartition::Partition::setKind(
                            BPartition::Partition::clone(par.clone(), false)?,
                            BPartition::Kind::INI_0.clone(),
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                init_0 = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (init_0).into_iter().cloned() {
                        let __x = BPartition::Partition::mapEqn(
                            par.clone(),
                            &({
                                let __pe_b1 = BPartition::Partition::getKind(&(par.clone()));
                                move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone())
                            }),
                        )?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                init_0 = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (init_0).into_iter().cloned() {
                        let __x = BPartition::Partition::mapExp(
                            par.clone(),
                            (std::sync::Arc::new({
                                let __pe_b1 = BPartition::Partition::getKind(&(par.clone()));
                                move |__pe_a0| cleanupHomotopy(__pe_a0, __pe_b1.clone())
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
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init_0 = Some(init_0));
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                for mut par in (var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                    let __x = BPartition::Partition::mapEqn(par.clone(), &({ let __pe_b1 = BPartition::Partition::getKind(&(par.clone())); move |__pe_a0| cleanupInitialCall(__pe_a0, __pe_b1.clone()) }))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            bdae
        }
        _ => bdae,
    });
    Ok(bdae)
}

pub(crate) fn cleanupInitialCall(
    mut eq: metamodelica::Ref<Equation::Equation>,
    mut kind: BPartition::Kind,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    fn cleanupInitialCallExp(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut kind: BPartition::Kind,
        mut simplify: Pointer::Pointer<bool>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        if Expression::isCallNamed(&exp, &(literal!("initial")))? {
            exp = metamodelica::Ref::new(Expression::NFExpression::BOOLEAN {
                value: kind == BPartition::Kind::INI.clone() || kind == BPartition::Kind::INI_0.clone(),
            });
            Pointer::update(simplify, true);
        } else if Flags::isConfigFlagSet(
            Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
            literal!("initialSimplified"),
        )? && Expression::isCallNamed(&exp, &(literal!("initialSimplified")))?
        {
            exp = metamodelica::Ref::new(Expression::NFExpression::BOOLEAN {
                value: kind == BPartition::Kind::INI_0.clone(),
            });
            Pointer::update(simplify, true);
        }
        Ok(exp)
    }

    let mut eq: metamodelica::Ref<Equation::Equation> = eq;
    let mut simplify: Pointer::Pointer<bool> = Pointer::create(false);
    eq = BEquation::Equation::map(
        eq,
        (std::sync::Arc::new({
            let __pe_b1 = kind;
            let __pe_b2 = simplify.clone();
            move |__pe_a0| cleanupInitialCallExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
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
    if Pointer::access(simplify) {
        eq = BEquation::Equation::simplify(
            eq,
            &(literal!("")),
            &(literal!("")),
            Pointer::create(metamodelica::nil()),
            Pointer::create(metamodelica::nil()),
            (std::sync::Arc::new({
                let __pe_b1 = true;
                let __pe_b2 = literal!("");
                let __pe_b3 = literal!("");
                move |__pe_a0| NFSimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    }
    Ok(eq)
}

pub(crate) fn cleanupHomotopy(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut kind: BPartition::Kind,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp.clone() {
        Expression::CALL { call: __exp_call }
            if (Call::isNamed(metamodelica::AsArg::as_arg(&__exp_call), &(literal!("homotopy")))?) =>
        {
            (match kind {
                BPartition::Kind::INI_0 => (Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?).get(2)?,
                BPartition::Kind::INI => exp,
                _ => (Call::arguments(metamodelica::AsArg::as_arg(&__exp_call))?)
                    .head()
                    .cloned()?,
            })
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn containsHomotopyCall(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut b: Pointer::Pointer<bool>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    if !(Pointer::access(b.clone())) && Expression::isCallNamed(&exp, &(literal!("homotopy")))? {
        Pointer::update(b, true);
    }
    Ok(exp)
}

pub(crate) fn containsLambda0(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut b: Pointer::Pointer<bool>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    if !(Pointer::access(b.clone()))
        && (Expression::isCallNamed(&exp, &(literal!("homotopy")))?
            || Flags::isConfigFlagSet(
                Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
                literal!("initialSimplified"),
            )? && Expression::isCallNamed(&exp, &(literal!("initialSimplified")))?)
    {
        Pointer::update(b, true);
    }
    Ok(exp)
}

pub(crate) fn minimizeHomotopySystem(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    bdae = (match &*bdae {
        BackendDAE::MAIN { .. } => {
            if (var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition>> = metamodelica::nil();
                    for mut par in (var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
                        let __x = BPartition::Partition::mapStrongComponents(par.clone(), &({ let __pe_b1 = true; move |__pe_a0| Ok(StrongComponent::setHomotopy(__pe_a0, __pe_b1.clone())) }))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
            }
            bdae
        }
        _ => bdae,
    });
    Ok(bdae)
}

pub(crate) fn removeWhenEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut cref_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Iterator::Iterator>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    eqn = (match &*eqn.clone() {
        BEquation::Equation::FOR_EQUATION {
            body: __eqn_body,
            iter: __eqn_iter,
            ..
        } => {
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::Equation>> = metamodelica::nil();
                for mut b in (__eqn_body.clone()).into_iter().cloned() {
                    let __x = removeWhenEquation(b.clone(), metamodelica::AsArg::as_arg(&__eqn_iter), cref_map.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            if (List::all(
                var_field!((*eqn).body, Equation::Equation::FOR_EQUATION),
                &move |__a0: metamodelica::Ref<Equation::Equation>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BEquation::Equation::isDummy(&__a0))
                },
            )?) {
                crate::NBEquation::Equation::interned_DUMMY_EQUATION()
            } else {
                eqn
            }
        }
        BEquation::Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            let mut new_eqn: metamodelica::Ref<Equation::Equation>;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut lhs_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            stmts = removeWhenEquationBody(Some(__eqn_body.clone()))?;
            if !((stmts).is_empty()) {
                new_eqn = Pointer::access(BEquation::Equation::makeAlgorithm(stmts, true)?);
                new_eqn = BEquation::Equation::setResidualVar(
                    new_eqn,
                    BEquation::Equation::getResidualVar(Pointer::create(eqn))?,
                )?;
            } else {
                lhs_crefs = BEquation::WhenEquationBody::getAllAssigned(metamodelica::AsArg::as_arg(&__eqn_body));
                for mut cref in &*lhs_crefs {
                    UnorderedMap::add(cref.clone(), iter.clone(), cref_map.clone())?;
                }
                new_eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
            }
            new_eqn
        }
        BEquation::Equation::IF_EQUATION { body: __eqn_body, .. } => {
            assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = removeWhenEquationIfBody(__eqn_body.clone(), iter, cref_map)?);
            assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; size = BEquation::IfEquationBody::size(var_field!((*eqn).body, Equation::Equation::IF_EQUATION), false)?);
            if (var_field!((*eqn).size, Equation::Equation::IF_EQUATION).clone() > 0) {
                eqn
            } else {
                crate::NBEquation::Equation::interned_DUMMY_EQUATION()
            }
        }
        BEquation::Equation::ALGORITHM { alg, .. } => {
            let mut new_eqn: metamodelica::Ref<Equation::Equation>;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            let mut alg = (*alg).clone();
            stmts = removeWhenEquationAlgorithmBody(alg.statements.clone())?;
            if !((stmts).is_empty()) {
                assign_field!(alg.statements = stmts);
                assign_variant_field!(eqn => Equation::Equation::ALGORITHM; alg = Algorithm::setInputsOutputs(alg.clone())?);
                assign_variant_field!(eqn => Equation::Equation::ALGORITHM; size = ({
                    let mut __acc: i32 = 0;
                    for mut out in (var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone()).into_iter().cloned() {
                        let __x = ComponentRef::size(&(out.clone()), true, false)?;
                        __acc += __x;
                    }
                    __acc
                }));
                new_eqn = eqn;
            } else {
                new_eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
            }
            new_eqn
        }
        _ => eqn,
    });
    Ok(eqn)
}

pub(crate) fn removeWhenEquationBody(
    mut body_opt: Option<metamodelica::Ref<WhenEquationBody::WhenEquationBody>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    stmts = (::match_deref::match_deref! { match &(body_opt) {
        Some(body) => {
            if isInitialCall(&body.condition)? {
                stmts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut st in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = BEquation::WhenStatement::toStatement(&(st.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            } else {
                stmts = removeWhenEquationBody(body.else_when.clone())?;
            }
            stmts
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmts)
}

pub(crate) fn removeWhenEquationIfBody(
    mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut cref_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Iterator::Iterator>,
        >,
    >,
) -> Result<metamodelica::Ref<IfEquationBody::IfEquationBody>> {
    let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody> = body;
    assign_field!(
        body.then_eqns = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                metamodelica::nil();
            for mut e in (body.then_eqns.clone()).into_iter().cloned() {
                let __x = Pointer::apply(
                    e.clone(),
                    (std::sync::Arc::new({
                        let __pe_b1 = iter.clone();
                        let __pe_b2 = cref_map.clone();
                        move |__pe_a0| removeWhenEquation(__pe_a0, &__pe_b1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Equation::Equation>,
                                )
                                    -> Result<metamodelica::Ref<Equation::Equation>>
                                + 'static,
                        >),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        body.else_if = Util::applyOption(
            body.else_if.clone(),
            &({
                let __pe_b1 = iter.clone();
                let __pe_b2 = cref_map;
                move |__pe_a0| removeWhenEquationIfBody(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
        )?
    );
    Ok(body)
}

pub(crate) fn removeWhenEquationAlgorithmBody(
    mut in_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut out_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut condition_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>> =
        UnorderedSet::new(
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
    let mut tail_stmts_ptr: Pointer::Pointer<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> =
        Pointer::create(metamodelica::nil());
    out_stmts = List::flatten(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> =
                metamodelica::nil();
            for mut stmt in (in_stmts).into_iter().cloned() {
                let __x = removeWhenEquationStatement(stmt.clone(), condition_set.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    out_stmts = List::flatten(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> =
                metamodelica::nil();
            for mut stmt in (out_stmts).into_iter().cloned() {
                let __x = removeConditionEquation(stmt.clone(), condition_set.clone(), tail_stmts_ptr.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    out_stmts = listAppend(out_stmts, Pointer::access(tail_stmts_ptr));
    Ok(out_stmts)
}

pub(crate) fn removeWhenEquationStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut condition_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut out_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
    out_stmts = ({
        let mut stmts_acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> =
            metamodelica::nil();
        (match &*stmt {
            Statement::WHEN {
                branches: __stmt_branches,
                ..
            } => {
                let mut cond: metamodelica::Ref<Expression::NFExpression>;
                let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                for mut tpl in &*__stmt_branches.clone() {
                    (cond, stmts) = tpl.clone();
                    if isInitialCall(&cond)? {
                        out_stmts = stmts;
                    }
                    collectNonInitial(&cond, condition_set.clone())?;
                }
                out_stmts
            }
            Statement::FOR { body: __stmt_body, .. } => {
                let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                for mut body_stmt in &*__stmt_body.clone().reverse() {
                    stmts_acc = metamodelica::cons(
                        removeWhenEquationStatement(body_stmt.clone(), condition_set.clone())?,
                        stmts_acc,
                    );
                }
                stmts = List::flatten(stmts_acc)?;
                if !((stmts).is_empty()) {
                    assign_variant_field!(stmt => Statement::NFStatement::FOR; body = stmts);
                    out_stmts = list![stmt];
                } else {
                    out_stmts = metamodelica::nil();
                }
                out_stmts
            }
            _ => {
                list![stmt]
            }
        })
    });
    Ok(out_stmts)
}

pub(crate) fn removeConditionEquation(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut condition_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
    mut tail_stmts_ptr: Pointer::Pointer<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut out_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
    out_stmts = (match &*stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } if (UnorderedSet::contains(__stmt_lhs.clone(), condition_set.clone())?) => {
            let mut pre_set: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >;
            let mut post_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut tail_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            pre_set = UnorderedSet::new(
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
                __stmt_rhs.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = pre_set.clone();
                    move |__pe_a0| findPreVars(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            if UnorderedSet::isEmpty(pre_set.clone()) {
                out_stmts = list![stmt];
            } else {
                tail_stmts = metamodelica::cons(stmt, Pointer::access(tail_stmts_ptr.clone()));
                for mut pre_cref in &*UnorderedSet::toList(pre_set) {
                    post_cref = BVariable::getPartnerCref(
                        metamodelica::AsArg::as_arg(&pre_cref),
                        &fnptr!(
                            BVariable::getVarPre,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                        ),
                        false,
                    )?;
                    tail_stmts = metamodelica::cons(
                        metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                            lhs: Expression::fromCref(pre_cref.clone(), false)?,
                            rhs: Expression::fromCref(post_cref, false)?,
                            ty: ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&pre_cref), false)?,
                            source: DAE::emptyElementSource().clone(),
                        }),
                        tail_stmts,
                    );
                }
                Pointer::update(tail_stmts_ptr, tail_stmts);
            }
            out_stmts
        }
        _ => {
            list![stmt]
        }
    });
    Ok(out_stmts)
}

pub(crate) fn findPreVars(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut pre_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (BVariable::isPrevious(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBInitialization.mo"),
            )?)) =>
        {
            UnorderedSet::add(__exp_cref.clone(), pre_set)?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

pub(crate) fn replaceClockedFunctionsEqn(
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = eqn;
    Pointer::update(
        eqn.clone(),
        BEquation::Equation::map(
            Pointer::access(eqn.clone()),
            (std::sync::Arc::new(replaceClockedFunctions)
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
        )?,
    );
    Ok(eqn)
}

pub(crate) fn replaceClockedFunctions(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } if (metamodelica::stringEq(&(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?), &(literal!("$getPart")))) => {
            Expression::makeZero(&(Expression::typeOf(exp)))?
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn isInitialCall(mut condition: &metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match condition {
        Deref @ Expression::CALL { call: __condition_call } => Call::isNamed(metamodelica::AsArg::as_arg(&__condition_call), &(literal!("initial")))?,
        Deref @ Expression::LBINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::OR, .. }, exp1: __condition_exp1, exp2: __condition_exp2 } => isInitialCall(metamodelica::AsArg::as_arg(&__condition_exp1))? || isInitialCall(metamodelica::AsArg::as_arg(&__condition_exp2))?,
        Deref @ Expression::ARRAY { .. } => Array::any(var_field!((**condition).elements, Expression::NFExpression::ARRAY).clone(), &move |__a0: metamodelica::Ref<Expression::NFExpression>| isInitialCall(&__a0))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn collectNonInitial(
    mut condition: &metamodelica::Ref<Expression::NFExpression>,
    mut condition_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Expression::NFExpression>>>,
) -> Result<()> {
    let () = (match &**condition {
        Expression::CREF { .. } => {
            UnorderedSet::add(condition.clone(), condition_set)?;
            ()
        }
        Expression::ARRAY { .. } => {
            let __range0 = var_field!((**condition).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut elem in __range0 {
                collectNonInitial(&elem, condition_set.clone())?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectAlgorithmOutputs(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut outputs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let () = (match &*eqn {
        BEquation::Equation::ALGORITHM { alg, .. } => {
            let mut out_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            out_crefs = List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    > = metamodelica::nil();
                    for mut o in (alg.outputs.clone()).into_iter().cloned() {
                        let __x = BVariable::getRecordChildrenCrefOrSelf(o.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            for mut cr in &*out_crefs {
                UnorderedSet::add(cr.clone(), outputs.clone())?;
            }
            ()
        }
        _ => (),
    });
    Ok(eqn)
}
