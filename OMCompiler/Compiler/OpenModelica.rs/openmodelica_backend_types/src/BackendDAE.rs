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

use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::SCode;
use openmodelica_util::ExpandableArray;
use openmodelica_util::MMath;
use openmodelica_util_datatypes_basic::DoubleEnded;

/// Once we are in BackendDAE, the Type can be only basic types or enumeration.
/// We cannot do this in DAE because functions may contain many more types.
/// adrpo: yes we can, we just simplify the DAE.Type, see Types.simplifyType
pub type Type = metamodelica::Ref<openmodelica_frontend_types::DAE::Type>;

/// THE LOWERED DAE consist of variables and equations. The variables are split into
///  two lists, one for unknown variables states and algebraic and one for known variables
///  constants and parameters.
///  The equations are also split into two lists, one with simple equations, a=b, a-b=0, etc., that
///  are removed from  the set of equations to speed up calculations.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct BackendDAE {
    pub eqs: EqSystems,
    pub shared: metamodelica::Ref<Shared>,
}

impl metamodelica::gc::MMTrace for BackendDAE {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.eqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.shared, __mmv)?;
        Ok(())
    }
}
impl Default for BackendDAE {
    fn default() -> Self {
        Self {
            eqs: Default::default(),
            shared: Default::default(),
        }
    }
}

pub type DAE = BackendDAE;

pub type EqSystems = metamodelica::List<metamodelica::Ref<EqSystem>>;

/// An independent system of equations (and their corresponding variables)
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EqSystem {
    /// ordered Variables, only states and alg. vars
    pub orderedVars: Variables,
    /// ordered Equations
    pub orderedEqs: EquationArray,
    pub m: Option<metamodelica::Array<metamodelica::List<i32>>>,
    pub mT: Option<metamodelica::Array<metamodelica::List<i32>>>,
    /// current type of adjacency matrix, boolean is true if scalar
    pub mapping: Option<(
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        IndexType,
        bool,
        bool,
    )>,
    pub matching: metamodelica::Ref<Matching>,
    /// the state sets of the system
    pub stateSets: StateSets,
    pub partitionKind: BaseClockPartitionKind,
    /// these are equations that cannot solve for a variable.
    ///                                             e.g. assertions, external function calls, algorithm sections without effect
    pub removedEqs: EquationArray,
}

impl metamodelica::gc::MMTrace for EqSystem {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.orderedVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.orderedEqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.m, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matching, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stateSets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partitionKind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.removedEqs, __mmv)?;
        Ok(())
    }
}
impl Default for EqSystem {
    fn default() -> Self {
        Self {
            orderedVars: Default::default(),
            orderedEqs: Default::default(),
            m: Default::default(),
            mT: Default::default(),
            mapping: Default::default(),
            matching: Default::default(),
            stateSets: Default::default(),
            partitionKind: Default::default(),
            removedEqs: Default::default(),
        }
    }
}

pub type EQSYSTEM = EqSystem;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SubClock {
    SUBCLOCK {
        factor: MMath::Rational,
        shift: MMath::Rational,
        solver: Option<ArcStr>,
    },
    INFERED_SUBCLOCK,
}
impl metamodelica::gc::MMTrace for SubClock {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SubClock::SUBCLOCK { factor, shift, solver } => {
                metamodelica::gc::MMTrace::mm_accept(factor, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(shift, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(solver, __mmv)?;
                Ok(())
            }
            SubClock::INFERED_SUBCLOCK => Ok(()),
        }
    }
}
impl Default for SubClock {
    fn default() -> Self {
        Self::INFERED_SUBCLOCK
    }
}
pub use self::SubClock::{INFERED_SUBCLOCK, SUBCLOCK};

pub static DEFAULT_SUBCLOCK: std::sync::LazyLock<SubClock> = std::sync::LazyLock::new(|| SubClock::SUBCLOCK {
    factor: MMath::RAT1.clone(),
    shift: MMath::RAT0.clone(),
    solver: None,
});

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BaseClockPartitionKind {
    UNKNOWN_PARTITION,
    CLOCKED_PARTITION {
        subPartIdx: i32,
    },
    CONTINUOUS_TIME_PARTITION,
    /// treated as CONTINUOUS_TIME_PARTITION
    UNSPECIFIED_PARTITION,
}
impl metamodelica::gc::MMTrace for BaseClockPartitionKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BaseClockPartitionKind::UNKNOWN_PARTITION => Ok(()),
            BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx } => {
                metamodelica::gc::MMTrace::mm_accept(subPartIdx, __mmv)?;
                Ok(())
            }
            BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION => Ok(()),
            BaseClockPartitionKind::UNSPECIFIED_PARTITION => Ok(()),
        }
    }
}
impl Default for BaseClockPartitionKind {
    fn default() -> Self {
        Self::UNKNOWN_PARTITION
    }
}
pub use self::BaseClockPartitionKind::{
    CLOCKED_PARTITION, CONTINUOUS_TIME_PARTITION, UNKNOWN_PARTITION, UNSPECIFIED_PARTITION,
};

/// Data shared for all equation-systems
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Shared {
    /// variables only depending on parameters and constants [TODO: move stuff (like inputs) to localKnownVars]
    pub globalKnownVars: Variables,
    /// variables only depending on locally constant variables in the simulation step, i.e. states, input variables
    pub localKnownVars: Variables,
    /// External object variables
    pub externalObjects: Variables,
    /// Data originating from removed simple equations needed to build
    ///                                             variables' lookup table (in C output).
    ///                                             In that way, double buffering of variables in pre()-buffer, extrapolation
    ///                                             buffer and results caching, etc., is avoided, but in C-code output all the
    ///                                             data about variables' names, comments, units, etc. is preserved as well as
    ///                                             pointer to their values (trajectories).
    pub aliasVars: Variables,
    /// Initial equations
    pub initialEqs: EquationArray,
    /// these are equations that cannot solve for a variable. for example assertions, external function calls, algorithm sections without effect
    pub removedEqs: EquationArray,
    /// constraints (Optimica extension)
    pub constraints: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Constraint>>,
    /// class attributes (Optimica extension)
    pub classAttrs: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ClassAttributes>>,
    pub cache: FCore::Cache,
    pub graph: FCore::Graph,
    /// functions for Backend
    pub functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    /// eventInfo
    pub eventInfo: EventInfo,
    /// classes of external objects, contains constructor & destructor
    pub extObjClasses: ExternalObjectClasses,
    /// indicate for what the BackendDAE is used
    pub backendDAEType: BackendDAEType,
    /// Symbolic Jacobians
    pub symjacs: SymbolicJacobians,
    /// contains extra info that we send around like the model name
    pub info: ExtraInfo,
    pub partitionsInfo: PartitionsInfo,
    /// DAEMode Data
    pub daeModeData: BackendDAEModeData,
    pub dataReconciliationData: Option<DataReconciliationData>,
    /// from experiment annotation Interval, used for derivative nominal guesswork
    pub timeInterval: Option<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
}

impl metamodelica::gc::MMTrace for Shared {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.globalKnownVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localKnownVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.externalObjects, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.aliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialEqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.removedEqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.constraints, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.classAttrs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cache, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.graph, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.functionTree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eventInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extObjClasses, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.backendDAEType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.symjacs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partitionsInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.daeModeData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dataReconciliationData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.timeInterval, __mmv)?;
        Ok(())
    }
}
impl Default for Shared {
    fn default() -> Self {
        Self {
            globalKnownVars: Default::default(),
            localKnownVars: Default::default(),
            externalObjects: Default::default(),
            aliasVars: Default::default(),
            initialEqs: Default::default(),
            removedEqs: Default::default(),
            constraints: Default::default(),
            classAttrs: Default::default(),
            cache: Default::default(),
            graph: Default::default(),
            functionTree: Default::default(),
            eventInfo: Default::default(),
            extObjClasses: Default::default(),
            backendDAEType: Default::default(),
            symjacs: Default::default(),
            info: Default::default(),
            partitionsInfo: Default::default(),
            daeModeData: Default::default(),
            dataReconciliationData: Default::default(),
            timeInterval: Default::default(),
        }
    }
}

pub type SHARED = Shared;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct InlineData {
    pub inlineSystems: EqSystems,
    pub knownVariables: Variables,
}

impl metamodelica::gc::MMTrace for InlineData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.inlineSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.knownVariables, __mmv)?;
        Ok(())
    }
}
impl Default for InlineData {
    fn default() -> Self {
        Self {
            inlineSystems: Default::default(),
            knownVariables: Default::default(),
        }
    }
}

pub type INLINE_DATA = InlineData;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct BasePartition {
    pub clock: metamodelica::Ref<openmodelica_frontend_types::DAE::ClockKind>,
    pub nSubClocks: i32,
}

impl metamodelica::gc::MMTrace for BasePartition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.clock, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nSubClocks, __mmv)?;
        Ok(())
    }
}
impl Default for BasePartition {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            nSubClocks: Default::default(),
        }
    }
}

pub type BASE_PARTITION = BasePartition;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SubPartition {
    pub clock: SubClock,
    pub holdEvents: bool,
    pub prevVars: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
}

impl metamodelica::gc::MMTrace for SubPartition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.clock, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.holdEvents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.prevVars, __mmv)?;
        Ok(())
    }
}
impl Default for SubPartition {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            holdEvents: Default::default(),
            prevVars: Default::default(),
        }
    }
}

pub type SUB_PARTITION = SubPartition;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct PartitionsInfo {
    pub basePartitions: metamodelica::Array<BasePartition>,
    pub subPartitions: metamodelica::Array<SubPartition>,
}

impl metamodelica::gc::MMTrace for PartitionsInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.basePartitions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.subPartitions, __mmv)?;
        Ok(())
    }
}
impl Default for PartitionsInfo {
    fn default() -> Self {
        Self {
            basePartitions: Default::default(),
            subPartitions: Default::default(),
        }
    }
}

pub type PARTITIONS_INFO = PartitionsInfo;

/// extra information that we should send around with the DAE
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ExtraInfo {
    /// the model description string
    pub description: ArcStr,
    /// the model name to be used in the dumps
    pub fileNamePrefix: ArcStr,
    /// the simulation flag string (-sx=...) needed for data reconciliation to read measurement start values from a csv file. Kept as a plain String rather than a SimCode.SimulationSettings reference so this datatype package does not depend on SimCode.
    pub simflags: Option<ArcStr>,
}

impl metamodelica::gc::MMTrace for ExtraInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.description, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fileNamePrefix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.simflags, __mmv)?;
        Ok(())
    }
}
impl Default for ExtraInfo {
    fn default() -> Self {
        Self {
            description: Default::default(),
            fileNamePrefix: Default::default(),
            simflags: Default::default(),
        }
    }
}

pub type EXTRA_INFO = ExtraInfo;

/// BackendDAEType to indicate different types of BackendDAEs.
///  For example for simulation, initialization, Jacobian, algebraic loops etc.
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BackendDAEType {
    /// Type for the normal BackendDAE.DAE for simulation
    SIMULATION,
    /// Type for Jacobian BackendDAE.DAE
    JACOBIAN,
    /// Type for algebraic loop BackendDAE.DAE
    ALGEQSYSTEM,
    /// Type for multi dim equation arrays BackendDAE.DAE
    ARRAYSYSTEM,
    /// Type for parameter system BackendDAE.DAE
    PARAMETERSYSTEM,
    /// Type for initial system BackendDAE.DAE
    INITIALSYSTEM,
    /// Type for inline system BackendDAE.DAE
    INLINESYSTEM,
    /// Type for DAEmode system BackendDAE.DAE
    DAEMODESYSTEM,
}
impl metamodelica::gc::MMTrace for BackendDAEType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BackendDAEType::SIMULATION => Ok(()),
            BackendDAEType::JACOBIAN => Ok(()),
            BackendDAEType::ALGEQSYSTEM => Ok(()),
            BackendDAEType::ARRAYSYSTEM => Ok(()),
            BackendDAEType::PARAMETERSYSTEM => Ok(()),
            BackendDAEType::INITIALSYSTEM => Ok(()),
            BackendDAEType::INLINESYSTEM => Ok(()),
            BackendDAEType::DAEMODESYSTEM => Ok(()),
        }
    }
}
impl Default for BackendDAEType {
    fn default() -> Self {
        Self::SIMULATION
    }
}
pub use self::BackendDAEType::{
    ALGEQSYSTEM, ARRAYSYSTEM, DAEMODESYSTEM, INITIALSYSTEM, INLINESYSTEM, JACOBIAN, PARAMETERSYSTEM, SIMULATION,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DataReconciliationData {
    /// jacobians for set-C and set-S
    pub symbolicJacobian: metamodelica::Ref<Jacobian>,
    /// setc solved vars
    pub setcVars: Variables,
    pub datareconinputs: Variables,
    /// setB solved vars which computes boundary conditions
    pub setBVars: Option<Variables>,
    /// For solving state estimation we need two Jacobians F for data Reconciliation and H for boundary conditions set-B and set-Sprime
    pub symbolicJacobianH: Option<metamodelica::Ref<Jacobian>>,
    /// count number of boundary conditions which failed the extraction algorithm
    pub relatedBoundaryConditions: i32,
}

impl metamodelica::gc::MMTrace for DataReconciliationData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.symbolicJacobian, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.setcVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.datareconinputs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.setBVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.symbolicJacobianH, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relatedBoundaryConditions, __mmv)?;
        Ok(())
    }
}
impl Default for DataReconciliationData {
    fn default() -> Self {
        Self {
            symbolicJacobian: Default::default(),
            setcVars: Default::default(),
            datareconinputs: Default::default(),
            setBVars: Default::default(),
            symbolicJacobianH: Default::default(),
            relatedBoundaryConditions: Default::default(),
        }
    }
}

pub type DATA_RECON = DataReconciliationData;

//
//  variables and equations definition
//
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Variables {
    /// HashTB, cref->indx
    pub crefIndices: metamodelica::Array<metamodelica::List<CrefIndex>>,
    /// HashTB, array or record cref->indices of its elements
    pub prefixIndices: metamodelica::Array<metamodelica::List<PrefixIndex>>,
    /// Array of variables
    pub varArr: VariableArray,
    /// bucket size
    pub bucketSize: i32,
    /// no. of vars
    pub numberOfVars: i32,
    /// Set once a $START. variable is added, never cleared:
    ///      false means getVar of a $START. cref cannot succeed.
    pub hasStartVars: bool,
}

impl metamodelica::gc::MMTrace for Variables {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.crefIndices, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.prefixIndices, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varArr, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.bucketSize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hasStartVars, __mmv)?;
        Ok(())
    }
}
impl Default for Variables {
    fn default() -> Self {
        Self {
            crefIndices: Default::default(),
            prefixIndices: Default::default(),
            varArr: Default::default(),
            bucketSize: Default::default(),
            numberOfVars: Default::default(),
            hasStartVars: Default::default(),
        }
    }
}

pub type VARIABLES = Variables;

/// Component Reference Index
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CrefIndex {
    pub cref: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    pub index: i32,
}

impl metamodelica::gc::MMTrace for CrefIndex {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.cref, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        Ok(())
    }
}
impl Default for CrefIndex {
    fn default() -> Self {
        Self {
            cref: Default::default(),
            index: Default::default(),
        }
    }
}

pub type CREFINDEX = CrefIndex;

/// Indices of the variables under one array or record prefix
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct PrefixIndex {
    /// some variable name starting with the prefix
    pub cref: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    /// qualifiers of cref in the prefix
    pub depth: i32,
    /// subscripts of the last qualifier in the prefix
    pub numSubscripts: i32,
    pub indices: metamodelica::List<i32>,
}

impl metamodelica::gc::MMTrace for PrefixIndex {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.cref, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.depth, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numSubscripts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.indices, __mmv)?;
        Ok(())
    }
}
impl Default for PrefixIndex {
    fn default() -> Self {
        Self {
            cref: Default::default(),
            depth: Default::default(),
            numSubscripts: Default::default(),
            indices: Default::default(),
        }
    }
}

pub type PREFIXINDEX = PrefixIndex;

/// array of Equations are expandable, to amortize the cost of adding
///  equations in a more efficient manner
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VariableArray {
    /// no. elements
    pub numberOfElements: i32,
    pub varOptArr: metamodelica::Array<Option<metamodelica::Ref<Var>>>,
}

impl metamodelica::gc::MMTrace for VariableArray {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfElements, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varOptArr, __mmv)?;
        Ok(())
    }
}
impl Default for VariableArray {
    fn default() -> Self {
        Self {
            numberOfElements: Default::default(),
            varOptArr: Default::default(),
        }
    }
}

pub type VARIABLE_ARRAY = VariableArray;

pub type EquationArray = metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<Equation>>>;

/// variables
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Var {
    /// variable name
    pub varName: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    /// kind of variable
    pub varKind: VarKind,
    /// input, output or bidirectional
    pub varDirection: openmodelica_frontend_types::DAE::VarDirection,
    /// parallelism of the variable. parglobal, parlocal or non-parallel
    pub varParallelism: openmodelica_frontend_types::DAE::VarParallelism,
    /// built-in type or enumeration
    pub varType: Type,
    /// Binding expression e.g. for parameters
    pub bindExp: Option<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
    /// Variable is part of a tuple. Needed for the globalKnownVars and localKnownVars
    pub tplExp: Option<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
    /// array dimensions of non-expanded var
    pub arryDim: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Dimension>>,
    /// origin of variable
    pub source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
    /// values on built-in attributes
    pub values: Option<metamodelica::Ref<openmodelica_frontend_types::DAE::VariableAttributes>>,
    /// value for TearingSelect
    pub tearingSelectOption: Option<TearingSelect>,
    /// expression from the hideResult annotation
    pub hideResult: Option<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
    /// this contains the comment and annotation from Absyn
    pub comment: Option<metamodelica::Ref<SCode::Comment>>,
    /// flow, stream, unspecified or not connector.
    pub connectorType: metamodelica::Ref<openmodelica_frontend_types::DAE::ConnectorType>,
    /// inner, outer, inner outer or unspecified
    pub innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter,
    /// indicates if it is allowed to replace this variable
    pub unreplaceable: bool,
    /// indicates if the variable is a nonlinear iteration variable during initialization
    pub initNonlinear: bool,
    /// true if the variable belongs to an encrypted class
    pub encrypted: bool,
}

impl metamodelica::gc::MMTrace for Var {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.varName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varKind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varDirection, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varParallelism, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.bindExp, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tplExp, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arryDim, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.values, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tearingSelectOption, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hideResult, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.comment, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.connectorType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerOuter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.unreplaceable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initNonlinear, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.encrypted, __mmv)?;
        Ok(())
    }
}
impl Default for Var {
    fn default() -> Self {
        Self {
            varName: Default::default(),
            varKind: Default::default(),
            varDirection: Default::default(),
            varParallelism: Default::default(),
            varType: Default::default(),
            bindExp: Default::default(),
            tplExp: Default::default(),
            arryDim: Default::default(),
            source: Default::default(),
            values: Default::default(),
            tearingSelectOption: Default::default(),
            hideResult: Default::default(),
            comment: Default::default(),
            connectorType: Default::default(),
            innerOuter: Default::default(),
            unreplaceable: Default::default(),
            initNonlinear: Default::default(),
            encrypted: Default::default(),
        }
    }
}

pub type VAR = Var;

/// variable kind
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum VarKind {
    VARIABLE,
    STATE {
        /// how often this states was differentiated
        index: i32,
        /// the name of the derivative
        derName: Option<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        /// false if it was forced by StateSelect.always or StateSelect.prefer or generated by index reduction
        natural: bool,
    },
    STATE_DER,
    DUMMY_DER,
    DUMMY_STATE,
    CLOCKED_STATE {
        /// the name of the previous variable
        previousName: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        /// is fixed at first clock tick
        isStartFixed: bool,
    },
    DISCRETE,
    PARAM,
    CONST,
    EXTOBJ {
        fullClassName: metamodelica::Ref<Absyn::Path>,
    },
    JAC_VAR,
    JAC_TMP_VAR,
    SEED_VAR,
    OPT_CONSTR,
    OPT_FCONSTR,
    OPT_INPUT_WITH_DER,
    OPT_INPUT_DER,
    OPT_TGRID,
    OPT_LOOP_INPUT {
        replaceExp: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    },
    /// algebraic state used by inline solver
    ALG_STATE,
    /// algebraic state old value used by inline solver
    ALG_STATE_OLD,
    /// variable kind used for DAEmode
    DAE_RESIDUAL_VAR,
    /// auxiliary variable used for DAEmode
    DAE_AUX_VAR,
    /// used in SIMCODE, iteration variables in algebraic loops
    LOOP_ITERATION,
    /// used in SIMCODE, inner variables of a torn algebraic loop
    LOOP_SOLVED,
}
impl metamodelica::gc::MMTrace for VarKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            VarKind::VARIABLE => Ok(()),
            VarKind::STATE {
                index,
                derName,
                natural,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(derName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(natural, __mmv)?;
                Ok(())
            }
            VarKind::STATE_DER => Ok(()),
            VarKind::DUMMY_DER => Ok(()),
            VarKind::DUMMY_STATE => Ok(()),
            VarKind::CLOCKED_STATE {
                previousName,
                isStartFixed,
            } => {
                metamodelica::gc::MMTrace::mm_accept(previousName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isStartFixed, __mmv)?;
                Ok(())
            }
            VarKind::DISCRETE => Ok(()),
            VarKind::PARAM => Ok(()),
            VarKind::CONST => Ok(()),
            VarKind::EXTOBJ { fullClassName } => {
                metamodelica::gc::MMTrace::mm_accept(fullClassName, __mmv)?;
                Ok(())
            }
            VarKind::JAC_VAR => Ok(()),
            VarKind::JAC_TMP_VAR => Ok(()),
            VarKind::SEED_VAR => Ok(()),
            VarKind::OPT_CONSTR => Ok(()),
            VarKind::OPT_FCONSTR => Ok(()),
            VarKind::OPT_INPUT_WITH_DER => Ok(()),
            VarKind::OPT_INPUT_DER => Ok(()),
            VarKind::OPT_TGRID => Ok(()),
            VarKind::OPT_LOOP_INPUT { replaceExp } => {
                metamodelica::gc::MMTrace::mm_accept(replaceExp, __mmv)?;
                Ok(())
            }
            VarKind::ALG_STATE => Ok(()),
            VarKind::ALG_STATE_OLD => Ok(()),
            VarKind::DAE_RESIDUAL_VAR => Ok(()),
            VarKind::DAE_AUX_VAR => Ok(()),
            VarKind::LOOP_ITERATION => Ok(()),
            VarKind::LOOP_SOLVED => Ok(()),
        }
    }
}
impl Default for VarKind {
    fn default() -> Self {
        Self::VARIABLE
    }
}
pub use self::VarKind::{
    ALG_STATE, ALG_STATE_OLD, CLOCKED_STATE, CONST, DAE_AUX_VAR, DAE_RESIDUAL_VAR, DISCRETE, DUMMY_DER, DUMMY_STATE,
    EXTOBJ, JAC_TMP_VAR, JAC_VAR, LOOP_ITERATION, LOOP_SOLVED, OPT_CONSTR, OPT_FCONSTR, OPT_INPUT_DER,
    OPT_INPUT_WITH_DER, OPT_LOOP_INPUT, OPT_TGRID, PARAM, SEED_VAR, STATE, STATE_DER, VARIABLE,
};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TearingSelect {
    NEVER,
    AVOID,
    DEFAULT,
    PREFER,
    ALWAYS,
}
impl metamodelica::gc::MMTrace for TearingSelect {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TearingSelect::NEVER => Ok(()),
            TearingSelect::AVOID => Ok(()),
            TearingSelect::DEFAULT => Ok(()),
            TearingSelect::PREFER => Ok(()),
            TearingSelect::ALWAYS => Ok(()),
        }
    }
}
impl Default for TearingSelect {
    fn default() -> Self {
        Self::NEVER
    }
}
pub use self::TearingSelect::{ALWAYS, AVOID, DEFAULT, NEVER, PREFER};

pub const WHENCLK_PRREFIX: &'static str = "$whenclk";

/// equation kind
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum EquationKind {
    BINDING_EQUATION,
    DYNAMIC_EQUATION,
    INITIAL_EQUATION,
    CLOCKED_EQUATION { clk: i32 },
    DISCRETE_EQUATION,
    AUX_EQUATION,
    UNKNOWN_EQUATION_KIND,
}
impl metamodelica::gc::MMTrace for EquationKind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EquationKind::BINDING_EQUATION => Ok(()),
            EquationKind::DYNAMIC_EQUATION => Ok(()),
            EquationKind::INITIAL_EQUATION => Ok(()),
            EquationKind::CLOCKED_EQUATION { clk } => {
                metamodelica::gc::MMTrace::mm_accept(clk, __mmv)?;
                Ok(())
            }
            EquationKind::DISCRETE_EQUATION => Ok(()),
            EquationKind::AUX_EQUATION => Ok(()),
            EquationKind::UNKNOWN_EQUATION_KIND => Ok(()),
        }
    }
}
impl Default for EquationKind {
    fn default() -> Self {
        Self::BINDING_EQUATION
    }
}
pub use self::EquationKind::{
    AUX_EQUATION, BINDING_EQUATION, CLOCKED_EQUATION, DISCRETE_EQUATION, DYNAMIC_EQUATION, INITIAL_EQUATION,
    UNKNOWN_EQUATION_KIND,
};

/// evaluation stages
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EvaluationStages {
    pub dynamicEval: bool,
    pub algebraicEval: bool,
    pub zerocrossEval: bool,
    pub discreteEval: bool,
}

impl metamodelica::gc::MMTrace for EvaluationStages {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.dynamicEval, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.algebraicEval, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.zerocrossEval, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.discreteEval, __mmv)?;
        Ok(())
    }
}
impl Default for EvaluationStages {
    fn default() -> Self {
        Self {
            dynamicEval: Default::default(),
            algebraicEval: Default::default(),
            zerocrossEval: Default::default(),
            discreteEval: Default::default(),
        }
    }
}

pub type EVALUATION_STAGES = EvaluationStages;

pub static defaultEvalStages: EvaluationStages = EvaluationStages {
    dynamicEval: false,
    algebraicEval: false,
    zerocrossEval: false,
    discreteEval: false,
};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EquationAttributes {
    /// true if the equation was differentiated, and should not be differentiated again to avoid equal equations
    pub differentiated: bool,
    pub kind: EquationKind,
    pub evalStages: EvaluationStages,
}

impl metamodelica::gc::MMTrace for EquationAttributes {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.differentiated, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.evalStages, __mmv)?;
        Ok(())
    }
}
impl Default for EquationAttributes {
    fn default() -> Self {
        Self {
            differentiated: Default::default(),
            kind: Default::default(),
            evalStages: Default::default(),
        }
    }
}

pub type EQUATION_ATTRIBUTES = EquationAttributes;

pub static EQ_ATTR_DEFAULT_DYNAMIC: std::sync::LazyLock<EquationAttributes> =
    std::sync::LazyLock::new(|| EquationAttributes {
        differentiated: false,
        kind: crate::BackendDAE::EquationKind::DYNAMIC_EQUATION,
        evalStages: defaultEvalStages.clone(),
    });

pub static EQ_ATTR_DEFAULT_BINDING: std::sync::LazyLock<EquationAttributes> =
    std::sync::LazyLock::new(|| EquationAttributes {
        differentiated: false,
        kind: crate::BackendDAE::EquationKind::BINDING_EQUATION,
        evalStages: defaultEvalStages.clone(),
    });

pub static EQ_ATTR_DEFAULT_INITIAL: std::sync::LazyLock<EquationAttributes> =
    std::sync::LazyLock::new(|| EquationAttributes {
        differentiated: false,
        kind: crate::BackendDAE::EquationKind::INITIAL_EQUATION,
        evalStages: defaultEvalStages.clone(),
    });

pub static EQ_ATTR_DEFAULT_DISCRETE: std::sync::LazyLock<EquationAttributes> =
    std::sync::LazyLock::new(|| EquationAttributes {
        differentiated: false,
        kind: crate::BackendDAE::EquationKind::DISCRETE_EQUATION,
        evalStages: defaultEvalStages.clone(),
    });

pub static EQ_ATTR_DEFAULT_AUX: std::sync::LazyLock<EquationAttributes> =
    std::sync::LazyLock::new(|| EquationAttributes {
        differentiated: false,
        kind: crate::BackendDAE::EquationKind::AUX_EQUATION,
        evalStages: defaultEvalStages.clone(),
    });

pub static EQ_ATTR_DEFAULT_UNKNOWN: std::sync::LazyLock<EquationAttributes> =
    std::sync::LazyLock::new(|| EquationAttributes {
        differentiated: false,
        kind: crate::BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND,
        evalStages: defaultEvalStages.clone(),
    });

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Equation {
    EQUATION {
        exp: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        scalar: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    ARRAY_EQUATION {
        /// dimension sizes
        dimSize: metamodelica::List<i32>,
        /// lhs
        left: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// rhs
        right: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
        /// NONE() if not a record
        recordSize: Option<i32>,
    },
    SOLVED_EQUATION {
        componentRef: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        exp: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    RESIDUAL_EQUATION {
        /// not present from FrontEnd
        exp: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    ALGORITHM {
        /// size of equation
        size: i32,
        alg: metamodelica::Ref<openmodelica_frontend_types::DAE::Algorithm>,
        /// origin of algorithm
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        /// this algorithm was translated from an equation. we should not expand array crefs!
        expand: openmodelica_frontend_types::DAE::Expand,
        attr: EquationAttributes,
    },
    WHEN_EQUATION {
        /// size of equation
        size: i32,
        whenEquation: metamodelica::Ref<WhenEquation>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    /// complex equations: recordX = function call(x, y, ..);
    COMPLEX_EQUATION {
        /// size of equation
        size: i32,
        /// lhs
        left: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// rhs
        right: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    /// an if-equation
    IF_EQUATION {
        /// Condition
        conditions: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
        /// Equations of true branch
        eqnstrue: metamodelica::List<metamodelica::List<metamodelica::Ref<Equation>>>,
        /// Equations of false branch
        eqnsfalse: metamodelica::List<metamodelica::Ref<Equation>>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    /// a for-equation
    FOR_EQUATION {
        /// the iterator variable
        iter: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// start of iteration
        start: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// end of iteration
        stop: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// iterated equation
        body: metamodelica::Ref<Equation>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
        attr: EquationAttributes,
    },
    DUMMY_EQUATION,
}
impl metamodelica::gc::MMTrace for Equation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Equation::EQUATION {
                exp,
                scalar,
                source,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scalar, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::ARRAY_EQUATION {
                dimSize,
                left,
                right,
                source,
                attr,
                recordSize,
            } => {
                metamodelica::gc::MMTrace::mm_accept(dimSize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(recordSize, __mmv)?;
                Ok(())
            }
            Equation::SOLVED_EQUATION {
                componentRef,
                exp,
                source,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(componentRef, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::RESIDUAL_EQUATION { exp, source, attr } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::ALGORITHM {
                size,
                alg,
                source,
                expand,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(alg, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(expand, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::WHEN_EQUATION {
                size,
                whenEquation,
                source,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(whenEquation, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::COMPLEX_EQUATION {
                size,
                left,
                right,
                source,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::IF_EQUATION {
                conditions,
                eqnstrue,
                eqnsfalse,
                source,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(conditions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqnstrue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqnsfalse, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::FOR_EQUATION {
                iter,
                start,
                stop,
                body,
                source,
                attr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stop, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                Ok(())
            }
            Equation::DUMMY_EQUATION => Ok(()),
        }
    }
}
impl Equation {
    pub fn interned_DUMMY_EQUATION() -> metamodelica::Ref<Equation> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Equation> = metamodelica::Ref::new(Equation::DUMMY_EQUATION);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_DUMMY_EQUATION() -> metamodelica::Ref<Equation> {
    Equation::interned_DUMMY_EQUATION()
}
impl Default for Equation {
    fn default() -> Self {
        Self::DUMMY_EQUATION
    }
}
pub use self::Equation::{
    ALGORITHM, ARRAY_EQUATION, COMPLEX_EQUATION, DUMMY_EQUATION, EQUATION, FOR_EQUATION, IF_EQUATION,
    RESIDUAL_EQUATION, SOLVED_EQUATION, WHEN_EQUATION,
};

/// equation when condition then cr = exp, reinit(...), terminate(...) or assert(...)
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct WhenEquation {
    /// the when-condition
    pub condition: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
    pub whenStmtLst: metamodelica::List<WhenOperator>,
    /// elsewhen equation with the same cref on the left hand side.
    pub elsewhenPart: Option<metamodelica::Ref<WhenEquation>>,
}

impl metamodelica::gc::MMTrace for WhenEquation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.whenStmtLst, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.elsewhenPart, __mmv)?;
        Ok(())
    }
}
impl Default for WhenEquation {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            whenStmtLst: Default::default(),
            elsewhenPart: Default::default(),
        }
    }
}

pub type WHEN_STMTS = WhenEquation;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum WhenOperator {
    /// left_cr = right_exp
    ASSIGN {
        /// left hand side of equation
        left: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// right hand side of equation
        right: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
    },
    /// Reinit Statement
    REINIT {
        /// State variable to reinit
        stateVar: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        /// Value after reinit
        value: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// origin of equation
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
    },
    ASSERT {
        condition: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        message: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        level: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
    },
    /// The Modelica built-in terminate(msg)
    TERMINATE {
        message: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
    },
    /// call with no return value, i.e. no equation.
    ///    Typically side effect call of external function but also
    ///    Connections.* i.e. Connections.root(...) functions.
    NORETCALL {
        exp: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// the origin of the component/equation/algorithm
        source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
    },
}
impl metamodelica::gc::MMTrace for WhenOperator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            WhenOperator::ASSIGN { left, right, source } => {
                metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            WhenOperator::REINIT {
                stateVar,
                value,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(stateVar, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            WhenOperator::ASSERT {
                condition,
                message,
                level,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            WhenOperator::TERMINATE { message, source } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            WhenOperator::NORETCALL { exp, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for WhenOperator {
    fn default() -> Self {
        Self::TERMINATE {
            message: Default::default(),
            source: Default::default(),
        }
    }
}
pub use self::WhenOperator::{ASSERT, ASSIGN, NORETCALL, REINIT, TERMINATE};

/// classes of external objects stored in list
pub type ExternalObjectClasses = metamodelica::List<ExternalObjectClass>;

/// class of external objects
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ExternalObjectClass {
    /// className of external object
    pub path: metamodelica::Ref<Absyn::Path>,
    /// origin of equation
    pub source: metamodelica::Ref<openmodelica_frontend_types::DAE::ElementSource>,
}

impl metamodelica::gc::MMTrace for ExternalObjectClass {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        Ok(())
    }
}
impl Default for ExternalObjectClass {
    fn default() -> Self {
        Self {
            path: Default::default(),
            source: Default::default(),
        }
    }
}

pub type EXTOBJCLASS = ExternalObjectClass;

//
//  Matching, strong components and StateSets
//
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Matching {
    /// matching has not yet been performed
    NO_MATCHING,
    /// not yet used
    MATCHING {
        /// ass[varindx]=eqnindx
        ass1: metamodelica::Array<i32>,
        /// ass[eqnindx]=varindx
        ass2: metamodelica::Array<i32>,
        comps: StrongComponents,
    },
}
impl metamodelica::gc::MMTrace for Matching {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Matching::NO_MATCHING => Ok(()),
            Matching::MATCHING { ass1, ass2, comps } => {
                metamodelica::gc::MMTrace::mm_accept(ass1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ass2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comps, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Matching {
    pub fn interned_NO_MATCHING() -> metamodelica::Ref<Matching> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Matching> = metamodelica::Ref::new(Matching::NO_MATCHING);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NO_MATCHING() -> metamodelica::Ref<Matching> {
    Matching::interned_NO_MATCHING()
}
impl Default for Matching {
    fn default() -> Self {
        Self::NO_MATCHING
    }
}
pub use self::Matching::{MATCHING, NO_MATCHING};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum IndexReduction {
    /// Use index reduction during matching
    INDEX_REDUCTION,
    /// do not use index reduction during matching
    NO_INDEX_REDUCTION,
}
impl metamodelica::gc::MMTrace for IndexReduction {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            IndexReduction::INDEX_REDUCTION => Ok(()),
            IndexReduction::NO_INDEX_REDUCTION => Ok(()),
        }
    }
}
pub use self::IndexReduction::{INDEX_REDUCTION, NO_INDEX_REDUCTION};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum EquationConstraints {
    /// for e.g. initial eqns.
    ///                  where not all variables
    ///                  have a solution
    ALLOW_UNDERCONSTRAINED,
    /// exact as many equations
    ///                   as variables
    EXACT,
}
impl metamodelica::gc::MMTrace for EquationConstraints {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            EquationConstraints::ALLOW_UNDERCONSTRAINED => Ok(()),
            EquationConstraints::EXACT => Ok(()),
        }
    }
}
pub use self::EquationConstraints::{ALLOW_UNDERCONSTRAINED, EXACT};

pub type MatchingOptions = (IndexReduction, EquationConstraints);

/// StateOrder,ConstraintEqns,Eqn->EqnsIndxes,EqnIndex->Eqns,NrOfEqnsbeforeIndexReduction
pub type StructurallySingularSystemHandlerArg = (
    StateOrder,
    metamodelica::Array<metamodelica::List<metamodelica::Ref<Equation>>>,
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    i32,
);

pub type ConstraintEquations = metamodelica::Array<metamodelica::List<metamodelica::Ref<Equation>>>;

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub enum StateOrder {
    STATEORDER {
        /// x -> dx
        hashTable: (
            metamodelica::Array<
                metamodelica::List<(metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>, i32)>,
            >,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
                        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
                    )>,
                >,
            ),
            i32,
            (
                HashTableCG::FuncHashCref,
                HashTableCG::FuncCrefEqual,
                HashTableCG::FuncCrefStr,
                HashTableCG::FuncExpStr,
            ),
        ),
        /// dx -> {x,y,z}
        invHashTable: (
            metamodelica::Array<
                metamodelica::List<(metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>, i32)>,
            >,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
                        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
                    )>,
                >,
            ),
            i32,
            (
                HashTable3::FuncHashCref,
                HashTable3::FuncCrefEqual,
                HashTable3::FuncCrefStr,
                HashTable3::FuncExpStr,
            ),
        ),
    },
    /// Index reduction disabled; don't need big hashtables
    NOSTATEORDER,
}
impl metamodelica::gc::MMTrace for StateOrder {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            StateOrder::STATEORDER {
                hashTable,
                invHashTable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(hashTable, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(invHashTable, __mmv)?;
                Ok(())
            }
            StateOrder::NOSTATEORDER => Ok(()),
        }
    }
}
impl PartialEq for StateOrder {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::STATEORDER {
                    hashTable: __l_hashTable,
                    invHashTable: __l_invHashTable,
                },
                Self::STATEORDER {
                    hashTable: __r_hashTable,
                    invHashTable: __r_invHashTable,
                },
            ) => {
                (match (__l_hashTable, __r_hashTable) {
                    ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                        (__lt0 == __rt0)
                            && (__lt1 == __rt1)
                            && (__lt2 == __rt2)
                            && (match (__lt3, __rt3) {
                                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                    std::sync::Arc::ptr_eq(__lt0, __rt0)
                                        && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                        && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                        && std::sync::Arc::ptr_eq(__lt3, __rt3)
                                }
                            })
                    }
                }) && (match (__l_invHashTable, __r_invHashTable) {
                    ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                        (__lt0 == __rt0)
                            && (__lt1 == __rt1)
                            && (__lt2 == __rt2)
                            && (match (__lt3, __rt3) {
                                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                    std::sync::Arc::ptr_eq(__lt0, __rt0)
                                        && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                        && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                        && std::sync::Arc::ptr_eq(__lt3, __rt3)
                                }
                            })
                    }
                })
            }
            (Self::NOSTATEORDER, Self::NOSTATEORDER) => true,
            _ => false,
        }
    }
}
impl Eq for StateOrder {}
impl PartialOrd for StateOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StateOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn __variant_idx(__v: &StateOrder) -> u32 {
            match __v {
                StateOrder::STATEORDER { .. } => 0,
                StateOrder::NOSTATEORDER => 1,
            }
        }
        match __variant_idx(self).cmp(&__variant_idx(other)) {
            std::cmp::Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match (self, other) {
            (
                Self::STATEORDER {
                    hashTable: __l_hashTable,
                    invHashTable: __l_invHashTable,
                },
                Self::STATEORDER {
                    hashTable: __r_hashTable,
                    invHashTable: __r_invHashTable,
                },
            ) => (match (__l_hashTable, __r_hashTable) {
                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                    .cmp(__rt0)
                    .then_with(|| __lt1.cmp(__rt1))
                    .then_with(|| __lt2.cmp(__rt2))
                    .then_with(|| {
                        (match (__lt3, __rt3) {
                            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                (std::sync::Arc::as_ptr(__lt0) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt1) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                    })
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt2) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                    })
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt3) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                                    })
                            }
                        })
                    }),
            })
            .then_with(|| {
                (match (__l_invHashTable, __r_invHashTable) {
                    ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                        .cmp(__rt0)
                        .then_with(|| __lt1.cmp(__rt1))
                        .then_with(|| __lt2.cmp(__rt2))
                        .then_with(|| {
                            (match (__lt3, __rt3) {
                                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                    (std::sync::Arc::as_ptr(__lt0) as *const ())
                                        .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                        .then_with(|| {
                                            (std::sync::Arc::as_ptr(__lt1) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                        })
                                        .then_with(|| {
                                            (std::sync::Arc::as_ptr(__lt2) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                        })
                                        .then_with(|| {
                                            (std::sync::Arc::as_ptr(__lt3) as *const ())
                                                .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                                        })
                                }
                            })
                        }),
                })
            }),
            (Self::NOSTATEORDER, Self::NOSTATEORDER) => std::cmp::Ordering::Equal,
            _ => unreachable!("variant-index equality already implies same variant"),
        }
    }
}
impl std::fmt::Debug for StateOrder {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STATEORDER {
                hashTable: __d_hashTable,
                invHashTable: __d_invHashTable,
            } => {
                let mut __ds = __f.debug_struct("STATEORDER");
                __ds.field(
                    "hashTable",
                    &format_args!("<dyn-fn-container@{:p}>", __d_hashTable as *const _),
                );
                __ds.field(
                    "invHashTable",
                    &format_args!("<dyn-fn-container@{:p}>", __d_invHashTable as *const _),
                );
                __ds.finish()
            }
            Self::NOSTATEORDER => __f.debug_struct("NOSTATEORDER").finish(),
        }
    }
}

impl Default for StateOrder {
    fn default() -> Self {
        Self::NOSTATEORDER
    }
}
pub use self::StateOrder::{NOSTATEORDER, STATEORDER};

/// Order of the equations the have to be solved
pub type StrongComponents = metamodelica::List<metamodelica::Ref<StrongComponent>>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum StrongComponent {
    SINGLEEQUATION {
        eqn: i32,
        var: i32,
    },
    EQUATIONSYSTEM {
        eqns: metamodelica::List<i32>,
        /// be careful with states, this are solved for der(x)
        vars: metamodelica::List<i32>,
        jac: metamodelica::Ref<Jacobian>,
        jacType: JacobianType,
        /// true for system that discrete dependencies to the iteration variables
        mixedSystem: bool,
    },
    SINGLEARRAY {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: metamodelica::List<i32>,
    },
    SINGLEALGORITHM {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: metamodelica::List<i32>,
    },
    SINGLECOMPLEXEQUATION {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: metamodelica::List<i32>,
    },
    SINGLEWHENEQUATION {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: metamodelica::List<i32>,
    },
    SINGLEIFEQUATION {
        eqn: i32,
        /// be careful with states, this are solved for der(x)
        vars: metamodelica::List<i32>,
    },
    TORNSYSTEM {
        strictTearingSet: TearingSet,
        casualTearingSet: Option<TearingSet>,
        linear: bool,
        /// true for system that discrete dependencies to the iteration variables
        mixedSystem: bool,
    },
}
impl metamodelica::gc::MMTrace for StrongComponent {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            StrongComponent::SINGLEEQUATION { eqn, var } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            StrongComponent::EQUATIONSYSTEM {
                eqns,
                vars,
                jac,
                jacType,
                mixedSystem,
            } => {
                metamodelica::gc::MMTrace::mm_accept(eqns, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(jac, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(jacType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mixedSystem, __mmv)?;
                Ok(())
            }
            StrongComponent::SINGLEARRAY { eqn, vars } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
            StrongComponent::SINGLEALGORITHM { eqn, vars } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
            StrongComponent::SINGLECOMPLEXEQUATION { eqn, vars } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
            StrongComponent::SINGLEWHENEQUATION { eqn, vars } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
            StrongComponent::SINGLEIFEQUATION { eqn, vars } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
            StrongComponent::TORNSYSTEM {
                strictTearingSet,
                casualTearingSet,
                linear,
                mixedSystem,
            } => {
                metamodelica::gc::MMTrace::mm_accept(strictTearingSet, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(casualTearingSet, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(linear, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mixedSystem, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for StrongComponent {
    fn default() -> Self {
        Self::SINGLEEQUATION {
            eqn: Default::default(),
            var: Default::default(),
        }
    }
}
pub use self::StrongComponent::{
    EQUATIONSYSTEM, SINGLEALGORITHM, SINGLEARRAY, SINGLECOMPLEXEQUATION, SINGLEEQUATION, SINGLEIFEQUATION,
    SINGLEWHENEQUATION, TORNSYSTEM,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TearingSet {
    pub tearingvars: metamodelica::List<i32>,
    pub residualequations: metamodelica::List<i32>,
    /// list of matched equations and variables; these will be solved explicitly in the given order
    pub innerEquations: InnerEquations,
    pub jac: metamodelica::Ref<Jacobian>,
}

impl metamodelica::gc::MMTrace for TearingSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.tearingvars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.residualequations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jac, __mmv)?;
        Ok(())
    }
}
impl Default for TearingSet {
    fn default() -> Self {
        Self {
            tearingvars: Default::default(),
            residualequations: Default::default(),
            innerEquations: Default::default(),
            jac: Default::default(),
        }
    }
}

pub type TEARINGSET = TearingSet;

pub type InnerEquations = metamodelica::List<InnerEquation>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum InnerEquation {
    INNEREQUATION {
        eqn: i32,
        vars: metamodelica::List<i32>,
    },
    INNEREQUATIONCONSTRAINTS {
        eqn: i32,
        vars: metamodelica::List<i32>,
        cons: Constraints,
    },
}
impl metamodelica::gc::MMTrace for InnerEquation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            InnerEquation::INNEREQUATION { eqn, vars } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
            InnerEquation::INNEREQUATIONCONSTRAINTS { eqn, vars, cons } => {
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cons, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for InnerEquation {
    fn default() -> Self {
        Self::INNEREQUATION {
            eqn: Default::default(),
            vars: Default::default(),
        }
    }
}
pub use self::InnerEquation::{INNEREQUATION, INNEREQUATIONCONSTRAINTS};

/// List of StateSets
pub type StateSets = metamodelica::List<StateSet>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct StateSet {
    pub index: i32,
    pub rang: i32,
    pub state: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    /// set.x=A*states
    pub crA: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    pub varA: metamodelica::List<metamodelica::Ref<Var>>,
    pub statescandidates: metamodelica::List<metamodelica::Ref<Var>>,
    pub ovars: metamodelica::List<metamodelica::Ref<Var>>,
    pub eqns: metamodelica::List<metamodelica::Ref<Equation>>,
    pub oeqns: metamodelica::List<metamodelica::Ref<Equation>>,
    pub crJ: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    pub varJ: metamodelica::List<metamodelica::Ref<Var>>,
    pub jacobian: metamodelica::Ref<Jacobian>,
}

impl metamodelica::gc::MMTrace for StateSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rang, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.state, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crA, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varA, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.statescandidates, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ovars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.oeqns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crJ, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varJ, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobian, __mmv)?;
        Ok(())
    }
}
impl Default for StateSet {
    fn default() -> Self {
        Self {
            index: Default::default(),
            rang: Default::default(),
            state: Default::default(),
            crA: Default::default(),
            varA: Default::default(),
            statescandidates: Default::default(),
            ovars: Default::default(),
            eqns: Default::default(),
            oeqns: Default::default(),
            crJ: Default::default(),
            varJ: Default::default(),
            jacobian: Default::default(),
        }
    }
}

pub type STATESET = StateSet;

//
// event info and stuff
//
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EventInfo {
    /// stores all information related to time events
    pub timeEvents: metamodelica::List<TimeEvent>,
    /// list of zero crossing conditions
    pub zeroCrossings: ZeroCrossingSet,
    /// list of zero crossing function as before
    pub relations: ZeroCrossingSet,
    /// [deprecated] list of sample as before, only used by cpp runtime (TODO: REMOVE ME)
    pub samples: ZeroCrossingSet,
    /// stores the number of math function that trigger events e.g. floor, ceil, integer, ...
    pub numberMathEvents: i32,
}

impl metamodelica::gc::MMTrace for EventInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.timeEvents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.zeroCrossings, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.samples, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numberMathEvents, __mmv)?;
        Ok(())
    }
}
impl Default for EventInfo {
    fn default() -> Self {
        Self {
            timeEvents: Default::default(),
            zeroCrossings: Default::default(),
            relations: Default::default(),
            samples: Default::default(),
            numberMathEvents: Default::default(),
        }
    }
}

pub type EVENT_INFO = EventInfo;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ZeroCrossingSet {
    pub zc: DoubleEnded::MutableList<ZeroCrossing>,
    pub tree: metamodelica::Array<metamodelica::Ref<ZeroCrossings::ZeroCrossingTree::Tree>>,
}

impl metamodelica::gc::MMTrace for ZeroCrossingSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.zc, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tree, __mmv)?;
        Ok(())
    }
}
impl Default for ZeroCrossingSet {
    fn default() -> Self {
        Self {
            zc: Default::default(),
            tree: Default::default(),
        }
    }
}

pub type ZERO_CROSSING_SET = ZeroCrossingSet;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ZeroCrossing {
    /// zero crossing index
    pub index: i32,
    /// function
    pub relation_: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
    /// list of equations where the function occurs
    pub occurEquLst: metamodelica::List<i32>,
    /// optional iterator for for-loops
    pub iter: Option<metamodelica::List<SimIterator>>,
}

impl metamodelica::gc::MMTrace for ZeroCrossing {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relation_, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.occurEquLst, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.iter, __mmv)?;
        Ok(())
    }
}
impl Default for ZeroCrossing {
    fn default() -> Self {
        Self {
            index: Default::default(),
            relation_: Default::default(),
            occurEquLst: Default::default(),
            iter: Default::default(),
        }
    }
}

pub type ZERO_CROSSING = ZeroCrossing;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SimIterator {
    SIM_ITERATOR_RANGE {
        name: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        start: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        step: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        stop: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        size: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        non_resizable_size: i32,
        sub_iter: metamodelica::List<(
            metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
            metamodelica::Array<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
        )>,
    },
    SIM_ITERATOR_LIST {
        name: metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        lst: metamodelica::List<i32>,
        size: i32,
        sub_iter: metamodelica::List<(
            metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
            metamodelica::Array<metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>>,
        )>,
    },
}
impl metamodelica::gc::MMTrace for SimIterator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SimIterator::SIM_ITERATOR_RANGE {
                name,
                start,
                step,
                stop,
                size,
                non_resizable_size,
                sub_iter,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(step, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stop, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(non_resizable_size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sub_iter, __mmv)?;
                Ok(())
            }
            SimIterator::SIM_ITERATOR_LIST {
                name,
                lst,
                size,
                sub_iter,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sub_iter, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for SimIterator {
    fn default() -> Self {
        Self::SIM_ITERATOR_LIST {
            name: Default::default(),
            lst: Default::default(),
            size: Default::default(),
            sub_iter: Default::default(),
        }
    }
}
pub use self::SimIterator::{SIM_ITERATOR_LIST, SIM_ITERATOR_RANGE};

pub fn getSimIteratorSize(mut iters: &metamodelica::List<SimIterator>) -> i32 {
    let mut size: i32 = 1;
    let mut local_size: i32;
    for mut iter in &**iters {
        local_size = (match iter.clone() {
            SimIterator::SIM_ITERATOR_RANGE { .. } => {
                var_field!(iter.non_resizable_size, SimIterator::SIM_ITERATOR_RANGE).clone()
            }
            SimIterator::SIM_ITERATOR_LIST { .. } => var_field!(iter.size, SimIterator::SIM_ITERATOR_LIST).clone(),
        });
        size = size * local_size;
    }
    size
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TimeEvent {
    /// e.g. time > 0.5
    SIMPLE_TIME_EVENT,
    /// e.g. sample(1, 1)
    SAMPLE_TIME_EVENT {
        /// unique sample index
        index: i32,
        startExp: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        intervalExp: metamodelica::Ref<openmodelica_frontend_types::DAE::Exp>,
        /// optional iterator for for-loops
        iter: Option<metamodelica::List<SimIterator>>,
    },
}
impl metamodelica::gc::MMTrace for TimeEvent {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            TimeEvent::SIMPLE_TIME_EVENT => Ok(()),
            TimeEvent::SAMPLE_TIME_EVENT {
                index,
                startExp,
                intervalExp,
                iter,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(intervalExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for TimeEvent {
    fn default() -> Self {
        Self::SIMPLE_TIME_EVENT
    }
}
pub use self::TimeEvent::{SAMPLE_TIME_EVENT, SIMPLE_TIME_EVENT};

//
// AdjacencyMatrices
//
pub type AdjacencyMatrixElementEntry = i32;

pub type AdjacencyMatrixElement = metamodelica::List<i32>;

/// array<list<Integer>>
pub type AdjacencyMatrix = metamodelica::Array<metamodelica::List<i32>>;

/// a list of equation indices (1..n), one for each variable. Equations that -only-
/// contain the state variable and not the derivative have a negative index.
pub type AdjacencyMatrixT = metamodelica::Array<metamodelica::List<i32>>;

/// a mapping for adjacency matrices that contains:
/// array<list<Integer>>: array index -> scalar index list
/// array<Integer>      : scalar index -> array index (not unique)
/// IndexType           : the occurence condition type for the current adjacency matrix
/// Boolean             : true if scalar
/// Boolean             : true if analytical to structural singularity processing has already been done
pub type AdjacencyMatrixMapping = (
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    IndexType,
    bool,
    bool,
);

pub type AdjacencyMatrixElementEnhancedEntry = (
    i32,
    Solvability,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Constraint>>,
);

pub type AdjacencyMatrixElementEnhanced = metamodelica::List<(
    i32,
    Solvability,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Constraint>>,
)>;

pub type AdjacencyMatrixEnhanced = metamodelica::Array<
    metamodelica::List<(
        i32,
        Solvability,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Constraint>>,
    )>,
>;

pub type AdjacencyMatrixTEnhanced = metamodelica::Array<
    metamodelica::List<(
        i32,
        Solvability,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Constraint>>,
    )>,
>;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Solvability {
    /// Equation is already solved for the variable
    SOLVABILITY_SOLVED,
    /// Coefficient is equal 1 or -1
    SOLVABILITY_CONSTONE,
    /// Coefficient is constant
    SOLVABILITY_CONST {
        /// false if the constant is almost zero (<1e-6)
        b: bool,
    },
    /// Coefficient contains parameters
    SOLVABILITY_PARAMETER {
        /// false if the partial derivative is zero
        b: bool,
    },
    /// Coefficient contains variables, is time varying
    SOLVABILITY_LINEAR {
        /// false if the partial derivative is zero
        b: bool,
    },
    /// The variable occurs non-linear in the equation.
    SOLVABILITY_NONLINEAR,
    /// The variable occurs in the equation, but it is not possible to solve
    ///                     the equation for it.
    SOLVABILITY_UNSOLVABLE,
    /// It is possible to solve the equation for the variable, it is not considered
    ///                     how the variable occurs in the equation.
    SOLVABILITY_SOLVABLE,
}
impl metamodelica::gc::MMTrace for Solvability {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Solvability::SOLVABILITY_SOLVED => Ok(()),
            Solvability::SOLVABILITY_CONSTONE => Ok(()),
            Solvability::SOLVABILITY_CONST { b } => {
                metamodelica::gc::MMTrace::mm_accept(b, __mmv)?;
                Ok(())
            }
            Solvability::SOLVABILITY_PARAMETER { b } => {
                metamodelica::gc::MMTrace::mm_accept(b, __mmv)?;
                Ok(())
            }
            Solvability::SOLVABILITY_LINEAR { b } => {
                metamodelica::gc::MMTrace::mm_accept(b, __mmv)?;
                Ok(())
            }
            Solvability::SOLVABILITY_NONLINEAR => Ok(()),
            Solvability::SOLVABILITY_UNSOLVABLE => Ok(()),
            Solvability::SOLVABILITY_SOLVABLE => Ok(()),
        }
    }
}
impl Default for Solvability {
    fn default() -> Self {
        Self::SOLVABILITY_SOLVED
    }
}
pub use self::Solvability::{
    SOLVABILITY_CONST, SOLVABILITY_CONSTONE, SOLVABILITY_LINEAR, SOLVABILITY_NONLINEAR, SOLVABILITY_PARAMETER,
    SOLVABILITY_SOLVABLE, SOLVABILITY_SOLVED, SOLVABILITY_UNSOLVABLE,
};

/// Constraints on the solvability of the (casual) tearing set; needed for proper Dynamic Tearing
pub type Constraints = metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::Constraint>>;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum IndexType {
    /// adjacency matrix with absolute indexes
    ABSOLUTE,
    /// adjacency matrix with positive/negative indexes
    NORMAL,
    /// adjacency matrix with only solvable entries, for example {a,b,c}[d] then d is skipped
    SOLVABLE,
    /// adjacency matrix for base-clock partitioning
    BASECLOCK_IDX,
    /// adjacency matrix for sub-clock partitioning
    SUBCLOCK_IDX,
    /// adjacency matrix as normal, but add for inputs also a value
    SPARSE,
}
impl metamodelica::gc::MMTrace for IndexType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            IndexType::ABSOLUTE => Ok(()),
            IndexType::NORMAL => Ok(()),
            IndexType::SOLVABLE => Ok(()),
            IndexType::BASECLOCK_IDX => Ok(()),
            IndexType::SUBCLOCK_IDX => Ok(()),
            IndexType::SPARSE => Ok(()),
        }
    }
}
impl Default for IndexType {
    fn default() -> Self {
        Self::ABSOLUTE
    }
}
pub use self::IndexType::{ABSOLUTE, BASECLOCK_IDX, NORMAL, SOLVABLE, SPARSE, SUBCLOCK_IDX};

//
// Jacobian stuff
//
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum JacobianType {
    /// If Jacobian has only constant values, for system
    ///               of equations this means that it can be solved statically.
    JAC_CONSTANT,
    /// If Jacobian has time varying parts, like parameters or
    ///                  algebraic variables
    JAC_LINEAR,
    /// If Jacobian contains variables that are solved for,
    ///              means that a non-linear system of equations needs to be
    ///              solved
    JAC_NONLINEAR,
    /// GENERIC_JACOBIAN Jacobian available
    JAC_GENERIC,
    /// No analytic Jacobian available
    JAC_NO_ANALYTIC,
}
impl metamodelica::gc::MMTrace for JacobianType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            JacobianType::JAC_CONSTANT => Ok(()),
            JacobianType::JAC_LINEAR => Ok(()),
            JacobianType::JAC_NONLINEAR => Ok(()),
            JacobianType::JAC_GENERIC => Ok(()),
            JacobianType::JAC_NO_ANALYTIC => Ok(()),
        }
    }
}
impl Default for JacobianType {
    fn default() -> Self {
        Self::JAC_CONSTANT
    }
}
pub use self::JacobianType::{JAC_CONSTANT, JAC_GENERIC, JAC_LINEAR, JAC_NO_ANALYTIC, JAC_NONLINEAR};

pub const SymbolicJacobianAIndex: i32 = 1;

pub(crate) const SymbolicJacobianBIndex: i32 = 2;

pub(crate) const SymbolicJacobianCIndex: i32 = 3;

pub(crate) const SymbolicJacobianDIndex: i32 = 4;

pub const derivativeNamePrefix: &'static str = "$DERAlias";

pub const partialDerivativeNamePrefix: &'static str = "$pDER";

pub const functionDerivativeNamePrefix: &'static str = "$funDER";

pub const outputAliasPrefix: &'static str = "$outputAlias_";

pub const optimizationMayerTermName: &'static str = "$OMC$objectMayerTerm";

pub const optimizationLagrangeTermName: &'static str = "$OMC$objectLagrangeTerm";

pub const symSolverDT: &'static str = "__OMC_DT";

pub const homotopyLambda: &'static str = "__HOM_LAMBDA";

pub type FullJacobian = Option<metamodelica::List<(i32, i32, metamodelica::Ref<Equation>)>>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Jacobian {
    FULL_JACOBIAN {
        jacobian: FullJacobian,
    },
    GENERIC_JACOBIAN {
        jacobian: Option<(
            metamodelica::Ref<BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<Var>>,
            metamodelica::List<metamodelica::Ref<Var>>,
            metamodelica::List<metamodelica::Ref<Var>>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        )>,
        sparsePattern: SparsePattern,
        coloring: SparseColoring,
        nonlinearPattern: NonlinearPattern,
    },
    EMPTY_JACOBIAN,
}
impl metamodelica::gc::MMTrace for Jacobian {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Jacobian::FULL_JACOBIAN { jacobian } => {
                metamodelica::gc::MMTrace::mm_accept(jacobian, __mmv)?;
                Ok(())
            }
            Jacobian::GENERIC_JACOBIAN {
                jacobian,
                sparsePattern,
                coloring,
                nonlinearPattern,
            } => {
                metamodelica::gc::MMTrace::mm_accept(jacobian, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sparsePattern, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(coloring, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nonlinearPattern, __mmv)?;
                Ok(())
            }
            Jacobian::EMPTY_JACOBIAN => Ok(()),
        }
    }
}
impl Jacobian {
    pub fn interned_EMPTY_JACOBIAN() -> metamodelica::Ref<Jacobian> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Jacobian> = metamodelica::Ref::new(Jacobian::EMPTY_JACOBIAN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_EMPTY_JACOBIAN() -> metamodelica::Ref<Jacobian> {
    Jacobian::interned_EMPTY_JACOBIAN()
}
impl Default for Jacobian {
    fn default() -> Self {
        Self::EMPTY_JACOBIAN
    }
}
pub use self::Jacobian::{EMPTY_JACOBIAN, FULL_JACOBIAN, GENERIC_JACOBIAN};

pub type SymbolicJacobians = metamodelica::List<(
    Option<(
        metamodelica::Ref<BackendDAE>,
        ArcStr,
        metamodelica::List<metamodelica::Ref<Var>>,
        metamodelica::List<metamodelica::Ref<Var>>,
        metamodelica::List<metamodelica::Ref<Var>>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<(
            metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        ),
        i32,
    ),
    metamodelica::List<metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>>,
    (
        metamodelica::List<(
            metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
            metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        ),
        i32,
    ),
)>;

pub type SymbolicJacobian = (
    metamodelica::Ref<BackendDAE>,
    ArcStr,
    metamodelica::List<metamodelica::Ref<Var>>,
    metamodelica::List<metamodelica::Ref<Var>>,
    metamodelica::List<metamodelica::Ref<Var>>,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
);

// symbolic equation system
// Matrix name
// diff vars (independent vars)
// diffed vars (residual vars)
// all diffed vars (residual vars + dependent vars)
// original dependent variables
pub type SparsePatternCref = (
    metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
);

pub type SparsePatternCrefs = metamodelica::List<(
    metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
)>;

pub type NonlinearPatternCref = (
    metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
);

pub type NonlinearPatternCrefs = metamodelica::List<(
    metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
)>;

pub type SparsePattern = (
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    ),
    i32,
);

// column-wise sparse pattern
// row-wise sparse pattern
// diff vars (independent vars) of associated jacobian
// diffed vars (residual vars) of associated jacobian
// nonZeroElements
pub type NonlinearPattern = (
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    ),
    i32,
);

thread_local! { static __emptySparsePattern_TLS: (metamodelica::List<(metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>)>, metamodelica::List<(metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>)>, (metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>), i32) = (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0); }
pub fn emptySparsePattern() -> (
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    ),
    i32,
) {
    __emptySparsePattern_TLS.with(|__t| __t.clone())
}

thread_local! { static __emptyNonlinearPattern_TLS: (metamodelica::List<(metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>)>, metamodelica::List<(metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>)>, (metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>), i32) = (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0); }
pub fn emptyNonlinearPattern() -> (
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    metamodelica::List<(
        metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    )>,
    (
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    ),
    i32,
) {
    __emptyNonlinearPattern_TLS.with(|__t| __t.clone())
}

pub type SparseColoring =
    metamodelica::List<metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>>;

// colouring
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DifferentiateInputData {
    pub independenentVars: Option<Variables>,
    pub dependenentVars: Option<Variables>,
    pub knownVars: Option<Variables>,
    pub allVars: Option<Variables>,
    pub controlVars: metamodelica::List<metamodelica::Ref<Var>>,
    pub diffCrefs: metamodelica::List<metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>>,
    pub matrixName: Option<ArcStr>,
    pub diffedFunctions: metamodelica::Ref<AvlSetPath::Tree>,
}

impl metamodelica::gc::MMTrace for DifferentiateInputData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.independenentVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dependenentVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.knownVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.allVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.controlVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.diffCrefs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matrixName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.diffedFunctions, __mmv)?;
        Ok(())
    }
}
impl Default for DifferentiateInputData {
    fn default() -> Self {
        Self {
            independenentVars: Default::default(),
            dependenentVars: Default::default(),
            knownVars: Default::default(),
            allVars: Default::default(),
            controlVars: Default::default(),
            diffCrefs: Default::default(),
            matrixName: Default::default(),
            diffedFunctions: Default::default(),
        }
    }
}

pub type DIFFINPUTDATA = DifferentiateInputData;

thread_local! { static __emptyInputData_TLS: DifferentiateInputData = DifferentiateInputData { independenentVars: None, dependenentVars: None, knownVars: None, allVars: None, controlVars: metamodelica::nil(), diffCrefs: metamodelica::nil(), matrixName: None, diffedFunctions: openmodelica_ast_collections::AvlSetPath::Tree::interned_EMPTY() }; }
pub fn emptyInputData() -> DifferentiateInputData {
    __emptyInputData_TLS.with(|__t| __t.clone())
}

pub type DifferentiateInputArguments = (
    metamodelica::Ref<openmodelica_frontend_types::DAE::ComponentRef>,
    DifferentiateInputData,
    DifferentiationType,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
);

/// Define the behaviour of differentiation method for (e.g. index reduction, ...)
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum DifferentiationType {
    /// Used for index reduction differentiation w.r.t. time (e.g. create dummy derivative variables)
    DIFFERENTIATION_TIME,
    /// Used to solve expression for a cref or by the older Jacobian generation, differentiation w.r.t. a given cref
    SIMPLE_DIFFERENTIATION,
    /// Used to differentiate a function call w.r.t. a given cref, which need to expand the input arguments
    ///                                  by differentiate arguments.
    DIFFERENTIATION_FUNCTION,
    /// Used to generate a full Jacobian matrix
    DIFF_FULL_JACOBIAN,
    /// Used to generate a generic gradient for generation the Jacobian matrix while the runtime.
    GENERIC_GRADIENT {
        /// true if computing for dae mode
        daeMode: bool,
    },
}
impl metamodelica::gc::MMTrace for DifferentiationType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            DifferentiationType::DIFFERENTIATION_TIME => Ok(()),
            DifferentiationType::SIMPLE_DIFFERENTIATION => Ok(()),
            DifferentiationType::DIFFERENTIATION_FUNCTION => Ok(()),
            DifferentiationType::DIFF_FULL_JACOBIAN => Ok(()),
            DifferentiationType::GENERIC_GRADIENT { daeMode } => {
                metamodelica::gc::MMTrace::mm_accept(daeMode, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for DifferentiationType {
    fn default() -> Self {
        Self::DIFFERENTIATION_TIME
    }
}
pub use self::DifferentiationType::{
    DIFF_FULL_JACOBIAN, DIFFERENTIATION_FUNCTION, DIFFERENTIATION_TIME, GENERIC_GRADIENT, SIMPLE_DIFFERENTIATION,
};

/// types to count operations for the components
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum CompInfo {
    COUNTER {
        comp: metamodelica::Ref<StrongComponent>,
        numAdds: i32,
        numMul: i32,
        numDiv: i32,
        numTrig: i32,
        numRelations: i32,
        numLog: i32,
        numOth: i32,
        funcCalls: i32,
    },
    SYSTEM {
        comp: metamodelica::Ref<StrongComponent>,
        allOperations: metamodelica::Ref<CompInfo>,
        size: i32,
        density: metamodelica::Real,
    },
    TORN_ANALYSE {
        comp: metamodelica::Ref<StrongComponent>,
        tornEqs: metamodelica::Ref<CompInfo>,
        otherEqs: metamodelica::Ref<CompInfo>,
        tornSize: i32,
    },
    NO_COMP {
        numAdds: i32,
        numMul: i32,
        numDiv: i32,
        numTrig: i32,
        numRelations: i32,
        numLog: i32,
        numOth: i32,
        funcCalls: i32,
    },
}
impl metamodelica::gc::MMTrace for CompInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            CompInfo::COUNTER {
                comp,
                numAdds,
                numMul,
                numDiv,
                numTrig,
                numRelations,
                numLog,
                numOth,
                funcCalls,
            } => {
                metamodelica::gc::MMTrace::mm_accept(comp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numAdds, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numMul, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numDiv, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numTrig, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numRelations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numLog, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numOth, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(funcCalls, __mmv)?;
                Ok(())
            }
            CompInfo::SYSTEM {
                comp,
                allOperations,
                size,
                density,
            } => {
                metamodelica::gc::MMTrace::mm_accept(comp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(allOperations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(density, __mmv)?;
                Ok(())
            }
            CompInfo::TORN_ANALYSE {
                comp,
                tornEqs,
                otherEqs,
                tornSize,
            } => {
                metamodelica::gc::MMTrace::mm_accept(comp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tornEqs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(otherEqs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tornSize, __mmv)?;
                Ok(())
            }
            CompInfo::NO_COMP {
                numAdds,
                numMul,
                numDiv,
                numTrig,
                numRelations,
                numLog,
                numOth,
                funcCalls,
            } => {
                metamodelica::gc::MMTrace::mm_accept(numAdds, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numMul, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numDiv, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numTrig, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numRelations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numLog, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numOth, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(funcCalls, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for CompInfo {
    fn default() -> Self {
        Self::NO_COMP {
            numAdds: Default::default(),
            numMul: Default::default(),
            numDiv: Default::default(),
            numTrig: Default::default(),
            numRelations: Default::default(),
            numLog: Default::default(),
            numOth: Default::default(),
            funcCalls: Default::default(),
        }
    }
}
pub use self::CompInfo::{COUNTER, NO_COMP, SYSTEM, TORN_ANALYSE};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct BackendDAEModeData {
    pub stateVars: metamodelica::List<metamodelica::Ref<Var>>,
    pub algStateVars: metamodelica::List<metamodelica::Ref<Var>>,
    pub numResVars: i32,
    pub modelVars: Option<Variables>,
}

impl metamodelica::gc::MMTrace for BackendDAEModeData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.stateVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.algStateVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numResVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.modelVars, __mmv)?;
        Ok(())
    }
}
impl Default for BackendDAEModeData {
    fn default() -> Self {
        Self {
            stateVars: Default::default(),
            algStateVars: Default::default(),
            numResVars: Default::default(),
            modelVars: Default::default(),
        }
    }
}

pub type BDAE_MODE_DATA = BackendDAEModeData;

thread_local! { static __emptyDAEModeData_TLS: BackendDAEModeData = BackendDAEModeData { stateVars: metamodelica::nil(), algStateVars: metamodelica::nil(), numResVars: 0, modelVars: None }; }
pub fn emptyDAEModeData() -> BackendDAEModeData {
    __emptyDAEModeData_TLS.with(|__t| __t.clone())
}
