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

use crate::AvlTreeCRToInt;
use crate::HashTableCrefSimVar;
use crate::HpcOmSimCode;
use crate::SimCodeFunction;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;

// public imports
pub type ExtConstructor = (
    metamodelica::Ref<DAE::ComponentRef>,
    ArcStr,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
);

pub type ExtDestructor = (ArcStr, metamodelica::Ref<DAE::ComponentRef>);

pub type ExtAlias = (
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
);

pub type SparsityPattern = metamodelica::List<(i32, metamodelica::List<i32>)>;

pub type NonlinearPattern = metamodelica::List<(i32, metamodelica::List<i32>)>;

// same structure but different name for the sake of maintenance
/// the dependency kind to show how a component reference occurs in an equation.
///  for each dimension there has to be one dependency kind.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Dependency {
    pub skips: metamodelica::Array<metamodelica::List<i32>>,
    /// true = reduced, false = regular
    pub kinds: metamodelica::List<bool>,
}

impl metamodelica::gc::MMTrace for Dependency {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.skips, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.kinds, __mmv)?;
        Ok(())
    }
}
impl Default for Dependency {
    fn default() -> Self {
        Self {
            skips: Default::default(),
            kinds: Default::default(),
        }
    }
}

pub type DEPENDENCY = Dependency;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SparsityRow {
    /// only for debugging
    pub equation_name: metamodelica::Ref<DAE::ComponentRef>,
    pub equation_iterators: metamodelica::List<BackendDAE::SimIterator>,
    pub dependencies: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, Dependency, bool)>,
    pub solved_crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
}

impl metamodelica::gc::MMTrace for SparsityRow {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.equation_name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.equation_iterators, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dependencies, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.solved_crefs, __mmv)?;
        Ok(())
    }
}
impl Default for SparsityRow {
    fn default() -> Self {
        Self {
            equation_name: Default::default(),
            equation_iterators: Default::default(),
            dependencies: Default::default(),
            solved_crefs: Default::default(),
        }
    }
}

pub type SPARSITY_ROW = SparsityRow;

/// the new resizable sparsity pattern for the NB
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Sparsity {
    SPARSITY { rows: metamodelica::List<SparsityRow> },
    EMPTY,
}
impl metamodelica::gc::MMTrace for Sparsity {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Sparsity::SPARSITY { rows } => {
                metamodelica::gc::MMTrace::mm_accept(rows, __mmv)?;
                Ok(())
            }
            Sparsity::EMPTY => Ok(()),
        }
    }
}
impl Default for Sparsity {
    fn default() -> Self {
        Self::EMPTY
    }
}
pub use self::Sparsity::{EMPTY, SPARSITY};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct JacobianColumn {
    pub columnEqns: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub columnVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub numberOfResultVars: i32,
    pub constantEqns: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
}

impl metamodelica::gc::MMTrace for JacobianColumn {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.columnEqns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.columnVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfResultVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.constantEqns, __mmv)?;
        Ok(())
    }
}
impl Default for JacobianColumn {
    fn default() -> Self {
        Self {
            columnEqns: Default::default(),
            columnVars: Default::default(),
            numberOfResultVars: Default::default(),
            constantEqns: Default::default(),
        }
    }
}

pub type JAC_COLUMN = JacobianColumn;

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct JacobianMatrix {
    pub columns: metamodelica::List<metamodelica::Ref<JacobianColumn>>,
    pub seedVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub matrixName: ArcStr,
    pub sparsityMatrix: Sparsity,
    pub sparsity: SparsityPattern,
    pub sparsityT: SparsityPattern,
    pub nonlinear: NonlinearPattern,
    pub nonlinearT: NonlinearPattern,
    pub coloredCols: metamodelica::List<metamodelica::List<i32>>,
    pub coloredRows: metamodelica::List<metamodelica::List<i32>>,
    pub maxColorCols: i32,
    pub jacobianIndex: i32,
    pub partitionIndex: i32,
    pub generic_loop_calls: metamodelica::List<SimGenericCall>,
    pub crefsHT: Option<(
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
    )>,
    pub isAdjoint: bool,
    pub isBidirectional: bool,
    pub adjointJacobianIndex: i32,
    pub adjointMatrixName: ArcStr,
}

impl metamodelica::gc::MMTrace for JacobianMatrix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.columns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.seedVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matrixName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sparsityMatrix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sparsity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sparsityT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nonlinear, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nonlinearT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.coloredCols, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.coloredRows, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.maxColorCols, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partitionIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generic_loop_calls, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crefsHT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isAdjoint, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isBidirectional, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.adjointJacobianIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.adjointMatrixName, __mmv)?;
        Ok(())
    }
}
impl PartialEq for JacobianMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.columns == other.columns
            && self.seedVars == other.seedVars
            && self.matrixName == other.matrixName
            && self.sparsityMatrix == other.sparsityMatrix
            && self.sparsity == other.sparsity
            && self.sparsityT == other.sparsityT
            && self.nonlinear == other.nonlinear
            && self.nonlinearT == other.nonlinearT
            && self.coloredCols == other.coloredCols
            && self.coloredRows == other.coloredRows
            && self.maxColorCols == other.maxColorCols
            && self.jacobianIndex == other.jacobianIndex
            && self.partitionIndex == other.partitionIndex
            && self.generic_loop_calls == other.generic_loop_calls
            && (match ((&self.crefsHT), (&other.crefsHT)) {
                (Some(__lo), Some(__ro)) => {
                    (match (__lo, __ro) {
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
                (None, None) => true,
                _ => false,
            })
            && self.isAdjoint == other.isAdjoint
            && self.isBidirectional == other.isBidirectional
            && self.adjointJacobianIndex == other.adjointJacobianIndex
            && self.adjointMatrixName == other.adjointMatrixName
    }
}
impl Eq for JacobianMatrix {}
impl PartialOrd for JacobianMatrix {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for JacobianMatrix {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.columns
            .cmp(&other.columns)
            .then_with(|| self.seedVars.cmp(&other.seedVars))
            .then_with(|| self.matrixName.cmp(&other.matrixName))
            .then_with(|| self.sparsityMatrix.cmp(&other.sparsityMatrix))
            .then_with(|| self.sparsity.cmp(&other.sparsity))
            .then_with(|| self.sparsityT.cmp(&other.sparsityT))
            .then_with(|| self.nonlinear.cmp(&other.nonlinear))
            .then_with(|| self.nonlinearT.cmp(&other.nonlinearT))
            .then_with(|| self.coloredCols.cmp(&other.coloredCols))
            .then_with(|| self.coloredRows.cmp(&other.coloredRows))
            .then_with(|| self.maxColorCols.cmp(&other.maxColorCols))
            .then_with(|| self.jacobianIndex.cmp(&other.jacobianIndex))
            .then_with(|| self.partitionIndex.cmp(&other.partitionIndex))
            .then_with(|| self.generic_loop_calls.cmp(&other.generic_loop_calls))
            .then_with(|| {
                (match ((&self.crefsHT), (&other.crefsHT)) {
                    (Some(__lo), Some(__ro)) => {
                        (match (__lo, __ro) {
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
                    }
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                })
            })
            .then_with(|| self.isAdjoint.cmp(&other.isAdjoint))
            .then_with(|| self.isBidirectional.cmp(&other.isBidirectional))
            .then_with(|| self.adjointJacobianIndex.cmp(&other.adjointJacobianIndex))
            .then_with(|| self.adjointMatrixName.cmp(&other.adjointMatrixName))
    }
}
impl std::fmt::Debug for JacobianMatrix {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("JacobianMatrix");
        __ds.field("columns", &self.columns);
        __ds.field("seedVars", &self.seedVars);
        __ds.field("matrixName", &self.matrixName);
        __ds.field("sparsityMatrix", &self.sparsityMatrix);
        __ds.field("sparsity", &self.sparsity);
        __ds.field("sparsityT", &self.sparsityT);
        __ds.field("nonlinear", &self.nonlinear);
        __ds.field("nonlinearT", &self.nonlinearT);
        __ds.field("coloredCols", &self.coloredCols);
        __ds.field("coloredRows", &self.coloredRows);
        __ds.field("maxColorCols", &self.maxColorCols);
        __ds.field("jacobianIndex", &self.jacobianIndex);
        __ds.field("partitionIndex", &self.partitionIndex);
        __ds.field("generic_loop_calls", &self.generic_loop_calls);
        __ds.field(
            "crefsHT",
            &format_args!("<dyn-fn-container@{:p}>", (&self.crefsHT) as *const _),
        );
        __ds.field("isAdjoint", &self.isAdjoint);
        __ds.field("isBidirectional", &self.isBidirectional);
        __ds.field("adjointJacobianIndex", &self.adjointJacobianIndex);
        __ds.field("adjointMatrixName", &self.adjointMatrixName);
        __ds.finish()
    }
}

impl Default for JacobianMatrix {
    fn default() -> Self {
        Self {
            columns: Default::default(),
            seedVars: Default::default(),
            matrixName: Default::default(),
            sparsityMatrix: Default::default(),
            sparsity: Default::default(),
            sparsityT: Default::default(),
            nonlinear: Default::default(),
            nonlinearT: Default::default(),
            coloredCols: Default::default(),
            coloredRows: Default::default(),
            maxColorCols: Default::default(),
            jacobianIndex: Default::default(),
            partitionIndex: Default::default(),
            generic_loop_calls: Default::default(),
            crefsHT: None,
            isAdjoint: Default::default(),
            isBidirectional: Default::default(),
            adjointJacobianIndex: Default::default(),
            adjointMatrixName: Default::default(),
        }
    }
}

pub type JAC_MATRIX = JacobianMatrix;

thread_local! { static __emptyJacobian_TLS: metamodelica::Ref<JacobianMatrix> = metamodelica::Ref::new(JacobianMatrix { columns: metamodelica::nil(), seedVars: metamodelica::nil(), matrixName: literal!(""), sparsityMatrix: crate::SimCode::Sparsity::EMPTY, sparsity: metamodelica::nil(), sparsityT: metamodelica::nil(), nonlinear: metamodelica::nil(), nonlinearT: metamodelica::nil(), coloredCols: metamodelica::nil(), coloredRows: metamodelica::nil(), maxColorCols: 0, jacobianIndex: -1, partitionIndex: 0, generic_loop_calls: metamodelica::nil(), crefsHT: None, isAdjoint: false, isBidirectional: false, adjointJacobianIndex: -1, adjointMatrixName: literal!("") }); }
pub fn emptyJacobian() -> metamodelica::Ref<JacobianMatrix> {
    __emptyJacobian_TLS.with(|__t| __t.clone())
}

pub static emptyPartitionData: std::sync::LazyLock<PartitionData> = std::sync::LazyLock::new(|| PartitionData {
    numPartitions: -1,
    partitions: metamodelica::nil(),
    activatorsForPartitions: metamodelica::nil(),
    stateToActivators: metamodelica::nil(),
});

/// Root data structure containing information required for templates to
///  generate simulation code for a Modelica model.
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct SimCode {
    pub modelInfo: ModelInfo,
    /// shared literals
    pub literals: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    pub recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>,
    pub externalFunctionIncludes: metamodelica::List<ArcStr>,
    pub generic_loop_calls: metamodelica::List<SimGenericCall>,
    /// state and input dependent variables, that are not inserted into any partion
    pub localKnownVars: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub allEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub odeEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimEqSystem>>>,
    pub algebraicEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimEqSystem>>>,
    pub clockedPartitions: metamodelica::List<ClockedPartition>,
    pub initialEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub initialEquations_lambda0: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub removedInitialEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub startValueEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub nominalValueEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub minValueEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub maxValueEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub parameterEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub removedEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub algorithmAndEquationAsserts: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub equationsForZeroCrossings: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub jacobianEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub stateSets: metamodelica::List<StateSet>,
    pub constraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    pub classAttributes: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>,
    pub zeroCrossings: metamodelica::List<BackendDAE::ZeroCrossing>,
    /// only used by c runtime
    pub relations: metamodelica::List<BackendDAE::ZeroCrossing>,
    /// only used by c runtime yet
    pub timeEvents: metamodelica::List<BackendDAE::TimeEvent>,
    pub discreteModelVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    pub extObjInfo: ExtObjInfo,
    pub makefileParams: SimCodeFunction::MakefileParams,
    pub delayedExps: DelayedExpression,
    pub spatialInfo: SpatialDistributionInfo,
    pub jacobianMatrices: metamodelica::List<metamodelica::Ref<JacobianMatrix>>,
    pub simulationSettingsOpt: Option<SimulationSettings>,
    /// Prefix for all enerated C files. Usually the model name with dots replaced by underscores.
    pub fileNamePrefix: ArcStr,
    /// Used in FMI where files are generated in a special directory
    pub fullPathPrefix: ArcStr,
    /// Name of FMU file <fmuTargetName>.fmu
    pub fmuTargetName: ArcStr,
    pub hpcomData: HpcOmSimCode::HpcOmData,
    /// Used in FMI
    pub valueReferences: metamodelica::Ref<AvlTreeCRToInt::Tree>,
    pub varToArrayIndexMapping: (
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
    ),
    pub varToIndexMapping: (
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
    ),
    /// hidden from typeview - used by cref2simvar() for cref -> SIMVAR lookup available in templates.
    pub crefToSimVarHT: HashTableCrefToSimVar,
    /// map variables to clock indices
    pub crefToClockIndexHT: (
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
    ),
    pub backendMapping: Option<BackendMapping>,
    pub modelStructure: Option<FmiModelStructure>,
    pub fmiSimulationFlags: Option<FmiSimulationFlags>,
    pub partitionData: PartitionData,
    pub daeModeData: Option<DaeModeData>,
    pub inlineEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    /// used for OMSI to generate equations code
    pub omsiData: Option<OMSIData>,
    pub scalarized: bool,
    /// set by SimCodeUtil.addFMI3Figures for an FMI 3.0 export
    pub fmiFigures: metamodelica::List<FmiFigure>,
}

impl metamodelica::gc::MMTrace for SimCode {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.modelInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.literals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.recordDecls, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.externalFunctionIncludes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generic_loop_calls, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.localKnownVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.allEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.odeEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.algebraicEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.clockedPartitions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialEquations_lambda0, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.removedInitialEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.startValueEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nominalValueEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.minValueEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.maxValueEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.parameterEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.removedEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.algorithmAndEquationAsserts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.equationsForZeroCrossings, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stateSets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.constraints, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.classAttributes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.zeroCrossings, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.timeEvents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.discreteModelVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extObjInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.makefileParams, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.delayedExps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.spatialInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianMatrices, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.simulationSettingsOpt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fileNamePrefix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fullPathPrefix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmuTargetName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hpcomData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.valueReferences, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varToArrayIndexMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varToIndexMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crefToSimVarHT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crefToClockIndexHT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.backendMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.modelStructure, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiSimulationFlags, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partitionData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.daeModeData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inlineEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.omsiData, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scalarized, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiFigures, __mmv)?;
        Ok(())
    }
}
impl PartialEq for SimCode {
    fn eq(&self, other: &Self) -> bool {
        self.modelInfo == other.modelInfo
            && self.literals == other.literals
            && self.recordDecls == other.recordDecls
            && self.externalFunctionIncludes == other.externalFunctionIncludes
            && self.generic_loop_calls == other.generic_loop_calls
            && self.localKnownVars == other.localKnownVars
            && self.allEquations == other.allEquations
            && self.odeEquations == other.odeEquations
            && self.algebraicEquations == other.algebraicEquations
            && self.clockedPartitions == other.clockedPartitions
            && self.initialEquations == other.initialEquations
            && self.initialEquations_lambda0 == other.initialEquations_lambda0
            && self.removedInitialEquations == other.removedInitialEquations
            && self.startValueEquations == other.startValueEquations
            && self.nominalValueEquations == other.nominalValueEquations
            && self.minValueEquations == other.minValueEquations
            && self.maxValueEquations == other.maxValueEquations
            && self.parameterEquations == other.parameterEquations
            && self.removedEquations == other.removedEquations
            && self.algorithmAndEquationAsserts == other.algorithmAndEquationAsserts
            && self.equationsForZeroCrossings == other.equationsForZeroCrossings
            && self.jacobianEquations == other.jacobianEquations
            && self.stateSets == other.stateSets
            && self.constraints == other.constraints
            && self.classAttributes == other.classAttributes
            && self.zeroCrossings == other.zeroCrossings
            && self.relations == other.relations
            && self.timeEvents == other.timeEvents
            && self.discreteModelVars == other.discreteModelVars
            && self.extObjInfo == other.extObjInfo
            && self.makefileParams == other.makefileParams
            && self.delayedExps == other.delayedExps
            && self.spatialInfo == other.spatialInfo
            && self.jacobianMatrices == other.jacobianMatrices
            && self.simulationSettingsOpt == other.simulationSettingsOpt
            && self.fileNamePrefix == other.fileNamePrefix
            && self.fullPathPrefix == other.fullPathPrefix
            && self.fmuTargetName == other.fmuTargetName
            && self.hpcomData == other.hpcomData
            && self.valueReferences == other.valueReferences
            && (match ((&self.varToArrayIndexMapping), (&other.varToArrayIndexMapping)) {
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
            && (match ((&self.varToIndexMapping), (&other.varToIndexMapping)) {
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
            && (match ((&self.crefToSimVarHT), (&other.crefToSimVarHT)) {
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
            && (match ((&self.crefToClockIndexHT), (&other.crefToClockIndexHT)) {
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
            && self.backendMapping == other.backendMapping
            && self.modelStructure == other.modelStructure
            && self.fmiSimulationFlags == other.fmiSimulationFlags
            && self.partitionData == other.partitionData
            && self.daeModeData == other.daeModeData
            && self.inlineEquations == other.inlineEquations
            && self.omsiData == other.omsiData
            && self.scalarized == other.scalarized
            && self.fmiFigures == other.fmiFigures
    }
}
impl Eq for SimCode {}
impl PartialOrd for SimCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SimCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.modelInfo
            .cmp(&other.modelInfo)
            .then_with(|| self.literals.cmp(&other.literals))
            .then_with(|| self.recordDecls.cmp(&other.recordDecls))
            .then_with(|| self.externalFunctionIncludes.cmp(&other.externalFunctionIncludes))
            .then_with(|| self.generic_loop_calls.cmp(&other.generic_loop_calls))
            .then_with(|| self.localKnownVars.cmp(&other.localKnownVars))
            .then_with(|| self.allEquations.cmp(&other.allEquations))
            .then_with(|| self.odeEquations.cmp(&other.odeEquations))
            .then_with(|| self.algebraicEquations.cmp(&other.algebraicEquations))
            .then_with(|| self.clockedPartitions.cmp(&other.clockedPartitions))
            .then_with(|| self.initialEquations.cmp(&other.initialEquations))
            .then_with(|| self.initialEquations_lambda0.cmp(&other.initialEquations_lambda0))
            .then_with(|| self.removedInitialEquations.cmp(&other.removedInitialEquations))
            .then_with(|| self.startValueEquations.cmp(&other.startValueEquations))
            .then_with(|| self.nominalValueEquations.cmp(&other.nominalValueEquations))
            .then_with(|| self.minValueEquations.cmp(&other.minValueEquations))
            .then_with(|| self.maxValueEquations.cmp(&other.maxValueEquations))
            .then_with(|| self.parameterEquations.cmp(&other.parameterEquations))
            .then_with(|| self.removedEquations.cmp(&other.removedEquations))
            .then_with(|| self.algorithmAndEquationAsserts.cmp(&other.algorithmAndEquationAsserts))
            .then_with(|| self.equationsForZeroCrossings.cmp(&other.equationsForZeroCrossings))
            .then_with(|| self.jacobianEquations.cmp(&other.jacobianEquations))
            .then_with(|| self.stateSets.cmp(&other.stateSets))
            .then_with(|| self.constraints.cmp(&other.constraints))
            .then_with(|| self.classAttributes.cmp(&other.classAttributes))
            .then_with(|| self.zeroCrossings.cmp(&other.zeroCrossings))
            .then_with(|| self.relations.cmp(&other.relations))
            .then_with(|| self.timeEvents.cmp(&other.timeEvents))
            .then_with(|| self.discreteModelVars.cmp(&other.discreteModelVars))
            .then_with(|| self.extObjInfo.cmp(&other.extObjInfo))
            .then_with(|| self.makefileParams.cmp(&other.makefileParams))
            .then_with(|| self.delayedExps.cmp(&other.delayedExps))
            .then_with(|| self.spatialInfo.cmp(&other.spatialInfo))
            .then_with(|| self.jacobianMatrices.cmp(&other.jacobianMatrices))
            .then_with(|| self.simulationSettingsOpt.cmp(&other.simulationSettingsOpt))
            .then_with(|| self.fileNamePrefix.cmp(&other.fileNamePrefix))
            .then_with(|| self.fullPathPrefix.cmp(&other.fullPathPrefix))
            .then_with(|| self.fmuTargetName.cmp(&other.fmuTargetName))
            .then_with(|| self.hpcomData.cmp(&other.hpcomData))
            .then_with(|| self.valueReferences.cmp(&other.valueReferences))
            .then_with(|| {
                (match ((&self.varToArrayIndexMapping), (&other.varToArrayIndexMapping)) {
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
            })
            .then_with(|| {
                (match ((&self.varToIndexMapping), (&other.varToIndexMapping)) {
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
            })
            .then_with(|| {
                (match ((&self.crefToSimVarHT), (&other.crefToSimVarHT)) {
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
            })
            .then_with(|| {
                (match ((&self.crefToClockIndexHT), (&other.crefToClockIndexHT)) {
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
            })
            .then_with(|| self.backendMapping.cmp(&other.backendMapping))
            .then_with(|| self.modelStructure.cmp(&other.modelStructure))
            .then_with(|| self.fmiSimulationFlags.cmp(&other.fmiSimulationFlags))
            .then_with(|| self.partitionData.cmp(&other.partitionData))
            .then_with(|| self.daeModeData.cmp(&other.daeModeData))
            .then_with(|| self.inlineEquations.cmp(&other.inlineEquations))
            .then_with(|| self.omsiData.cmp(&other.omsiData))
            .then_with(|| self.scalarized.cmp(&other.scalarized))
            .then_with(|| self.fmiFigures.cmp(&other.fmiFigures))
    }
}
impl std::fmt::Debug for SimCode {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("SimCode");
        __ds.field("modelInfo", &self.modelInfo);
        __ds.field("literals", &self.literals);
        __ds.field("recordDecls", &self.recordDecls);
        __ds.field("externalFunctionIncludes", &self.externalFunctionIncludes);
        __ds.field("generic_loop_calls", &self.generic_loop_calls);
        __ds.field("localKnownVars", &self.localKnownVars);
        __ds.field("allEquations", &self.allEquations);
        __ds.field("odeEquations", &self.odeEquations);
        __ds.field("algebraicEquations", &self.algebraicEquations);
        __ds.field("clockedPartitions", &self.clockedPartitions);
        __ds.field("initialEquations", &self.initialEquations);
        __ds.field("initialEquations_lambda0", &self.initialEquations_lambda0);
        __ds.field("removedInitialEquations", &self.removedInitialEquations);
        __ds.field("startValueEquations", &self.startValueEquations);
        __ds.field("nominalValueEquations", &self.nominalValueEquations);
        __ds.field("minValueEquations", &self.minValueEquations);
        __ds.field("maxValueEquations", &self.maxValueEquations);
        __ds.field("parameterEquations", &self.parameterEquations);
        __ds.field("removedEquations", &self.removedEquations);
        __ds.field("algorithmAndEquationAsserts", &self.algorithmAndEquationAsserts);
        __ds.field("equationsForZeroCrossings", &self.equationsForZeroCrossings);
        __ds.field("jacobianEquations", &self.jacobianEquations);
        __ds.field("stateSets", &self.stateSets);
        __ds.field("constraints", &self.constraints);
        __ds.field("classAttributes", &self.classAttributes);
        __ds.field("zeroCrossings", &self.zeroCrossings);
        __ds.field("relations", &self.relations);
        __ds.field("timeEvents", &self.timeEvents);
        __ds.field("discreteModelVars", &self.discreteModelVars);
        __ds.field("extObjInfo", &self.extObjInfo);
        __ds.field("makefileParams", &self.makefileParams);
        __ds.field("delayedExps", &self.delayedExps);
        __ds.field("spatialInfo", &self.spatialInfo);
        __ds.field("jacobianMatrices", &self.jacobianMatrices);
        __ds.field("simulationSettingsOpt", &self.simulationSettingsOpt);
        __ds.field("fileNamePrefix", &self.fileNamePrefix);
        __ds.field("fullPathPrefix", &self.fullPathPrefix);
        __ds.field("fmuTargetName", &self.fmuTargetName);
        __ds.field("hpcomData", &self.hpcomData);
        __ds.field("valueReferences", &self.valueReferences);
        __ds.field(
            "varToArrayIndexMapping",
            &format_args!("<dyn-fn-container@{:p}>", (&self.varToArrayIndexMapping) as *const _),
        );
        __ds.field(
            "varToIndexMapping",
            &format_args!("<dyn-fn-container@{:p}>", (&self.varToIndexMapping) as *const _),
        );
        __ds.field(
            "crefToSimVarHT",
            &format_args!("<dyn-fn-container@{:p}>", (&self.crefToSimVarHT) as *const _),
        );
        __ds.field(
            "crefToClockIndexHT",
            &format_args!("<dyn-fn-container@{:p}>", (&self.crefToClockIndexHT) as *const _),
        );
        __ds.field("backendMapping", &self.backendMapping);
        __ds.field("modelStructure", &self.modelStructure);
        __ds.field("fmiSimulationFlags", &self.fmiSimulationFlags);
        __ds.field("partitionData", &self.partitionData);
        __ds.field("daeModeData", &self.daeModeData);
        __ds.field("inlineEquations", &self.inlineEquations);
        __ds.field("omsiData", &self.omsiData);
        __ds.field("scalarized", &self.scalarized);
        __ds.field("fmiFigures", &self.fmiFigures);
        __ds.finish()
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
            localKnownVars: Default::default(),
            allEquations: Default::default(),
            odeEquations: Default::default(),
            algebraicEquations: Default::default(),
            clockedPartitions: Default::default(),
            initialEquations: Default::default(),
            initialEquations_lambda0: Default::default(),
            removedInitialEquations: Default::default(),
            startValueEquations: Default::default(),
            nominalValueEquations: Default::default(),
            minValueEquations: Default::default(),
            maxValueEquations: Default::default(),
            parameterEquations: Default::default(),
            removedEquations: Default::default(),
            algorithmAndEquationAsserts: Default::default(),
            equationsForZeroCrossings: Default::default(),
            jacobianEquations: Default::default(),
            stateSets: Default::default(),
            constraints: Default::default(),
            classAttributes: Default::default(),
            zeroCrossings: Default::default(),
            relations: Default::default(),
            timeEvents: Default::default(),
            discreteModelVars: Default::default(),
            extObjInfo: Default::default(),
            makefileParams: Default::default(),
            delayedExps: Default::default(),
            spatialInfo: Default::default(),
            jacobianMatrices: Default::default(),
            simulationSettingsOpt: Default::default(),
            fileNamePrefix: Default::default(),
            fullPathPrefix: Default::default(),
            fmuTargetName: Default::default(),
            hpcomData: Default::default(),
            valueReferences: Default::default(),
            varToArrayIndexMapping: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTableCrIListArray::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrIListArray::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrIListArray::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrIListArray::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            varToIndexMapping: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTableCrILst::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrILst::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrILst::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrILst::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            crefToSimVarHT: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTableCrefSimVar::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrefSimVar::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrefSimVar::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTableCrefSimVar::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            crefToClockIndexHT: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTable::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            backendMapping: Default::default(),
            modelStructure: Default::default(),
            fmiSimulationFlags: Default::default(),
            partitionData: Default::default(),
            daeModeData: Default::default(),
            inlineEquations: Default::default(),
            omsiData: Default::default(),
            scalarized: Default::default(),
            fmiFigures: Default::default(),
        }
    }
}

pub type SIMCODE = SimCode;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ClockedPartition {
    pub baseClock: metamodelica::Ref<DAE::ClockKind>,
    pub subPartitions: metamodelica::List<SubPartition>,
}

impl metamodelica::gc::MMTrace for ClockedPartition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.baseClock, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.subPartitions, __mmv)?;
        Ok(())
    }
}
impl Default for ClockedPartition {
    fn default() -> Self {
        Self {
            baseClock: Default::default(),
            subPartitions: Default::default(),
        }
    }
}

pub type CLOCKED_PARTITION = ClockedPartition;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SubPartition {
    pub vars: metamodelica::List<(metamodelica::Ref<SimCodeVar::SimVar>, bool)>,
    pub equations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub removedEquations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub subClock: BackendDAE::SubClock,
    pub holdEvents: bool,
}

impl metamodelica::gc::MMTrace for SubPartition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.equations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.removedEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.subClock, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.holdEvents, __mmv)?;
        Ok(())
    }
}
impl Default for SubPartition {
    fn default() -> Self {
        Self {
            vars: Default::default(),
            equations: Default::default(),
            removedEquations: Default::default(),
            subClock: Default::default(),
            holdEvents: Default::default(),
        }
    }
}

pub type SUBPARTITION = SubPartition;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BackendMapping {
    BACKENDMAPPING {
        m: metamodelica::Array<metamodelica::List<i32>>,
        mT: metamodelica::Array<metamodelica::List<i32>>,
        eqMapping: metamodelica::List<(i32, metamodelica::List<i32>)>,
        varMapping: metamodelica::List<(i32, i32)>,
        eqMatch: metamodelica::Array<i32>,
        varMatch: metamodelica::Array<i32>,
        eqTree: metamodelica::Array<metamodelica::List<i32>>,
        simVarMapping: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>,
    },
    NO_MAPPING,
}
impl metamodelica::gc::MMTrace for BackendMapping {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BackendMapping::BACKENDMAPPING {
                m,
                mT,
                eqMapping,
                varMapping,
                eqMatch,
                varMatch,
                eqTree,
                simVarMapping,
            } => {
                metamodelica::gc::MMTrace::mm_accept(m, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mT, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqMapping, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varMapping, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqMatch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varMatch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqTree, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(simVarMapping, __mmv)?;
                Ok(())
            }
            BackendMapping::NO_MAPPING => Ok(()),
        }
    }
}
impl Default for BackendMapping {
    fn default() -> Self {
        Self::NO_MAPPING
    }
}
pub use self::BackendMapping::{BACKENDMAPPING, NO_MAPPING};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct PartitionData {
    pub numPartitions: i32,
    pub partitions: metamodelica::List<metamodelica::List<i32>>,
    pub activatorsForPartitions: metamodelica::List<metamodelica::List<i32>>,
    pub stateToActivators: metamodelica::List<i32>,
}

impl metamodelica::gc::MMTrace for PartitionData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numPartitions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partitions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.activatorsForPartitions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stateToActivators, __mmv)?;
        Ok(())
    }
}
impl Default for PartitionData {
    fn default() -> Self {
        Self {
            numPartitions: Default::default(),
            partitions: Default::default(),
            activatorsForPartitions: Default::default(),
            stateToActivators: Default::default(),
        }
    }
}

pub type PARTITIONDATA = PartitionData;

/// Delayed expressions type
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DelayedExpression {
    pub delayedExps: metamodelica::List<(
        i32,
        (
            metamodelica::Ref<DAE::Exp>,
            metamodelica::Ref<DAE::Exp>,
            metamodelica::Ref<DAE::Exp>,
        ),
    )>,
    pub maxDelayedIndex: i32,
}

impl metamodelica::gc::MMTrace for DelayedExpression {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.delayedExps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.maxDelayedIndex, __mmv)?;
        Ok(())
    }
}
impl Default for DelayedExpression {
    fn default() -> Self {
        Self {
            delayedExps: Default::default(),
            maxDelayedIndex: Default::default(),
        }
    }
}

pub type DELAYED_EXPRESSIONS = DelayedExpression;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SpatialDistributionInfo {
    pub spatialDistributions: metamodelica::List<SpatialDistribution>,
    pub maxIndex: i32,
}

impl metamodelica::gc::MMTrace for SpatialDistributionInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.spatialDistributions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.maxIndex, __mmv)?;
        Ok(())
    }
}
impl Default for SpatialDistributionInfo {
    fn default() -> Self {
        Self {
            spatialDistributions: Default::default(),
            maxIndex: Default::default(),
        }
    }
}

pub type SPATIAL_DISTRIBUTION_INFO = SpatialDistributionInfo;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SpatialDistribution {
    /// uniqueIndex
    pub index: i32,
    /// input 0
    pub in0: metamodelica::Ref<DAE::Exp>,
    /// input 1
    pub in1: metamodelica::Ref<DAE::Exp>,
    /// current pos
    pub pos: metamodelica::Ref<DAE::Exp>,
    /// flow direction
    pub dir: metamodelica::Ref<DAE::Exp>,
    /// initial grid points
    pub initPnts: metamodelica::Ref<DAE::Exp>,
    /// initial grid values
    pub initVals: metamodelica::Ref<DAE::Exp>,
    /// number of initial points
    pub initSize: i32,
    /// guard condition of the enclosing if-branch, if any
    pub condition: Option<metamodelica::Ref<DAE::Exp>>,
}

impl metamodelica::gc::MMTrace for SpatialDistribution {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.in0, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.in1, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pos, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dir, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initPnts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initVals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initSize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
        Ok(())
    }
}
impl Default for SpatialDistribution {
    fn default() -> Self {
        Self {
            index: Default::default(),
            in0: Default::default(),
            in1: Default::default(),
            pos: Default::default(),
            dir: Default::default(),
            initPnts: Default::default(),
            initVals: Default::default(),
            initSize: Default::default(),
            condition: Default::default(),
        }
    }
}

pub type SPATIAL_DISTRIBUTION = SpatialDistribution;

/// unitDefinitions for fmi modelDescription.xml
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct UnitDefinition {
    pub name: ArcStr,
    pub baseUnit: BaseUnit,
}

impl metamodelica::gc::MMTrace for UnitDefinition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.baseUnit, __mmv)?;
        Ok(())
    }
}
impl Default for UnitDefinition {
    fn default() -> Self {
        Self {
            name: Default::default(),
            baseUnit: Default::default(),
        }
    }
}

pub type UNITDEFINITION = UnitDefinition;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum BaseUnit {
    BASEUNIT {
        /// exponent
        s: i32,
        /// exponent
        m: i32,
        /// exponent
        kg: i32,
        /// exponent
        A: i32,
        /// exponent
        K: i32,
        /// exponent
        mol: i32,
        /// exponent
        cd: i32,
        /// prefix
        factor: metamodelica::Real,
        /// offset
        offset: metamodelica::Real,
    },
    /// no baseunit definition available
    NOBASEUNIT,
}
impl metamodelica::gc::MMTrace for BaseUnit {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            BaseUnit::BASEUNIT {
                s,
                m,
                kg,
                A,
                K,
                mol,
                cd,
                factor,
                offset,
            } => {
                metamodelica::gc::MMTrace::mm_accept(s, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(m, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(kg, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(A, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(K, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mol, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cd, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(factor, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(offset, __mmv)?;
                Ok(())
            }
            BaseUnit::NOBASEUNIT => Ok(()),
        }
    }
}
impl Default for BaseUnit {
    fn default() -> Self {
        Self::NOBASEUNIT
    }
}
pub use self::BaseUnit::{BASEUNIT, NOBASEUNIT};

/// Container for metadata about a Modelica model.
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
    pub varInfo: VarInfo,
    pub vars: SimCodeVar::SimVars,
    pub functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    pub labels: metamodelica::List<ArcStr>,
    /// Paths of all resources used by the model. Used in FMI2 to package resources in the FMU.
    pub resourcePaths: metamodelica::List<ArcStr>,
    pub sortedClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>>,
    pub nClocks: i32,
    pub nSubClocks: i32,
    pub nSpatialDistributions: i32,
    pub hasLargeLinearEquationSystems: bool,
    pub linearSystems: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub nonLinearSystems: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    /// export unitDefintion in modelDescription.xml
    pub unitDefinitions: metamodelica::List<UnitDefinition>,
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
        metamodelica::gc::MMTrace::mm_accept(&self.varInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.functions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.labels, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.resourcePaths, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sortedClasses, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nClocks, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nSubClocks, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nSpatialDistributions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hasLargeLinearEquationSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.linearSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nonLinearSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.unitDefinitions, __mmv)?;
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
            varInfo: Default::default(),
            vars: Default::default(),
            functions: Default::default(),
            labels: Default::default(),
            resourcePaths: Default::default(),
            sortedClasses: Default::default(),
            nClocks: Default::default(),
            nSubClocks: Default::default(),
            nSpatialDistributions: Default::default(),
            hasLargeLinearEquationSystems: Default::default(),
            linearSystems: Default::default(),
            nonLinearSystems: Default::default(),
            unitDefinitions: Default::default(),
        }
    }
}

pub type MODELINFO = ModelInfo;

pub type Files = metamodelica::List<FileInfo>;

/// contains all the .mo files present in all SourceInfo and DAE.ElementSource.info
///   of all the variables, functions, etc from SimCode that have origin info.
///   it is used to generate the file information in one place and use an index
///   whenever we need to refer to one file from a var or function.
///   this is done so that we don't repeat long filenames everywhere.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FileInfo {
    /// fileName where the class/component is defined in
    pub fileName: ArcStr,
    /// isReadOnly : (true|false). Should be true for libraries
    pub isReadOnly: bool,
}

impl metamodelica::gc::MMTrace for FileInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isReadOnly, __mmv)?;
        Ok(())
    }
}
impl Default for FileInfo {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            isReadOnly: Default::default(),
        }
    }
}

pub type FILEINFO = FileInfo;

/// Number of variables of various types in a Modelica model.
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VarInfo {
    pub numZeroCrossings: i32,
    pub numTimeEvents: i32,
    pub numRelations: i32,
    pub numMathEventFunctions: i32,
    pub numStateVars: i32,
    pub numAlgVars: i32,
    pub numDiscreteReal: i32,
    pub numIntAlgVars: i32,
    pub numBoolAlgVars: i32,
    pub numAlgAliasVars: i32,
    pub numIntAliasVars: i32,
    pub numBoolAliasVars: i32,
    pub numParams: i32,
    pub numIntParams: i32,
    pub numBoolParams: i32,
    pub numOutVars: i32,
    pub numInVars: i32,
    pub numExternalObjects: i32,
    pub numStringAlgVars: i32,
    pub numStringParamVars: i32,
    pub numStringAliasVars: i32,
    pub numEquations: i32,
    pub numLinearSystems: i32,
    pub numNonLinearSystems: i32,
    pub numMixedSystems: i32,
    pub numStateSets: i32,
    pub numJacobians: i32,
    pub numOptimizeConstraints: i32,
    pub numOptimizeFinalConstraints: i32,
    pub numSensitivityParameters: i32,
    pub numSetcVars: i32,
    pub numDataReconVars: i32,
    /// for fmi cs to interpolate inputs
    pub numRealInputVars: i32,
    /// for data reconciliation setB vars
    pub numSetbVars: i32,
    /// for data reconciliation count number of boundary conditions which failed the extraction algorithm
    pub numRelatedBoundaryConditions: i32,
}

impl metamodelica::gc::MMTrace for VarInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numZeroCrossings, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numTimeEvents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numRelations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numMathEventFunctions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numStateVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numDiscreteReal, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numIntAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numBoolAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numAlgAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numIntAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numBoolAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numParams, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numIntParams, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numBoolParams, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numOutVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numInVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numExternalObjects, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numStringAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numStringParamVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numStringAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numLinearSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numNonLinearSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numMixedSystems, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numStateSets, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numJacobians, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numOptimizeConstraints, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numOptimizeFinalConstraints, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numSensitivityParameters, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numSetcVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numDataReconVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numRealInputVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numSetbVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numRelatedBoundaryConditions, __mmv)?;
        Ok(())
    }
}
impl Default for VarInfo {
    fn default() -> Self {
        Self {
            numZeroCrossings: Default::default(),
            numTimeEvents: Default::default(),
            numRelations: Default::default(),
            numMathEventFunctions: Default::default(),
            numStateVars: Default::default(),
            numAlgVars: Default::default(),
            numDiscreteReal: Default::default(),
            numIntAlgVars: Default::default(),
            numBoolAlgVars: Default::default(),
            numAlgAliasVars: Default::default(),
            numIntAliasVars: Default::default(),
            numBoolAliasVars: Default::default(),
            numParams: Default::default(),
            numIntParams: Default::default(),
            numBoolParams: Default::default(),
            numOutVars: Default::default(),
            numInVars: Default::default(),
            numExternalObjects: Default::default(),
            numStringAlgVars: Default::default(),
            numStringParamVars: Default::default(),
            numStringAliasVars: Default::default(),
            numEquations: Default::default(),
            numLinearSystems: Default::default(),
            numNonLinearSystems: Default::default(),
            numMixedSystems: Default::default(),
            numStateSets: Default::default(),
            numJacobians: Default::default(),
            numOptimizeConstraints: Default::default(),
            numOptimizeFinalConstraints: Default::default(),
            numSensitivityParameters: Default::default(),
            numSetcVars: Default::default(),
            numDataReconVars: Default::default(),
            numRealInputVars: Default::default(),
            numSetbVars: Default::default(),
            numRelatedBoundaryConditions: Default::default(),
        }
    }
}

pub type VARINFO = VarInfo;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum DaeModeConfig {
    ALL_EQUATIONS,
    DYNAMIC_EQUATIONS,
}
impl metamodelica::gc::MMTrace for DaeModeConfig {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            DaeModeConfig::ALL_EQUATIONS => Ok(()),
            DaeModeConfig::DYNAMIC_EQUATIONS => Ok(()),
        }
    }
}
impl Default for DaeModeConfig {
    fn default() -> Self {
        Self::ALL_EQUATIONS
    }
}
pub use self::DaeModeConfig::{ALL_EQUATIONS, DYNAMIC_EQUATIONS};

/// contains data that belongs to the dae mode
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DaeModeData {
    /// daeModel residuals equations
    pub daeEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimEqSystem>>>,
    /// contains the sparsity pattern for the daeMode
    pub sparsityPattern: Option<metamodelica::Ref<JacobianMatrix>>,
    /// variable used to calculate residuals of a DAE form, they are real
    pub residualVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub algebraicVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub auxiliaryVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub modeCreated: DaeModeConfig,
}

impl metamodelica::gc::MMTrace for DaeModeData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.daeEquations, __mmv)?;
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
            daeEquations: Default::default(),
            sparsityPattern: Default::default(),
            residualVars: Default::default(),
            algebraicVars: Default::default(),
            auxiliaryVars: Default::default(),
            modeCreated: Default::default(),
        }
    }
}

pub type DAEMODEDATA = DaeModeData;

/// contains data for code generation for OMSI
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct OMSIData {
    /// contains equations and variables for initialization problem
    pub initialization: metamodelica::Ref<OMSIFunction>,
    /// contains equations and variables for simulation problem
    pub simulation: metamodelica::Ref<OMSIFunction>,
}

impl metamodelica::gc::MMTrace for OMSIData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.initialization, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.simulation, __mmv)?;
        Ok(())
    }
}
impl Default for OMSIData {
    fn default() -> Self {
        Self {
            initialization: Default::default(),
            simulation: Default::default(),
        }
    }
}

pub type OMSI_DATA = OMSIData;

/// contains equations and variables for initialization or simulation problem
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct OMSIFunction {
    /// causalized list of single equations and systems of equations
    pub equations: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    /// list of simcode variables determining input variables for equation(s)
    pub inputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    /// list of simcode variables determining output variables for equation(s)
    pub outputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    /// list of simcode variables determining inner variables for equation(s), e.g $DER(x)
    pub innerVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    /// number of input, inner and output vars
    pub nAllVars: i32,
    /// contains crefToSimVar hash table for lookup function in templates
    pub context: SimCodeFunction::Context,
    /// number of linear and non-linear algebraic systems in OMSI_FUNCTION.equations
    pub nAlgebraicSystems: i32,
}

impl metamodelica::gc::MMTrace for OMSIFunction {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.equations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inputVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outputVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nAllVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.context, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nAlgebraicSystems, __mmv)?;
        Ok(())
    }
}
impl Default for OMSIFunction {
    fn default() -> Self {
        Self {
            equations: Default::default(),
            inputVars: Default::default(),
            outputVars: Default::default(),
            innerVars: Default::default(),
            nAllVars: Default::default(),
            context: Default::default(),
            nAlgebraicSystems: Default::default(),
        }
    }
}

pub type OMSI_FUNCTION = OMSIFunction;

thread_local! { static __emptyOMSIFunction_TLS: metamodelica::Ref<OMSIFunction> = metamodelica::Ref::new(OMSIFunction { equations: metamodelica::nil(), inputVars: metamodelica::nil(), outputVars: metamodelica::nil(), innerVars: metamodelica::nil(), nAllVars: 0, context: SimCodeFunction::contextOMSI().clone(), nAlgebraicSystems: 0 }); }
pub fn emptyOMSIFunction() -> metamodelica::Ref<OMSIFunction> {
    __emptyOMSIFunction_TLS.with(|__t| __t.clone())
}

/// Represents a single equation or a system of equations that must be solved together.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SimEqSystem {
    SES_RESIDUAL {
        index: i32,
        res_index: i32,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_RESIDUAL {
        index: i32,
        res_index: i32,
        iterators: metamodelica::List<BackendDAE::SimIterator>,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a generic residual calling a for loop body function with an index list.
    SES_GENERIC_RESIDUAL {
        index: i32,
        res_index: i32,
        scal_indices: metamodelica::List<i32>,
        iterators: metamodelica::List<BackendDAE::SimIterator>,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_SIMPLE_ASSIGN {
        index: i32,
        /// left hand side of equation
        cref: metamodelica::Ref<DAE::ComponentRef>,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// Solved inner equation of (casual) tearing set (Dynamic Tearing) with constraints on the solvability
    SES_SIMPLE_ASSIGN_CONSTRAINTS {
        index: i32,
        cref: metamodelica::Ref<DAE::ComponentRef>,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        cons: metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ARRAY_CALL_ASSIGN {
        index: i32,
        lhs: metamodelica::Ref<DAE::Exp>,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a resizable assignment calling a for loop body function.
    SES_RESIZABLE_ASSIGN {
        index: i32,
        call_index: i32,
        iters: metamodelica::List<BackendDAE::SimIterator>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// a generic assignment calling a for loop body function with an index list.
    SES_GENERIC_ASSIGN {
        index: i32,
        call_index: i32,
        scal_indices: metamodelica::List<i32>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    /// entwined generic assignments calling for loop body functions with an index list and a call order.
    SES_ENTWINED_ASSIGN {
        index: i32,
        call_order: metamodelica::List<i32>,
        single_calls: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_IFEQUATION {
        index: i32,
        ifbranches: metamodelica::List<(
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<SimEqSystem>>,
        )>,
        elsebranch: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ALGORITHM {
        index: i32,
        statements: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_INVERSE_ALGORITHM {
        index: i32,
        statements: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
        /// this is a subset of output crefs of the original algorithm, which are already known
        knownOutputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        insideNonLinearSystem: bool,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_LINEAR {
        lSystem: metamodelica::Ref<LinearSystem>,
        alternativeTearing: Option<metamodelica::Ref<LinearSystem>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_NONLINEAR {
        nlSystem: metamodelica::Ref<NonlinearSystem>,
        alternativeTearing: Option<metamodelica::Ref<NonlinearSystem>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_MIXED {
        index: i32,
        cont: metamodelica::Ref<SimEqSystem>,
        discVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
        discEqs: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
        indexMixedSystem: i32,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_WHEN {
        index: i32,
        /// list of boolean variables as conditions
        conditions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        /// true, if top-level branch with initial()
        initialCall: bool,
        whenStmtLst: metamodelica::List<BackendDAE::WhenOperator>,
        elseWhen: Option<metamodelica::Ref<SimEqSystem>>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_LOOP {
        index: i32,
        iter: metamodelica::Ref<DAE::Exp>,
        startIt: metamodelica::Ref<DAE::Exp>,
        endIt: metamodelica::Ref<DAE::Exp>,
        cref: metamodelica::Ref<DAE::ComponentRef>,
        exp: metamodelica::Ref<DAE::Exp>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_FOR_EQUATION {
        index: i32,
        iter: metamodelica::Ref<DAE::Exp>,
        startIt: metamodelica::Ref<DAE::Exp>,
        endIt: metamodelica::Ref<DAE::Exp>,
        body: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
        source: metamodelica::Ref<DAE::ElementSource>,
        eqAttr: BackendDAE::EquationAttributes,
    },
    SES_ALIAS {
        index: i32,
        aliasOf: i32,
    },
    SES_ALGEBRAIC_SYSTEM {
        /// equation index
        index: i32,
        /// index of algebraic system
        algSysIndex: i32,
        /// dimension of algebraic loop (after tearing)
        dim_n: i32,
        partOfMixed: bool,
        tornSystem: bool,
        linearSystem: bool,
        residual: metamodelica::Ref<OMSIFunction>,
        matrix: Option<metamodelica::Ref<DerivativeMatrix>>,
        zeroCrossingConditions: metamodelica::List<i32>,
        sources: metamodelica::List<metamodelica::Ref<DAE::ElementSource>>,
        eqAttr: BackendDAE::EquationAttributes,
    },
}
impl metamodelica::gc::MMTrace for SimEqSystem {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SimEqSystem::SES_RESIDUAL {
                index,
                res_index,
                exp,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_FOR_RESIDUAL {
                index,
                res_index,
                iterators,
                exp,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterators, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_GENERIC_RESIDUAL {
                index,
                res_index,
                scal_indices,
                iterators,
                exp,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scal_indices, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iterators, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_SIMPLE_ASSIGN {
                index,
                cref,
                exp,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS {
                index,
                cref,
                exp,
                source,
                cons,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cons, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_ARRAY_CALL_ASSIGN {
                index,
                lhs,
                exp,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_RESIZABLE_ASSIGN {
                index,
                call_index,
                iters,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(call_index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_GENERIC_ASSIGN {
                index,
                call_index,
                scal_indices,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(call_index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scal_indices, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_ENTWINED_ASSIGN {
                index,
                call_order,
                single_calls,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(call_order, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(single_calls, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_IFEQUATION {
                index,
                ifbranches,
                elsebranch,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ifbranches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elsebranch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_ALGORITHM {
                index,
                statements,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_INVERSE_ALGORITHM {
                index,
                statements,
                knownOutputCrefs,
                insideNonLinearSystem,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(statements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(knownOutputCrefs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(insideNonLinearSystem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_LINEAR {
                lSystem,
                alternativeTearing,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(lSystem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(alternativeTearing, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_NONLINEAR {
                nlSystem,
                alternativeTearing,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(nlSystem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(alternativeTearing, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_MIXED {
                index,
                cont,
                discVars,
                discEqs,
                indexMixedSystem,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cont, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(discVars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(discEqs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(indexMixedSystem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_WHEN {
                index,
                conditions,
                initialCall,
                whenStmtLst,
                elseWhen,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(conditions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(initialCall, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(whenStmtLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elseWhen, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_FOR_LOOP {
                index,
                iter,
                startIt,
                endIt,
                cref,
                exp,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startIt, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(endIt, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_FOR_EQUATION {
                index,
                iter,
                startIt,
                endIt,
                body,
                source,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startIt, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(endIt, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_ALIAS { index, aliasOf } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(aliasOf, __mmv)?;
                Ok(())
            }
            SimEqSystem::SES_ALGEBRAIC_SYSTEM {
                index,
                algSysIndex,
                dim_n,
                partOfMixed,
                tornSystem,
                linearSystem,
                residual,
                matrix,
                zeroCrossingConditions,
                sources,
                eqAttr,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algSysIndex, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dim_n, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(partOfMixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(tornSystem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(linearSystem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(residual, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matrix, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(zeroCrossingConditions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sources, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqAttr, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for SimEqSystem {
    fn default() -> Self {
        Self::SES_ALIAS {
            index: Default::default(),
            aliasOf: Default::default(),
        }
    }
}
pub use self::SimEqSystem::{
    SES_ALGEBRAIC_SYSTEM, SES_ALGORITHM, SES_ALIAS, SES_ARRAY_CALL_ASSIGN, SES_ENTWINED_ASSIGN, SES_FOR_EQUATION,
    SES_FOR_LOOP, SES_FOR_RESIDUAL, SES_GENERIC_ASSIGN, SES_GENERIC_RESIDUAL, SES_IFEQUATION, SES_INVERSE_ALGORITHM,
    SES_LINEAR, SES_MIXED, SES_NONLINEAR, SES_RESIDUAL, SES_RESIZABLE_ASSIGN, SES_SIMPLE_ASSIGN,
    SES_SIMPLE_ASSIGN_CONSTRAINTS, SES_WHEN,
};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SimGenericCall {
    SINGLE_GENERIC_CALL {
        index: i32,
        iters: metamodelica::List<BackendDAE::SimIterator>,
        lhs: metamodelica::Ref<DAE::Exp>,
        rhs: metamodelica::Ref<DAE::Exp>,
        resizable: bool,
    },
    IF_GENERIC_CALL {
        index: i32,
        iters: metamodelica::List<BackendDAE::SimIterator>,
        branches: metamodelica::List<SimBranch>,
        resizable: bool,
    },
    WHEN_GENERIC_CALL {
        index: i32,
        iters: metamodelica::List<BackendDAE::SimIterator>,
        branches: metamodelica::List<SimBranch>,
        resizable: bool,
    },
}
impl metamodelica::gc::MMTrace for SimGenericCall {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SimGenericCall::SINGLE_GENERIC_CALL {
                index,
                iters,
                lhs,
                rhs,
                resizable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resizable, __mmv)?;
                Ok(())
            }
            SimGenericCall::IF_GENERIC_CALL {
                index,
                iters,
                branches,
                resizable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resizable, __mmv)?;
                Ok(())
            }
            SimGenericCall::WHEN_GENERIC_CALL {
                index,
                iters,
                branches,
                resizable,
            } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(resizable, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for SimGenericCall {
    fn default() -> Self {
        Self::IF_GENERIC_CALL {
            index: Default::default(),
            iters: Default::default(),
            branches: Default::default(),
            resizable: Default::default(),
        }
    }
}
pub use self::SimGenericCall::{IF_GENERIC_CALL, SINGLE_GENERIC_CALL, WHEN_GENERIC_CALL};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum SimBranch {
    SIM_BRANCH {
        condition: Option<metamodelica::Ref<DAE::Exp>>,
        body: metamodelica::List<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>,
    },
    SIM_BRANCH_STMT {
        condition: Option<metamodelica::Ref<DAE::Exp>>,
        body: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    },
}
impl metamodelica::gc::MMTrace for SimBranch {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            SimBranch::SIM_BRANCH { condition, body } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                Ok(())
            }
            SimBranch::SIM_BRANCH_STMT { condition, body } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for SimBranch {
    fn default() -> Self {
        Self::SIM_BRANCH {
            condition: Default::default(),
            body: Default::default(),
        }
    }
}
pub use self::SimBranch::{SIM_BRANCH, SIM_BRANCH_STMT};

/// represents directional derivatives with sparsity and coloring
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct DerivativeMatrix {
    pub columns: metamodelica::List<metamodelica::Ref<OMSIFunction>>,
    /// unique matrix name
    pub matrixName: ArcStr,
    pub sparsity: SparsityPattern,
    pub sparsityT: SparsityPattern,
    pub coloredCols: metamodelica::List<metamodelica::List<i32>>,
    pub maxColorCols: i32,
}

impl metamodelica::gc::MMTrace for DerivativeMatrix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.columns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matrixName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sparsity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sparsityT, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.coloredCols, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.maxColorCols, __mmv)?;
        Ok(())
    }
}
pub type DERIVATIVE_MATRIX = DerivativeMatrix;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct LinearSystem {
    pub index: i32,
    pub partOfMixed: bool,
    pub tornSystem: bool,
    pub vars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub beqs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    pub simJac: metamodelica::List<(i32, i32, metamodelica::Ref<SimEqSystem>)>,
    pub residual: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub jacobianMatrix: Option<metamodelica::Ref<JacobianMatrix>>,
    pub sources: metamodelica::List<metamodelica::Ref<DAE::ElementSource>>,
    pub indexLinearSystem: i32,
    /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
    pub nUnknowns: i32,
    /// if TRUE then this system is part of a jacobian matrix
    pub partOfJac: bool,
}

impl metamodelica::gc::MMTrace for LinearSystem {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partOfMixed, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tornSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.beqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.simJac, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.residual, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianMatrix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sources, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.indexLinearSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nUnknowns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.partOfJac, __mmv)?;
        Ok(())
    }
}
impl Default for LinearSystem {
    fn default() -> Self {
        Self {
            index: Default::default(),
            partOfMixed: Default::default(),
            tornSystem: Default::default(),
            vars: Default::default(),
            beqs: Default::default(),
            simJac: Default::default(),
            residual: Default::default(),
            jacobianMatrix: Default::default(),
            sources: Default::default(),
            indexLinearSystem: Default::default(),
            nUnknowns: Default::default(),
            partOfJac: Default::default(),
        }
    }
}

pub type LINEARSYSTEM = LinearSystem;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NonlinearSystem {
    pub index: i32,
    pub eqs: metamodelica::List<metamodelica::Ref<SimEqSystem>>,
    pub crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    pub indexNonLinearSystem: i32,
    /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
    pub nUnknowns: i32,
    pub jacobianMatrix: Option<metamodelica::Ref<JacobianMatrix>>,
    pub homotopySupport: bool,
    pub mixedSystem: bool,
    pub tornSystem: bool,
    pub clockIndex: Option<i32>,
}

impl metamodelica::gc::MMTrace for NonlinearSystem {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crefs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.indexNonLinearSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nUnknowns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianMatrix, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.homotopySupport, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.mixedSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tornSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.clockIndex, __mmv)?;
        Ok(())
    }
}
impl Default for NonlinearSystem {
    fn default() -> Self {
        Self {
            index: Default::default(),
            eqs: Default::default(),
            crefs: Default::default(),
            indexNonLinearSystem: Default::default(),
            nUnknowns: Default::default(),
            jacobianMatrix: Default::default(),
            homotopySupport: Default::default(),
            mixedSystem: Default::default(),
            tornSystem: Default::default(),
            clockIndex: Default::default(),
        }
    }
}

pub type NONLINEARSYSTEM = NonlinearSystem;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct StateSet {
    pub index: i32,
    pub nCandidates: i32,
    pub nStates: i32,
    pub states: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    pub statescandidates: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    pub crA: metamodelica::Ref<DAE::ComponentRef>,
    pub jacobianMatrix: metamodelica::Ref<JacobianMatrix>,
}

impl metamodelica::gc::MMTrace for StateSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nCandidates, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nStates, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.states, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.statescandidates, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.crA, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianMatrix, __mmv)?;
        Ok(())
    }
}
impl Default for StateSet {
    fn default() -> Self {
        Self {
            index: Default::default(),
            nCandidates: Default::default(),
            nStates: Default::default(),
            states: Default::default(),
            statescandidates: Default::default(),
            crA: Default::default(),
            jacobianMatrix: Default::default(),
        }
    }
}

pub type SES_STATESET = StateSet;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ExtObjInfo {
    pub vars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    pub aliases: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>,
}

impl metamodelica::gc::MMTrace for ExtObjInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.aliases, __mmv)?;
        Ok(())
    }
}
impl Default for ExtObjInfo {
    fn default() -> Self {
        Self {
            vars: Default::default(),
            aliases: Default::default(),
        }
    }
}

pub type EXTOBJINFO = ExtObjInfo;

/// Settings for simulation init file header.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SimulationSettings {
    pub startTime: metamodelica::Real,
    pub stopTime: metamodelica::Real,
    pub numberOfIntervals: i32,
    pub stepSize: metamodelica::Real,
    pub tolerance: metamodelica::Real,
    pub method: ArcStr,
    pub options: ArcStr,
    pub outputFormat: ArcStr,
    pub variableFilter: ArcStr,
    pub cflags: ArcStr,
    pub simflags: ArcStr,
}

impl metamodelica::gc::MMTrace for SimulationSettings {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.startTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stopTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfIntervals, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stepSize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.tolerance, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.method, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.options, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outputFormat, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variableFilter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cflags, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.simflags, __mmv)?;
        Ok(())
    }
}
impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            startTime: Default::default(),
            stopTime: Default::default(),
            numberOfIntervals: Default::default(),
            stepSize: Default::default(),
            tolerance: Default::default(),
            method: Default::default(),
            options: Default::default(),
            outputFormat: Default::default(),
            variableFilter: Default::default(),
            cflags: Default::default(),
            simflags: Default::default(),
        }
    }
}

pub type SIMULATION_SETTINGS = SimulationSettings;

/* ***** HashTable ComponentRef -> SimCodeVar.SimVar ******/
pub type Key = metamodelica::Ref<DAE::ComponentRef>;

pub type Value = metamodelica::Ref<SimCodeVar::SimVar>;

pub type HashTableCrefToSimVar = (
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

/* FMI 2.0 Export */
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiUnknown {
    pub index: i32,
    pub dependencies: metamodelica::List<i32>,
    pub dependenciesKind: metamodelica::List<ArcStr>,
}

impl metamodelica::gc::MMTrace for FmiUnknown {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dependencies, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dependenciesKind, __mmv)?;
        Ok(())
    }
}
impl Default for FmiUnknown {
    fn default() -> Self {
        Self {
            index: Default::default(),
            dependencies: Default::default(),
            dependenciesKind: Default::default(),
        }
    }
}

pub type FMIUNKNOWN = FmiUnknown;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiOutputs {
    pub fmiUnknownsList: metamodelica::List<FmiUnknown>,
}

impl metamodelica::gc::MMTrace for FmiOutputs {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiUnknownsList, __mmv)?;
        Ok(())
    }
}
impl Default for FmiOutputs {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
        }
    }
}

pub type FMIOUTPUTS = FmiOutputs;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiDerivatives {
    pub fmiUnknownsList: metamodelica::List<FmiUnknown>,
}

impl metamodelica::gc::MMTrace for FmiDerivatives {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiUnknownsList, __mmv)?;
        Ok(())
    }
}
impl Default for FmiDerivatives {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
        }
    }
}

pub type FMIDERIVATIVES = FmiDerivatives;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiDiscreteStates {
    pub fmiUnknownsList: metamodelica::List<FmiUnknown>,
}

impl metamodelica::gc::MMTrace for FmiDiscreteStates {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiUnknownsList, __mmv)?;
        Ok(())
    }
}
impl Default for FmiDiscreteStates {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
        }
    }
}

pub type FMIDISCRETESTATES = FmiDiscreteStates;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiInitialUnknowns {
    pub fmiUnknownsList: metamodelica::List<FmiUnknown>,
    /// use the sorted crefs to get the ValueReference of unknowns
    pub sortedUnknownCrefs: metamodelica::List<(i32, metamodelica::Ref<DAE::ComponentRef>)>,
    /// use the sorted crefs to get the ValueReference of knowns
    pub sortedknownCrefs: metamodelica::List<(i32, metamodelica::Ref<DAE::ComponentRef>)>,
}

impl metamodelica::gc::MMTrace for FmiInitialUnknowns {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiUnknownsList, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sortedUnknownCrefs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sortedknownCrefs, __mmv)?;
        Ok(())
    }
}
impl Default for FmiInitialUnknowns {
    fn default() -> Self {
        Self {
            fmiUnknownsList: Default::default(),
            sortedUnknownCrefs: Default::default(),
            sortedknownCrefs: Default::default(),
        }
    }
}

pub type FMIINITIALUNKNOWNS = FmiInitialUnknowns;

/// An array the FMI 3.0 modelDescription.xml lists as one variable
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiArray {
    /// the SimVar of the first scalar element
    pub first: metamodelica::Ref<DAE::ComponentRef>,
    /// of that element
    pub fmiIndex: i32,
    pub numElements: i32,
}

impl metamodelica::gc::MMTrace for FmiArray {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.first, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numElements, __mmv)?;
        Ok(())
    }
}
impl Default for FmiArray {
    fn default() -> Self {
        Self {
            first: Default::default(),
            fmiIndex: Default::default(),
            numElements: Default::default(),
        }
    }
}

pub type FMIARRAY = FmiArray;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiModelStructure {
    pub fmiOutputs: FmiOutputs,
    pub fmiDerivatives: FmiDerivatives,
    pub continuousPartialDerivatives: Option<metamodelica::Ref<JacobianMatrix>>,
    pub initialPartialDerivatives: Option<metamodelica::Ref<JacobianMatrix>>,
    pub fmiDiscreteStates: FmiDiscreteStates,
    pub fmiInitialUnknowns: FmiInitialUnknowns,
    pub fmiArrays: metamodelica::List<FmiArray>,
}

impl metamodelica::gc::MMTrace for FmiModelStructure {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiOutputs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiDerivatives, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.continuousPartialDerivatives, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialPartialDerivatives, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiDiscreteStates, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiInitialUnknowns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiArrays, __mmv)?;
        Ok(())
    }
}
impl Default for FmiModelStructure {
    fn default() -> Self {
        Self {
            fmiOutputs: Default::default(),
            fmiDerivatives: Default::default(),
            continuousPartialDerivatives: Default::default(),
            initialPartialDerivatives: Default::default(),
            fmiDiscreteStates: Default::default(),
            fmiInitialUnknowns: Default::default(),
            fmiArrays: Default::default(),
        }
    }
}

pub type FMIMODELSTRUCTURE = FmiModelStructure;

/* FMI 3.0 Terminals (terminalsAndIcons.xml) */
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiTerminal {
    /// connector instance name, e.g. bus
    pub name: ArcStr,
    /// the connector type path (e.g. Modelica....Flange_a), \"\" if unknown
    pub terminalKind: ArcStr,
    /// true for expandable connectors (matchingRule=bus, else plug)
    pub isExpandable: bool,
    pub members: metamodelica::List<FmiTerminalMember>,
}

impl metamodelica::gc::MMTrace for FmiTerminal {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.terminalKind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isExpandable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.members, __mmv)?;
        Ok(())
    }
}
pub type FMI_TERMINAL = FmiTerminal;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiTerminalMember {
    /// exported variable cref; template formats it exactly as in modelDescription.xml (e.g. bus.a)
    pub variable: metamodelica::Ref<DAE::ComponentRef>,
    /// name within the terminal, e.g. a
    pub memberName: ArcStr,
    /// role of the member, derived from causality (input/output/...)
    pub variableKind: ArcStr,
}

impl metamodelica::gc::MMTrace for FmiTerminalMember {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.variable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.memberName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variableKind, __mmv)?;
        Ok(())
    }
}
impl Default for FmiTerminalMember {
    fn default() -> Self {
        Self {
            variable: Default::default(),
            memberName: Default::default(),
            variableKind: Default::default(),
        }
    }
}

pub type FMI_TERMINAL_MEMBER = FmiTerminalMember;

/* The Documentation(figures=...) annotation resolved against the exported
variables; emitted by CodegenFMU3 as the OpenModelica <Figures> vendor
annotation. See SimCodeUtil.getFMI3Figures. */
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiFigure {
    pub title: ArcStr,
    /// plot-group name, \"\" if none
    pub group: ArcStr,
    /// display automatically after simulation
    pub preferred: bool,
    /// \"\" if none
    pub caption: ArcStr,
    pub plots: metamodelica::List<FmiPlot>,
}

impl metamodelica::gc::MMTrace for FmiFigure {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.title, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.group, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.preferred, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.caption, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.plots, __mmv)?;
        Ok(())
    }
}
impl Default for FmiFigure {
    fn default() -> Self {
        Self {
            title: Default::default(),
            group: Default::default(),
            preferred: Default::default(),
            caption: Default::default(),
            plots: Default::default(),
        }
    }
}

pub type FMI_FIGURE = FmiFigure;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiPlot {
    pub title: ArcStr,
    pub curves: metamodelica::List<FmiCurve>,
    pub xAxis: FmiFigureAxis,
    pub yAxis: FmiFigureAxis,
    /// set when every curve's y variable is a member of this one terminal
    pub terminal: Option<ArcStr>,
}

impl metamodelica::gc::MMTrace for FmiPlot {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.title, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.curves, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.xAxis, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.yAxis, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.terminal, __mmv)?;
        Ok(())
    }
}
impl Default for FmiPlot {
    fn default() -> Self {
        Self {
            title: Default::default(),
            curves: Default::default(),
            xAxis: Default::default(),
            yAxis: Default::default(),
            terminal: Default::default(),
        }
    }
}

pub type FMI_PLOT = FmiPlot;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiCurve {
    /// NONE() means simulation time; a cref is formatted like a modelDescription variable
    pub xVariable: Option<metamodelica::Ref<DAE::ComponentRef>>,
    /// resolved exported variable; formatted like a modelDescription variable
    pub yVariable: metamodelica::Ref<DAE::ComponentRef>,
    /// \"\" if none
    pub legend: ArcStr,
}

impl metamodelica::gc::MMTrace for FmiCurve {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.xVariable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.yVariable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.legend, __mmv)?;
        Ok(())
    }
}
impl Default for FmiCurve {
    fn default() -> Self {
        Self {
            xVariable: Default::default(),
            yVariable: Default::default(),
            legend: Default::default(),
        }
    }
}

pub type FMI_CURVE = FmiCurve;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiFigureAxis {
    /// \"\" if none
    pub label: ArcStr,
    /// \"\" if none
    pub unit: ArcStr,
    /// only when explicitly set
    pub min: Option<metamodelica::Real>,
    /// only when explicitly set
    pub max: Option<metamodelica::Real>,
    /// true for a logarithmic axis
    pub logScale: bool,
}

impl metamodelica::gc::MMTrace for FmiFigureAxis {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.label, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.unit, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.min, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.max, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.logScale, __mmv)?;
        Ok(())
    }
}
impl Default for FmiFigureAxis {
    fn default() -> Self {
        Self {
            label: Default::default(),
            unit: Default::default(),
            min: Default::default(),
            max: Default::default(),
            logScale: Default::default(),
        }
    }
}

pub type FMI_FIGURE_AXIS = FmiFigureAxis;

/* FMI 3.0 Clocks (output clocks from the model's clocked partitions) */
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiClock {
    /// globally unique value reference (clock base-type block)
    pub valueReference: i32,
    /// clock name as in modelDescription.xml
    pub name: ArcStr,
    /// constant | fixed | tunable | changing | countdown | triggered
    pub intervalVariability: ArcStr,
    /// true for rational (counter/resolution) clocks
    pub supportsFraction: bool,
    /// the period for a periodic clock, or \"\" if not constant
    pub intervalDecimal: ArcStr,
    /// rational-clock counter, or \"\"
    pub intervalCounter: ArcStr,
    /// rational-clock resolution, or \"\"
    pub resolution: ArcStr,
}

impl metamodelica::gc::MMTrace for FmiClock {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.valueReference, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intervalVariability, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.supportsFraction, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intervalDecimal, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intervalCounter, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.resolution, __mmv)?;
        Ok(())
    }
}
impl Default for FmiClock {
    fn default() -> Self {
        Self {
            valueReference: Default::default(),
            name: Default::default(),
            intervalVariability: Default::default(),
            supportsFraction: Default::default(),
            intervalDecimal: Default::default(),
            intervalCounter: Default::default(),
            resolution: Default::default(),
        }
    }
}

pub type FMI_CLOCK = FmiClock;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum FmiSimulationFlags {
    FMI_SIMULATION_FLAGS {
        nameValueTuples: metamodelica::List<(ArcStr, ArcStr)>,
    },
    FMI_SIMULATION_FLAGS_FILE {
        path: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for FmiSimulationFlags {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            FmiSimulationFlags::FMI_SIMULATION_FLAGS { nameValueTuples } => {
                metamodelica::gc::MMTrace::mm_accept(nameValueTuples, __mmv)?;
                Ok(())
            }
            FmiSimulationFlags::FMI_SIMULATION_FLAGS_FILE { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for FmiSimulationFlags {
    fn default() -> Self {
        Self::FMI_SIMULATION_FLAGS {
            nameValueTuples: Default::default(),
        }
    }
}
pub use self::FmiSimulationFlags::{FMI_SIMULATION_FLAGS, FMI_SIMULATION_FLAGS_FILE};

pub static defaultFmiSimulationFlags: std::sync::LazyLock<FmiSimulationFlags> =
    std::sync::LazyLock::new(|| FmiSimulationFlags::FMI_SIMULATION_FLAGS {
        nameValueTuples: list![(literal!("s"), literal!("euler"))],
    });
