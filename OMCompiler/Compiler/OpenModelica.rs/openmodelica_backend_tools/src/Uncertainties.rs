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

use crate::MathematicaDump;
use openmodelica_ast::Absyn;
use openmodelica_backend::AdjacencyMatrix;
use openmodelica_backend::BackendDAECreate;
use openmodelica_backend::BackendDAEUtil;
use openmodelica_backend::BackendDump;
use openmodelica_backend::BackendEquation;
use openmodelica_backend::BackendVarTransform;
use openmodelica_backend::BackendVariable;
use openmodelica_backend::ExpressionSolve;
use openmodelica_backend::Matching;
use openmodelica_backend::Sorting;
use openmodelica_backend::SymbolTable;
use openmodelica_backend::SymbolicJacobian;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::StateMachineFlatten;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::HashTable2;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type ExtAdjacencyMatrixRow = (i32, metamodelica::List<i32>);

pub type ExtAdjacencyMatrix = metamodelica::List<(i32, metamodelica::List<i32>)>;

pub type mapBlocks = metamodelica::List<(metamodelica::List<i32>, bool, bool)>;

// {blocks,blocks.visited,blocks.square}
pub(crate) const UNDERLINE: &'static str = "==========================================================================";

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct AliasSet {
    pub symbols: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ),
    pub expl: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTable2::FuncHashCref,
            HashTable2::FuncCrefEqual,
            HashTable2::FuncCrefStr,
            HashTable2::FuncExpStr,
        ),
    ),
    pub signs: (
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
    pub source: Option<metamodelica::Ref<DAE::ElementSource>>,
}

impl metamodelica::gc::MMTrace for AliasSet {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.symbols, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.expl, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.signs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        Ok(())
    }
}
impl PartialEq for AliasSet {
    fn eq(&self, other: &Self) -> bool {
        (match ((&self.symbols), (&other.symbols)) {
            ((__lt0, __lt1, __lt2, __lt3, __lt4), (__rt0, __rt1, __rt2, __rt3, __rt4)) => {
                (__lt0 == __rt0)
                    && (__lt1 == __rt1)
                    && (__lt2 == __rt2)
                    && (__lt3 == __rt3)
                    && (match (__lt4, __rt4) {
                        ((__lt0, __lt1, __lt2), (__rt0, __rt1, __rt2)) => {
                            std::sync::Arc::ptr_eq(__lt0, __rt0)
                                && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                && std::sync::Arc::ptr_eq(__lt2, __rt2)
                        }
                    })
            }
        }) && (match ((&self.expl), (&other.expl)) {
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
        }) && (match ((&self.signs), (&other.signs)) {
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
        }) && self.source == other.source
    }
}
impl Eq for AliasSet {}
impl PartialOrd for AliasSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AliasSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (match ((&self.symbols), (&other.symbols)) {
            ((__lt0, __lt1, __lt2, __lt3, __lt4), (__rt0, __rt1, __rt2, __rt3, __rt4)) => __lt0
                .cmp(__rt0)
                .then_with(|| __lt1.cmp(__rt1))
                .then_with(|| __lt2.cmp(__rt2))
                .then_with(|| __lt3.cmp(__rt3))
                .then_with(|| {
                    (match (__lt4, __rt4) {
                        ((__lt0, __lt1, __lt2), (__rt0, __rt1, __rt2)) => (std::sync::Arc::as_ptr(__lt0) as *const ())
                            .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt1) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                            })
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt2) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                            }),
                    })
                }),
        })
        .then_with(|| {
            (match ((&self.expl), (&other.expl)) {
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
            (match ((&self.signs), (&other.signs)) {
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
        .then_with(|| self.source.cmp(&other.source))
    }
}
impl std::fmt::Debug for AliasSet {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("AliasSet");
        __ds.field(
            "symbols",
            &format_args!("<dyn-fn-container@{:p}>", (&self.symbols) as *const _),
        );
        __ds.field(
            "expl",
            &format_args!("<dyn-fn-container@{:p}>", (&self.expl) as *const _),
        );
        __ds.field(
            "signs",
            &format_args!("<dyn-fn-container@{:p}>", (&self.signs) as *const _),
        );
        __ds.field("source", &self.source);
        __ds.finish()
    }
}

impl Default for AliasSet {
    fn default() -> Self {
        Self {
            symbols: (
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashSet::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashSet::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashSet::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            expl: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTable2::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable2::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable2::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable2::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            signs: (
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
            source: Default::default(),
        }
    }
}

pub type ALIASSET = AliasSet;

pub fn modelEquationsUC(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut className: metamodelica::Ref<Absyn::Path>,
    mut outputFileIn: ArcStr,
    mut dumpSteps: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = ({
        let mut forceOrdering: bool = Flags::getConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone())?;
        'mc: {
            let __mc_input = (inCache, inEnv, outputFileIn);
            if let Ok(__v) = (|| -> Result<_> {
                let (mut cache, mut graph, mut outputFile) = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut resstr: ArcStr;
                let mut dae: DAE::DAElist;
                let mut p: Absyn::Program;
                let mut dlow: metamodelica::Ref<BackendDAE::BackendDAE>;
                let mut dlow_1: metamodelica::Ref<BackendDAE::BackendDAE>;
                let mut m: metamodelica::Array<metamodelica::List<i32>>;
                let mut approximatedEquations: metamodelica::List<i32>;
                let mut approximatedEquations_one: metamodelica::List<i32>;
                let mut setC_eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut setS_eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqsyslist: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
                let mut allVars: BackendDAE::Variables;
                let mut knownVariables: BackendDAE::Variables;
                let mut unknownVariables: BackendDAE::Variables;
                let mut globalKnownVars: BackendDAE::Variables;
                let mut allEqs: metamodelica::Ref<
                    ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                >;
                let mut variables: metamodelica::List<i32>;
                let mut knowns: metamodelica::List<i32>;
                let mut unknowns: metamodelica::List<i32>;
                let mut directlyLinked: metamodelica::List<i32>;
                let mut indirectlyLinked: metamodelica::List<i32>;
                let mut outputvars: metamodelica::List<i32>;
                let mut shared: metamodelica::Ref<BackendDAE::Shared>;
                let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
                let mut mExt: ExtAdjacencyMatrix;
                let mut setS: metamodelica::List<i32>;
                let mut setC: metamodelica::List<i32>;
                let mut unknownsVarsMatch: metamodelica::List<i32>;
                let mut remainingEquations: metamodelica::List<i32>;
                let mut removed_equations_squared: metamodelica::List<i32>;
                let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
                let mut mapIncRowEqn: metamodelica::Array<i32>;
                let mut outStringA: ArcStr;
                let mut outStringB: ArcStr;
                let mut outString: ArcStr;
                let mut description: ArcStr;
                let mut distributions: metamodelica::List<Option<metamodelica::Ref<DAE::Distribution>>>;
                Print::clearBuf();
                p = SymbolTable::getAbsyn();
                (dae, cache, graph) = flattenModel(className.clone(), p.clone(), cache.clone())?;
                description = DAEUtil::daeDescription(&dae);
                {
                    let __v = Some(true);
                    openmodelica_util::Globals::uncertaintyExtraction.with(|__root| *__root.borrow_mut() = __v)
                };
                dlow = BackendDAECreate::lower(
                    dae.clone(),
                    cache.clone(),
                    graph.clone(),
                    BackendDAE::ExtraInfo {
                        description: description.clone(),
                        fileNamePrefix: outputFile.clone(),
                        simflags: None,
                    },
                )?;
                FlagsUtil::setConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone(), false)?;
                (dlow_1, _, _, _, _) = BackendDAEUtil::getSolvedSystem(
                    dlow.clone(),
                    &(literal!("")),
                    Some(list![
                        literal!("removeSimpleEquations"),
                        literal!("removeUnusedVariables"),
                        literal!("removeEqualRHS"),
                        literal!("expandDerOperator")
                    ]),
                    None,
                    None,
                    Some(metamodelica::nil()),
                )?;
                FlagsUtil::setConfigBool(Flags::DEFAULT_OPT_MODULES_ORDERING.clone(), forceOrdering)?;
                {
                    let __v = None;
                    openmodelica_util::Globals::uncertaintyExtraction.with(|__root| *__root.borrow_mut() = __v)
                };
                dlow_1 = removeSimpleEquationsUC(dlow_1.clone())?;
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(dlow_1.clone()) {
                    Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 }, shared: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                currentSystem = metamodelica::Own::own(__pa0);
                eqsyslist = metamodelica::Own::own(__pa1);
                shared = metamodelica::Own::own(__pa2);
                let __arc5 = currentSystem.clone();
                let BackendDAE::EQSYSTEM {
                    orderedVars: __pa3,
                    orderedEqs: __pa4,
                    ..
                } = &*__arc5;
                allVars = metamodelica::Own::own(__pa3);
                allEqs = metamodelica::Own::own(__pa4);
                let __arc7 = shared.clone();
                let BackendDAE::SHARED {
                    globalKnownVars: __pa6, ..
                } = &*__arc7;
                globalKnownVars = metamodelica::Own::own(__pa6);
                (m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
                    &currentSystem,
                    openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
                    None,
                    BackendDAEUtil::isInitializationDAE(&shared),
                )?;
                let true = ((eqsyslist).is_empty()) else {
                    return Err("pattern mismatch");
                };
                mExt = getExtAdjacencyMatrix(m.clone());
                variables = List::intRange(BackendVariable::varsSize(&allVars));
                (knowns, _) = getUncertainRefineVariableIndexes(&allVars, &variables)?;
                directlyLinked = getRelatedVariables(&mExt, &knowns)?;
                indirectlyLinked = List::setDifference(getRelatedVariables(&mExt, &directlyLinked)?, &knowns)?;
                unknowns = listAppend(directlyLinked.clone(), indirectlyLinked.clone());
                outputvars = List::setDifference(
                    List::intRange(BackendVariable::varsSize(&allVars)),
                    &(listAppend(unknowns.clone(), knowns.clone())),
                )?;
                dlow_1 = eliminateVariablesDAE(&unknowns, dlow_1.clone())?;
                printSep(getMathematicaText(&(literal!("== Initial system =="))))?;
                let (__pa8, __pa9) = ::match_deref::match_deref! { match &(dlow_1.clone()) {
                    Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: _ }, shared: __pa9 } => (__pa8.clone(), __pa9.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                currentSystem = metamodelica::Own::own(__pa8);
                shared = metamodelica::Own::own(__pa9);
                let __arc12 = currentSystem.clone();
                let BackendDAE::EQSYSTEM {
                    orderedVars: __pa10,
                    orderedEqs: __pa11,
                    ..
                } = &*__arc12;
                allVars = metamodelica::Own::own(__pa10);
                allEqs = metamodelica::Own::own(__pa11);
                let __arc14 = shared.clone();
                let BackendDAE::SHARED {
                    globalKnownVars: __pa13,
                    ..
                } = &*__arc14;
                globalKnownVars = metamodelica::Own::own(__pa13);
                (m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(
                    &currentSystem,
                    openmodelica_backend_types::BackendDAE::IndexType::NORMAL,
                    None,
                    BackendDAEUtil::isInitializationDAE(&shared),
                )?;
                printSep(getMathematicaText(&(literal!("After Symbolic Elimination"))))?;
                printSep(getMathematicaText(
                    &(literal!("Equations (Function calls represent more than one equation)")),
                ))?;
                printSep(equationsToMathematicaGrid(
                    List::intRange(BackendEquation::equationArraySize(allEqs.clone())?),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                printSep(getMathematicaText(&(literal!("Variables"))))?;
                printSep(variablesToMathematicaGrid(
                    List::intRange(BackendVariable::varsSize(&allVars)),
                    allVars.clone(),
                )?)?;
                mExt = getExtAdjacencyMatrix(m.clone());
                approximatedEquations_one = getEquationsWithApproximatedAnnotation(&dlow_1)?;
                approximatedEquations = List::flatten(List::map1r(
                    approximatedEquations_one.clone(),
                    &listGet,
                    mapEqnIncRow
                        .clone()
                        .borrow()
                        .iter()
                        .cloned()
                        .collect::<metamodelica::List<_>>(),
                )?)?;
                mExt = removeEquations(&mExt, &approximatedEquations)?;
                printSep(getMathematicaText(&(literal!("Approximated equations to be removed"))))?;
                printSep(equationsToMathematicaGrid(
                    approximatedEquations.clone(),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                printSep(getMathematicaText(
                    &(literal!("After eliminating approximated equations")),
                ))?;
                printSep(equationsToMathematicaGrid(
                    getEquationsNumber(&mExt),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                variables = List::intRange(BackendVariable::varsSize(&allVars));
                (knowns, distributions) = getUncertainRefineVariableIndexes(&allVars, &variables)?;
                directlyLinked = getRelatedVariables(&mExt, &knowns)?;
                indirectlyLinked = List::setDifference(getRelatedVariables(&mExt, &directlyLinked)?, &knowns)?;
                unknowns = listAppend(directlyLinked.clone(), indirectlyLinked.clone());
                outputvars = List::setDifference(
                    List::intRange(BackendVariable::varsSize(&allVars)),
                    &(listAppend(unknowns.clone(), knowns.clone())),
                )?;
                printSep(getMathematicaText(&(literal!("Known variables"))))?;
                printSep(variablesToMathematicaGrid(knowns.clone(), allVars.clone())?)?;
                printSep(getMathematicaText(&(literal!("Directly linked variables"))))?;
                printSep(variablesToMathematicaGrid(directlyLinked.clone(), allVars.clone())?)?;
                printSep(getMathematicaText(&(literal!("Indirectly linked variables"))))?;
                printSep(variablesToMathematicaGrid(indirectlyLinked.clone(), allVars.clone())?)?;
                printSep(getMathematicaText(&(literal!("Output variables"))))?;
                printSep(variablesToMathematicaGrid(outputvars.clone(), allVars.clone())?)?;
                mExt = eliminateOutputVariables(mExt.clone(), &outputvars)?;
                printSep(getMathematicaText(&(literal!("After eliminating output variables"))))?;
                printSep(equationsToMathematicaGrid(
                    getEquationsNumber(&mExt),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                (setS, unknownsVarsMatch) = getEquationsForUnknownsSystem(&mExt, knowns.clone(), unknowns.clone())?;
                printSep(getMathematicaText(
                    &(literal!("Matching performed after step 5 (Set S)")),
                ))?;
                printSep(unknowsMatchingToMathematicaGrid(
                    unknownsVarsMatch.clone(),
                    setS.clone(),
                    allEqs.clone(),
                    allVars.clone(),
                    &globalKnownVars,
                    mapIncRowEqn.clone(),
                )?)?;
                remainingEquations = List::setDifference(getEquationsNumber(&mExt), &setS)?;
                printSep(getMathematicaText(&(literal!("Remaining equations"))))?;
                printSep(equationsToMathematicaGrid(
                    remainingEquations.clone(),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                (setC, removed_equations_squared) = getEquationsForKnownsSystem(
                    &mExt,
                    knowns.clone(),
                    &unknowns,
                    &setS,
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?;
                if !((removed_equations_squared).is_empty()) {
                    metamodelica::print(literal!(
                        "Warning: the system is ill-posed. One or more equations have been removed from squared system of knowns.\n"
                    ));
                }
                printSep(getMathematicaText(
                    &(literal!("Equations removed from squared blocks (with more than one equation)")),
                ))?;
                printSep(equationsToMathematicaGrid(
                    removed_equations_squared.clone(),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                printSep(getMathematicaText(&(literal!("Final Equations"))))?;
                printSep(equationsToMathematicaGrid(
                    setC.clone(),
                    allEqs.clone(),
                    allVars.clone(),
                    globalKnownVars.clone(),
                    mapIncRowEqn.clone(),
                )?)?;
                setC = List::map1r(
                    setC.clone(),
                    &listGet,
                    mapIncRowEqn
                        .clone()
                        .borrow()
                        .iter()
                        .cloned()
                        .collect::<metamodelica::List<_>>(),
                )?;
                setC = List::unique(&setC);
                setS = List::map1r(
                    setS.clone(),
                    &listGet,
                    mapIncRowEqn
                        .clone()
                        .borrow()
                        .iter()
                        .cloned()
                        .collect::<metamodelica::List<_>>(),
                )?;
                setS = List::unique(&setS);
                setC_eq = List::map1r(setC.clone(), &BackendEquation::get, allEqs.clone())?;
                setS_eq = List::map1r(setS.clone(), &BackendEquation::get, allEqs.clone())?;
                knownVariables = BackendVariable::listVar(List::map1r(
                    knowns.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars.clone(),
                )?)?;
                unknownVariables = BackendVariable::listVar(List::map1r(
                    unknowns.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars.clone(),
                )?)?;
                outStringB = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("{{"));
                    __mm_s.push_str(&*getMathematicaVarStr(knownVariables.clone())?);
                    __mm_s.push_str(&*literal!(","));
                    __mm_s.push_str(&*getMathematicaEqStr(
                        setC_eq.clone(),
                        allVars.clone(),
                        globalKnownVars.clone(),
                    )?);
                    __mm_s.push_str(&*literal!("},{"));
                    __mm_s.push_str(&*getMathematicaVarStr(unknownVariables.clone())?);
                    __mm_s.push_str(&*literal!(","));
                    __mm_s.push_str(&*getMathematicaEqStr(
                        setS_eq.clone(),
                        allVars.clone(),
                        globalKnownVars.clone(),
                    )?);
                    __mm_s.push_str(&*literal!("},"));
                    __mm_s.push_str(&*dumpVarsDistributionInfo(distributions.clone())?);
                    __mm_s.push_str(&*literal!("}"));
                    ArcStr::from(__mm_s)
                };
                Print::printBuf({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("{"));
                    __mm_s.push_str(&*getMathematicaText(&(literal!("Extraction finished"))));
                    __mm_s.push_str(&*literal!("}"));
                    ArcStr::from(__mm_s)
                })?;
                outStringA = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Grid[{"));
                    __mm_s.push_str(&*Print::getString()?);
                    __mm_s.push_str(&*literal!("}]"));
                    ArcStr::from(__mm_s)
                };
                outString = if (dumpSteps) {
                    outStringA.clone()
                } else {
                    outStringB.clone()
                };
                resstr = writeFileIfNonEmpty(outputFile.clone(), outString.clone());
                Ok((
                    cache.clone(),
                    metamodelica::Ref::new(Values::Value::STRING { string: resstr.clone() }),
                ))
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let (_, _, mut outputFile) = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut resstr: ArcStr;
                let mut outStringA: ArcStr;
                {
                    let __v = None;
                    openmodelica_util::Globals::uncertaintyExtraction.with(|__root| *__root.borrow_mut() = __v)
                };
                Print::printBuf({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("{"));
                    __mm_s.push_str(&*getMathematicaText(&(literal!("Extraction failed"))));
                    __mm_s.push_str(&*literal!("}"));
                    ArcStr::from(__mm_s)
                })?;
                outStringA = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Grid[{"));
                    __mm_s.push_str(&*Print::getString()?);
                    __mm_s.push_str(&*literal!("}]"));
                    ArcStr::from(__mm_s)
                };
                writeFileIfNonEmpty(outputFile.clone(), outStringA.clone());
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                    return Err("pattern mismatch");
                };
                resstr = AbsynUtil::pathStringNoQual(className.clone(), literal!("."), false, false)?;
                resstr = stringAppendList(list![
                    literal!("modelEquationsUC: The model equations in model"),
                    resstr.clone(),
                    literal!(" could not be extracted")
                ]);
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![resstr.clone()])?;
                Ok(return Err("fail"))
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    Ok((outCache, outValue))
}

/*
Function which runs the Extraction Algorithm for DataReconcilaiton Procedure
*/
pub(crate) fn dataReconciliation(
    mut inDae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDae: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDae = (::match_deref::match_deref! { match &(inDae.clone()) {
        dae => {
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut approximatedEquations: metamodelica::List<i32>;
            let mut approximatedEquations_one: metamodelica::List<i32>;
            let mut constantvars: metamodelica::List<i32>;
            let mut extractedvars: metamodelica::List<i32>;
            let mut setC_eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut setS_eq: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut eqsyslist: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
            let mut allVars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut tmpglobalKnownVars: BackendDAE::Variables;
            let mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut resVarsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut variables: metamodelica::List<i32>;
            let mut knowns: metamodelica::List<i32>;
            let mut unknowns: metamodelica::List<i32>;
            let mut directlyLinked: metamodelica::List<i32>;
            let mut indirectlyLinked: metamodelica::List<i32>;
            let mut outputvars: metamodelica::List<i32>;
            let mut finalvarlist: metamodelica::List<i32>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let mut currentSystem: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut mExt: ExtAdjacencyMatrix;
            let mut setS: metamodelica::List<i32>;
            let mut setC: metamodelica::List<i32>;
            let mut tempsetS: metamodelica::List<i32>;
            let mut tempsetC: metamodelica::List<i32>;
            let mut inputvarlist: metamodelica::List<i32>;
            let mut mapEqnIncRow: metamodelica::Array<metamodelica::List<i32>>;
            let mut mapIncRowEqn: metamodelica::Array<i32>;
            let mut match1: metamodelica::Array<i32>;
            let mut bltblocks: metamodelica::List<metamodelica::List<i32>>;
            let mut blockstofind: metamodelica::List<metamodelica::List<i32>>;
            let mut blockranks: metamodelica::List<(metamodelica::List<i32>, i32)>;
            let mut blockstatus: metamodelica::List<metamodelica::List<ArcStr>>;
            let mut var: metamodelica::List<(i32, i32)>;
            let mut tempvar: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut tmpparamvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut blocktargetinfo: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<(metamodelica::List<i32>, i32)>, metamodelica::List<(metamodelica::List<ArcStr>, i32)>)>;
            let mut predecessorblocktargetinfo: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<(metamodelica::List<i32>, i32)>, metamodelica::List<(metamodelica::List<ArcStr>, i32)>, metamodelica::List<i32>, metamodelica::List<i32>)>;
            let mut initblocks: mapBlocks;
            let mut modelname: ArcStr;
            let mut einfo: BackendDAE::ExtraInfo;
            let mut simcodejacobian: metamodelica::Ref<BackendDAE::Jacobian>;
            let mut outDiffVars: BackendDAE::Variables;
            let mut outResidualVars: BackendDAE::Variables;
            let mut outOtherVars: BackendDAE::Variables;
            let mut outResidualEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut outOtherEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
            let mut r#str: ArcStr;
            let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(dae.clone()) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 }, shared: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            currentSystem = metamodelica::Own::own(__pa0);
            eqsyslist = metamodelica::Own::own(__pa1);
            shared = metamodelica::Own::own(__pa2);
            let __arc5 = currentSystem.clone();
            let BackendDAE::EQSYSTEM { orderedVars: __pa3, orderedEqs: __pa4, .. } = &*__arc5;
            allVars = metamodelica::Own::own(__pa3);
            allEqs = metamodelica::Own::own(__pa4);
            let __arc8 = shared.clone();
            let BackendDAE::SHARED { globalKnownVars: __pa6, info: __pa7, .. } = &*__arc8;
            globalKnownVars = metamodelica::Own::own(__pa6);
            einfo = metamodelica::Own::own(__pa7);
            let BackendDAE::EXTRA_INFO { fileNamePrefix: __pa9, .. } = einfo;
            modelname = metamodelica::Own::own(__pa9);
            (m, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(&currentSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(&shared))?;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nModelInfo: ")); __mm_s.push_str(&*modelname); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            BackendDump::dumpEquationArray(allEqs.clone(), &(literal!("orderedEquation")))?;
            BackendDump::dumpVariables(&allVars, &(literal!("orderedVariables")))?;
            (match1, _) = Matching::PerfectMatching(m.clone())?;
            var = dumpMatching(match1.clone());
            BackendDump::dumpMatching(match1.clone())?;
            bltblocks = Sorting::Tarjan(m.clone(), match1.clone(), metamodelica::arrayLength(match1.clone()))?;
            dumpListList(bltblocks.clone(), &(literal!("BLT_BLOCKS")))?;
            let true = ((eqsyslist).is_empty()) else { return Err("pattern mismatch") };
            mExt = getExtAdjacencyMatrix(m.clone());
            variables = List::intRange(BackendVariable::varsSize(&allVars));
            (knowns, _) = getUncertainRefineVariableIndexes(&allVars, &variables)?;
            directlyLinked = getRelatedVariables(&mExt, &knowns)?;
            indirectlyLinked = List::setDifference(getRelatedVariables(&mExt, &directlyLinked)?, &knowns)?;
            unknowns = listAppend(directlyLinked, indirectlyLinked);
            outputvars = List::setDifference(List::intRange(BackendVariable::varsSize(&allVars)), &(listAppend(unknowns.clone(), knowns.clone())))?;
            unknowns = listAppend(unknowns, outputvars.clone());
            listAppend(unknowns.clone(), knowns.clone());
            initblocks = setInitialBlocks(&bltblocks);
            constantvars = getConstantVariables(&mExt);
            approximatedEquations_one = getEquationsWithApproximatedAnnotation(metamodelica::AsArg::as_arg(&dae))?;
            approximatedEquations = List::flatten(List::map1r(approximatedEquations_one, &listGet, mapEqnIncRow.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?)?;
            getRemovedEquationSolvedVariables(&approximatedEquations, &var)?;
            (blockstofind, blockstatus) = originalBlocks(&bltblocks, knowns.clone(), unknowns.clone(), outputvars, &var)?;
            blockranks = List::toListWithPositions(&blockstofind);
            blockstatus = checkBlockStatus(&blockstofind, &blockstatus);
            blocktargetinfo = findBlockTargets(blockstofind, &blockstatus, var.clone(), mExt.clone(), initblocks, &blockranks)?;
            predecessorblocktargetinfo = findPredecessorBlocks(&blocktargetinfo)?;
            (tempsetC, tempsetS) = ExtractEquationsfromPredecessorBlocks(&predecessorblocktargetinfo, &blockranks, &approximatedEquations)?;
            getVariableOccurence(&tempsetC, &mExt, knowns.clone());
            extractedvars = getVariablesAfterExtraction(tempsetC.clone(), tempsetS.clone(), &mExt);
            getVariablesAfterExtraction(tempsetS.clone(), metamodelica::nil(), &mExt);
            finalvarlist = getRemovedEquationSolvedVariables(&(listAppend(tempsetC.clone(), tempsetS.clone())), &var)?;
            (finalvarlist, inputvarlist, _) = List::intersection1OnTrue(extractedvars.clone(), finalvarlist, &fnptr!(intEq, i32, i32))?;
            inputvarlist = List::setDifferenceOnTrue(inputvarlist, &knowns, &fnptr!(intEq, i32, i32))?;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFINAL SET OF EQUATIONS After Reconciliation \n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_C: ")); __mm_s.push_str(&*dumplistInteger(tempsetC.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("SET_S: ")); __mm_s.push_str(&*dumplistInteger(tempsetS.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) });
            setC = List::map1r(tempsetC.clone(), &listGet, mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?;
            setC = List::unique(&setC);
            setS = List::map1r(tempsetS.clone(), &listGet, mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?;
            setS = List::unique(&setS);
            setC_eq = List::map1r(setC, &BackendEquation::get, allEqs.clone())?;
            setS_eq = List::map1r(setS, &BackendEquation::get, allEqs.clone())?;
            BackendDump::dumpEquationList(&setC_eq, &(literal!("SET_C")))?;
            BackendDump::dumpEquationList(&setS_eq, &(literal!("SET_S")))?;
            outDiffVars = BackendVariable::listVar(List::map1r(knowns.clone().reverse(), &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), allVars.clone())?)?;
            outDiffVars = BackendVariable::listVar(List::map1(BackendVariable::varList(&outDiffVars)?, &fnptr!(BackendVariable::setVarUnreplaceable, metamodelica::Ref<BackendDAE::Var>, bool), true)?)?;
            (_, reqns) = BackendEquation::traverseEquationArray(BackendEquation::listEquation(&setC_eq)?, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (metamodelica::Ref<AvlTreePathFunction::Tree>, metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>)| BackendEquation::traverseEquationToScalarResidualForm(__a0, &__a1), (shared.functionTree.clone(), metamodelica::nil()))?;
            (reqns, resVarsLst, _) = BackendEquation::convertResidualsIntoSolvedEquations(&(reqns.reverse()), &(literal!("$res")), 1, false)?;
            outResidualVars = BackendVariable::listVar(resVarsLst)?;
            outResidualEqns = BackendEquation::listEquation(&reqns)?;
            outOtherEqns = BackendEquation::listEquation(&setS_eq)?;
            tmpparamvars = BackendEquation::equationsVars(outOtherEqns.clone(), globalKnownVars.clone())?;
            cr_lst = BackendEquation::getAllCrefFromEquations(BackendEquation::listEquation(&setS_eq)?)?;
            outOtherVars = dumpCrefList(&cr_lst, &outDiffVars, tmpparamvars.clone())?;
            BackendDump::dumpVariables(&outOtherVars, &(literal!("Unknown variables in SET_S ")))?;
            BackendDump::dumpVariables(&(BackendVariable::listVar(tmpparamvars)?), &(literal!("Parameters in SET_S")))?;
            VerifyDataReconciliation(tempsetC.clone(), tempsetS, knowns, &unknowns, &mExt, &var, &constantvars, &approximatedEquations, allVars.clone(), allEqs, mapIncRowEqn.clone(), &outOtherVars, &setS_eq)?;
            (simcodejacobian, shared) = SymbolicJacobian::getSymbolicJacobian(&outDiffVars, outResidualEqns.clone(), outResidualVars.clone(), outOtherEqns.clone(), outOtherVars.clone(), shared, &(BackendVariable::listVar(List::map1r(extractedvars, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), allVars.clone())?)?), literal!("F"), false)?;
            BackendVariable::listVar(List::map1r(getRemovedEquationSolvedVariables(&tempsetC, &var)?, &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1), allVars)?)?;
            assign_field!(shared.dataReconciliationData = Some(BackendDAE::DataReconciliationData { symbolicJacobian: simcodejacobian, setcVars: outResidualVars.clone(), datareconinputs: outDiffVars.clone(), setBVars: None, symbolicJacobianH: None, relatedBoundaryConditions: 0 }));
            currentSystem = BackendDAEUtil::setEqSystVars(currentSystem, BackendVariable::mergeVariables(outResidualVars, outOtherVars, true)?);
            currentSystem = BackendDAEUtil::setEqSystEqs(currentSystem, BackendEquation::merge(outResidualEqns, outOtherEqns)?);
            tempvar = BackendVariable::varList(&outDiffVars)?;
            tmpglobalKnownVars = BackendVariable::listVar(List::map1(tempvar.clone(), &fnptr!(BackendVariable::setVarDirection, metamodelica::Ref<BackendDAE::Var>, DAE::VarDirection), openmodelica_frontend_types::DAE::VarDirection::INPUT)?)?;
            shared = BackendDAEUtil::setSharedGlobalKnownVars(shared, BackendVariable::mergeVariables(globalKnownVars, tmpglobalKnownVars, true)?);
            if !(System::regularFileExists({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelname); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) })) {
                r#str = literal!("Variable Names,Measured Value-x,HalfWidthConfidenceInterval,xi,xk,rx_ik\n");
                r#str = dumpToCsv(&r#str, &tempvar)?;
                System::writeFile({ let mut __mm_s = String::new(); __mm_s.push_str(&*modelname); __mm_s.push_str(&*literal!("_Inputs.csv")); ArcStr::from(__mm_s) }, r#str)?;
            }
            outDae = metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: list![currentSystem], shared: shared });
            outDae
        },
        _ => {
            inDae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDae)
}

pub(crate) fn dumpCrefList(
    mut cr_lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut invar: &BackendDAE::Variables,
    mut paramvars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<BackendDAE::Variables> {
    let mut outvar: BackendDAE::Variables;
    let mut tmpcr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tmpparamcrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tmpvar: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut count: i32 = 1;
    tmpcr = List::map(BackendVariable::varList(invar)?, &move |__a0: metamodelica::Ref<
        BackendDAE::Var,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
    })?;
    tmpparamcrefs = List::map(
        paramvars,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    for mut i in &**cr_lst {
        if !(listMember(i.clone(), tmpcr.clone())) && !(listMember(i.clone(), tmpparamcrefs.clone())) {
            tmpvar = metamodelica::cons(BackendVariable::makeVar(i.clone())?, tmpvar);
            count = count + 1;
        }
    }
    outvar = BackendVariable::listVar(List::unique(&tmpvar))?;
    Ok(outvar)
}

/* function which dumps the variable names to csv file */
pub(crate) fn dumpToCsv(
    mut instring: &ArcStr,
    mut invar: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<ArcStr> {
    let mut outstring: ArcStr = literal!("");
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    for mut i in &**invar {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&i));
        outstring = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*outstring);
            __mm_s.push_str(&*ComponentReference::crefStr(&cr)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    outstring = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*instring);
        __mm_s.push_str(&*outstring);
        ArcStr::from(__mm_s)
    };
    Ok(outstring)
}

/* creates list of equations from SET-S needed for jacobian calculation */
pub(crate) fn createInnerEquations(
    mut tempsets: &metamodelica::List<i32>,
    mut solvedeqvar: &metamodelica::List<(i32, i32)>,
    mut sets: &metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut inputlist: &metamodelica::List<i32>,
) -> Result<metamodelica::List<BackendDAE::InnerEquation>> {
    let mut outequations: metamodelica::List<BackendDAE::InnerEquation> = metamodelica::nil();
    let mut varnumber: i32;
    let mut count: i32 = 1;
    let mut inpcount: i32 = 1;
    for mut eqnumber in &**tempsets {
        varnumber = getSolvedVariableNumber(eqnumber.clone(), solvedeqvar)?;
        if !(listMember(varnumber, knowns.clone())) {
            outequations = metamodelica::cons(
                BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: (sets).get(count)?,
                    vars: list![varnumber],
                },
                outequations,
            );
        } else {
            outequations = metamodelica::cons(
                BackendDAE::InnerEquation::INNEREQUATION {
                    eqn: (sets).get(count)?,
                    vars: list![(inputlist).get(inpcount)?],
                },
                outequations,
            );
            inpcount = inpcount + 1;
        }
        count = count + 1;
    }
    outequations = outequations.reverse();
    Ok(outequations)
}

pub(crate) fn dumpDependencyTree(
    mut invartree: &metamodelica::List<(i32, metamodelica::List<i32>)>,
    mut ineqtree: &metamodelica::List<(i32, metamodelica::List<i32>)>,
    mut knowns: metamodelica::List<i32>,
    mut constantvars: metamodelica::List<i32>,
    mut allVars: BackendDAE::Variables,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<()> {
    let mut varnumber: i32;
    let mut count: i32 = 1;
    let mut eqs: metamodelica::List<i32>;
    let mut varlist: metamodelica::List<i32>;
    let mut depeqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut var: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut kn1: metamodelica::List<i32>;
    let mut kn2: metamodelica::List<i32>;
    let mut kn3: metamodelica::List<i32>;
    for mut i in &**invartree {
        (varnumber, varlist) = i.clone();
        (_, eqs) = (ineqtree).get(count)?;
        var = List::map1r(
            list![varnumber],
            &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
            allVars.clone(),
        )?;
        depeqs = List::map1r(
            List::map1r(
                eqs,
                &listGet,
                mapIncRowEqn
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>(),
            )?,
            &BackendEquation::get,
            allEqs.clone(),
        )?;
        (kn1, kn2, kn3) = List::intersection1OnTrue(
            varlist,
            listAppend(knowns.clone(), constantvars.clone()),
            &fnptr!(intEq, i32, i32),
        )?;
        if (kn1).is_empty() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n-The intermediate variable: "));
                __mm_s.push_str(&*intString(varnumber));
                __mm_s.push_str(&*literal!(" does not have any knowns or constants as Leaf"));
                ArcStr::from(__mm_s)
            });
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(": Condition 5-Failed : The system is ill-posed.")],
            )?;
            return Ok(());
        }
        BackendDump::dumpVarList(&var, &(literal!("Intermediate_Variable_in_SET_C")))?;
        BackendDump::dumpEquationList(&depeqs, &(literal!("Dependency_tree")))?;
        count = count + 1;
    }
    Ok(())
}

pub(crate) fn getSolvedDependentEquationAndVars(
    mut inlist: &metamodelica::List<i32>,
    mut solvedvar: &metamodelica::List<(i32, i32)>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut sets_eqs: metamodelica::List<i32> = metamodelica::nil();
    let mut sets_vars: metamodelica::List<i32> = metamodelica::nil();
    let mut eqnumber: i32;
    for mut varnumber in &**inlist {
        eqnumber = getSolvedEquationNumber(varnumber.clone(), solvedvar)?;
        sets_eqs = metamodelica::cons(eqnumber, sets_eqs);
        sets_vars = metamodelica::cons(varnumber.clone(), sets_vars);
    }
    Ok((sets_eqs, sets_vars))
}

pub(crate) fn getVariablesAfterExtraction(
    mut setc: metamodelica::List<i32>,
    mut sets: metamodelica::List<i32>,
    mut mext: &ExtAdjacencyMatrix,
) -> metamodelica::List<i32> {
    let mut finalvars: metamodelica::List<i32> = metamodelica::nil();
    let mut fulleqs: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    let mut eq: i32;
    fulleqs = listAppend(setc, sets);
    for mut i in &*fulleqs {
        for mut j in &**mext {
            (eq, vars) = j.clone();
            if intEq(i.clone(), eq) {
                for mut k in &*vars {
                    finalvars = metamodelica::cons(k.clone(), finalvars);
                }
            }
        }
    }
    finalvars = List::unique(&finalvars);
    finalvars
}

pub(crate) fn getConstantVariables(mut mext: &ExtAdjacencyMatrix) -> metamodelica::List<i32> {
    let mut constantvars: metamodelica::List<i32> = metamodelica::nil();
    let mut vars: metamodelica::List<i32>;
    let mut eqnumber: i32;
    for mut i in &**mext {
        (eqnumber, vars) = i.clone();
        if ((vars).len() as i32) == 1 {
            for mut j in &*vars {
                constantvars = metamodelica::cons(j.clone(), constantvars);
            }
        }
    }
    constantvars
}

pub(crate) fn VerifyDataReconciliation(
    mut setc: metamodelica::List<i32>,
    mut sets: metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: &metamodelica::List<i32>,
    mut mExt: &ExtAdjacencyMatrix,
    mut solvedvar: &metamodelica::List<(i32, i32)>,
    mut constantvars: &metamodelica::List<i32>,
    mut approximatedEquations: &metamodelica::List<i32>,
    mut allVars: BackendDAE::Variables,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut mapIncRowEqn: metamodelica::Array<i32>,
    mut outsetS_vars: &BackendDAE::Variables,
    mut outsetS_eq: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<()> {
    let mut matchedeq: metamodelica::List<i32>;
    let mut matchedknownssetc: metamodelica::List<i32>;
    let mut matchedunknownssetc: metamodelica::List<i32>;
    let mut matchedknownssets: metamodelica::List<i32>;
    let mut matchedunknownssets: metamodelica::List<i32>;
    let mut tmplist1: metamodelica::List<i32>;
    let mut tmplist2: metamodelica::List<i32>;
    let mut tmplist3: metamodelica::List<i32>;
    let mut tmplist1sets: metamodelica::List<i32>;
    let mut tmplistvar1: metamodelica::List<i32>;
    let mut tmplistvar2: metamodelica::List<i32>;
    let mut tmplistvar3: metamodelica::List<i32>;
    let mut r#str: ArcStr;
    let mut resstr: ArcStr;
    let mut var: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\n\nAutomatic Verification Steps of DataReconciliation Algorithm"
        ));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    var = List::map1r(
        knowns.clone().reverse(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        allVars.clone(),
    )?;
    BackendDump::dumpVarList(
        &var,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("knownVariables:"));
            __mm_s.push_str(&*dumplistInteger(knowns.clone().reverse())?);
            ArcStr::from(__mm_s)
        }),
    )?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("-SET_C:"));
        __mm_s.push_str(&*dumplistInteger(setc.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("-SET_S:"));
        __mm_s.push_str(&*dumplistInteger(sets.clone())?);
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    matchedeq = List::intersectionOnTrue(&setc, &sets, &fnptr!(intEq, i32, i32))?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Condition-1 "));
        __mm_s.push_str(&*literal!("\"SET_C and SET_S must not have no equations in common\""));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if (matchedeq).is_empty() {
        metamodelica::print(literal!("-Passed\n\n"));
    } else {
        metamodelica::print(literal!("-Failed\n"));
        BackendDump::dumpEquationList(
            &(List::map1r(matchedeq.clone(), &BackendEquation::get, allEqs)?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-Equations Found in SET_C and SET_S:"));
                __mm_s.push_str(&*dumplistInteger(matchedeq)?);
                ArcStr::from(__mm_s)
            }),
        )?;
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(": Condition 1- Failed : The system is ill-posed.")],
        )?;
        return Err("fail");
    }
    (matchedknownssetc, matchedunknownssetc) = getVariableOccurence(&setc, mExt, knowns.clone());
    (matchedknownssets, matchedunknownssets) = getVariableOccurence(&sets, mExt, knowns.clone());
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Condition-2 "));
        __mm_s.push_str(&*literal!(
            "\"All variables of interest must be involved in SET_C or SET_S\""
        ));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    (tmplist1, tmplist2, tmplist3) =
        List::intersection1OnTrue(matchedknownssetc, knowns.clone(), &fnptr!(intEq, i32, i32))?;
    if (tmplist3).is_empty() {
        metamodelica::print(literal!("-Passed \n"));
        BackendDump::dumpVarList(
            &(List::map1r(
                tmplist1.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_C has all known variables:"));
                __mm_s.push_str(&*dumplistInteger(tmplist1)?);
                ArcStr::from(__mm_s)
            }),
        )?;
    } else if !((tmplist3).is_empty()) {
        (tmplist1sets, tmplist2, _) = List::intersection1OnTrue(tmplist3, matchedknownssets, &fnptr!(intEq, i32, i32))?;
        if !((tmplist2).is_empty()) {
            r#str = dumplistInteger(tmplist2.clone())?;
            metamodelica::print(literal!("-Failed\n"));
            BackendDump::dumpVarList(
                &(List::map1r(
                    tmplist2.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars.clone(),
                )?),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("knownVariables not Found:"));
                    __mm_s.push_str(&*dumplistInteger(tmplist2)?);
                    ArcStr::from(__mm_s)
                }),
            )?;
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(": Condition 2- Failed : The system is ill-posed.")],
            )?;
            return Err("fail");
        }
        metamodelica::print(literal!("-Passed \n"));
        BackendDump::dumpVarList(
            &(List::map1r(
                tmplist1.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_C has known variables:"));
                __mm_s.push_str(&*dumplistInteger(tmplist1)?);
                ArcStr::from(__mm_s)
            }),
        )?;
        BackendDump::dumpVarList(
            &(List::map1r(
                tmplist1sets.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_S has known variables:"));
                __mm_s.push_str(&*dumplistInteger(tmplist1sets)?);
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Condition-3 "));
        __mm_s.push_str(&*literal!(
            "\"SET_C equations must be strictly less than Variable of Interest\""
        ));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if ((setc).len() as i32) < ((knowns).len() as i32) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Passed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("-SET_C contains:"));
            __mm_s.push_str(&*intString(((setc).len() as i32)));
            __mm_s.push_str(&*literal!(" equations < "));
            __mm_s.push_str(&*intString(((knowns).len() as i32)));
            __mm_s.push_str(&*literal!(" known variables \n\n"));
            ArcStr::from(__mm_s)
        });
    } else {
        resstr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Failed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("-SET_C contains:"));
            __mm_s.push_str(&*intString(((setc).len() as i32)));
            __mm_s.push_str(&*literal!(" equations  > "));
            __mm_s.push_str(&*intString(((knowns).len() as i32)));
            __mm_s.push_str(&*literal!(" known variables \n\n"));
            ArcStr::from(__mm_s)
        };
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(": Condition 3-Failed : The system is ill-posed.")],
        )?;
        return Err("fail");
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Condition-4 "));
        __mm_s.push_str(&*literal!(
            "\"SET_S should contain all intermediate variables involved in SET_C\""
        ));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    (tmplistvar1, tmplistvar2, tmplistvar3) = List::intersection1OnTrue(
        matchedunknownssetc.clone(),
        matchedunknownssets,
        &fnptr!(intEq, i32, i32),
    )?;
    if (matchedunknownssetc).is_empty() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Passed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("-SET_C contains No Intermediate Variables \n\n"));
            ArcStr::from(__mm_s)
        });
        return Ok(());
    } else {
        BackendDump::dumpVarList(
            &(List::map1r(
                matchedunknownssetc.clone(),
                &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                allVars.clone(),
            )?),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-SET_C has intermediate variables:"));
                __mm_s.push_str(&*dumplistInteger(matchedunknownssetc)?);
                ArcStr::from(__mm_s)
            }),
        )?;
        if (tmplistvar2).is_empty() {
            BackendDump::dumpVarList(
                &(List::map1r(
                    tmplistvar1.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars,
                )?),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("-SET_S has intermediate variables involved in SET_C:"));
                    __mm_s.push_str(&*dumplistInteger(tmplistvar1)?);
                    ArcStr::from(__mm_s)
                }),
            )?;
            metamodelica::print(literal!("-Passed\n\n"));
        } else {
            BackendDump::dumpVarList(
                &(List::map1r(
                    tmplistvar2.clone(),
                    &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
                    allVars,
                )?),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "-SET_S does not have intermediate variables involved in SET_C:"
                    ));
                    __mm_s.push_str(&*dumplistInteger(tmplistvar2)?);
                    ArcStr::from(__mm_s)
                }),
            )?;
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(": Condition 4-Failed : The system is ill-posed.")],
            )?;
            return Err("fail");
        }
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Condition-5 "));
        __mm_s.push_str(&*literal!("\"SET_S should be square \""));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if (outsetS_eq).is_empty() {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("-Passed"));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!(
                "-SET_S contains 0 intermediate variables and 0 equations \n\n"
            ));
            ArcStr::from(__mm_s)
        });
        return Ok(());
    } else {
        if ((outsetS_eq).len() as i32) == ((BackendVariable::varList(outsetS_vars)?).len() as i32) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-Passed"));
                __mm_s.push_str(&*literal!("\n "));
                __mm_s.push_str(&*literal!("Set_S has "));
                __mm_s.push_str(&*intString(((outsetS_eq).len() as i32)));
                __mm_s.push_str(&*literal!(" equations and "));
                __mm_s.push_str(&*intString(((BackendVariable::varList(outsetS_vars)?).len() as i32)));
                __mm_s.push_str(&*literal!(" variables\n\n"));
                ArcStr::from(__mm_s)
            });
        } else {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-Failed"));
                __mm_s.push_str(&*literal!("\n "));
                __mm_s.push_str(&*literal!("Set_S has "));
                __mm_s.push_str(&*intString(((outsetS_eq).len() as i32)));
                __mm_s.push_str(&*literal!(" equations and "));
                __mm_s.push_str(&*intString(((BackendVariable::varList(outsetS_vars)?).len() as i32)));
                __mm_s.push_str(&*literal!(" variables\n\n"));
                ArcStr::from(__mm_s)
            });
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    ": Condition 5-Failed Set_S is not square: The system is ill-posed."
                )],
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

pub(crate) fn BuildSquareSubSetHelper(
    mut invars: metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut mExt: ExtAdjacencyMatrix,
    mut solvedeqvar: metamodelica::List<(i32, i32)>,
    mut solvedvars: metamodelica::List<i32>,
    mut solvedeqs: metamodelica::List<i32>,
    mut constantvars: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut outlist1: metamodelica::List<i32>;
    let mut outlist2: metamodelica::List<i32>;
    (outlist1, outlist2) = ({
        let mut found: bool = false;
        (::match_deref::match_deref! { match &((invars, knowns.clone(), mExt.clone(), solvedeqvar.clone(), solvedvars.clone(), solvedeqs, constantvars)) {
            (tmpvars, tmpknowns, tmpExt, tmpsolveeqvar, tempsolvedvars, tempsolvedeqs, tmpconstantvars) => {
                let mut t1: metamodelica::List<i32>;
                let mut t2: metamodelica::List<i32>;
                let mut tempeqs: metamodelica::List<i32>;
                let mut tempvars1: metamodelica::List<i32>;
                let mut tempvars2: metamodelica::List<i32>;
                let mut allvars: metamodelica::List<i32>;
                let mut tmp2: metamodelica::List<i32>;
                let mut c1: metamodelica::List<i32>;
                let mut tempsolvedvars = (*tempsolvedvars).clone();
                let mut tempsolvedeqs = (*tempsolvedeqs).clone();
                (t1, t2, _) = List::intersection1OnTrue(tmpvars.clone(), tmpknowns.clone(), &fnptr!(intEq, i32, i32))?;
                (c1, _, _) = List::intersection1OnTrue(tmpvars.clone(), tmpconstantvars.clone(), &fnptr!(intEq, i32, i32))?;
                if !((c1).is_empty()) {
                    (tempsolvedeqs, _) = BuildSquareSubSetHelper1(&c1, metamodelica::AsArg::as_arg(&tmpsolveeqvar), tempsolvedeqs.clone())?;
                    tempsolvedvars = listAppend(tempsolvedvars.clone(), c1);
                }
                if !((t1).is_empty()) {
                    (tempsolvedeqs, _) = BuildSquareSubSetHelper1(&t1, metamodelica::AsArg::as_arg(&tmpsolveeqvar), tempsolvedeqs.clone())?;
                    tempsolvedvars = listAppend(tempsolvedvars.clone(), t1);
                }
                if found == false {
                    tempsolvedvars = listAppend(tempsolvedvars.clone(), t2.clone());
                    (tempsolvedeqs, tempeqs) = BuildSquareSubSetHelper1(&t2, &solvedeqvar, tempsolvedeqs.clone())?;
                    (tempvars1, tempvars2) = getVariableOccurence(&tempeqs, &mExt, knowns);
                    allvars = List::unique(&(listAppend(tempvars1, tempvars2)));
                    (_, tmp2, _) = List::intersection1OnTrue(allvars, solvedvars, &fnptr!(intEq, i32, i32))?;
                    if !((tmp2).is_empty()) {
                        (tempsolvedvars, tempsolvedeqs) = BuildSquareSubSetHelper(tmp2, tmpknowns.clone(), tmpExt.clone(), tmpsolveeqvar.clone(), tempsolvedvars.clone(), tempsolvedeqs.clone(), tmpconstantvars.clone())?;
                    }
                }
                (tempsolvedvars.clone(), tempsolvedeqs.clone())
            },
            (_, _, _, _, _, _, _) => {
                (metamodelica::nil(), metamodelica::nil())
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok((outlist1, outlist2))
}

pub(crate) fn BuildSquareSubSetHelper1(
    mut inlist1: &metamodelica::List<i32>,
    mut solvedeqvar: &metamodelica::List<(i32, i32)>,
    mut solvedeqs: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut tempsolvedeqs: metamodelica::List<i32> = metamodelica::nil();
    let mut tempeqs: metamodelica::List<i32> = metamodelica::nil();
    let mut eqnumber: i32;
    for mut varnumber in &**inlist1 {
        eqnumber = getSolvedEquationNumber(varnumber.clone(), solvedeqvar)?;
        if !(listMember(eqnumber, solvedeqs.clone())) {
            tempeqs = metamodelica::cons(eqnumber, tempeqs);
            tempsolvedeqs = metamodelica::cons(eqnumber, tempsolvedeqs);
        }
    }
    tempsolvedeqs = listAppend(solvedeqs, tempsolvedeqs);
    Ok((tempsolvedeqs, tempeqs))
}

pub(crate) fn BuildSquareSubSet(
    mut ineqs: &metamodelica::List<i32>,
    mut invars: &metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut mExt: ExtAdjacencyMatrix,
    mut solvedeqvar: metamodelica::List<(i32, i32)>,
    mut constantvars: metamodelica::List<i32>,
    mut approximatedEquations: &metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<(i32, metamodelica::List<i32>)>,
    metamodelica::List<(i32, metamodelica::List<i32>)>,
)> {
    let mut solvedvars: metamodelica::List<i32> = metamodelica::nil();
    let mut solvedeqs: metamodelica::List<i32> = metamodelica::nil();
    let mut dependency_variables_tree: metamodelica::List<(i32, metamodelica::List<i32>)> = metamodelica::nil();
    let mut dependency_equation_tree: metamodelica::List<(i32, metamodelica::List<i32>)> = metamodelica::nil();
    let mut tempvars1: metamodelica::List<i32>;
    let mut tempvars2: metamodelica::List<i32>;
    let mut allvars: metamodelica::List<i32>;
    let mut t1: metamodelica::List<i32>;
    let mut t2: metamodelica::List<i32>;
    let mut t3: metamodelica::List<i32>;
    let mut tmpvars: metamodelica::List<i32>;
    let mut tmpeqs: metamodelica::List<i32>;
    let mut varnumber: i32;
    let mut count: i32 = 1;
    for mut i in &**ineqs {
        (tempvars1, tempvars2) = getVariableOccurence(&(list![i.clone()]), &mExt, knowns.clone());
        varnumber = (invars).get(count)?;
        allvars = List::unique(&(listAppend(tempvars1, tempvars2)));
        (t1, t2, t3) = List::intersection1OnTrue(allvars.clone(), list![varnumber], &fnptr!(intEq, i32, i32))?;
        (tmpvars, tmpeqs) = BuildSquareSubSetHelper(
            allvars,
            knowns.clone(),
            mExt.clone(),
            solvedeqvar.clone(),
            list![varnumber],
            list![i.clone()],
            constantvars.clone(),
        )?;
        solvedvars = listAppend(solvedvars, tmpvars.clone());
        solvedeqs = listAppend(solvedeqs, tmpeqs.clone());
        dependency_variables_tree = metamodelica::cons((varnumber, List::unique(&tmpvars)), dependency_variables_tree);
        tmpeqs = List::setDifferenceOnTrue(tmpeqs, approximatedEquations, &fnptr!(intEq, i32, i32))?;
        dependency_equation_tree = metamodelica::cons((i.clone(), List::unique(&tmpeqs)), dependency_equation_tree);
        count = count + 1;
    }
    solvedvars = List::unique(&solvedvars);
    solvedeqs = List::unique(&solvedeqs);
    Ok((
        solvedvars,
        solvedeqs,
        dependency_variables_tree,
        dependency_equation_tree,
    ))
}

pub(crate) fn dumpListList(
    mut lstLst: metamodelica::List<metamodelica::List<i32>>,
    mut heading: &ArcStr,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*heading);
        __mm_s.push_str(&*literal!(":\n"));
        __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(List::map(lstLst, &dumplistInteger)?, literal!(",")));
        __mm_s.push_str(&*literal!("}"));
        __mm_s.push_str(&*literal!("\n\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub(crate) fn dumplistInteger(mut inlist: metamodelica::List<i32>) -> Result<ArcStr> {
    let mut outstring: ArcStr;
    let mut s: metamodelica::List<ArcStr>;
    s = List::map(inlist, &fnptr!(intString, i32))?;
    outstring = stringDelimitList(s, literal!(", "));
    outstring = stringAppendList(list![literal!("{"), outstring, literal!("}")]);
    Ok(outstring)
}

pub(crate) fn getVariableOccurence(
    mut setc: &metamodelica::List<i32>,
    mut mext: &ExtAdjacencyMatrix,
    mut knowns: metamodelica::List<i32>,
) -> (metamodelica::List<i32>, metamodelica::List<i32>) {
    let mut knownvariables: metamodelica::List<i32> = metamodelica::nil();
    let mut unknownvariables: metamodelica::List<i32> = metamodelica::nil();
    let mut vars: metamodelica::List<i32>;
    let mut eq: i32;
    for mut i in &**setc {
        for mut j in &**mext {
            (eq, vars) = j.clone();
            if intEq(i.clone(), eq) {
                for mut var in &*vars {
                    if listMember(var.clone(), knowns.clone()) {
                        knownvariables = metamodelica::cons(var.clone(), knownvariables);
                    } else {
                        unknownvariables = metamodelica::cons(var.clone(), unknownvariables);
                    }
                }
            }
        }
    }
    knownvariables = List::unique(&knownvariables);
    unknownvariables = List::unique(&unknownvariables);
    (knownvariables, unknownvariables)
}

pub(crate) fn setInitialBlocks(mut inlist1: &metamodelica::List<metamodelica::List<i32>>) -> mapBlocks {
    let mut outlist: mapBlocks = metamodelica::nil();
    for mut i in &**inlist1 {
        outlist = metamodelica::cons((i.clone(), false, true), outlist);
    }
    outlist = outlist.reverse();
    outlist
}

pub(crate) fn updateBlocks(
    mut blocktoupdate: &metamodelica::List<i32>,
    mut inlist: &mapBlocks,
    mut visited: bool,
    mut square: bool,
) -> Result<mapBlocks> {
    let mut outlist: mapBlocks = metamodelica::nil();
    let mut i1: metamodelica::List<i32>;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    for mut i in &**inlist {
        (i1, b1, b2) = i.clone();
        b3 = List::setEqualOnTrue(&i1, blocktoupdate, &fnptr!(intEq, i32, i32))?;
        if b3 == true {
            b1 = visited;
            b2 = square;
        }
        outlist = metamodelica::cons((i1, b1, b2), outlist);
    }
    outlist = outlist.reverse();
    Ok(outlist)
}

pub(crate) fn sortBlocks(
    mut sortedranklist: &metamodelica::List<i32>,
    mut inlist2: &metamodelica::List<(metamodelica::List<i32>, i32)>,
) -> metamodelica::List<(metamodelica::List<i32>, i32)> {
    let mut outlist: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut e1: i32;
    let mut blocks: metamodelica::List<i32>;
    for mut i in &**sortedranklist {
        for mut j in &**inlist2 {
            (blocks, e1) = j.clone();
            if i.clone() == e1 {
                outlist = metamodelica::cons((blocks, e1), outlist);
            }
        }
    }
    outlist = outlist.reverse();
    outlist
}

pub(crate) fn findBlocksRanks(
    mut inlist1: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inlist2: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<(metamodelica::List<i32>, i32)>,
    metamodelica::List<i32>,
)> {
    let mut outlist: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut ranklist: metamodelica::List<i32> = metamodelica::nil();
    let mut blocks: metamodelica::List<i32>;
    let mut rank: i32;
    for mut i in &**inlist2 {
        for mut j in &**inlist1 {
            (blocks, rank) = j.clone();
            if i.clone() == blocks {
                outlist = metamodelica::cons((i.clone(), rank), outlist);
                ranklist = metamodelica::cons(rank, ranklist);
            }
        }
    }
    outlist = outlist.reverse();
    ranklist = List::sort(
        ranklist,
        (std::sync::Arc::new(fnptr!(intGt, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
    )?;
    Ok((outlist, ranklist))
}

pub(crate) fn findBlockTargets(
    mut inlist1: metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut solvedvariables: metamodelica::List<(i32, i32)>,
    mut mxt: ExtAdjacencyMatrix,
    mut map: mapBlocks,
    mut blockranks: &metamodelica::List<(metamodelica::List<i32>, i32)>,
) -> Result<
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>,
> {
    let mut outlist: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )> = metamodelica::nil();
    let mut targetblocks: metamodelica::List<metamodelica::List<i32>>;
    let mut targetvarlist: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut blockvarlst: metamodelica::List<ArcStr>;
    let mut ranklist: metamodelica::List<i32>;
    let mut blocks1: metamodelica::List<i32>;
    let mut rank: i32;
    let mut updatedblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    for mut i in &*inlist1 {
        targetblocks = findBlockTargetsHelper(
            &(list![i.clone()]),
            inlist2,
            solvedvariables.clone(),
            mxt.clone(),
            map.clone(),
            inlist1.clone(),
        )?;
        targetblocks = listAppend(list![i.clone()], targetblocks);
        (updatedblocks, ranklist) = findBlocksRanks(blockranks, &targetblocks)?;
        updatedblocks = sortBlocks(&ranklist, &updatedblocks);
        targetvarlist = metamodelica::nil();
        for mut blocks in &*updatedblocks {
            (blocks1, rank) = blocks.clone();
            blockvarlst = getBlockVarList(&blocks1, &inlist1, inlist2)?;
            targetvarlist = metamodelica::cons((blockvarlst, rank), targetvarlist);
        }
        outlist = metamodelica::cons((i.clone(), updatedblocks, targetvarlist.reverse()), outlist);
    }
    outlist = outlist.reverse();
    Ok(outlist)
}

pub(crate) fn findBlockTargetsHelper(
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut solvedvariables: metamodelica::List<(i32, i32)>,
    mut mxt: ExtAdjacencyMatrix,
    mut map: mapBlocks,
    mut actualblocks: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outlist: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    outlist = (::match_deref::match_deref! { match (inlist1, inlist2) {
        (Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }, Deref @ metamodelica::ListNode::Cons { head: firstitem, tail: restitem }) => {
            let mut solvar = solvedvariables;
            let mut mxt1 = mxt;
            let mut map1 = map;
            let mut originalblocks = actualblocks;
            let mut dependencyequation: metamodelica::List<i32>;
            let mut targetblocks: metamodelica::List<metamodelica::List<i32>>;
            let mut targetblocks1: metamodelica::List<metamodelica::List<i32>>;
            dependencyequation = findBlockTargetsHelper1(&(metamodelica::cons(first.clone(), rest.clone())), &solvar, &mxt1)?;
            targetblocks = getActualBlocks(&dependencyequation, &originalblocks, metamodelica::AsArg::as_arg(&first))?;
            targetblocks1 = findBlockTargetsHelper(&targetblocks, &(metamodelica::cons(firstitem.clone(), restitem.clone())), solvar, mxt1, map1, originalblocks)?;
            List::unique(&(listAppend(targetblocks, targetblocks1)))
        },
        (_, _) => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outlist)
}

pub(crate) fn findBlockTargetsHelper1(
    mut inlist: &metamodelica::List<metamodelica::List<i32>>,
    mut solvedvariables: &metamodelica::List<(i32, i32)>,
    mut mxt: &ExtAdjacencyMatrix,
) -> Result<metamodelica::List<i32>> {
    let mut outlist: metamodelica::List<i32> = metamodelica::nil();
    let mut dependencyequations: metamodelica::List<i32>;
    for mut i in &**inlist {
        dependencyequations = getDependencyequation(i.clone(), metamodelica::nil(), solvedvariables, mxt)?;
        for mut v in &*dependencyequations.reverse() {
            outlist = metamodelica::cons(v.clone(), outlist);
        }
    }
    Ok(outlist)
}

pub(crate) fn findPredecessorBlocks(
    mut blockinfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
> {
    let mut outblockinfo: metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )> = metamodelica::nil();
    let mut dependencyequation: metamodelica::List<i32>;
    let mut targetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut tmptargetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut targetblocksvar: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut blockitems1: metamodelica::List<i32>;
    let mut foundblockranks: metamodelica::List<i32>;
    let mut count: i32 = 1;
    let mut tmpcount: i32;
    let mut exist: bool;
    let mut targetexist: bool;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Targets of blocks without predecessors\n"));
        __mm_s.push_str(&*literal!("=====================================\n"));
        ArcStr::from(__mm_s)
    });
    for mut blocks in &**blockinfo {
        (blockitems1, targetblocks, targetblocksvar) = blocks.clone();
        tmpcount = 1;
        targetexist = false;
        for mut tmpblocks in &**blockinfo {
            (_, tmptargetblocks, _) = tmpblocks.clone();
            if !(intEq(count, tmpcount)) {
                if listMember((targetblocks).head().cloned()?, tmptargetblocks) {
                    targetexist = true;
                }
            }
            tmpcount = tmpcount + 1;
        }
        if targetexist == false {
            (exist, dependencyequation, foundblockranks) =
                findSquareAndNonSquareBlocksHelper1(&targetblocks, &targetblocksvar)?;
            if exist {
                (targetblocks, targetblocksvar) = EliminatePredecessorBlockTarget(targetblocks, targetblocksvar)?;
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("target("));
                    __mm_s.push_str(&*dumplistInteger(blockitems1.clone())?);
                    __mm_s.push_str(&*literal!(") : "));
                    __mm_s.push_str(&*anyString(targetblocks.clone()));
                    __mm_s.push_str(&*literal!(" => Blue_Block_Ranks in target "));
                    __mm_s.push_str(&*dumplistInteger(foundblockranks.clone())?);
                    __mm_s.push_str(&*literal!(" => Blue Block Equations : "));
                    __mm_s.push_str(&*dumplistInteger(dependencyequation.clone())?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                outblockinfo = metamodelica::cons(
                    (
                        blockitems1,
                        targetblocks,
                        targetblocksvar,
                        dependencyequation,
                        foundblockranks,
                    ),
                    outblockinfo,
                );
            }
        }
        count = count + 1;
    }
    outblockinfo = outblockinfo.reverse();
    Ok(outblockinfo)
}

// This function eliminates the Blocks which are not needed for extraction algorithm
pub(crate) fn EliminatePredecessorBlockTarget(
    mut inlist1: metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inlist2: metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
) -> Result<(
    metamodelica::List<(metamodelica::List<i32>, i32)>,
    metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
)> {
    let mut targetblocks: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut targetblocksvar: metamodelica::List<(metamodelica::List<ArcStr>, i32)> = metamodelica::nil();
    let mut checkknowns: bool;
    let mut blocksvarlist: metamodelica::List<ArcStr>;
    let mut count: i32 = 1;
    let mut rank: i32;
    for mut i in &*inlist2.clone().reverse() {
        (blocksvarlist, rank) = i.clone();
        checkknowns = listMember(literal!("knowns"), blocksvarlist.clone());
        if listMember(literal!("knowns"), blocksvarlist) {
            targetblocks = List::firstN(inlist1.clone(), ((inlist1).len() as i32) - count + 1)?;
            targetblocksvar = List::firstN(inlist2.clone(), ((inlist2).len() as i32) - count + 1)?;
            break;
        }
        count = count + 1;
    }
    Ok((targetblocks, targetblocksvar))
}

/*
public function findPredecessorBlocksHelper
  input list<tuple<list<Integer>,list<tuple<list<Integer>,Integer>>,list<tuple<list<String>,Integer>>>> blockinfo;
protected
  list<Integer> dependencyequation;
  list<tuple<list<Integer>,Integer>> blockstoupdate,targetblocks;
  list<tuple<list<String>,Integer>> targetblocksvar;
  list<Integer> blockitem,blockitems1,blockitems2;
  list<String> blockvarlst,blockvarlst1,blockvarlst2;
  Integer foundblock,count=1,foundblockrank,tmpcount=1;
  //mapBlocks map1=map;
  Boolean visited,square,status,checkknowns,finalsquarestauts,exist,exist1;
  list<tuple<list<Integer>,list<String>,Boolean,Integer>> outlist1={};
algorithm
  print("\n PredeccesorBlocks\n");
  for blocks in blockinfo loop
    (blockitems1,targetblocks,targetblocksvar):= blocks;
    print(intString(count) + ": " + anyString(targetblocks) + "\n ");
    for tmpblocks in blockinfo loop
      (blockitems1,targetblocks,targetblocksvar):= blocks;
    end for;
    count:=count+1;
  end for;
end findPredecessorBlocksHelper;
*/
pub(crate) fn findSquareAndNonSquareBlocks(
    mut blockinfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
    )>,
    mut solvedvariables: &metamodelica::List<(i32, i32)>,
    mut mxt: &ExtAdjacencyMatrix,
    mut map: mapBlocks,
) -> Result<(
    metamodelica::List<bool>,
    metamodelica::List<(metamodelica::List<i32>, metamodelica::List<ArcStr>, bool, i32, bool)>,
)> {
    let mut outlist: metamodelica::List<bool> = metamodelica::nil();
    let mut outlist2: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<ArcStr>, bool, i32, bool)> =
        metamodelica::nil();
    let mut blockstoupdate: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut targetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut targetblocksvar: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut blockitem: metamodelica::List<i32>;
    let mut blockitems1: metamodelica::List<i32>;
    let mut blockvarlst1: metamodelica::List<ArcStr>;
    let mut blockvarlst2: metamodelica::List<ArcStr>;
    let mut foundblock: i32;
    let mut count: i32 = 1;
    let mut foundblockrank: i32;
    let mut map1: mapBlocks = map;
    let mut visited: bool;
    let mut finalsquarestauts: bool;
    let mut exist: bool;
    let mut exist1: bool;
    let mut outlist1: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<ArcStr>, bool, i32)> =
        metamodelica::nil();
    for mut blocks in &**blockinfo {
        (blockitems1, targetblocks, targetblocksvar) = blocks.clone();
        (blockstoupdate, exist, foundblock) = findSquareAndNonSquareBlocksHelper(targetblocks, &targetblocksvar)?;
        (blockvarlst1, _) = (targetblocksvar).head().cloned()?;
        outlist1 = metamodelica::cons((blockitems1, blockvarlst1, exist, foundblock), outlist1);
        for mut j in &*blockstoupdate {
            (blockitem, _) = j.clone();
            visited = false;
            map1 = updateBlocks(&blockitem, &map1, visited, false)?;
        }
    }
    for mut k in &*map1 {
        (_, _, finalsquarestauts) = k.clone();
        (blockitems1, blockvarlst2, exist1, foundblockrank) = (outlist1.clone().reverse()).get(count)?;
        outlist = metamodelica::cons(finalsquarestauts, outlist);
        outlist2 = metamodelica::cons(
            (blockitems1, blockvarlst2, exist1, foundblockrank, finalsquarestauts),
            outlist2,
        );
        count = count + 1;
    }
    outlist = outlist.reverse();
    outlist2 = outlist2.reverse();
    Ok((outlist, outlist2))
}

pub(crate) fn findSquareAndNonSquareBlocksHelper(
    mut inlist1: metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inlist2: &metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
) -> Result<(metamodelica::List<(metamodelica::List<i32>, i32)>, bool, i32)> {
    let mut targetblocks: metamodelica::List<(metamodelica::List<i32>, i32)> = metamodelica::nil();
    let mut exists: bool = false;
    let mut foundblock: i32 = -1;
    let mut checkknowns: bool;
    let mut blocksvarlist: metamodelica::List<ArcStr>;
    let mut count: i32 = 1;
    let mut rank: i32;
    for mut i in &**inlist2 {
        (blocksvarlist, rank) = i.clone();
        checkknowns = listMember(literal!("knowns"), blocksvarlist);
        if checkknowns == true {
            targetblocks = List::lastN(inlist1.clone(), ((inlist1).len() as i32) - count)?;
            foundblock = rank;
            exists = true;
            break;
        }
        count = count + 1;
    }
    Ok((targetblocks, exists, foundblock))
}

pub(crate) fn findSquareAndNonSquareBlocksHelper1(
    mut inlist1: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut inlist2: &metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
) -> Result<(bool, metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut exists: bool = false;
    let mut foundknownblocks: metamodelica::List<i32> = metamodelica::nil();
    let mut blockranks: metamodelica::List<i32> = metamodelica::nil();
    let mut checkknowns: bool;
    let mut blocksvarlist: metamodelica::List<ArcStr>;
    let mut count: i32 = 1;
    let mut rank: i32;
    let mut tmpcount: i32;
    let mut targetblocks: metamodelica::List<i32>;
    for mut i in &**inlist2 {
        (blocksvarlist, rank) = i.clone();
        (targetblocks, _) = (inlist1).get(count)?;
        checkknowns = listMember(literal!("knowns"), blocksvarlist.clone());
        if checkknowns == true {
            exists = true;
            blockranks = metamodelica::cons(rank, blockranks);
            tmpcount = 1;
            for mut j in &*blocksvarlist {
                if j.clone() == literal!("knowns") {
                    foundknownblocks = metamodelica::cons((targetblocks).get(tmpcount)?, foundknownblocks);
                }
                tmpcount = tmpcount + 1;
            }
        }
        count = count + 1;
    }
    foundknownblocks = foundknownblocks.reverse();
    blockranks = blockranks.reverse();
    Ok((exists, foundknownblocks, blockranks))
}

pub(crate) fn getBlockVarList(
    mut blocktofind: &metamodelica::List<i32>,
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outstringlist: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut b3: bool;
    for mut i in &**inlist1 {
        b3 = List::setEqualOnTrue(metamodelica::AsArg::as_arg(&i), blocktofind, &fnptr!(intEq, i32, i32))?;
        if b3 == true {
            outstringlist = (inlist2).get(count)?;
        }
        count = count + 1;
    }
    Ok(outstringlist)
}

pub(crate) fn getActualBlocks(
    mut searchblock: &metamodelica::List<i32>,
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outlist: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &**inlist1 {
        if !((List::intersectionOnTrue(searchblock, metamodelica::AsArg::as_arg(&i), &fnptr!(intEq, i32, i32))?)
            .is_empty())
        {
            outlist = metamodelica::cons(i.clone(), outlist);
        }
    }
    outlist = outlist.reverse();
    Ok(outlist)
}

pub(crate) fn ExtractEquationsfromPredecessorBlocks(
    mut predecessortargetinfo: &metamodelica::List<(
        metamodelica::List<i32>,
        metamodelica::List<(metamodelica::List<i32>, i32)>,
        metamodelica::List<(metamodelica::List<ArcStr>, i32)>,
        metamodelica::List<i32>,
        metamodelica::List<i32>,
    )>,
    mut allblockranks: &metamodelica::List<(metamodelica::List<i32>, i32)>,
    mut approximatedEquations: &metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut setc: metamodelica::List<i32> = metamodelica::nil();
    let mut sets: metamodelica::List<i32> = metamodelica::nil();
    let mut dependendblock: metamodelica::List<i32>;
    let mut foundblockranks: metamodelica::List<i32>;
    let mut knownblocks: metamodelica::List<i32>;
    let mut usedblocks: metamodelica::List<i32>;
    let mut targetblocktobeinserted: metamodelica::List<i32>;
    let mut blockspostoberemoved: metamodelica::List<i32> = metamodelica::nil();
    let mut targetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut tmptargetblocks: metamodelica::List<(metamodelica::List<i32>, i32)>;
    let mut targetblocksvar: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut tmptargetblocksvar: metamodelica::List<(metamodelica::List<ArcStr>, i32)>;
    let mut blockitems: metamodelica::List<i32>;
    let mut blockitems1: metamodelica::List<i32>;
    let mut tmpsetc: metamodelica::List<i32>;
    let mut tmpsets: metamodelica::List<i32>;
    let mut blockvarlst1: metamodelica::List<ArcStr>;
    let mut count: i32;
    let mut tmpcount: i32;
    let mut blocksize: i32;
    tmpcount = 1;
    usedblocks = metamodelica::nil();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("\nLoop-1\n"));
        __mm_s.push_str(&*literal!("========\n"));
        ArcStr::from(__mm_s)
    });
    for mut blocks in &**predecessortargetinfo {
        (blockitems, targetblocks, targetblocksvar, knownblocks, foundblockranks) = blocks.clone();
        (dependendblock, _) = (allblockranks).get((foundblockranks).head().cloned()?)?;
        targetblocktobeinserted = List::setDifferenceOnTrue(knownblocks, &usedblocks, &fnptr!(intEq, i32, i32))?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nExtractEquationsfromNoPredecessorBlocks :"));
            __mm_s.push_str(&*dumplistInteger(blockitems.clone())?);
            __mm_s.push_str(&*literal!(" => "));
            __mm_s.push_str(&*dumplistInteger(dependendblock.clone())?);
            __mm_s.push_str(&*literal!(" => known blocks:"));
            __mm_s.push_str(&*dumplistInteger(targetblocktobeinserted.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        if !((targetblocktobeinserted).is_empty()) {
            usedblocks = metamodelica::cons((targetblocktobeinserted).head().cloned()?, usedblocks);
        } else {
            blockspostoberemoved = metamodelica::cons(tmpcount, blockspostoberemoved);
        }
        if !(List::setEqualOnTrue(&blockitems, &dependendblock, &fnptr!(intEq, i32, i32))?)
            && !((targetblocktobeinserted).is_empty())
        {
            blocksize = ((blockitems).len() as i32) - 1;
            sets = listAppend(List::firstN(blockitems, blocksize)?, sets);
            sets = metamodelica::cons((targetblocktobeinserted).head().cloned()?, sets);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nAfterinsertion :"));
                __mm_s.push_str(&*dumplistInteger(targetblocktobeinserted)?);
                __mm_s.push_str(&*literal!("=> SET_S :"));
                __mm_s.push_str(&*dumplistInteger(sets.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        } else if !(List::setEqualOnTrue(&blockitems, &dependendblock, &fnptr!(intEq, i32, i32))?)
            && (targetblocktobeinserted).is_empty()
        {
            metamodelica::print(literal!(
                "\nProblem is ill posed because there are two few variables of interest. Boundary condition A is ignored \n"
            ));
            sets = listAppend(blockitems, sets);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nAfterinsertion :"));
                __mm_s.push_str(&*dumplistInteger(targetblocktobeinserted)?);
                __mm_s.push_str(&*literal!("=> SET_S :"));
                __mm_s.push_str(&*dumplistInteger(sets.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        } else if List::setEqualOnTrue(&blockitems, &dependendblock, &fnptr!(intEq, i32, i32))?
            && ((blockitems).len() as i32) == 1
        {
            sets = metamodelica::nil();
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    ": Problem is ill posed because a variable of interest is set on boundary condition B"
                )],
            )?;
        }
        tmpcount = tmpcount + 1;
    }
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\nFinal Extraction After Loop-1:\n===================================\n"
        ));
        __mm_s.push_str(&*literal!("SET_C : "));
        __mm_s.push_str(&*dumplistInteger(setc.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_S : "));
        __mm_s.push_str(&*dumplistInteger(sets.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(literal!("\nLoop-2\n===========\n"));
    for mut i in &**predecessortargetinfo {
        (_, targetblocks, targetblocksvar, _, _) = i.clone();
        tmptargetblocks = List::restOrEmpty(targetblocks)?;
        tmptargetblocksvar = List::restOrEmpty(targetblocksvar)?;
        count = 1;
        for mut j in &*tmptargetblocks {
            (blockitems1, _) = j.clone();
            (blockvarlst1, _) = (tmptargetblocksvar).get(count)?;
            (tmpsetc, tmpsets) = extractMixedBlock(&blockitems1, &blockvarlst1)?;
            setc = listAppend(setc, tmpsetc);
            sets = listAppend(sets, tmpsets);
            count = count + 1;
        }
    }
    sets = List::unique(&sets);
    setc = List::unique(&setc);
    setc = List::setDifferenceOnTrue(setc, &sets, &fnptr!(intEq, i32, i32))?;
    setc = List::setDifferenceOnTrue(setc, approximatedEquations, &fnptr!(intEq, i32, i32))?;
    sets = List::setDifferenceOnTrue(sets, approximatedEquations, &fnptr!(intEq, i32, i32))?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!(
            "\nFinal Extraction After Loop 2:\n=============================\n"
        ));
        __mm_s.push_str(&*literal!("SET_C : "));
        __mm_s.push_str(&*dumplistInteger(setc.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("SET_S: "));
        __mm_s.push_str(&*dumplistInteger(sets.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if (setc).is_empty() {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!(
                " Set-C is Empty! : Problem is ill posed because there are two few variables of interest"
            )],
        )?;
        return Err("fail");
    }
    Ok((setc, sets))
}

pub(crate) fn ExtractEquationsfromBlocks(
    mut blockdata: &metamodelica::List<(metamodelica::List<i32>, metamodelica::List<ArcStr>, bool, i32, bool)>,
    mut approximatedEquation: metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<i32>,
)> {
    let mut setc: metamodelica::List<i32> = metamodelica::nil();
    let mut sets: metamodelica::List<i32> = metamodelica::nil();
    let mut removedeq: metamodelica::List<i32> = metamodelica::nil();
    let mut blockitem: metamodelica::List<i32>;
    let mut blockitem1: metamodelica::List<i32>;
    let mut setc1: metamodelica::List<i32>;
    let mut sets1: metamodelica::List<i32>;
    let mut temp1: metamodelica::List<i32>;
    let mut tmplist1: metamodelica::List<i32>;
    let mut tmplist2: metamodelica::List<i32>;
    let mut tmplist3: metamodelica::List<i32>;
    let mut usedblocklist: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut blockvarlist: metamodelica::List<ArcStr>;
    let mut blockexist: bool;
    let mut squarestatus: bool;
    let mut checkusedblock: bool;
    let mut targetBlockSquareStatus: bool;
    let mut blockrank: i32;
    for mut i in &**blockdata {
        (blockitem, blockvarlist, blockexist, blockrank, squarestatus) = i.clone();
        if blockexist == true && squarestatus == true {
            (blockitem1, _, _, _, targetBlockSquareStatus) = (blockdata).get(blockrank)?;
            checkusedblock = listMember(blockitem1.clone(), usedblocklist.clone());
            if !(List::setEqualOnTrue(&blockitem, &blockitem1, &fnptr!(intEq, i32, i32))?) {
                if targetBlockSquareStatus == true && checkusedblock == false {
                    temp1 = List::lastN(blockitem.clone(), ((blockitem).len() as i32) - 1)?;
                    if (temp1).is_empty() {
                        removedeq = listAppend(blockitem, removedeq);
                    }
                    sets = listAppend(temp1, sets);
                    sets = listAppend(List::firstOrEmpty(&blockitem1), sets);
                    usedblocklist = metamodelica::cons(blockitem1, usedblocklist);
                } else if targetBlockSquareStatus == false || checkusedblock == true {
                    sets = listAppend(blockitem, sets);
                }
            } else {
                (setc1, sets1) = extractMixedBlock(&blockitem, &blockvarlist)?;
                (tmplist1, tmplist2, tmplist3) =
                    List::intersection1OnTrue(setc1, approximatedEquation.clone(), &fnptr!(intEq, i32, i32))?;
                setc1 = listAppend(tmplist1, tmplist2);
                setc = listAppend(List::restOrEmpty(setc1.clone())?, setc);
                sets = listAppend(sets, sets1);
                removedeq = listAppend(List::firstOrEmpty(&setc1), removedeq);
            }
        } else if blockexist == true && squarestatus == false {
            (setc1, sets1) = extractMixedBlock(&blockitem, &blockvarlist)?;
            sets = listAppend(sets, sets1);
            setc = listAppend(setc, setc1);
        } else {
            removedeq = listAppend(blockitem, removedeq);
        }
    }
    setc = List::unique(&setc);
    sets = List::unique(&sets);
    removedeq = List::unique(&removedeq);
    Ok((setc, sets, removedeq))
}

pub(crate) fn getRemovedEquationSolvedVariables(
    mut inlist: &metamodelica::List<i32>,
    mut solvedvar: &metamodelica::List<(i32, i32)>,
) -> Result<metamodelica::List<i32>> {
    let mut outvarlist: metamodelica::List<i32> = metamodelica::nil();
    let mut varnumber: i32;
    for mut i in &**inlist {
        varnumber = getSolvedVariableNumber(i.clone(), solvedvar)?;
        outvarlist = metamodelica::cons(varnumber, outvarlist);
    }
    Ok(outvarlist)
}

pub(crate) fn countKnownVariables(mut inlist1: &metamodelica::List<ArcStr>) -> i32 {
    let mut count: i32 = 0;
    for mut i in &**inlist1 {
        if i.clone() == literal!("knowns") {
            count = count + 1;
        }
    }
    count
}

pub(crate) fn checkBlockStatus(
    mut inlist1: &metamodelica::List<metamodelica::List<i32>>,
    mut inlist2: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> metamodelica::List<metamodelica::List<ArcStr>> {
    let mut instringlist: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut count: i32 = 0;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut setinputs: bool = true;
    for mut i in &**inlist2 {
        let mut i = i.clone();
        b1 = listMember(literal!("knowns"), i.clone());
        b2 = listMember(literal!("unknowns"), i.clone());
        b3 = listMember(literal!("inputs"), i.clone());
        if setinputs == true && b2 == true && b1 == false {
            i = List::fill(literal!("inputs"), ((i).len() as i32));
        }
        if b1 == true && b2 == false {
            setinputs = false;
        }
        if b1 == true && b2 == true {
            setinputs = false;
        }
        instringlist = metamodelica::cons(i, instringlist);
        count = count + 1;
    }
    instringlist = instringlist.reverse();
    instringlist
}

pub(crate) fn originalBlocks(
    mut inlist: &metamodelica::List<metamodelica::List<i32>>,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: metamodelica::List<i32>,
    mut outputs: metamodelica::List<i32>,
    mut solvedvariables: &metamodelica::List<(i32, i32)>,
) -> Result<(
    metamodelica::List<metamodelica::List<i32>>,
    metamodelica::List<metamodelica::List<ArcStr>>,
)> {
    let mut outlist: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut outstringlist: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut blocks: metamodelica::List<i32>;
    let mut blockinfo: metamodelica::List<ArcStr>;
    for mut i in &**inlist {
        (blocks, blockinfo) = checkBlueOrRedSquareBlocks(
            metamodelica::AsArg::as_arg(&i),
            knowns.clone(),
            unknowns.clone(),
            outputs.clone(),
            solvedvariables,
        )?;
        outlist = metamodelica::cons(blocks, outlist);
        outstringlist = metamodelica::cons(blockinfo, outstringlist);
    }
    outlist = outlist.reverse();
    outstringlist = outstringlist.reverse();
    Ok((outlist, outstringlist))
}

pub(crate) fn extractMixedBlock(
    mut inlist: &metamodelica::List<i32>,
    mut instringList: &metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut setc: metamodelica::List<i32> = metamodelica::nil();
    let mut sets: metamodelica::List<i32> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut s: ArcStr;
    for mut e in &**inlist {
        s = (instringList).get(count)?;
        if s == literal!("knowns") {
            setc = metamodelica::cons(e.clone(), setc);
        } else {
            sets = metamodelica::cons(e.clone(), sets);
        }
        count = count + 1;
    }
    Ok((setc, sets))
}

pub(crate) fn getDependencyequation(
    mut inlist: metamodelica::List<i32>,
    mut inlist1: metamodelica::List<i32>,
    mut solvedvariables: &metamodelica::List<(i32, i32)>,
    mut m: &ExtAdjacencyMatrix,
) -> Result<metamodelica::List<i32>> {
    let mut outinteger: metamodelica::List<i32>;
    let mut t: metamodelica::List<i32> = metamodelica::nil();
    let mut nonsq: metamodelica::List<i32>;
    let mut varnumber: i32;
    for mut eqnumber in &*inlist {
        varnumber = getSolvedVariableNumber(eqnumber.clone(), solvedvariables)?;
        nonsq = getdirectOccurrencesinEquation(m, eqnumber.clone(), varnumber)?;
        for mut lst in &*nonsq {
            if !(listMember(lst.clone(), inlist.clone())) {
                t = metamodelica::cons(lst.clone(), t);
            }
        }
    }
    outinteger = listAppend(t, inlist1);
    Ok(outinteger)
}

pub(crate) fn getdirectOccurrencesinEquation(
    mut m: &ExtAdjacencyMatrix,
    mut eqnumber: i32,
    mut varnumber: i32,
) -> Result<metamodelica::List<i32>> {
    let mut out: metamodelica::List<i32>;
    out = (::match_deref::match_deref! { match m {
        Deref @ metamodelica::ListNode::Cons { head: (eq, vars), tail: tail } => {
            let mut eqnum = eqnumber;
            let mut varnum = varnumber;
            let mut ret: metamodelica::List<i32>;
            let mut matchedeq: metamodelica::List<i32>;
            if !(intEq(eq.clone(), eqnum)) {
                if listMember(varnum, vars.clone()) {
                    matchedeq = list![eq.clone()];
                } else {
                    matchedeq = metamodelica::nil();
                }
            } else {
                matchedeq = metamodelica::nil();
            }
            ret = getdirectOccurrencesinEquation(tail, eqnum, varnum)?;
            listAppend(matchedeq, ret)
        },
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub(crate) fn checkBlueOrRedSquareBlocks(
    mut inlist: &metamodelica::List<i32>,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: metamodelica::List<i32>,
    mut outputs: metamodelica::List<i32>,
    mut solvedvar: &metamodelica::List<(i32, i32)>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<ArcStr>)> {
    let mut outlist: metamodelica::List<i32> = metamodelica::nil();
    let mut outstring: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut count: i32 = 1;
    let mut varnumber: i32;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut s1: ArcStr;
    for mut i in &**inlist {
        varnumber = getSolvedVariableNumber(i.clone(), solvedvar)?;
        b1 = listMember(varnumber, knowns.clone());
        b2 = listMember(varnumber, unknowns.clone());
        b3 = listMember(varnumber, outputs.clone());
        if b1 == false && b2 == true {
            s1 = literal!("unknowns");
            outstring = metamodelica::cons(s1, outstring);
            outlist = metamodelica::cons(i.clone(), outlist);
        }
        if b1 == true && b2 == false {
            s1 = literal!("knowns");
            outstring = metamodelica::cons(s1, outstring);
            outlist = metamodelica::cons(i.clone(), outlist);
        }
        if b1 == false && b2 == false {
            s1 = literal!("unknowns");
            outstring = metamodelica::cons(s1, outstring);
            outlist = metamodelica::cons(i.clone(), outlist);
        }
        count = count + 1;
    }
    outlist = outlist.reverse();
    outstring = outstring.reverse();
    Ok((outlist, outstring))
}

/* function which gives solvedvars based on the equation */
pub(crate) fn getSolvedVariableNumber(mut eqnumber: i32, mut inlist: &metamodelica::List<(i32, i32)>) -> Result<i32> {
    let mut solvedvar: i32;
    let mut solvedeq: i32;
    for mut var in &**inlist {
        (solvedeq, solvedvar) = var.clone();
        if intEq(eqnumber, solvedeq) {
            return Ok(solvedvar);
        }
    }
    return Err("fail");
    Ok(solvedvar)
}

/* function which gives solvedeqs based on the variables */
pub(crate) fn getSolvedEquationNumber(mut varnumber: i32, mut inlist: &metamodelica::List<(i32, i32)>) -> Result<i32> {
    let mut solvedeq: i32;
    let mut solvedvar: i32;
    for mut var in &**inlist {
        (solvedeq, solvedvar) = var.clone();
        if intEq(varnumber, solvedvar) {
            return Ok(solvedeq);
        }
    }
    return Err("fail");
    Ok(solvedeq)
}

pub(crate) fn dumpMatching(mut v: metamodelica::Array<i32>) -> metamodelica::List<(i32, i32)> {
    let mut eqvarlist: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut count: i32 = 1;
    let __range0 = v.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqvarlist = metamodelica::cons((i, count), eqvarlist);
        count = count + 1;
    }
    eqvarlist
}

fn printSep(mut s: ArcStr) -> Result<()> {
    Print::printBuf(literal!("{ "))?;
    Print::printBuf(s)?;
    Print::printBuf(literal!("} , "))?;
    Ok(())
}

fn wrapInList(mut text: &ArcStr) -> ArcStr {
    let mut oText: ArcStr;
    oText = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*text);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    oText
}

fn verticalGrid(mut elems: metamodelica::List<ArcStr>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Grid[{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(elems, &move |__a0: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(wrapInList(&__a0))
            })?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}]"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn verticalGridBoxed(mut elems: metamodelica::List<ArcStr>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Grid[{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(elems, &move |__a0: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(wrapInList(&__a0))
            })?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("},Frame -> All]"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn numerateList(mut elems: &metamodelica::List<ArcStr>, mut index: i32) -> ArcStr {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match elems {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut s: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(index)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
            s
        },
        Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
            let mut s: ArcStr;
            let mut ss: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(index)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
            ss = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*numerateList(t, index + 1)); ArcStr::from(__mm_s) };
            ss
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn numerateListIndex(mut elems: &metamodelica::List<ArcStr>, mut indices: &metamodelica::List<i32>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match (elems, indices) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            literal!("")
        },
        (Deref @ metamodelica::ListNode::Cons { head: h, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: n, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut s: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
            s
        },
        (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, Deref @ metamodelica::ListNode::Cons { head: n, tail: tn }) => {
            let mut s: ArcStr;
            let mut ss: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(n.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*h); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
            ss = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*numerateListIndex(t, tn)?); ArcStr::from(__mm_s) };
            ss
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out)
}

fn equationsToMathematicaGrid(
    mut equIndices: metamodelica::List<i32>,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut variables: BackendDAE::Variables,
    mut knownVariables: BackendDAE::Variables,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut eqList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqsString: metamodelica::List<ArcStr>;
    let mut eqns: metamodelica::List<i32>;
    eqns = List::unique(
        &(List::map1r(
            equIndices,
            &listGet,
            mapIncRowEqn
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>(),
        )?),
    );
    eqList = List::map1r(eqns.clone(), &BackendEquation::get, allEqs)?;
    eqsString = List::map1(
        eqList,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: (BackendDAE::Variables, BackendDAE::Variables)| {
            MathematicaDump::printMmaEqnStr(&__a0, &__a1)
        },
        (variables, knownVariables),
    )?;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Grid[{"));
        __mm_s.push_str(&*numerateListIndex(&eqsString, &eqns)?);
        __mm_s.push_str(&*literal!("}, Frame -> All]"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn unknowsMatchingToMathematicaGrid2(
    mut vars: &metamodelica::List<ArcStr>,
    mut eqns: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut out: metamodelica::List<ArcStr>;
    out = (::match_deref::match_deref! { match (vars, eqns) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::print(literal!("Warning: The system is ill-posed. When computing the unknowns, there are more equations than variables.\n"));
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::print(literal!("Warning: The system is ill-posed. When computing the unknowns, there are more variables than equations.\n"));
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: var, tail: var_t }, Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqn_t }) => {
            let mut s: ArcStr;
            let mut r: metamodelica::List<ArcStr>;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*var); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*eqn); ArcStr::from(__mm_s) };
            r = unknowsMatchingToMathematicaGrid2(var_t, eqn_t)?;
            metamodelica::cons(s, r)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out)
}

fn getEquationStringOrNothing(
    mut equations: &metamodelica::List<i32>,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut variables: &BackendDAE::Variables,
    mut knownVariables: &BackendDAE::Variables,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut out: metamodelica::List<ArcStr>;
    out = 'mc: {
        let __mc_input = &**equations;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqn_t } => {
                    let mut s: ArcStr;
                    let mut r: metamodelica::List<ArcStr>;
                    let true = (intEq(eqn.clone(), 0)) else { return Err("pattern mismatch") };
                    r = getEquationStringOrNothing(metamodelica::AsArg::as_arg(&eqn_t), allEqs.clone(), variables, knownVariables, mapIncRowEqn.clone())?;
                    s = literal!("\"-\"");
                    Ok(metamodelica::cons(s.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqn_t } => {
                    let mut s: ArcStr;
                    let mut r: metamodelica::List<ArcStr>;
                    let mut e: metamodelica::Ref<BackendDAE::Equation>;
                    e = BackendEquation::get(allEqs.clone(), eqn.clone())?;
                    r = getEquationStringOrNothing(metamodelica::AsArg::as_arg(&eqn_t), allEqs.clone(), variables, knownVariables, mapIncRowEqn.clone())?;
                    s = MathematicaDump::printMmaEqnStr(&e, &((variables.clone(), knownVariables.clone())))?;
                    Ok(metamodelica::cons(s.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(out)
}

fn unknowsMatchingToMathematicaGrid(
    mut vars: metamodelica::List<i32>,
    mut equations: metamodelica::List<i32>,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut variables: BackendDAE::Variables,
    mut knownVariables: &BackendDAE::Variables,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqsString: metamodelica::List<ArcStr>;
    let mut varString: metamodelica::List<ArcStr>;
    let mut eqns: metamodelica::List<i32>;
    eqns = List::map1r(
        equations,
        &listGet,
        mapIncRowEqn
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
    )?;
    eqsString = getEquationStringOrNothing(&eqns, allEqs, &variables, knownVariables, mapIncRowEqn.clone())?;
    varList = List::map1r(
        vars,
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        variables.clone(),
    )?;
    varString = List::map2(
        varList,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
               __a1: bool,
               __a2: BackendDAE::Variables|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(MathematicaDump::printMmaVarStr(&__a0, __a1, &__a2))
        },
        false,
        variables,
    )?;
    out = verticalGridBoxed(unknowsMatchingToMathematicaGrid2(&varString, &eqsString)?)?;
    Ok(out)
}

fn variablesToMathematicaGrid(
    mut varIndices: metamodelica::List<i32>,
    mut variables: BackendDAE::Variables,
) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqsString: metamodelica::List<ArcStr>;
    varList = List::map1r(
        varIndices.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        variables.clone(),
    )?;
    eqsString = List::map2(
        varList,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
               __a1: bool,
               __a2: BackendDAE::Variables|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(MathematicaDump::printMmaVarStr(&__a0, __a1, &__a2))
        },
        false,
        variables,
    )?;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Grid[{"));
        __mm_s.push_str(&*numerateListIndex(&eqsString, &varIndices)?);
        __mm_s.push_str(&*literal!("},Frame -> All]"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn writeFileIfNonEmpty(mut filename: ArcStr, mut content: ArcStr) -> ArcStr {
    let mut out: ArcStr;
    out = 'mc: {
        let __mc_input = filename.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "" => {
                    Ok(content.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut directory: ArcStr;
                    directory = System::dirname(filename.clone());
                    let true = (System::directoryExists(directory.clone())) else { return Err("pattern mismatch") };
                    System::writeFile(filename.clone(), content.clone())?;
                    Ok(literal!("Done..."))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(content.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out
}

fn dumpVarDistributionInfo(mut d: Option<metamodelica::Ref<DAE::Distribution>>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = (::match_deref::match_deref! { match &(d) {
        Some(Deref @ DAE::Distribution { name, params, paramNames }) => {
            let mut e1: ArcStr;
            let mut e2: ArcStr;
            let mut e3: ArcStr;
            let mut s: ArcStr;
            let mut s1: ArcStr;
            e1 = MathematicaDump::printExpMmaStr(name.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
            e2 = MathematicaDump::printExpMmaStr(params.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
            e3 = MathematicaDump::printExpMmaStr(paramNames.clone(), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())), &(BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
            s1 = stringDelimitList(list![e1, e2, e3], literal!(","));
            s = stringAppendList(list![literal!("{"), s1, literal!("}")]);
            s
        },
        None => {
            literal!("\"None\"")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn dumpVarsDistributionInfo(mut d: metamodelica::List<Option<metamodelica::Ref<DAE::Distribution>>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(d, &dumpVarDistributionInfo)?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

fn getEquationsWithApproximatedAnnotation(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::List<i32>> {
    let mut outEqs: metamodelica::List<i32>;
    outEqs = (::match_deref::match_deref! { match dae {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedEqs, .. }, tail: _ }, shared: _ } => {
            let mut ret: metamodelica::List<i32>;
            ret = getEquationsWithApproximatedAnnotation2(&(BackendEquation::equationList(orderedEqs.clone())?), 1)?;
            ret
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqs)
}

fn getEquationsWithApproximatedAnnotation2(
    mut eqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut index: i32,
) -> Result<metamodelica::List<i32>> {
    let mut listOut: metamodelica::List<i32>;
    listOut = 'mc: {
        let __mc_input = (&**eqs, index);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: h, tail: t }, i) => {
                    let mut inner_ret: metamodelica::List<i32>;
                    let true = (isApproximatedEquation(metamodelica::AsArg::as_arg(&h))) else { return Err("pattern mismatch") };
                    inner_ret = getEquationsWithApproximatedAnnotation2(metamodelica::AsArg::as_arg(&t), i.clone() + 1)?;
                    Ok(metamodelica::cons(i.clone(), inner_ret.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: t }, i) => {
                    let mut inner_ret: metamodelica::List<i32>;
                    inner_ret = getEquationsWithApproximatedAnnotation2(metamodelica::AsArg::as_arg(&t), i.clone() + 1)?;
                    Ok(inner_ret.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(listOut)
}

fn isApproximatedEquation(mut eqn: &metamodelica::Ref<BackendDAE::Equation>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match eqn {
        Deref @ BackendDAE::Equation::EQUATION { source: Deref @ DAE::ElementSource { comment, .. }, .. } => {
            let mut ret: bool;
            ret = isApproximatedEquation2(metamodelica::AsArg::as_arg(&comment));
            ret
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn isApproximatedEquation2(mut commentIn: &metamodelica::List<metamodelica::Ref<SCode::Comment>>) -> bool {
    let mut out: bool;
    out = 'mc: {
        let __mc_input = &**commentIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst, .. } }), .. }, tail: t } => {
                    let mut ret: bool;
                    ret = List::any(metamodelica::AsArg::as_arg(&subModLst), &move |__a0: metamodelica::Ref<SCode::SubMod>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isApproximatedEquation3(&__a0)) })? || isApproximatedEquation2(metamodelica::AsArg::as_arg(&t));
                    Ok(ret)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: t } => {
                    let mut ret: bool;
                    ret = isApproximatedEquation2(metamodelica::AsArg::as_arg(&t));
                    Ok(ret)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out
}

fn isApproximatedEquation3(mut m: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match m {
        Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_ApproximatedEquation", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn flattenModel(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut p: Absyn::Program,
    mut icache: FCore::Cache,
) -> Result<(DAE::DAElist, FCore::Cache, FCore::Graph)> {
    let mut daeOut: DAE::DAElist;
    let mut cacheOut: FCore::Cache;
    let mut graphOut: FCore::Graph;
    (daeOut, cacheOut, graphOut) = 'mc: {
        let __mc_input = icache.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut p_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut dae: DAE::DAElist;
            let mut graph: FCore::Graph;
            let mut cache: FCore::Cache;
            System::realtimeTick(ClockIndexes::RT_CLOCK_UNCERTAINTIES.clone())?;
            p_1 = AbsynToSCode::translateAbsyn2SCode(p.clone())?;
            (cache, graph, _, dae) = Inst::instantiateClass(
                icache.clone(),
                InnerOuter::emptyInstHierarchy().clone(),
                p_1.clone(),
                className.clone(),
                true,
                true,
                true,
            )?;
            System::realtimeTock(ClockIndexes::RT_CLOCK_UNCERTAINTIES.clone())?;
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(
                cache.clone(),
                graph.clone(),
                dae.clone(),
                &move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: DAE::DAElist| {
                    StateMachineFlatten::stateMachineToDataFlow(&__a0, &__a1, __a2)
                },
            )?;
            Ok((dae.clone(), cache.clone(), graph.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut resstr: ArcStr;
            resstr = AbsynUtil::pathStringNoQual(className.clone(), literal!("."), false, false)?;
            resstr = stringAppendList(list![
                literal!("modelEquationsUC: The model "),
                resstr.clone(),
                literal!(" could not be flattened")
            ]);
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![resstr.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((daeOut, cacheOut, graphOut))
}

fn getMathematicaVarStr(mut vars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut states: metamodelica::List<ArcStr>;
    let mut algs: metamodelica::List<ArcStr>;
    let mut outputs: metamodelica::List<ArcStr>;
    let mut inputsStates: metamodelica::List<ArcStr>;
    (states, algs, outputs, inputsStates) = MathematicaDump::printMmaVarsStr(vars)?;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*Util::stringDelimitListNonEmptyElts(
            listAppend(states, listAppend(algs, listAppend(outputs, inputsStates))),
            literal!(","),
        )?);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn getMathematicaEqStr(
    mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut systemVars: BackendDAE::Variables,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = MathematicaDump::printMmaEqnsStr(eqns, (systemVars, globalKnownVars))?;
    Ok(out)
}

fn getEquationsForUnknownsSystem(
    mut m: &ExtAdjacencyMatrix,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut eqnsOut: metamodelica::List<i32>;
    let mut varsOut: metamodelica::List<i32>;
    (eqnsOut, varsOut) = (::match_deref::match_deref! { match &(unknowns.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        _ => {
            let mut unknownsSystem: ExtAdjacencyMatrix;
            let mut yEqMap: metamodelica::List<i32>;
            let mut yVarMap: metamodelica::List<i32>;
            let mut setS: metamodelica::List<i32>;
            let mut nv: i32;
            let mut ne: i32;
            let mut my: metamodelica::Array<metamodelica::List<i32>>;
            let mut ass1: metamodelica::Array<i32>;
            let mut ass2: metamodelica::Array<i32>;
            let mut vars: metamodelica::List<i32>;
            unknownsSystem = getSystemForUnknowns(m, knowns, unknowns)?;
            (yEqMap, yVarMap, my) = prepareForMatching(&unknownsSystem)?;
            ne = ((yEqMap).len() as i32);
            nv = ((yVarMap).len() as i32);
            ass1 = arrayCreate(ne, -1);
            ass2 = arrayCreate(nv, -1);
            let true = (BackendDAEEXT::setAssignment(ne, nv, ass1.clone(), ass2.clone())) else { return Err("pattern mismatch") };
            Matching::matchingExternalsetAdjacencyMatrix(nv, ne, my.clone())?;
            BackendDAEEXT::matching(nv, ne, 1, -1, metamodelica::OrderedFloat(0.0_f64), 0);
            BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
            vars = yVarMap;
            setS = restoreIndicesEquivalence(&(List::filter1OnTrue(ass2.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), -1)?), &yEqMap)?;
            (setS, vars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqnsOut, varsOut))
}

fn getEquationsForKnownsSystem(
    mut m: &ExtAdjacencyMatrix,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: &metamodelica::List<i32>,
    mut setS: &metamodelica::List<i32>,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut variables: BackendDAE::Variables,
    mut knownVariables: BackendDAE::Variables,
    mut mapIncRowEqn: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut setCOut: metamodelica::List<i32>;
    let mut removed_equations_squaredOut: metamodelica::List<i32>;
    (setCOut, removed_equations_squaredOut) = 'mc: {
        let __mc_input = &*knowns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut knownsSystem: ExtAdjacencyMatrix;
                    knownsSystem = removeEquations(m, setS)?;
                    knownsSystem = removeUnrelatedEquations(knownsSystem.clone(), knowns.clone())?;
                    let true = ((knownsSystem).is_empty()) else { return Err("pattern mismatch") };
                    metamodelica::print(literal!("Warning: The system is ill-posed. There are no remaining equations containing the knowns.\n"));
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut knownsSystem: ExtAdjacencyMatrix;
                    let mut knownsSystemComp: ExtAdjacencyMatrix;
                    let mut xEqMap: metamodelica::List<i32>;
                    let mut xVarMap: metamodelica::List<i32>;
                    let mut mx: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
                    let mut ass1: metamodelica::Array<i32>;
                    let mut ass2: metamodelica::Array<i32>;
                    let mut comps: metamodelica::List<metamodelica::List<i32>>;
                    let mut comps_fixed: metamodelica::List<metamodelica::List<i32>>;
                    let mut setC: metamodelica::List<i32>;
                    let mut removed_equations_squared: metamodelica::List<i32>;
                    let mut nxVarMap: i32;
                    let mut nxEqMap: i32;
                    let mut size: i32;
                    knownsSystem = removeEquations(m, setS)?;
                    knownsSystem = removeUnrelatedEquations(knownsSystem.clone(), knowns.clone())?;
                    printSep(getMathematicaText(&(literal!("System of knowns after step 7"))))?;
                    printSep(equationsToMathematicaGrid(getEquationsNumber(&knownsSystem), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?)?;
                    knownsSystemComp = sortEquations(&knownsSystem, knowns.clone())?;
                    knownsSystemComp = removeVarsNotInSet(&knownsSystemComp, knowns.clone())?;
                    (xEqMap, xVarMap, mx) = prepareForMatching(&knownsSystemComp)?;
                    nxVarMap = ((xVarMap).len() as i32);
                    nxEqMap = ((xEqMap).len() as i32);
                    size = if (nxEqMap > nxVarMap) {nxEqMap} else {nxVarMap};
                    Matching::matchingExternalsetAdjacencyMatrix(size, size, mx.clone())?;
                    ass1 = arrayCreate(size, 0);
                    ass2 = arrayCreate(size, 0);
                    let true = (BackendDAEEXT::setAssignment(size, size, ass2.clone(), ass1.clone())) else { return Err("pattern mismatch") };
                    BackendDAEEXT::matching(size, size, 1, -1, metamodelica::OrderedFloat(1.0_f64), 0);
                    BackendDAEEXT::getAssignment(ass1.clone(), ass2.clone())?;
                    mt = AdjacencyMatrix::transposeAdjacencyMatrix(mx.clone(), nxVarMap)?;
                    comps = getComponentsWrapper(mx.clone(), mt.clone(), ass1.clone(), ass2.clone())?;
                    comps = removeDummyEquations(&comps, ((xEqMap).len() as i32))?;
                    comps_fixed = List::map1(comps.clone(), &move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<i32>| restoreIndicesEquivalence(&__a0, &__a1), xEqMap.clone())?;
                    (knownsSystem, removed_equations_squared) = removeEquationInSquaredBlock(&knownsSystem, &knowns, unknowns, &comps_fixed)?;
                    comps_fixed = List::map1(comps_fixed.clone(), &move |__a0: metamodelica::List<i32>, __a1: metamodelica::List<i32>| restoreIndicesEquivalence(&__a0, &__a1), mapIncRowEqn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())?;
                    printSep(getMathematicaText(&(literal!("Blocks (each row is a block)"))))?;
                    printSep({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Grid[")); __mm_s.push_str(&*listString(List::map(comps_fixed.clone(), &intListString)?)); __mm_s.push_str(&*literal!(",Frame->All]")); ArcStr::from(__mm_s) })?;
                    printSep(getMathematicaText(&(literal!("System of knowns after step 8 and 9"))))?;
                    printSep(equationsToMathematicaGrid(getEquationsNumber(&knownsSystem), allEqs.clone(), variables.clone(), knownVariables.clone(), mapIncRowEqn.clone())?)?;
                    checkSystemContainsVars(&knownsSystem, &knowns, &variables)?;
                    setC = getEquationsNumber(&knownsSystem);
                    Ok((setC.clone(), removed_equations_squared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((setCOut, removed_equations_squaredOut))
}

fn printVarReduction(mut elems: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>) -> Result<()> {
    metamodelica::print(literal!("Reduced variables:\n"));
    metamodelica::print(stringDelimitList(
        List::map(elems, &move |__a0: (
            metamodelica::List<i32>,
            metamodelica::List<i32>,
        )| printVarReduction2(&__a0))?,
        literal!("\n"),
    ));
    Ok(())
}

fn printVarReduction2(mut elem: &(metamodelica::List<i32>, metamodelica::List<i32>)) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut occurrences: metamodelica::List<i32>;
    let mut vars: metamodelica::List<i32>;
    (occurrences, vars) = elem.clone();
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*stringDelimitList(
            List::map(vars, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!(") ("));
        __mm_s.push_str(&*stringDelimitList(
            List::map(occurrences, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn pickReductionCandidates<'__b>(
    mut elems: &'__b metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>,
) -> metamodelica::List<metamodelica::List<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match elems {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: (occurrence, vars), tail: tail } if (((vars).len() as i32) > 1 && ((occurrence).len() as i32) > 1) => {
                let mut newElems: metamodelica::List<metamodelica::List<i32>>;
                newElems = pickReductionCandidates(tail);
                return metamodelica::cons(vars.clone(), newElems)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: tail } => {
                { elems = tail; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn reduceVariables(mut m: ExtAdjacencyMatrix, mut knowns: metamodelica::List<i32>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    let mut neq: i32 = 0;
    let mut nvar: i32 = 0;
    let mut variables: metamodelica::List<i32> = metamodelica::nil();
    let mut occurrences: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut candidates: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut reducedVars: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)> = metamodelica::nil();
    let mut newM: ExtAdjacencyMatrix = metamodelica::nil();
    mOut = 'mc: {
        let __mc_input = &*knowns;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut neq: i32 = neq.clone();
                    let mut nvar: i32 = nvar.clone();
                    let mut variables: metamodelica::List<i32> = variables.clone();
                    neq = (((getEquationsNumber(&m))).len() as i32);
                    variables = getVariables(&m);
                    nvar = ((variables).len() as i32);
                    let true = (neq >= nvar) else { return Err("pattern mismatch") };
                    Ok((m.clone(), neq.clone(), nvar.clone(), variables.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            neq = __wb0;
            nvar = __wb1;
            variables = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut candidates: metamodelica::List<metamodelica::List<i32>> = candidates.clone();
                    let mut neq: i32 = neq.clone();
                    let mut newM: metamodelica::List<(i32, metamodelica::List<i32>)> = newM.clone();
                    let mut nvar: i32 = nvar.clone();
                    let mut occurrences: metamodelica::List<metamodelica::List<i32>> = occurrences.clone();
                    let mut reducedVars: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)> = reducedVars.clone();
                    let mut variables: metamodelica::List<i32> = variables.clone();
                    neq = (((getEquationsNumber(&m))).len() as i32);
                    variables = getVariables(&m);
                    nvar = ((variables).len() as i32);
                    let true = (neq < nvar) else { return Err("pattern mismatch") };
                    occurrences = List::map1r(knowns.clone(), &move |__a0: metamodelica::List<(i32, metamodelica::List<i32>)>, __a1: i32| -> metamodelica::Result<_> { ::std::result::Result::Ok(occurrencesOfVariable(&__a0, __a1)) }, m.clone())?;
                    reducedVars = findReductionCantidates(&variables, &occurrences, metamodelica::nil())?;
                    candidates = pickReductionCandidates(&reducedVars);
                    newM = reduceVariablesInMatrix(m.clone(), &candidates, nvar - neq)?;
                    Ok((newM.clone(), candidates.clone(), neq.clone(), newM.clone(), nvar.clone(), occurrences.clone(), reducedVars.clone(), variables.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            candidates = __wb0;
            neq = __wb1;
            newM = __wb2;
            nvar = __wb3;
            occurrences = __wb4;
            reducedVars = __wb5;
            variables = __wb6;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(mOut)
}

fn reduceVariablesInMatrix<'__b>(
    mut m: ExtAdjacencyMatrix,
    mut candidates: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut count: i32,
) -> Result<ExtAdjacencyMatrix> {
    '__tco: loop {
        ::match_deref::match_deref! { match candidates {
            Deref @ metamodelica::ListNode::Nil if (count > 0) => {
                metamodelica::print(literal!("Warning: The system of equations is under-determined. The results may be incorrect.\n"));
                return Ok(m)
            },
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(m)
            },
            _ if (intEq(count, 0)) => {
                return Ok(m)
            },
            Deref @ metamodelica::ListNode::Cons { head: candidate, tail: candidatesTail } => {
                let mut variables: metamodelica::List<i32>;
                let mut temp: i32;
                let mut newM: ExtAdjacencyMatrix;
                let true = (count > 0) else { return Err("pattern mismatch") };
                temp = (candidate).head().cloned()?;
                variables = List::setDifference(getVariables(&m), &(list![temp]))?;
                newM = removeVarsNotInSet(&m, variables)?;
                { (m, candidates, count) = (newM, candidatesTail, count - 1); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findReductionCantidates<'__b>(
    mut variables: &'__b metamodelica::List<i32>,
    mut occurrences: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut acc: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>,
) -> Result<metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (variables, occurrences) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(acc)
            },
            (Deref @ metamodelica::ListNode::Cons { head: var, tail: varTail }, Deref @ metamodelica::ListNode::Cons { head: occurrence, tail: occurrenceTail }) => {
                let mut newAcc: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>;
                newAcc = findReductionCantidates2(var.clone(), metamodelica::AsArg::as_arg(&occurrence), &acc);
                { (variables, occurrences, acc) = (varTail, occurrenceTail, newAcc); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findReductionCantidates2(
    mut var: i32,
    mut occurrence: &metamodelica::List<i32>,
    mut acc: &metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>,
) -> metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut accOut: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>;
    accOut = 'mc: {
        let __mc_input = &**acc;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut newAcc: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>;
                    newAcc = list![(occurrence.clone(), list![var])];
                    Ok(newAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (elemOccurrences, vars), tail: tail } => {
                    let mut newAcc: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>;
                    let mut elem: (metamodelica::List<i32>, metamodelica::List<i32>);
                    let true = (intEq(((occurrence).len() as i32), ((elemOccurrences).len() as i32))) else { return Err("pattern mismatch") };
                    let true = (containsAll(occurrence, metamodelica::AsArg::as_arg(&elemOccurrences))?) else { return Err("pattern mismatch") };
                    elem = (elemOccurrences.clone(), listAppend(vars.clone(), list![var]));
                    newAcc = metamodelica::cons(elem.clone(), tail.clone());
                    Ok(newAcc.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (elemOccurrences, vars), tail: tail } => {
                    let mut newAcc: metamodelica::List<(metamodelica::List<i32>, metamodelica::List<i32>)>;
                    newAcc = findReductionCantidates2(var, occurrence, metamodelica::AsArg::as_arg(&tail));
                    Ok(metamodelica::cons((elemOccurrences.clone(), vars.clone()), newAcc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    accOut
}

fn eliminateOutputVariables(
    mut mIn: ExtAdjacencyMatrix,
    mut outputs: &metamodelica::List<i32>,
) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    mOut = 'mc: {
        let __mc_input = (mIn, &**outputs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(m.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ metamodelica::ListNode::Cons { head: var, tail: tail }) => {
                    let mut o: metamodelica::List<i32>;
                    let mut newM: ExtAdjacencyMatrix;
                    o = occurrencesOfVariable(metamodelica::AsArg::as_arg(&m), var.clone());
                    let true = (intEq(((o).len() as i32), 1)) else { return Err("pattern mismatch") };
                    newM = removeEquations(metamodelica::AsArg::as_arg(&m), &o)?;
                    newM = eliminateOutputVariables(newM.clone(), metamodelica::AsArg::as_arg(&tail))?;
                    Ok(newM.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (m, Deref @ metamodelica::ListNode::Cons { head: _, tail: tail }) => {
                    let mut newM: ExtAdjacencyMatrix;
                    newM = eliminateOutputVariables(m.clone(), metamodelica::AsArg::as_arg(&tail))?;
                    Ok(newM.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(mOut)
}

fn occurrencesOfVariable(mut m: &ExtAdjacencyMatrix, mut var: i32) -> metamodelica::List<i32> {
    let mut out: metamodelica::List<i32>;
    out = 'mc: {
        let __mc_input = &**m;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (eq, vars), tail: tail } => {
                    let mut ret: metamodelica::List<i32>;
                    let true = (containsAny(metamodelica::AsArg::as_arg(&vars), &(list![var]))?) else { return Err("pattern mismatch") };
                    ret = occurrencesOfVariable(metamodelica::AsArg::as_arg(&tail), var);
                    Ok(metamodelica::cons(eq.clone(), ret.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (_, _), tail: tail } => {
                    let mut ret: metamodelica::List<i32>;
                    ret = occurrencesOfVariable(metamodelica::AsArg::as_arg(&tail), var);
                    Ok(ret.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out
}

fn getEquationsNumber(mut m: &ExtAdjacencyMatrix) -> metamodelica::List<i32> {
    let mut numbers: metamodelica::List<i32>;
    numbers = (::match_deref::match_deref! { match m {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (eq, _), tail: t } => {
            let mut inner_ret: metamodelica::List<i32>;
            inner_ret = getEquationsNumber(t);
            metamodelica::cons(eq.clone(), inner_ret)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    numbers
}

fn getMathematicaText(mut text: &ArcStr) -> ArcStr {
    let mut textOut: ArcStr;
    textOut = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Text[Style[\""));
        __mm_s.push_str(&*text);
        __mm_s.push_str(&*literal!("\",Bold,Large]]"));
        ArcStr::from(__mm_s)
    };
    textOut
}

fn getComponentsWrapper(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mt: metamodelica::Array<metamodelica::List<i32>>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut compsOut: metamodelica::List<metamodelica::List<i32>>;
    compsOut = 'mc: {
        let __mc_input = ass2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intEq(0, metamodelica::arrayLength(m.clone()))) else {
                return Err("pattern mismatch");
            };
            Ok(list![metamodelica::nil()])
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (intEq(1, metamodelica::arrayLength(m.clone()))) else {
                return Err("pattern mismatch");
            };
            Ok(list![list![1]])
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut comps: metamodelica::List<metamodelica::List<i32>>;
            let mut comp: metamodelica::List<i32>;
            if '__try0: {
                unwrap_break_err!(Sorting::TarjanTransposed(mt.clone(), ass2.clone()), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_ok()
            {
                return Err("failure(): body succeeded");
            }
            metamodelica::print(literal!("TarjanAlgorithm failed\n"));
            Error::clearMessages();
            comp = List::intRange(metamodelica::arrayLength(m.clone()));
            comps = list![comp.clone()];
            Ok(comps.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut comps: metamodelica::List<metamodelica::List<i32>>;
            comps = Sorting::TarjanTransposed(mt.clone(), ass2.clone())?;
            Ok(comps.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(compsOut)
}

fn getVariables(mut m: &ExtAdjacencyMatrix) -> metamodelica::List<i32> {
    let mut varsOut: metamodelica::List<i32>;
    varsOut = (::match_deref::match_deref! { match m {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (_, vars), tail: t } => {
            let mut newVars: metamodelica::List<i32>;
            newVars = listAppend(vars.clone(), getVariables(t));
            newVars = List::unique(&newVars);
            newVars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varsOut
}

fn removeEquationInSquaredBlock(
    mut m: &ExtAdjacencyMatrix,
    mut knowns: &metamodelica::List<i32>,
    mut unknowns: &metamodelica::List<i32>,
    mut components: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<(ExtAdjacencyMatrix, metamodelica::List<i32>)> {
    let mut mOut: ExtAdjacencyMatrix;
    let mut removedEquations: metamodelica::List<i32>;
    (mOut, removedEquations) = 'mc: {
        let __mc_input = &**components;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut vars: metamodelica::List<i32>;
                    let mut usedKnowns: metamodelica::List<i32>;
                    let mut compEqns: ExtAdjacencyMatrix;
                    let mut compsSorted: ExtAdjacencyMatrix;
                    let mut tailEquations: ExtAdjacencyMatrix;
                    let mut inner_ret: ExtAdjacencyMatrix;
                    let mut removeEquation: i32;
                    let mut removed_inner: metamodelica::List<i32>;
                    compEqns = getEquations(m.clone(), h.clone())?;
                    vars = getVariables(&compEqns);
                    usedKnowns = List::intersectionOnTrue(&vars, knowns, &fnptr!(intEq, i32, i32))?;
                    let true = (intEq(((h).len() as i32), ((usedKnowns).len() as i32))) else { return Err("pattern mismatch") };
                    compsSorted = sortEquations(&compEqns, unknowns.clone())?.reverse();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(compsSorted.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: (__pa0, _), tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    removeEquation = metamodelica::Own::own(__pa0);
                    tailEquations = metamodelica::Own::own(__pa1);
                    (inner_ret, removed_inner) = removeEquationInSquaredBlock(m, knowns, unknowns, metamodelica::AsArg::as_arg(&t))?;
                    removed_inner = if (((compsSorted).len() as i32) > 1) {metamodelica::cons(removeEquation, removed_inner.clone())} else {removed_inner.clone()};
                    Ok((listAppend(tailEquations.clone(), inner_ret.clone()), removed_inner.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut vars: metamodelica::List<i32>;
                    let mut usedKnowns: metamodelica::List<i32>;
                    let mut compEqns: ExtAdjacencyMatrix;
                    let mut inner_ret: ExtAdjacencyMatrix;
                    let mut removed_inner: metamodelica::List<i32>;
                    compEqns = getEquations(m.clone(), h.clone())?;
                    vars = getVariables(&compEqns);
                    usedKnowns = List::intersectionOnTrue(&vars, knowns, &fnptr!(intEq, i32, i32))?;
                    let false = (intEq(((h).len() as i32), ((usedKnowns).len() as i32))) else { return Err("pattern mismatch") };
                    (inner_ret, removed_inner) = removeEquationInSquaredBlock(m, knowns, unknowns, metamodelica::AsArg::as_arg(&t))?;
                    Ok((listAppend(compEqns.clone(), inner_ret.clone()), removed_inner.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((mOut, removedEquations))
}

fn printIntList(mut l: metamodelica::List<i32>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("List of size = "));
        __mm_s.push_str(&*intString(((l).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    metamodelica::print(stringDelimitList(List::map(l, &fnptr!(intString, i32))?, literal!(",")));
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn intListString(mut l: metamodelica::List<i32>) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(l, &fnptr!(intString, i32))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn listString(mut l: metamodelica::List<ArcStr>) -> ArcStr {
    let mut out: ArcStr;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(l, literal!(",")));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    out
}

fn setOfList(mut inList: &metamodelica::List<i32>) -> metamodelica::List<i32> {
    let mut outList: metamodelica::List<i32>;
    outList = List::unique(inList);
    outList
}

fn countKnowns(mut row: &ExtAdjacencyMatrixRow, mut knowns: &metamodelica::List<i32>) -> Result<i32> {
    let mut out: i32;
    out = (::match_deref::match_deref! { match &(row) {
        (_, vars) => {
            let mut n: i32;
            n = (((List::intersectionOnTrue(metamodelica::AsArg::as_arg(&vars), knowns, &fnptr!(intEq, i32, i32))?)).len() as i32);
            n
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn sortEquations(mut m: &ExtAdjacencyMatrix, mut knowns: metamodelica::List<i32>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    mOut = sortBy1(
        m,
        &move |__a0: (i32, metamodelica::List<i32>), __a1: metamodelica::List<i32>| countKnowns(&__a0, &__a1),
        knowns,
    )?;
    Ok(mOut)
}

fn removeVarsNotInSet_helper(mut var: i32, mut elems: &metamodelica::List<i32>) -> Result<bool> {
    let mut out: bool;
    out = containsAny(&(list![var]), elems)?;
    Ok(out)
}

fn removeVarsNotInSet(mut m: &ExtAdjacencyMatrix, mut set: metamodelica::List<i32>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix = metamodelica::nil();
    let mut vars: metamodelica::List<i32>;
    let mut newVars: metamodelica::List<i32>;
    let mut eq: i32;
    for mut el in &**m {
        (eq, vars) = el.clone();
        newVars = List::filter1OnTrue(
            vars,
            (std::sync::Arc::new(move |__a0: i32, __a1: metamodelica::List<i32>| removeVarsNotInSet_helper(__a0, &__a1))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::List<i32>) -> Result<bool> + 'static>),
            set.clone(),
        )?;
        if !((newVars).is_empty()) {
            mOut = metamodelica::cons((eq, newVars), mOut);
        }
    }
    mOut = metamodelica::Dangerous::listReverseInPlace(mOut);
    Ok(mOut)
}

fn removeEquations(mut m: &ExtAdjacencyMatrix, mut eqns: &metamodelica::List<i32>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    mOut = 'mc: {
        let __mc_input = &**m;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e @ (eq, _), tail: t } => {
                    let mut inner_ret: ExtAdjacencyMatrix;
                    let false = (containsAny(&(list![eq.clone()]), eqns)?) else { return Err("pattern mismatch") };
                    inner_ret = removeEquations(metamodelica::AsArg::as_arg(&t), eqns)?;
                    Ok(metamodelica::cons(e.clone(), inner_ret.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (eq, _), tail: t } => {
                    let mut inner_ret: ExtAdjacencyMatrix;
                    let true = (containsAny(&(list![eq.clone()]), eqns)?) else { return Err("pattern mismatch") };
                    inner_ret = removeEquations(metamodelica::AsArg::as_arg(&t), eqns)?;
                    Ok(inner_ret.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(mOut)
}

fn getEquationsHelper(mut m: &ExtAdjacencyMatrixRow, mut eqns: &metamodelica::List<i32>) -> Result<bool> {
    let mut out: bool;
    out = (::match_deref::match_deref! { match &(m) {
        (e, _) => {
            List::isMemberOnTrue(e.clone(), eqns, &fnptr!(intEq, i32, i32))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn getEquations(mut m: ExtAdjacencyMatrix, mut eqns: metamodelica::List<i32>) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    mOut = List::filter1OnTrue(
        m,
        (std::sync::Arc::new(
            move |__a0: (i32, metamodelica::List<i32>), __a1: metamodelica::List<i32>| getEquationsHelper(&__a0, &__a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn((i32, metamodelica::List<i32>), metamodelica::List<i32>) -> Result<bool> + 'static,
            >),
        eqns,
    )?;
    Ok(mOut)
}

fn removeUnrelatedEquations2(mut row: &ExtAdjacencyMatrixRow, mut knowns: &metamodelica::List<i32>) -> Result<bool> {
    let mut out: bool;
    out = (::match_deref::match_deref! { match &(row) {
        (_, vars) => {
            let mut ret: bool;
            ret = containsAny(metamodelica::AsArg::as_arg(&vars), knowns)?;
            ret
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn removeUnrelatedEquations(
    mut m: ExtAdjacencyMatrix,
    mut knowns: metamodelica::List<i32>,
) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    mOut = List::filter1OnTrue(
        m,
        (std::sync::Arc::new(
            move |__a0: (i32, metamodelica::List<i32>), __a1: metamodelica::List<i32>| {
                removeUnrelatedEquations2(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn((i32, metamodelica::List<i32>), metamodelica::List<i32>) -> Result<bool> + 'static,
            >),
        knowns,
    )?;
    Ok(mOut)
}

fn checkSystemContainsVars(
    mut m: &ExtAdjacencyMatrix,
    mut knows: &metamodelica::List<i32>,
    mut variables: &BackendDAE::Variables,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**knows;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut not_found_var: metamodelica::Ref<BackendDAE::Var>;
                    let mut r#str: ArcStr;
                    let true = (((removeUnrelatedEquations(m.clone(), list![h.clone()])?)).is_empty()) else { return Err("pattern mismatch") };
                    not_found_var = BackendVariable::getVarAt(variables, h.clone())?;
                    r#str = ComponentReference::crefStr(&(BackendVariable::varCref(&not_found_var)))?;
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: The variable '")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("' was not found in the system of knowns\n")); ArcStr::from(__mm_s) });
                    checkSystemContainsVars(m, metamodelica::AsArg::as_arg(&t), variables)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let false = (((removeUnrelatedEquations(m.clone(), list![h.clone()])?)).is_empty()) else { return Err("pattern mismatch") };
                    checkSystemContainsVars(m, metamodelica::AsArg::as_arg(&t), variables)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getSystemForUnknowns(
    mut m: &ExtAdjacencyMatrix,
    mut knowns: metamodelica::List<i32>,
    mut unknowns: metamodelica::List<i32>,
) -> Result<ExtAdjacencyMatrix> {
    let mut mOut: ExtAdjacencyMatrix;
    let mut mTemp: ExtAdjacencyMatrix;
    mTemp = sortEquations(m, knowns)?;
    mOut = removeVarsNotInSet(&mTemp, unknowns)?;
    Ok(mOut)
}

fn getRelatedVariables(
    mut m: &ExtAdjacencyMatrix,
    mut vars: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut varsOut: metamodelica::List<i32>;
    varsOut = 'mc: {
        let __mc_input = &**m;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (_, eqvars), tail: t } => {
                    let mut eqvars = (*eqvars).clone();
                    let true = (containsAny(metamodelica::AsArg::as_arg(&eqvars), vars)?) else { return Err("pattern mismatch") };
                    eqvars = listAppend(eqvars.clone(), getRelatedVariables(metamodelica::AsArg::as_arg(&t), vars)?);
                    eqvars = List::setDifference(setOfList(metamodelica::AsArg::as_arg(&eqvars)), vars)?;
                    Ok(eqvars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (_, eqvars), tail: t } => {
                    let mut eqvars = (*eqvars).clone();
                    let false = (containsAny(metamodelica::AsArg::as_arg(&eqvars), vars)?) else { return Err("pattern mismatch") };
                    eqvars = getRelatedVariables(metamodelica::AsArg::as_arg(&t), vars)?;
                    eqvars = List::setDifference(setOfList(metamodelica::AsArg::as_arg(&eqvars)), vars)?;
                    Ok(eqvars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(varsOut)
}

fn restoreIndicesEquivalence(
    mut inList: &metamodelica::List<i32>,
    mut map: &metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    let mut out: metamodelica::List<i32>;
    out = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
            let mut inner_ret: metamodelica::List<i32>;
            let mut v: i32;
            v = (map).get(h.clone())?;
            inner_ret = restoreIndicesEquivalence(t, map)?;
            metamodelica::cons(v, inner_ret)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn addIndexEquivalence(mut index: i32, mut map: metamodelica::List<i32>) -> Result<(i32, metamodelica::List<i32>)> {
    let mut indexOut: i32;
    let mut mapOut: metamodelica::List<i32>;
    (indexOut, mapOut) = 'mc: {
        let __mc_input = &*map;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pos: i32;
                    let true = (List::isMemberOnTrue(index, &map, &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    pos = List::position(index, &map)?;
                    Ok((pos, map.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pos: i32;
                    let mut newMap: metamodelica::List<i32>;
                    let false = (List::isMemberOnTrue(index, &map, &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    pos = ((map).len() as i32) + 1;
                    newMap = listAppend(map.clone(), list![index]);
                    Ok((pos, newMap.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((indexOut, mapOut))
}

fn addVarEquivalences<'__b>(
    mut vars: &'__b metamodelica::List<i32>,
    mut map: metamodelica::List<i32>,
    mut varsFixed: metamodelica::List<i32>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match vars {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((map, varsFixed))
            },
            Deref @ metamodelica::ListNode::Cons { head: h, tail: remaining } => {
                let mut v: i32;
                let mut newMap: metamodelica::List<i32>;
                let mut innerVars: metamodelica::List<i32>;
                let mut innerMap: metamodelica::List<i32>;
                (v, newMap) = addIndexEquivalence(h.clone(), map)?;
                { (vars, map, varsFixed) = (remaining, newMap, metamodelica::cons(v, varsFixed)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn prepareForMatching2<'__b>(
    mut mExt: &'__b ExtAdjacencyMatrix,
    mut eqMap: metamodelica::List<i32>,
    mut varMap: metamodelica::List<i32>,
    mut m: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match mExt {
            Deref @ metamodelica::ListNode::Nil => {
                let mut newM: metamodelica::List<metamodelica::List<i32>>;
                newM = m.reverse();
                return Ok((eqMap, varMap, newM))
            },
            Deref @ metamodelica::ListNode::Cons { head: (eq, vars), tail: t } => {
                let mut newVarMap: metamodelica::List<i32>;
                let mut newEqMap: metamodelica::List<i32>;
                let mut newVars: metamodelica::List<i32>;
                let mut newM: metamodelica::List<metamodelica::List<i32>>;
                (_, newEqMap) = addIndexEquivalence(eq.clone(), eqMap)?;
                (newVarMap, newVars) = addVarEquivalences(metamodelica::AsArg::as_arg(&vars), varMap, metamodelica::nil())?;
                { (mExt, eqMap, varMap, m) = (t, newEqMap, newVarMap, metamodelica::cons(newVars, m)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn prepareForMatching(
    mut mExt: &ExtAdjacencyMatrix,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut eqMap: metamodelica::List<i32>;
    let mut varMap: metamodelica::List<i32>;
    let mut mOut: metamodelica::Array<metamodelica::List<i32>>;
    let mut m: metamodelica::List<metamodelica::List<i32>>;
    (eqMap, varMap, m) = prepareForMatching2(mExt, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
    mOut = metamodelica::arrayFromVec(
        fixUnderdeterminedSystem(m, ((varMap).len() as i32), ((eqMap).len() as i32))
            .into_iter()
            .cloned()
            .collect(),
    );
    Ok((eqMap, varMap, mOut))
}

fn removeDummyEquations(
    mut comps: &metamodelica::List<metamodelica::List<i32>>,
    mut max_neqs: i32,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut out: metamodelica::List<metamodelica::List<i32>>;
    out = (::match_deref::match_deref! { match comps {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
            let mut ret: metamodelica::List<metamodelica::List<i32>>;
            let mut row: metamodelica::List<i32>;
            row = List::removeOnTrue(max_neqs, &fnptr!(intLt, i32, i32), h.clone())?;
            ret = removeDummyEquations(t, max_neqs)?;
            metamodelica::cons(row, ret)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn fixUnderdeterminedSystem(
    mut m: metamodelica::List<metamodelica::List<i32>>,
    mut nvars: i32,
    mut neqs: i32,
) -> metamodelica::List<metamodelica::List<i32>> {
    '__tco: loop {
        if intGt(nvars, neqs) {
            {
                (m, nvars, neqs) = (listAppend(m, list![List::intRange(nvars)]), nvars, neqs + 1);
                continue '__tco;
            }
        } else {
            return m;
        }
    }
}

fn getExtAdjacencyMatrix(mut m: metamodelica::Array<metamodelica::List<i32>>) -> ExtAdjacencyMatrix {
    let mut mOut: ExtAdjacencyMatrix;
    mOut = getExtAdjacencyMatrix2(
        1,
        &(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()),
        metamodelica::nil(),
    );
    mOut
}

fn getExtAdjacencyMatrix2<'__b>(
    mut i: i32,
    mut m: &'__b metamodelica::List<metamodelica::List<i32>>,
    mut acc: ExtAdjacencyMatrix,
) -> ExtAdjacencyMatrix {
    '__tco: loop {
        ::match_deref::match_deref! { match m {
            Deref @ metamodelica::ListNode::Nil => {
                return acc.reverse()
            },
            Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                { (i, m, acc) = (i + 1, t, metamodelica::cons((i, h.clone()), acc)); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn dumpExtAdjacencyMatrix(mut m: &ExtAdjacencyMatrix) -> Result<()> {
    let () = (::match_deref::match_deref! { match m {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: (eq, vars), tail: t } => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::map(vars.clone(), &fnptr!(intString, i32))?, literal!(","))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            dumpExtAdjacencyMatrix(t)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn containsAny(mut m1: &metamodelica::List<i32>, mut m2: &metamodelica::List<i32>) -> Result<bool> {
    let mut out: bool;
    let mut m3: metamodelica::List<i32>;
    m3 = List::intersectionOnTrue(m1, m2, &fnptr!(intEq, i32, i32))?;
    out = !((m3).is_empty());
    Ok(out)
}

fn containsAll(mut m1: &metamodelica::List<i32>, mut m2: &metamodelica::List<i32>) -> Result<bool> {
    let mut out: bool;
    let mut m3: metamodelica::List<i32>;
    m3 = List::intersectionOnTrue(m1, m2, &fnptr!(intEq, i32, i32))?;
    out = intEq(((m3).len() as i32), ((m2).len() as i32));
    Ok(out)
}

pub(crate) fn getUncertainRefineVariableIndexes(
    mut allVariables: &BackendDAE::Variables,
    mut variableIndexList: &metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<Option<metamodelica::Ref<DAE::Distribution>>>,
)> {
    let mut indices: metamodelica::List<i32>;
    let mut distributions: metamodelica::List<Option<metamodelica::Ref<DAE::Distribution>>>;
    (indices, distributions) = 'mc: {
        let __mc_input = &**variableIndexList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: index, tail: variableIndexListRest } => {
                    let mut refineVariableIndexList: metamodelica::List<i32>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut dist: Option<metamodelica::Ref<DAE::Distribution>>;
                    let mut distInner: metamodelica::List<Option<metamodelica::Ref<DAE::Distribution>>>;
                    var = BackendVariable::getVarAt(allVariables, index.clone())?;
                    let true = (BackendVariable::varHasUncertainValueRefine(&var)) else { return Err("pattern mismatch") };
                    dist = BackendVariable::varTryGetDistribution(&var);
                    (refineVariableIndexList, distInner) = getUncertainRefineVariableIndexes(allVariables, metamodelica::AsArg::as_arg(&variableIndexListRest))?;
                    Ok((metamodelica::cons(index.clone(), refineVariableIndexList.clone()), metamodelica::cons(dist.clone(), distInner.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: index, tail: variableIndexListRest } => {
                    let mut refineVariableIndexList: metamodelica::List<i32>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut distInner: metamodelica::List<Option<metamodelica::Ref<DAE::Distribution>>>;
                    var = BackendVariable::getVarAt(allVariables, index.clone())?;
                    let false = (BackendVariable::varHasUncertainValueRefine(&var)) else { return Err("pattern mismatch") };
                    (refineVariableIndexList, distInner) = getUncertainRefineVariableIndexes(allVariables, metamodelica::AsArg::as_arg(&variableIndexListRest))?;
                    Ok((refineVariableIndexList.clone(), distInner.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("getUncertainRefineVariableIndexes failed!\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((indices, distributions))
}

pub(crate) fn eliminateVariablesDAE(
    mut elimVarIndexList: &metamodelica::List<i32>,
    mut indae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDae: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDae = (::match_deref::match_deref! { match &(indae) {
        dae @ Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, tail: _ }, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, initialEqs: ieqns, .. } } => {
            let mut vars_1: BackendDAE::Variables;
            let mut kvars_1: BackendDAE::Variables;
            let mut crefDouble: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut m: metamodelica::Array<metamodelica::List<i32>>;
            let mut movedvars_1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut eqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut dae = (*dae).clone();
            eqnLst = BackendEquation::equationList(eqns.clone())?;
            crefDouble = findArraysPartiallyIndexed(&eqnLst)?;
            repl = BackendVarTransform::emptyReplacements();
            (m, _, _, _) = BackendDAEUtil::adjacencyMatrixScalar(metamodelica::AsArg::as_arg(&syst), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(metamodelica::AsArg::as_arg(&shared)))?;
            (eqnLst, _, movedvars_1, repl) = eliminateVariablesDAE2(&eqnLst, 1, metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&globalKnownVars), &(HashTable::emptyHashTable()), &repl, &crefDouble, m.clone(), elimVarIndexList, false)?;
            dae = setDaeEqns(metamodelica::AsArg::as_arg(&dae), BackendEquation::listEquation(&eqnLst)?, false)?;
            dae = replaceDAElow(metamodelica::AsArg::as_arg(&dae), repl, None, false)?;
            (vars_1, kvars_1) = moveVariables(&(BackendVariable::daeVars(metamodelica::AsArg::as_arg(&syst))), &(BackendVariable::daeGlobalKnownVars(metamodelica::AsArg::as_arg(&shared))), movedvars_1)?;
            dae = setDaeVars(metamodelica::AsArg::as_arg(&dae), vars_1)?;
            dae = BackendDAEUtil::setDAEGlobalKnownVars(metamodelica::AsArg::as_arg(&dae), kvars_1);
            dae = BackendDAEUtil::transformBackendDAE(metamodelica::AsArg::as_arg(&dae), Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED)), None, None)?;
            dae = BackendDAEUtil::mapEqSystem1(metamodelica::AsArg::as_arg(&dae), &BackendDAEUtil::getAdjacencyMatrixfromOptionForMapEqSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL)?;
            dae.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outDae)
}

fn findArraysPartiallyIndexed(
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut ht: (
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
    ht = findArraysPartiallyIndexed1(inEqs, HashTable::emptyHashTable())?;
    ht = findArraysPartiallyIndexedRecords(inEqs, ht)?;
    Ok(ht)
}

fn findArraysPartiallyIndexed1(
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHt: (
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
    outHt = 'mc: {
        let __mc_input = (&**inEqs, inht);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ht) => {
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { alg, .. }, tail: eqs }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysPartiallyIndexed1(metamodelica::AsArg::as_arg(&eqs), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. }, tail: eqs }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysPartiallyIndexed2(list![e1.clone(), e2.clone()], ht.clone(), HashTable::emptyHashTable())?;
                    ht = findArrayVariables(&(list![e1.clone(), e2.clone()]), ht.clone())?;
                    ht = findArraysPartiallyIndexed1(metamodelica::AsArg::as_arg(&eqs), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: eqs }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysPartiallyIndexed1(metamodelica::AsArg::as_arg(&eqs), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHt)
}

fn findArraysPartiallyIndexed2(
    mut inRef: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut indubRef: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inRef, indubRef, inht)) {
            (Deref @ metamodelica::ListNode::Nil, _, ht) => {
                return Ok(ht.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: c1, ty: _ }, tail: expl1 }, dubRef, ht) => {
                let mut c2: metamodelica::Ref<DAE::ComponentRef>;
                let mut dubRef = (*dubRef).clone();
                let mut ht = (*ht).clone();
                c2 = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&c1))?;
                if BaseHashTable::hasKey(c2.clone(), &(dubRef.clone()))? {
                    if BaseHashTable::hasKey(c2.clone(), &(ht.clone()))? {
                    } else {
                        ht = BaseHashTable::add((c2, 1), ht.clone())?;
                    }
                } else {
                    dubRef = BaseHashTable::add((c2, 1), dubRef.clone())?;
                }
                { (inRef, indubRef, inht) = (expl1.clone(), dubRef.clone(), ht.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: expl1 }, dubRef, ht) => {
                let mut ht = (*ht).clone();
                { (inRef, indubRef, inht) = (expl1.clone(), dubRef.clone(), ht.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn findArrayVariables(
    mut inRef: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHt: (
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
    outHt = 'mc: {
        let __mc_input = (&**inRef, inht);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ht) => {
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: c1, ty: _ }, tail: expl1 }, ht) => {
                    let mut ht = (*ht).clone();
                    let true = (Expression::isArrayType(&(ComponentReference::crefTypeConsiderSubs(metamodelica::AsArg::as_arg(&c1))?))) else { return Err("pattern mismatch") };
                    ht = BaseHashTable::add((c1.clone(), 1), ht.clone())?;
                    ht = findArrayVariables(metamodelica::AsArg::as_arg(&expl1), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: expl1 }, ht) => {
                    let mut ht = (*ht).clone();
                    ht = findArrayVariables(metamodelica::AsArg::as_arg(&expl1), ht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHt)
}

fn findArraysPartiallyIndexedRecords(
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHt: (
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
    (_, outHt) = BackendEquation::traverseExpsOfEquationList(
        inEqs,
        (std::sync::Arc::new(fnptr!(
            findArraysPartiallyIndexedRecordsExpVisitor,
            metamodelica::Ref<DAE::Exp>,
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>
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
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>
                )
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >,
                                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                            ),
                        ),
                    )> + 'static,
            >),
        ht,
    )?;
    Ok(outHt)
}

fn findArraysPartiallyIndexedRecordsExpVisitor(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut ht: (
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
    (e, ht) = 'mc: {
        let __mc_input = (inExp.clone(), inHt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, ht) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut ht = (*ht).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastType(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, complexClassType: ClassInf::State::RECORD { path: _ }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    ht = findArraysInRecordLst(ht.clone(), metamodelica::AsArg::as_arg(&cr), &varLst)?;
                    Ok((e.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (e, ht)
}

fn findArraysInRecordLst(
    mut inht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut recordCr: &metamodelica::Ref<DAE::ComponentRef>,
    mut invarLst: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHt: (
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
    outHt = 'mc: {
        let __mc_input = (inht, &**invarLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ht, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ht, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, ty: tp, .. }, tail: varLst }) => {
                    let mut thisCr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ht = (*ht).clone();
                    let true = (Expression::isArrayType(metamodelica::AsArg::as_arg(&tp))) else { return Err("pattern mismatch") };
                    thisCr = ComponentReference::joinCrefs(recordCr, metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: tp.clone(), subscriptLst: metamodelica::nil() }))?;
                    ht = BaseHashTable::add((thisCr.clone(), 0), ht.clone())?;
                    ht = findArraysInRecordLst(ht.clone(), recordCr, metamodelica::AsArg::as_arg(&varLst))?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ht, Deref @ metamodelica::ListNode::Cons { head: _, tail: varLst }) => {
                    let mut ht = (*ht).clone();
                    ht = findArraysInRecordLst(ht.clone(), recordCr, metamodelica::AsArg::as_arg(&varLst))?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHt)
}

fn eliminateVariablesDAE2(
    mut ieqns: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut eqnIndex: i32,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
    mut mvars: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut repl: &BackendVarTransform::VariableReplacements,
    mut inDoubles: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut elimVarIndexList: &metamodelica::List<i32>,
    mut failCheck: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    BackendVarTransform::VariableReplacements,
)> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outSimpleEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outMvars: (
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
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outEqns, outSimpleEqns, outMvars, outRepl) = 'mc: {
        let __mc_input = (&**ieqns, failCheck);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, false) => {
                    Ok((metamodelica::nil(), metamodelica::nil(), mvars.clone(), repl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: eqns }, false) => {
                    let mut mvars_1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut mvars_2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut repl_2: BackendVarTransform::VariableReplacements;
                    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut eqns_1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut seqns_1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut varIndexList: metamodelica::List<i32>;
                    let mut elimVarIndexList_1: metamodelica::List<i32>;
                    let mut elimVarIndex: i32;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut elimVar: metamodelica::Ref<BackendDAE::Var>;
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![e.clone()], repl, None)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    varIndexList = ({let __elt = (*metamodelica::index_checked(&m.borrow(), eqnIndex)?).clone(); __elt});
                    let __pa2 = ::match_deref::match_deref! { match &(List::intersectionOnTrue(&varIndexList, elimVarIndexList, &fnptr!(intEq, i32, i32))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: _ } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elimVarIndex = metamodelica::Own::own(__pa2);
                    elimVarIndexList_1 = List::removeOnTrue(elimVarIndex, &fnptr!(intEq, i32, i32), elimVarIndexList.clone())?;
                    elimVar = BackendVariable::getVarAt(vars, elimVarIndex)?;
                    let __arc4 = elimVar.clone();
                    let BackendDAE::VAR { varName: __pa3, .. } = &*__arc4;
                    cr1 = metamodelica::Own::own(__pa3);
                    (e2, source) = solveEqn2(metamodelica::AsArg::as_arg(&e), cr1.clone())?;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), cr1.clone(), e2.clone(), None)?;
                    mvars_1 = BaseHashTable::add((cr1.clone(), 0), mvars.clone())?;
                    (eqns_1, seqns_1, mvars_2, repl_2) = eliminateVariablesDAE2(metamodelica::AsArg::as_arg(&eqns), eqnIndex + 1, vars, globalKnownVars, &mvars_1, &repl_1, inDoubles, m.clone(), &elimVarIndexList_1, failCheck)?;
                    Ok((eqns_1.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1.clone(), exp: e2.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), seqns_1.clone()), mvars_2.clone(), repl_2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: eqns }, false) => {
                    let mut mvars_1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut eqns_1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut seqns_1: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (eqns_1, seqns_1, mvars_1, repl_1) = eliminateVariablesDAE2(metamodelica::AsArg::as_arg(&eqns), eqnIndex + 1, vars, globalKnownVars, mvars, repl, inDoubles, m.clone(), elimVarIndexList, false)?;
                    Ok((metamodelica::cons(e.clone(), eqns_1.clone()), seqns_1.clone(), mvars_1.clone(), repl_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEqns, outSimpleEqns, outMvars, outRepl))
}

fn solveEqn2(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    (exp, source) = (match &**eqn {
        BackendDAE::Equation::EQUATION {
            exp: e1,
            scalar: e2,
            source: __esc_source,
            ..
        } => {
            source = (*__esc_source).clone();
            (exp, _) = ExpressionSolve::solve(
                e1.clone(),
                e2.clone(),
                metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: cr,
                    ty: DAE::T_REAL_DEFAULT().clone(),
                }),
                None,
            )?;
            (exp, source.clone())
        }
        _ => return Err("fail"),
    });
    Ok((exp, source))
}

fn setDaeVars(
    mut systIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut newVarsIn: BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut sysOut: metamodelica::Ref<BackendDAE::BackendDAE>;
    sysOut = BackendDAEUtil::setVars(systIn, newVarsIn)?;
    Ok(sysOut)
}

fn setDaeEqns(
    mut dae: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut initEqs: bool,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut odae: metamodelica::Ref<BackendDAE::BackendDAE>;
    odae = (::match_deref::match_deref! { match &((&**dae, initEqs)) {
        (Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst, tail: systList }, shared }, false) => {
            let mut syst = (*syst).clone();
            syst = BackendDAEUtil::setEqSystEqs(syst.clone(), eqns);
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(syst.clone(), systList.clone()), shared: shared.clone() })
        },
        (Deref @ BackendDAE::BackendDAE { eqs: systList, shared }, false) => {
            let mut shared = (*shared).clone();
            shared = BackendDAEUtil::setSharedInitialEqns(shared.clone(), eqns);
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systList.clone(), shared: shared.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(odae)
}

pub(crate) fn replaceDAElow(
    mut idlow: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut func: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
    mut replaceVariables: bool,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    pub type PredicateFunction =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut odae: metamodelica::Ref<BackendDAE::BackendDAE>;
    odae = (::match_deref::match_deref! { match idlow {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst @ Deref @ BackendDAE::EqSystem { orderedVars, orderedEqs, .. }, tail: systList }, shared } => {
            let mut syst = (*syst).clone();
            let mut orderedVars = (*orderedVars).clone();
            let mut orderedEqs = (*orderedEqs).clone();
            orderedVars = BackendVariable::listVar1(&(replaceVars(&(BackendVariable::varList(metamodelica::AsArg::as_arg(&orderedVars))?), &repl, func, replaceVariables)?))?;
            (orderedEqs, _) = BackendVarTransform::replaceEquationsArr(orderedEqs.clone(), repl, None);
            syst = BackendDAEUtil::setEqSystVars(syst.clone(), orderedVars.clone());
            syst = BackendDAEUtil::setEqSystEqs(syst.clone(), orderedEqs.clone());
            metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(syst.clone(), systList.clone()), shared: shared.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(odae)
}

fn replaceVars(
    mut invarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut repl: &BackendVarTransform::VariableReplacements,
    mut func: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
    mut replaceName: bool,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    pub type PredicateFunction =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = (::match_deref::match_deref! { match &((invarLst.clone(), replaceName)) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: v, tail: varLst }, true) => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut bindExp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut v = (*v).clone();
            let mut varLst = (*varLst).clone();
            cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
            bindExp = varBindingOpt(metamodelica::AsArg::as_arg(&v));
            bindExp = replaceExpOpt(bindExp, repl, func.clone());
            bindExp = applyOptionSimplify(bindExp)?;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(&(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr, ty: DAE::T_REAL_DEFAULT().clone() })), repl, func.clone())) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            v = setVarCref(v.clone(), cr);
            v = setVarBindingOpt(metamodelica::AsArg::as_arg(&v), bindExp);
            varLst = replaceVars(metamodelica::AsArg::as_arg(&varLst), repl, func, replaceName)?;
            metamodelica::cons(v.clone(), varLst.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: v, tail: varLst }, false) => {
            let mut bindExp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut v = (*v).clone();
            let mut varLst = (*varLst).clone();
            bindExp = varBindingOpt(metamodelica::AsArg::as_arg(&v));
            bindExp = replaceExpOpt(bindExp, repl, func.clone());
            bindExp = applyOptionSimplify(bindExp)?;
            v = setVarBindingOpt(metamodelica::AsArg::as_arg(&v), bindExp);
            varLst = replaceVars(metamodelica::AsArg::as_arg(&varLst), repl, func, replaceName)?;
            metamodelica::cons(v.clone(), varLst.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVarLst)
}

pub(crate) fn varBindingOpt(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut exp: Option<metamodelica::Ref<DAE::Exp>>;
    exp = (match &**v {
        BackendDAE::Var { bindExp: __esc_exp, .. } => {
            exp = (*__esc_exp).clone();
            exp.clone()
        }
    });
    exp
}

pub(crate) fn replaceExpOpt(
    mut inExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut repl: &BackendVarTransform::VariableReplacements,
    mut funcOpt: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &(inExp) {
        None => {
            None
        },
        Some(e) => {
            let mut e = (*e).clone();
            (e, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), repl, funcOpt);
            Some(e.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub(crate) fn applyOptionSimplify(
    mut bindExpIn: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut bindExpOut: Option<metamodelica::Ref<DAE::Exp>>;
    bindExpOut = (::match_deref::match_deref! { match &(bindExpIn) {
        None => {
            None
        },
        Some(e) => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            (e1, _) = ExpressionSimplify::simplify1(e.clone())?;
            Some(e1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bindExpOut)
}

pub(crate) fn setVarCref(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.varName = cr, outVar.unreplaceable = false);
    outVar
}

pub(crate) fn setVarBindingOpt(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut bindExp: Option<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut kind: BackendDAE::VarKind;
    let mut dir: DAE::VarDirection;
    let mut prl: DAE::VarParallelism;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut bind: Option<metamodelica::Ref<DAE::Exp>>;
    let mut tplExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut ad: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut ts: Option<BackendDAE::TearingSelect>;
    let mut hideResult: Option<metamodelica::Ref<DAE::Exp>>;
    let mut cmt: Option<metamodelica::Ref<SCode::Comment>>;
    let mut ct: metamodelica::Ref<DAE::ConnectorType>;
    let mut innerOuter: DAE::VarInnerOuter;
    let mut encrypted: bool;
    let __arc16 = &(*inVar);
    let BackendDAE::VAR {
        varName: __pa0,
        varKind: __pa1,
        varDirection: __pa2,
        varParallelism: __pa3,
        varType: __pa4,
        bindExp: __pa5,
        tplExp: __pa6,
        arryDim: __pa7,
        source: __pa8,
        values: __pa9,
        tearingSelectOption: __pa10,
        hideResult: __pa11,
        comment: __pa12,
        connectorType: __pa13,
        innerOuter: __pa14,
        unreplaceable: _,
        initNonlinear: _,
        encrypted: __pa15,
    } = &**__arc16;
    name = metamodelica::Own::own(__pa0);
    kind = metamodelica::Own::own(__pa1);
    dir = metamodelica::Own::own(__pa2);
    prl = metamodelica::Own::own(__pa3);
    tp = metamodelica::Own::own(__pa4);
    bind = metamodelica::Own::own(__pa5);
    tplExp = metamodelica::Own::own(__pa6);
    ad = metamodelica::Own::own(__pa7);
    source = metamodelica::Own::own(__pa8);
    attr = metamodelica::Own::own(__pa9);
    ts = metamodelica::Own::own(__pa10);
    hideResult = metamodelica::Own::own(__pa11);
    cmt = metamodelica::Own::own(__pa12);
    ct = metamodelica::Own::own(__pa13);
    innerOuter = metamodelica::Own::own(__pa14);
    encrypted = metamodelica::Own::own(__pa15);
    outVar = metamodelica::Ref::new(BackendDAE::Var {
        varName: name,
        varKind: kind,
        varDirection: dir,
        varParallelism: prl,
        varType: tp,
        bindExp: bindExp,
        tplExp: tplExp,
        arryDim: ad,
        source: source,
        values: attr,
        tearingSelectOption: ts,
        hideResult: hideResult,
        comment: cmt,
        connectorType: ct,
        innerOuter: innerOuter,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: encrypted,
    });
    outVar
}

pub(crate) fn moveVariables(
    mut inVariables1: &BackendDAE::Variables,
    mut inVariables2: &BackendDAE::Variables,
    mut hashTable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(BackendDAE::Variables, BackendDAE::Variables)> {
    let mut outVariables1: BackendDAE::Variables;
    let mut outVariables2: BackendDAE::Variables;
    let mut lst1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut lst2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut lst1_1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut lst2_1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v1: BackendDAE::Variables;
    let mut v2: BackendDAE::Variables;
    lst1 = BackendVariable::varList(inVariables1)?;
    lst2 = BackendVariable::varList(inVariables2)?;
    (lst1_1, lst2_1) = moveVariables2(&lst1, lst2, hashTable)?;
    v1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    v2 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    outVariables1 = BackendVariable::addVars(&lst1_1, v1)?;
    outVariables2 = BackendVariable::addVars(&lst2_1, v2)?;
    Ok((outVariables1, outVariables2))
}

fn moveVariables2(
    mut inVarLst1: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVarLst2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut hashTable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outVarLst1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outVarLst2: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (outVarLst1, outVarLst2) = (::match_deref::match_deref! { match inVarLst1 {
        Deref @ metamodelica::ListNode::Nil => {
            let mut globalKnownVars = inVarLst2;
            (metamodelica::nil(), globalKnownVars)
        },
        Deref @ metamodelica::ListNode::Cons { head: v @ Deref @ BackendDAE::Var { varName: cr, .. }, tail: vs } => {
            let mut globalKnownVars = inVarLst2;
            let mut mvars = hashTable;
            let mut vs_1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut knvars_1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            if BaseHashTable::hasKey(cr.clone(), &mvars)? {
                (vs_1, knvars_1) = moveVariables2(vs, globalKnownVars, mvars)?;
                knvars_1 = metamodelica::cons(v.clone(), knvars_1);
            } else {
                (vs_1, knvars_1) = moveVariables2(vs, globalKnownVars, mvars)?;
                vs_1 = metamodelica::cons(v.clone(), vs_1);
            }
            (vs_1, knvars_1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVarLst1, outVarLst2))
}

pub(crate) fn sortBy1<
    ElementType: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inList: &metamodelica::List<ElementType>,
    mut inCompFunc: &dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32>,
    mut inArgument1: ArgType1,
) -> Result<metamodelica::List<ElementType>> {
    pub type CompareFunc<ElementType: Clone + 'static, ArgType1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32> + 'static>;

    let mut outList: metamodelica::List<ElementType>;
    outList = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil } => {
            list![e.clone()]
        },
        _ => {
            let mut left: metamodelica::List<ElementType>;
            let mut right: metamodelica::List<ElementType>;
            let mut middle: i32;
            middle = intDiv(((inList).len() as i32), 2);
            (left, right) = List::split(inList.clone(), middle)?;
            left = sortBy1(&left, inCompFunc, inArgument1.clone())?;
            right = sortBy1(&right, inCompFunc, inArgument1.clone())?;
            mergeBy1(&left, &right, inCompFunc, inArgument1)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outList)
}

fn mergeBy1<
    ElementType: Clone + 'static + metamodelica::gc::MMTrace,
    ArgType1: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inLeft: &metamodelica::List<ElementType>,
    mut inRight: &metamodelica::List<ElementType>,
    mut inCompFunc: &dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32>,
    mut inArgument1: ArgType1,
) -> Result<metamodelica::List<ElementType>> {
    pub type CompareFunc<ElementType: Clone + 'static, ArgType1: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(ElementType, ArgType1) -> Result<i32> + 'static>;

    let mut outList: metamodelica::List<ElementType>;
    outList = 'mc: {
        let __mc_input = (&**inLeft, &**inRight);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: l, tail: l_rest }, Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }) => {
                    let mut res: metamodelica::List<ElementType>;
                    let mut ri: i32;
                    let mut li: i32;
                    ri = inCompFunc(r.clone(), inArgument1.clone())?;
                    li = inCompFunc(l.clone(), inArgument1.clone())?;
                    let true = (intGt(ri, li)) else { return Err("pattern mismatch") };
                    res = mergeBy1(metamodelica::AsArg::as_arg(&l_rest), inRight, inCompFunc, inArgument1.clone())?;
                    Ok(metamodelica::cons(l.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: r, tail: r_rest }) => {
                    let mut res: metamodelica::List<ElementType>;
                    res = mergeBy1(inLeft, metamodelica::AsArg::as_arg(&r_rest), inCompFunc, inArgument1.clone())?;
                    Ok(metamodelica::cons(r.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inRight.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inLeft.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outList)
}

fn removeSimpleEquationsUC(
    mut daeIn: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut daeOut: metamodelica::Ref<BackendDAE::BackendDAE>;
    daeOut = (::match_deref::match_deref! { match &(daeIn) {
        dae @ Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::EqSystem { orderedEqs: eqns, orderedVars: vars, .. }, tail: _ }, shared: Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut sets: metamodelica::List<AliasSet>;
            let mut other_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut simple_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut set_solutions: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut removed_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut removed_vars_table: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
            let mut dae = (*dae).clone();
            let mut vars = (*vars).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            removed_vars_table = HashTable::emptyHashTable();
            (sets, other_eqns) = separateAliasSetsAndEquations(&(BackendEquation::equationList(eqns.clone())?), metamodelica::nil(), metamodelica::nil())?;
            set_solutions = List::map2(sets.clone(), &move |__a0: AliasSet, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| solveAliasSet(&__a0, __a1, __a2), vars.clone(), globalKnownVars.clone())?;
            (repl, simple_eqns, removed_vars) = createReplacementsAndEquations(&set_solutions, &sets, metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&globalKnownVars), repl, metamodelica::nil(), metamodelica::nil())?;
            (other_eqns, _) = BackendVarTransform::replaceEquations(other_eqns, &repl, None)?;
            removed_vars_table = addCrefsToHashTable(&removed_vars, removed_vars_table)?;
            (vars, globalKnownVars) = moveVariables(metamodelica::AsArg::as_arg(&vars), metamodelica::AsArg::as_arg(&globalKnownVars), removed_vars_table)?;
            dae = setDaeVars(metamodelica::AsArg::as_arg(&dae), vars.clone())?;
            dae = BackendDAEUtil::setDAEGlobalKnownVars(metamodelica::AsArg::as_arg(&dae), globalKnownVars.clone());
            dae = setDaeEqns(metamodelica::AsArg::as_arg(&dae), BackendEquation::listEquation(&(listAppend(simple_eqns, other_eqns)))?, false)?;
            dae = BackendDAEUtil::transformBackendDAE(metamodelica::AsArg::as_arg(&dae), Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::ALLOW_UNDERCONSTRAINED)), None, None)?;
            dae = BackendDAEUtil::mapEqSystem1(metamodelica::AsArg::as_arg(&dae), &BackendDAEUtil::getAdjacencyMatrixfromOptionForMapEqSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL)?;
            dae.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(daeOut)
}

fn addCrefsToHashTable<'__b>(
    mut crefs: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut table: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
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
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match crefs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(table)
            },
            Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                let mut new_table: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                new_table = BaseHashTable::add((h.clone(), 0), table)?;
                { (crefs, table) = (t, new_table); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getAllVariablesForCref(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = globalKnownVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut out: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            (out, _) = BackendVariable::getVar(cr.clone(), vars)?;
            Ok(out.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut out: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            (out, _) = BackendVariable::getVar(cr.clone(), globalKnownVars)?;
            Ok(out.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarLst)
}

fn rateVariable(mut var: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Real> {
    let mut out: metamodelica::Real;
    let mut acc: metamodelica::Real;
    let mut i: metamodelica::Real;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    acc = metamodelica::OrderedFloat(0.0_f64);
    let __arc1 = &(*var);
    let BackendDAE::VAR { varName: __pa0, .. } = &**__arc1;
    cr = metamodelica::Own::own(__pa0);
    i = metamodelica::real_div_checked(
        metamodelica::OrderedFloat(1.0_f64),
        (metamodelica::OrderedFloat(1.0_f64) + intReal(ComponentReference::crefDepth(&cr)?)),
    )?;
    acc = acc + i;
    i = if (BackendVariable::isParam(var)) {
        metamodelica::OrderedFloat(3.0_f64)
    } else {
        metamodelica::OrderedFloat(0.0_f64)
    };
    acc = acc + i;
    i = if (BackendVariable::isStateVar(var)) {
        metamodelica::OrderedFloat(5.0_f64)
    } else {
        metamodelica::OrderedFloat(0.0_f64)
    };
    acc = acc + i;
    i = if (BackendVariable::varHasUncertainValueRefine(var)) {
        metamodelica::OrderedFloat(7.0_f64)
    } else {
        metamodelica::OrderedFloat(0.0_f64)
    };
    acc = acc + i;
    out = acc;
    Ok(out)
}

fn rateVariableList(mut vars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> Result<metamodelica::Real> {
    let mut out: metamodelica::Real;
    out = (::match_deref::match_deref! { match vars {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::OrderedFloat(0.0_f64)
        },
        Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
            let mut r1: metamodelica::Real;
            let mut r2: metamodelica::Real;
            let mut r: metamodelica::Real;
            r1 = rateVariable(metamodelica::AsArg::as_arg(&h))?;
            r2 = rateVariableList(t)?;
            r = if (realGt(r1, r2)) {r1} else {r2};
            r
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

fn rateSetElement(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)> {
    let mut out: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real);
    let mut var: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    var = getAllVariablesForCref(cr.clone(), vars, globalKnownVars)?;
    out = (cr, rateVariableList(&var)?);
    Ok(out)
}

fn setPairSortFunction(
    mut a: &(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real),
    mut b: &(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real),
) -> bool {
    let mut out: bool;
    let mut av: metamodelica::Real;
    let mut bv: metamodelica::Real;
    (_, av) = a.clone();
    (_, bv) = b.clone();
    out = realLt(av, bv);
    out
}

fn solveAliasSet(
    mut set: &AliasSet,
    mut vars: BackendDAE::Variables,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut out: metamodelica::Ref<DAE::ComponentRef>;
    let mut names: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut name_rate_list: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)>;
    names = getAliasSetSymbolList(set)?;
    name_rate_list = List::map2(
        names,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: BackendDAE::Variables, __a2: BackendDAE::Variables| {
            rateSetElement(__a0, &__a1, &__a2)
        },
        vars,
        globalKnownVars,
    )?;
    name_rate_list = List::sort(
        name_rate_list,
        (std::sync::Arc::new(
            move |__a0: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real),
                  __a1: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real)|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(setPairSortFunction(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real),
                        (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Real),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    let __pa0 = ::match_deref::match_deref! { match &(name_rate_list) {
        Deref @ metamodelica::ListNode::Cons { head: (__pa0, _), tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    out = metamodelica::Own::own(__pa0);
    Ok(out)
}

fn isRemovableVar(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut out: bool;
    out = !(BackendVariable::isStateVar(var)) && !(BackendVariable::varHasUncertainValueRefine(var));
    out
}

fn isRemovableVarList(mut vars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match vars {
        Deref @ metamodelica::ListNode::Nil => {
            true
        },
        Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
            let mut r1: bool;
            let mut r2: bool;
            let mut r: bool;
            r1 = isRemovableVar(metamodelica::AsArg::as_arg(&h));
            r2 = isRemovableVarList(t);
            r = r1 && r2;
            r
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn isRemovableSymbol(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
) -> Result<bool> {
    let mut out: bool;
    let mut var: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    var = getAllVariablesForCref(cr, vars, globalKnownVars)?;
    out = isRemovableVarList(&var);
    Ok(out)
}

fn fixSingOfExp(mut sign: i32, mut eIn: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut out: metamodelica::Ref<DAE::Exp>;
    out = (match sign {
        (-1) => {
            let mut tp: metamodelica::Ref<DAE::Type>;
            tp = Expression::r#typeof(eIn.clone())?;
            metamodelica::Ref::new(DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS { ty: tp },
                exp: eIn,
            })
        }
        _ => eIn,
    });
    Ok(out)
}

fn generateEquation(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<BackendDAE::Equation> {
    let mut out: metamodelica::Ref<BackendDAE::Equation>;
    out = metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION {
        componentRef: cr,
        exp: e,
        source: source,
        attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
    });
    out
}

fn createReplacementsAndEquationsForSet(
    mut solution: &metamodelica::Ref<DAE::ComponentRef>,
    mut symbols: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut set: &AliasSet,
    mut vars: &BackendDAE::Variables,
    mut globalKnownVars: &BackendDAE::Variables,
    mut repl_acc: &BackendVarTransform::VariableReplacements,
    mut eqns_acc: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut removed_vars_acc: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    BackendVarTransform::VariableReplacements,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut eqnsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut removed_varsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (replOut, eqnsOut, removed_varsOut) = 'mc: {
        let __mc_input = &**symbols;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((repl_acc.clone(), eqns_acc.clone(), removed_vars_acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut new_removed_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut new_repl: BackendVarTransform::VariableReplacements;
                    let mut new_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let true = (ComponentReferenceBasics::crefEqual(solution, metamodelica::AsArg::as_arg(&h))?) else { return Err("pattern mismatch") };
                    (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution, metamodelica::AsArg::as_arg(&t), set, vars, globalKnownVars, repl_acc, eqns_acc, removed_vars_acc)?;
                    Ok((new_repl.clone(), new_eqns.clone(), new_removed_vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut new_removed_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut sign1: i32;
                    let mut sign2: i32;
                    let mut sign: i32;
                    let mut new_repl: BackendVarTransform::VariableReplacements;
                    let mut new_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let true = (isRemovableSymbol(h.clone(), vars, globalKnownVars)?) else { return Err("pattern mismatch") };
                    (sign1, e) = getAliasSetExpressionAndSign(solution.clone(), set)?;
                    (sign2, _) = getAliasSetExpressionAndSign(h.clone(), set)?;
                    sign = if (sign2 < 0) {-(sign1)} else {sign1};
                    e = fixSingOfExp(sign, e.clone())?;
                    new_repl = BackendVarTransform::addReplacement(repl_acc.clone(), h.clone(), e.clone(), None)?;
                    new_removed_vars = metamodelica::cons(h.clone(), removed_vars_acc.clone());
                    (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution, metamodelica::AsArg::as_arg(&t), set, vars, globalKnownVars, &new_repl, eqns_acc, &new_removed_vars)?;
                    Ok((new_repl.clone(), new_eqns.clone(), new_removed_vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut new_removed_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut sign1: i32;
                    let mut sign2: i32;
                    let mut sign: i32;
                    let mut new_repl: BackendVarTransform::VariableReplacements;
                    let mut new_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let false = (isRemovableSymbol(h.clone(), vars, globalKnownVars)?) else { return Err("pattern mismatch") };
                    (sign1, e) = getAliasSetExpressionAndSign(solution.clone(), set)?;
                    (sign2, _) = getAliasSetExpressionAndSign(h.clone(), set)?;
                    sign = if (sign2 < 0) {-(sign1)} else {sign1};
                    e = fixSingOfExp(sign, e.clone())?;
                    source = getAliasSetSource(set)?;
                    eqn = generateEquation(h.clone(), e.clone(), source.clone());
                    new_eqns = metamodelica::cons(eqn.clone(), eqns_acc.clone());
                    (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(solution, metamodelica::AsArg::as_arg(&t), set, vars, globalKnownVars, repl_acc, &new_eqns, removed_vars_acc)?;
                    Ok((new_repl.clone(), new_eqns.clone(), new_removed_vars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((replOut, eqnsOut, removed_varsOut))
}

fn createReplacementsAndEquations<'__b>(
    mut solutions: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut sets: &'__b metamodelica::List<AliasSet>,
    mut vars: &'__b BackendDAE::Variables,
    mut globalKnownVars: &'__b BackendDAE::Variables,
    mut repl_acc: BackendVarTransform::VariableReplacements,
    mut eqns_acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut removed_vars_acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    BackendVarTransform::VariableReplacements,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (solutions, sets) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((repl_acc, eqns_acc, removed_vars_acc))
            },
            (Deref @ metamodelica::ListNode::Cons { head: solution, tail: solt }, Deref @ metamodelica::ListNode::Cons { head: set, tail: sett }) => {
                let mut symbols: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut new_removed_vars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut new_repl: BackendVarTransform::VariableReplacements;
                let mut new_eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                symbols = getAliasSetSymbolList(metamodelica::AsArg::as_arg(&set))?;
                (new_repl, new_eqns, new_removed_vars) = createReplacementsAndEquationsForSet(metamodelica::AsArg::as_arg(&solution), &symbols, metamodelica::AsArg::as_arg(&set), vars, globalKnownVars, &repl_acc, &eqns_acc, &removed_vars_acc)?;
                { (solutions, sets, vars, globalKnownVars, repl_acc, eqns_acc, removed_vars_acc) = (solt, sett, vars, globalKnownVars, new_repl, new_eqns, new_removed_vars); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn separateAliasSetsAndEquations<'__b>(
    mut eqnIn: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut sets: metamodelica::List<AliasSet>,
    mut eqn_accIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<AliasSet>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match eqnIn {
            Deref @ metamodelica::ListNode::Nil => {
                let mut eqn_acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                eqn_acc = eqn_accIn.reverse();
                return Ok((sets, eqn_acc))
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. }, tail: t } => {
                let mut eqn_acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut new_sets: metamodelica::List<AliasSet>;
                (new_sets, eqn_acc) = addPairToSet(sets, eqn_accIn, eqn.clone(), e1.clone(), e2.clone())?;
                { (eqnIn, sets, eqn_accIn) = (t, new_sets, eqn_acc); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn @ Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. }, tail: t } => {
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut eqn_acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut new_sets: metamodelica::List<AliasSet>;
                e1 = Expression::crefExp(cr.clone())?;
                (new_sets, eqn_acc) = addPairToSet(sets, eqn_accIn, eqn.clone(), e1, e2.clone())?;
                { (eqnIn, sets, eqn_accIn) = (t, new_sets, eqn_acc); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn, tail: t } => {
                let mut eqn_acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut new_sets: metamodelica::List<AliasSet>;
                { (eqnIn, sets, eqn_accIn) = (t, sets, metamodelica::cons(eqn.clone(), eqn_accIn)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addPairToSet(
    mut sets: metamodelica::List<AliasSet>,
    mut eqn_acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut eqn: metamodelica::Ref<BackendDAE::Equation>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::List<AliasSet>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut out: metamodelica::List<AliasSet>;
    let mut eqn_acc_out: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (out, eqn_acc_out) = (::match_deref::match_deref! { match &((lhs, rhs)) {
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            let mut new_sets: metamodelica::List<AliasSet>;
            let mut source: Option<metamodelica::Ref<DAE::ElementSource>>;
            source = getSourceIfApproximated(&eqn)?;
            new_sets = pushToSetList(&sets, metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&e1), 1, metamodelica::AsArg::as_arg(&cr2), metamodelica::AsArg::as_arg(&e2), 1, source)?;
            (new_sets, eqn_acc)
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            let mut new_sets: metamodelica::List<AliasSet>;
            let mut source: Option<metamodelica::Ref<DAE::ElementSource>>;
            source = getSourceIfApproximated(&eqn)?;
            new_sets = pushToSetList(&sets, metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&e1), 1, metamodelica::AsArg::as_arg(&cr2), metamodelica::AsArg::as_arg(&e2), -1, source)?;
            (new_sets, eqn_acc)
        },
        (e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
            let mut new_sets: metamodelica::List<AliasSet>;
            let mut source: Option<metamodelica::Ref<DAE::ElementSource>>;
            source = getSourceIfApproximated(&eqn)?;
            new_sets = pushToSetList(&sets, metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&e1), -1, metamodelica::AsArg::as_arg(&cr2), metamodelica::AsArg::as_arg(&e2), 1, source)?;
            (new_sets, eqn_acc)
        },
        (e1 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. } }, e2 @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr2, .. } }) => {
            let mut new_sets: metamodelica::List<AliasSet>;
            let mut source: Option<metamodelica::Ref<DAE::ElementSource>>;
            source = getSourceIfApproximated(&eqn)?;
            new_sets = pushToSetList(&sets, metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&e1), -1, metamodelica::AsArg::as_arg(&cr2), metamodelica::AsArg::as_arg(&e2), -1, source)?;
            (new_sets, eqn_acc)
        },
        _ => {
            (sets, metamodelica::cons(eqn, eqn_acc))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out, eqn_acc_out))
}

fn getSourceIfApproximated(
    mut eqn: &metamodelica::Ref<BackendDAE::Equation>,
) -> Result<Option<metamodelica::Ref<DAE::ElementSource>>> {
    let mut source: Option<metamodelica::Ref<DAE::ElementSource>>;
    let mut temp: metamodelica::Ref<DAE::ElementSource>;
    temp = BackendEquation::equationSource(eqn)?;
    source = if (isApproximatedEquation(eqn)) {
        Some(temp)
    } else {
        None
    };
    Ok(source)
}

/*     Set handling functions    */
fn createSet(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut sign1In: i32,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut sign2In: i32,
    mut source: Option<metamodelica::Ref<DAE::ElementSource>>,
) -> Result<AliasSet> {
    let mut setOut: AliasSet;
    setOut = (match (sign1In, sign2In) {
        (mut sign1, mut sign2) => {
            let mut new_symbols: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
                i32,
                (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
            );
            let mut new_signs: (
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
            let mut new_expl: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTable2::FuncHashCref,
                    HashTable2::FuncCrefEqual,
                    HashTable2::FuncCrefStr,
                    HashTable2::FuncExpStr,
                ),
            );
            new_signs = HashTable::emptyHashTable();
            new_symbols = HashSet::emptyHashSet();
            new_expl = HashTable2::emptyHashTable();
            new_signs = BaseHashTable::add((cr1.clone(), sign1), new_signs)?;
            new_signs = BaseHashTable::add((cr2.clone(), sign2), new_signs)?;
            new_symbols = BaseHashSet::add(cr1.clone(), &new_symbols)?;
            new_symbols = BaseHashSet::add(cr2.clone(), &new_symbols)?;
            new_expl = BaseHashTable::add((cr1, e1), new_expl)?;
            new_expl = BaseHashTable::add((cr2, e2), new_expl)?;
            AliasSet {
                symbols: new_symbols,
                expl: new_expl,
                signs: new_signs,
                source: source,
            }
        }
    });
    Ok(setOut)
}

fn addToSet(
    mut set: &AliasSet,
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut e1: &metamodelica::Ref<DAE::Exp>,
    mut sign1In: i32,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut sign2In: i32,
    mut sourceIn: Option<metamodelica::Ref<DAE::ElementSource>>,
) -> Result<AliasSet> {
    let mut setOut: AliasSet;
    setOut = (match (set.clone(), sign1In, sign2In) {
        (
            AliasSet {
                symbols: mut symbols,
                expl: mut expl,
                signs: mut signs,
                source: mut source_current,
            },
            mut sign1,
            mut sign2,
        ) => {
            let mut current_sign: i32;
            let mut sign1_temp: i32;
            let mut new_symbols: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                ),
                i32,
                i32,
                (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
            );
            let mut new_signs: (
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
            let mut new_expl: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTable2::FuncHashCref,
                    HashTable2::FuncCrefEqual,
                    HashTable2::FuncCrefStr,
                    HashTable2::FuncExpStr,
                ),
            );
            let mut source_new: Option<metamodelica::Ref<DAE::ElementSource>>;
            current_sign = BaseHashTable::get(cr1, &(signs.clone()))?;
            sign1_temp = sign1;
            sign1 = if (intEq(sign1_temp, current_sign)) {
                sign1
            } else {
                -(sign1)
            };
            sign2 = if (intEq(sign1_temp, current_sign)) {
                sign2
            } else {
                -(sign2)
            };
            new_signs = BaseHashTable::add((cr2.clone(), sign2), signs.clone())?;
            new_symbols = BaseHashSet::add(cr2.clone(), &(symbols.clone()))?;
            new_expl = BaseHashTable::add((cr2, e2), expl.clone())?;
            source_new = updateSource(source_current.clone(), sourceIn)?;
            AliasSet {
                symbols: new_symbols,
                expl: new_expl,
                signs: new_signs,
                source: source_new,
            }
        }
    });
    Ok(setOut)
}

fn updateSource(
    mut source1: Option<metamodelica::Ref<DAE::ElementSource>>,
    mut source2: Option<metamodelica::Ref<DAE::ElementSource>>,
) -> Result<Option<metamodelica::Ref<DAE::ElementSource>>> {
    let mut sourceOut: Option<metamodelica::Ref<DAE::ElementSource>>;
    sourceOut = (::match_deref::match_deref! { match &((source1, source2)) {
        (None, None) => {
            None
        },
        (Some(s), None) => {
            Some(s.clone())
        },
        (None, Some(s)) => {
            Some(s.clone())
        },
        (Some(s), Some(_)) => {
            Some(s.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(sourceOut)
}

fn existsInSet(mut set: &AliasSet, mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut out: bool;
    out = (match set.clone() {
        AliasSet {
            symbols: mut symbols,
            expl: _,
            signs: _,
            source: _,
        } => {
            let mut ret: bool;
            ret = BaseHashSet::has(cr, &(symbols.clone()))?;
            ret
        }
    });
    Ok(out)
}

fn pushToSetList(
    mut sets: &metamodelica::List<AliasSet>,
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut e1: &metamodelica::Ref<DAE::Exp>,
    mut sign1: i32,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
    mut e2: &metamodelica::Ref<DAE::Exp>,
    mut sign2: i32,
    mut source: Option<metamodelica::Ref<DAE::ElementSource>>,
) -> Result<metamodelica::List<AliasSet>> {
    let mut setsOut: metamodelica::List<AliasSet>;
    setsOut = 'mc: {
        let __mc_input = &**sets;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut new_set: AliasSet;
                    new_set = createSet(cr1.clone(), e1.clone(), sign1, cr2.clone(), e2.clone(), sign2, source.clone())?;
                    Ok(list![new_set.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut new_set: AliasSet;
                    let true = (existsInSet(metamodelica::AsArg::as_arg(&h), cr1.clone())?) else { return Err("pattern mismatch") };
                    new_set = addToSet(metamodelica::AsArg::as_arg(&h), cr1.clone(), e1, sign1, cr2.clone(), e2.clone(), sign2, source.clone())?;
                    Ok(metamodelica::cons(new_set.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut new_set: AliasSet;
                    let true = (existsInSet(metamodelica::AsArg::as_arg(&h), cr2.clone())?) else { return Err("pattern mismatch") };
                    new_set = addToSet(metamodelica::AsArg::as_arg(&h), cr2.clone(), e2, sign2, cr1.clone(), e1.clone(), sign1, source.clone())?;
                    Ok(metamodelica::cons(new_set.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: h, tail: t } => {
                    let mut inner_sets: metamodelica::List<AliasSet>;
                    inner_sets = pushToSetList(metamodelica::AsArg::as_arg(&t), cr1, e1, sign1, cr2, e2, sign2, source.clone())?;
                    Ok(metamodelica::cons(h.clone(), inner_sets.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(setsOut)
}

fn getAliasSetSymbolList(mut set: &AliasSet) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut out: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    out = (match set.clone() {
        AliasSet {
            symbols: mut symbols,
            expl: _,
            signs: _,
            source: _,
        } => {
            let mut crl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crl = BaseHashSet::hashSetList(&(symbols.clone()))?;
            crl
        }
    });
    Ok(out)
}

fn getAliasSetSource(mut set: &AliasSet) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut out: metamodelica::Ref<DAE::ElementSource>;
    out = (::match_deref::match_deref! { match &(set) {
        AliasSet { symbols: _, expl: _, signs: _, source: Some(source) } => {
            source.clone()
        },
        AliasSet { symbols: _, expl: _, signs: _, source: None } => {
            DAE::emptyElementSource().clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out)
}

fn getAliasSetExpressionAndSign(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut set: &AliasSet,
) -> Result<(i32, metamodelica::Ref<DAE::Exp>)> {
    let mut signOut: i32;
    let mut eOut: metamodelica::Ref<DAE::Exp>;
    (signOut, eOut) = (match set.clone() {
        AliasSet {
            symbols: _,
            expl: mut expl,
            signs: mut signs,
            source: _,
        } => {
            let mut sign: i32;
            let mut e: metamodelica::Ref<DAE::Exp>;
            sign = BaseHashTable::get(cr.clone(), &(signs.clone()))?;
            e = BaseHashTable::get(cr, &(expl.clone()))?;
            (sign, e)
        }
    });
    Ok((signOut, eOut))
}

fn dumpAliasSets(mut sets: &metamodelica::List<AliasSet>) -> Result<()> {
    let () = (::match_deref::match_deref! { match sets {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: AliasSet { symbols, expl: _, signs, source }, tail: t } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut sign_values: metamodelica::List<i32>;
            crefs = BaseHashSet::hashSetList(&(symbols.clone()))?;
            sign_values = List::map1(crefs.clone(), &move |__a0: _, __a1: _| BaseHashTable::get(__a0, &__a1), signs.clone())?;
            dumpAliasSets2(&crefs, &sign_values)?;
            dumpAliasSets3(source.clone())?;
            metamodelica::print(literal!("\n"));
            dumpAliasSets(t)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpAliasSets2(
    mut crefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut sign_values: &metamodelica::List<i32>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (crefs, sign_values) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: cr, tail: cr_t }, Deref @ metamodelica::ListNode::Cons { head: i, tail: i_t }) => {
            let mut s: ArcStr;
            s = if (i.clone() > 0) {literal!("+")} else {literal!("-")};
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) });
            dumpAliasSets2(cr_t, i_t)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn dumpAliasSets3(mut sourceIn: Option<metamodelica::Ref<DAE::ElementSource>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(sourceIn) {
        None => {
            metamodelica::print(literal!(" *Approximated = false"));
            ()
        },
        Some(Deref @ DAE::ElementSource { comment, .. }) => {
            let mut r#str: ArcStr;
            r#str = boolString(isApproximatedEquation2(metamodelica::AsArg::as_arg(&comment)));
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" *Approximated = ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) });
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
