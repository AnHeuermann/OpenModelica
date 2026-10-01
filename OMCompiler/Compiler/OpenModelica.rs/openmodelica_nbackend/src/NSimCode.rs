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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEvents::EventInfo;
use crate::NBPartition as Partition;
use crate::NBPartitioning::ClockedInfo;
use crate::NBStrongComponent::AliasInfo;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCodeUtil as SimCodeUtil;
use crate::NSimGenericCall as SimGenericCall;
use crate::NSimJacobian::SimJacobian;
use crate::NSimPartition as SimPartition;
use crate::NSimStrongComponent as SimStrongComponent;
use crate::NSimVar::ConvertMemo;
use crate::NSimVar::ExtObjInfo;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarInfo;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression as OldExpression;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBuiltinCall as BuiltinCall;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_program_util::ProgramUtil;
use openmodelica_simcode_types::AvlTreeCRToInt;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_simcode_types::SimCodeFunction as OldSimCodeFunction;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeFunctionUtil as OldSimCodeFunctionUtil;
use openmodelica_simcode_util::SimCodeUtilShared;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// OF imports
// NF imports
// Old Backend imports
// Backend imports
// SimCode imports
// Old SimCode imports
// Util imports
// Script imports
/// Unique simulation code indices
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SimCodeIndices {
    pub uniqueIndex: i32,
    pub realVarIndex: i32,
    pub integerVarIndex: i32,
    pub booleanVarIndex: i32,
    pub stringVarIndex: i32,
    pub enumerationVarIndex: i32,
    pub realParamIndex: i32,
    pub integerParamIndex: i32,
    pub booleanParamIndex: i32,
    pub stringParamIndex: i32,
    pub enumerationParamIndex: i32,
    pub realAliasIndex: i32,
    pub integerAliasIndex: i32,
    pub booleanAliasIndex: i32,
    pub stringAliasIndex: i32,
    pub enumerationAliasIndex: i32,
    pub equationIndex: i32,
    pub linearSystemIndex: i32,
    pub nonlinearSystemIndex: i32,
    pub jacobianIndex: i32,
    pub residualIndex: i32,
    pub implicitIndex: i32,
    pub extObjIndex: i32,
    pub alias_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<AliasInfo::AliasInfo>, i32>>,
    pub generic_call_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Identifier::Identifier>, i32>>,
}

impl metamodelica::gc::MMTrace for SimCodeIndices {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.uniqueIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.realVarIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.integerVarIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.booleanVarIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringVarIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.enumerationVarIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.realParamIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.integerParamIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.booleanParamIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringParamIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.enumerationParamIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.realAliasIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.integerAliasIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.booleanAliasIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringAliasIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.enumerationAliasIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.equationIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.linearSystemIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nonlinearSystemIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.residualIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.implicitIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extObjIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.alias_map, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generic_call_map, __mmv)?;
        Ok(())
    }
}
impl Default for SimCodeIndices {
    fn default() -> Self {
        Self {
            uniqueIndex: Default::default(),
            realVarIndex: Default::default(),
            integerVarIndex: Default::default(),
            booleanVarIndex: Default::default(),
            stringVarIndex: Default::default(),
            enumerationVarIndex: Default::default(),
            realParamIndex: Default::default(),
            integerParamIndex: Default::default(),
            booleanParamIndex: Default::default(),
            stringParamIndex: Default::default(),
            enumerationParamIndex: Default::default(),
            realAliasIndex: Default::default(),
            integerAliasIndex: Default::default(),
            booleanAliasIndex: Default::default(),
            stringAliasIndex: Default::default(),
            enumerationAliasIndex: Default::default(),
            equationIndex: Default::default(),
            linearSystemIndex: Default::default(),
            nonlinearSystemIndex: Default::default(),
            jacobianIndex: Default::default(),
            residualIndex: Default::default(),
            implicitIndex: Default::default(),
            extObjIndex: Default::default(),
            alias_map: Default::default(),
            generic_call_map: Default::default(),
        }
    }
}

pub type SIM_CODE_INDICES = SimCodeIndices;

pub mod Identifier {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Identifier {
        pub eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        pub var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        pub resizable: bool,
    }

    impl metamodelica::gc::MMTrace for Identifier {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.eqn, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_cref, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.resizable, __mmv)?;
            Ok(())
        }
    }
    impl Default for Identifier {
        fn default() -> Self {
            Self {
                eqn: Default::default(),
                var_cref: Default::default(),
                resizable: Default::default(),
            }
        }
    }

    pub type IDENTIFIER = Identifier;

    pub(crate) fn toString(mut ident: &metamodelica::Ref<Identifier>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("cref: "));
            __mm_s.push_str(&*ComponentRef::toString(&ident.var_cref)?);
            __mm_s.push_str(&*literal!("\neqn: "));
            __mm_s.push_str(&*BEquation::Equation::pointerToString(ident.eqn.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n(resizable="));
            __mm_s.push_str(&*boolString(ident.resizable.clone()));
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn hash(mut ident: &metamodelica::Ref<Identifier>) -> Result<i32> {
        let mut i: i32 = stringHashDjb2(&(toString(ident)?));
        Ok(i)
    }

    pub(crate) fn isEqual(
        mut ident1: &metamodelica::Ref<Identifier>,
        mut ident2: &metamodelica::Ref<Identifier>,
    ) -> Result<bool> {
        let mut b: bool = BEquation::Equation::equalName(ident1.eqn.clone(), ident2.eqn.clone())?
            && ComponentRef::isEqual(&ident1.var_cref, &ident2.var_cref)?;
        Ok(b)
    }
}

pub(crate) fn EMPTY_SIM_CODE_INDICES() -> SimCodeIndices {
    let mut indices: SimCodeIndices = SimCodeIndices {
        uniqueIndex: 1,
        realVarIndex: 0,
        integerVarIndex: 0,
        booleanVarIndex: 0,
        stringVarIndex: 0,
        enumerationVarIndex: 0,
        realParamIndex: 0,
        integerParamIndex: 0,
        booleanParamIndex: 0,
        stringParamIndex: 0,
        enumerationParamIndex: 0,
        realAliasIndex: 0,
        integerAliasIndex: 0,
        booleanAliasIndex: 0,
        stringAliasIndex: 0,
        enumerationAliasIndex: 0,
        equationIndex: 1,
        linearSystemIndex: 0,
        nonlinearSystemIndex: 0,
        jacobianIndex: 0,
        residualIndex: 0,
        implicitIndex: 0,
        extObjIndex: 0,
        alias_map: UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<AliasInfo::AliasInfo>| AliasInfo::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<AliasInfo::AliasInfo>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<AliasInfo::AliasInfo>,
                      __a1: metamodelica::Ref<AliasInfo::AliasInfo>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AliasInfo::isEqual(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<AliasInfo::AliasInfo>,
                            metamodelica::Ref<AliasInfo::AliasInfo>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
        generic_call_map: UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Identifier::Identifier>| Identifier::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Identifier::Identifier>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Identifier::Identifier>,
                      __a1: metamodelica::Ref<Identifier::Identifier>| {
                    Identifier::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Identifier::Identifier>,
                            metamodelica::Ref<Identifier::Identifier>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
    };
    indices
}

pub mod SimCode {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SimCode {
        pub modelInfo: metamodelica::Ref<ModelInfo::ModelInfo>,
        /// shared literals
        pub literals: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        pub recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>,
        /// Names of all external functions that are called
        pub externalFunctionIncludes: metamodelica::List<ArcStr>,
        /// Generic for-loop and array calls
        pub generic_loop_calls: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>>,
        /// state and strictly input dependent variables. they are not inserted into any partion
        pub independent: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// All simulation system blocks
        pub allSim: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Only ode blocks for integrator
        pub ode: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>,
        /// Additional purely algebraic blocks
        pub algebraic: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>,
        /// Clocked Partitions
        pub clockedPartitions: metamodelica::List<metamodelica::Ref<SimPartition::NSimPartition>>,
        /// Blocks for nominal value equations
        pub nominal: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for min value equations
        pub min: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for max value equations
        pub max: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for parameter equations
        pub param: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for equations without return value
        pub no_ret: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for algorithms and asserts
        pub algorithms: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for zero crossing functions
        pub event_blocks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for jacobian equations
        pub jac_blocks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for start value equations
        pub start: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for initial equations
        pub init: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for initial lambda 0 equations (homotopy)
        pub init_0: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// Blocks for initial equations without return value
        pub init_no_ret: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// List of discrete variables
        pub discreteVars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        pub extObjInfo: metamodelica::Ref<ExtObjInfo::ExtObjInfo>,
        pub makefileParams: SimCodeFunction::MakefileParams,
        /// List of symbolic jacobians
        pub jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>,
        pub simulationSettingsOpt: Option<OldSimCode::SimulationSettings>,
        pub fileNamePrefix: ArcStr,
        pub simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        pub equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimStrongComponent::Block::Block>,
            >,
        >,
        pub eventInfo: metamodelica::Ref<EventInfo::EventInfo>,
        /// Simulation system in case of DAEMode
        pub daeModeData: Option<metamodelica::Ref<DaeModeData::DaeModeData>>,
        pub inlineEquations: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
    }

    impl metamodelica::gc::MMTrace for SimCode {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.modelInfo, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.literals, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.recordDecls, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.externalFunctionIncludes, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.generic_loop_calls, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.independent, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.allSim, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.ode, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.algebraic, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.clockedPartitions, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nominal, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.min, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.max, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.param, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.no_ret, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.algorithms, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.event_blocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jac_blocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.start, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.init, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.init_0, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.init_no_ret, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.discreteVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.extObjInfo, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.makefileParams, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jacobians, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.simulationSettingsOpt, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.fileNamePrefix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.simcode_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.equation_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eventInfo, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.daeModeData, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.inlineEquations, __mmv)?;
            Ok(())
        }
    }
    impl Default for SimCode {
        fn default() -> Self {
            Self {
                modelInfo: Default::default(),
                literals: Default::default(),
                recordDecls: Default::default(),
                externalFunctionIncludes: Default::default(),
                generic_loop_calls: Default::default(),
                independent: Default::default(),
                allSim: Default::default(),
                ode: Default::default(),
                algebraic: Default::default(),
                clockedPartitions: Default::default(),
                nominal: Default::default(),
                min: Default::default(),
                max: Default::default(),
                param: Default::default(),
                no_ret: Default::default(),
                algorithms: Default::default(),
                event_blocks: Default::default(),
                jac_blocks: Default::default(),
                start: Default::default(),
                init: Default::default(),
                init_0: Default::default(),
                init_no_ret: Default::default(),
                discreteVars: Default::default(),
                extObjInfo: Default::default(),
                makefileParams: Default::default(),
                jacobians: Default::default(),
                simulationSettingsOpt: Default::default(),
                fileNamePrefix: Default::default(),
                simcode_map: Default::default(),
                equation_map: Default::default(),
                eventInfo: Default::default(),
                daeModeData: Default::default(),
                inlineEquations: Default::default(),
            }
        }
    }

    pub type SIM_CODE = SimCode;

    pub fn toString(mut simCode: &metamodelica::Ref<SimCode>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut idx: i32 = 1;
        r#str = StringUtil::headline_1(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("SimCode "));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*simCode.fileNamePrefix);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }),
        )?;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*ModelInfo::toString(&simCode.modelInfo)?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*ExtObjInfo::toString(&simCode.extObjInfo)?);
            ArcStr::from(__mm_s)
        };
        if !((simCode.init_0).is_empty()) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*SimStrongComponent::Block::listToString(
                    &simCode.init_0,
                    literal!("  "),
                    &(literal!("Initial Partition (Lambda = 0)")),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimStrongComponent::Block::listToString(
                &simCode.init,
                literal!("  "),
                &(literal!("Initial Partition")),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        for mut blck_lst in &*simCode.ode.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*SimStrongComponent::Block::listToString(
                    metamodelica::AsArg::as_arg(&blck_lst),
                    literal!("  "),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("ODE Partition "));
                        __mm_s.push_str(&*intString(idx));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            idx = idx + 1;
        }
        idx = 1;
        for mut blck_lst in &*simCode.algebraic.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*SimStrongComponent::Block::listToString(
                    metamodelica::AsArg::as_arg(&blck_lst),
                    literal!("  "),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Algebraic Partition "));
                        __mm_s.push_str(&*intString(idx));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            idx = idx + 1;
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimStrongComponent::Block::listToString(
                &simCode.event_blocks,
                literal!("  "),
                &(literal!("Event Partition")),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        if !((simCode.clockedPartitions).is_empty()) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*SimPartition::listToString(
                    &simCode.clockedPartitions,
                    literal!("  "),
                    &(literal!("Clocked Partitions")),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        if !((simCode.literals).is_empty()) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Shared Literals")))?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    simCode.literals.clone(),
                    &Expression::toString,
                    List::Style::NEWLINE_INDENT.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
        }
        if !((simCode.generic_loop_calls).is_empty()) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Generic Calls")))?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    simCode.generic_loop_calls.clone(),
                    &move |__a0: metamodelica::Ref<SimGenericCall::NSimGenericCall>| SimGenericCall::toString(&__a0),
                    List::Style::NEWLINE_INDENT.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
        }
        if (simCode.daeModeData).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*DaeModeData::toString(
                    &(simCode.daeModeData.clone().ok_or("pattern mismatch")?),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        for mut jac in &*simCode.jacobians.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*SimJacobian::toString(metamodelica::AsArg::as_arg(&jac))?);
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*EventInfo::toString(&simCode.eventInfo)?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub fn create(
        mut bdae: &metamodelica::Ref<BackendDAE::NBackendDAE>,
        mut name: metamodelica::Ref<Absyn::Path>,
        mut fileNamePrefix: ArcStr,
        mut simSettingsOpt: Option<OldSimCode::SimulationSettings>,
        mut program: Absyn::Program,
    ) -> Result<(metamodelica::Ref<SimCode>, metamodelica::Ref<AvlTreePathFunction::Tree>)> {
        type mapExp = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut simCode: metamodelica::Ref<SimCode>;
        let mut oldFunctionTree: metamodelica::Ref<AvlTreePathFunction::Tree> =
            metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
        simCode = ({
            let mut literals_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<Expression::NFExpression>, i32>,
            > = UnorderedMap::new(
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
                1,
            );
            let mut allSim: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>> =
                metamodelica::nil();
            let mut event_blocks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>> =
                metamodelica::nil();
            (::match_deref::match_deref! { match bdae {
                Deref @ BackendDAE::MAIN { varData: varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, alg_event: __bdae_alg_event, algebraic: __bdae_algebraic, clocked: __bdae_clocked, clockedInfo: __bdae_clockedInfo, dae: __bdae_dae, eventInfo: __bdae_eventInfo, init: __bdae_init, init_0: __bdae_init_0, ode: __bdae_ode, ode_event: __bdae_ode_event, parameters: __bdae_parameters, .. } => {
                    let mut funcMap: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Function::Function>>>;
                    let mut residual_vars: metamodelica::Ref<VariablePointers::VariablePointers>;
                    let mut vars: metamodelica::Ref<SimVars::SimVars>;
                    let mut libs: metamodelica::List<ArcStr>;
                    let mut includeDirs: metamodelica::List<ArcStr>;
                    let mut libPaths: metamodelica::List<ArcStr>;
                    let mut directory: ArcStr;
                    let mut fileName: ArcStr;
                    let mut makefileParams: SimCodeFunction::MakefileParams;
                    let mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                    let mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
                    let mut modelInfo: metamodelica::Ref<ModelInfo::ModelInfo>;
                    let mut simCodeIndices: SimCodeIndices;
                    let mut clockedPartitions: metamodelica::List<metamodelica::Ref<SimPartition::NSimPartition>>;
                    let mut literals_idx: Pointer::Pointer<i32>;
                    let mut literals: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                    let mut externalFunctionIncludes: metamodelica::List<ArcStr>;
                    let mut generic_loop_calls: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>>;
                    let mut independent: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut nominal: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut min: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut max: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut param: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut no_ret: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut event_clocks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut algorithms: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut jac_blocks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut init: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut init_0: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut init_no_ret: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut start: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut ode: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>;
                    let mut algebraic: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>;
                    let mut linearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut nonlinearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut discreteVars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut extObjInfo: metamodelica::Ref<ExtObjInfo::ExtObjInfo>;
                    let mut jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>;
                    let mut simcode_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>>;
                    let mut equation_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimStrongComponent::Block::Block>>>;
                    let mut daeModeData: Option<metamodelica::Ref<DaeModeData::DaeModeData>>;
                    let mut jacA: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacB: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacC: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacD: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacF: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacH: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacAdjoint: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacLfg: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacMrf: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut jacR0: metamodelica::Ref<SimJacobian::SimJacobian>;
                    let mut inlineEquations: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
                    let mut collect_literals: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>;
                    simCodeIndices = EMPTY_SIM_CODE_INDICES();
                    funcMap = BackendDAE::getFunctionMap(bdae)?;
                    literals_idx = Pointer::create(0);
                    collect_literals = (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = (std::sync::Arc::new({ let __pe_b1 = literals_map.clone(); let __pe_b2 = literals_idx; move |__pe_a0| Expression::replaceLiteral(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::fakeMap(__pe_a0, &*__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>);
                    UnorderedMap::apply(funcMap.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = collect_literals.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = collect_literals.clone(); let __pe_b3 = true; let __pe_b4 = true; move |__pe_a0| Function::mapExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Function::Function>) -> Result<metamodelica::Ref<Function::Function>> + 'static>))?;
                    residual_vars = BackendDAE::getLoopResiduals(bdae)?;
                    (vars, simCodeIndices) = SimVars::create(metamodelica::AsArg::as_arg(&varData), residual_vars, simCodeIndices)?;
                    (extObjInfo, vars, simCodeIndices) = ExtObjInfo::create(var_field!((**varData).external_objects, VarData::VarData::VAR_DATA_SIM), vars, simCodeIndices)?;
                    simcode_map = SimCodeUtil::createSimCodeMap(&vars, &extObjInfo)?;
                    equation_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                    independent = metamodelica::nil();
                    nominal = metamodelica::nil();
                    min = metamodelica::nil();
                    max = metamodelica::nil();
                    param = metamodelica::nil();
                    algorithms = metamodelica::nil();
                    (init, simCodeIndices) = SimStrongComponent::Block::createInitialBlocks(metamodelica::AsArg::as_arg(&__bdae_init), simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                    if (__bdae_init_0).is_some() {
                        (init_0, simCodeIndices) = SimStrongComponent::Block::createInitialBlocks(&(__bdae_init_0.clone().ok_or("pattern mismatch")?), simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                    } else {
                        init_0 = metamodelica::nil();
                    }
                    (no_ret, simCodeIndices) = SimStrongComponent::Block::createNoReturnBlocks(var_field!((**eqData).removed, EqData::EqData::EQ_DATA_SIM), simCodeIndices, Partition::Kind::ODE.clone(), simcode_map.clone(), equation_map.clone())?;
                    init_no_ret = metamodelica::nil();
                    start = metamodelica::nil();
                    discreteVars = metamodelica::nil();
                    jacobians = metamodelica::nil();
                    if (__bdae_dae).is_some() {
                        ode = metamodelica::nil();
                        algebraic = metamodelica::nil();
                        (daeModeData, simCodeIndices) = DaeModeData::create(__bdae_dae.clone().ok_or("pattern mismatch")?, simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                    } else {
                        daeModeData = None;
                        (ode, allSim, simCodeIndices) = SimStrongComponent::Block::createBlocks(metamodelica::AsArg::as_arg(&__bdae_ode), allSim, simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                        (algebraic, allSim, simCodeIndices) = SimStrongComponent::Block::createBlocks(metamodelica::AsArg::as_arg(&__bdae_algebraic), allSim, simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                        (ode, allSim, event_blocks, simCodeIndices) = SimStrongComponent::Block::createDiscreteBlocks(metamodelica::AsArg::as_arg(&__bdae_ode_event), ode, allSim, event_blocks, simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                        (algebraic, allSim, event_blocks, simCodeIndices) = SimStrongComponent::Block::createDiscreteBlocks(metamodelica::AsArg::as_arg(&__bdae_alg_event), algebraic, allSim, event_blocks, simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                    }
                    (clockedPartitions, event_clocks, simCodeIndices) = SimStrongComponent::Block::createClockedBlocks(__bdae_clocked.clone(), simCodeIndices, simcode_map.clone(), equation_map.clone(), metamodelica::AsArg::as_arg(&__bdae_clockedInfo))?;
                    if !((no_ret).is_empty()) {
                        algebraic = metamodelica::cons(no_ret.clone(), algebraic.reverse()).reverse();
                    }
                    no_ret = listAppend(event_clocks, no_ret);
                    if !((no_ret).is_empty()) {
                        allSim = listAppend(no_ret.clone(), allSim.reverse()).reverse();
                    }
                    allSim = listAppend(List::flatten(({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>> = metamodelica::nil();
                for mut blck in (allSim.clone()).into_iter().cloned() {
                    let __x = SimStrongComponent::Block::collectEntwinedEquations(&(blck.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?, allSim);
                    inlineEquations = metamodelica::nil();
                    directory = ProgramUtil::getFileDir(AbsynUtil::pathToCref(&name), program.clone());
                    oldFunctionTree = ConvertDAE::convertFunctionTree(&(NFFlatten::FunctionTreeImpl::fromList(&(UnorderedMap::toList(funcMap)), &*((std::sync::Arc::new(fnptr!(NFFlatten::FunctionTreeImpl::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?))?;
                    (libs, libPaths, externalFunctionIncludes, includeDirs, recordDecls, functions, _) = SimCodeUtilShared::createFunctions(&program, &oldFunctionTree)?;
                    makefileParams = OldSimCodeFunctionUtil::createMakefileParams(includeDirs, libs, libPaths, false, false)?;
                    fileName = System::basename(AbsynUtil::classFilename(&(ProgramUtil::getPathedClassInProgram(name.clone(), &program, false, false)?))?);
                    (min, max, nominal, simCodeIndices) = SimStrongComponent::Block::createAttributeBlocks(&(list![var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((**varData).algebraics, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone()]), simCodeIndices, simcode_map.clone())?;
                    (param, simCodeIndices) = SimStrongComponent::Block::createParameterBlocks(metamodelica::AsArg::as_arg(&__bdae_parameters), simCodeIndices, simcode_map.clone(), equation_map.clone())?;
                    (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = collectAlgebraicLoops(init.clone(), init_0.clone(), &ode, &algebraic, daeModeData.clone(), simCodeIndices, simcode_map.clone())?;
                    if (daeModeData).is_some() {
                        (jacA, jacAdjoint, simCodeIndices) = SimJacobian::createSimulationJacobian(&(__bdae_dae.clone().ok_or("pattern mismatch")?), simCodeIndices, simcode_map.clone())?;
                        daeModeData = DaeModeData::addJacobian(daeModeData, jacA.clone());
                    } else {
                        (jacA, jacAdjoint, simCodeIndices) = SimJacobian::createSimulationJacobian(&(listAppend(__bdae_ode.clone(), __bdae_ode_event.clone())), simCodeIndices, simcode_map.clone())?;
                    }
                    (jacB, simCodeIndices) = SimJacobian::empty(literal!("B"), simCodeIndices)?;
                    (jacC, simCodeIndices) = SimJacobian::empty(literal!("C"), simCodeIndices)?;
                    (jacD, simCodeIndices) = SimJacobian::empty(literal!("D"), simCodeIndices)?;
                    (jacF, simCodeIndices) = SimJacobian::empty(literal!("F"), simCodeIndices)?;
                    (jacH, simCodeIndices) = SimJacobian::empty(literal!("H"), simCodeIndices)?;
                    (jacLfg, jacMrf, jacR0, simCodeIndices) = SimJacobian::createOptimizationJacobian(&(listAppend(__bdae_ode.clone(), __bdae_ode_event.clone())), simCodeIndices, simcode_map.clone())?;
                    jacobians = metamodelica::cons(jacR0.clone(), metamodelica::cons(jacMrf.clone(), metamodelica::cons(jacLfg.clone(), metamodelica::cons(jacAdjoint.clone(), metamodelica::cons(jacH.clone(), metamodelica::cons(jacF.clone(), metamodelica::cons(jacD.clone(), metamodelica::cons(jacC.clone(), metamodelica::cons(jacB.clone(), metamodelica::cons(jacA.clone(), jacobians)))))))))).reverse();
                    for mut jac in &*jacobians {
                        if (jac.jac_map).is_some() {
                            vars = SimVars::addSeedAndJacobianVars(vars, &(UnorderedMap::toList(jac.jac_map.clone().ok_or("pattern mismatch")?)))?;
                        }
                    }
                    jac_blocks = SimJacobian::getJacobiansBlocks(&(list![jacA, jacB, jacC, jacD, jacF, jacH, jacAdjoint, jacLfg, jacMrf, jacR0]))?;
                    generic_loop_calls = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>> = metamodelica::nil();
                for mut tpl in (UnorderedMap::toList(simCodeIndices.generic_call_map.clone())).into_iter().cloned() {
                    let __x = SimGenericCall::fromIdentifier(&(tpl.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    generic_loop_calls = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>> = metamodelica::nil();
                for mut call in (generic_loop_calls).into_iter().cloned() {
                    let __x = SimGenericCall::mapShallow(call.clone(), &*(collect_literals.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    literals = UnorderedMap::keyList(literals_map);
                    (modelInfo, simCodeIndices) = ModelInfo::create(vars, name, fileName, directory, functions, linearLoops, nonlinearLoops, metamodelica::AsArg::as_arg(&__bdae_eventInfo), metamodelica::AsArg::as_arg(&__bdae_clockedInfo), simCodeIndices)?;
                    simCode = metamodelica::Ref::new(SimCode { modelInfo: modelInfo, literals: literals, recordDecls: recordDecls, externalFunctionIncludes: externalFunctionIncludes, generic_loop_calls: generic_loop_calls, independent: independent, allSim: allSim, ode: ode, algebraic: algebraic, clockedPartitions: clockedPartitions, nominal: nominal, min: min, max: max, param: param, no_ret: no_ret, algorithms: algorithms, event_blocks: event_blocks, jac_blocks: jac_blocks, start: start, init: init, init_0: init_0, init_no_ret: init_no_ret, discreteVars: discreteVars, extObjInfo: extObjInfo, makefileParams: makefileParams, jacobians: jacobians, simulationSettingsOpt: simSettingsOpt, fileNamePrefix: fileNamePrefix, simcode_map: simcode_map, equation_map: equation_map, eventInfo: __bdae_eventInfo.clone(), daeModeData: daeModeData, inlineEquations: inlineEquations });
                    simCode
                },
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.SimCode.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        Ok((simCode, oldFunctionTree))
    }

    pub fn convert(mut simCode: &metamodelica::Ref<SimCode>) -> Result<metamodelica::Ref<OldSimCode::SimCode>> {
        let mut oldSimCode: metamodelica::Ref<OldSimCode::SimCode>;
        let mut modelInfo: OldSimCode::ModelInfo;
        let mut discreteModelVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        let mut zeroCrossings: metamodelica::List<OldBackendDAE::ZeroCrossing>;
        let mut relations: metamodelica::List<OldBackendDAE::ZeroCrossing>;
        let mut timeEvents: metamodelica::List<OldBackendDAE::TimeEvent>;
        let mut spatialInfo: OldSimCode::SpatialDistributionInfo;
        let mut varToArrayIndexMapping: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        (metamodelica::List<i32>, metamodelica::Array<i32>),
                    )>,
                >,
            ),
            i32,
            (
                HashTableCrIListArray::FuncHashCref,
                HashTableCrIListArray::FuncCrefEqual,
                HashTableCrIListArray::FuncCrefStr,
                HashTableCrIListArray::FuncExpStr,
            ),
        );
        let mut varToIndexMapping: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
            ),
            i32,
            (
                HashTableCrILst::FuncHashCref,
                HashTableCrILst::FuncCrefEqual,
                HashTableCrILst::FuncCrefStr,
                HashTableCrILst::FuncExpStr,
            ),
        );
        let mut crefToSimVarHT: (
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
        let mut crefToClockIndexHT: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            ),
            i32,
            (
                HashTable::FuncHashCref,
                HashTable::FuncCrefEqual,
                HashTable::FuncCrefStr,
                HashTable::FuncExpStr,
            ),
        );
        let mut residualVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
        let mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (metamodelica::Ref<SimVar::SimVar>, metamodelica::Ref<SimCodeVar::SimVar>),
            >,
        >;
        memo = SimVar::newConvertMemo(UnorderedMap::size(simCode.simcode_map.clone()));
        modelInfo = ModelInfo::convert(&simCode.modelInfo, memo.clone())?;
        (zeroCrossings, relations, timeEvents, spatialInfo) =
            EventInfo::convert(&simCode.eventInfo, simCode.equation_map.clone())?;
        (varToArrayIndexMapping, varToIndexMapping) = SimCodeUtilShared::createVarToArrayIndexMapping(&modelInfo)?;
        crefToSimVarHT = SimCodeUtil::convertSimCodeMap(simCode.simcode_map.clone(), memo)?;
        if (simCode.daeModeData).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(simCode.daeModeData.clone()) {
                Some(Deref @ DaeModeData::DAE_MODE_DATA { residualVars: __pa0, .. }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            residualVars = metamodelica::Own::own(__pa0);
            crefToSimVarHT = List::fold(
                &(SimVar::convertList(residualVars)?),
                &HashTableCrefSimVar::addSimVarToHashTable,
                crefToSimVarHT,
            )?;
        }
        crefToClockIndexHT = HashTable::emptyHashTable();
        for mut cref in &*simCode.discreteVars.clone() {
            discreteModelVars = metamodelica::cons(
                ComponentRef::toDAE(metamodelica::AsArg::as_arg(&cref))?,
                discreteModelVars,
            );
        }
        oldSimCode = metamodelica::Ref::new(OldSimCode::SimCode {
            modelInfo: modelInfo,
            literals: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut lit in (simCode.literals.clone()).into_iter().cloned() {
                    let __x = Expression::toDAE(lit.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            recordDecls: simCode.recordDecls.clone(),
            externalFunctionIncludes: simCode.externalFunctionIncludes.clone(),
            generic_loop_calls: ({
                let mut __acc: metamodelica::List<OldSimCode::SimGenericCall> = metamodelica::nil();
                for mut gc in (simCode.generic_loop_calls.clone()).into_iter().cloned() {
                    let __x = SimGenericCall::convert(&(gc.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            localKnownVars: SimStrongComponent::Block::convertList(simCode.independent.clone())?,
            allEquations: SimStrongComponent::Block::convertList(simCode.allSim.clone())?,
            odeEquations: SimStrongComponent::Block::convertListList(simCode.ode.clone())?,
            algebraicEquations: SimStrongComponent::Block::convertListList(simCode.algebraic.clone())?,
            clockedPartitions: ({
                let mut __acc: metamodelica::List<OldSimCode::ClockedPartition> = metamodelica::nil();
                for mut part in (simCode.clockedPartitions.clone()).into_iter().cloned() {
                    let __x = SimPartition::convertBase(&(part.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            initialEquations: SimStrongComponent::Block::convertList(simCode.init.clone())?,
            initialEquations_lambda0: SimStrongComponent::Block::convertList(simCode.init_0.clone())?,
            removedInitialEquations: SimStrongComponent::Block::convertList(simCode.init_no_ret.clone())?,
            startValueEquations: SimStrongComponent::Block::convertList(simCode.start.clone())?,
            nominalValueEquations: SimStrongComponent::Block::convertList(simCode.nominal.clone())?,
            minValueEquations: SimStrongComponent::Block::convertList(simCode.min.clone())?,
            maxValueEquations: SimStrongComponent::Block::convertList(simCode.max.clone())?,
            parameterEquations: SimStrongComponent::Block::convertList(simCode.param.clone())?,
            removedEquations: SimStrongComponent::Block::convertList(simCode.no_ret.clone())?,
            algorithmAndEquationAsserts: SimStrongComponent::Block::convertList(simCode.algorithms.clone())?,
            equationsForZeroCrossings: SimStrongComponent::Block::convertList(simCode.event_blocks.clone())?,
            jacobianEquations: SimStrongComponent::Block::convertList(simCode.jac_blocks.clone())?,
            stateSets: metamodelica::nil(),
            constraints: metamodelica::nil(),
            classAttributes: metamodelica::nil(),
            zeroCrossings: zeroCrossings,
            relations: relations,
            timeEvents: timeEvents,
            discreteModelVars: discreteModelVars,
            extObjInfo: ExtObjInfo::convert(&simCode.extObjInfo)?,
            makefileParams: simCode.makefileParams.clone(),
            delayedExps: OldSimCode::DelayedExpression {
                delayedExps: metamodelica::nil(),
                maxDelayedIndex: 0,
            },
            spatialInfo: spatialInfo,
            jacobianMatrices: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::JacobianMatrix>> = metamodelica::nil();
                for mut jac in (simCode.jacobians.clone()).into_iter().cloned() {
                    let __x = SimJacobian::convert(&(jac.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            simulationSettingsOpt: simCode.simulationSettingsOpt.clone(),
            fileNamePrefix: simCode.fileNamePrefix.clone(),
            fullPathPrefix: literal!(""),
            fmuTargetName: literal!(""),
            hpcomData: HpcOmSimCode::emptyHpcomData().clone(),
            valueReferences: openmodelica_simcode_types::AvlTreeCRToInt::Tree::interned_EMPTY(),
            varToArrayIndexMapping: varToArrayIndexMapping,
            varToIndexMapping: varToIndexMapping,
            crefToSimVarHT: crefToSimVarHT,
            crefToClockIndexHT: crefToClockIndexHT,
            backendMapping: None,
            modelStructure: None,
            fmiSimulationFlags: None,
            partitionData: OldSimCode::PartitionData {
                numPartitions: -1,
                partitions: metamodelica::nil(),
                activatorsForPartitions: metamodelica::nil(),
                stateToActivators: metamodelica::nil(),
            },
            daeModeData: if ((simCode.daeModeData).is_some()) {
                Some(DaeModeData::convert(
                    &(simCode.daeModeData.clone().ok_or("pattern mismatch")?),
                )?)
            } else {
                None
            },
            inlineEquations: metamodelica::nil(),
            omsiData: None,
            scalarized: Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())?,
            fmiFigures: metamodelica::nil(),
        });
        Ok(oldSimCode)
    }

    pub fn getDirectoryAndLibs(
        mut simCode: &metamodelica::Ref<SimCode>,
    ) -> Result<(ArcStr, metamodelica::List<ArcStr>)> {
        let mut directory: ArcStr;
        let mut libs: metamodelica::List<ArcStr>;
        (directory, libs) = (::match_deref::match_deref! { match simCode {
            Deref @ SimCode { modelInfo: Deref @ ModelInfo::MODEL_INFO { directory: __esc_directory, .. }, makefileParams: SimCodeFunction::MakefileParams { libs: __esc_libs, .. }, .. } => {
                directory = (*__esc_directory).clone();
                libs = (*__esc_libs).clone();
                (directory.clone(), libs.clone())
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.SimCode.getDirectoryAndLibs")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((directory, libs))
    }

    fn collectAlgebraicLoops(
        mut init: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        mut init_0: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        mut ode: &metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>,
        mut algebraic: &metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>,
        mut daeModeData: Option<metamodelica::Ref<DaeModeData::DaeModeData>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>,
        SimCodeIndices,
    )> {
        let mut linearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>> =
            metamodelica::nil();
        let mut nonlinearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>> =
            metamodelica::nil();
        let mut jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut dae_mode_blcks: metamodelica::List<
            metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        >;
        (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(
            &(list![init, init_0]),
            linearLoops,
            nonlinearLoops,
            jacobians,
            simCodeIndices,
            simcode_map.clone(),
        )?;
        (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(
            ode,
            linearLoops,
            nonlinearLoops,
            jacobians,
            simCodeIndices,
            simcode_map.clone(),
        )?;
        (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = SimStrongComponent::Block::collectAlgebraicLoops(
            algebraic,
            linearLoops,
            nonlinearLoops,
            jacobians,
            simCodeIndices,
            simcode_map.clone(),
        )?;
        if (daeModeData).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(daeModeData) {
                Some(Deref @ DaeModeData::DAE_MODE_DATA { blcks: __pa0, .. }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dae_mode_blcks = metamodelica::Own::own(__pa0);
            (linearLoops, nonlinearLoops, jacobians, simCodeIndices) =
                SimStrongComponent::Block::collectAlgebraicLoops(
                    &dae_mode_blcks,
                    linearLoops,
                    nonlinearLoops,
                    jacobians,
                    simCodeIndices,
                    simcode_map,
                )?;
        }
        Ok((linearLoops, nonlinearLoops, jacobians, simCodeIndices))
    }
}

pub mod ModelInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ModelInfo {
        pub name: metamodelica::Ref<Absyn::Path>,
        pub description: ArcStr,
        pub version: ArcStr,
        pub author: ArcStr,
        pub license: ArcStr,
        pub copyright: ArcStr,
        pub directory: ArcStr,
        pub fileName: ArcStr,
        pub vars: metamodelica::Ref<SimVars::SimVars>,
        pub varInfo: metamodelica::Ref<VarInfo::VarInfo>,
        pub functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
        pub labels: metamodelica::List<ArcStr>,
        /// Paths of all resources used by the model. Used in FMI2 to package resources in the FMU.
        pub resourcePaths: metamodelica::List<ArcStr>,
        pub sortedClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>,
        pub nClocks: i32,
        pub nSubClocks: i32,
        pub nSpatialDistributions: i32,
        pub hasLargeLinearEquationSystems: bool,
        pub linearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        pub nonlinearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
    }

    impl metamodelica::gc::MMTrace for ModelInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.description, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.version, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.author, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.license, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.copyright, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.directory, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.varInfo, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.functions, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.labels, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.resourcePaths, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.sortedClasses, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nClocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nSubClocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nSpatialDistributions, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.hasLargeLinearEquationSystems, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.linearLoops, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nonlinearLoops, __mmv)?;
            Ok(())
        }
    }
    impl Default for ModelInfo {
        fn default() -> Self {
            Self {
                name: Default::default(),
                description: Default::default(),
                version: Default::default(),
                author: Default::default(),
                license: Default::default(),
                copyright: Default::default(),
                directory: Default::default(),
                fileName: Default::default(),
                vars: Default::default(),
                varInfo: Default::default(),
                functions: Default::default(),
                labels: Default::default(),
                resourcePaths: Default::default(),
                sortedClasses: Default::default(),
                nClocks: Default::default(),
                nSubClocks: Default::default(),
                nSpatialDistributions: Default::default(),
                hasLargeLinearEquationSystems: Default::default(),
                linearLoops: Default::default(),
                nonlinearLoops: Default::default(),
            }
        }
    }

    pub type MODEL_INFO = ModelInfo;

    pub(crate) fn toString(mut modelInfo: &metamodelica::Ref<ModelInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = SimVars::toString(&modelInfo.vars, literal!(""))?;
        Ok(r#str)
    }

    pub(crate) fn create(
        mut vars: metamodelica::Ref<SimVars::SimVars>,
        mut name: metamodelica::Ref<Absyn::Path>,
        mut fileName: ArcStr,
        mut directory: ArcStr,
        mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
        mut linearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        mut nonlinearLoops: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        mut eventInfo: &metamodelica::Ref<EventInfo::EventInfo>,
        mut clockedInfo: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(metamodelica::Ref<ModelInfo>, SimCodeIndices)> {
        let mut modelInfo: metamodelica::Ref<ModelInfo>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut info: metamodelica::Ref<VarInfo::VarInfo>;
        info = VarInfo::create(&vars, eventInfo, &simCodeIndices)?;
        modelInfo = metamodelica::Ref::new(ModelInfo {
            name: name,
            description: literal!(""),
            version: literal!(""),
            author: literal!(""),
            license: literal!(""),
            copyright: literal!(""),
            directory: directory,
            fileName: fileName,
            vars: vars,
            varInfo: info,
            functions: functions,
            labels: metamodelica::nil(),
            resourcePaths: metamodelica::nil(),
            sortedClasses: metamodelica::nil(),
            nClocks: ClockedInfo::baseClockCount(clockedInfo, false)?,
            nSubClocks: ClockedInfo::subClockCount(clockedInfo),
            nSpatialDistributions: ((eventInfo.spatial_lst).len() as i32),
            hasLargeLinearEquationSystems: true,
            linearLoops: linearLoops,
            nonlinearLoops: nonlinearLoops,
        });
        Ok((modelInfo, simCodeIndices))
    }

    pub(crate) fn setSeedVars(
        mut modelInfo: metamodelica::Ref<ModelInfo>,
        mut seedVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
    ) -> Result<metamodelica::Ref<ModelInfo>> {
        let mut modelInfo: metamodelica::Ref<ModelInfo> = modelInfo;
        modelInfo = (match &*modelInfo {
            ModelInfo { vars, .. } => {
                let mut vars = (*vars).clone();
                assign_field!(vars.seedVars = seedVars);
                assign_field!(modelInfo.vars = vars.clone());
                modelInfo
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimCode.ModelInfo.setSeedVars"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(modelInfo)
    }

    pub(crate) fn convert(
        mut modelInfo: &metamodelica::Ref<ModelInfo>,
        mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (metamodelica::Ref<SimVar::SimVar>, metamodelica::Ref<SimCodeVar::SimVar>),
            >,
        >,
    ) -> Result<OldSimCode::ModelInfo> {
        let mut oldModelInfo: OldSimCode::ModelInfo;
        let mut varInfo: OldSimCode::VarInfo;
        varInfo = VarInfo::convert(&modelInfo.varInfo);
        oldModelInfo = OldSimCode::ModelInfo {
            name: modelInfo.name.clone(),
            description: modelInfo.description.clone(),
            version: modelInfo.version.clone(),
            author: modelInfo.author.clone(),
            license: modelInfo.license.clone(),
            copyright: modelInfo.copyright.clone(),
            directory: modelInfo.directory.clone(),
            fileName: modelInfo.fileName.clone(),
            varInfo: VarInfo::convert(&modelInfo.varInfo),
            vars: SimVars::convert(&modelInfo.vars, memo)?,
            functions: modelInfo.functions.clone(),
            labels: modelInfo.labels.clone(),
            resourcePaths: modelInfo.resourcePaths.clone(),
            sortedClasses: modelInfo.sortedClasses.clone(),
            nClocks: modelInfo.nClocks.clone(),
            nSubClocks: modelInfo.nSubClocks.clone(),
            nSpatialDistributions: modelInfo.nSpatialDistributions.clone(),
            hasLargeLinearEquationSystems: modelInfo.hasLargeLinearEquationSystems.clone(),
            linearSystems: SimStrongComponent::Block::convertList(modelInfo.linearLoops.clone())?,
            nonLinearSystems: SimStrongComponent::Block::convertList(modelInfo.nonlinearLoops.clone())?,
            unitDefinitions: metamodelica::nil(),
        };
        Ok(oldModelInfo)
    }
}

pub mod DaeModeData {
    use super::*;
    /// contains data that belongs to the dae mode
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct DaeModeData {
        /// daeMode blocks
        pub blcks: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>,
        /// contains the sparsity pattern for the daeMode
        pub sparsityPattern: Option<metamodelica::Ref<SimJacobian::SimJacobian>>,
        /// variable used to calculate residuals of a DAE form, they are of type real
        pub residualVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub algebraicVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub auxiliaryVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub modeCreated: DaeModeConfig,
    }

    impl metamodelica::gc::MMTrace for DaeModeData {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.blcks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.sparsityPattern, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.residualVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.algebraicVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.auxiliaryVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.modeCreated, __mmv)?;
            Ok(())
        }
    }
    impl Default for DaeModeData {
        fn default() -> Self {
            Self {
                blcks: Default::default(),
                sparsityPattern: Default::default(),
                residualVars: Default::default(),
                algebraicVars: Default::default(),
                auxiliaryVars: Default::default(),
                modeCreated: Default::default(),
            }
        }
    }

    pub type DAE_MODE_DATA = DaeModeData;

    pub(crate) fn toString(mut data: &metamodelica::Ref<DaeModeData>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut idx: i32 = 1;
        for mut blck_lst in &*data.blcks.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*SimStrongComponent::Block::listToString(
                    metamodelica::AsArg::as_arg(&blck_lst),
                    literal!("  "),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("DAE Partition "));
                        __mm_s.push_str(&*intString(idx));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                ArcStr::from(__mm_s)
            };
            idx = idx + 1;
        }
        if (data.sparsityPattern).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*SimJacobian::toString(
                    &(data.sparsityPattern.clone().ok_or("pattern mismatch")?),
                )?);
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn create(
        mut systems: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimStrongComponent::Block::Block>,
            >,
        >,
    ) -> Result<(Option<metamodelica::Ref<DaeModeData>>, SimCodeIndices)> {
        let mut data: Option<metamodelica::Ref<DaeModeData>>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut blcks: metamodelica::List<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>>;
        let mut residualVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
        (blcks, residualVars, simCodeIndices) =
            SimStrongComponent::Block::createDAEModeBlocks(systems, simCodeIndices, simcode_map, equation_map)?;
        data = Some(metamodelica::Ref::new(DaeModeData {
            blcks: blcks,
            sparsityPattern: None,
            residualVars: residualVars,
            algebraicVars: metamodelica::nil(),
            auxiliaryVars: metamodelica::nil(),
            modeCreated: DaeModeConfig::ALL.clone(),
        }));
        Ok((data, simCodeIndices))
    }

    pub(crate) fn addJacobian(
        mut data: Option<metamodelica::Ref<DaeModeData>>,
        mut daeModeJac: metamodelica::Ref<SimJacobian::SimJacobian>,
    ) -> Option<metamodelica::Ref<DaeModeData>> {
        let mut data: Option<metamodelica::Ref<DaeModeData>> = data;
        data = (::match_deref::match_deref! { match &(data) {
            Some(dmd) => {
                let mut dmd = (*dmd).clone();
                assign_field!(dmd.sparsityPattern = Some(daeModeJac));
                Some(dmd.clone())
            },
            _ => {
                None
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        data
    }

    pub(crate) fn convert(mut data: &metamodelica::Ref<DaeModeData>) -> Result<OldSimCode::DaeModeData> {
        let mut oldData: OldSimCode::DaeModeData;
        let mut simEqSystems: metamodelica::List<metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>>> =
            metamodelica::nil();
        simEqSystems = SimStrongComponent::Block::convertListList(data.blcks.clone())?;
        oldData = OldSimCode::DaeModeData {
            daeEquations: simEqSystems,
            sparsityPattern: Util::applyOption(data.sparsityPattern.clone(), &move |__a0: metamodelica::Ref<
                SimJacobian::SimJacobian,
            >| {
                SimJacobian::convert(&__a0)
            })?,
            residualVars: SimVar::convertList(data.residualVars.clone())?,
            algebraicVars: SimVar::convertList(data.algebraicVars.clone())?,
            auxiliaryVars: SimVar::convertList(data.auxiliaryVars.clone())?,
            modeCreated: convertMode(data.modeCreated.clone())?,
        };
        Ok(oldData)
    }

    fn convertMode(mut mode: DaeModeConfig) -> Result<OldSimCode::DaeModeConfig> {
        let mut oldMode: OldSimCode::DaeModeConfig;
        oldMode = (match mode {
            DaeModeConfig::ALL => openmodelica_simcode_types::SimCode::DaeModeConfig::ALL_EQUATIONS,
            DaeModeConfig::DYNAMIC => openmodelica_simcode_types::SimCode::DaeModeConfig::DYNAMIC_EQUATIONS,
        });
        Ok(oldMode)
    }

    fn createSparsityJacobian(
        mut daeModeDataOpt: Option<metamodelica::Ref<DaeModeData>>,
        mut modelInfo: metamodelica::Ref<ModelInfo::ModelInfo>,
        mut systems: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(
        Option<metamodelica::Ref<DaeModeData>>,
        metamodelica::Ref<ModelInfo::ModelInfo>,
        metamodelica::Ref<SimJacobian::SimJacobian>,
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        SimCodeIndices,
    )> {
        let mut daeModeDataOpt: Option<metamodelica::Ref<DaeModeData>> = daeModeDataOpt;
        let mut modelInfo: metamodelica::Ref<ModelInfo::ModelInfo> = modelInfo;
        let mut jacobian: metamodelica::Ref<SimJacobian::SimJacobian> =
            <metamodelica::Ref<SimJacobian::SimJacobian> as ::std::default::Default>::default();
        let mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        > = simcode_map;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        daeModeDataOpt = (::match_deref::match_deref! { match &(daeModeDataOpt) {
            Some(daeModeData) => {
                (jacobian, simCodeIndices) = SimJacobian::empty(literal!("A"), simCodeIndices)?;
                Some(daeModeData.clone())
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimCode.DaeModeData.createSparsityJacobian")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((daeModeDataOpt, modelInfo, jacobian, simcode_map, simCodeIndices))
    }

    fn rewriteAlgebraicVarsIdx(
        mut simulationAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> {
        let mut daeModeAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut seedCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        seedCref = ComponentRef::fromNode(
            metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
                name: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*arcstr::literal!(BVariable::SEED_STR));
                    __mm_s.push_str(&*literal!("_A"));
                    ArcStr::from(__mm_s)
                },
                varPointer: PointerWeak::downgrade(Pointer::createImmutable(BVariable::DUMMY_VARIABLE().clone())),
            }),
            openmodelica_nf_frontend::NFType::interned_UNKNOWN(),
            metamodelica::nil(),
            ComponentRef::Origin::CREF.clone(),
        )?;
        for mut var in &*simulationAlgVars.reverse() {
            let mut var = var.clone();
            cref = ComponentRef::append(var.name.clone(), &seedCref)?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Searching for: "));
                __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            assign_field!(var.index = SimVar::getIndex(cref, simcode_map.clone())?);
            daeModeAlgVars = metamodelica::cons(var, daeModeAlgVars);
        }
        Ok(daeModeAlgVars)
    }

    fn replaceDerCrefSES(
        mut sys: metamodelica::Ref<OldSimCode::SimEqSystem>,
    ) -> Result<metamodelica::Ref<OldSimCode::SimEqSystem>> {
        let mut sys: metamodelica::Ref<OldSimCode::SimEqSystem> = sys;
        sys = (::match_deref::match_deref! { match &(sys) {
            qual @ Deref @ OldSimCode::SimEqSystem::SES_RESIDUAL { .. } => {
                let mut qual = (*qual).clone();
                let (__asg0_0, _) = OldExpression::traverseExpTopDown(var_field!((*qual).exp, OldSimCode::SimEqSystem::SES_RESIDUAL).clone(), &replaceDerCref, 0)?;
                assign_variant_field!(qual => OldSimCode::SimEqSystem::SES_RESIDUAL; exp = __asg0_0.clone());
                qual.clone()
            },
            qual @ Deref @ OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN { .. } => {
                let mut qual = (*qual).clone();
                let (__asg0_0, _) = OldExpression::traverseExpTopDown(var_field!((*qual).exp, OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN).clone(), &replaceDerCref, 0)?;
                assign_variant_field!(qual => OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN; exp = __asg0_0.clone());
                qual.clone()
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(sys)
    }

    fn replaceDerCref(
        mut exp: metamodelica::Ref<DAE::Exp>,
        mut i: i32,
    ) -> Result<(metamodelica::Ref<DAE::Exp>, bool, i32)> {
        let mut exp: metamodelica::Ref<DAE::Exp> = exp;
        let mut b: bool;
        let mut i: i32 = i;
        (exp, b) = (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: cref, .. }, .. } => {
                (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: list![metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cref))? })], attr: DAE::callAttrBuiltinReal().clone() }), false)
            },
            _ => {
                (exp, true)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((exp, b, i))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum DaeModeConfig {
    ALL = 1,
    DYNAMIC = 2,
}
impl PartialOrd for DaeModeConfig {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DaeModeConfig {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for DaeModeConfig {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for DaeModeConfig {
    fn default() -> Self {
        Self::ALL
    }
}
