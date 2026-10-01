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
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBEvents::EventInfo;
use crate::NBInline as Inline;
use crate::NBJacobian::JacobianType;
use crate::NBMatching as Matching;
use crate::NBPartition as Partition;
use crate::NBPartitioning::ClockedInfo;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Util imports
pub type wrapper = std::sync::Arc<
    dyn ::std::ops::Fn(metamodelica::Ref<Jacobian::NBackendDAE>) -> Result<metamodelica::Ref<Jacobian::NBackendDAE>>
        + 'static,
>;

pub(crate) fn moduleClockString(mut name_clock: &(ArcStr, metamodelica::Real)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut name: ArcStr;
    let mut clck: metamodelica::Real;
    (name, clck) = name_clock.clone();
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\t"));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*StringUtil::repeat(literal!("."), 50 - ((name).len() as i32))?);
        __mm_s.push_str(&*System::sprintff(literal!("%.4g"), clck)?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

// =========================================================================
//                                MAIN MODULES
// =========================================================================
//                               PARTITIONING
// *************************************************************************
pub type partitioningInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            Partition::Kind,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            metamodelica::Ref<EquationPointers::EquationPointers>,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            metamodelica::Ref<EquationPointers::EquationPointers>,
            metamodelica::Ref<ClockedInfo::ClockedInfo>,
        ) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>>
        + 'static,
>;

//                               CAUSALIZE
// *************************************************************************
pub type causalizeInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Partition::Partition::Partition>,
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
            >,
            metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        ) -> Result<(
            metamodelica::Ref<Partition::Partition::Partition>,
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        )> + 'static,
>;

//                           RESOLVING SINGULARITIES
//                  Index Reduction + Balance Initialization
// *************************************************************************
pub type resolveSingularitiesInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<Adjacency::Matrix::Matrix>,
            metamodelica::Ref<Adjacency::Matrix::Matrix>,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            metamodelica::Ref<EquationPointers::EquationPointers>,
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            Partition::Kind,
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
            >,
            metamodelica::Ref<Matching::NBMatching>,
            Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>,
        ) -> Result<(
            metamodelica::Ref<Adjacency::Matrix::Matrix>,
            metamodelica::Ref<Adjacency::Matrix::Matrix>,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            metamodelica::Ref<EquationPointers::EquationPointers>,
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            bool,
        )> + 'static,
>;

//                               DAEMODE
// *************************************************************************
pub type daeModeInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            Pointer::Pointer<i32>,
        ) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>>
        + 'static,
>;

// =========================================================================
//                         MANDATORY PRE-OPT MODULES
// =========================================================================
//                            COLLECT EVENTS
// *************************************************************************
pub type eventsInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            metamodelica::Ref<EventInfo::EventInfo>,
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
            >,
        ) -> Result<(
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            metamodelica::Ref<EventInfo::EventInfo>,
        )> + 'static,
>;

//                               DETECT STATES
// *************************************************************************
pub type detectStatesInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            detectContinuousStatesInterface,
            detectDiscreteStatesInterface,
        ) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)>
        + 'static,
>;

pub type detectContinuousStatesInterface = std::sync::Arc<
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
>;

pub type detectDiscreteStatesInterface = std::sync::Arc<
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
>;

// =========================================================================
//                         Optional PRE-OPT MODULES
// =========================================================================
//                                 ALIAS
// *************************************************************************
pub type functionAliasInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            Partition::Kind,
        ) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)>
        + 'static,
>;

pub type aliasInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<EqData::EqData>,
            Partition::Kind,
        ) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)>
        + 'static,
>;

//                                 INLINE
// *************************************************************************
pub type inlineInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            metamodelica::Ref<EqData::EqData>,
            metamodelica::Ref<VarData::VarData>,
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
            >,
            metamodelica::List<DAE::InlineType>,
            bool,
        ) -> Result<(metamodelica::Ref<EqData::EqData>, metamodelica::Ref<VarData::VarData>)>
        + 'static,
>;

// =========================================================================
//                         MANDATORY POST-OPT MODULES
// =========================================================================
//                               JACOBIAN
// *************************************************************************
pub type jacobianInterface = std::sync::Arc<
    dyn ::std::ops::Fn(
            ArcStr,
            JacobianType,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            metamodelica::Ref<VariablePointers::VariablePointers>,
            metamodelica::Ref<EquationPointers::EquationPointers>,
            Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
            Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
            >,
            bool,
        ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>
        + 'static,
>;

// =========================================================================
//                         Optional POST-OPT MODULES
// =========================================================================
//                                 TEARING
// *************************************************************************
pub type tearingInterface = std::sync::Arc<
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
>;
