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

use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::HpcOmScheduler;
use crate::HpcOmTaskGraph;
use crate::SimCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeUtilShared;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// -------------------------------------------
// STRUCTURES
// -------------------------------------------
pub(crate) const VARDATATYPE_FLOAT: i32 = 1;

pub(crate) const VARDATATYPE_INTEGER: i32 = 2;

pub(crate) const VARDATATYPE_BOOLEAN: i32 = 3;

pub(crate) const VARDATATYPE_STRING: i32 = 4;

pub(crate) const VARTYPE_STATE: i32 = 1;

pub(crate) const VARTYPE_STATEDER: i32 = 2;

pub(crate) const VARTYPE_PARAM: i32 = 3;

pub(crate) const VARTYPE_ALIAS: i32 = 4;

pub(crate) const VARTYPE_OTHER: i32 = 5;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum CacheMap {
    CACHEMAP {
        cacheLineSize: i32,
        cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
        cacheLinesFloat: metamodelica::List<CacheLineMap>,
        cacheLinesInt: metamodelica::List<CacheLineMap>,
        cacheLinesBool: metamodelica::List<CacheLineMap>,
    },
    UNIFORM_CACHEMAP {
        cacheLineSize: i32,
        cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
        cacheLines: metamodelica::List<CacheLineMap>,
    },
}
impl metamodelica::gc::MMTrace for CacheMap {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            CacheMap::CACHEMAP {
                cacheLineSize,
                cacheVariables,
                cacheLinesFloat,
                cacheLinesInt,
                cacheLinesBool,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cacheLineSize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cacheVariables, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cacheLinesFloat, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cacheLinesInt, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cacheLinesBool, __mmv)?;
                Ok(())
            }
            CacheMap::UNIFORM_CACHEMAP {
                cacheLineSize,
                cacheVariables,
                cacheLines,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cacheLineSize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cacheVariables, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cacheLines, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for CacheMap {
    fn default() -> Self {
        Self::UNIFORM_CACHEMAP {
            cacheLineSize: Default::default(),
            cacheVariables: Default::default(),
            cacheLines: Default::default(),
        }
    }
}
pub(crate) use self::CacheMap::{CACHEMAP, UNIFORM_CACHEMAP};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CacheLineMap {
    pub idx: i32,
    pub numBytesFree: i32,
    pub entries: metamodelica::List<CacheLineEntry>,
}

impl metamodelica::gc::MMTrace for CacheLineMap {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.idx, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numBytesFree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.entries, __mmv)?;
        Ok(())
    }
}
impl Default for CacheLineMap {
    fn default() -> Self {
        Self {
            idx: Default::default(),
            numBytesFree: Default::default(),
            entries: Default::default(),
        }
    }
}

pub type CACHELINEMAP = CacheLineMap;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CacheLineEntry {
    pub start: i32,
    pub dataType: i32,
    pub size: i32,
    pub scVarIdx: i32,
    pub threadOwner: i32,
}

impl metamodelica::gc::MMTrace for CacheLineEntry {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.start, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dataType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.size, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scVarIdx, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.threadOwner, __mmv)?;
        Ok(())
    }
}
impl Default for CacheLineEntry {
    fn default() -> Self {
        Self {
            start: Default::default(),
            dataType: Default::default(),
            size: Default::default(),
            scVarIdx: Default::default(),
            threadOwner: Default::default(),
        }
    }
}

pub type CACHELINEENTRY = CacheLineEntry;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CacheMapMeta {
    pub allSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    pub simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    pub scVarCLMapping: metamodelica::Array<(i32, i32)>,
}

impl metamodelica::gc::MMTrace for CacheMapMeta {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.allSCVarsMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.simCodeVarTypes, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scVarCLMapping, __mmv)?;
        Ok(())
    }
}
impl Default for CacheMapMeta {
    fn default() -> Self {
        Self {
            allSCVarsMapping: Default::default(),
            simCodeVarTypes: Default::default(),
            scVarCLMapping: Default::default(),
        }
    }
}

pub type CACHEMAPMETA = CacheMapMeta;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum PartlyFilledCacheLine {
    PARTLYFILLEDCACHELINE_LEVEL {
        cacheLineMap: CacheLineMap,
        prefetchLevel: metamodelica::List<i32>,
        writeLevel: metamodelica::List<(i32, i32)>,
    },
    PARTLYFILLEDCACHELINE_THREAD {
        cacheLineMap: CacheLineMap,
    },
}
impl metamodelica::gc::MMTrace for PartlyFilledCacheLine {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL {
                cacheLineMap,
                prefetchLevel,
                writeLevel,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cacheLineMap, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(prefetchLevel, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(writeLevel, __mmv)?;
                Ok(())
            }
            PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD { cacheLineMap } => {
                metamodelica::gc::MMTrace::mm_accept(cacheLineMap, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for PartlyFilledCacheLine {
    fn default() -> Self {
        Self::PARTLYFILLEDCACHELINE_THREAD {
            cacheLineMap: Default::default(),
        }
    }
}
pub use self::PartlyFilledCacheLine::{PARTLYFILLEDCACHELINE_LEVEL, PARTLYFILLEDCACHELINE_THREAD};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ScVarInfo {
    pub ownerThread: i32,
    pub isShared: bool,
}

impl metamodelica::gc::MMTrace for ScVarInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ownerThread, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isShared, __mmv)?;
        Ok(())
    }
}
impl Default for ScVarInfo {
    fn default() -> Self {
        Self {
            ownerThread: Default::default(),
            isShared: Default::default(),
        }
    }
}

pub type SCVARINFO = ScVarInfo;

pub type PartlyFilledCacheLines = (
    metamodelica::List<PartlyFilledCacheLine>,
    metamodelica::List<PartlyFilledCacheLine>,
    metamodelica::List<PartlyFilledCacheLine>,
);

pub type CacheLines = (
    metamodelica::List<CacheLineMap>,
    metamodelica::List<CacheLineMap>,
    metamodelica::List<CacheLineMap>,
);

// -------------------------------------------
// FUNCTIONS
// -------------------------------------------
pub(crate) fn createMemoryMap(
    mut iModelInfo: SimCode::ModelInfo,
    mut iVarToArrayIndexMapping: (
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iVarToIndexMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iEqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iFileNamePrefix: &ArcStr,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iSccSimEqMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iCriticalPaths: &metamodelica::List<metamodelica::List<i32>>,
    mut iCriticalPathsWoC: &metamodelica::List<metamodelica::List<i32>>,
    mut iCriticalPathInfo: ArcStr,
    mut iNumberOfThreads: i32,
    mut iAllComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut isInitial: bool,
) -> Result<(
    Option<HpcOmSimCode::MemoryMap>,
    (
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut oMemoryMap: Option<HpcOmSimCode::MemoryMap>;
    let mut oVarToArrayIndexMapping: (
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
    let mut oVarToIndexMapping: (
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
    let mut simCodeVars: SimCodeVar::SimVars = <SimCodeVar::SimVars as ::std::default::Default>::default();
    let mut stateVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut derivativeVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut discreteAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut inputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut outputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut aliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut paramVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsFloatOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
        metamodelica::nil();
    let mut notOptimizedVarsIntOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
        metamodelica::nil();
    let mut notOptimizedVarsBoolOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
        metamodelica::nil();
    let mut notOptimizedVarsStringOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
        metamodelica::nil();
    let mut notOptimizedVarsFloat: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsInt: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsBool: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsString: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVars: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ) = (
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
    );
    let mut allVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>> = Default::default();
    let mut simVarIdxMappingHashTable: (
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
    let mut numCL: i32 = 0;
    let mut threadAttIdx: i32 = 0;
    let mut clTaskMapping: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut scVarSolvedTaskMapping: metamodelica::Array<i32> = Default::default();
    let mut sccNodeMapping: metamodelica::Array<i32> = Default::default();
    let mut scVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut annotInfo: metamodelica::Array<ArcStr> = Default::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut graphIdx: i32 = 0;
    let mut graphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut eqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>> = Default::default();
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut tmpMemoryMapOpt: Option<HpcOmSimCode::MemoryMap> = None;
    let mut varCount: i32 = 0;
    let mut stateVarsCnt: i32 = 0;
    let mut derivativeVarsCnt: i32 = 0;
    let mut algVarsCnt: i32 = 0;
    let mut discreteAlgVarsCnt: i32 = 0;
    let mut intAlgVarsCnt: i32 = 0;
    let mut boolAlgVarsCnt: i32 = 0;
    let mut stringAlgVarsCnt: i32 = 0;
    let mut inputVarsCnt: i32 = 0;
    let mut outputVarsCnt: i32 = 0;
    let mut aliasVarsCnt: i32 = 0;
    let mut intAliasVarsCnt: i32 = 0;
    let mut boolAliasVarsCnt: i32 = 0;
    let mut stringAliasVarsCnt: i32 = 0;
    let mut paramVarsCnt: i32 = 0;
    let mut intParamVarsCnt: i32 = 0;
    let mut boolParamVarsCnt: i32 = 0;
    let mut stringParamVarsCnt: i32 = 0;
    let mut VARSIZE_FLOAT: i32 = 0;
    let mut VARSIZE_INTEGER: i32 = 0;
    let mut VARSIZE_BOOLEAN: i32 = 0;
    let mut VARSIZE_STRING: i32 = 0;
    let mut CACHELINE_SIZE: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut taskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut taskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut nodeSccMapping: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut flatEqSimCodeVarMapping: metamodelica::Array<(i32, metamodelica::List<i32>)> = Default::default();
    let mut sccEqMapping: metamodelica::Array<metamodelica::List<(i32, i32, i32)>> = Default::default();
    let mut scVarInfos: metamodelica::Array<ScVarInfo> = Default::default();
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
    (oMemoryMap, oVarToArrayIndexMapping, oVarToIndexMapping) = 'mc: {
        let __mc_input = (
            iVarToArrayIndexMapping.clone(),
            iVarToIndexMapping.clone(),
            iTaskGraphMeta.clone(),
        );
        if let Ok((
            __v,
            __wb0,
            __wb1,
            __wb2,
            __wb3,
            __wb4,
            __wb5,
            __wb6,
            __wb7,
            __wb8,
            __wb9,
            __wb10,
            __wb11,
            __wb12,
            __wb13,
            __wb14,
            __wb15,
            __wb16,
            __wb17,
            __wb18,
            __wb19,
            __wb20,
            __wb21,
            __wb22,
            __wb23,
            __wb24,
            __wb25,
            __wb26,
            __wb27,
            __wb28,
            __wb29,
            __wb30,
            __wb31,
            __wb32,
            __wb33,
            __wb34,
            __wb35,
            __wb36,
            __wb37,
            __wb38,
            __wb39,
            __wb40,
            __wb41,
            __wb42,
            __wb43,
            __wb44,
            __wb45,
            __wb46,
            __wb47,
            __wb48,
            __wb49,
            __wb50,
            __wb51,
            __wb52,
            __wb53,
            __wb54,
            __wb55,
            __wb56,
            __wb57,
            __wb58,
            __wb59,
            __wb60,
            __wb61,
            __wb62,
            __wb63,
            __wb64,
            __wb65,
            __wb66,
            __wb67,
            __wb68,
            __wb69,
            __wb70,
            __wb71,
            __wb72,
        )) = (|| -> Result<_> {
            let (
                mut varToArrayIndexMapping,
                mut varToIndexMapping,
                HpcOmTaskGraph::TaskGraphMeta {
                    eqCompMapping: mut eqCompMapping,
                    varCompMapping: mut varCompMapping,
                    ..
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut CACHELINE_SIZE: i32 = CACHELINE_SIZE.clone();
            let mut VARSIZE_BOOLEAN: i32 = VARSIZE_BOOLEAN.clone();
            let mut VARSIZE_FLOAT: i32 = VARSIZE_FLOAT.clone();
            let mut VARSIZE_INTEGER: i32 = VARSIZE_INTEGER.clone();
            let mut VARSIZE_STRING: i32 = VARSIZE_STRING.clone();
            let mut adjacencyMatrix: metamodelica::Array<metamodelica::List<i32>> = adjacencyMatrix.clone();
            let mut algVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = algVars.clone();
            let mut algVarsCnt: i32 = algVarsCnt.clone();
            let mut aliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = aliasVars.clone();
            let mut aliasVarsCnt: i32 = aliasVarsCnt.clone();
            let mut allVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
                allVarsMapping.clone();
            let mut annotInfo: metamodelica::Array<ArcStr> = annotInfo.clone();
            let mut boolAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = boolAlgVars.clone();
            let mut boolAlgVarsCnt: i32 = boolAlgVarsCnt.clone();
            let mut boolAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = boolAliasVars.clone();
            let mut boolAliasVarsCnt: i32 = boolAliasVarsCnt.clone();
            let mut boolParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = boolParamVars.clone();
            let mut boolParamVarsCnt: i32 = boolParamVarsCnt.clone();
            let mut cacheMap: CacheMap = cacheMap.clone();
            let mut clTaskMapping: metamodelica::Array<metamodelica::List<i32>> = clTaskMapping.clone();
            let mut derivativeVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = derivativeVars.clone();
            let mut derivativeVarsCnt: i32 = derivativeVarsCnt.clone();
            let mut discreteAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                discreteAlgVars.clone();
            let mut discreteAlgVarsCnt: i32 = discreteAlgVarsCnt.clone();
            let mut eqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>> =
                eqSimCodeVarMapping.clone();
            let mut fileName: ArcStr = fileName.clone();
            let mut flatEqSimCodeVarMapping: metamodelica::Array<(i32, metamodelica::List<i32>)> =
                flatEqSimCodeVarMapping.clone();
            let mut graphIdx: i32 = graphIdx.clone();
            let mut graphInfo: GraphML::GraphInfo = graphInfo.clone();
            let mut inputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = inputVars.clone();
            let mut inputVarsCnt: i32 = inputVarsCnt.clone();
            let mut intAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = intAlgVars.clone();
            let mut intAlgVarsCnt: i32 = intAlgVarsCnt.clone();
            let mut intAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = intAliasVars.clone();
            let mut intAliasVarsCnt: i32 = intAliasVarsCnt.clone();
            let mut intParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = intParamVars.clone();
            let mut intParamVarsCnt: i32 = intParamVarsCnt.clone();
            let mut nodeSccMapping: metamodelica::Array<metamodelica::List<i32>> = nodeSccMapping.clone();
            let mut notOptimizedVars: (
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
                metamodelica::List<i32>,
            ) = notOptimizedVars.clone();
            let mut notOptimizedVarsBool: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                notOptimizedVarsBool.clone();
            let mut notOptimizedVarsBoolOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
                notOptimizedVarsBoolOpt.clone();
            let mut notOptimizedVarsFloat: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                notOptimizedVarsFloat.clone();
            let mut notOptimizedVarsFloatOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
                notOptimizedVarsFloatOpt.clone();
            let mut notOptimizedVarsInt: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                notOptimizedVarsInt.clone();
            let mut notOptimizedVarsIntOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
                notOptimizedVarsIntOpt.clone();
            let mut notOptimizedVarsString: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                notOptimizedVarsString.clone();
            let mut notOptimizedVarsStringOpt: metamodelica::List<Option<metamodelica::Ref<SimCodeVar::SimVar>>> =
                notOptimizedVarsStringOpt.clone();
            let mut numCL: i32 = numCL.clone();
            let mut outputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = outputVars.clone();
            let mut outputVarsCnt: i32 = outputVarsCnt.clone();
            let mut paramVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = paramVars.clone();
            let mut paramVarsCnt: i32 = paramVarsCnt.clone();
            let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = scVarCLMapping.clone();
            let mut scVarInfos: metamodelica::Array<ScVarInfo> = scVarInfos.clone();
            let mut scVarSolvedTaskMapping: metamodelica::Array<i32> = scVarSolvedTaskMapping.clone();
            let mut scVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>> =
                scVarUnsolvedTaskMapping.clone();
            let mut sccEqMapping: metamodelica::Array<metamodelica::List<(i32, i32, i32)>> = sccEqMapping.clone();
            let mut sccNodeMapping: metamodelica::Array<i32> = sccNodeMapping.clone();
            let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = simCodeVarTypes.clone();
            let mut simCodeVars: SimCodeVar::SimVars = simCodeVars.clone();
            let mut simVarIdxMappingHashTable: (
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
            let mut stateVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = stateVars.clone();
            let mut stateVarsCnt: i32 = stateVarsCnt.clone();
            let mut stringAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = stringAlgVars.clone();
            let mut stringAlgVarsCnt: i32 = stringAlgVarsCnt.clone();
            let mut stringAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                stringAliasVars.clone();
            let mut stringAliasVarsCnt: i32 = stringAliasVarsCnt.clone();
            let mut stringParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> =
                stringParamVars.clone();
            let mut stringParamVarsCnt: i32 = stringParamVarsCnt.clone();
            let mut taskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>> = taskSolvedVarsMapping.clone();
            let mut taskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>> =
                taskUnsolvedVarsMapping.clone();
            let mut threadAttIdx: i32 = threadAttIdx.clone();
            let mut tmpMemoryMapOpt: Option<HpcOmSimCode::MemoryMap> = tmpMemoryMapOpt.clone();
            let mut varCount: i32 = varCount.clone();
            VARSIZE_FLOAT = 8;
            VARSIZE_INTEGER = 4;
            VARSIZE_BOOLEAN = 1;
            VARSIZE_STRING = 4;
            CACHELINE_SIZE = 64;
            let SimCode::MODELINFO { vars: __pa0, .. } = &iModelInfo;
            simCodeVars = metamodelica::Own::own(__pa0);
            let SimCodeVar::SIMVARS {
                stateVars: __pa1,
                derivativeVars: __pa2,
                algVars: __pa3,
                discreteAlgVars: __pa4,
                intAlgVars: __pa5,
                boolAlgVars: __pa6,
                stringAlgVars: __pa7,
                inputVars: __pa8,
                outputVars: __pa9,
                aliasVars: __pa10,
                intAliasVars: __pa11,
                boolAliasVars: __pa12,
                stringAliasVars: __pa13,
                paramVars: __pa14,
                intParamVars: __pa15,
                boolParamVars: __pa16,
                stringParamVars: __pa17,
                ..
            } = &simCodeVars;
            stateVars = metamodelica::Own::own(__pa1);
            derivativeVars = metamodelica::Own::own(__pa2);
            algVars = metamodelica::Own::own(__pa3);
            discreteAlgVars = metamodelica::Own::own(__pa4);
            intAlgVars = metamodelica::Own::own(__pa5);
            boolAlgVars = metamodelica::Own::own(__pa6);
            stringAlgVars = metamodelica::Own::own(__pa7);
            inputVars = metamodelica::Own::own(__pa8);
            outputVars = metamodelica::Own::own(__pa9);
            aliasVars = metamodelica::Own::own(__pa10);
            intAliasVars = metamodelica::Own::own(__pa11);
            boolAliasVars = metamodelica::Own::own(__pa12);
            stringAliasVars = metamodelica::Own::own(__pa13);
            paramVars = metamodelica::Own::own(__pa14);
            intParamVars = metamodelica::Own::own(__pa15);
            boolParamVars = metamodelica::Own::own(__pa16);
            stringParamVars = metamodelica::Own::own(__pa17);
            allVarsMapping = SimCodeUtil::createIdxSCVarMapping(simCodeVars.clone())?;
            simVarIdxMappingHashTable = HashTableCrILst::emptyHashTableSized(BaseHashTable::biggerBucketSize.clone());
            varCount = 0;
            stateVarsCnt = ((stateVars).len() as i32);
            varCount = varCount + stateVarsCnt;
            derivativeVarsCnt = ((derivativeVars).len() as i32);
            varCount = varCount + derivativeVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &algVars,
                varCount,
                VARDATATYPE_FLOAT.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            algVarsCnt = ((algVars).len() as i32);
            varCount = varCount + algVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &discreteAlgVars,
                varCount,
                VARDATATYPE_FLOAT.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            discreteAlgVarsCnt = ((discreteAlgVars).len() as i32);
            varCount = varCount + discreteAlgVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &intAlgVars,
                varCount,
                VARDATATYPE_INTEGER.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            intAlgVarsCnt = ((intAlgVars).len() as i32);
            varCount = varCount + intAlgVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &boolAlgVars,
                varCount,
                VARDATATYPE_BOOLEAN.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            boolAlgVarsCnt = ((boolAlgVars).len() as i32);
            varCount = varCount + boolAlgVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &stringAlgVars,
                varCount,
                VARDATATYPE_STRING.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            stringAlgVarsCnt = ((stringAlgVars).len() as i32);
            varCount = varCount + stringAlgVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &inputVars,
                varCount,
                VARDATATYPE_FLOAT.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            inputVarsCnt = ((inputVars).len() as i32);
            varCount = varCount + inputVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &outputVars,
                varCount,
                VARDATATYPE_FLOAT.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            outputVarsCnt = ((outputVars).len() as i32);
            varCount = varCount + outputVarsCnt;
            aliasVarsCnt = ((aliasVars).len() as i32);
            varCount = varCount + aliasVarsCnt;
            intAliasVarsCnt = ((intAliasVars).len() as i32);
            varCount = varCount + intAliasVarsCnt;
            boolAliasVarsCnt = ((boolAliasVars).len() as i32);
            varCount = varCount + boolAliasVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &stringAliasVars,
                varCount,
                VARDATATYPE_STRING.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            stringAliasVarsCnt = ((stringAliasVars).len() as i32);
            varCount = varCount + stringAliasVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &paramVars,
                varCount,
                VARDATATYPE_FLOAT.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            paramVarsCnt = ((paramVars).len() as i32);
            varCount = varCount + paramVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &intParamVars,
                varCount,
                VARDATATYPE_INTEGER.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            intParamVarsCnt = ((intParamVars).len() as i32);
            varCount = varCount + intParamVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &boolParamVars,
                varCount,
                VARDATATYPE_BOOLEAN.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            boolParamVarsCnt = ((boolParamVars).len() as i32);
            varCount = varCount + boolParamVarsCnt;
            simVarIdxMappingHashTable = fillSimVarHashTable(
                &stringParamVars,
                varCount,
                VARDATATYPE_STRING.clone(),
                simVarIdxMappingHashTable.clone(),
            )?;
            stringParamVarsCnt = ((stringParamVars).len() as i32);
            varCount = varCount + stringParamVarsCnt;
            simCodeVarTypes = arrayCreate(varCount, (-1, -1, -1));
            varCount = 0;
            varCount = varCount + stateVarsCnt;
            varCount = varCount + derivativeVarsCnt;
            if algVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + algVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + algVarsCnt;
            if discreteAlgVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + discreteAlgVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + discreteAlgVarsCnt;
            if intAlgVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + intAlgVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_INTEGER.clone(), VARSIZE_INTEGER, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + intAlgVarsCnt;
            if boolAlgVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + boolAlgVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_BOOLEAN.clone(), VARSIZE_BOOLEAN, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + boolAlgVarsCnt;
            if stringAlgVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + stringAlgVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_STRING.clone(), VARSIZE_STRING, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + stringAlgVarsCnt;
            if inputVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + inputVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + inputVarsCnt;
            if outputVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + outputVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT, VARTYPE_OTHER.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + outputVarsCnt;
            varCount = varCount + aliasVarsCnt;
            varCount = varCount + intAliasVarsCnt;
            varCount = varCount + boolAliasVarsCnt;
            varCount = varCount + stringAliasVarsCnt;
            if paramVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + paramVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT, VARTYPE_PARAM.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + paramVarsCnt;
            if intParamVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + intParamVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_INTEGER.clone(), VARSIZE_INTEGER, VARTYPE_PARAM.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + intParamVarsCnt;
            if boolParamVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + boolParamVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_BOOLEAN.clone(), VARSIZE_BOOLEAN, VARTYPE_PARAM.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + boolParamVarsCnt;
            if stringParamVarsCnt > 0 {
                List::map_0(
                    &(List::intRange2(varCount + 1, varCount + stringParamVarsCnt)),
                    &({
                        let __pe_b1 = (VARDATATYPE_STRING.clone(), VARSIZE_STRING, VARTYPE_PARAM.clone());
                        let __pe_b2 = simCodeVarTypes.clone();
                        move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    }),
                )?;
            }
            varCount = varCount + stringParamVarsCnt;
            sccNodeMapping = HpcOmTaskGraph::getSccNodeMapping(
                metamodelica::arrayLength(iSccSimEqMapping.clone()),
                iTaskGraphMeta.clone(),
            )?;
            scVarSolvedTaskMapping = getSimCodeVarNodeMapping(
                iTaskGraphMeta.clone(),
                &iEqSystems,
                varCount,
                sccNodeMapping.clone(),
                &simVarIdxMappingHashTable,
            )?;
            eqSimCodeVarMapping = getEqSCVarMapping(iEqSystems.clone(), simVarIdxMappingHashTable.clone())?;
            sccEqMapping =
                invertEqCompMapping(eqCompMapping.clone(), metamodelica::arrayLength(sccNodeMapping.clone()))?;
            nodeSccMapping =
                invertSccNodeMapping(sccNodeMapping.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            flatEqSimCodeVarMapping = flattenEqSimCodeVarMapping(eqSimCodeVarMapping.clone())?;
            (taskSolvedVarsMapping, taskUnsolvedVarsMapping) = getTaskSimVarMapping(
                sccEqMapping.clone(),
                nodeSccMapping.clone(),
                flatEqSimCodeVarMapping.clone(),
                scVarSolvedTaskMapping.clone(),
                simCodeVarTypes.clone(),
            )?;
            scVarUnsolvedTaskMapping = transposeTasksScVarsMapping(taskUnsolvedVarsMapping.clone(), varCount)?;
            scVarInfos = createVarInfos(
                scVarSolvedTaskMapping.clone(),
                scVarUnsolvedTaskMapping.clone(),
                iSchedulerInfo.clone(),
            )?;
            if Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())? {
                (cacheMap, scVarCLMapping, numCL) = createCacheMapOptimized(
                    iTaskGraph.clone(),
                    &iTaskGraphMeta,
                    &simCodeVars,
                    allVarsMapping.clone(),
                    simCodeVarTypes.clone(),
                    scVarSolvedTaskMapping.clone(),
                    scVarUnsolvedTaskMapping.clone(),
                    CACHELINE_SIZE,
                    iAllComponents,
                    iSchedule,
                    iSchedulerInfo.clone(),
                    iNumberOfThreads,
                    taskSolvedVarsMapping.clone(),
                    taskUnsolvedVarsMapping.clone(),
                    scVarInfos.clone(),
                )?;
            } else {
                (cacheMap, scVarCLMapping, numCL) = createCacheMapDefault(
                    allVarsMapping.clone(),
                    CACHELINE_SIZE,
                    &simCodeVars,
                    scVarSolvedTaskMapping.clone(),
                    iSchedulerInfo.clone(),
                    simCodeVarTypes.clone(),
                )?;
            }
            (clTaskMapping, _) = getCacheLineTaskMapping(
                iTaskGraphMeta.clone(),
                &iEqSystems,
                &simVarIdxMappingHashTable,
                numCL,
                scVarCLMapping.clone(),
            )?;
            notOptimizedVars = getNotOptimizedVarsByCacheLineMapping(
                scVarCLMapping.clone(),
                allVarsMapping.clone(),
                simCodeVarTypes.clone(),
            )?;
            notOptimizedVarsFloatOpt = List::map(
                Util::tuple41(notOptimizedVars.clone()),
                &({
                    let __pe_b0 = allVarsMapping.clone();
                    move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1)
                }),
            )?;
            notOptimizedVarsIntOpt = List::map(
                Util::tuple42(notOptimizedVars.clone()),
                &({
                    let __pe_b0 = allVarsMapping.clone();
                    move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1)
                }),
            )?;
            notOptimizedVarsBoolOpt = List::map(
                Util::tuple43(notOptimizedVars.clone()),
                &({
                    let __pe_b0 = allVarsMapping.clone();
                    move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1)
                }),
            )?;
            notOptimizedVarsStringOpt = List::map(
                Util::tuple44(notOptimizedVars.clone()),
                &({
                    let __pe_b0 = allVarsMapping.clone();
                    move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1)
                }),
            )?;
            notOptimizedVarsFloat = List::map(notOptimizedVarsFloatOpt.clone(), &|o: Option<_>| {
                o.ok_or("pattern mismatch")
            })?;
            notOptimizedVarsInt = List::map(notOptimizedVarsIntOpt.clone(), &|o: Option<_>| {
                o.ok_or("pattern mismatch")
            })?;
            notOptimizedVarsBool = List::map(notOptimizedVarsBoolOpt.clone(), &|o: Option<_>| {
                o.ok_or("pattern mismatch")
            })?;
            notOptimizedVarsString = List::map(notOptimizedVarsStringOpt.clone(), &|o: Option<_>| {
                o.ok_or("pattern mismatch")
            })?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa18, (_, __pa19)) = GraphML::addGraph(literal!("TasksGroupGraph"), true, graphInfo.clone())?;
            graphInfo = metamodelica::Own::own(__pa18);
            graphIdx = metamodelica::Own::own(__pa19);
            let (__pa20, _, (_, __pa21)) = GraphML::addGroupNode(
                literal!("TasksGroup"),
                graphIdx,
                false,
                literal!("TG"),
                graphInfo.clone(),
            )?;
            graphInfo = metamodelica::Own::own(__pa20);
            graphIdx = metamodelica::Own::own(__pa21);
            annotInfo = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), literal!("nothing"));
            graphInfo = HpcOmTaskGraph::convertToGraphMLSccLevelSubgraph(
                iTaskGraph.clone(),
                iTaskGraphMeta.clone(),
                iCriticalPathInfo.clone(),
                HpcOmTaskGraph::convertNodeListToEdgeTuples(&((iCriticalPaths).head().cloned()?)),
                HpcOmTaskGraph::convertNodeListToEdgeTuples(&((iCriticalPathsWoC).head().cloned()?)),
                iSccSimEqMapping.clone(),
                iSchedulerInfo.clone(),
                annotInfo.clone(),
                graphIdx,
                HpcOmTaskGraph::GraphDumpOptions {
                    visualizeCriticalPath: false,
                    visualizeTaskStartAndFinishTime: false,
                    visualizeTaskCalcTime: true,
                    visualizeCommTime: true,
                },
                graphInfo.clone(),
            )?;
            let __pa22 = ::match_deref::match_deref! { match &(GraphML::getAttributeByNameAndTarget(&(literal!("ThreadId")), openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, &graphInfo)?) {
                Some((_, __pa22)) => __pa22.clone(),
                _ => return Err("pattern mismatch"),
            } };
            threadAttIdx = metamodelica::Own::own(__pa22);
            (_, adjacencyMatrix, _) = BackendDAEUtil::getAdjacencyMatrix(
                (iEqSystems).head().cloned()?,
                openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE,
                None,
                isInitial,
            )?;
            graphInfo = appendCacheLinesToGraph(
                &cacheMap,
                metamodelica::arrayLength(iTaskGraph.clone()),
                eqSimCodeVarMapping.clone(),
                &iEqSystems,
                &simVarIdxMappingHashTable,
                eqCompMapping.clone(),
                scVarSolvedTaskMapping.clone(),
                iSchedulerInfo.clone(),
                threadAttIdx,
                sccNodeMapping.clone(),
                taskSolvedVarsMapping.clone(),
                taskUnsolvedVarsMapping.clone(),
                scVarCLMapping.clone(),
                scVarInfos.clone(),
                graphInfo.clone(),
            )?;
            fileName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("taskGraph"));
                __mm_s.push_str(&*iFileNamePrefix);
                __mm_s.push_str(&*literal!("ODE_schedule_CL.graphml"));
                ArcStr::from(__mm_s)
            };
            GraphML::dumpGraph(graphInfo.clone(), fileName.clone())?;
            if Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())? {
                (varToArrayIndexMapping, varToIndexMapping, tmpMemoryMapOpt) = convertCacheToVarArrayMapping(
                    &cacheMap,
                    CACHELINE_SIZE,
                    &stateVars,
                    &derivativeVars,
                    &aliasVars,
                    &intAliasVars,
                    &boolAliasVars,
                    &stringAliasVars,
                    (VARSIZE_FLOAT, VARSIZE_INTEGER, VARSIZE_BOOLEAN),
                    &((
                        notOptimizedVarsFloat.clone(),
                        notOptimizedVarsInt.clone(),
                        notOptimizedVarsBool.clone(),
                        notOptimizedVarsString.clone(),
                    )),
                )?;
            } else {
                tmpMemoryMapOpt = None;
            }
            evaluateCacheBehaviour(
                &varToIndexMapping,
                &simVarIdxMappingHashTable,
                taskSolvedVarsMapping.clone(),
                taskUnsolvedVarsMapping.clone(),
                iTaskGraph.clone(),
                iTaskGraphT.clone(),
                iNumberOfThreads,
                CACHELINE_SIZE,
                simCodeVarTypes.clone(),
                iSchedulerInfo.clone(),
            );
            graphInfo = GraphML::createGraphInfo();
            let (__pa23, (_, __pa24)) = GraphML::addGraph(literal!("TasksGroupGraph"), true, graphInfo.clone())?;
            graphInfo = metamodelica::Own::own(__pa23);
            graphIdx = metamodelica::Own::own(__pa24);
            annotInfo = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), literal!("nothing"));
            graphInfo = HpcOmTaskGraph::convertToGraphMLSccLevelSubgraph(
                iTaskGraph.clone(),
                iTaskGraphMeta.clone(),
                iCriticalPathInfo.clone(),
                HpcOmTaskGraph::convertNodeListToEdgeTuples(&((iCriticalPaths).head().cloned()?)),
                HpcOmTaskGraph::convertNodeListToEdgeTuples(&((iCriticalPathsWoC).head().cloned()?)),
                iSccSimEqMapping.clone(),
                iSchedulerInfo.clone(),
                annotInfo.clone(),
                graphIdx,
                HpcOmTaskGraph::GraphDumpOptions {
                    visualizeCriticalPath: false,
                    visualizeTaskStartAndFinishTime: false,
                    visualizeTaskCalcTime: true,
                    visualizeCommTime: true,
                },
                graphInfo.clone(),
            )?;
            let __pa25 = ::match_deref::match_deref! { match &(GraphML::getAttributeByNameAndTarget(&(literal!("ThreadId")), openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, &graphInfo)?) {
                Some((_, __pa25)) => __pa25.clone(),
                _ => return Err("pattern mismatch"),
            } };
            threadAttIdx = metamodelica::Own::own(__pa25);
            graphInfo = appendVariablesToGraph(
                taskSolvedVarsMapping.clone(),
                taskUnsolvedVarsMapping.clone(),
                metamodelica::arrayLength(scVarSolvedTaskMapping.clone()),
                graphIdx,
                threadAttIdx,
                &simVarIdxMappingHashTable,
                allVarsMapping.clone(),
                scVarInfos.clone(),
                graphInfo.clone(),
            )?;
            fileName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("taskGraph"));
                __mm_s.push_str(&*iFileNamePrefix);
                __mm_s.push_str(&*literal!("ODE_schedule_vars.graphml"));
                ArcStr::from(__mm_s)
            };
            GraphML::dumpGraph(graphInfo.clone(), fileName.clone())?;
            Ok((
                (
                    tmpMemoryMapOpt.clone(),
                    varToArrayIndexMapping.clone(),
                    varToIndexMapping.clone(),
                ),
                CACHELINE_SIZE.clone(),
                VARSIZE_BOOLEAN.clone(),
                VARSIZE_FLOAT.clone(),
                VARSIZE_INTEGER.clone(),
                VARSIZE_STRING.clone(),
                adjacencyMatrix.clone(),
                algVars.clone(),
                algVarsCnt.clone(),
                aliasVars.clone(),
                aliasVarsCnt.clone(),
                allVarsMapping.clone(),
                annotInfo.clone(),
                boolAlgVars.clone(),
                boolAlgVarsCnt.clone(),
                boolAliasVars.clone(),
                boolAliasVarsCnt.clone(),
                boolParamVars.clone(),
                boolParamVarsCnt.clone(),
                cacheMap.clone(),
                clTaskMapping.clone(),
                derivativeVars.clone(),
                derivativeVarsCnt.clone(),
                discreteAlgVars.clone(),
                discreteAlgVarsCnt.clone(),
                eqSimCodeVarMapping.clone(),
                fileName.clone(),
                flatEqSimCodeVarMapping.clone(),
                graphIdx.clone(),
                graphInfo.clone(),
                inputVars.clone(),
                inputVarsCnt.clone(),
                intAlgVars.clone(),
                intAlgVarsCnt.clone(),
                intAliasVars.clone(),
                intAliasVarsCnt.clone(),
                intParamVars.clone(),
                intParamVarsCnt.clone(),
                nodeSccMapping.clone(),
                notOptimizedVars.clone(),
                notOptimizedVarsBool.clone(),
                notOptimizedVarsBoolOpt.clone(),
                notOptimizedVarsFloat.clone(),
                notOptimizedVarsFloatOpt.clone(),
                notOptimizedVarsInt.clone(),
                notOptimizedVarsIntOpt.clone(),
                notOptimizedVarsString.clone(),
                notOptimizedVarsStringOpt.clone(),
                numCL.clone(),
                outputVars.clone(),
                outputVarsCnt.clone(),
                paramVars.clone(),
                paramVarsCnt.clone(),
                scVarCLMapping.clone(),
                scVarInfos.clone(),
                scVarSolvedTaskMapping.clone(),
                scVarUnsolvedTaskMapping.clone(),
                sccEqMapping.clone(),
                sccNodeMapping.clone(),
                simCodeVarTypes.clone(),
                simCodeVars.clone(),
                stateVars.clone(),
                stateVarsCnt.clone(),
                stringAlgVars.clone(),
                stringAlgVarsCnt.clone(),
                stringAliasVars.clone(),
                stringAliasVarsCnt.clone(),
                stringParamVars.clone(),
                stringParamVarsCnt.clone(),
                taskSolvedVarsMapping.clone(),
                taskUnsolvedVarsMapping.clone(),
                threadAttIdx.clone(),
                tmpMemoryMapOpt.clone(),
                varCount.clone(),
            ))
        })() {
            CACHELINE_SIZE = __wb0;
            VARSIZE_BOOLEAN = __wb1;
            VARSIZE_FLOAT = __wb2;
            VARSIZE_INTEGER = __wb3;
            VARSIZE_STRING = __wb4;
            adjacencyMatrix = __wb5;
            algVars = __wb6;
            algVarsCnt = __wb7;
            aliasVars = __wb8;
            aliasVarsCnt = __wb9;
            allVarsMapping = __wb10;
            annotInfo = __wb11;
            boolAlgVars = __wb12;
            boolAlgVarsCnt = __wb13;
            boolAliasVars = __wb14;
            boolAliasVarsCnt = __wb15;
            boolParamVars = __wb16;
            boolParamVarsCnt = __wb17;
            cacheMap = __wb18;
            clTaskMapping = __wb19;
            derivativeVars = __wb20;
            derivativeVarsCnt = __wb21;
            discreteAlgVars = __wb22;
            discreteAlgVarsCnt = __wb23;
            eqSimCodeVarMapping = __wb24;
            fileName = __wb25;
            flatEqSimCodeVarMapping = __wb26;
            graphIdx = __wb27;
            graphInfo = __wb28;
            inputVars = __wb29;
            inputVarsCnt = __wb30;
            intAlgVars = __wb31;
            intAlgVarsCnt = __wb32;
            intAliasVars = __wb33;
            intAliasVarsCnt = __wb34;
            intParamVars = __wb35;
            intParamVarsCnt = __wb36;
            nodeSccMapping = __wb37;
            notOptimizedVars = __wb38;
            notOptimizedVarsBool = __wb39;
            notOptimizedVarsBoolOpt = __wb40;
            notOptimizedVarsFloat = __wb41;
            notOptimizedVarsFloatOpt = __wb42;
            notOptimizedVarsInt = __wb43;
            notOptimizedVarsIntOpt = __wb44;
            notOptimizedVarsString = __wb45;
            notOptimizedVarsStringOpt = __wb46;
            numCL = __wb47;
            outputVars = __wb48;
            outputVarsCnt = __wb49;
            paramVars = __wb50;
            paramVarsCnt = __wb51;
            scVarCLMapping = __wb52;
            scVarInfos = __wb53;
            scVarSolvedTaskMapping = __wb54;
            scVarUnsolvedTaskMapping = __wb55;
            sccEqMapping = __wb56;
            sccNodeMapping = __wb57;
            simCodeVarTypes = __wb58;
            simCodeVars = __wb59;
            stateVars = __wb60;
            stateVarsCnt = __wb61;
            stringAlgVars = __wb62;
            stringAlgVarsCnt = __wb63;
            stringAliasVars = __wb64;
            stringAliasVarsCnt = __wb65;
            stringParamVars = __wb66;
            stringParamVarsCnt = __wb67;
            taskSolvedVarsMapping = __wb68;
            taskUnsolvedVarsMapping = __wb69;
            threadAttIdx = __wb70;
            tmpMemoryMapOpt = __wb71;
            varCount = __wb72;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("CreateMemoryMap failed!"),
                metamodelica::sourceInfo!("BackEnd/HpcOmMemory.mo"),
            )?;
            Ok((None, iVarToArrayIndexMapping.clone(), iVarToIndexMapping.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oMemoryMap, oVarToArrayIndexMapping, oVarToIndexMapping))
}

fn createCacheMapOptimized(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSimCodeVars: &SimCodeVar::SimVars,
    mut iAllSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    mut iScVarSolvedTaskMapping: metamodelica::Array<i32>,
    mut iScVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iCacheLineSize: i32,
    mut iAllComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iSchedule: &metamodelica::Ref<HpcOmSimCode::Schedule>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iNumberOfThreads: i32,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32;
    let mut cacheMap: CacheMap;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut numCL: i32;
    let mut tasksOfLevels: metamodelica::List<HpcOmSimCode::TaskList>;
    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut threadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>;
    let mut allTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    (oCacheMap, oScVarCLMapping, oNumCL) = (match &**iSchedule {
        HpcOmSimCode::Schedule::LEVELSCHEDULE {
            tasksOfLevels: __esc_tasksOfLevels,
            useFixedAssignments: true,
        } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            metamodelica::print(literal!("Creating optimized cache map for fixed level scheduler\n"));
            scheduleInfo =
                HpcOmScheduler::convertScheduleStrucToInfo(iSchedule, metamodelica::arrayLength(iTaskGraph.clone()))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapLevelFixedOptimized(
                iTaskGraph.clone(),
                iTaskGraphMeta,
                iAllSCVarsMapping.clone(),
                iSimCodeVarTypes.clone(),
                iScVarSolvedTaskMapping.clone(),
                iScVarUnsolvedTaskMapping.clone(),
                iCacheLineSize,
                iAllComponents,
                metamodelica::AsArg::as_arg(&tasksOfLevels),
                iNumberOfThreads,
                scheduleInfo.clone(),
                iTaskSolvedVarsMapping.clone(),
                iTaskUnsolvedVarsMapping.clone(),
                iScVarInfos.clone(),
            )?;
            (cacheMap, scVarCLMapping.clone(), numCL)
        }
        HpcOmSimCode::Schedule::THREADSCHEDULE {
            threadTasks: __esc_threadTasks,
            ..
        } => {
            threadTasks = (*__esc_threadTasks).clone();
            metamodelica::print(literal!("Creating optimized cache map for thread scheduler\n"));
            scheduleInfo =
                HpcOmScheduler::convertScheduleStrucToInfo(iSchedule, metamodelica::arrayLength(iTaskGraph.clone()))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapThreadOptimized(
                iTaskGraph.clone(),
                iTaskGraphMeta,
                iAllSCVarsMapping.clone(),
                iSimCodeVarTypes.clone(),
                iScVarSolvedTaskMapping.clone(),
                iScVarUnsolvedTaskMapping.clone(),
                iCacheLineSize,
                iAllComponents,
                threadTasks.clone(),
                iNumberOfThreads,
                scheduleInfo.clone(),
                iTaskSolvedVarsMapping.clone(),
                iTaskUnsolvedVarsMapping.clone(),
                iScVarInfos.clone(),
            )?;
            (cacheMap, scVarCLMapping.clone(), numCL)
        }
        HpcOmSimCode::Schedule::EMPTYSCHEDULE {
            tasks: HpcOmSimCode::TaskList::SERIALTASKLIST {
                tasks: __esc_allTasks, ..
            },
        } => {
            allTasks = (*__esc_allTasks).clone();
            metamodelica::print(literal!("Creating optimized cache map for empty scheduler\n"));
            threadTasks = arrayCreate(1, allTasks.clone());
            scheduleInfo =
                HpcOmScheduler::convertScheduleStrucToInfo(iSchedule, metamodelica::arrayLength(iTaskGraph.clone()))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapThreadOptimized(
                iTaskGraph.clone(),
                iTaskGraphMeta,
                iAllSCVarsMapping.clone(),
                iSimCodeVarTypes.clone(),
                iScVarSolvedTaskMapping.clone(),
                iScVarUnsolvedTaskMapping.clone(),
                iCacheLineSize,
                iAllComponents,
                threadTasks.clone(),
                1,
                scheduleInfo.clone(),
                iTaskSolvedVarsMapping.clone(),
                iTaskUnsolvedVarsMapping.clone(),
                iScVarInfos.clone(),
            )?;
            (cacheMap, scVarCLMapping.clone(), numCL)
        }
        _ => {
            metamodelica::print(literal!(
                "No optimized cache map for the selected scheduler avaiable. Using default cacheMap!\n"
            ));
            (cacheMap, scVarCLMapping, numCL) = createCacheMapDefault(
                iAllSCVarsMapping.clone(),
                iCacheLineSize,
                iSimCodeVars,
                iScVarSolvedTaskMapping.clone(),
                iSchedulerInfo.clone(),
                iSimCodeVarTypes.clone(),
            )?;
            (cacheMap, scVarCLMapping.clone(), numCL)
        }
    });
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelOptimized(
    mut iAllSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iCacheLineSize: i32,
    mut iAllComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iTasksOfLevels: &metamodelica::List<HpcOmSimCode::TaskList>,
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut numCL: i32;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    cacheMap = CacheMap::CACHEMAP {
        cacheLineSize: iCacheLineSize,
        cacheVariables: metamodelica::nil(),
        cacheLinesFloat: metamodelica::nil(),
        cacheLinesInt: metamodelica::nil(),
        cacheLinesBool: metamodelica::nil(),
    };
    scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVarsMapping.clone()), (-1, -1));
    numCL = 0;
    cacheMapMeta = CacheMapMeta {
        allSCVarsMapping: iAllSCVarsMapping.clone(),
        simCodeVarTypes: iSimCodeVarTypes.clone(),
        scVarCLMapping: scVarCLMapping.clone(),
    };
    (_, cacheMap, cacheMapMeta, numCL) = List::fold1(
        iTasksOfLevels,
        &move |__a0: HpcOmSimCode::TaskList,
               __a1: metamodelica::Array<metamodelica::List<i32>>,
               __a2: (metamodelica::List<i32>, CacheMap, CacheMapMeta, i32)| {
            createCacheMapLevelOptimized0(&__a0, __a1, &__a2)
        },
        iNodeSimCodeVarMapping.clone(),
        (metamodelica::nil(), cacheMap, cacheMapMeta, numCL),
    )?;
    oCacheMap = cacheMap;
    let CacheMapMeta {
        scVarCLMapping: __pa0, ..
    } = cacheMapMeta;
    oScVarCLMapping = metamodelica::Own::own(__pa0);
    oNumCL = numCL;
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelOptimized0(
    mut iLevelTasks: &HpcOmSimCode::TaskList,
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iInfo: &(metamodelica::List<i32>, CacheMap, CacheMapMeta, i32),
) -> Result<(metamodelica::List<i32>, CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (metamodelica::List<i32>, CacheMap, CacheMapMeta, i32);
    let mut createdCL: i32;
    let mut numCL: i32;
    let mut cacheLineSize: i32;
    let mut allCL: metamodelica::List<i32>;
    let mut availableCL: metamodelica::List<i32>;
    let mut availableCLold: metamodelica::List<i32>;
    let mut writtenCL: metamodelica::List<i32>;
    let mut cacheLinesPrevLevel: metamodelica::List<i32>;
    let mut detailedCacheLineInfo: metamodelica::List<(i32, i32)>;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    (cacheLinesPrevLevel, cacheMap, cacheMapMeta, numCL) = iInfo.clone();
    allCL = List::intRange(numCL);
    let CacheMap::CACHEMAP {
        cacheLinesFloat: __pa0,
        cacheLineSize: __pa1,
        ..
    } = (cacheMap.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheLinesFloat = metamodelica::Own::own(__pa0);
    cacheLineSize = metamodelica::Own::own(__pa1);
    availableCLold = List::setDifferenceIntN(&allCL, &cacheLinesPrevLevel, numCL)?;
    detailedCacheLineInfo = createDetailedCacheMapInformation(&availableCLold, cacheLinesFloat, cacheLineSize)?;
    detailedCacheLineInfo = detailedCacheLineInfo.reverse();
    (cacheMap, cacheMapMeta, createdCL, detailedCacheLineInfo) = List::fold1(
        &(getTaskListTasks(iLevelTasks)),
        &move |__a0: metamodelica::Ref<HpcOmSimCode::Task>,
               __a1: metamodelica::Array<metamodelica::List<i32>>,
               __a2: (CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>)| {
            createCacheMapLevelOptimizedForTask(&__a0, __a1, __a2)
        },
        iNodeSimCodeVarMapping.clone(),
        (cacheMap, cacheMapMeta, 0, detailedCacheLineInfo),
    )?;
    availableCL = List::map(detailedCacheLineInfo, &fnptr!(Util::tuple21, _))?;
    writtenCL = List::setDifferenceIntN(&availableCLold, &availableCL, numCL)?;
    writtenCL = listAppend(
        writtenCL,
        if (intLe(numCL + 1, numCL + createdCL)) {
            List::intRange2(numCL + 1, numCL + createdCL)
        } else {
            metamodelica::nil()
        },
    );
    oInfo = (writtenCL, cacheMap, cacheMapMeta, numCL + createdCL);
    Ok(oInfo)
}

fn createCacheMapLevelOptimizedForTask(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iInfo: (CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>),
) -> Result<(CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>);
    let mut nodeIdc: metamodelica::List<i32>;
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>);
    oInfo = (match &**iTask {
        HpcOmSimCode::Task::CALCTASK_LEVEL {
            nodeIdc: __esc_nodeIdc, ..
        } => {
            nodeIdc = (*__esc_nodeIdc).clone();
            tmpInfo = List::fold(
                metamodelica::AsArg::as_arg(&nodeIdc),
                &({
                    let __pe_b1 = -1;
                    let __pe_b2 = iNodeSimCodeVarMapping.clone();
                    move |__pe_a0, __pe_a3| {
                        appendNodeVarsToCacheMap(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_a3)
                    }
                }),
                iInfo,
            )?;
            tmpInfo
        }
        _ => {
            metamodelica::print(literal!("createCacheMapLevelOptimized1: Unsupported task type\n"));
            return Err("fail");
        }
    });
    Ok(oInfo)
}

fn createCacheMapLevelFixedOptimized(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iAllSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    mut iScVarSolvedTaskMapping: metamodelica::Array<i32>,
    mut iScVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iCacheLineSize: i32,
    mut iAllComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iTasksOfLevels: &metamodelica::List<HpcOmSimCode::TaskList>,
    mut iNumberOfThreads: i32,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut handledVariables: metamodelica::Array<bool>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut threadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>;
    let mut sharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>;
    cacheMap = CacheMap::CACHEMAP {
        cacheLineSize: iCacheLineSize,
        cacheVariables: metamodelica::nil(),
        cacheLinesFloat: metamodelica::nil(),
        cacheLinesInt: metamodelica::nil(),
        cacheLinesBool: metamodelica::nil(),
    };
    scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVarsMapping.clone()), (-1, -1));
    handledVariables = arrayCreate(metamodelica::arrayLength(iSimCodeVarTypes.clone()), false);
    oNumCL = 0;
    threadCacheLines = arrayCreate(
        iNumberOfThreads,
        (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
    );
    sharedCacheLines = arrayCreate(
        iNumberOfThreads,
        (
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
        ),
    );
    cacheMapMeta = CacheMapMeta {
        allSCVarsMapping: iAllSCVarsMapping.clone(),
        simCodeVarTypes: iSimCodeVarTypes.clone(),
        scVarCLMapping: scVarCLMapping.clone(),
    };
    (cacheMap, cacheMapMeta, oNumCL, _) = List::fold(
        iTasksOfLevels,
        &({
            let __pe_b1 = iTaskGraph.clone();
            let __pe_b2 = iTaskGraphMeta.clone();
            let __pe_b3 = iNumberOfThreads;
            let __pe_b4 = iScVarInfos.clone();
            let __pe_b5 = iTaskSolvedVarsMapping.clone();
            let __pe_b6 = iTaskUnsolvedVarsMapping.clone();
            let __pe_b7 = handledVariables.clone();
            let __pe_b8 = iSchedulerInfo.clone();
            let __pe_b9 = threadCacheLines.clone();
            let __pe_b10 = sharedCacheLines.clone();
            move |__pe_a0, __pe_a11| {
                createCacheMapLevelFixedOptimizedForLevel(
                    &__pe_a0,
                    __pe_b1.clone(),
                    &__pe_b2,
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_b6.clone(),
                    __pe_b7.clone(),
                    __pe_b8.clone(),
                    __pe_b9.clone(),
                    __pe_b10.clone(),
                    &__pe_a11,
                )
            }
        }),
        (cacheMap, cacheMapMeta, oNumCL, 1),
    )?;
    for mut threadIdx in 1..=iNumberOfThreads {
        cacheMap = createCacheMapFromThreadAndSharedCLs(
            metamodelica::arrayGet(threadCacheLines.clone(), threadIdx)?,
            &(metamodelica::arrayGet(sharedCacheLines.clone(), threadIdx)?),
            cacheMap,
        )?;
    }
    oCacheMap = cacheMap;
    let CacheMapMeta {
        scVarCLMapping: __pa0, ..
    } = cacheMapMeta;
    oScVarCLMapping = metamodelica::Own::own(__pa0);
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelFixedOptimizedForLevel(
    mut iLevelTasks: &HpcOmSimCode::TaskList,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iNumberOfThreads: i32,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iHandledVariables: metamodelica::Array<bool>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iThreadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32, i32),
) -> Result<(CacheMap, CacheMapMeta, i32, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, i32);
    let mut createdCL: i32;
    let mut numCL: i32;
    let mut cacheLineSize: i32;
    let mut level: i32;
    let mut allCL: metamodelica::List<i32>;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    (cacheMap, cacheMapMeta, numCL, level) = iInfo.clone();
    let CacheMap::CACHEMAP {
        cacheVariables: __pa0, ..
    } = (cacheMap.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheVariables = metamodelica::Own::own(__pa0);
    allCL = List::intRange(numCL);
    let CacheMap::CACHEMAP {
        cacheLinesFloat: __pa1,
        cacheLineSize: __pa2,
        ..
    } = (cacheMap.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheLinesFloat = metamodelica::Own::own(__pa1);
    cacheLineSize = metamodelica::Own::own(__pa2);
    (cacheMap, cacheMapMeta, createdCL) = List::fold(
        &(getTaskListTasks(iLevelTasks)),
        &({
            let __pe_b1 = iTaskGraph.clone();
            let __pe_b2 = iTaskGraphMeta.clone();
            let __pe_b3 = iSchedulerInfo.clone();
            let __pe_b4 = iNumberOfThreads;
            let __pe_b5 = level;
            let __pe_b6 = iScVarInfos.clone();
            let __pe_b7 = iTaskSolvedVarsMapping.clone();
            let __pe_b8 = iTaskUnsolvedVarsMapping.clone();
            let __pe_b9 = iHandledVariables.clone();
            let __pe_b10 = iThreadCacheLines.clone();
            let __pe_b11 = iSharedCacheLines.clone();
            move |__pe_a0, __pe_a12| {
                createCacheMapLevelFixedOptimizedForTask(
                    &__pe_a0,
                    __pe_b1.clone(),
                    &__pe_b2,
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                    __pe_b6.clone(),
                    __pe_b7.clone(),
                    __pe_b8.clone(),
                    __pe_b9.clone(),
                    __pe_b10.clone(),
                    __pe_b11.clone(),
                    &__pe_a12,
                )
            }
        }),
        (cacheMap, cacheMapMeta, numCL),
    )?;
    let CacheMap::CACHEMAP {
        cacheVariables: __pa3, ..
    } = (cacheMap.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheVariables = metamodelica::Own::own(__pa3);
    oInfo = (cacheMap, cacheMapMeta, createdCL, level + 1);
    Ok(oInfo)
}

fn createCacheMapLevelFixedOptimizedForTask(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iNumberOfThreads: i32,
    mut iLevel: i32,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iHandledVariables: metamodelica::Array<bool>,
    mut iThreadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32),
) -> Result<(CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut nodeIdc: metamodelica::List<i32>;
    let mut solvedVars: metamodelica::List<i32>;
    let mut unsolvedVars: metamodelica::List<i32>;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32);
    let mut threadIdx: i32;
    let mut numNewCL: i32;
    let mut allSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    oInfo = (::match_deref::match_deref! { match &((&**iTask, iInfo)) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: __esc_nodeIdc, threadIdx: Some(__esc_threadIdx), .. }, (__esc_cacheMap, __esc_cacheMapMeta @ CacheMapMeta { allSCVarsMapping: __esc_allSCVarsMapping, .. }, __esc_numNewCL)) => {
            nodeIdc = (*__esc_nodeIdc).clone();
            threadIdx = (*__esc_threadIdx).clone();
            cacheMap = (*__esc_cacheMap).clone();
            cacheMapMeta = (*__esc_cacheMapMeta).clone();
            allSCVarsMapping = (*__esc_allSCVarsMapping).clone();
            numNewCL = (*__esc_numNewCL).clone();
            solvedVars = List::flatten(List::map(nodeIdc.clone(), &({ let __pe_b0 = iTaskSolvedVarsMapping.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }))?)?;
            unsolvedVars = getUnsolvedVarsByNodeList(metamodelica::AsArg::as_arg(&nodeIdc), metamodelica::arrayLength(iScVarInfos.clone()), iTaskUnsolvedVarsMapping.clone())?;
            tmpInfo = List::fold(&(listAppend(solvedVars, unsolvedVars)), &({ let __pe_b1 = threadIdx.clone(); let __pe_b2 = iScVarInfos.clone(); let __pe_b3 = iHandledVariables.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((metamodelica::List<PartlyFilledCacheLine>, metamodelica::List<PartlyFilledCacheLine>, metamodelica::List<PartlyFilledCacheLine>), (metamodelica::List<CacheLineMap>, metamodelica::List<CacheLineMap>, metamodelica::List<CacheLineMap>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = (std::sync::Arc::new(findMatchingSharedCLLevelfix) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, (i32, i32), metamodelica::Array<((metamodelica::List<PartlyFilledCacheLine>, metamodelica::List<PartlyFilledCacheLine>, metamodelica::List<PartlyFilledCacheLine>), (metamodelica::List<CacheLineMap>, metamodelica::List<CacheLineMap>, metamodelica::List<CacheLineMap>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>); let __pe_b5 = (iLevel, threadIdx.clone()); let __pe_b6: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = (std::sync::Arc::new(createSharedClLevelFix) as std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, (i32, i32)) -> Result<PartlyFilledCacheLine> + 'static>); let __pe_b7 = iThreadCacheLines.clone(); let __pe_b8 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a9| createCacheMapOptimizedForTask1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), &*__pe_b4, __pe_b5.clone(), &*__pe_b6, __pe_b7.clone(), __pe_b8.clone(), &__pe_a9) }), (cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone()))?;
            let CacheMap::CACHEMAP { cacheVariables: __pa0, .. } = (Util::tuple31(tmpInfo.clone())) else { return Err("pattern mismatch") };
            cacheVariables = metamodelica::Own::own(__pa0);
            tmpInfo
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: __esc_nodeIdc, threadIdx: None, .. }, _) => {
            nodeIdc = (*__esc_nodeIdc).clone();
            metamodelica::print(literal!("createCacheMapLevelOptimized1: Calctask without threadIdx given\n"));
            return Err("fail")
        },
        _ => {
            metamodelica::print(literal!("createCacheMapLevelOptimized1: Unsupported task type\n"));
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn getUnsolvedVarsByNodeList(
    mut iNodeList: &metamodelica::List<i32>,
    mut iVarCount: i32,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut oUnsolvedVars: metamodelica::List<i32>;
    let mut varMarks: metamodelica::Array<bool>;
    let mut nodeIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut nodeUnsolvedVars: metamodelica::List<i32>;
    let mut tmpUnsolvedVars: metamodelica::List<i32> = metamodelica::nil();
    varMarks = arrayCreate(iVarCount, false);
    for mut nodeIdx in &**iNodeList {
        let mut nodeIdx = nodeIdx.clone();
        nodeUnsolvedVars = metamodelica::arrayGet(iTaskUnsolvedVarsMapping.clone(), nodeIdx)?;
        for mut varIdx in &*nodeUnsolvedVars {
            let mut varIdx = varIdx.clone();
            if boolNot(metamodelica::arrayGet(varMarks.clone(), varIdx)?) {
                tmpUnsolvedVars = metamodelica::cons(varIdx, tmpUnsolvedVars);
                varMarks = metamodelica::arrayUpdate(varMarks.clone(), varIdx, true)?;
            }
        }
    }
    oUnsolvedVars = tmpUnsolvedVars;
    Ok(oUnsolvedVars)
}

fn createCacheMapThreadOptimized(
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iAllSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    mut iScVarSolvedTaskMapping: metamodelica::Array<i32>,
    mut iScVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iCacheLineSize: i32,
    mut iAllComponents: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut iThreadTasks: metamodelica::Array<metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>>,
    mut iNumberOfThreads: i32,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32;
    let mut threadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>;
    let mut sharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>;
    let mut tmpCacheInfo: (CacheMap, CacheMapMeta, i32);
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut handledVariables: metamodelica::Array<bool>;
    threadCacheLines = arrayCreate(
        iNumberOfThreads,
        (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
    );
    sharedCacheLines = arrayCreate(
        iNumberOfThreads,
        (
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()),
        ),
    );
    handledVariables = arrayCreate(metamodelica::arrayLength(iSimCodeVarTypes.clone()), false);
    cacheMap = CacheMap::CACHEMAP {
        cacheLineSize: iCacheLineSize,
        cacheVariables: metamodelica::nil(),
        cacheLinesFloat: metamodelica::nil(),
        cacheLinesInt: metamodelica::nil(),
        cacheLinesBool: metamodelica::nil(),
    };
    scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVarsMapping.clone()), (-1, -1));
    oNumCL = 0;
    cacheMapMeta = CacheMapMeta {
        allSCVarsMapping: iAllSCVarsMapping.clone(),
        simCodeVarTypes: iSimCodeVarTypes.clone(),
        scVarCLMapping: scVarCLMapping.clone(),
    };
    tmpCacheInfo = (cacheMap, cacheMapMeta.clone(), oNumCL);
    for mut threadIdx in 1..=iNumberOfThreads {
        (cacheMap, cacheMapMeta, oNumCL) = List::fold(
            &(metamodelica::arrayGet(iThreadTasks.clone(), threadIdx)?),
            &({
                let __pe_b1 = iTaskGraph.clone();
                let __pe_b2 = iTaskGraphMeta.clone();
                let __pe_b3 = iSchedulerInfo.clone();
                let __pe_b4 = iTaskSolvedVarsMapping.clone();
                let __pe_b5 = iTaskUnsolvedVarsMapping.clone();
                let __pe_b6 = handledVariables.clone();
                let __pe_b7 = iNumberOfThreads;
                let __pe_b8: Arc<
                    dyn ::std::ops::Fn(
                            i32,
                            i32,
                            i32,
                            i32,
                            _,
                            metamodelica::Array<(
                                (
                                    metamodelica::List<PartlyFilledCacheLine>,
                                    metamodelica::List<PartlyFilledCacheLine>,
                                    metamodelica::List<PartlyFilledCacheLine>,
                                ),
                                (
                                    metamodelica::List<CacheLineMap>,
                                    metamodelica::List<CacheLineMap>,
                                    metamodelica::List<CacheLineMap>,
                                ),
                            )>,
                        ) -> Result<Option<(PartlyFilledCacheLine, i32)>>
                        + 'static,
                > = (std::sync::Arc::new(findMatchingSharedCLThread)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                                metamodelica::Array<(
                                    (
                                        metamodelica::List<PartlyFilledCacheLine>,
                                        metamodelica::List<PartlyFilledCacheLine>,
                                        metamodelica::List<PartlyFilledCacheLine>,
                                    ),
                                    (
                                        metamodelica::List<CacheLineMap>,
                                        metamodelica::List<CacheLineMap>,
                                        metamodelica::List<CacheLineMap>,
                                    ),
                                )>,
                            ) -> Result<Option<(PartlyFilledCacheLine, i32)>>
                            + 'static,
                    >);
                let __pe_b9 = 0;
                let __pe_b10: Arc<
                    dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine>
                        + 'static,
                > = (std::sync::Arc::new(fnptr!(
                    createSharedClThread,
                    Option<PartlyFilledCacheLine>,
                    CacheLineMap,
                    i32
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                Option<PartlyFilledCacheLine>,
                                CacheLineMap,
                                i32,
                            ) -> Result<PartlyFilledCacheLine>
                            + 'static,
                    >);
                let __pe_b11 = threadCacheLines.clone();
                let __pe_b12 = sharedCacheLines.clone();
                let __pe_b13 = iScVarInfos.clone();
                move |__pe_a0, __pe_a14| {
                    createCacheMapOptimizedForTask(
                        &__pe_a0,
                        __pe_b1.clone(),
                        &__pe_b2,
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_b5.clone(),
                        __pe_b6.clone(),
                        __pe_b7.clone(),
                        __pe_b8.clone(),
                        __pe_b9.clone(),
                        __pe_b10.clone(),
                        __pe_b11.clone(),
                        __pe_b12.clone(),
                        __pe_b13.clone(),
                        __pe_a14,
                    )
                }
            }),
            tmpCacheInfo,
        )?;
        cacheMap = createCacheMapFromThreadAndSharedCLs(
            metamodelica::arrayGet(threadCacheLines.clone(), threadIdx)?,
            &(metamodelica::arrayGet(sharedCacheLines.clone(), threadIdx)?),
            cacheMap,
        )?;
        tmpCacheInfo = (cacheMap, cacheMapMeta.clone(), oNumCL);
    }
    oCacheMap = Util::tuple31(tmpCacheInfo);
    let CacheMapMeta {
        scVarCLMapping: __pa0, ..
    } = cacheMapMeta;
    oScVarCLMapping = metamodelica::Own::own(__pa0);
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapOptimizedForTask<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iTask: &metamodelica::Ref<HpcOmSimCode::Task>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphMeta: &HpcOmTaskGraph::TaskGraphMeta,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iHandledVariables: metamodelica::Array<bool>,
    mut iNumberOfThreads: i32,
    mut iSharedClSelectFunction: Arc<
        dyn ::std::ops::Fn(
                i32,
                i32,
                i32,
                i32,
                T,
                metamodelica::Array<(
                    (
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                    ),
                    (
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                    ),
                )>,
            ) -> Result<Option<(PartlyFilledCacheLine, i32)>>
            + 'static,
    >,
    mut iCompareFuncArgument: T,
    mut iFactoryMethod: Arc<
        dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static,
    >,
    mut iThreadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iInfo: (CacheMap, CacheMapMeta, i32),
) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                i32,
                i32,
                i32,
                i32,
                T,
                metamodelica::Array<(
                    (
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                    ),
                    (
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                    ),
                )>,
            ) -> Result<Option<(PartlyFilledCacheLine, i32)>>
            + 'static,
    >;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static,
    >;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut threadIdx: i32;
    let mut taskIdx: i32;
    let mut solvedVars: metamodelica::List<i32>;
    let mut unsolvedVars: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut numOfCLs: i32;
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32);
    let mut allSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>;
    oInfo = (::match_deref::match_deref! { match &((iTask.clone(), iInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_taskIdx, threadIdx: __esc_threadIdx, .. }, (__esc_cacheMap, __esc_cacheMapMeta @ CacheMapMeta { allSCVarsMapping: __esc_allSCVarsMapping, .. }, __esc_numOfCLs)) => {
            taskIdx = (*__esc_taskIdx).clone();
            threadIdx = (*__esc_threadIdx).clone();
            cacheMap = (*__esc_cacheMap).clone();
            cacheMapMeta = (*__esc_cacheMapMeta).clone();
            allSCVarsMapping = (*__esc_allSCVarsMapping).clone();
            numOfCLs = (*__esc_numOfCLs).clone();
            solvedVars = metamodelica::arrayGet(iTaskSolvedVarsMapping.clone(), taskIdx.clone())?;
            unsolvedVars = metamodelica::arrayGet(iTaskUnsolvedVarsMapping.clone(), taskIdx.clone())?;
            vars = List::sort(listAppend(solvedVars, unsolvedVars), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            tmpInfo = List::fold(&vars, &({ let __pe_b1 = threadIdx.clone(); let __pe_b2 = iScVarInfos.clone(); let __pe_b3 = iHandledVariables.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((metamodelica::List<PartlyFilledCacheLine>, metamodelica::List<PartlyFilledCacheLine>, metamodelica::List<PartlyFilledCacheLine>), (metamodelica::List<CacheLineMap>, metamodelica::List<CacheLineMap>, metamodelica::List<CacheLineMap>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = iSharedClSelectFunction.clone(); let __pe_b5 = iCompareFuncArgument; let __pe_b6: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = iFactoryMethod.clone(); let __pe_b7 = iThreadCacheLines.clone(); let __pe_b8 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a9| createCacheMapOptimizedForTask1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), &*__pe_b4, __pe_b5.clone(), &*__pe_b6, __pe_b7.clone(), __pe_b8.clone(), &__pe_a9) }), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
            tmpInfo
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: _, .. }, (__esc_cacheMap, __esc_cacheMapMeta @ CacheMapMeta { allSCVarsMapping: __esc_allSCVarsMapping, .. }, __esc_numOfCLs)) => {
            cacheMap = (*__esc_cacheMap).clone();
            cacheMapMeta = (*__esc_cacheMapMeta).clone();
            allSCVarsMapping = (*__esc_allSCVarsMapping).clone();
            numOfCLs = (*__esc_numOfCLs).clone();
            iInfo
        },
        _ => {
            metamodelica::print(literal!("createCacheMapThreadOptimizedForTask failed!\n"));
            iInfo
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn createCacheMapOptimizedForTask1<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iScVar: i32,
    mut iThreadIdx: i32,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iHandledVariables: metamodelica::Array<bool>,
    mut iSharedClSelectFunction: &dyn ::std::ops::Fn(
        i32,
        i32,
        i32,
        i32,
        T,
        metamodelica::Array<(
            (
                metamodelica::List<PartlyFilledCacheLine>,
                metamodelica::List<PartlyFilledCacheLine>,
                metamodelica::List<PartlyFilledCacheLine>,
            ),
            (
                metamodelica::List<CacheLineMap>,
                metamodelica::List<CacheLineMap>,
                metamodelica::List<CacheLineMap>,
            ),
        )>,
    ) -> Result<Option<(PartlyFilledCacheLine, i32)>>,
    mut iCompareFuncArgument: T,
    mut iFactoryMethod: &dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine>,
    mut iThreadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32),
) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                i32,
                i32,
                i32,
                i32,
                T,
                metamodelica::Array<(
                    (
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                    ),
                    (
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                    ),
                )>,
            ) -> Result<Option<(PartlyFilledCacheLine, i32)>>
            + 'static,
    >;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static,
    >;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut isShared: bool;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut numOfCLs: i32;
    let mut ownerThread: i32;
    (cacheMap, cacheMapMeta, numOfCLs) = iInfo.clone();
    let ScVarInfo {
        ownerThread: __pa0,
        isShared: __pa1,
    } = metamodelica::arrayGet(iScVarInfos.clone(), iScVar)?;
    ownerThread = metamodelica::Own::own(__pa0);
    isShared = metamodelica::Own::own(__pa1);
    if boolAnd(
        boolNot(boolAnd(intEq(ownerThread, -1), isShared)),
        boolNot(metamodelica::arrayGet(iHandledVariables.clone(), iScVar)?),
    ) {
        if isShared {
            (cacheMap, cacheMapMeta, numOfCLs) = addVarsToSharedCL(
                &(list![iScVar]),
                iSharedClSelectFunction,
                iFactoryMethod,
                iThreadIdx,
                iCompareFuncArgument,
                iSharedCacheLines.clone(),
                &((cacheMap, cacheMapMeta, numOfCLs)),
            )?;
        } else {
            (cacheMap, cacheMapMeta, numOfCLs) = addVarsToThreadCL(
                &(list![iScVar]),
                iThreadIdx,
                iThreadCacheLines.clone(),
                &((cacheMap, cacheMapMeta, numOfCLs)),
            )?;
        }
    }
    metamodelica::arrayUpdate(iHandledVariables.clone(), iScVar, true)?;
    oInfo = (cacheMap, cacheMapMeta, numOfCLs);
    Ok(oInfo)
}

fn createVarInfos(
    mut iScVarSolvedTaskMapping: metamodelica::Array<i32>,
    mut iScVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::Array<ScVarInfo>> {
    let mut oVarInfos: metamodelica::Array<ScVarInfo>;
    let mut tmpVarInfos: metamodelica::Array<ScVarInfo>;
    let mut scVarIdx: i32 = 0;
    let mut numberOfScVars: i32;
    numberOfScVars = metamodelica::arrayLength(iScVarSolvedTaskMapping.clone());
    tmpVarInfos = arrayCreate(
        numberOfScVars,
        ScVarInfo {
            ownerThread: -1,
            isShared: false,
        },
    );
    for mut scVarIdx in 1..=numberOfScVars {
        tmpVarInfos = metamodelica::arrayUpdate(
            tmpVarInfos.clone(),
            scVarIdx,
            getVarInfoByScVarIdx(
                scVarIdx,
                iScVarSolvedTaskMapping.clone(),
                iScVarUnsolvedTaskMapping.clone(),
                iSchedulerInfo.clone(),
            )?,
        )?;
    }
    oVarInfos = tmpVarInfos.clone();
    Ok(oVarInfos)
}

fn getVarInfoByScVarIdx(
    mut iScVarIdx: i32,
    mut iScVarSolvedTaskMapping: metamodelica::Array<i32>,
    mut iScVarUnsolvedTaskMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<ScVarInfo> {
    let mut oVarInfo: ScVarInfo;
    let mut solvingThreadIdx: i32;
    let mut solvingTaskIdx: i32;
    let mut listLen: i32;
    let mut owner: i32 = -1;
    let mut isShared: bool = false;
    let mut threads: metamodelica::List<i32> = metamodelica::nil();
    let mut unsolvingThreadIdc: metamodelica::List<i32>;
    let mut unsolvingTaskIdc: metamodelica::List<i32>;
    solvingTaskIdx = metamodelica::arrayGet(iScVarSolvedTaskMapping.clone(), iScVarIdx)?;
    unsolvingTaskIdc = metamodelica::arrayGet(iScVarUnsolvedTaskMapping.clone(), iScVarIdx)?;
    if intGt(solvingTaskIdx, 0) {
        solvingThreadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), solvingTaskIdx)?);
        owner = solvingThreadIdx;
        threads = metamodelica::cons(owner, threads);
    }
    listLen = ((unsolvingTaskIdc).len() as i32);
    unsolvingThreadIdc = List::map(
        List::map(
            unsolvingTaskIdc,
            &({
                let __pe_b0 = iSchedulerInfo.clone();
                move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1)
            }),
        )?,
        &fnptr!(Util::tuple31, _),
    )?;
    if intEq(listLen, 1) {
        if intLt(owner, 0) {
            owner = (unsolvingThreadIdc).head().cloned()?;
            threads = metamodelica::cons(owner, threads);
        } else {
            isShared = true;
        }
    }
    if intGt(listLen, 1) {
        threads = List::unique(&(listAppend(unsolvingThreadIdc, threads)));
        isShared = true;
    }
    oVarInfo = ScVarInfo {
        ownerThread: owner,
        isShared: isShared,
    };
    Ok(oVarInfo)
}

fn addVarsToThreadCL(
    mut iNodeVars: &metamodelica::List<i32>,
    mut iThreadIdx: i32,
    mut iThreadCacheLines: metamodelica::Array<(
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
        metamodelica::List<CacheLineMap>,
    )>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32),
) -> Result<(CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut lastCL: CacheLineMap;
    let mut cacheVariable: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut allSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut varIdx: i32 = 0;
    let mut varDataType: i32;
    let mut varNumBytesRequired: i32;
    let mut numCLs: i32;
    let mut cacheLineSize: i32;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut fullCLs: metamodelica::List<CacheLineMap>;
    let mut threadCacheLines: metamodelica::List<CacheLineMap>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut lastCLidx: i32;
    let mut lastCLnumBytesFree: i32;
    let mut lastCLentries: metamodelica::List<CacheLineEntry>;
    let mut varEntry: CacheLineEntry;
    let mut cacheVarName: metamodelica::Ref<DAE::ComponentRef>;
    let mut threadCacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut threadCacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut threadCacheLinesBool: metamodelica::List<CacheLineMap>;
    let (
        CacheMap::CACHEMAP {
            cacheLineSize: __pa0,
            cacheVariables: __pa1,
            cacheLinesFloat: __pa2,
            cacheLinesInt: __pa3,
            cacheLinesBool: __pa4,
        },
        CacheMapMeta {
            allSCVarsMapping: __pa5,
            simCodeVarTypes: __pa6,
            scVarCLMapping: __pa7,
        },
        __pa8,
    ) = (iInfo.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheLineSize = metamodelica::Own::own(__pa0);
    cacheVariables = metamodelica::Own::own(__pa1);
    cacheLinesFloat = metamodelica::Own::own(__pa2);
    cacheLinesInt = metamodelica::Own::own(__pa3);
    cacheLinesBool = metamodelica::Own::own(__pa4);
    allSCVarsMapping = metamodelica::Own::own(__pa5);
    simCodeVarTypes = metamodelica::Own::own(__pa6);
    scVarCLMapping = metamodelica::Own::own(__pa7);
    numCLs = metamodelica::Own::own(__pa8);
    for mut varIdx in &**iNodeVars {
        let mut varIdx = varIdx.clone();
        (varDataType, varNumBytesRequired, _) = metamodelica::arrayGet(simCodeVarTypes.clone(), varIdx)?;
        (
            threadCacheLinesFloat,
            threadCacheLinesInt,
            threadCacheLinesBool,
            threadCacheLines,
        ) = getCacheLineForVarType(
            varDataType,
            &(metamodelica::arrayGet(iThreadCacheLines.clone(), iThreadIdx)?),
        );
        if !((threadCacheLines).is_empty()) {
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(threadCacheLines) {
                Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: __pa10 } => (__pa9.clone(), __pa10.clone()),
                _ => return Err("pattern mismatch"),
            } };
            lastCL = metamodelica::Own::own(__pa9);
            fullCLs = metamodelica::Own::own(__pa10);
        } else {
            lastCLidx = numCLs + 1;
            lastCLnumBytesFree = cacheLineSize;
            lastCLentries = metamodelica::nil();
            lastCL = CacheLineMap {
                idx: lastCLidx,
                numBytesFree: lastCLnumBytesFree,
                entries: lastCLentries,
            };
            numCLs = numCLs + 1;
            fullCLs = metamodelica::nil();
        }
        let CacheLineMap {
            idx: __pa11,
            numBytesFree: __pa12,
            entries: __pa13,
        } = &lastCL;
        lastCLidx = metamodelica::Own::own(__pa11);
        lastCLnumBytesFree = metamodelica::Own::own(__pa12);
        lastCLentries = metamodelica::Own::own(__pa13);
        if intLt(lastCLnumBytesFree, varNumBytesRequired) {
            fullCLs = metamodelica::cons(lastCL, fullCLs);
            lastCLidx = numCLs + 1;
            lastCLnumBytesFree = cacheLineSize;
            lastCLentries = metamodelica::nil();
            lastCL = CacheLineMap {
                idx: lastCLidx,
                numBytesFree: lastCLnumBytesFree,
                entries: lastCLentries.clone(),
            };
            numCLs = numCLs + 1;
        }
        let (__pa15, __pa14) = ::match_deref::match_deref! { match &(metamodelica::arrayGet(allSCVarsMapping.clone(), varIdx)?) {
            Some(__pa15 @ Deref @ SimCodeVar::SimVar { name: __pa14, .. }) => (__pa15.clone(), __pa14.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cacheVarName = metamodelica::Own::own(__pa14);
        cacheVariable = metamodelica::Own::own(__pa15);
        cacheVariables = metamodelica::cons(cacheVariable, cacheVariables);
        scVarCLMapping = metamodelica::arrayUpdate(scVarCLMapping.clone(), varIdx, (lastCLidx, varDataType))?;
        varEntry = CacheLineEntry {
            start: cacheLineSize - lastCLnumBytesFree,
            dataType: varDataType,
            size: varNumBytesRequired,
            scVarIdx: ((cacheVariables).len() as i32),
            threadOwner: iThreadIdx,
        };
        lastCL = CacheLineMap {
            idx: lastCLidx,
            numBytesFree: lastCLnumBytesFree - varNumBytesRequired,
            entries: metamodelica::cons(varEntry, lastCLentries),
        };
        metamodelica::arrayUpdate(
            iThreadCacheLines.clone(),
            iThreadIdx,
            contractCacheLineForVarType(
                varDataType,
                threadCacheLinesFloat,
                threadCacheLinesInt,
                threadCacheLinesBool,
                metamodelica::cons(lastCL, fullCLs),
            ),
        )?;
    }
    oInfo = (
        CacheMap::CACHEMAP {
            cacheLineSize: cacheLineSize,
            cacheVariables: cacheVariables,
            cacheLinesFloat: cacheLinesFloat,
            cacheLinesInt: cacheLinesInt,
            cacheLinesBool: cacheLinesBool,
        },
        CacheMapMeta {
            allSCVarsMapping: allSCVarsMapping.clone(),
            simCodeVarTypes: simCodeVarTypes.clone(),
            scVarCLMapping: scVarCLMapping.clone(),
        },
        numCLs,
    );
    Ok(oInfo)
}

fn getCacheLineForVarType(
    mut iVarDataType: i32,
    mut iCacheLinesForTypes: &CacheLines,
) -> (
    metamodelica::List<CacheLineMap>,
    metamodelica::List<CacheLineMap>,
    metamodelica::List<CacheLineMap>,
    metamodelica::List<CacheLineMap>,
) {
    let mut oCacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut oCacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut oCacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut oVarCacheLines: metamodelica::List<CacheLineMap> = metamodelica::nil();
    (oCacheLinesFloat, oCacheLinesInt, oCacheLinesBool) = iCacheLinesForTypes.clone();
    if intEq(iVarDataType, VARDATATYPE_FLOAT.clone()) {
        (oVarCacheLines, _, _) = iCacheLinesForTypes.clone();
    } else {
        if intEq(iVarDataType, VARDATATYPE_INTEGER.clone()) {
            (_, oVarCacheLines, _) = iCacheLinesForTypes.clone();
        } else {
            if intEq(iVarDataType, VARDATATYPE_BOOLEAN.clone()) {
                (_, _, oVarCacheLines) = iCacheLinesForTypes.clone();
            } else {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "getCacheLineForVarType: Found Variable with unknown type ( "
                    ));
                    __mm_s.push_str(&*intString(iVarDataType));
                    __mm_s.push_str(&*literal!(")!\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
    }
    (oCacheLinesFloat, oCacheLinesInt, oCacheLinesBool, oVarCacheLines)
}

fn contractCacheLineForVarType(
    mut iVarDataType: i32,
    mut iCacheLinesFloat: metamodelica::List<CacheLineMap>,
    mut iCacheLinesInt: metamodelica::List<CacheLineMap>,
    mut iCacheLinesBool: metamodelica::List<CacheLineMap>,
    mut iVarCacheLines: metamodelica::List<CacheLineMap>,
) -> CacheLines {
    let mut oContractedCacheLines: CacheLines = (
        iCacheLinesFloat.clone(),
        iCacheLinesInt.clone(),
        iCacheLinesBool.clone(),
    );
    if intEq(iVarDataType, VARDATATYPE_FLOAT.clone()) {
        oContractedCacheLines = (iVarCacheLines, iCacheLinesInt, iCacheLinesBool);
    } else {
        if intEq(iVarDataType, VARDATATYPE_INTEGER.clone()) {
            oContractedCacheLines = (iCacheLinesFloat, iVarCacheLines, iCacheLinesBool);
        } else {
            if intEq(iVarDataType, VARDATATYPE_BOOLEAN.clone()) {
                oContractedCacheLines = (iCacheLinesFloat, iCacheLinesInt, iVarCacheLines);
            }
        }
    }
    oContractedCacheLines
}

fn addVarsToSharedCL<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iNodeVars: &metamodelica::List<i32>,
    mut iSharedClSelectFunction: &dyn ::std::ops::Fn(
        i32,
        i32,
        i32,
        i32,
        T,
        metamodelica::Array<(
            (
                metamodelica::List<PartlyFilledCacheLine>,
                metamodelica::List<PartlyFilledCacheLine>,
                metamodelica::List<PartlyFilledCacheLine>,
            ),
            (
                metamodelica::List<CacheLineMap>,
                metamodelica::List<CacheLineMap>,
                metamodelica::List<CacheLineMap>,
            ),
        )>,
    ) -> Result<Option<(PartlyFilledCacheLine, i32)>>,
    mut iFactoryMethod: &dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine>,
    mut iThreadIdx: i32,
    mut iCompareFuncArgument: T,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32),
) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                i32,
                i32,
                i32,
                i32,
                T,
                metamodelica::Array<(
                    (
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                        metamodelica::List<PartlyFilledCacheLine>,
                    ),
                    (
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                        metamodelica::List<CacheLineMap>,
                    ),
                )>,
            ) -> Result<Option<(PartlyFilledCacheLine, i32)>>
            + 'static,
    >;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static,
    >;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut allSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut varIdx: i32 = 0;
    let mut varDataType: i32;
    let mut numOfCLs: i32;
    let mut cacheLineSize: i32;
    let mut varSize: i32;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta;
    let mut matchedCacheLine: Option<(PartlyFilledCacheLine, i32)>;
    let (
        ref __pa3 @ CacheMap::CACHEMAP {
            cacheLineSize: ref __pa0,
            cacheVariables: ref __pa1,
            cacheLinesFloat: ref __pa2,
            ..
        },
        ref __pa7 @ CacheMapMeta {
            allSCVarsMapping: ref __pa4,
            simCodeVarTypes: ref __pa5,
            scVarCLMapping: ref __pa6,
        },
        __pa8,
    ) = (iInfo.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheLineSize = metamodelica::Own::own(__pa0);
    cacheVariables = metamodelica::Own::own(__pa1);
    cacheLinesFloat = metamodelica::Own::own(__pa2);
    cacheMap = metamodelica::Own::own(__pa3);
    allSCVarsMapping = metamodelica::Own::own(__pa4);
    simCodeVarTypes = metamodelica::Own::own(__pa5);
    scVarCLMapping = metamodelica::Own::own(__pa6);
    cacheMapMeta = metamodelica::Own::own(__pa7);
    numOfCLs = metamodelica::Own::own(__pa8);
    for mut varIdx in &**iNodeVars {
        let mut varIdx = varIdx.clone();
        (varDataType, varSize, _) = metamodelica::arrayGet(simCodeVarTypes.clone(), varIdx)?;
        matchedCacheLine = iSharedClSelectFunction(
            varIdx,
            varSize,
            varDataType,
            iThreadIdx,
            iCompareFuncArgument.clone(),
            iSharedCacheLines.clone(),
        )?;
        (cacheMap, cacheMapMeta, numOfCLs) = addVarsToSharedCL0(
            matchedCacheLine,
            varIdx,
            iFactoryMethod,
            iCompareFuncArgument.clone(),
            iThreadIdx,
            iSharedCacheLines.clone(),
            &((cacheMap, cacheMapMeta, numOfCLs)),
        )?;
    }
    oInfo = (cacheMap, cacheMapMeta, numOfCLs);
    Ok(oInfo)
}

fn addVarsToSharedCL0<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>,
    mut iVarIdx: i32,
    mut iFactoryMethod: &dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine>,
    mut iAdditionalArgument: T,
    mut iThreadIdx: i32,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32),
) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static,
    >;

    let mut oInfo: (CacheMap, CacheMapMeta, i32);
    let mut threadPartlyFilledCacheLines: PartlyFilledCacheLines;
    let mut partlyFilledClFloat: metamodelica::List<PartlyFilledCacheLine>;
    let mut partlyFilledClInt: metamodelica::List<PartlyFilledCacheLine>;
    let mut partlyFilledClBool: metamodelica::List<PartlyFilledCacheLine>;
    let mut threadFullyFilledCacheLines: CacheLines;
    let mut fullyFilledClFloat: metamodelica::List<CacheLineMap>;
    let mut fullyFilledClInt: metamodelica::List<CacheLineMap>;
    let mut fullyFilledClBool: metamodelica::List<CacheLineMap>;
    let mut allSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut partlyFilledCacheLine: PartlyFilledCacheLine;
    let mut partlyFilledCacheLineOption: Option<PartlyFilledCacheLine>;
    let mut matchedClIndex: i32;
    let mut numOfCLs: i32;
    let mut clMapIdx: i32;
    let mut clMapNumBytesFree: i32;
    let mut varDataType: i32;
    let mut varSize: i32;
    let mut cacheLineSize: i32;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut clMapEntries: metamodelica::List<CacheLineEntry>;
    let mut entry: CacheLineEntry;
    let mut cacheLineMap: CacheLineMap;
    let mut cacheVariable: metamodelica::Ref<SimCodeVar::SimVar>;
    let (
        CacheMap::CACHEMAP {
            cacheLineSize: __pa0,
            cacheVariables: __pa1,
            cacheLinesFloat: __pa2,
            cacheLinesInt: __pa3,
            cacheLinesBool: __pa4,
        },
        CacheMapMeta {
            allSCVarsMapping: __pa5,
            simCodeVarTypes: __pa6,
            scVarCLMapping: __pa7,
        },
        __pa8,
    ) = (iInfo.clone())
    else {
        return Err("pattern mismatch");
    };
    cacheLineSize = metamodelica::Own::own(__pa0);
    cacheVariables = metamodelica::Own::own(__pa1);
    cacheLinesFloat = metamodelica::Own::own(__pa2);
    cacheLinesInt = metamodelica::Own::own(__pa3);
    cacheLinesBool = metamodelica::Own::own(__pa4);
    allSCVarsMapping = metamodelica::Own::own(__pa5);
    simCodeVarTypes = metamodelica::Own::own(__pa6);
    scVarCLMapping = metamodelica::Own::own(__pa7);
    numOfCLs = metamodelica::Own::own(__pa8);
    (varDataType, varSize, _) = metamodelica::arrayGet(simCodeVarTypes.clone(), iVarIdx)?;
    (threadPartlyFilledCacheLines, threadFullyFilledCacheLines) =
        metamodelica::arrayGet(iSharedCacheLines.clone(), iThreadIdx)?;
    (partlyFilledClFloat, partlyFilledClInt, partlyFilledClBool) = threadPartlyFilledCacheLines;
    (fullyFilledClFloat, fullyFilledClInt, fullyFilledClBool) = threadFullyFilledCacheLines;
    if (iMatchedCacheLine).is_some() {
        clMapIdx = numOfCLs;
        let (__pa9, __pa10) = ::match_deref::match_deref! { match &(iMatchedCacheLine) {
            Some((__pa9, __pa10)) => (__pa9.clone(), __pa10.clone()),
            _ => return Err("pattern mismatch"),
        } };
        partlyFilledCacheLine = metamodelica::Own::own(__pa9);
        matchedClIndex = metamodelica::Own::own(__pa10);
        partlyFilledCacheLineOption = Some(partlyFilledCacheLine.clone());
        let CacheLineMap {
            idx: __pa11,
            numBytesFree: __pa12,
            entries: __pa13,
        } = getCacheLineMapOfPartlyFilledCacheLine(&partlyFilledCacheLine);
        clMapIdx = metamodelica::Own::own(__pa11);
        clMapNumBytesFree = metamodelica::Own::own(__pa12);
        clMapEntries = metamodelica::Own::own(__pa13);
    } else {
        numOfCLs = numOfCLs + 1;
        partlyFilledCacheLineOption = None;
        clMapIdx = numOfCLs;
        clMapNumBytesFree = cacheLineSize;
        clMapEntries = metamodelica::nil();
        matchedClIndex = -1;
    }
    clMapNumBytesFree = clMapNumBytesFree - varSize;
    let __pa14 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(allSCVarsMapping.clone(), iVarIdx)?) {
        Some(__pa14) => __pa14.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cacheVariable = metamodelica::Own::own(__pa14);
    cacheVariables = metamodelica::cons(cacheVariable, cacheVariables);
    entry = CacheLineEntry {
        start: cacheLineSize - clMapNumBytesFree - varSize,
        dataType: varDataType,
        size: varSize,
        scVarIdx: ((cacheVariables).len() as i32),
        threadOwner: iThreadIdx,
    };
    cacheLineMap = CacheLineMap {
        idx: clMapIdx,
        numBytesFree: clMapNumBytesFree,
        entries: metamodelica::cons(entry, clMapEntries),
    };
    partlyFilledCacheLine = iFactoryMethod(partlyFilledCacheLineOption, cacheLineMap.clone(), iAdditionalArgument)?;
    scVarCLMapping = metamodelica::arrayUpdate(scVarCLMapping.clone(), iVarIdx, (clMapIdx, varDataType))?;
    if intEq(clMapNumBytesFree, 0) {
        if intEq(varDataType, VARDATATYPE_FLOAT.clone()) {
            partlyFilledClFloat = listDelete(partlyFilledClFloat, matchedClIndex)?;
            fullyFilledClFloat = metamodelica::cons(cacheLineMap, fullyFilledClFloat);
        } else {
            if intEq(varDataType, VARDATATYPE_INTEGER.clone()) {
                partlyFilledClInt = listDelete(partlyFilledClInt, matchedClIndex)?;
                fullyFilledClInt = metamodelica::cons(cacheLineMap, fullyFilledClInt);
            } else {
                partlyFilledClBool = listDelete(partlyFilledClBool, matchedClIndex)?;
                fullyFilledClBool = metamodelica::cons(cacheLineMap, fullyFilledClBool);
            }
        }
    } else {
        if intNe(matchedClIndex, -1) {
            if intEq(varDataType, VARDATATYPE_FLOAT.clone()) {
                partlyFilledClFloat = List::set(partlyFilledClFloat, matchedClIndex, partlyFilledCacheLine)?;
            } else {
                if intEq(varDataType, VARDATATYPE_INTEGER.clone()) {
                    partlyFilledClInt = List::set(partlyFilledClInt, matchedClIndex, partlyFilledCacheLine)?;
                } else {
                    partlyFilledClBool = List::set(partlyFilledClBool, matchedClIndex, partlyFilledCacheLine)?;
                }
            }
        } else {
            if intEq(varDataType, VARDATATYPE_FLOAT.clone()) {
                partlyFilledClFloat = metamodelica::cons(partlyFilledCacheLine, partlyFilledClFloat);
            } else {
                if intEq(varDataType, VARDATATYPE_INTEGER.clone()) {
                    partlyFilledClInt = metamodelica::cons(partlyFilledCacheLine, partlyFilledClInt);
                } else {
                    partlyFilledClBool = metamodelica::cons(partlyFilledCacheLine, partlyFilledClBool);
                }
            }
        }
    }
    metamodelica::arrayUpdate(
        iSharedCacheLines.clone(),
        iThreadIdx,
        (
            (partlyFilledClFloat, partlyFilledClInt, partlyFilledClBool),
            (fullyFilledClFloat, fullyFilledClInt, fullyFilledClBool),
        ),
    )?;
    oInfo = (
        CacheMap::CACHEMAP {
            cacheLineSize: cacheLineSize,
            cacheVariables: cacheVariables,
            cacheLinesFloat: cacheLinesFloat,
            cacheLinesInt: cacheLinesInt,
            cacheLinesBool: cacheLinesBool,
        },
        CacheMapMeta {
            allSCVarsMapping: allSCVarsMapping.clone(),
            simCodeVarTypes: simCodeVarTypes.clone(),
            scVarCLMapping: scVarCLMapping.clone(),
        },
        numOfCLs,
    );
    Ok(oInfo)
}

fn getPartlyFilledCLByVarType(
    mut iVarType: i32,
    mut iSharedCacheLines: PartlyFilledCacheLines,
) -> metamodelica::List<PartlyFilledCacheLine> {
    let mut oSharedCacheLinesForType: metamodelica::List<PartlyFilledCacheLine>;
    if intEq(iVarType, VARDATATYPE_FLOAT.clone()) {
        oSharedCacheLinesForType = Util::tuple31(iSharedCacheLines);
    } else {
        if intEq(iVarType, VARDATATYPE_INTEGER.clone()) {
            oSharedCacheLinesForType = Util::tuple32(iSharedCacheLines);
        } else {
            oSharedCacheLinesForType = Util::tuple33(iSharedCacheLines);
        }
    }
    oSharedCacheLinesForType
}

fn findMatchingSharedCLLevelfix(
    mut iNodeVar: i32,
    mut iVarSize: i32,
    mut iVarType: i32,
    mut iThreadIdx: i32,
    mut iLevelThreadIdx: (i32, i32),
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>;
    let mut partlyFilledCacheLines: metamodelica::List<PartlyFilledCacheLine>;
    let mut sharedCacheLines: PartlyFilledCacheLines;
    let mut levelIdx: i32;
    (levelIdx, _) = iLevelThreadIdx;
    sharedCacheLines = Util::tuple21(metamodelica::arrayGet(iSharedCacheLines.clone(), iThreadIdx)?);
    oMatchedCacheLine = None;
    partlyFilledCacheLines = getPartlyFilledCLByVarType(iVarType, sharedCacheLines);
    oMatchedCacheLine =
        findMatchingSharedCLLevelfix0(iNodeVar, iVarSize, levelIdx, iThreadIdx, 1, &partlyFilledCacheLines)?;
    Ok(oMatchedCacheLine)
}

fn findMatchingSharedCLLevelfix0(
    mut iNodeVar: i32,
    mut iVarSize: i32,
    mut iLevelIdx: i32,
    mut iThreadIdx: i32,
    mut iCurrentListIdx: i32,
    mut iSharedCacheLines: &metamodelica::List<PartlyFilledCacheLine>,
) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>;
    let mut head: PartlyFilledCacheLine;
    let mut rest: metamodelica::List<PartlyFilledCacheLine>;
    let mut tmpMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>;
    let mut cacheLineMap: CacheLineMap;
    let mut numBytesFree: i32;
    let mut prefetchLevel: metamodelica::List<i32>;
    let mut writeLevel: metamodelica::List<(i32, i32)>;
    oMatchedCacheLine = (::match_deref::match_deref! { match iSharedCacheLines {
        Deref @ metamodelica::ListNode::Cons { head: __esc_head @ PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { cacheLineMap: __esc_cacheLineMap @ CacheLineMap { numBytesFree: __esc_numBytesFree, .. }, prefetchLevel: __esc_prefetchLevel, writeLevel: __esc_writeLevel }, tail: __esc_rest } => {
            head = (*__esc_head).clone();
            cacheLineMap = (*__esc_cacheLineMap).clone();
            numBytesFree = (*__esc_numBytesFree).clone();
            prefetchLevel = (*__esc_prefetchLevel).clone();
            writeLevel = (*__esc_writeLevel).clone();
            rest = (*__esc_rest).clone();
            if boolOr(intLt(numBytesFree.clone(), iVarSize), List::exist1(metamodelica::AsArg::as_arg(&prefetchLevel), &fnptr!(intEq, i32, i32), iLevelIdx)?) {
                tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar, iVarSize, iLevelIdx, iThreadIdx, iCurrentListIdx + 1, metamodelica::AsArg::as_arg(&rest))?;
            } else {
                if List::any(metamodelica::AsArg::as_arg(&writeLevel), &({ let __pe_b1 = iLevelIdx; let __pe_b2 = iThreadIdx; move |__pe_a0| Ok(isCLWrittenByOtherThread(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))? {
                    tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar, iVarSize, iLevelIdx, iThreadIdx, iCurrentListIdx + 1, metamodelica::AsArg::as_arg(&rest))?;
                } else {
                    if List::any(metamodelica::AsArg::as_arg(&writeLevel), &({ let __pe_b1 = iLevelIdx - 1; let __pe_b2 = iThreadIdx; move |__pe_a0| Ok(isCLWrittenByOtherThread(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))? {
                        tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar, iVarSize, iLevelIdx, iThreadIdx, iCurrentListIdx + 1, metamodelica::AsArg::as_arg(&rest))?;
                    } else {
                        tmpMatchedCacheLine = Some((head.clone(), iCurrentListIdx));
                    }
                }
            }
            tmpMatchedCacheLine
        },
        Deref @ metamodelica::ListNode::Nil => None,
        _ => {
            metamodelica::print(literal!("findMatchingSharedCLLevelfix0: Unknown partly filled cache line type given.\n"));
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oMatchedCacheLine)
}

fn findMatchingSharedCLThread(
    mut iNodeVar: i32,
    mut iVarSize: i32,
    mut iVarType: i32,
    mut iThreadIdx: i32,
    mut iAdditionalArgument: i32,
    mut iSharedCacheLines: metamodelica::Array<(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    )>,
) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>;
    let mut partlyFilledCacheLines: metamodelica::List<PartlyFilledCacheLine>;
    let mut partlyFilledCL: PartlyFilledCacheLine = <PartlyFilledCacheLine as ::std::default::Default>::default();
    let mut numBytesFree: i32;
    let mut listIdx: i32;
    oMatchedCacheLine = None;
    partlyFilledCacheLines = getPartlyFilledCLByVarType(
        iVarType,
        Util::tuple21(metamodelica::arrayGet(iSharedCacheLines.clone(), iThreadIdx)?),
    );
    listIdx = 1;
    for mut partlyFilledCL in &*partlyFilledCacheLines {
        let mut partlyFilledCL = partlyFilledCL.clone();
        let CacheLineMap {
            numBytesFree: __pa0, ..
        } = getCacheLineMapOfPartlyFilledCacheLine(&partlyFilledCL);
        numBytesFree = metamodelica::Own::own(__pa0);
        if intGe(numBytesFree, iVarSize) {
            oMatchedCacheLine = Some((partlyFilledCL, listIdx));
            break;
        }
        listIdx = listIdx + 1;
    }
    Ok(oMatchedCacheLine)
}

fn createSharedClThread(
    mut iOldPartlyFilledCacheLine: Option<PartlyFilledCacheLine>,
    mut iCacheLineMap: CacheLineMap,
    mut iAdditionalArgument: i32,
) -> PartlyFilledCacheLine {
    let mut oCreatedCacheLine: PartlyFilledCacheLine;
    oCreatedCacheLine = PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD {
        cacheLineMap: iCacheLineMap,
    };
    oCreatedCacheLine
}

fn createSharedClLevelFix(
    mut iOldPartlyFilledCacheLine: Option<PartlyFilledCacheLine>,
    mut iCacheLineMap: CacheLineMap,
    mut iLevelThreadIdx: (i32, i32),
) -> Result<PartlyFilledCacheLine> {
    let mut oCreatedCacheLine: PartlyFilledCacheLine;
    let mut prefetchLevel: metamodelica::List<i32>;
    let mut writeLevel: metamodelica::List<(i32, i32)>;
    let mut levelIdx: i32;
    let mut threadIdx: i32;
    (levelIdx, threadIdx) = iLevelThreadIdx;
    if (iOldPartlyFilledCacheLine).is_some() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iOldPartlyFilledCacheLine) {
            Some(PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { prefetchLevel: __pa0, writeLevel: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        prefetchLevel = metamodelica::Own::own(__pa0);
        writeLevel = metamodelica::Own::own(__pa1);
    } else {
        prefetchLevel = metamodelica::nil();
        writeLevel = metamodelica::nil();
    }
    if intGt(levelIdx - 1, 0) {
        prefetchLevel = metamodelica::cons(levelIdx - 1, prefetchLevel);
    }
    writeLevel = metamodelica::cons((levelIdx, threadIdx), writeLevel);
    oCreatedCacheLine = PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL {
        cacheLineMap: iCacheLineMap,
        prefetchLevel: prefetchLevel,
        writeLevel: writeLevel,
    };
    Ok(oCreatedCacheLine)
}

fn isCLWrittenByOtherThread(mut iLevelInfo: (i32, i32), mut iLevelIdx: i32, mut iThreadIdx: i32) -> bool {
    let mut oWrittenByOtherThread: bool;
    let mut levelIdx: i32;
    let mut threadIdx: i32;
    let mut ret: bool;
    (levelIdx, threadIdx) = iLevelInfo;
    ret = boolAnd(intEq(levelIdx, iLevelIdx), intNe(threadIdx, iThreadIdx));
    oWrittenByOtherThread = ret;
    oWrittenByOtherThread
}

fn createCacheMapFromThreadAndSharedCLs(
    mut iThreadCacheLines: CacheLines,
    mut iSharedCacheLines: &(
        (
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
            metamodelica::List<PartlyFilledCacheLine>,
        ),
        (
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
            metamodelica::List<CacheLineMap>,
        ),
    ),
    mut iCacheMap: CacheMap,
) -> Result<CacheMap> {
    let mut oCacheMap: CacheMap;
    let mut cacheLineSize: i32;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut fullyFilledSharedCacheLines: CacheLines;
    let mut partlyFilledCacheLines: PartlyFilledCacheLines;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let CacheMap::CACHEMAP {
        cacheLineSize: __pa0,
        cacheVariables: __pa1,
        cacheLinesFloat: __pa2,
        cacheLinesInt: __pa3,
        cacheLinesBool: __pa4,
    } = (iCacheMap)
    else {
        return Err("pattern mismatch");
    };
    cacheLineSize = metamodelica::Own::own(__pa0);
    cacheVariables = metamodelica::Own::own(__pa1);
    cacheLinesFloat = metamodelica::Own::own(__pa2);
    cacheLinesInt = metamodelica::Own::own(__pa3);
    cacheLinesBool = metamodelica::Own::own(__pa4);
    (partlyFilledCacheLines, fullyFilledSharedCacheLines) = iSharedCacheLines.clone();
    cacheLinesFloat = listAppend(
        cacheLinesFloat,
        listAppend(
            Util::tuple31(iThreadCacheLines.clone()),
            Util::tuple31(fullyFilledSharedCacheLines.clone()),
        ),
    );
    cacheLinesInt = listAppend(
        cacheLinesInt,
        listAppend(
            Util::tuple32(iThreadCacheLines.clone()),
            Util::tuple32(fullyFilledSharedCacheLines.clone()),
        ),
    );
    cacheLinesBool = listAppend(
        cacheLinesBool,
        listAppend(
            Util::tuple33(iThreadCacheLines),
            Util::tuple33(fullyFilledSharedCacheLines),
        ),
    );
    cacheLinesFloat = listAppend(
        cacheLinesFloat,
        List::map(
            Util::tuple31(partlyFilledCacheLines.clone()),
            &move |__a0: PartlyFilledCacheLine| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(getCacheLineMapOfPartlyFilledCacheLine(&__a0))
            },
        )?,
    );
    cacheLinesInt = listAppend(
        cacheLinesInt,
        List::map(
            Util::tuple32(partlyFilledCacheLines.clone()),
            &move |__a0: PartlyFilledCacheLine| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(getCacheLineMapOfPartlyFilledCacheLine(&__a0))
            },
        )?,
    );
    cacheLinesBool = listAppend(
        cacheLinesBool,
        List::map(
            Util::tuple33(partlyFilledCacheLines),
            &move |__a0: PartlyFilledCacheLine| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(getCacheLineMapOfPartlyFilledCacheLine(&__a0))
            },
        )?,
    );
    oCacheMap = CacheMap::CACHEMAP {
        cacheLineSize: cacheLineSize,
        cacheVariables: cacheVariables,
        cacheLinesFloat: cacheLinesFloat,
        cacheLinesInt: cacheLinesInt,
        cacheLinesBool: cacheLinesBool,
    };
    Ok(oCacheMap)
}

fn createCacheMapDefault(
    mut iAllSCVars: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iCacheLineSize: i32,
    mut iSimCodeVars: &SimCodeVar::SimVars,
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32;
    if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
        (oCacheMap, oScVarCLMapping, oNumCL) = createCacheMapDefaultCppRuntime(
            iAllSCVars.clone(),
            iCacheLineSize,
            iSimCodeVars,
            iScVarTaskMapping.clone(),
            iSchedulerInfo.clone(),
            iSimCodeVarTypes.clone(),
        )?;
    } else {
        oCacheMap = CacheMap::UNIFORM_CACHEMAP {
            cacheLineSize: iCacheLineSize,
            cacheVariables: metamodelica::nil(),
            cacheLines: metamodelica::nil(),
        };
        oNumCL = 0;
        oScVarCLMapping = arrayCreate(0, (-1, -1));
    }
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapDefaultCppRuntime(
    mut iAllSCVars: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iCacheLineSize: i32,
    mut iSimCodeVars: &SimCodeVar::SimVars,
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap;
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut oNumCL: i32;
    let mut stateVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut derivativeVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut algVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut discreteAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut paramVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut aliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut intAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut intParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut intAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut boolAlgVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut boolParamVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut boolAliasVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut inputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut outputVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheMap: CacheMap;
    let mut lastCacheLine: CacheLineMap;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut currentScVarIdx: i32;
    let mut paramVarsStart: i32;
    let mut aliasVarsStart: i32;
    let mut stateDerVarsStart: i32;
    let mut algVarsStart: i32;
    let mut discreteAlgVarsStart: i32;
    let mut intAlgVarsStart: i32;
    let mut intParamVarsStart: i32;
    let mut filledCacheLines: metamodelica::List<CacheLineMap>;
    let mut allVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    (oCacheMap, oScVarCLMapping, oNumCL) = (match iSimCodeVars.clone() {
        SimCodeVar::SimVars {
            stateVars: mut __esc_stateVars,
            derivativeVars: mut __esc_derivativeVars,
            algVars: mut __esc_algVars,
            discreteAlgVars: mut __esc_discreteAlgVars,
            paramVars: mut __esc_paramVars,
            aliasVars: mut __esc_aliasVars,
            intAlgVars: mut __esc_intAlgVars,
            intParamVars: mut __esc_intParamVars,
            intAliasVars: mut __esc_intAliasVars,
            boolAlgVars: mut __esc_boolAlgVars,
            boolParamVars: mut __esc_boolParamVars,
            boolAliasVars: mut __esc_boolAliasVars,
            inputVars: mut __esc_inputVars,
            outputVars: mut __esc_outputVars,
            ..
        } => {
            stateVars = __esc_stateVars.clone();
            derivativeVars = __esc_derivativeVars.clone();
            algVars = __esc_algVars.clone();
            discreteAlgVars = __esc_discreteAlgVars.clone();
            paramVars = __esc_paramVars.clone();
            aliasVars = __esc_aliasVars.clone();
            intAlgVars = __esc_intAlgVars.clone();
            intParamVars = __esc_intParamVars.clone();
            intAliasVars = __esc_intAliasVars.clone();
            boolAlgVars = __esc_boolAlgVars.clone();
            boolParamVars = __esc_boolParamVars.clone();
            boolAliasVars = __esc_boolAliasVars.clone();
            inputVars = __esc_inputVars.clone();
            outputVars = __esc_outputVars.clone();
            currentScVarIdx = 1;
            stateDerVarsStart = ((stateVars).len() as i32) + 1;
            scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVars.clone()), (-1, -1));
            filledCacheLines = metamodelica::nil();
            lastCacheLine = CacheLineMap {
                idx: 1,
                numBytesFree: iCacheLineSize,
                entries: metamodelica::nil(),
            };
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&derivativeVars),
                currentScVarIdx,
                stateDerVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            filledCacheLines = metamodelica::cons(lastCacheLine, filledCacheLines);
            lastCacheLine = CacheLineMap {
                idx: ((filledCacheLines).len() as i32) + 1,
                numBytesFree: iCacheLineSize,
                entries: metamodelica::nil(),
            };
            allVars = derivativeVars.clone().reverse();
            algVarsStart = stateDerVarsStart + ((derivativeVars).len() as i32);
            discreteAlgVarsStart = algVarsStart + ((algVars).len() as i32);
            intAlgVarsStart = discreteAlgVarsStart + ((discreteAlgVars).len() as i32);
            aliasVarsStart = intAlgVarsStart
                + ((boolAlgVars).len() as i32)
                + ((inputVars).len() as i32)
                + ((outputVars).len() as i32);
            paramVarsStart = aliasVarsStart
                + ((aliasVars).len() as i32)
                + ((intAliasVars).len() as i32)
                + ((boolAliasVars).len() as i32);
            intParamVarsStart = paramVarsStart + ((paramVars).len() as i32);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&algVars),
                currentScVarIdx,
                algVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            allVars = List::append_reverse(metamodelica::AsArg::as_arg(&algVars), allVars);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&discreteAlgVars),
                currentScVarIdx,
                discreteAlgVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            allVars = List::append_reverse(metamodelica::AsArg::as_arg(&discreteAlgVars), allVars);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&paramVars),
                currentScVarIdx,
                paramVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            allVars = List::append_reverse(metamodelica::AsArg::as_arg(&paramVars), allVars);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&aliasVars),
                currentScVarIdx,
                aliasVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            allVars = List::append_reverse(metamodelica::AsArg::as_arg(&aliasVars), allVars);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&intAlgVars),
                currentScVarIdx,
                intAlgVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            allVars = List::append_reverse(metamodelica::AsArg::as_arg(&intAlgVars), allVars);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(
                metamodelica::AsArg::as_arg(&intParamVars),
                currentScVarIdx,
                intAlgVarsStart,
                scVarCLMapping.clone(),
                filledCacheLines,
                iScVarTaskMapping.clone(),
                iSchedulerInfo.clone(),
                lastCacheLine,
                iCacheLineSize,
                iSimCodeVarTypes.clone(),
            )?;
            allVars = List::append_reverse(metamodelica::AsArg::as_arg(&intParamVars), allVars);
            cacheMap = CacheMap::UNIFORM_CACHEMAP {
                cacheLineSize: iCacheLineSize,
                cacheVariables: allVars,
                cacheLines: metamodelica::cons(lastCacheLine, filledCacheLines.clone()),
            };
            (cacheMap, scVarCLMapping.clone(), ((filledCacheLines).len() as i32) + 1)
        }
    });
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapDefaultCppRuntime0(
    mut iVariables: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iScVarIdxStart: i32,
    mut iRealScVarIdxStart: i32,
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iFilledCacheLines: metamodelica::List<CacheLineMap>,
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iLastCacheLine: CacheLineMap,
    mut iCacheLineSize: i32,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(metamodelica::List<CacheLineMap>, CacheLineMap, i32)> {
    let mut oFilledCacheLines: metamodelica::List<CacheLineMap>;
    let mut oLastCacheLine: CacheLineMap;
    let mut oScVarIdx: i32;
    let mut currentScVarIdx: i32;
    let mut varSize: i32;
    let mut varDataType: i32;
    let mut varTask: i32;
    let mut threadIdx: i32;
    let mut varCLIdx: i32;
    let mut var: metamodelica::Ref<SimCodeVar::SimVar> =
        <metamodelica::Ref<SimCodeVar::SimVar> as ::std::default::Default>::default();
    let mut entry: CacheLineEntry;
    let mut newCacheLineCreated: bool;
    let mut lastCacheLine: CacheLineMap;
    let mut lastCacheLineNew: CacheLineMap;
    let mut filledCacheLines: metamodelica::List<CacheLineMap>;
    let mut cachelineEntries: metamodelica::List<CacheLineEntry>;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut nameStr: ArcStr;
    currentScVarIdx = 0;
    lastCacheLine = iLastCacheLine;
    filledCacheLines = iFilledCacheLines;
    for mut var in &**iVariables {
        let mut var = var.clone();
        let __arc1 = var;
        let SimCodeVar::SIMVAR { name: __pa0, .. } = &*__arc1;
        name = metamodelica::Own::own(__pa0);
        nameStr = ComponentReferenceBasics::printComponentRefStr(&name)?;
        if boolAnd(
            intLt(currentScVarIdx, metamodelica::arrayLength(iSimCodeVarTypes.clone())),
            intLt(currentScVarIdx, metamodelica::arrayLength(iScVarCLMapping.clone())),
        ) {
            (varDataType, varSize, _) =
                metamodelica::arrayGet(iSimCodeVarTypes.clone(), currentScVarIdx + iRealScVarIdxStart)?;
            if intLe(
                currentScVarIdx + iRealScVarIdxStart,
                metamodelica::arrayLength(iScVarTaskMapping.clone()),
            ) {
                varTask = metamodelica::arrayGet(iScVarTaskMapping.clone(), currentScVarIdx + iRealScVarIdxStart)?;
            } else {
                varTask = -1;
            }
            if boolAnd(
                intGe(varTask, 1),
                intGe(metamodelica::arrayLength(iSchedulerInfo.clone()), varTask),
            ) {
                threadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), varTask)?);
            } else {
                threadIdx = -1;
            }
            entry = CacheLineEntry {
                start: -1,
                dataType: varDataType,
                size: varSize,
                scVarIdx: currentScVarIdx + iScVarIdxStart,
                threadOwner: threadIdx,
            };
            (entry, lastCacheLineNew, newCacheLineCreated) =
                createCacheMapDefaultCppRuntime1(entry, iCacheLineSize, lastCacheLine.clone());
            let CacheLineMap {
                idx: __pa2,
                entries: __pa3,
                ..
            } = &lastCacheLineNew;
            varCLIdx = metamodelica::Own::own(__pa2);
            cachelineEntries = metamodelica::Own::own(__pa3);
            metamodelica::arrayUpdate(
                iScVarCLMapping.clone(),
                currentScVarIdx + iRealScVarIdxStart,
                (varCLIdx, varDataType),
            )?;
            if newCacheLineCreated {
                filledCacheLines = metamodelica::cons(lastCacheLine, filledCacheLines);
            }
            lastCacheLine = lastCacheLineNew;
        }
        currentScVarIdx = currentScVarIdx + 1;
    }
    oFilledCacheLines = filledCacheLines;
    oLastCacheLine = lastCacheLine;
    oScVarIdx = currentScVarIdx + iScVarIdxStart;
    Ok((oFilledCacheLines, oLastCacheLine, oScVarIdx))
}

fn createCacheMapDefaultCppRuntime1(
    mut iCacheLineEntry: CacheLineEntry,
    mut iCacheLineSize: i32,
    mut iLastCacheLine: CacheLineMap,
) -> (CacheLineEntry, CacheLineMap, bool) {
    let mut oCacheLineEntry: CacheLineEntry;
    let mut oLastCacheLine: CacheLineMap;
    let mut oNewOneCreated: bool;
    let mut numberOfFreeBytesLastCacheLine: i32;
    let mut lastCacheLineEntries: metamodelica::List<CacheLineEntry>;
    let mut cacheLine: CacheLineMap;
    let mut cacheLineEntry: CacheLineEntry;
    let mut entrySize: i32;
    let mut entryStart: i32;
    let mut entryType: i32;
    let mut entryVarIdx: i32;
    let mut entryThreadOwner: i32;
    let mut lastCacheLineIdx: i32;
    let CacheLineEntry {
        start: __pa0,
        dataType: __pa1,
        size: __pa2,
        scVarIdx: __pa3,
        threadOwner: __pa4,
    } = iCacheLineEntry;
    entryStart = metamodelica::Own::own(__pa0);
    entryType = metamodelica::Own::own(__pa1);
    entrySize = metamodelica::Own::own(__pa2);
    entryVarIdx = metamodelica::Own::own(__pa3);
    entryThreadOwner = metamodelica::Own::own(__pa4);
    let CacheLineMap {
        idx: __pa5,
        numBytesFree: __pa6,
        entries: __pa7,
    } = iLastCacheLine;
    lastCacheLineIdx = metamodelica::Own::own(__pa5);
    numberOfFreeBytesLastCacheLine = metamodelica::Own::own(__pa6);
    lastCacheLineEntries = metamodelica::Own::own(__pa7);
    if intGt(entrySize, numberOfFreeBytesLastCacheLine) {
        cacheLineEntry = CacheLineEntry {
            start: 0,
            dataType: entryType,
            size: entrySize,
            scVarIdx: entryVarIdx,
            threadOwner: entryThreadOwner,
        };
        cacheLine = CacheLineMap {
            idx: lastCacheLineIdx + 1,
            numBytesFree: iCacheLineSize - entrySize,
            entries: list![cacheLineEntry],
        };
        oNewOneCreated = true;
    } else {
        cacheLineEntry = CacheLineEntry {
            start: iCacheLineSize - numberOfFreeBytesLastCacheLine,
            dataType: entryType,
            size: entrySize,
            scVarIdx: entryVarIdx,
            threadOwner: entryThreadOwner,
        };
        cacheLine = CacheLineMap {
            idx: lastCacheLineIdx,
            numBytesFree: numberOfFreeBytesLastCacheLine - entrySize,
            entries: metamodelica::cons(cacheLineEntry, lastCacheLineEntries),
        };
        oNewOneCreated = false;
    }
    oCacheLineEntry = cacheLineEntry;
    oLastCacheLine = cacheLine;
    (oCacheLineEntry, oLastCacheLine, oNewOneCreated)
}

fn appendNodeVarsToCacheMap(
    mut iNodeIdx: i32,
    mut iOwnerThread: i32,
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iInfo: &(CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>),
) -> Result<(CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>);
    let mut simCodeVars: metamodelica::List<i32>;
    let mut writtenCL: metamodelica::List<i32>;
    let mut iCacheMap: CacheMap;
    let mut iCacheMapMeta: CacheMapMeta;
    let mut iNumNewCL: i32;
    let mut varsString: ArcStr;
    let mut clCandidates: metamodelica::List<(i32, i32)>;
    simCodeVars = metamodelica::arrayGet(iNodeSimCodeVarMapping.clone(), iNodeIdx)?;
    (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates) = iInfo.clone();
    varsString = stringDelimitList(List::map(simCodeVars.clone(), &fnptr!(intString, i32))?, literal!(","));
    (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates, writtenCL, _) = List::fold(
        &simCodeVars,
        &({
            let __pe_b1 = iOwnerThread;
            move |__pe_a0, __pe_a2| Ok(appendSCVarToCacheMap(__pe_a0, __pe_b1.clone(), &__pe_a2))
        }),
        (
            iCacheMap,
            iCacheMapMeta,
            iNumNewCL,
            clCandidates,
            metamodelica::nil(),
            1,
        ),
    )?;
    clCandidates = List::removeOnTrue(
        writtenCL,
        &move |__a0: metamodelica::List<i32>, __a1: (i32, i32)| appendNodeVarsToCacheMap0(&__a0, __a1),
        clCandidates,
    )?;
    oInfo = (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates);
    Ok(oInfo)
}

fn appendNodeVarsToCacheMap0(
    mut iWrittenCLs: &metamodelica::List<i32>,
    mut iDetailedCLInfo: (i32, i32),
) -> Result<bool> {
    let mut oRemove: bool;
    let mut clIdx: i32;
    let mut freeBytes: i32;
    let mut res: bool = false;
    oRemove = 'mc: {
        let __mc_input = iDetailedCLInfo;
        if let Ok(__v) = (|| -> Result<_> {
            let (mut clIdx, mut freeBytes) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (intEq(freeBytes, 0)) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut clIdx, mut freeBytes) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut res: bool = res.clone();
            res = List::isMemberOnTrue(clIdx, iWrittenCLs, &fnptr!(intEq, i32, i32))?;
            Ok((res, res.clone()))
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("appendNodeVarsToCacheMap0 failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oRemove)
}

fn appendSCVarToCacheMap(
    mut iSCVarIdx: i32,
    mut iOwnerThread: i32,
    mut iInfo: &(
        CacheMap,
        CacheMapMeta,
        i32,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        i32,
    ),
) -> (
    CacheMap,
    CacheMapMeta,
    i32,
    metamodelica::List<(i32, i32)>,
    metamodelica::List<i32>,
    i32,
) {
    let mut oInfo: (
        CacheMap,
        CacheMapMeta,
        i32,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        i32,
    );
    let mut iAllSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>;
    let mut iScVarCLMapping: metamodelica::Array<(i32, i32)>;
    let mut currentCLCandidateIdx: i32;
    let mut currentCLCandidateCLIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut currentCLCandidateFreeBytes: i32 = 0;
    let mut cacheLineSize: i32;
    let mut numNewCL: i32;
    let mut varDataType: i32 = 0;
    let mut numBytesRequired: i32 = 0;
    let mut entryStart: i32 = 0;
    let mut currentCLCandidate: (i32, i32) = (0, 0);
    let mut cacheLineCandidates: metamodelica::List<(i32, i32)>;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut CLentries: metamodelica::List<CacheLineEntry> = metamodelica::nil();
    let mut scVar: metamodelica::Ref<SimCodeVar::SimVar> =
        <metamodelica::Ref<SimCodeVar::SimVar> as ::std::default::Default>::default();
    let mut numCacheVars: i32 = 0;
    let mut freeSpace: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut cacheMap: CacheMap;
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut writtenCL: metamodelica::List<i32>;
    let mut tmpInfo: (
        CacheMap,
        CacheMapMeta,
        i32,
        metamodelica::List<(i32, i32)>,
        metamodelica::List<i32>,
        i32,
    ) = (
        <CacheMap as ::std::default::Default>::default(),
        <CacheMapMeta as ::std::default::Default>::default(),
        0,
        metamodelica::nil(),
        metamodelica::nil(),
        0,
    );
    oInfo = 'mc: {
        let __mc_input = iInfo;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11)) =
            (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cacheMap @ CacheMap::CACHEMAP { cacheLineSize, cacheVariables, cacheLinesFloat, cacheLinesInt, cacheLinesBool }, cacheMapMeta @ CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx) => {
                        let mut cacheMap = (*cacheMap).clone();
                        let mut cacheVariables = (*cacheVariables).clone();
                        let mut cacheLinesFloat = (*cacheLinesFloat).clone();
                        let mut cacheMapMeta = (*cacheMapMeta).clone();
                        let mut iScVarCLMapping = (*iScVarCLMapping).clone();
                        let mut cacheLineCandidates = (*cacheLineCandidates).clone();
                        let mut writtenCL = (*writtenCL).clone();
                        let mut CLentries: metamodelica::List<CacheLineEntry> = CLentries.clone();
                        let mut cacheLine: CacheLineMap = cacheLine.clone();
                        let mut clIdx: i32 = clIdx.clone();
                        let mut currentCLCandidate: (i32, i32) = currentCLCandidate.clone();
                        let mut currentCLCandidateCLIdx: i32 = currentCLCandidateCLIdx.clone();
                        let mut currentCLCandidateFreeBytes: i32 = currentCLCandidateFreeBytes.clone();
                        let mut entryStart: i32 = entryStart.clone();
                        let mut numBytesFree: i32 = numBytesFree.clone();
                        let mut numBytesRequired: i32 = numBytesRequired.clone();
                        let mut numCacheVars: i32 = numCacheVars.clone();
                        let mut scVar: metamodelica::Ref<SimCodeVar::SimVar> = scVar.clone();
                        let mut varDataType: i32 = varDataType.clone();
                        let true = (intGe(((cacheLineCandidates).len() as i32), currentCLCandidateIdx.clone())) else { return Err("pattern mismatch") };
                        currentCLCandidate = (cacheLineCandidates).get(currentCLCandidateIdx.clone())?;
                        (varDataType, numBytesRequired, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), iSCVarIdx)?;
                        let true = (doesSCVarFitIntoCL(currentCLCandidate, numBytesRequired)) else { return Err("pattern mismatch") };
                        (currentCLCandidateCLIdx, currentCLCandidateFreeBytes) = currentCLCandidate;
                        cacheLine = ((cacheLinesFloat.clone())).get(((cacheLinesFloat).len() as i32) - currentCLCandidateCLIdx + 1)?;
                        let CacheLineMap { idx: __pa0, numBytesFree: __pa1, entries: __pa2 } = &cacheLine;
                        clIdx = metamodelica::Own::own(__pa0);
                        numBytesFree = metamodelica::Own::own(__pa1);
                        CLentries = metamodelica::Own::own(__pa2);
                        entryStart = cacheLineSize.clone() - currentCLCandidateFreeBytes;
                        numCacheVars = ((cacheVariables).len() as i32) + 1;
                        CLentries = metamodelica::cons(CacheLineEntry { start: entryStart, dataType: varDataType, size: numBytesRequired, scVarIdx: numCacheVars, threadOwner: iOwnerThread }, CLentries.clone());
                        cacheLine = CacheLineMap { idx: clIdx, numBytesFree: numBytesFree + numBytesRequired, entries: CLentries.clone() };
                        cacheLinesFloat = List::set(cacheLinesFloat.clone(), ((cacheLinesFloat).len() as i32) - currentCLCandidateCLIdx + 1, cacheLine.clone())?;
                        iScVarCLMapping = metamodelica::arrayUpdate(iScVarCLMapping.clone(), iSCVarIdx, (clIdx, varDataType))?;
                        let __pa3 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(iAllSCVarsMapping.clone(), iSCVarIdx)?) {
                            Some(__pa3) => __pa3.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        scVar = metamodelica::Own::own(__pa3);
                        cacheVariables = metamodelica::cons(scVar.clone(), cacheVariables.clone());
                        writtenCL = metamodelica::cons(clIdx, writtenCL.clone());
                        currentCLCandidate = (currentCLCandidateCLIdx, currentCLCandidateFreeBytes - numBytesRequired);
                        cacheLineCandidates = List::set(cacheLineCandidates.clone(), currentCLCandidateIdx.clone(), currentCLCandidate)?;
                        cacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
                        cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: iScVarCLMapping.clone() };
                        Ok(((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone(), cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone()), CLentries.clone(), cacheLine.clone(), clIdx.clone(), currentCLCandidate.clone(), currentCLCandidateCLIdx.clone(), currentCLCandidateFreeBytes.clone(), entryStart.clone(), numBytesFree.clone(), numBytesRequired.clone(), numCacheVars.clone(), scVar.clone(), varDataType.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })()
        {
            CLentries = __wb0;
            cacheLine = __wb1;
            clIdx = __wb2;
            currentCLCandidate = __wb3;
            currentCLCandidateCLIdx = __wb4;
            currentCLCandidateFreeBytes = __wb5;
            entryStart = __wb6;
            numBytesFree = __wb7;
            numBytesRequired = __wb8;
            numCacheVars = __wb9;
            scVar = __wb10;
            varDataType = __wb11;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cacheMap @ CacheMap::CACHEMAP { cacheLineSize, cacheVariables, cacheLinesFloat, cacheLinesInt, cacheLinesBool }, cacheMapMeta @ CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx) => {
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, metamodelica::List<(i32, i32)>, metamodelica::List<i32>, i32) = tmpInfo.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    let true = (intGe(((cacheLineCandidates).len() as i32), currentCLCandidateIdx.clone())) else { return Err("pattern mismatch") };
                    (varDataType, numBytesRequired, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), iSCVarIdx)?;
                    tmpInfo = appendSCVarToCacheMap(iSCVarIdx, iOwnerThread, &((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone(), cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone() + 1)));
                    Ok((tmpInfo.clone(), numBytesRequired.clone(), tmpInfo.clone(), varDataType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            numBytesRequired = __wb0;
            tmpInfo = __wb1;
            varDataType = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cacheMap @ CacheMap::CACHEMAP { cacheLineSize, cacheVariables, cacheLinesFloat, cacheLinesInt, cacheLinesBool }, CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx) => {
                    let mut cacheMap = (*cacheMap).clone();
                    let mut cacheVariables = (*cacheVariables).clone();
                    let mut cacheLinesFloat = (*cacheLinesFloat).clone();
                    let mut iScVarCLMapping = (*iScVarCLMapping).clone();
                    let mut cacheLineCandidates = (*cacheLineCandidates).clone();
                    let mut writtenCL = (*writtenCL).clone();
                    let mut CLentries: metamodelica::List<CacheLineEntry> = CLentries.clone();
                    let mut cacheLine: CacheLineMap = cacheLine.clone();
                    let mut cacheMapMeta: CacheMapMeta = cacheMapMeta.clone();
                    let mut clIdx: i32 = clIdx.clone();
                    let mut entryStart: i32 = entryStart.clone();
                    let mut freeSpace: i32 = freeSpace.clone();
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut numCacheVars: i32 = numCacheVars.clone();
                    let mut scVar: metamodelica::Ref<SimCodeVar::SimVar> = scVar.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    (varDataType, numBytesRequired, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), iSCVarIdx)?;
                    entryStart = 0;
                    numCacheVars = ((cacheVariables).len() as i32) + 1;
                    CLentries = list![CacheLineEntry { start: entryStart, dataType: varDataType, size: numBytesRequired, scVarIdx: numCacheVars, threadOwner: iOwnerThread }];
                    clIdx = ((cacheLinesFloat).len() as i32) + 1;
                    cacheLine = CacheLineMap { idx: clIdx, numBytesFree: numBytesRequired, entries: CLentries.clone() };
                    cacheLinesFloat = metamodelica::cons(cacheLine.clone(), cacheLinesFloat.clone());
                    iScVarCLMapping = metamodelica::arrayUpdate(iScVarCLMapping.clone(), iSCVarIdx, (clIdx, varDataType))?;
                    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(iAllSCVarsMapping.clone(), iSCVarIdx)?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    scVar = metamodelica::Own::own(__pa0);
                    cacheVariables = metamodelica::cons(scVar.clone(), cacheVariables.clone());
                    writtenCL = metamodelica::cons(clIdx, writtenCL.clone());
                    freeSpace = cacheLineSize.clone() - numBytesRequired;
                    cacheLineCandidates = List::appendElt((clIdx, freeSpace), cacheLineCandidates.clone());
                    cacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
                    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: iScVarCLMapping.clone() };
                    Ok(((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone() + 1, cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone()), CLentries.clone(), cacheLine.clone(), cacheMapMeta.clone(), clIdx.clone(), entryStart.clone(), freeSpace.clone(), numBytesRequired.clone(), numCacheVars.clone(), scVar.clone(), varDataType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            CLentries = __wb0;
            cacheLine = __wb1;
            cacheMapMeta = __wb2;
            clIdx = __wb3;
            entryStart = __wb4;
            freeSpace = __wb5;
            numBytesRequired = __wb6;
            numCacheVars = __wb7;
            scVar = __wb8;
            varDataType = __wb9;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("appendSCVarToCacheMap failed! Variable skipped.\n"));
                    Ok(iInfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oInfo
}

fn doesSCVarFitIntoCL(mut iCacheLineCandidate: (i32, i32), mut iNumBytes: i32) -> bool {
    let mut oResult: bool;
    let mut freeSpace: i32;
    (_, freeSpace) = iCacheLineCandidate;
    oResult = intGe(freeSpace, iNumBytes);
    oResult
}

fn createDetailedCacheMapInformation(
    mut iCacheLinesIdc: &metamodelica::List<i32>,
    mut iCacheLines: metamodelica::List<CacheLineMap>,
    mut iCacheLineSize: i32,
) -> Result<metamodelica::List<(i32, i32)>> {
    let mut oCacheLines: metamodelica::List<(i32, i32)>;
    let mut iCacheLinesArray: metamodelica::Array<CacheLineMap>;
    iCacheLinesArray = metamodelica::arrayFromVec(iCacheLines.into_iter().cloned().collect());
    oCacheLines = List::fold2(
        iCacheLinesIdc,
        &fnptr!(
            createDetailedCacheMapInformation0,
            i32,
            metamodelica::Array<CacheLineMap>,
            i32,
            metamodelica::List<(i32, i32)>
        ),
        iCacheLinesArray.clone(),
        iCacheLineSize,
        metamodelica::nil(),
    )?;
    Ok(oCacheLines)
}

fn createDetailedCacheMapInformation0(
    mut iCacheLineIdx: i32,
    mut iCacheLinesArray: metamodelica::Array<CacheLineMap>,
    mut iCacheLineSize: i32,
    mut iCacheLines: metamodelica::List<(i32, i32)>,
) -> metamodelica::List<(i32, i32)> {
    let mut oCacheLines: metamodelica::List<(i32, i32)>;
    let mut cacheLineEntry: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut numBytesFree: i32 = 0;
    let mut cacheLines: metamodelica::List<(i32, i32)> = metamodelica::nil();
    oCacheLines = 'mc: {
        let __mc_input = &*iCacheLines;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cacheLineEntry: CacheLineMap = cacheLineEntry.clone();
                    let mut cacheLines: metamodelica::List<(i32, i32)> = cacheLines.clone();
                    let mut numBytesFree: i32 = numBytesFree.clone();
                    cacheLineEntry = metamodelica::arrayGet(iCacheLinesArray.clone(), metamodelica::arrayLength(iCacheLinesArray.clone()) - iCacheLineIdx + 1)?;
                    numBytesFree = iCacheLineSize - getNumOfUsedBytesByCacheLine(cacheLineEntry.clone())?;
                    let true = (intGt(numBytesFree, 0)) else { return Err("pattern mismatch") };
                    cacheLines = metamodelica::cons((iCacheLineIdx, numBytesFree), iCacheLines.clone());
                    Ok((cacheLines.clone(), cacheLineEntry.clone(), cacheLines.clone(), numBytesFree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cacheLineEntry = __wb0;
            cacheLines = __wb1;
            numBytesFree = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCacheLines.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCacheLines
}

fn getNumOfUsedBytesByCacheLine(mut iCacheLineMap: CacheLineMap) -> Result<i32> {
    let mut oNumBytes: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let mut firstEntryStart: i32;
    let mut firstEntrySize: i32;
    let CacheLineMap { entries: __pa0, .. } = iCacheLineMap;
    entries = metamodelica::Own::own(__pa0);
    entries = List::sort(
        entries,
        (std::sync::Arc::new(fnptr!(sortCacheLineEntriesByPos, CacheLineEntry, CacheLineEntry))
            as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, CacheLineEntry) -> Result<bool> + 'static>),
    )?;
    let CacheLineEntry {
        start: __pa1,
        size: __pa2,
        ..
    } = List::last(&entries)?;
    firstEntryStart = metamodelica::Own::own(__pa1);
    firstEntrySize = metamodelica::Own::own(__pa2);
    oNumBytes = firstEntryStart + firstEntrySize;
    Ok(oNumBytes)
}

fn sortCacheLineEntriesByPos(mut iCacheLineEntry1: CacheLineEntry, mut iCacheLineEntry2: CacheLineEntry) -> bool {
    let mut oIsGreater: bool;
    let mut start1: i32;
    let mut start2: i32;
    let CacheLineEntry { start: __pa0, .. } = iCacheLineEntry1;
    start1 = metamodelica::Own::own(__pa0);
    let CacheLineEntry { start: __pa1, .. } = iCacheLineEntry2;
    start2 = metamodelica::Own::own(__pa1);
    oIsGreater = intGt(start1, start2);
    oIsGreater
}

fn reverseCacheLineMapEntries(mut iCacheLineMap: CacheLineMap) -> CacheLineMap {
    let mut oCacheLineMap: CacheLineMap;
    let mut idx: i32;
    let mut numBytesFree: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let CacheLineMap {
        idx: __pa0,
        numBytesFree: __pa1,
        entries: __pa2,
    } = iCacheLineMap;
    idx = metamodelica::Own::own(__pa0);
    numBytesFree = metamodelica::Own::own(__pa1);
    entries = metamodelica::Own::own(__pa2);
    entries = entries.reverse();
    oCacheLineMap = CacheLineMap {
        idx: idx,
        numBytesFree: numBytesFree,
        entries: entries,
    };
    oCacheLineMap
}

fn compareCacheLineMapByIdx(mut iCacheLineMap: CacheLineMap, mut iCacheLineMap2: CacheLineMap) -> bool {
    let mut oIsGreater: bool;
    let mut idx1: i32;
    let mut idx2: i32;
    let CacheLineMap { idx: __pa0, .. } = iCacheLineMap;
    idx1 = metamodelica::Own::own(__pa0);
    let CacheLineMap { idx: __pa1, .. } = iCacheLineMap2;
    idx2 = metamodelica::Own::own(__pa1);
    oIsGreater = intGt(idx1, idx2);
    oIsGreater
}

fn convertCacheToVarArrayMapping(
    mut iCacheMap: &CacheMap,
    mut iCacheLineSize: i32,
    mut iStateVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iDerivativeVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iAliasVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iIntAliasVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iBoolAliasVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iStringAliasVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iVarSizes: (i32, i32, i32),
    mut iNotOptimizedVars: &(
        metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
        metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
        metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
        metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    ),
) -> Result<(
    (
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    Option<HpcOmSimCode::MemoryMap>,
)> {
    let mut oVarToArrayIndexMapping: (
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
    let mut oVarToIndexMapping: (
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
    let mut oMemoryMap: Option<HpcOmSimCode::MemoryMap>;
    let mut cacheLineSize: i32;
    let mut maxNumElemsFloat: i32;
    let mut maxNumElemsInt: i32;
    let mut maxNumElemsBool: i32;
    let mut stateAndStateDerSize: i32;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheVariablesArray: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut allCacheLines: metamodelica::List<CacheLineMap>;
    let mut varArrayIndexMappingHashTable: (
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
    let mut varIndexMappingHashTable: (
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
    let mut varSizeFloat: i32;
    let mut varSizeInt: i32;
    let mut varSizeBool: i32;
    let mut varSizeString: i32;
    let mut varIdxOffsets: metamodelica::Array<i32>;
    let mut notOptimizedVarsFloat: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut notOptimizedVarsInt: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut notOptimizedVarsBool: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut notOptimizedVarsString: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut currentVarIndices: metamodelica::Array<i32>;
    (oVarToArrayIndexMapping, oVarToIndexMapping, oMemoryMap) = (::match_deref::match_deref! { match &((iCacheMap.clone(), iVarSizes, iNotOptimizedVars.clone())) {
        (CacheMap::CACHEMAP { cacheLineSize: __esc_cacheLineSize, cacheVariables: __esc_cacheVariables, cacheLinesFloat: __esc_cacheLinesFloat, cacheLinesInt: __esc_cacheLinesInt, cacheLinesBool: __esc_cacheLinesBool }, (__esc_varSizeFloat, __esc_varSizeInt, __esc_varSizeBool), (__esc_notOptimizedVarsFloat, __esc_notOptimizedVarsInt, __esc_notOptimizedVarsBool, __esc_notOptimizedVarsString)) => {
            cacheLineSize = (*__esc_cacheLineSize).clone();
            cacheVariables = (*__esc_cacheVariables).clone();
            cacheLinesFloat = (*__esc_cacheLinesFloat).clone();
            cacheLinesInt = (*__esc_cacheLinesInt).clone();
            cacheLinesBool = (*__esc_cacheLinesBool).clone();
            varSizeFloat = (*__esc_varSizeFloat).clone();
            varSizeInt = (*__esc_varSizeInt).clone();
            varSizeBool = (*__esc_varSizeBool).clone();
            notOptimizedVarsFloat = (*__esc_notOptimizedVarsFloat).clone();
            notOptimizedVarsInt = (*__esc_notOptimizedVarsInt).clone();
            notOptimizedVarsBool = (*__esc_notOptimizedVarsBool).clone();
            notOptimizedVarsString = (*__esc_notOptimizedVarsString).clone();
            maxNumElemsFloat = intDiv(iCacheLineSize, varSizeFloat.clone());
            maxNumElemsInt = intDiv(iCacheLineSize, varSizeInt.clone());
            maxNumElemsBool = intDiv(iCacheLineSize, varSizeBool.clone());
            cacheVariablesArray = metamodelica::arrayFromVec(cacheVariables.clone().into_iter().cloned().collect());
            varArrayIndexMappingHashTable = HashTableCrIListArray::emptyHashTable();
            varIndexMappingHashTable = HashTableCrILst::emptyHashTable();
            currentVarIndices = arrayCreate(4, 1);
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iStateVars, VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iDerivativeVars, VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            stateAndStateDerSize = intAdd(((iStateVars).len() as i32), ((iDerivativeVars).len() as i32));
            if intEq(intMod(stateAndStateDerSize, maxNumElemsFloat), 0) {
                metamodelica::arrayUpdate(currentVarIndices.clone(), 1, stateAndStateDerSize + 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 2, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 3, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 4, 1)?;
            } else {
                metamodelica::arrayUpdate(currentVarIndices.clone(), 1, stateAndStateDerSize + (maxNumElemsFloat - intMod(stateAndStateDerSize, maxNumElemsFloat)) + 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 2, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 3, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 4, 1)?;
            }
            varSizeFloat = metamodelica::arrayGet(currentVarIndices.clone(), 1)?;
            varIdxOffsets = arrayCreate(3, 1);
            varIdxOffsets = metamodelica::arrayUpdate(varIdxOffsets.clone(), 1, metamodelica::arrayGet(currentVarIndices.clone(), 1)? + 1)?;
            allCacheLines = List::sort(getAllCacheLinesOfCacheMap(iCacheMap), (std::sync::Arc::new(fnptr!(compareCacheLineMapByIdx, CacheLineMap, CacheLineMap)) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, CacheLineMap) -> Result<bool> + 'static>))?;
            (varArrayIndexMappingHashTable, varIndexMappingHashTable) = List::fold(&allCacheLines, &({ let __pe_b1 = cacheLineSize.clone(); let __pe_b2 = varIdxOffsets.clone(); let __pe_b3 = cacheVariablesArray.clone(); move |__pe_a0, __pe_a4| addCacheLineMapToVarArrayMapping(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), (varArrayIndexMappingHashTable, varIndexMappingHashTable))?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 1, metamodelica::arrayGet(currentVarIndices.clone(), 1)? + intMul(((cacheLinesFloat).len() as i32), maxNumElemsFloat))?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 2, intMul(((cacheLinesInt).len() as i32), maxNumElemsInt) + 1)?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 3, intMul(((cacheLinesBool).len() as i32), maxNumElemsBool) + 1)?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 4, 1)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(&(notOptimizedVarsFloat.clone().reverse()), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(&(notOptimizedVarsInt.clone().reverse()), VARDATATYPE_INTEGER.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(&(notOptimizedVarsBool.clone().reverse()), VARDATATYPE_BOOLEAN.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(&(notOptimizedVarsString.clone().reverse()), VARDATATYPE_STRING.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iAliasVars, VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iIntAliasVars, VARDATATYPE_INTEGER.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iBoolAliasVars, VARDATATYPE_BOOLEAN.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iStringAliasVars, VARDATATYPE_STRING.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable, varIndexMappingHashTable)?;
            varSizeFloat = varSizeFloat.clone() + intMul(((cacheLinesFloat).len() as i32), maxNumElemsFloat) + ((notOptimizedVarsFloat).len() as i32);
            varSizeInt = intMul(((cacheLinesInt).len() as i32), maxNumElemsInt) + ((notOptimizedVarsInt).len() as i32);
            varSizeBool = intMul(((cacheLinesBool).len() as i32), maxNumElemsBool) + ((notOptimizedVarsBool).len() as i32);
            varSizeString = ((notOptimizedVarsString).len() as i32);
            (varArrayIndexMappingHashTable, varIndexMappingHashTable, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { floatArraySize: varSizeFloat.clone(), intArraySize: varSizeInt.clone(), boolArraySize: varSizeBool.clone(), stringArraySize: varSizeString }))
        },
        (CacheMap::UNIFORM_CACHEMAP { .. }, _, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("ConvertCacheToVarArrayMapping: Uniform-CacheMap not supported!")])?;
            return Err("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("ConvertCacheToVarArrayMapping: CacheMap-Type not supported!")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVarToArrayIndexMapping, oVarToIndexMapping, oMemoryMap))
}

fn addCacheLineMapToVarArrayMapping(
    mut iCacheLineMap: &CacheLineMap,
    mut iCacheLineSize: i32,
    mut iVarIdxOffsets: metamodelica::Array<i32>,
    mut iCacheVariables: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iPositionMapping: (
        (
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<
                    dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static,
                >,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    (
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut oPositionMapping: (
        (
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
        (
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
    );
    let mut varArrayIndexMappingHashTable: (
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
    let mut varIndexMappingHashTable: (
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
    let mut idx: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let mut dataType: i32;
    let mut size: i32;
    oPositionMapping = (match (iCacheLineMap.clone(), iPositionMapping.clone()) {
        (
            CacheLineMap {
                idx: mut __esc_idx,
                entries: mut __esc_entries,
                ..
            },
            (mut __esc_varArrayIndexMappingHashTable, mut __esc_varIndexMappingHashTable),
        ) => {
            idx = __esc_idx.clone();
            entries = __esc_entries.clone();
            varArrayIndexMappingHashTable = __esc_varArrayIndexMappingHashTable.clone();
            varIndexMappingHashTable = __esc_varIndexMappingHashTable.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(entries.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: CacheLineEntry { dataType: __pa0, size: __pa1, .. }, tail: _ } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            dataType = metamodelica::Own::own(__pa0);
            size = metamodelica::Own::own(__pa1);
            (varArrayIndexMappingHashTable, varIndexMappingHashTable) = List::fold(
                metamodelica::AsArg::as_arg(&entries),
                &({
                    let __pe_b1 = dataType;
                    let __pe_b2 = (idx, iCacheLineSize);
                    let __pe_b3 = iVarIdxOffsets.clone();
                    let __pe_b4 = iCacheVariables.clone();
                    move |__pe_a0, __pe_a5| {
                        addCacheLineEntryToVarArrayMapping(
                            __pe_a0,
                            __pe_b1.clone(),
                            __pe_b2.clone(),
                            __pe_b3.clone(),
                            __pe_b4.clone(),
                            &__pe_a5,
                        )
                    }
                }),
                iPositionMapping,
            )?;
            metamodelica::arrayUpdate(
                iVarIdxOffsets.clone(),
                dataType,
                intAdd(
                    metamodelica::arrayGet(iVarIdxOffsets.clone(), dataType)?,
                    intDiv(iCacheLineSize, size),
                ),
            )?;
            (varArrayIndexMappingHashTable, varIndexMappingHashTable)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    "addCacheLineMapToVarArrayMapping failed! CacheLineMap-Type not supported!"
                )],
            )?;
            return Err("fail");
        }
    });
    Ok(oPositionMapping)
}

fn addCacheLineEntryToVarArrayMapping(
    mut iCacheLineEntry: CacheLineEntry,
    mut iArrayIdx: i32,
    mut iClIdxSize: (i32, i32),
    mut iVarIdxOffsets: metamodelica::Array<i32>,
    mut iCacheVariables: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iPositionMapping: &(
        (
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                Arc<
                    dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static,
                >,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> Result<(
    (
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut oPositionMapping: (
        (
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
        (
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
    );
    let mut varArrayIndexMappingHashTable: (
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
    let mut varIndexMappingHashTable: (
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
    let mut scVarIdx: i32;
    let mut start: i32;
    let mut size: i32;
    let mut arrayPosition: i32;
    let mut offset: i32;
    let mut currentVarIndices: metamodelica::Array<i32>;
    oPositionMapping = (match (iCacheLineEntry, iPositionMapping.clone()) {
        (
            CacheLineEntry {
                scVarIdx: mut __esc_scVarIdx,
                start: mut __esc_start,
                size: mut __esc_size,
                ..
            },
            (mut __esc_varArrayIndexMappingHashTable, mut __esc_varIndexMappingHashTable),
        ) => {
            scVarIdx = __esc_scVarIdx.clone();
            start = __esc_start.clone();
            size = __esc_size.clone();
            varArrayIndexMappingHashTable = __esc_varArrayIndexMappingHashTable.clone();
            varIndexMappingHashTable = __esc_varIndexMappingHashTable.clone();
            offset = metamodelica::arrayGet(iVarIdxOffsets.clone(), iArrayIdx)?;
            arrayPosition = intDiv(start, size) + offset;
            currentVarIndices = arrayCreate(4, arrayPosition);
            (_, varArrayIndexMappingHashTable, varIndexMappingHashTable) =
                SimCodeUtilShared::addVarToArrayIndexMapping(
                    &(metamodelica::arrayGet(
                        iCacheVariables.clone(),
                        metamodelica::arrayLength(iCacheVariables.clone()) - scVarIdx + 1,
                    )?),
                    iArrayIdx,
                    currentVarIndices.clone(),
                    varArrayIndexMappingHashTable,
                    varIndexMappingHashTable,
                )?;
            (varArrayIndexMappingHashTable, varIndexMappingHashTable)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    "addCacheLineEntryToVarArrayMapping failed! Unsupported entry-type\n"
                )],
            )?;
            return Err("fail");
        }
    });
    Ok(oPositionMapping)
}

fn convertCacheToVarArrayMapping2Helper(
    mut iArray: metamodelica::Array<i32>,
    mut iOffset: i32,
    mut iIndex: i32,
) -> Result<metamodelica::Array<i32>> {
    let mut oArray: metamodelica::Array<i32>;
    let mut tmpArray: metamodelica::Array<i32>;
    let mut i: i32 = 0;
    tmpArray = iArray.clone();
    for mut i in 1..=metamodelica::arrayLength(tmpArray.clone()) {
        if intNe(i, iIndex) {
            tmpArray = metamodelica::arrayUpdate(
                tmpArray.clone(),
                i,
                metamodelica::arrayGet(tmpArray.clone(), i)? + iOffset,
            )?;
        }
    }
    oArray = tmpArray.clone();
    Ok(oArray)
}

fn getNotOptimizedVarsByCacheLineMapping(
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iAllVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut oNotOptimizedVars: (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    );
    (oNotOptimizedVars, _) = Array::fold(
        iScVarCLMapping.clone(),
        &({
            let __pe_b1 = iAllVarsMapping.clone();
            let __pe_b2 = iSimCodeVarTypes.clone();
            move |__pe_a0, __pe_a3| {
                Ok(getNotOptimizedVarsByCacheLineMapping0(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    &__pe_a3,
                ))
            }
        }),
        (
            (
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
                metamodelica::nil(),
            ),
            1,
        ),
    )?;
    Ok(oNotOptimizedVars)
}

fn getNotOptimizedVarsByCacheLineMapping0(
    mut iScVarCLMapping: (i32, i32),
    mut iAllVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    mut iEntries: &(
        (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
        i32,
    ),
) -> (
    (
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    ),
    i32,
) {
    let mut oEntries: (
        (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        ),
        i32,
    );
    let mut tmpSimVarsFloat: metamodelica::List<i32>;
    let mut tmpSimVarsInt: metamodelica::List<i32>;
    let mut tmpSimVarsBool: metamodelica::List<i32>;
    let mut tmpSimVarsString: metamodelica::List<i32>;
    let mut scVarIdx: i32;
    let mut dataType: i32 = 0;
    oEntries = 'mc: {
        let __mc_input = (iScVarCLMapping, iEntries);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (((-1), _), ((tmpSimVarsFloat, tmpSimVarsInt, tmpSimVarsBool, tmpSimVarsString), scVarIdx)) => {
                    let mut tmpSimVarsFloat = (*tmpSimVarsFloat).clone();
                    let mut tmpSimVarsInt = (*tmpSimVarsInt).clone();
                    let mut tmpSimVarsBool = (*tmpSimVarsBool).clone();
                    let mut tmpSimVarsString = (*tmpSimVarsString).clone();
                    let mut dataType: i32 = dataType.clone();
                    dataType = Util::tuple31(metamodelica::arrayGet(iSimCodeVarTypes.clone(), scVarIdx.clone())?);
                    if intEq(dataType, VARDATATYPE_FLOAT.clone()) {
                        tmpSimVarsFloat = metamodelica::cons(scVarIdx.clone(), tmpSimVarsFloat.clone());
                    } else {
                        if intEq(dataType, VARDATATYPE_INTEGER.clone()) {
                            tmpSimVarsInt = metamodelica::cons(scVarIdx.clone(), tmpSimVarsInt.clone());
                        } else {
                            if intEq(dataType, VARDATATYPE_BOOLEAN.clone()) {
                                        tmpSimVarsBool = metamodelica::cons(scVarIdx.clone(), tmpSimVarsBool.clone());
                            } else {
                                        if intEq(dataType, VARDATATYPE_STRING.clone()) {
                                            tmpSimVarsString = metamodelica::cons(scVarIdx.clone(), tmpSimVarsString.clone());
                                        }
                            }
                        }
                    }
                    Ok((((tmpSimVarsFloat.clone(), tmpSimVarsInt.clone(), tmpSimVarsBool.clone(), tmpSimVarsString.clone()), scVarIdx.clone() + 1), dataType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dataType = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ((tmpSimVarsFloat, tmpSimVarsInt, tmpSimVarsBool, tmpSimVarsString), scVarIdx)) => {
                    Ok(((tmpSimVarsFloat.clone(), tmpSimVarsInt.clone(), tmpSimVarsBool.clone(), tmpSimVarsString.clone()), scVarIdx.clone() + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oEntries
}

// -------------------------------------------
// ANALYSIS
// -------------------------------------------
fn evaluateCacheBehaviour(
    mut iVarToIndexMappingHashTable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iSimVarIdxMappingHashTable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut taskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut taskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iNumberOfThreads: i32,
    mut iCacheLineSize: i32,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> () {
    ()
}

fn createVarCLMappingFromVarArrayIndexHashTable(
    mut iVarToIndexMappingHashTable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iSimVarIdxMappingHashTable: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iCacheLineSize: i32,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut oNumberOfVars: metamodelica::Array<i32>;
    let mut oVarToCLMapping: metamodelica::Array<i32>;
    let mut hashTableElements: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>;
    let mut hashTableElement: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>) =
        (metamodelica::Ref::new(DAE::ComponentRef::WILD), metamodelica::nil());
    let mut varToCLMapping: metamodelica::Array<i32>;
    let mut numberOfVars: metamodelica::Array<i32>;
    let mut pos: i32;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    varToCLMapping = arrayCreate(metamodelica::arrayLength(iSimCodeVarTypes.clone()), -1);
    numberOfVars = arrayCreate(3, 0);
    hashTableElements = BaseHashTable::hashTableList(iVarToIndexMappingHashTable)?;
    for mut hashTableElement in &*hashTableElements {
        let mut hashTableElement = hashTableElement.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(hashTableElement) {
            (__pa0, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa0);
        pos = metamodelica::Own::own(__pa1);
    }
    oNumberOfVars = numberOfVars.clone();
    oVarToCLMapping = varToCLMapping.clone();
    Ok((oNumberOfVars, oVarToCLMapping))
}

fn createCacheLineThreadProperties(
    mut iCacheLine: CacheLineMap,
    mut iNumberOfThreads: i32,
    mut iCacheLineSize: i32,
    mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
) -> Result<()> {
    let mut bytesPerThread: metamodelica::Array<i32>;
    let mut threadProperties: metamodelica::Array<metamodelica::Real>;
    let mut cacheLineIdx: i32;
    let mut threadOwner: i32;
    let mut size: i32;
    let mut threadIdx: i32 = 0;
    let mut numBytesFree: i32;
    let mut numBytesUnassigned: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let mut entry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut sizeReal: metamodelica::Real;
    let CacheLineMap {
        idx: __pa0,
        entries: __pa1,
        numBytesFree: __pa2,
    } = iCacheLine;
    cacheLineIdx = metamodelica::Own::own(__pa0);
    entries = metamodelica::Own::own(__pa1);
    numBytesFree = metamodelica::Own::own(__pa2);
    numBytesUnassigned = 0;
    threadProperties = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    bytesPerThread = arrayCreate(iNumberOfThreads, 0);
    for mut entry in &*entries {
        let mut entry = entry.clone();
        let CacheLineEntry {
            threadOwner: __pa3,
            size: __pa4,
            ..
        } = entry;
        threadOwner = metamodelica::Own::own(__pa3);
        size = metamodelica::Own::own(__pa4);
        if intLt(threadOwner, 0) {
            numBytesUnassigned = numBytesUnassigned + size;
        } else {
            bytesPerThread = metamodelica::arrayUpdate(
                bytesPerThread.clone(),
                threadOwner,
                metamodelica::arrayGet(bytesPerThread.clone(), threadOwner)? + size,
            )?;
        }
    }
    sizeReal = intReal(iCacheLineSize - numBytesFree - numBytesUnassigned);
    if realGt(sizeReal, metamodelica::OrderedFloat((0) as f64)) {
        for mut threadIdx in 1..=iNumberOfThreads {
            metamodelica::arrayUpdate(
                threadProperties.clone(),
                threadIdx,
                realDiv(
                    intReal(metamodelica::arrayGet(bytesPerThread.clone(), threadIdx)?),
                    sizeReal,
                ),
            )?;
        }
    }
    metamodelica::arrayUpdate(
        iCacheLineThreadProperties.clone(),
        cacheLineIdx,
        threadProperties.clone(),
    )?;
    Ok(())
}

fn calculateLocCoRead(
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut cacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::Real> {
    let mut oLocCoRead: metamodelica::Real;
    let mut nodeIdx: i32 = 0;
    let mut numberOfNodes: i32;
    let mut threadIdx: i32;
    let mut sum: metamodelica::Real;
    let mut locCoRead: metamodelica::Real;
    numberOfNodes = metamodelica::arrayLength(iNodeSimCodeVarMapping.clone());
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut nodeIdx in 1..=numberOfNodes {
        threadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), nodeIdx)?);
        locCoRead = calculateLocCoReadForTask(
            nodeIdx,
            threadIdx,
            iTaskGraphT.clone(),
            iNodeSimCodeVarMapping.clone(),
            iScVarCLMapping.clone(),
            cacheLineThreadProperties.clone(),
        )?;
        sum = sum + locCoRead;
    }
    if intGt(numberOfNodes, 0) {
        oLocCoRead = realDiv(sum, metamodelica::OrderedFloat((numberOfNodes) as f64));
    } else {
        oLocCoRead = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoRead)
}

fn calculateLocCoReadForTask(
    mut iNodeIdx: i32,
    mut iThreadIdx: i32,
    mut iTaskGraphT: metamodelica::Array<metamodelica::List<i32>>,
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
) -> Result<metamodelica::Real> {
    let mut oLocCoRead: metamodelica::Real;
    let mut predecessor: i32 = 0;
    let mut numberOfPredecessors: i32;
    let mut predecessors: metamodelica::List<i32>;
    let mut sum: metamodelica::Real;
    sum = metamodelica::OrderedFloat(0.0_f64);
    predecessors = metamodelica::arrayGet(iTaskGraphT.clone(), iNodeIdx)?;
    numberOfPredecessors = ((predecessors).len() as i32);
    for mut predecessor in &*predecessors {
        let mut predecessor = predecessor.clone();
        sum = sum
            + calculateLocCoForTask(
                predecessor,
                iThreadIdx,
                &(metamodelica::arrayGet(iNodeSimCodeVarMapping.clone(), predecessor)?),
                iScVarCLMapping.clone(),
                iCacheLineThreadProperties.clone(),
            )?;
    }
    if intGt(numberOfPredecessors, 0) {
        oLocCoRead = realDiv(sum, metamodelica::OrderedFloat((numberOfPredecessors) as f64));
    } else {
        oLocCoRead = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoRead)
}

fn calculateLocCoWrite(
    mut iNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut cacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
) -> Result<metamodelica::Real> {
    let mut oLocCoWrite: metamodelica::Real;
    let mut nodeIdx: i32 = 0;
    let mut numberOfNodes: i32;
    let mut threadIdx: i32;
    let mut sum: metamodelica::Real;
    let mut locCoWrite: metamodelica::Real;
    numberOfNodes = metamodelica::arrayLength(iNodeSimCodeVarMapping.clone());
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut nodeIdx in 1..=numberOfNodes {
        threadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), nodeIdx)?);
        locCoWrite = calculateLocCoForTask(
            nodeIdx,
            threadIdx,
            &(metamodelica::arrayGet(iNodeSimCodeVarMapping.clone(), nodeIdx)?),
            iScVarCLMapping.clone(),
            cacheLineThreadProperties.clone(),
        )?;
        sum = sum + locCoWrite;
    }
    if intGt(numberOfNodes, 0) {
        oLocCoWrite = realDiv(sum, metamodelica::OrderedFloat((numberOfNodes) as f64));
    } else {
        oLocCoWrite = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoWrite)
}

fn calculateLocCoForTask(
    mut iTaskIdx: i32,
    mut iThreadIdx: i32,
    mut iNodeSimCodeVarMapping: &metamodelica::List<i32>,
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>,
) -> Result<metamodelica::Real> {
    let mut oLocCo: metamodelica::Real;
    let mut simCodeVar: i32 = 0;
    let mut clIdx: i32;
    let mut sum: metamodelica::Real;
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut simCodeVar in &**iNodeSimCodeVarMapping {
        let mut simCodeVar = simCodeVar.clone();
        clIdx = Util::tuple21(metamodelica::arrayGet(iScVarCLMapping.clone(), simCodeVar)?);
        sum = sum
            + metamodelica::arrayGet(
                metamodelica::arrayGet(iCacheLineThreadProperties.clone(), clIdx)?,
                iThreadIdx,
            )?;
    }
    oLocCo = realDiv(sum, intReal(((iNodeSimCodeVarMapping).len() as i32)));
    Ok(oLocCo)
}

// -------------------------------------------
// MAPPINGS
// -------------------------------------------
fn fillSimVarHashTable(
    mut iSimVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iOffset: i32,
    mut iType: i32,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut oHt: (
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
    let mut tmpHashTable: (
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
    let mut simVar: metamodelica::Ref<SimCodeVar::SimVar> =
        <metamodelica::Ref<SimCodeVar::SimVar> as ::std::default::Default>::default();
    let mut index: i32;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    tmpHashTable = iHt;
    for mut simVar in &**iSimVars {
        let mut simVar = simVar.clone();
        let __arc2 = simVar;
        let SimCodeVar::SIMVAR {
            name: __pa0,
            index: __pa1,
            ..
        } = &*__arc2;
        name = metamodelica::Own::own(__pa0);
        index = metamodelica::Own::own(__pa1);
        index = index + 1;
        tmpHashTable = BaseHashTable::add((name, list![index, iOffset, iType]), tmpHashTable)?;
    }
    oHt = tmpHashTable;
    Ok(oHt)
}

fn transposeScVarTaskMapping(
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iTaskGraph: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpNodeSimCodeVarMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut scVarIdx: i32 = 0;
    let mut taskIdx: i32;
    let mut oldList: metamodelica::List<i32>;
    tmpNodeSimCodeVarMapping = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), metamodelica::nil());
    for mut scVarIdx in 1..=metamodelica::arrayLength(iScVarTaskMapping.clone()) {
        taskIdx = metamodelica::arrayGet(iScVarTaskMapping.clone(), scVarIdx)?;
        if intGt(taskIdx, 0) {
            oldList = metamodelica::arrayGet(tmpNodeSimCodeVarMapping.clone(), taskIdx)?;
            oldList = metamodelica::cons(scVarIdx, oldList);
            metamodelica::arrayUpdate(tmpNodeSimCodeVarMapping.clone(), taskIdx, oldList)?;
        }
    }
    oNodeSimCodeVarMapping = tmpNodeSimCodeVarMapping.clone();
    Ok(oNodeSimCodeVarMapping)
}

fn transposeTasksScVarsMapping(
    mut iTasksScVarMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iNumberOfScVars: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oScVarTasksMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpScVarTasksMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut scVarIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut oldList: metamodelica::List<i32>;
    let mut scVarIdc: metamodelica::List<i32>;
    tmpScVarTasksMapping = arrayCreate(iNumberOfScVars, metamodelica::nil());
    for mut taskIdx in 1..=metamodelica::arrayLength(iTasksScVarMapping.clone()) {
        scVarIdc = metamodelica::arrayGet(iTasksScVarMapping.clone(), taskIdx)?;
        for mut scVarIdx in &*scVarIdc {
            let mut scVarIdx = scVarIdx.clone();
            if intGt(scVarIdx, 0) {
                oldList = metamodelica::arrayGet(tmpScVarTasksMapping.clone(), scVarIdx)?;
                oldList = metamodelica::cons(taskIdx, oldList);
                metamodelica::arrayUpdate(tmpScVarTasksMapping.clone(), scVarIdx, oldList)?;
            }
        }
    }
    oScVarTasksMapping = tmpScVarTasksMapping.clone();
    Ok(oScVarTasksMapping)
}

fn getEqSCVarMapping(
    mut iEqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>>> {
    let mut oMapping: metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>>;
    let mut tmpMapping: metamodelica::List<metamodelica::Array<metamodelica::List<i32>>>;
    tmpMapping = List::map1(
        iEqSystems,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>,
               __a1: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
            ),
        )| getEqSCVarMappingByEqSystem(&__a0, __a1),
        iHt,
    )?;
    oMapping = metamodelica::arrayFromVec(tmpMapping.into_iter().cloned().collect());
    Ok(oMapping)
}

fn getEqSCVarMappingByEqSystem(
    mut iEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut equOptList: metamodelica::List<Option<metamodelica::Ref<BackendDAE::Equation>>>;
    let __arc1 = &(*iEqSystem);
    let BackendDAE::EQSYSTEM { orderedEqs: __pa0, .. } = &**__arc1;
    orderedEqs = metamodelica::Own::own(__pa0);
    equOptList = ExpandableArray::getData(orderedEqs)
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    oMapping = metamodelica::arrayFromVec(
        List::map1Option(&equOptList, &getEqSCVarMapping0, iHt)?
            .into_iter()
            .cloned()
            .collect(),
    );
    Ok(oMapping)
}

fn getEqSCVarMapping0(
    mut iEquation: metamodelica::Ref<BackendDAE::Equation>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::List<i32>> {
    let mut oMapping: metamodelica::List<i32>;
    let (_, (_, (_, __pa0))) = BackendEquation::traverseExpsOfEquation(
        iEquation,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                createMemoryMapTraverse0,
                metamodelica::Ref<DAE::Exp>,
                (
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<
                                Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>,
                            >
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
                            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>
                        )
                    ),
                    metamodelica::List<i32>
                )
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                (
                                    metamodelica::Array<
                                        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
                                    >,
                                    (
                                        i32,
                                        i32,
                                        metamodelica::Array<
                                            Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>,
                                        >,
                                    ),
                                    i32,
                                    (
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<DAE::ComponentRef>,
                                                    metamodelica::Ref<DAE::ComponentRef>,
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                                + 'static,
                                        >,
                                        Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
                                    ),
                                ),
                                metamodelica::List<i32>,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                (
                                    metamodelica::Array<
                                        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>,
                                    >,
                                    (
                                        i32,
                                        i32,
                                        metamodelica::Array<
                                            Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>,
                                        >,
                                    ),
                                    i32,
                                    (
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<DAE::ComponentRef>,
                                                    metamodelica::Ref<DAE::ComponentRef>,
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                                + 'static,
                                        >,
                                        Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
                                    ),
                                ),
                                metamodelica::List<i32>,
                            ),
                        )> + 'static,
                >),
            (iHt, metamodelica::nil()),
        ),
    )?;
    oMapping = metamodelica::Own::own(__pa0);
    Ok(oMapping)
}

fn createMemoryMapTraverse0(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<i32>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<i32>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut oTpl: (
        (
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
        metamodelica::List<i32>,
    );
    let mut iVarList: metamodelica::List<i32>;
    let mut oVarList: metamodelica::List<i32> = metamodelica::nil();
    let mut varInfo: metamodelica::List<i32> = metamodelica::nil();
    let mut varIdx: i32 = 0;
    let mut varHead: i32 = 0;
    let mut iHashTable: (
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
    let mut iExp: metamodelica::Ref<DAE::Exp>;
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    (outExp, oTpl) = 'mc: {
        let __mc_input = (inExp.clone(), &inTpl);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iExp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (iHashTable, iVarList)) => {
                    let mut iVarList = (*iVarList).clone();
                    let mut oVarList: metamodelica::List<i32> = oVarList.clone();
                    let mut varHead: i32 = varHead.clone();
                    let mut varIdx: i32 = varIdx.clone();
                    let mut varInfo: metamodelica::List<i32> = varInfo.clone();
                    varInfo = BaseHashTable::get(componentRef.clone(), &(iHashTable.clone()))?;
                    varIdx = (varInfo).head().cloned()? + List::second(&varInfo)?;
                    if boolNot((iVarList).is_empty()) {
                        varHead = (iVarList).head().cloned()?;
                        if intEq(varHead, varIdx) {
                            iVarList = (iVarList).rest()?;
                        }
                    }
                    varInfo = BaseHashTable::get(ComponentReference::crefPrefixDer(componentRef.clone()), &(iHashTable.clone()))?;
                    varIdx = (varInfo).head().cloned()? + List::second(&varInfo)?;
                    oVarList = metamodelica::cons(varIdx, iVarList.clone());
                    Ok(((iExp.clone(), (iHashTable.clone(), oVarList.clone())), oVarList.clone(), varHead.clone(), varIdx.clone(), varInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oVarList = __wb0;
            varHead = __wb1;
            varIdx = __wb2;
            varInfo = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iExp @ Deref @ DAE::Exp::CREF { componentRef, .. }, (iHashTable, iVarList)) => {
                    let mut oVarList: metamodelica::List<i32> = oVarList.clone();
                    let mut varIdx: i32 = varIdx.clone();
                    let mut varInfo: metamodelica::List<i32> = varInfo.clone();
                    varInfo = BaseHashTable::get(componentRef.clone(), &(iHashTable.clone()))?;
                    varIdx = (varInfo).head().cloned()? + List::second(&varInfo)?;
                    oVarList = metamodelica::cons(varIdx, iVarList.clone());
                    Ok(((iExp.clone(), (iHashTable.clone(), oVarList.clone())), oVarList.clone(), varIdx.clone(), varInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oVarList = __wb0;
            varIdx = __wb1;
            varInfo = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, oTpl)
}

fn getSimCodeVarNodeMapping(
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iNumScVars: i32,
    mut iCompNodeMapping: metamodelica::Array<i32>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Array<i32>> {
    let mut oScVarTaskMapping: metamodelica::Array<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut scVarTaskMapping: metamodelica::Array<i32>;
    scVarTaskMapping = arrayCreate(iNumScVars, -1);
    let HpcOmTaskGraph::TASKGRAPHMETA {
        varCompMapping: __pa0, ..
    } = iTaskGraphMeta;
    varCompMapping = metamodelica::Own::own(__pa0);
    (oScVarTaskMapping, _) = Array::fold(
        varCompMapping.clone(),
        &({
            let __pe_b1 = iEqSystems.clone();
            let __pe_b2 = iVarNameSCVarIdxMapping.clone();
            let __pe_b3 = iCompNodeMapping.clone();
            move |__pe_a0, __pe_a4| {
                Ok(getSimCodeVarNodeMapping0(
                    __pe_a0,
                    &__pe_b1,
                    &__pe_b2,
                    __pe_b3.clone(),
                    __pe_a4,
                ))
            }
        }),
        (scVarTaskMapping.clone(), 1),
    )?;
    Ok(oScVarTaskMapping)
}

fn getSimCodeVarNodeMapping0(
    mut iCompIdx: (i32, i32, i32),
    mut iEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iCompNodeMapping: metamodelica::Array<i32>,
    mut iScVarTaskMappingVarIdx: (metamodelica::Array<i32>, i32),
) -> (metamodelica::Array<i32>, i32) {
    let mut oScVarTaskMappingVarIdx: (metamodelica::Array<i32>, i32);
    let mut iScVarTaskMapping: metamodelica::Array<i32>;
    let mut varIdx: i32;
    let mut eqSysIdx: i32;
    let mut varOffset: i32;
    let mut scVarIdx: i32 = 0;
    let mut compIdx: i32;
    let mut nodeIdx: i32 = 0;
    let mut scVarOffset: i32 = 0;
    let mut eqSystem: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut var: metamodelica::Ref<BackendDAE::Var> =
        <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
    let mut varName: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut scVarValues: metamodelica::List<i32> = metamodelica::nil();
    let mut varNameString: ArcStr = arcstr::literal!("");
    oScVarTaskMappingVarIdx = 'mc: {
        let __mc_input = (iCompIdx, iScVarTaskMappingVarIdx);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let ((mut compIdx, mut eqSysIdx, mut varOffset), (mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut eqSystem: metamodelica::Ref<BackendDAE::EqSystem> = eqSystem.clone();
            let mut nodeIdx: i32 = nodeIdx.clone();
            let mut orderedVars: BackendDAE::Variables = orderedVars.clone();
            let mut scVarIdx: i32 = scVarIdx.clone();
            let mut scVarOffset: i32 = scVarOffset.clone();
            let mut scVarValues: metamodelica::List<i32> = scVarValues.clone();
            let mut var: metamodelica::Ref<BackendDAE::Var> = var.clone();
            let mut varName: metamodelica::Ref<DAE::ComponentRef> = varName.clone();
            let mut varNameString: ArcStr = varNameString.clone();
            let true = (intGt(compIdx, 0)) else {
                return Err("pattern mismatch");
            };
            eqSystem = (iEqSystems).get(eqSysIdx)?;
            let __arc1 = eqSystem.clone();
            let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &*__arc1;
            orderedVars = metamodelica::Own::own(__pa0);
            var = BackendVariable::getVarAt(&orderedVars, varIdx - varOffset)?;
            let __arc3 = var.clone();
            let BackendDAE::VAR { varName: __pa2, .. } = &*__arc3;
            varName = metamodelica::Own::own(__pa2);
            varName = getModifiedVarName(&var);
            scVarValues = BaseHashTable::get(varName.clone(), iVarNameSCVarIdxMapping)?;
            varNameString = ComponentReferenceBasics::printComponentRefStr(&varName)?;
            scVarIdx = (scVarValues).head().cloned()?;
            scVarOffset = List::second(&scVarValues)?;
            scVarIdx = scVarIdx + scVarOffset;
            nodeIdx = metamodelica::arrayGet(iCompNodeMapping.clone(), compIdx)?;
            iScVarTaskMapping = metamodelica::arrayUpdate(iScVarTaskMapping.clone(), scVarIdx, nodeIdx)?;
            Ok((
                (iScVarTaskMapping.clone(), varIdx + 1),
                eqSystem.clone(),
                nodeIdx.clone(),
                orderedVars.clone(),
                scVarIdx.clone(),
                scVarOffset.clone(),
                scVarValues.clone(),
                var.clone(),
                varName.clone(),
                varNameString.clone(),
            ))
        })() {
            eqSystem = __wb0;
            nodeIdx = __wb1;
            orderedVars = __wb2;
            scVarIdx = __wb3;
            scVarOffset = __wb4;
            scVarValues = __wb5;
            var = __wb6;
            varName = __wb7;
            varNameString = __wb8;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, (mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((iScVarTaskMapping.clone(), varIdx + 1))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oScVarTaskMappingVarIdx
}

fn invertEqCompMapping(
    mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut iNumOfComps: i32,
) -> Result<metamodelica::Array<metamodelica::List<(i32, i32, i32)>>> {
    let mut oCompEqMapping: metamodelica::Array<metamodelica::List<(i32, i32, i32)>>;
    let mut tmpCompEqMapping: metamodelica::Array<metamodelica::List<(i32, i32, i32)>>;
    let mut eqIdx: i32 = 0;
    let mut compIdx: i32;
    let mut eqSystemIdx: i32;
    let mut offset: i32;
    let mut compEqEntry: metamodelica::List<(i32, i32, i32)>;
    tmpCompEqMapping = arrayCreate(iNumOfComps, metamodelica::nil());
    for mut eqIdx in 1..=metamodelica::arrayLength(iEqCompMapping.clone()) {
        (compIdx, eqSystemIdx, offset) = metamodelica::arrayGet(iEqCompMapping.clone(), eqIdx)?;
        compEqEntry = metamodelica::arrayGet(tmpCompEqMapping.clone(), compIdx)?;
        tmpCompEqMapping = metamodelica::arrayUpdate(
            tmpCompEqMapping.clone(),
            compIdx,
            metamodelica::cons((eqIdx, eqSystemIdx, offset), compEqEntry),
        )?;
    }
    oCompEqMapping = tmpCompEqMapping.clone();
    Ok(oCompEqMapping)
}

fn invertSccNodeMapping(
    mut iSccNodeMapping: metamodelica::Array<i32>,
    mut iNumberOfNodes: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut oNodeSccMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpNodeSccMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut sccIdx: i32 = 0;
    let mut nodeIdx: i32;
    let mut nodeSccEntry: metamodelica::List<i32>;
    tmpNodeSccMapping = arrayCreate(iNumberOfNodes, metamodelica::nil());
    for mut sccIdx in 1..=metamodelica::arrayLength(iSccNodeMapping.clone()) {
        nodeIdx = metamodelica::arrayGet(iSccNodeMapping.clone(), sccIdx)?;
        if intGt(nodeIdx, 0) {
            nodeSccEntry = metamodelica::arrayGet(tmpNodeSccMapping.clone(), nodeIdx)?;
            tmpNodeSccMapping = metamodelica::arrayUpdate(
                tmpNodeSccMapping.clone(),
                nodeIdx,
                metamodelica::cons(sccIdx, nodeSccEntry),
            )?;
        }
    }
    oNodeSccMapping = tmpNodeSccMapping.clone();
    Ok(oNodeSccMapping)
}

fn flattenEqSimCodeVarMapping(
    mut iEqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>>,
) -> Result<metamodelica::Array<(i32, metamodelica::List<i32>)>> {
    let mut oFlatEqSimCodeVarMapping: metamodelica::Array<(i32, metamodelica::List<i32>)>;
    let mut simCodeVarList: metamodelica::List<i32>;
    let mut tmpFlatEqSimCodeVarMapping: metamodelica::Array<(i32, metamodelica::List<i32>)>;
    let mut eqCount: i32;
    let mut eqIdx: i32;
    let mut eqSysIdx: i32 = 0;
    let mut eqSimCodeVarIdx: i32 = 0;
    let mut eqSimCodeVarMappingEntry: metamodelica::Array<metamodelica::List<i32>>;
    eqCount = 0;
    for mut eqSysIdx in 1..=metamodelica::arrayLength(iEqSimCodeVarMapping.clone()) {
        eqSimCodeVarMappingEntry = metamodelica::arrayGet(iEqSimCodeVarMapping.clone(), eqSysIdx)?;
        eqCount = eqCount + metamodelica::arrayLength(eqSimCodeVarMappingEntry.clone());
    }
    eqIdx = 1;
    tmpFlatEqSimCodeVarMapping = arrayCreate(eqCount, (-1, metamodelica::nil()));
    for mut eqSysIdx in 1..=metamodelica::arrayLength(iEqSimCodeVarMapping.clone()) {
        eqSimCodeVarMappingEntry = metamodelica::arrayGet(iEqSimCodeVarMapping.clone(), eqSysIdx)?;
        for mut eqSimCodeVarIdx in 1..=metamodelica::arrayLength(eqSimCodeVarMappingEntry.clone()) {
            simCodeVarList = metamodelica::arrayGet(eqSimCodeVarMappingEntry.clone(), eqSimCodeVarIdx)?;
            tmpFlatEqSimCodeVarMapping =
                metamodelica::arrayUpdate(tmpFlatEqSimCodeVarMapping.clone(), eqIdx, (eqSysIdx, simCodeVarList))?;
            eqIdx = eqIdx + 1;
        }
    }
    oFlatEqSimCodeVarMapping = tmpFlatEqSimCodeVarMapping.clone();
    Ok(oFlatEqSimCodeVarMapping)
}

fn getModifiedVarName(mut iVar: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut oVarName: metamodelica::Ref<DAE::ComponentRef>;
    let mut iVarName: metamodelica::Ref<DAE::ComponentRef>;
    let mut tmpVarName: metamodelica::Ref<DAE::ComponentRef>;
    let mut varKind: BackendDAE::VarKind;
    oVarName = (match &**iVar {
        BackendDAE::Var {
            varName: __esc_iVarName,
            varKind: BackendDAE::VarKind::STATE { index: 1, .. },
            ..
        } => {
            iVarName = (*__esc_iVarName).clone();
            tmpVarName = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: arcstr::literal!(DAE::derivativeNamePrefix),
                identType: metamodelica::Ref::new(DAE::Type::T_REAL {
                    varLst: metamodelica::nil(),
                }),
                subscriptLst: metamodelica::nil(),
                componentRef: iVarName.clone(),
            });
            tmpVarName
        }
        BackendDAE::Var {
            varName: __esc_iVarName,
            varKind: __esc_varKind,
            ..
        } => {
            iVarName = (*__esc_iVarName).clone();
            varKind = (*__esc_varKind).clone();
            tmpVarName = iVarName.clone();
            tmpVarName
        }
    });
    oVarName
}

fn getCacheLineTaskMapping(
    mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta,
    mut iEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iNumCacheLines: i32,
    mut iSCVarCLMapping: metamodelica::Array<(i32, i32)>,
) -> Result<(metamodelica::Array<metamodelica::List<i32>>, metamodelica::Array<i32>)> {
    let mut oCLTaskMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut oScVarTaskMapping: metamodelica::Array<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut tmpCLTaskMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut scVarTaskMapping: metamodelica::Array<i32>;
    tmpCLTaskMapping = arrayCreate(iNumCacheLines, metamodelica::nil());
    scVarTaskMapping = arrayCreate(metamodelica::arrayLength(iSCVarCLMapping.clone()), -1);
    let HpcOmTaskGraph::TASKGRAPHMETA {
        varCompMapping: __pa0, ..
    } = iTaskGraphMeta;
    varCompMapping = metamodelica::Own::own(__pa0);
    (tmpCLTaskMapping, oScVarTaskMapping, _) = Array::fold(
        varCompMapping.clone(),
        &({
            let __pe_b1 = iEqSystems.clone();
            let __pe_b2 = iVarNameSCVarIdxMapping.clone();
            let __pe_b3 = iSCVarCLMapping.clone();
            move |__pe_a0, __pe_a4| {
                Ok(getCacheLineTaskMapping0(
                    __pe_a0,
                    &__pe_b1,
                    &__pe_b2,
                    __pe_b3.clone(),
                    __pe_a4,
                ))
            }
        }),
        (tmpCLTaskMapping.clone(), scVarTaskMapping.clone(), 1),
    )?;
    tmpCLTaskMapping = Array::map1(
        tmpCLTaskMapping.clone(),
        &List::sort,
        (std::sync::Arc::new(fnptr!(intLt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    oCLTaskMapping = Array::map1(
        tmpCLTaskMapping.clone(),
        &move |__a0: _, __a1: _| List::sortedUnique(__a0, metamodelica::arc_ref(&__a1)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    Ok((oCLTaskMapping, oScVarTaskMapping))
}

fn getCacheLineTaskMapping0(
    mut iNodeIdx: (i32, i32, i32),
    mut iEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iSCVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iCLTaskMappingVarIdx: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    ),
) -> (
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<i32>,
    i32,
) {
    let mut oCLTaskMappingVarIdx: (
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<i32>,
        i32,
    );
    let mut iClTaskMapping: metamodelica::Array<metamodelica::List<i32>>;
    let mut iScVarTaskMapping: metamodelica::Array<i32>;
    let mut varIdx: i32;
    let mut eqSysIdx: i32;
    let mut varOffset: i32;
    let mut scVarIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut nodeIdx: i32;
    let mut scVarOffset: i32 = 0;
    let mut eqSystem: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut var: metamodelica::Ref<BackendDAE::Var> =
        <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
    let mut varName: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut oldVal: metamodelica::List<i32> = metamodelica::nil();
    let mut scVarValues: metamodelica::List<i32> = metamodelica::nil();
    oCLTaskMappingVarIdx = 'mc: {
        let __mc_input = (iNodeIdx, iCLTaskMappingVarIdx);
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let ((mut nodeIdx, mut eqSysIdx, mut varOffset), (mut iClTaskMapping, mut iScVarTaskMapping, mut varIdx)) =
                __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut clIdx: i32 = clIdx.clone();
            let mut eqSystem: metamodelica::Ref<BackendDAE::EqSystem> = eqSystem.clone();
            let mut oldVal: metamodelica::List<i32> = oldVal.clone();
            let mut orderedVars: BackendDAE::Variables = orderedVars.clone();
            let mut scVarIdx: i32 = scVarIdx.clone();
            let mut scVarOffset: i32 = scVarOffset.clone();
            let mut scVarValues: metamodelica::List<i32> = scVarValues.clone();
            let mut var: metamodelica::Ref<BackendDAE::Var> = var.clone();
            let mut varName: metamodelica::Ref<DAE::ComponentRef> = varName.clone();
            let true = (intGt(nodeIdx, 0)) else {
                return Err("pattern mismatch");
            };
            eqSystem = (iEqSystems).get(eqSysIdx)?;
            let __arc1 = eqSystem.clone();
            let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &*__arc1;
            orderedVars = metamodelica::Own::own(__pa0);
            var = BackendVariable::getVarAt(&orderedVars, varIdx - varOffset)?;
            let __arc3 = var.clone();
            let BackendDAE::VAR { varName: __pa2, .. } = &*__arc3;
            varName = metamodelica::Own::own(__pa2);
            varName = getModifiedVarName(&var);
            scVarValues = BaseHashTable::get(varName.clone(), iVarNameSCVarIdxMapping)?;
            scVarIdx = (scVarValues).head().cloned()?;
            scVarOffset = List::second(&scVarValues)?;
            scVarIdx = scVarIdx + scVarOffset;
            (clIdx, _) = metamodelica::arrayGet(iSCVarCLMapping.clone(), scVarIdx)?;
            oldVal = metamodelica::arrayGet(iClTaskMapping.clone(), clIdx)?;
            iClTaskMapping = metamodelica::arrayUpdate(
                iClTaskMapping.clone(),
                clIdx,
                metamodelica::cons(nodeIdx, oldVal.clone()),
            )?;
            iScVarTaskMapping = metamodelica::arrayUpdate(iScVarTaskMapping.clone(), scVarIdx, nodeIdx)?;
            Ok((
                (iClTaskMapping.clone(), iScVarTaskMapping.clone(), varIdx + 1),
                clIdx.clone(),
                eqSystem.clone(),
                oldVal.clone(),
                orderedVars.clone(),
                scVarIdx.clone(),
                scVarOffset.clone(),
                scVarValues.clone(),
                var.clone(),
                varName.clone(),
            ))
        })() {
            clIdx = __wb0;
            eqSystem = __wb1;
            oldVal = __wb2;
            orderedVars = __wb3;
            scVarIdx = __wb4;
            scVarOffset = __wb5;
            scVarValues = __wb6;
            var = __wb7;
            varName = __wb8;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, (mut iClTaskMapping, mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((iClTaskMapping.clone(), iScVarTaskMapping.clone(), varIdx + 1))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCLTaskMappingVarIdx
}

fn getTaskSimVarMapping(
    mut iSccEqMapping: metamodelica::Array<metamodelica::List<(i32, i32, i32)>>,
    mut iNodeSccMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iEqSimCodeVarMapping: metamodelica::Array<(i32, metamodelica::List<i32>)>,
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut oSolvedVars: metamodelica::Array<metamodelica::List<i32>>;
    let mut oNotSolvedVars: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpSolvedVars: metamodelica::Array<metamodelica::List<i32>>;
    let mut tmpNotSolvedVars: metamodelica::Array<metamodelica::List<i32>>;
    let mut scVarMarks: metamodelica::Array<i32>;
    let mut scSolvedVarMarks: metamodelica::Array<i32>;
    let mut nodeSccs: metamodelica::List<i32>;
    let mut eqVars: metamodelica::List<i32>;
    let mut nodeIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut eqIdx: i32;
    let mut var: i32;
    let mut varTask: i32;
    let mut varMark: i32;
    let mut varType: i32;
    let mut nvar: i32;
    let mut var: i32;
    let mut sccEqs: metamodelica::List<(i32, i32, i32)>;
    let mut sccEq: (i32, i32, i32) = (0, 0, 0);
    match '__try0: {
        tmpSolvedVars = arrayCreate(metamodelica::arrayLength(iNodeSccMapping.clone()), metamodelica::nil());
        tmpNotSolvedVars = arrayCreate(metamodelica::arrayLength(iNodeSccMapping.clone()), metamodelica::nil());
        scVarMarks = arrayCreate(metamodelica::arrayLength(iScVarTaskMapping.clone()), -1);
        scSolvedVarMarks = arrayCreate(metamodelica::arrayLength(iScVarTaskMapping.clone()), -1);
        nvar = metamodelica::arrayLength(iScVarTaskMapping.clone());
        for mut nodeIdx in 1..=metamodelica::arrayLength(iNodeSccMapping.clone()) {
            nodeSccs = unwrap_break_err!(metamodelica::arrayGet(iNodeSccMapping.clone(), nodeIdx), '__try0);
            for mut sccIdx in &*nodeSccs {
                let mut sccIdx = sccIdx.clone();
                sccEqs = unwrap_break_err!(metamodelica::arrayGet(iSccEqMapping.clone(), sccIdx), '__try0);
                for mut sccEq in &*sccEqs {
                    let mut sccEq = sccEq.clone();
                    (eqIdx, _, _) = sccEq;
                    (_, eqVars) =
                        unwrap_break_err!(metamodelica::arrayGet(iEqSimCodeVarMapping.clone(), eqIdx), '__try0);
                    for mut v2 in &*eqVars {
                        var = if (v2.clone() > nvar) {
                            v2.clone() - nvar
                        } else {
                            v2.clone()
                        };
                        varTask = unwrap_break_err!(metamodelica::arrayGet(iScVarTaskMapping.clone(), var), '__try0);
                        varType = Util::tuple31(
                            unwrap_break_err!(metamodelica::arrayGet(iSimCodeVarTypes.clone(), var), '__try0),
                        );
                        if intGt(varType, 0) {
                            if intEq(nodeIdx, varTask) {
                                varMark =
                                    unwrap_break_err!(metamodelica::arrayGet(scSolvedVarMarks.clone(), var), '__try0);
                                if intNe(varMark, nodeIdx) {
                                    tmpSolvedVars = unwrap_break_err!(metamodelica::arrayUpdate(tmpSolvedVars.clone(), nodeIdx, metamodelica::cons(var, unwrap_break_err!(metamodelica::arrayGet(tmpSolvedVars.clone(), nodeIdx), '__try0))), '__try0);
                                    scSolvedVarMarks = unwrap_break_err!(metamodelica::arrayUpdate(scSolvedVarMarks.clone(), var, nodeIdx), '__try0);
                                }
                            } else {
                                varMark = unwrap_break_err!(metamodelica::arrayGet(scVarMarks.clone(), var), '__try0);
                                if intNe(varMark, nodeIdx) {
                                    tmpNotSolvedVars = unwrap_break_err!(metamodelica::arrayUpdate(tmpNotSolvedVars.clone(), nodeIdx, metamodelica::cons(var, unwrap_break_err!(metamodelica::arrayGet(tmpNotSolvedVars.clone(), nodeIdx), '__try0))), '__try0);
                                    scVarMarks = unwrap_break_err!(metamodelica::arrayUpdate(scVarMarks.clone(), var, nodeIdx), '__try0);
                                }
                            }
                        }
                    }
                }
            }
        }
        oSolvedVars = tmpSolvedVars.clone();
        oNotSolvedVars = tmpNotSolvedVars.clone();
        Ok::<_, &'static str>((
            nvar.clone(),
            oNotSolvedVars.clone(),
            oSolvedVars.clone(),
            scSolvedVarMarks.clone(),
            scVarMarks.clone(),
            tmpNotSolvedVars.clone(),
            tmpSolvedVars.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            nvar = __try0_o0;
            oNotSolvedVars = __try0_o1;
            oSolvedVars = __try0_o2;
            scSolvedVarMarks = __try0_o3;
            scVarMarks = __try0_o4;
            tmpNotSolvedVars = __try0_o5;
            tmpSolvedVars = __try0_o6;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("HpcOmMemory.getTaskSimVarMapping"));
                    __mm_s.push_str(&*literal!(" failed"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/HpcOmMemory.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok((oSolvedVars, oNotSolvedVars))
}

// -------------------------------------------
// GRAPH
// -------------------------------------------
fn appendCacheLinesToGraph(
    mut iCacheMap: &CacheMap,
    mut iNumberOfNodes: i32,
    mut iEqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>>,
    mut iEqSystems: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut ieqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iThreadIdAttributeIdx: i32,
    mut iCompNodeMapping: metamodelica::Array<i32>,
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut clGroupNodeIdx: i32 = 0;
    let mut graphCount: i32;
    let mut tmpGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut knownEdges: metamodelica::Array<metamodelica::List<i32>> = Default::default();
    let mut addedVariables: metamodelica::Array<bool> = Default::default();
    let mut cacheVariables: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>> = Default::default();
    let mut cacheLines: metamodelica::List<CacheLineMap> = metamodelica::nil();
    oGraphInfo = 'mc: {
        let __mc_input = iGraphInfo.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let GraphML::GraphInfo::GRAPHINFO {
                graphCount: mut graphCount,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut addedVariables: metamodelica::Array<bool> = addedVariables.clone();
            let mut cacheLines: metamodelica::List<CacheLineMap> = cacheLines.clone();
            let mut cacheVariables: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>> = cacheVariables.clone();
            let mut clGroupNodeIdx: i32 = clGroupNodeIdx.clone();
            let mut knownEdges: metamodelica::Array<metamodelica::List<i32>> = knownEdges.clone();
            let mut tmpGraphInfo: GraphML::GraphInfo = tmpGraphInfo.clone();
            let true = (intLe(1, graphCount)) else {
                return Err("pattern mismatch");
            };
            knownEdges = arrayCreate(iNumberOfNodes, metamodelica::nil());
            addedVariables = arrayCreate(metamodelica::arrayLength(iScVarTaskMapping.clone()), false);
            let (__pa0, _, (_, __pa1)) =
                GraphML::addGroupNode(literal!("CL_GoupNode"), 1, false, literal!("CL"), iGraphInfo.clone())?;
            tmpGraphInfo = metamodelica::Own::own(__pa0);
            clGroupNodeIdx = metamodelica::Own::own(__pa1);
            cacheLines = getAllCacheLinesOfCacheMap(iCacheMap);
            cacheVariables =
                metamodelica::arrayFromVec(getCacheVariablesOfCacheMap(iCacheMap).into_iter().cloned().collect());
            tmpGraphInfo = List::fold(
                &cacheLines,
                &({
                    let __pe_b1 = cacheVariables.clone();
                    let __pe_b2 = addedVariables.clone();
                    let __pe_b3 = iSchedulerInfo.clone();
                    let __pe_b4 = (clGroupNodeIdx, iThreadIdAttributeIdx);
                    let __pe_b5 = iScVarTaskMapping.clone();
                    let __pe_b6 = iVarNameSCVarIdxMapping.clone();
                    let __pe_b7 = iScVarInfos.clone();
                    move |__pe_a0, __pe_a8| {
                        appendCacheLineMapToGraph(
                            __pe_a0,
                            __pe_b1.clone(),
                            __pe_b2.clone(),
                            __pe_b3.clone(),
                            __pe_b4.clone(),
                            __pe_b5.clone(),
                            &__pe_b6,
                            __pe_b7.clone(),
                            __pe_a8,
                        )
                    }
                }),
                tmpGraphInfo.clone(),
            )?;
            tmpGraphInfo = appendTaskVarEdgesToGraph(
                iTaskSolvedVarsMapping.clone(),
                iTaskUnsolvedVarsMapping.clone(),
                tmpGraphInfo.clone(),
            )?;
            Ok((
                tmpGraphInfo.clone(),
                addedVariables.clone(),
                cacheLines.clone(),
                cacheVariables.clone(),
                clGroupNodeIdx.clone(),
                knownEdges.clone(),
                tmpGraphInfo.clone(),
            ))
        })() {
            addedVariables = __wb0;
            cacheLines = __wb1;
            cacheVariables = __wb2;
            clGroupNodeIdx = __wb3;
            knownEdges = __wb4;
            tmpGraphInfo = __wb5;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let GraphML::GraphInfo::GRAPHINFO {
                graphCount: mut graphCount,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (intEq(graphCount, 0)) else {
                return Err("pattern mismatch");
            };
            Ok(iGraphInfo.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("HpcOmSimCode.appendCacheLinesToGraph failed!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oGraphInfo)
}

fn appendVariablesToGraph(
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iNumberOfScVars: i32,
    mut iGraphIdx: i32,
    mut iThreadIdAttributeIdx: i32,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iAllVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo;
    let mut description: ArcStr;
    let mut threadText: ArcStr;
    let mut simVarOpt: Option<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut simVar: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut varCompRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut nodeLabel: GraphML::NodeLabel;
    let mut isValidVar: bool;
    let mut realScVarIdxOffset: metamodelica::List<i32>;
    let mut realScVarIdx: i32;
    let mut realScVarOffset: i32;
    let mut threadOwner: i32;
    for mut varIdx in 1..=iNumberOfScVars {
        isValidVar = true;
        simVarOpt = metamodelica::arrayGet(iAllVarsMapping.clone(), varIdx)?;
        description = literal!("unknown");
        threadText = literal!("Th -1");
        if (simVarOpt).is_some() {
            simVar = simVarOpt.ok_or("pattern mismatch")?;
            varCompRef = simVar.name.clone();
            description = ComponentReferenceBasics::printComponentRefStr(&varCompRef)?;
            isValidVar = BaseHashTable::hasKey(varCompRef.clone(), iVarNameSCVarIdxMapping)?;
            if BaseHashTable::hasKey(varCompRef.clone(), iVarNameSCVarIdxMapping)? {
                realScVarIdxOffset = BaseHashTable::get(varCompRef, iVarNameSCVarIdxMapping)?;
                realScVarIdx = (realScVarIdxOffset).get(1)?;
                realScVarOffset = (realScVarIdxOffset).get(2)?;
                realScVarIdx = realScVarIdx + realScVarOffset;
                let ScVarInfo { ownerThread: __pa0, .. } = metamodelica::arrayGet(iScVarInfos.clone(), realScVarIdx)?;
                threadOwner = metamodelica::Own::own(__pa0);
                threadText = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Th "));
                    __mm_s.push_str(&*intString(threadOwner));
                    ArcStr::from(__mm_s)
                };
            }
        }
        if isValidVar {
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: intString(varIdx),
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (tmpGraphInfo, _) = GraphML::addNode(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("var"));
                    __mm_s.push_str(&*intString(varIdx));
                    ArcStr::from(__mm_s)
                },
                arcstr::literal!(GraphML::COLOR_GREEN2),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![nodeLabel],
                openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE,
                Some(description),
                list![(iThreadIdAttributeIdx, threadText)],
                iGraphIdx,
                tmpGraphInfo,
            )?;
        }
    }
    tmpGraphInfo = appendTaskVarEdgesToGraph(
        iTaskSolvedVarsMapping.clone(),
        iTaskUnsolvedVarsMapping.clone(),
        tmpGraphInfo,
    )?;
    oGraphInfo = tmpGraphInfo;
    Ok(oGraphInfo)
}

fn appendTaskVarEdgesToGraph(
    mut iTaskSolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iTaskUnsolvedVarsMapping: metamodelica::Array<metamodelica::List<i32>>,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo;
    let mut taskIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut taskVarList: metamodelica::List<i32>;
    for mut taskIdx in 1..=metamodelica::arrayLength(iTaskSolvedVarsMapping.clone()) {
        taskVarList = metamodelica::arrayGet(iTaskSolvedVarsMapping.clone(), taskIdx)?;
        for mut varIdx in &*taskVarList {
            let mut varIdx = varIdx.clone();
            (tmpGraphInfo, _) = GraphML::addEdge(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("varEdge_"));
                    __mm_s.push_str(&*intString(taskIdx));
                    __mm_s.push_str(&*literal!("_"));
                    __mm_s.push_str(&*intString(varIdx));
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("var"));
                    __mm_s.push_str(&*intString(varIdx));
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Node"));
                    __mm_s.push_str(&*intString(taskIdx));
                    ArcStr::from(__mm_s)
                },
                arcstr::literal!(GraphML::COLOR_BLACK),
                openmodelica_codegen_graphml::GraphML::LineType::LINE,
                GraphML::LINEWIDTH_STANDARD.clone(),
                false,
                metamodelica::nil(),
                (
                    openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
                    openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
                ),
                metamodelica::nil(),
                tmpGraphInfo,
            )?;
        }
    }
    for mut taskIdx in 1..=metamodelica::arrayLength(iTaskUnsolvedVarsMapping.clone()) {
        taskVarList = metamodelica::arrayGet(iTaskUnsolvedVarsMapping.clone(), taskIdx)?;
        for mut varIdx in &*taskVarList {
            let mut varIdx = varIdx.clone();
            (tmpGraphInfo, _) = GraphML::addEdge(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("varEdge_"));
                    __mm_s.push_str(&*intString(taskIdx));
                    __mm_s.push_str(&*literal!("_"));
                    __mm_s.push_str(&*intString(varIdx));
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Node"));
                    __mm_s.push_str(&*intString(taskIdx));
                    ArcStr::from(__mm_s)
                },
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("var"));
                    __mm_s.push_str(&*intString(varIdx));
                    ArcStr::from(__mm_s)
                },
                arcstr::literal!(GraphML::COLOR_BLACK),
                openmodelica_codegen_graphml::GraphML::LineType::LINE,
                GraphML::LINEWIDTH_STANDARD.clone(),
                false,
                metamodelica::nil(),
                (
                    openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE,
                    openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART,
                ),
                metamodelica::nil(),
                tmpGraphInfo,
            )?;
        }
    }
    oGraphInfo = tmpGraphInfo;
    Ok(oGraphInfo)
}

fn appendUnmappedVariablesToGraph(
    mut iScVarCLMapping: metamodelica::Array<(i32, i32)>,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo;
    let mut scVarIdx: i32 = 0;
    let mut clIdx: i32;
    for mut scVarIdx in 1..=metamodelica::arrayLength(iScVarCLMapping.clone()) {
        (clIdx, _) = metamodelica::arrayGet(iScVarCLMapping.clone(), scVarIdx)?;
        if intLt(clIdx, 1) {}
    }
    oGraphInfo = tmpGraphInfo;
    Ok(oGraphInfo)
}

fn appendCacheLineMapToGraph(
    mut iCacheLineMap: CacheLineMap,
    mut iCacheVariables: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iAddedVariables: metamodelica::Array<bool>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iTopGraphAttThreadIdIdx: (i32, i32),
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut idx: i32;
    let mut graphIdx: i32;
    let mut iTopGraphIdx: i32;
    let mut iAttThreadIdIdx: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let mut entry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut tmpGraphInfo: GraphML::GraphInfo;
    let mut entryThreadOwner: i32;
    let mut notOnlyParamters: bool;
    let CacheLineMap {
        idx: __pa0,
        entries: __pa1,
        ..
    } = iCacheLineMap;
    idx = metamodelica::Own::own(__pa0);
    entries = metamodelica::Own::own(__pa1);
    notOnlyParamters = false;
    for mut entry in &*entries {
        let mut entry = entry.clone();
        let CacheLineEntry { threadOwner: __pa2, .. } = entry;
        entryThreadOwner = metamodelica::Own::own(__pa2);
        notOnlyParamters = boolOr(notOnlyParamters, intNe(entryThreadOwner, -1));
    }
    if notOnlyParamters {
        (iTopGraphIdx, iAttThreadIdIdx) = iTopGraphAttThreadIdIdx;
        let (__pa3, _, (_, __pa4)) = GraphML::addGroupNode(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("CL_Meta_"));
                __mm_s.push_str(&*intString(idx));
                ArcStr::from(__mm_s)
            },
            iTopGraphIdx,
            true,
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("CL"));
                __mm_s.push_str(&*intString(idx));
                ArcStr::from(__mm_s)
            },
            iGraphInfo,
        )?;
        tmpGraphInfo = metamodelica::Own::own(__pa3);
        graphIdx = metamodelica::Own::own(__pa4);
        oGraphInfo = List::fold(
            &entries,
            &({
                let __pe_b1 = iCacheVariables.clone();
                let __pe_b2 = iAddedVariables.clone();
                let __pe_b3 = iSchedulerInfo.clone();
                let __pe_b4 = (graphIdx, iAttThreadIdIdx);
                let __pe_b5 = iScVarTaskMapping.clone();
                let __pe_b6 = iVarNameSCVarIdxMapping.clone();
                let __pe_b7 = iScVarInfos.clone();
                move |__pe_a0, __pe_a8| {
                    appendCacheLineEntryToGraph(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_b5.clone(),
                        &__pe_b6,
                        __pe_b7.clone(),
                        __pe_a8,
                    )
                }
            }),
            tmpGraphInfo,
        )?;
    } else {
        oGraphInfo = iGraphInfo;
    }
    Ok(oGraphInfo)
}

fn appendCacheLineEntryToGraph(
    mut iCacheLineEntry: CacheLineEntry,
    mut iCacheVariables: metamodelica::Array<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iAddedVariables: metamodelica::Array<bool>,
    mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>,
    mut iTopGraphAttThreadIdIdx: (i32, i32),
    mut iScVarTaskMapping: metamodelica::Array<i32>,
    mut iVarNameSCVarIdxMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iScVarInfos: metamodelica::Array<ScVarInfo>,
    mut iGraphInfo: GraphML::GraphInfo,
) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut realScVarIdxOffset: metamodelica::List<i32>;
    let mut scVarIdx: i32;
    let mut realScVarIdx: i32;
    let mut realScVarOffset: i32;
    let mut taskIdx: i32;
    let mut iTopGraphIdx: i32;
    let mut iAttThreadIdIdx: i32;
    let mut threadOwner: i32;
    let mut varString: ArcStr;
    let mut threadText: ArcStr;
    let mut nodeLabelText: ArcStr;
    let mut nodeId: ArcStr;
    let mut nodeLabel: GraphML::NodeLabel;
    let mut iVar: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let CacheLineEntry {
        scVarIdx: __pa0,
        threadOwner: __pa1,
        ..
    } = iCacheLineEntry;
    scVarIdx = metamodelica::Own::own(__pa0);
    threadOwner = metamodelica::Own::own(__pa1);
    (iTopGraphIdx, iAttThreadIdIdx) = iTopGraphAttThreadIdIdx;
    if intGe(metamodelica::arrayLength(iCacheVariables.clone()) - scVarIdx + 1, 1) {
        iVar = metamodelica::arrayGet(
            iCacheVariables.clone(),
            metamodelica::arrayLength(iCacheVariables.clone()) - scVarIdx + 1,
        )?;
        let __arc3 = iVar;
        let SimCodeVar::SIMVAR { name: __pa2, .. } = &*__arc3;
        name = metamodelica::Own::own(__pa2);
        if BaseHashTable::hasKey(name.clone(), iVarNameSCVarIdxMapping)? {
            realScVarIdxOffset = BaseHashTable::get(name.clone(), iVarNameSCVarIdxMapping)?;
            realScVarIdx = (realScVarIdxOffset).get(1)?;
            realScVarOffset = (realScVarIdxOffset).get(2)?;
            realScVarIdx = realScVarIdx + realScVarOffset;
            varString = ComponentReferenceBasics::printComponentRefStr(&name)?;
            taskIdx = metamodelica::arrayGet(iScVarTaskMapping.clone(), realScVarIdx)?;
            let ScVarInfo { ownerThread: __pa4, .. } = metamodelica::arrayGet(iScVarInfos.clone(), realScVarIdx)?;
            threadOwner = metamodelica::Own::own(__pa4);
            nodeId = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("var"));
                __mm_s.push_str(&*intString(realScVarIdx));
                ArcStr::from(__mm_s)
            };
            metamodelica::arrayUpdate(iAddedVariables.clone(), realScVarIdx, true)?;
            threadText = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Th "));
                __mm_s.push_str(&*intString(threadOwner));
                ArcStr::from(__mm_s)
            };
            nodeLabelText = intString(realScVarIdx);
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL {
                text: nodeLabelText,
                backgroundColor: None,
                fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN,
            };
            (oGraphInfo, _) = GraphML::addNode(
                nodeId,
                arcstr::literal!(GraphML::COLOR_GREEN2),
                GraphML::BORDERWIDTH_STANDARD.clone(),
                list![nodeLabel],
                openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE,
                Some(varString),
                list![(iAttThreadIdIdx, threadText)],
                iTopGraphIdx,
                iGraphInfo,
            )?;
        } else {
            oGraphInfo = iGraphInfo;
        }
    } else {
        oGraphInfo = iGraphInfo;
    }
    Ok(oGraphInfo)
}

// -------------------------------------------
// PRINT
// -------------------------------------------
fn printCacheMap(mut iCacheMap: &CacheMap) -> Result<()> {
    let mut cacheLineSize: i32;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut cacheLines: metamodelica::List<CacheLineMap>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let () = (match iCacheMap.clone() {
        CacheMap::CACHEMAP {
            cacheLineSize: mut __esc_cacheLineSize,
            cacheVariables: mut __esc_cacheVariables,
            cacheLinesFloat: mut __esc_cacheLinesFloat,
            cacheLinesInt: mut __esc_cacheLinesInt,
            cacheLinesBool: mut __esc_cacheLinesBool,
        } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheVariables = __esc_cacheVariables.clone();
            cacheLinesFloat = __esc_cacheLinesFloat.clone();
            cacheLinesInt = __esc_cacheLinesInt.clone();
            cacheLinesBool = __esc_cacheLinesBool.clone();
            metamodelica::print(literal!("\n\nCacheMap\n---------------\n"));
            metamodelica::print(literal!("  Variables\n"));
            List::fold(
                metamodelica::AsArg::as_arg(&cacheVariables),
                &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: i32| printCacheVariable(&__a0, __a1),
                ((cacheVariables).len() as i32),
            )?;
            metamodelica::print(literal!("  Float Cache Lines\n"));
            List::map1_0(
                metamodelica::AsArg::as_arg(&cacheLinesFloat),
                &printCacheLineMap,
                cacheVariables.clone(),
            )?;
            metamodelica::print(literal!("  Int Cache Lines\n"));
            List::map1_0(
                metamodelica::AsArg::as_arg(&cacheLinesInt),
                &printCacheLineMap,
                cacheVariables.clone(),
            )?;
            metamodelica::print(literal!("  Bool Cache Lines\n"));
            List::map1_0(
                metamodelica::AsArg::as_arg(&cacheLinesBool),
                &printCacheLineMap,
                cacheVariables.clone(),
            )?;
            ()
        }
        CacheMap::UNIFORM_CACHEMAP {
            cacheLineSize: mut __esc_cacheLineSize,
            cacheVariables: mut __esc_cacheVariables,
            cacheLines: mut __esc_cacheLines,
        } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheVariables = __esc_cacheVariables.clone();
            cacheLines = __esc_cacheLines.clone();
            metamodelica::print(literal!("\n\nUniform CacheMap\n---------------\n"));
            metamodelica::print(literal!("  Variables.\n"));
            List::map1_0(
                metamodelica::AsArg::as_arg(&cacheLines),
                &printCacheLineMap,
                cacheVariables.clone(),
            )?;
            ()
        }
        _ => {
            metamodelica::print(literal!("printCacheMap: Unsupported cache map type!\n"));
            ()
        }
    });
    Ok(())
}

fn printCacheVariable(mut iCacheVariable: &metamodelica::Ref<SimCodeVar::SimVar>, mut iIdx: i32) -> Result<i32> {
    let mut oIdx: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("    "));
        __mm_s.push_str(&*intString(iIdx));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*dumpSimCodeVar(iCacheVariable)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    oIdx = iIdx - 1;
    Ok(oIdx)
}

fn printCacheLineMap(
    mut iCacheLineMap: CacheLineMap,
    mut iCacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
) -> Result<()> {
    let mut idx: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let mut iVarsString: ArcStr;
    let mut iBytesString: ArcStr;
    let CacheLineMap {
        idx: __pa0,
        entries: __pa1,
        ..
    } = iCacheLineMap;
    idx = metamodelica::Own::own(__pa0);
    entries = metamodelica::Own::own(__pa1);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  CacheLineMap "));
        __mm_s.push_str(&*intString(idx));
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((entries).len() as i32)));
        __mm_s.push_str(&*literal!(" entries)\n"));
        ArcStr::from(__mm_s)
    });
    (iVarsString, iBytesString) = List::fold1(
        &entries,
        &move |__a0: CacheLineEntry,
               __a1: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
               __a2: (ArcStr, ArcStr)| cacheLineEntryToString(__a0, &__a1, &__a2),
        iCacheVariables,
        (literal!(""), literal!("")),
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("    "));
        __mm_s.push_str(&*iVarsString);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("    "));
        __mm_s.push_str(&*iBytesString);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn printCacheLineMapClean(mut iCacheLineMap: CacheLineMap) -> Result<()> {
    let mut idx: i32;
    let mut entries: metamodelica::List<CacheLineEntry>;
    let mut iVarsString: ArcStr;
    let mut iBytesString: ArcStr;
    let CacheLineMap {
        idx: __pa0,
        entries: __pa1,
        ..
    } = iCacheLineMap;
    idx = metamodelica::Own::own(__pa0);
    entries = metamodelica::Own::own(__pa1);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("  CacheLineMap "));
        __mm_s.push_str(&*intString(idx));
        __mm_s.push_str(&*literal!(" ("));
        __mm_s.push_str(&*intString(((entries).len() as i32)));
        __mm_s.push_str(&*literal!(" entries)\n"));
        ArcStr::from(__mm_s)
    });
    (iVarsString, iBytesString) = List::fold(
        &entries,
        &move |__a0: CacheLineEntry, __a1: (ArcStr, ArcStr)| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(cacheLineEntryToStringClean(__a0, &__a1))
        },
        (literal!(""), literal!("")),
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("    "));
        __mm_s.push_str(&*iVarsString);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("    "));
        __mm_s.push_str(&*iBytesString);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn cacheLineEntryToString(
    mut iCacheLineEntry: CacheLineEntry,
    mut iCacheVariables: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut iString: &(ArcStr, ArcStr),
) -> Result<(ArcStr, ArcStr)> {
    let mut oString: (ArcStr, ArcStr);
    let mut start: i32;
    let mut dataType: i32;
    let mut size: i32;
    let mut scVarIdx: i32;
    let mut scVarStr: ArcStr;
    let mut iVar: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut iVarsString: ArcStr;
    let mut iBytesString: ArcStr;
    let mut iBytesStringNew: ArcStr;
    (iVarsString, iBytesString) = iString.clone();
    let CacheLineEntry {
        start: __pa0,
        dataType: __pa1,
        size: __pa2,
        scVarIdx: __pa3,
        ..
    } = iCacheLineEntry;
    start = metamodelica::Own::own(__pa0);
    dataType = metamodelica::Own::own(__pa1);
    size = metamodelica::Own::own(__pa2);
    scVarIdx = metamodelica::Own::own(__pa3);
    iVar = (iCacheVariables.clone()).get(((iCacheVariables).len() as i32) - scVarIdx + 1)?;
    scVarStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*dumpSimCodeVar(&iVar)?);
        __mm_s.push_str(&*literal!(" ["));
        __mm_s.push_str(&*intString(scVarIdx));
        __mm_s.push_str(&*literal!("]"));
        ArcStr::from(__mm_s)
    };
    iVarsString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iVarsString);
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*scVarStr);
        ArcStr::from(__mm_s)
    };
    if intGt(start, 0) {
        iVarsString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*iVarsString);
            __mm_s.push_str(&*literal!(" | "));
            ArcStr::from(__mm_s)
        };
        iBytesStringNew = intString(start);
    } else {
        iBytesStringNew = literal!("");
    }
    iBytesStringNew = Util::stringPadLeft(
        iBytesStringNew.clone(),
        2 + ((scVarStr).len() as i32) + ((iBytesStringNew).len() as i32),
        literal!(" "),
    );
    iBytesString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iBytesString);
        __mm_s.push_str(&*iBytesStringNew);
        ArcStr::from(__mm_s)
    };
    oString = (iVarsString, iBytesString);
    Ok(oString)
}

fn cacheLineEntryToStringClean(
    mut iCacheLineEntry: CacheLineEntry,
    mut iString: &(ArcStr, ArcStr),
) -> (ArcStr, ArcStr) {
    let mut oString: (ArcStr, ArcStr);
    let mut start: i32;
    let mut dataType: i32;
    let mut size: i32;
    let mut scVarIdx: i32;
    let mut scVarStr: ArcStr;
    let mut iVarsString: ArcStr;
    let mut iBytesString: ArcStr;
    let mut iBytesStringNew: ArcStr;
    (iVarsString, iBytesString) = iString.clone();
    let CacheLineEntry {
        start: __pa0,
        dataType: __pa1,
        size: __pa2,
        scVarIdx: __pa3,
        ..
    } = iCacheLineEntry;
    start = metamodelica::Own::own(__pa0);
    dataType = metamodelica::Own::own(__pa1);
    size = metamodelica::Own::own(__pa2);
    scVarIdx = metamodelica::Own::own(__pa3);
    scVarStr = intString(scVarIdx);
    iVarsString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iVarsString);
        __mm_s.push_str(&*literal!("| "));
        __mm_s.push_str(&*scVarStr);
        __mm_s.push_str(&*literal!(" "));
        ArcStr::from(__mm_s)
    };
    iBytesStringNew = intString(start);
    iBytesStringNew = Util::stringPadRight(iBytesStringNew, 3 + ((scVarStr).len() as i32), literal!(" "));
    iBytesString = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*iBytesString);
        __mm_s.push_str(&*iBytesStringNew);
        ArcStr::from(__mm_s)
    };
    oString = (iVarsString, iBytesString);
    oString
}

fn dumpSimCodeVar(mut iVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let __arc1 = &(*iVar);
    let SimCodeVar::SIMVAR { name: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    oString = ComponentReferenceBasics::printComponentRefStr(&name)?;
    Ok(oString)
}

fn printNodeSimCodeVarMapping(mut iMapping: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::print(literal!("Node - SimCodeVar - Mapping\n------------------\n"));
    Array::fold(iMapping.clone(), &printNodeSimCodeVarMapping0, 1)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn printNodeSimCodeVarMapping0(mut iMappingEntry: metamodelica::List<i32>, mut iNodeIdx: i32) -> Result<i32> {
    let mut oNodeIdx: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Node "));
        __mm_s.push_str(&*intString(iNodeIdx));
        __mm_s.push_str(&*literal!(" uses sc-vars: "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(iMappingEntry, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    oNodeIdx = iNodeIdx + 1;
    Ok(oNodeIdx)
}

fn printScVarTaskMapping(mut iMapping: metamodelica::Array<i32>) -> Result<()> {
    metamodelica::print(literal!(
        "----------------------\nSCVar - Task - Mapping\n----------------------\n"
    ));
    Array::fold(iMapping.clone(), &fnptr!(printScVarTaskMapping0, i32, i32), 1)?;
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn printScVarTaskMapping0(mut iMappingEntry: i32, mut iScVarIdx: i32) -> i32 {
    let mut oScVarIdx: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("SCVar "));
        __mm_s.push_str(&*intString(iScVarIdx));
        __mm_s.push_str(&*literal!(" is solved in task: "));
        __mm_s.push_str(&*intString(iMappingEntry));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    oScVarIdx = iScVarIdx + 1;
    oScVarIdx
}

fn printCacheLineTaskMapping(mut iCacheLineTaskMapping: metamodelica::Array<metamodelica::List<i32>>) -> Result<()> {
    Array::fold(iCacheLineTaskMapping.clone(), &printCacheLineTaskMapping0, 1)?;
    Ok(())
}

fn printCacheLineTaskMapping0(mut iTasks: metamodelica::List<i32>, mut iCacheLineIdx: i32) -> Result<i32> {
    let mut oCacheLineIdx: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Tasks that are writing to cacheline "));
        __mm_s.push_str(&*intString(iCacheLineIdx));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*stringDelimitList(
            List::map(iTasks, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    oCacheLineIdx = iCacheLineIdx + 1;
    Ok(oCacheLineIdx)
}

fn printEqSimCodeVarMapping(
    mut iMapping: metamodelica::Array<metamodelica::Array<metamodelica::List<i32>>>,
) -> Result<()> {
    let mut sysInformation: metamodelica::Array<metamodelica::List<i32>>;
    let mut sysIdx: i32 = 0;
    let mut vars: metamodelica::List<i32>;
    for mut sysIdx in 1..=metamodelica::arrayLength(iMapping.clone()) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("System "));
            __mm_s.push_str(&*intString(sysIdx));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        sysInformation = metamodelica::arrayGet(iMapping.clone(), sysIdx)?;
        for mut eqIdx in 1..=metamodelica::arrayLength(sysInformation.clone()) {
            vars = metamodelica::arrayGet(sysInformation.clone(), eqIdx)?;
        }
    }
    Ok(())
}

fn printSccNodeMapping(mut iMapping: metamodelica::Array<i32>) -> Result<()> {
    metamodelica::print(literal!(
        "--------------------\nScc - Node - Mapping\n--------------------\n"
    ));
    Array::fold(iMapping.clone(), &fnptr!(printSccNodeMapping0, i32, i32), 1)?;
    Ok(())
}

fn printSccNodeMapping0(mut iMappingEntry: i32, mut iIdx: i32) -> i32 {
    let mut oIdx: i32;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Scc "));
        __mm_s.push_str(&*intString(iIdx));
        __mm_s.push_str(&*literal!(" is solved by node "));
        __mm_s.push_str(&*intString(iMappingEntry));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    oIdx = iIdx + 1;
    oIdx
}

fn printScVarInfos(mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<()> {
    let mut scVarIdx: i32 = 0;
    let mut ownerThread: i32;
    let mut isShared: bool;
    metamodelica::print(literal!("--------------------\nScVar - Infos\n--------------------\n"));
    for mut scVarIdx in 1..=metamodelica::arrayLength(iScVarInfos.clone()) {
        let ScVarInfo {
            ownerThread: __pa0,
            isShared: __pa1,
        } = metamodelica::arrayGet(iScVarInfos.clone(), scVarIdx)?;
        ownerThread = metamodelica::Own::own(__pa0);
        isShared = metamodelica::Own::own(__pa1);
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ScVar "));
            __mm_s.push_str(&*intString(scVarIdx));
            __mm_s.push_str(&*literal!(" has thread owner "));
            __mm_s.push_str(&*intString(ownerThread));
            __mm_s.push_str(&*literal!(" and shared state "));
            __mm_s.push_str(&*boolString(isShared));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn dumpScVarsByIdx(
    mut iSimCodeVarIdx: i32,
    mut iAllSCVarsMapping: metamodelica::Array<Option<metamodelica::Ref<SimCodeVar::SimVar>>>,
) -> ArcStr {
    let mut oString: ArcStr;
    let mut tmpString: ArcStr = arcstr::literal!("");
    let mut simVar: metamodelica::Ref<SimCodeVar::SimVar> =
        <metamodelica::Ref<SimCodeVar::SimVar> as ::std::default::Default>::default();
    oString = 'mc: {
        let __mc_input = iAllSCVarsMapping.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut simVar: metamodelica::Ref<SimCodeVar::SimVar> = simVar.clone();
            let mut tmpString: ArcStr = tmpString.clone();
            let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(iAllSCVarsMapping.clone(), iSimCodeVarIdx)?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            simVar = metamodelica::Own::own(__pa0);
            tmpString = dumpSimCodeVar(&simVar)?;
            Ok((tmpString.clone(), simVar.clone(), tmpString.clone()))
        })() {
            simVar = __wb0;
            tmpString = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "dumpScVarsByIdx: Failed to find simcode-variable with index "
                ));
                __mm_s.push_str(&*intString(iSimCodeVarIdx));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(literal!("NONE"))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oString
}

fn printSimCodeVarTypes(mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut varIdx: i32 = 0;
    let mut varDataType: i32;
    let mut varSize: i32;
    let mut varType: i32;
    for mut varIdx in 1..=metamodelica::arrayLength(iSimCodeVarTypes.clone()) {
        (varDataType, varSize, varType) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), varIdx)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Variable "));
            __mm_s.push_str(&*intString(varIdx));
            __mm_s.push_str(&*literal!(" has data type "));
            __mm_s.push_str(&*intString(varDataType));
            __mm_s.push_str(&*literal!(" and size "));
            __mm_s.push_str(&*intString(varSize));
            __mm_s.push_str(&*literal!(" and type "));
            __mm_s.push_str(&*intString(varType));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

// -------------------------------------------
// SUSAN
// -------------------------------------------
pub(crate) fn getSubscriptListOfArrayCref(
    mut iCref: metamodelica::Ref<DAE::ComponentRef>,
    mut iNumArrayElems: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>> {
    let mut oSubscriptList: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
    let mut tmpCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    tmpCrefs = expandCref(iCref, iNumArrayElems)?;
    oSubscriptList = List::map(tmpCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
        ComponentReference::crefLastSubs(&__a0)
    })?;
    Ok(oSubscriptList)
}

pub(crate) fn expandCref(
    mut iCref: metamodelica::Ref<DAE::ComponentRef>,
    mut iNumArrayElems: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut oCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut elems: i32;
    let mut dims: i32;
    let mut dimElemCount: metamodelica::List<i32>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    cref = removeSubscripts(&iCref);
    dims = getCrefDims(iCref);
    dimElemCount = getDimElemCount(iNumArrayElems.clone().reverse(), dims)?;
    elems = List::reduce(&dimElemCount, &fnptr!(intMul, i32, i32))?;
    dims = ((iNumArrayElems).len() as i32);
    oCrefs = expandCref1(cref, elems, dimElemCount)?;
    Ok(oCrefs)
}

pub(crate) fn expandCrefWithDims(
    mut iCref: metamodelica::Ref<DAE::ComponentRef>,
    mut iDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut oCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dim: metamodelica::Ref<DAE::Dimension> = metamodelica::Ref::new(DAE::Dimension::DIM_BOOLEAN);
    let mut numArrayElems: metamodelica::List<ArcStr>;
    numArrayElems = metamodelica::nil();
    for mut dim in &**iDims {
        let mut dim = dim.clone();
        numArrayElems = metamodelica::cons(getDimStringOfDimElement(&dim), numArrayElems);
    }
    oCrefs = expandCref(iCref, numArrayElems)?;
    Ok(oCrefs)
}

fn getDimStringOfDimElement(mut iDim: &metamodelica::Ref<DAE::Dimension>) -> ArcStr {
    let mut oDimString: ArcStr;
    let mut integer: i32;
    oDimString = (match &**iDim {
        DAE::Dimension::DIM_INTEGER { integer: __esc_integer } => {
            integer = (*__esc_integer).clone();
            intString(integer.clone())
        }
        _ => {
            metamodelica::print(literal!(
                "getDimStringOfDimElement: unsupported Dimension-type given!\n"
            ));
            literal!("")
        }
    });
    oDimString
}

fn removeSubscripts(mut iCref: &metamodelica::Ref<DAE::ComponentRef>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut oCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut ident: ArcStr;
    let mut identType: metamodelica::Ref<DAE::Type>;
    let mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    oCref = (match &**iCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: __esc_ident,
            identType: __esc_identType,
            subscriptLst: __esc_subscriptLst,
            componentRef: __esc_componentRef,
        } => {
            ident = (*__esc_ident).clone();
            identType = (*__esc_identType).clone();
            subscriptLst = (*__esc_subscriptLst).clone();
            componentRef = (*__esc_componentRef).clone();
            componentRef = removeSubscripts(metamodelica::AsArg::as_arg(&componentRef));
            metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: ident.clone(),
                identType: identType.clone(),
                subscriptLst: subscriptLst.clone(),
                componentRef: componentRef.clone(),
            })
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: __esc_ident,
            identType: __esc_identType,
            subscriptLst: __esc_subscriptLst,
        } => {
            ident = (*__esc_ident).clone();
            identType = (*__esc_identType).clone();
            subscriptLst = (*__esc_subscriptLst).clone();
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: ident.clone(),
                identType: identType.clone(),
                subscriptLst: metamodelica::nil(),
            })
        }
        _ => iCref.clone(),
    });
    oCref
}

fn getDimElemCount(mut iNumArrayElems: metamodelica::List<ArcStr>, mut iDims: i32) -> Result<metamodelica::List<i32>> {
    let mut oNumArrayElems: metamodelica::List<i32>;
    let mut dimList: metamodelica::List<i32>;
    let mut intNumArrayElems: metamodelica::List<i32>;
    let mut dims: i32;
    dims = if (intLe(iDims, 0)) {
        ((iNumArrayElems).len() as i32)
    } else {
        iDims
    };
    dimList = List::intRange(dims);
    intNumArrayElems = List::map(iNumArrayElems, &stringInt)?;
    oNumArrayElems = List::map1(
        dimList,
        &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1),
        intNumArrayElems,
    )?;
    Ok(oNumArrayElems)
}

fn getCrefDims(mut iCref: metamodelica::Ref<DAE::ComponentRef>) -> i32 {
    '__tco: loop {
        let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
        let mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
        let mut tmpDims: i32;
        match &*iCref {
            DAE::ComponentRef::CREF_QUAL {
                componentRef: __esc_componentRef,
                ..
            } => {
                componentRef = (*__esc_componentRef).clone();
                {
                    iCref = componentRef.clone();
                    continue '__tco;
                }
            }
            DAE::ComponentRef::CREF_IDENT {
                subscriptLst: __esc_subscriptLst,
                ..
            } => {
                subscriptLst = (*__esc_subscriptLst).clone();
                return ((subscriptLst).len() as i32);
            }
            _ => {
                metamodelica::print(literal!("HpcOmMemory.getCrefDims failed!\n"));
                return 0;
            }
        }
    }
}

fn expandCref1(
    mut iCref: metamodelica::Ref<DAE::ComponentRef>,
    mut iElems: i32,
    mut iDimElemCount: metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut oCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tmpCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut idxList: metamodelica::List<i32> = metamodelica::nil();
    oCrefs = 'mc: {
        let __mc_input = &*iDimElemCount;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = tmpCrefs.clone();
                    tmpCrefs = ComponentReference::expandCref(&iCref, false)?;
                    let true = (intEq(((tmpCrefs).len() as i32), iElems)) else { return Err("pattern mismatch") };
                    Ok((tmpCrefs.clone(), tmpCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tmpCrefs = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut idxList: metamodelica::List<i32> = idxList.clone();
                    let mut tmpCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = tmpCrefs.clone();
                    idxList = List::intRange(List::reduce(&iDimElemCount, &fnptr!(intMul, i32, i32))?);
                    tmpCrefs = List::map2(idxList.clone(), &move |__a0: i32, __a1: metamodelica::List<i32>, __a2: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(createArrayIndexCref(__a0, &__a1, __a2)) }, iDimElemCount.clone(), iCref.clone())?;
                    Ok((tmpCrefs.clone(), idxList.clone(), tmpCrefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            idxList = __wb0;
            tmpCrefs = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oCrefs)
}

fn createArrayIndexCref(
    mut iIdx: i32,
    mut iDimElemCount: &metamodelica::List<i32>,
    mut iCref: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut oCref: metamodelica::Ref<DAE::ComponentRef>;
    (oCref, _) = createArrayIndexCref_impl(iIdx, iDimElemCount, &((iCref, 1)));
    oCref
}

fn createArrayIndexCref_impl(
    mut iIdx: i32,
    mut iDimElemCount: &metamodelica::List<i32>,
    mut iRefCurrentDim: &(metamodelica::Ref<DAE::ComponentRef>, i32),
) -> (metamodelica::Ref<DAE::ComponentRef>, i32) {
    let mut oRefCurrentDim: (metamodelica::Ref<DAE::ComponentRef>, i32);
    let mut ident: ArcStr;
    let mut identType: metamodelica::Ref<DAE::Type>;
    let mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut currentDim: i32;
    let mut idxValue: i32 = 0;
    let mut dimElemsPre: i32 = 0;
    let mut dimElems: i32 = 0;
    oRefCurrentDim = 'mc: {
        let __mc_input = iRefCurrentDim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef }, 1) => {
                    let mut componentRef = (*componentRef).clone();
                    let true = (intLe(1, ((iDimElemCount).len() as i32))) else { return Err("pattern mismatch") };
                    (componentRef, _) = createArrayIndexCref_impl(iIdx, iDimElemCount, &((componentRef.clone(), 1)));
                    Ok((metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: ident.clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() }), 2))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef }, currentDim) => {
                    let mut componentRef = (*componentRef).clone();
                    let true = (intLe(currentDim.clone(), ((iDimElemCount).len() as i32))) else { return Err("pattern mismatch") };
                    (componentRef, _) = createArrayIndexCref_impl(iIdx, iDimElemCount, &((componentRef.clone(), currentDim.clone())));
                    Ok((metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL { ident: ident.clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() }), currentDim.clone() + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, 1) => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    let mut idxValue: i32 = idxValue.clone();
                    let true = (intLe(1, ((iDimElemCount).len() as i32))) else { return Err("pattern mismatch") };
                    idxValue = intMod(iIdx - 1, (iDimElemCount).head().cloned()?) + 1;
                    subscriptLst = metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: idxValue }) }), subscriptLst.clone());
                    Ok((createArrayIndexCref_impl(iIdx, iDimElemCount, &((metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }), 2))), idxValue.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            idxValue = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, currentDim) => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    let mut dimElems: i32 = dimElems.clone();
                    let mut dimElemsPre: i32 = dimElemsPre.clone();
                    let mut idxValue: i32 = idxValue.clone();
                    let true = (intLe(currentDim.clone(), ((iDimElemCount).len() as i32))) else { return Err("pattern mismatch") };
                    dimElemsPre = List::reduce(&(List::sublist(iDimElemCount.clone(), 1, ((iDimElemCount).len() as i32) - currentDim.clone() + 1)?), &fnptr!(intMul, i32, i32))?;
                    dimElems = (iDimElemCount).get(currentDim.clone())?;
                    idxValue = intMod(intDiv(iIdx - 1, dimElemsPre), dimElems) + 1;
                    subscriptLst = metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: idxValue }) }), subscriptLst.clone());
                    Ok((createArrayIndexCref_impl(iIdx, iDimElemCount, &((metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: ident.clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }), currentDim.clone() + 1))), dimElems.clone(), dimElemsPre.clone(), idxValue.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dimElems = __wb0;
            dimElemsPre = __wb1;
            idxValue = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, currentDim) => {
                    let false = (intLe(currentDim.clone(), ((iDimElemCount).len() as i32))) else { return Err("pattern mismatch") };
                    Ok(iRefCurrentDim.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("createArrayIndexCref_impl failed!\n"));
                    Ok(iRefCurrentDim.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oRefCurrentDim
}

// -------------------------------------------
// UTIL
// -------------------------------------------
fn getTaskListTasks(
    mut iTaskList: &HpcOmSimCode::TaskList,
) -> metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>> {
    let mut oTasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    let mut tasks: metamodelica::List<metamodelica::Ref<HpcOmSimCode::Task>>;
    oTasks = (match iTaskList.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        }
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        }
        _ => {
            metamodelica::print(literal!("getTaskListTasks failed!\n"));
            metamodelica::nil()
        }
    });
    oTasks
}

fn getCacheLineMapOfPartlyFilledCacheLine(mut iPartlyFilledCacheLine: &PartlyFilledCacheLine) -> CacheLineMap {
    let mut oCacheLineMap: CacheLineMap;
    let mut cacheLineMap: CacheLineMap;
    oCacheLineMap = (match iPartlyFilledCacheLine.clone() {
        PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL {
            cacheLineMap: mut __esc_cacheLineMap,
            ..
        } => {
            cacheLineMap = __esc_cacheLineMap.clone();
            cacheLineMap
        }
        PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD {
            cacheLineMap: mut __esc_cacheLineMap,
        } => {
            cacheLineMap = __esc_cacheLineMap.clone();
            cacheLineMap
        }
    });
    oCacheLineMap
}

fn getAllCacheLinesOfCacheMap(mut iCacheMap: &CacheMap) -> metamodelica::List<CacheLineMap> {
    let mut oCacheLines: metamodelica::List<CacheLineMap>;
    let mut cacheLinesFloat: metamodelica::List<CacheLineMap>;
    let mut cacheLinesInt: metamodelica::List<CacheLineMap>;
    let mut cacheLinesBool: metamodelica::List<CacheLineMap>;
    let mut allCacheLines: metamodelica::List<CacheLineMap>;
    oCacheLines = (match iCacheMap.clone() {
        CacheMap::CACHEMAP {
            cacheLinesFloat: mut __esc_cacheLinesFloat,
            cacheLinesInt: mut __esc_cacheLinesInt,
            cacheLinesBool: mut __esc_cacheLinesBool,
            ..
        } => {
            cacheLinesFloat = __esc_cacheLinesFloat.clone();
            cacheLinesInt = __esc_cacheLinesInt.clone();
            cacheLinesBool = __esc_cacheLinesBool.clone();
            allCacheLines = listAppend(
                cacheLinesFloat.clone(),
                listAppend(cacheLinesInt.clone(), cacheLinesBool.clone()),
            );
            allCacheLines
        }
        CacheMap::UNIFORM_CACHEMAP {
            cacheLines: ref __esc_allCacheLines,
            ..
        } => {
            allCacheLines = __esc_allCacheLines.clone();
            allCacheLines.clone()
        }
    });
    oCacheLines
}

fn getCacheVariablesOfCacheMap(mut iCacheMap: &CacheMap) -> metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> {
    let mut oCacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut cacheVariables: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    oCacheVariables = (match iCacheMap.clone() {
        CacheMap::CACHEMAP {
            cacheVariables: mut __esc_cacheVariables,
            ..
        } => {
            cacheVariables = __esc_cacheVariables.clone();
            cacheVariables.clone()
        }
        CacheMap::UNIFORM_CACHEMAP {
            cacheVariables: mut __esc_cacheVariables,
            ..
        } => {
            cacheVariables = __esc_cacheVariables.clone();
            cacheVariables.clone()
        }
    });
    oCacheVariables
}

fn getCacheLineSizeOfCacheMap(mut iCacheMap: &CacheMap) -> i32 {
    let mut oCacheLineSize: i32;
    let mut cacheLineSize: i32;
    oCacheLineSize = (match iCacheMap.clone() {
        CacheMap::CACHEMAP {
            cacheLineSize: mut __esc_cacheLineSize,
            ..
        } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheLineSize
        }
        CacheMap::UNIFORM_CACHEMAP {
            cacheLineSize: mut __esc_cacheLineSize,
            ..
        } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheLineSize
        }
    });
    oCacheLineSize
}
