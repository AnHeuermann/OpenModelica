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
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBInline as Inline;
use crate::NBJacobian as Jacobian;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::UnorderedSet;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend imports
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::daeModeInterface;
    if '__try0: {
        func = unwrap_break_err!(getModule(), '__try0);
        bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { ode, eqData: eqData @ Deref @ EqData::EQ_DATA_SIM { .. }, varData: Deref @ BVariable::VarData::VAR_DATA_SIM { variables, .. }, .. } => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; dae = Some(unwrap_break_err!(func(ode.clone(), variables.clone(), var_field!((**eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone()), '__try0)));
            bdae.clone()
        },
        _ => {
            unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDAEMode.main")); __mm_s.push_str(&*literal!(" failed due to wrong BackendDAE record!")); ArcStr::from(__mm_s) }]), '__try0);
            break '__try0 Err::<_, _>("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        bdae = unwrap_break_err!(Causalize::main(bdae.clone(), Partition::Kind::DAE.clone()), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDAEMode.main")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
    }
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                Pointer::Pointer<i32>,
            )
                -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>>
            + 'static,
    >,
> {
    let mut func: Module::daeModeInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => (std::sync::Arc::new(daeModeDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>, metamodelica::Ref<VariablePointers::VariablePointers>, Pointer::Pointer<i32>) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> + 'static>),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

fn daeModeDefault(
    mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut uniqueIndex: Pointer::Pointer<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> {
    let mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = partitions;
    let mut new_partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> =
        metamodelica::nil();
    for mut part in &*partitions {
        let mut part = part.clone();
        new_partitions = (::match_deref::match_deref! { match &(part.association.clone()) {
            association @ Deref @ Partition::Association::CONTINUOUS { .. } => {
                let mut new_eqns: metamodelica::Ref<EquationPointers::EquationPointers>;
                let mut new_vars: metamodelica::Ref<VariablePointers::VariablePointers>;
                let mut association = (*association).clone();
                assign_variant_field!(association => Partition::Association::Association::CONTINUOUS; kind = Partition::Kind::DAE.clone());
                assign_field!(
                    part.association = association.clone(),
                    part.strongComponents = StrongComponent::sortDAEModeComponents(part.strongComponents.clone(), &variables, uniqueIndex.clone())?
                );
                (new_eqns, new_vars) = (match part.strongComponents.clone() {
            Some(mut new_c) => {
                let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut new_eqns_set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
                let mut new_vars_set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;
                new_eqns_set = UnorderedSet::new((std::sync::Arc::new(Equation::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32> + 'static>), (std::sync::Arc::new(Equation::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<bool> + 'static>), 13);
                new_vars_set = UnorderedSet::new((std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
                let __range0 = new_c.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut comp in __range0 {
                    eqns = StrongComponent::getEquations(comp.clone())?;
                    vars = StrongComponent::getVariables(comp)?;
                    for mut eqn in &*eqns {
                        UnorderedSet::add(eqn.clone(), new_eqns_set.clone())?;
                    }
                    for mut var in &*vars {
                        UnorderedSet::add(var.clone(), new_vars_set.clone())?;
                    }
                }
                (EquationPointers::fromList(&(UnorderedSet::toList(new_eqns_set)))?, BVariable::VariablePointers::fromList(&(UnorderedSet::toList(new_vars_set)), false)?)
            },
            _ => {
                (part.equations.clone(), part.unknowns.clone())
            },
        });
                assign_field!(
                    part.equations = new_eqns,
                    part.daeUnknowns = Some(part.unknowns.clone()),
                    part.unknowns = new_vars
                );
                if (Partition::Partition::isEmpty(&part)?) {new_partitions} else {metamodelica::cons(part, new_partitions)}
            },
            _ => {
                new_partitions
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    partitions = new_partitions.reverse();
    Ok(partitions)
}
