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

use crate::NSimVar::ConvertMemo;
use crate::NSimVar::ExtObjInfo;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

// Frontend imports
// SimCode imports
// Old SimCode imports
// Util imports
pub(crate) fn createSimCodeMap(
    mut simVars: &metamodelica::Ref<SimVars::SimVars>,
    mut extObjInfo: &metamodelica::Ref<ExtObjInfo::ExtObjInfo>,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
    >,
> {
    let mut simcode_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
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
    addListSimCodeMap(&simVars.stateVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.derivativeVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.algVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.discreteAlgVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.intAlgVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.boolAlgVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.inputVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.outputVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.aliasVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.intAliasVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.boolAliasVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.paramVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.intParamVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.boolParamVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.stringAlgVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.stringParamVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.stringAliasVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.extObjVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.constVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.intConstVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.boolConstVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.stringConstVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.residualVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.jacobianVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.seedVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.realOptimizeConstraintsVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.realOptimizeFinalConstraintsVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.sensitivityVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.dataReconSetcVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.dataReconinputVars, simcode_map.clone())?;
    addListSimCodeMap(&simVars.dataReconSetBVars, simcode_map.clone())?;
    addListSimCodeMap(&extObjInfo.objects, simcode_map.clone())?;
    Ok(simcode_map)
}

pub(crate) fn addListSimCodeMap(
    mut simVars: &metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
    mut simcode_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
    >,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
    >,
> {
    let mut simcode_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
    > = simcode_map;
    for mut var in &**simVars {
        UnorderedMap::add(
            SimVar::getName(metamodelica::AsArg::as_arg(&var)),
            var.clone(),
            simcode_map.clone(),
        )?;
    }
    Ok(simcode_map)
}

pub(crate) fn convertSimCodeMap(
    mut simcode_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
    >,
    mut memo: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            (metamodelica::Ref<SimVar::SimVar>, metamodelica::Ref<SimCodeVar::SimVar>),
        >,
    >,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<SimCodeVar::SimVar>,
            )>,
        >,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut old_ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrefSimVar::FuncHashCref,
            HashTableCrefSimVar::FuncCrefEqual,
            HashTableCrefSimVar::FuncCrefStr,
            HashTableCrefSimVar::FuncExpStr,
        ),
    );
    old_ht = HashTableCrefSimVar::emptyHashTableSized(UnorderedMap::size(simcode_map.clone()));
    for mut var in &*UnorderedMap::valueList(simcode_map) {
        old_ht = HashTableCrefSimVar::addSimVarToHashTable(
            SimVar::convertMemoized(metamodelica::AsArg::as_arg(&var), memo.clone())?,
            old_ht,
        )?;
    }
    Ok(old_ht)
}
