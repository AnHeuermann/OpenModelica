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
use crate::NBFunctionAlias as FunctionAlias;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction as Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// Old Frontend Imports
// New Frontend Imports
// Backend imports
// Util
// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut mainFunc: Module::detectStatesInterface;
    let mut contFunc: Module::detectContinuousStatesInterface;
    let mut discFunc: Module::detectDiscreteStatesInterface;
    (mainFunc, contFunc, discFunc) = getModule()?;
    bdae = (match &*bdae {
        BackendDAE::MAIN { varData, eqData, .. } => {
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (varData, eqData) = mainFunc(varData.clone(), eqData.clone(), contFunc.clone(), discFunc.clone())?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBDetectStates.main"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<(
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<EquationPointers::EquationPointers>,
                        ) -> Result<(
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                        )> + 'static,
                >,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<EquationPointers::EquationPointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            ArcStr,
                        ) -> Result<(
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<EquationPointers::EquationPointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                            metamodelica::Ref<VariablePointers::VariablePointers>,
                        )> + 'static,
                >,
            )
                -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)>
            + 'static,
    >,
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
            ) -> Result<(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
            )> + 'static,
    >,
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                ArcStr,
            ) -> Result<(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
            )> + 'static,
    >,
)> {
    let mut mainFunc: Module::detectStatesInterface;
    let mut contFunc: Module::detectContinuousStatesInterface;
    let mut discFunc: Module::detectDiscreteStatesInterface;
    let mut flag: ArcStr = literal!("default");
    (mainFunc, contFunc, discFunc) = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => ((std::sync::Arc::new(detectStatesDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>) -> Result<(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>)> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, ArcStr) -> Result<(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>)> + 'static>) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> + 'static>), (std::sync::Arc::new(detectContinuousStatesDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>) -> Result<(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>)> + 'static>), (std::sync::Arc::new(detectDiscreteStatesDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, ArcStr) -> Result<(metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>)> + 'static>)),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((mainFunc, contFunc, discFunc))
}

/* =========================================================================
                              SUB ROUTINES
========================================================================= */
fn detectStatesDefault(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut continuousFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
            ) -> Result<(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
            )> + 'static,
    >,
    mut discreteFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                ArcStr,
            ) -> Result<(
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
            )> + 'static,
    >,
) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut disc_eqns: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut init_eqns: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut removed_eqns: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut states: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut discretes: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut discrete_states: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut clocked_states: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut previous: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut aux_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut newEqData: metamodelica::Ref<EqData::EqData>;
    (varData, eqData) = FunctionAlias::introduceSlicedStateAlias(varData, eqData, BPartition::Kind::ODE.clone())?;
    (varData, eqData) = (::match_deref::match_deref! { match &((varData.clone(), eqData.clone())) {
        (Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, Deref @ BEquation::EqData::EQ_DATA_SIM { .. }) => {
            (variables, unknowns, knowns, initials, states, derivatives, algebraics, aux_eqns) = continuousFunc(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).unknowns, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).knowns, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).initials, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).derivatives, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).algebraics, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone())?;
            (variables, disc_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*eqData).discretes, EqData::EqData::EQ_DATA_SIM).clone(), var_field!((*varData).knowns, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).initials, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).discretes, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).clocked_states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).previous, VarData::VarData::VAR_DATA_SIM).clone(), literal!("discrete equations"))?;
            (variables, disc_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables, var_field!((*eqData).clocked, EqData::EqData::EQ_DATA_SIM).clone(), knowns, initials, discretes, discrete_states, clocked_states, previous, literal!("clocked equations"))?;
            (variables, disc_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables, var_field!((*eqData).continuous, EqData::EqData::EQ_DATA_SIM).clone(), knowns, initials, discretes, discrete_states, clocked_states, previous, literal!("continuous equations"))?;
            (variables, init_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables, var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), knowns, initials, discretes, discrete_states, clocked_states, previous, literal!("initial equations"))?;
            (variables, removed_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables, var_field!((*eqData).removed, EqData::EqData::EQ_DATA_SIM).clone(), knowns, initials, discretes, discrete_states, clocked_states, previous, literal!("removed equations"))?;
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                variables = variables,
                unknowns = unknowns,
                knowns = knowns,
                initials = initials,
                derivatives = derivatives,
                algebraics = algebraics,
                discretes = discretes,
                discrete_states = discrete_states,
                clocked_states = clocked_states,
                previous = previous,
                states = states
            );
            newEqData = BEquation::EqData::addTypedList(eqData, &aux_eqns, EqData::EqType::CONTINUOUS.clone(), false)?;
            BEquation::EquationPointers::map(BEquation::EqData::getEquations(&newEqData)?, &({ let __pe_b1 = var_field!((*varData).state_order, VarData::VarData::VAR_DATA_SIM).clone(); move |__pe_a0| stateOrder(__pe_a0, __pe_b1.clone()) }))?;
            if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? && !(UnorderedMap::isEmpty(var_field!((*varData).state_order, VarData::VarData::VAR_DATA_SIM).clone())) {
                metamodelica::print(StringUtil::headline_4(&(literal!("[stateselection] State Order:")))?);
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*UnorderedMap::toString(var_field!((*varData).state_order, VarData::VarData::VAR_DATA_SIM).clone(), &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0), &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0), literal!("\n\t"), &(literal!(" --d/dt--> ")))?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            }
            (varData, newEqData)
        },
        _ => (varData, eqData),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((varData, eqData))
}

fn detectContinuousStatesDefault(
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut initials: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers> = unknowns;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers> = knowns;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers> = initials;
    let mut states: metamodelica::Ref<VariablePointers::VariablePointers> = states;
    let mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers> = derivatives;
    let mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers> = algebraics;
    let mut aux_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut acc_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut acc_derivatives: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut acc_aux_equations: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    > = Pointer::create(metamodelica::nil());
    let mut uniqueIndex: Pointer::Pointer<i32> = Pointer::create(0);
    let mut diffArgs: metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments> =
        Differentiate::DifferentiationArguments::default(
            Differentiate::DifferentiationType::TIME.clone(),
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
    BEquation::EquationPointers::mapExp(
        equations.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = acc_states.clone();
            let __pe_b2 = acc_derivatives.clone();
            let __pe_b3 = variables.scalarized.clone();
            move |__pe_a0| collectStatesAndDerivatives(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
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
    BEquation::EquationPointers::mapExp(
        equations,
        (std::sync::Arc::new({
            let __pe_b1 = acc_states.clone();
            let __pe_b2 = acc_derivatives.clone();
            let __pe_b3 = acc_aux_equations.clone();
            let __pe_b4 = uniqueIndex;
            let __pe_b5 = diffArgs;
            move |__pe_a0| {
                resolveGeneralDer(
                    __pe_a0,
                    __pe_b1.clone(),
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
    (variables, unknowns, knowns, initials, states, derivatives, algebraics) = updateStatesAndDerivatives(
        variables,
        unknowns,
        knowns,
        initials,
        states,
        derivatives,
        algebraics,
        Pointer::access(acc_states),
        &(Pointer::access(acc_derivatives)),
    )?;
    (variables, unknowns, knowns, initials, states, derivatives, algebraics) =
        promotePreferStates(variables, unknowns, knowns, initials, states, derivatives, algebraics)?;
    aux_eqns = Pointer::access(acc_aux_equations);
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? && !((aux_eqns).is_empty()) {
        metamodelica::print(StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[stateselection] ("));
                __mm_s.push_str(&*intString(((aux_eqns).len() as i32)));
                __mm_s.push_str(&*literal!(") Created auxiliary equations:"));
                ArcStr::from(__mm_s)
            }),
        )?);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*List::toString(
                aux_eqns.clone(),
                &({
                    let __pe_b1 = literal!("");
                    move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone())
                }),
                List::Style::NEWLINE_TAB.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((
        variables,
        unknowns,
        knowns,
        initials,
        states,
        derivatives,
        algebraics,
        aux_eqns,
    ))
}

fn detectDiscreteStatesDefault(
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut initials: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut discretes: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut discrete_states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut clocked_states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut previous: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut context: ArcStr,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
)> {
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers> = knowns;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers> = initials;
    let mut discretes: metamodelica::Ref<VariablePointers::VariablePointers> = discretes;
    let mut discrete_states: metamodelica::Ref<VariablePointers::VariablePointers> = discrete_states;
    let mut clocked_states: metamodelica::Ref<VariablePointers::VariablePointers> = clocked_states;
    let mut previous: metamodelica::Ref<VariablePointers::VariablePointers> = previous;
    let mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut acc_clocked_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut acc_previous: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    BEquation::EquationPointers::map(
        equations.clone(),
        &({
            let __pe_b1 = acc_discrete_states.clone();
            let __pe_b2 = acc_previous.clone();
            let __pe_b3 = variables.scalarized.clone();
            move |__pe_a0| collectDiscreteStatesFromWhen(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    BEquation::EquationPointers::mapExp(
        equations.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = acc_previous.clone();
            let __pe_b2 = acc_clocked_states.clone();
            let __pe_b3 = variables.scalarized.clone();
            move |__pe_a0| collectPreAndPrevious(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
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
    (
        variables,
        knowns,
        initials,
        discretes,
        discrete_states,
        clocked_states,
        previous,
    ) = updateDiscreteStatesAndPrevious(
        variables,
        knowns,
        initials,
        discretes,
        discrete_states,
        clocked_states,
        previous,
        Pointer::access(acc_discrete_states),
        Pointer::access(acc_clocked_states),
        Pointer::access(acc_previous),
        &context,
    )?;
    Ok((
        variables,
        equations,
        knowns,
        initials,
        discretes,
        discrete_states,
        clocked_states,
        previous,
    ))
}

fn collectStatesAndDerivatives(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut acc_states: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut acc_derivatives: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut scalarized: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: state_cref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } if (!(BVariable::checkCref(metamodelica::AsArg::as_arg(&state_cref), &fnptr!(BVariable::isStateDerivative, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?)) => {
            let mut res: metamodelica::Ref<Expression::NFExpression>;
            let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut state_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut der_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            state_var = BVariable::getVarPointer(metamodelica::AsArg::as_arg(&state_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?;
            if !(BVariable::isContinuous(state_var.clone(), false)?) {
                res = Expression::makeZero(&(ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&state_cref), false)?))?;
            } else {
                if BVariable::hasDerVar(state_var) {
                    der_cref = BVariable::getPartnerCref(metamodelica::AsArg::as_arg(&state_cref), &fnptr!(BVariable::getVarDer, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), scalarized)?;
                } else {
                    (der_cref, der_var) = BVariable::makeDerVar(state_cref.clone(), scalarized)?;
                    state_var = BVariable::getVarPointer(metamodelica::AsArg::as_arg(&state_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?;
                    BVariable::setStateDerivativeVar(state_var.clone(), der_var.clone());
                    Pointer::update(acc_states.clone(), metamodelica::cons(state_var, Pointer::access(acc_states)));
                    Pointer::update(acc_derivatives.clone(), metamodelica::cons(der_var, Pointer::access(acc_derivatives)));
                }
                res = Expression::fromCref(der_cref, false)?;
            }
            res
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn resolveGeneralDer(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut acc_states: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut acc_derivatives: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut acc_aux_equations: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    >,
    mut uniqueIndex: Pointer::Pointer<i32>,
    mut diffArgs: metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: Deref @ metamodelica::ListNode::Nil }, .. } } => {
            let mut state_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut state_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut der_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut returnExp: metamodelica::Ref<Expression::NFExpression>;
            let mut aux_equation: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
            let mut oDiffArgs: metamodelica::Ref<Differentiate::DifferentiationArguments::DifferentiationArguments>;
            if Expression::fold(arg.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: i32| checkAlgebraic(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, i32) -> Result<i32> + 'static>), 0)? > 1 {
                (state_var, state_cref, der_var, der_cref) = BVariable::makeAuxStateVar(Pointer::access(uniqueIndex.clone()), Some(arg.clone()))?;
                aux_equation = BEquation::Equation::makeAssignment(Expression::fromCref(state_cref, false)?, arg.clone(), uniqueIndex, &(arcstr::literal!(BVariable::AUXILIARY_STR)), crate::NBEquation::Iterator::interned_EMPTY(), BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None))?;
                returnExp = Expression::fromCref(der_cref, false)?;
                Pointer::update(acc_states.clone(), metamodelica::cons(state_var, Pointer::access(acc_states)));
                Pointer::update(acc_derivatives.clone(), metamodelica::cons(der_var, Pointer::access(acc_derivatives)));
                Pointer::update(acc_aux_equations.clone(), metamodelica::cons(aux_equation, Pointer::access(acc_aux_equations)));
            } else {
                (returnExp, oDiffArgs) = Differentiate::differentiateExpression(arg.clone(), diffArgs)?;
                returnExp = SimplifyExp::simplifyDump(returnExp, true, &(literal!("NBDetectStates.resolveGeneralDer")), &(literal!("")))?;
                if List::hasOneElement(&oDiffArgs.new_vars) {
                    der_var = (oDiffArgs.new_vars).head().cloned()?;
                    Pointer::update(acc_derivatives.clone(), metamodelica::cons(der_var.clone(), Pointer::access(acc_derivatives)));
                    Pointer::update(acc_states.clone(), metamodelica::cons(((BVariable::getVarState(der_var)?).0).ok_or("pattern mismatch")?, Pointer::access(acc_states)));
                } else if List::hasSeveralElements(&oDiffArgs.new_vars) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.resolveGeneralDer")); __mm_s.push_str(&*literal!(" failed because the number of algebraic variables were miscounted! ")); __mm_s.push_str(&*literal!("Expected: 0 or 1, got: ")); __mm_s.push_str(&*intString(((oDiffArgs.new_vars).len() as i32))); ArcStr::from(__mm_s) }])?;
                    return Err("fail");
                }
            }
            returnExp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn checkAlgebraic(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut i: i32) -> Result<i32> {
    let mut i: i32 = i;
    i = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (BVariable::isStateDerivative(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"),
            )?)) =>
        {
            i + 2
        }
        Expression::CREF { cref: __exp_cref, .. }
            if (BVariable::isAlgebraic(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"),
            )?)) =>
        {
            i + 1
        }
        _ => i,
    });
    Ok(i)
}

fn updateStatesAndDerivatives(
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut initials: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut acc_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut acc_derivatives: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
)> {
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers> = unknowns;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers> = knowns;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers> = initials;
    let mut states: metamodelica::Ref<VariablePointers::VariablePointers> = states;
    let mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers> = derivatives;
    let mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers> = algebraics;
    variables = BVariable::VariablePointers::addList(acc_derivatives, variables)?;
    unknowns = BVariable::VariablePointers::addList(acc_derivatives, unknowns)?;
    initials = BVariable::VariablePointers::addList(acc_derivatives, initials)?;
    derivatives = BVariable::VariablePointers::addList(acc_derivatives, derivatives)?;
    variables = BVariable::VariablePointers::addList(&acc_states, variables)?;
    states = BVariable::VariablePointers::addList(&acc_states, states)?;
    unknowns = BVariable::VariablePointers::removeList(&acc_states, unknowns)?;
    algebraics = BVariable::VariablePointers::removeList(&acc_states, algebraics)?;
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
        metamodelica::print(StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[stateselection] ("));
                __mm_s.push_str(&*intString(((acc_states).len() as i32)));
                __mm_s.push_str(&*literal!(") Natural states before index reduction:"));
                ArcStr::from(__mm_s)
            }),
        )?);
        if (acc_states).is_empty() {
            metamodelica::print(literal!("\t<no states>\n\n"));
        } else {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    acc_states,
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((variables, unknowns, knowns, initials, states, derivatives, algebraics))
}

fn collectPreAndPrevious(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut acc_clocked_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut scalarized: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn, arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::BOOLEAN { value: b }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            new_exp = (::match_deref::match_deref! { match &(r#fn.clone()) {
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: b.clone() }),
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: b.clone() }),
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, arguments: args, .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut old_exp: metamodelica::Ref<Expression::NFExpression>;
            (new_exp, old_exp) = preFromArgs(args.clone(), acc_previous, scalarized, &(literal!("previous")))?;
            let () = (match &*old_exp {
        Expression::CREF { cref: __old_exp_cref, .. } => {
            Pointer::update(acc_clocked_states.clone(), metamodelica::cons(BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__old_exp_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?, Pointer::access(acc_clocked_states)));
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.collectPreAndPrevious")); __mm_s.push_str(&*literal!(" failed because previous() can only contain component references, but contained: ")); __mm_s.push_str(&*Expression::toString(old_exp)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
    });
            new_exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, arguments: args, .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            (new_exp, _) = preFromArgs(args.clone(), acc_previous, scalarized, &(literal!("pre")))?;
            new_exp
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, arguments: args, .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut old_exp: metamodelica::Ref<Expression::NFExpression>;
            (new_exp, old_exp) = preFromArgs(args.clone(), acc_previous, scalarized, &(literal!("edge")))?;
            metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: old_exp.clone(), operator: Operator::makeAnd(Expression::typeOf(old_exp)), exp2: Expression::logicNegate(new_exp) })
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, arguments: args, .. } } => {
            let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
            let mut old_exp: metamodelica::Ref<Expression::NFExpression>;
            (new_exp, old_exp) = preFromArgs(args.clone(), acc_previous, scalarized, &(literal!("change")))?;
            metamodelica::Ref::new(Expression::NFExpression::RELATION { exp1: old_exp.clone(), operator: Operator::makeNotEqual(Expression::typeOf(old_exp)), exp2: new_exp, index: -1 })
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn preFromArgs(
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut scalarized: bool,
    mut context: &ArcStr,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut old_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut state_cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
    let mut pre_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut state_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut negated: bool;
    (state_var, old_exp, negated) = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_old_exp @ Deref @ Expression::CREF { cref: __esc_state_cref, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            old_exp = (*__esc_old_exp).clone();
            state_cref = (*__esc_state_cref).clone();
            (BVariable::getVarPointer(metamodelica::AsArg::as_arg(&state_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?, old_exp.clone(), false)
        },
        Deref @ metamodelica::ListNode::Cons { head: __esc_old_exp @ Deref @ Expression::LUNARY { exp: Deref @ Expression::CREF { cref: __esc_state_cref, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            old_exp = (*__esc_old_exp).clone();
            state_cref = (*__esc_state_cref).clone();
            (BVariable::getVarPointer(metamodelica::AsArg::as_arg(&state_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?, old_exp.clone(), true)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.preFromArgs")); __mm_s.push_str(&*literal!(" failed because of unexpected expression ")); __mm_s.push_str(&*context); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*List::toString(args, &Expression::toString, List::Style::FLAT.clone())?); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    pre_cref = getPreVar(state_cref, state_var, acc_previous, scalarized)?;
    new_exp = Expression::fromCref(pre_cref, false)?;
    if negated {
        new_exp = Expression::logicNegate(new_exp);
    }
    Ok((new_exp, old_exp))
}

fn updateDiscreteStatesAndPrevious(
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut initials: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut discretes: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut discrete_states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut clocked_states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut previous: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut acc_discrete_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut acc_clocked_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut acc_previous: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut context: &ArcStr,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
)> {
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers> = knowns;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers> = initials;
    let mut discretes: metamodelica::Ref<VariablePointers::VariablePointers> = discretes;
    let mut discrete_states: metamodelica::Ref<VariablePointers::VariablePointers> = discrete_states;
    let mut clocked_states: metamodelica::Ref<VariablePointers::VariablePointers> = clocked_states;
    let mut previous: metamodelica::Ref<VariablePointers::VariablePointers> = previous;
    variables = BVariable::VariablePointers::addList(&acc_previous, variables)?;
    knowns = BVariable::VariablePointers::addList(&acc_previous, knowns)?;
    initials = BVariable::VariablePointers::addList(&acc_previous, initials)?;
    previous = BVariable::VariablePointers::addList(&acc_previous, previous)?;
    discrete_states = BVariable::VariablePointers::addList(&acc_discrete_states, discrete_states)?;
    clocked_states = BVariable::VariablePointers::addList(&acc_clocked_states, clocked_states)?;
    discretes = BVariable::VariablePointers::removeList(&acc_discrete_states, discretes)?;
    discretes = BVariable::VariablePointers::removeList(&acc_clocked_states, discretes)?;
    discrete_states = BVariable::VariablePointers::removeList(&acc_clocked_states, discrete_states)?;
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
        if !((acc_discrete_states).is_empty()) {
            metamodelica::print(StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[stateselection] Natural discrete states from "));
                    __mm_s.push_str(&*context);
                    __mm_s.push_str(&*literal!(":"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    acc_discrete_states,
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        if !((acc_clocked_states).is_empty()) {
            metamodelica::print(StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[stateselection] Natural clocked states from "));
                    __mm_s.push_str(&*context);
                    __mm_s.push_str(&*literal!(":"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    acc_clocked_states,
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    if Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
        if !((acc_previous).is_empty()) {
            metamodelica::print(StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[discreteinfo] pre() and previous() variables from "));
                    __mm_s.push_str(&*context);
                    __mm_s.push_str(&*literal!(":"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    acc_previous,
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok((
        variables,
        knowns,
        initials,
        discretes,
        discrete_states,
        clocked_states,
        previous,
    ))
}

fn collectDiscreteStatesFromWhen(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut scalarized: bool,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let () = (match &*eqn {
        BEquation::Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            collectDiscreteStatesFromWhenBody(
                metamodelica::AsArg::as_arg(&__eqn_body),
                acc_discrete_states,
                acc_previous,
                scalarized,
            )?;
            ()
        }
        BEquation::Equation::FOR_EQUATION { body: __eqn_body, .. } => {
            for mut b_eqn in &*__eqn_body.clone() {
                collectDiscreteStatesFromWhen(
                    b_eqn.clone(),
                    acc_discrete_states.clone(),
                    acc_previous.clone(),
                    scalarized,
                )?;
            }
            ()
        }
        BEquation::Equation::IF_EQUATION { body: __eqn_body, .. } => {
            collectDiscreteStatesFromWhenInIf(
                metamodelica::AsArg::as_arg(&__eqn_body),
                acc_discrete_states,
                acc_previous,
                scalarized,
            )?;
            ()
        }
        _ => (),
    });
    Ok(eqn)
}

fn collectDiscreteStatesFromWhenBody(
    mut body: &metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut scalarized: bool,
) -> Result<()> {
    for mut body_stmt in &*body.when_stmts.clone() {
        let () = (::match_deref::match_deref! { match &(body_stmt.clone()) {
            Deref @ BEquation::WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: state_cref, .. }, .. } => {
                let mut state_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                state_var = BVariable::getVarPointer(metamodelica::AsArg::as_arg(&state_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?;
                BVariable::makeDiscreteStateVar(state_var.clone());
                getPreVar(state_cref.clone(), state_var.clone(), acc_previous.clone(), scalarized)?;
                Pointer::update(acc_discrete_states.clone(), metamodelica::cons(state_var, Pointer::access(acc_discrete_states.clone())));
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(())
}

fn collectDiscreteStatesFromWhenInIf(
    mut body: &metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut scalarized: bool,
) -> Result<()> {
    for mut eqn in &*body.then_eqns.clone() {
        collectDiscreteStatesFromWhen(
            Pointer::access(eqn.clone()),
            acc_discrete_states.clone(),
            acc_previous.clone(),
            scalarized,
        )?;
    }
    if (body.else_if).is_some() {
        collectDiscreteStatesFromWhenInIf(
            &(body.else_if.clone().ok_or("pattern mismatch")?),
            acc_discrete_states,
            acc_previous,
            scalarized,
        )?;
    }
    Ok(())
}

fn getPreVar(
    mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut scalarized: bool,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut pre_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let (mut pre, _): (
        Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        ArcStr,
    ) = BVariable::getVarPre(var_ptr.clone());
    let mut pre_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    if (pre).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(pre) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        pre_var = metamodelica::Own::own(__pa0);
        pre_cref = BVariable::getVarName(pre_var);
        pre_cref = ComponentRef::copySubscripts(&var_cref, pre_cref)?;
    } else {
        if !(scalarized) {
            (pre_cref, pre_var) = BVariable::makePreVar(ComponentRef::stripSubscriptsAll(&var_cref))?;
            pre_cref = ComponentRef::copySubscripts(&var_cref, pre_cref)?;
        } else {
            (pre_cref, pre_var) = BVariable::makePreVar(var_cref)?;
        }
        Pointer::update(
            acc_previous.clone(),
            metamodelica::cons(pre_var, Pointer::access(acc_previous)),
        );
    }
    Ok(pre_cref)
}

pub(crate) fn findDiscreteStatesFromWhenBody(
    mut body: &metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<()> {
    for mut body_stmt in &*body.when_stmts.clone() {
        let () = (::match_deref::match_deref! { match &(body_stmt.clone()) {
            Deref @ BEquation::WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: state_cref, .. }, .. } => {
                let mut state_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut pre_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                state_var = BVariable::getVarPointer(metamodelica::AsArg::as_arg(&state_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?;
                let () = (match (BVariable::getVarPre(state_var.clone())).0 {
            Some(mut __esc_pre_var) => {
                pre_var = __esc_pre_var.clone();
                Pointer::update(acc_previous.clone(), metamodelica::cons(pre_var, Pointer::access(acc_previous.clone())));
                ()
            },
            _ => (),
        });
                Pointer::update(acc_discrete_states.clone(), metamodelica::cons(state_var, Pointer::access(acc_discrete_states.clone())));
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(())
}

pub(crate) fn stateOrder(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut state_order: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let () = (::match_deref::match_deref! { match &(&*eqn) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: __esc_lhs @ Deref @ Expression::CREF { .. }, rhs: __esc_rhs @ Deref @ Expression::CREF { .. }, .. } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            updateStateOrder(var_field!((*lhs).cref, Expression::NFExpression::CREF), var_field!((*rhs).cref, Expression::NFExpression::CREF), state_order)?;
            ()
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: __esc_lhs @ Deref @ Expression::CREF { .. }, rhs: __esc_rhs @ Deref @ Expression::CREF { .. }, .. } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            updateStateOrder(var_field!((*lhs).cref, Expression::NFExpression::CREF), var_field!((*rhs).cref, Expression::NFExpression::CREF), state_order)?;
            ()
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: __eqn_body, .. } => {
            for mut b in &*__eqn_body.clone() {
                stateOrder(b.clone(), state_order.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

pub(crate) fn promotePreferStates(
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut initials: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut states: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<(
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
)> {
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers> = variables;
    let mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers> = unknowns;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers> = knowns;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers> = initials;
    let mut states: metamodelica::Ref<VariablePointers::VariablePointers> = states;
    let mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers> = derivatives;
    let mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers> = algebraics;
    let mut acc_prefer_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut acc_prefer_ders: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut der_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    if BVariable::VariablePointers::size(&states) > 0 {
        for mut alg_ptr in &*BVariable::VariablePointers::toList(&algebraics)? {
            if BVariable::isStateSelect(alg_ptr.clone(), StateSelect::PREFER.clone()) {
                (der_cref, der_var) =
                    BVariable::makeDerVar(BVariable::getVarName(alg_ptr.clone()), variables.scalarized.clone())?;
                BVariable::setVarKind(
                    alg_ptr.clone(),
                    metamodelica::Ref::new(VariableKind::VariableKind::STATE {
                        index: 1,
                        derivative: Some(PointerWeak::downgrade(der_var.clone())),
                        natural: false,
                    }),
                );
                acc_prefer_states = metamodelica::cons(alg_ptr.clone(), acc_prefer_states);
                acc_prefer_ders = metamodelica::cons(der_var, acc_prefer_ders);
            }
        }
        if !((acc_prefer_states).is_empty()) {
            states = BVariable::VariablePointers::addList(&acc_prefer_states, states)?;
            unknowns = BVariable::VariablePointers::removeList(&acc_prefer_states, unknowns)?;
            algebraics = BVariable::VariablePointers::removeList(&acc_prefer_states, algebraics)?;
            variables = BVariable::VariablePointers::addList(&acc_prefer_ders, variables)?;
            unknowns = BVariable::VariablePointers::addList(&acc_prefer_ders, unknowns)?;
            initials = BVariable::VariablePointers::addList(&acc_prefer_ders, initials)?;
            derivatives = BVariable::VariablePointers::addList(&acc_prefer_ders, derivatives)?;
            if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
                metamodelica::print(StringUtil::headline_4(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("[stateselection] ("));
                        __mm_s.push_str(&*intString(((acc_prefer_states).len() as i32)));
                        __mm_s.push_str(&*literal!(") Forced states by StateSelect.PREFER:"));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*List::toString(
                        acc_prefer_states,
                        &BVariable::pointerToString,
                        List::Style::NEWLINE_TAB.clone(),
                    )?);
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
    }
    Ok((variables, unknowns, knowns, initials, states, derivatives, algebraics))
}

pub(crate) fn updateStateOrder(
    mut lhs: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhs: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut state_order: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<()> {
    let mut state: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &((BVariable::getVarKind(BVariable::getVarPointer(lhs, metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?), BVariable::getVarKind(BVariable::getVarPointer(rhs, metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBDetectStates.mo"))?))) {
        (_, Deref @ VariableKind::STATE_DER { state: __esc_state, .. }) => {
            state = (*__esc_state).clone();
            UnorderedMap::add(BVariable::getVarName(PointerWeak::upgrade(state.clone())?), ComponentRef::stripSubscriptsAll(lhs), state_order)?;
            ()
        },
        (Deref @ VariableKind::STATE_DER { state: __esc_state, .. }, _) => {
            state = (*__esc_state).clone();
            UnorderedMap::add(BVariable::getVarName(PointerWeak::upgrade(state.clone())?), ComponentRef::stripSubscriptsAll(rhs), state_order)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
