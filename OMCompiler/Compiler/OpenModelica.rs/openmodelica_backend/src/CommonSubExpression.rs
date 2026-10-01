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

use crate::AvlSetInt;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::ResolveLoops;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTableExpToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CSE_Equation {
    /// lhs
    pub cse: metamodelica::Ref<DAE::Exp>,
    /// rhs
    pub call: metamodelica::Ref<DAE::Exp>,
    pub dependencies: metamodelica::List<i32>,
}

impl metamodelica::gc::MMTrace for CSE_Equation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.cse, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.call, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dependencies, __mmv)?;
        Ok(())
    }
}
impl Default for CSE_Equation {
    fn default() -> Self {
        Self {
            cse: Default::default(),
            call: Default::default(),
            dependencies: Default::default(),
        }
    }
}

pub type CSE_EQUATION = CSE_Equation;

thread_local! { static __dummy_equation_TLS: CSE_Equation = CSE_Equation { cse: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), call: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), dependencies: metamodelica::nil() }; }
pub(crate) fn dummy_equation() -> CSE_Equation {
    __dummy_equation_TLS.with(|__t| __t.clone())
}

pub(crate) const debug: bool = false;

pub(crate) const BORDER: &'static str = "###############################################################";

pub(crate) const UNDERLINE: &'static str = "========================================";

fn printCSEEquation(mut cseEquation: &CSE_Equation) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut first: bool = true;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ExpressionBasics::printExpStr(cseEquation.cse.clone())?);
        __mm_s.push_str(&*literal!(" - "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(cseEquation.call.clone())?);
        __mm_s.push_str(&*literal!(" - {"));
        ArcStr::from(__mm_s)
    };
    for mut i in &*cseEquation.dependencies.clone() {
        if first {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*intString(i.clone()));
                ArcStr::from(__mm_s)
            };
            first = false;
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*intString(i.clone()));
                ArcStr::from(__mm_s)
            };
        }
    }
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn wrapFunctionCalls(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut size: i32;
    let mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cseIndex: i32 = System::tmpTickIndex(Global::backendDAE_cseIndex.clone());
    let mut index: i32;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut orderedEqs_new: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut orderedVars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut eqSystems: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut daeTypeStr: ArcStr = BackendDump::printBackendDAEType2String(inDAE.shared.backendDAEType.clone())?;
    let mut isSimulationDAE: bool = stringEq(&daeTypeStr, &(literal!("simulation")));
    let mut allowGlobalKnown: bool = !(stringEq(&daeTypeStr, &(literal!("jacobian"))));
    let mut globalKnownVarHT: (
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
    size = BackendDAEUtil::maxSizeOfEqSystems(&inDAE.eqs)? + 42;
    exarray = ExpandableArray::new(size, dummy_equation().clone());
    size = Util::nextPrime(
        ((metamodelica::OrderedFloat(2.4_f64) * metamodelica::OrderedFloat((size) as f64))
            .0
            .floor() as i32),
    );
    HT = HashTableExpToIndex::emptyHashTableSized(size);
    shared = inDAE.shared.clone();
    let __arc2 = shared.clone();
    let BackendDAE::SHARED {
        globalKnownVars: __pa0,
        functionTree: __pa1,
        ..
    } = &*__arc2;
    globalKnownVars = metamodelica::Own::own(__pa0);
    functionTree = metamodelica::Own::own(__pa1);
    globalKnownVarHT = HashSet::emptyHashSetSized(Util::nextPrime(
        ((metamodelica::OrderedFloat(2.4_f64)
            * (metamodelica::OrderedFloat((globalKnownVars.numberOfVars.clone() + 42) as f64)))
        .0
        .floor() as i32),
    ));
    if isSimulationDAE {
        globalKnownVarHT = BackendVariable::traverseBackendDAEVars(
            globalKnownVars.clone(),
            (std::sync::Arc::new(VarToGlobalKnownVarHT)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
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
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                                ),
                                i32,
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
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                        )> + 'static,
                >),
            globalKnownVarHT,
        )?;
    }
    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Start optimization module wrapFunctionCalls for "));
            __mm_s.push_str(&*daeTypeStr);
            __mm_s.push_str(&*literal!(" DAE\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Phase 0: Set up data structure\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        BackendDump::dumpVariables(&globalKnownVars, &(literal!("globalKnownVars before WFC")))?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("globalKnownVarHT before algorithm\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        BaseHashSet::dumpHashSet(&globalKnownVarHT)?;
    }
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\nHandle system (belongs to "));
                __mm_s.push_str(&*daeTypeStr);
                __mm_s.push_str(&*literal!(" DAE):\n"));
                __mm_s.push_str(&*arcstr::literal!(BORDER));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            BackendDump::dumpVariables(&syst.orderedVars, &(literal!("Variables")))?;
            BackendDump::dumpEquationArray(syst.orderedEqs.clone(), &(literal!("Equations")))?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nPhase 1: Analysis\n"));
                __mm_s.push_str(&*arcstr::literal!(BORDER));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        HT = BaseHashTable::clear(HT)?;
        exarray = ExpandableArray::clear(exarray);
        index = 0;
        orderedEqs = syst.orderedEqs.clone();
        orderedVars = syst.orderedVars.clone();
        (HT, exarray, cseIndex, index, _) = BackendEquation::traverseEquationArray(
            orderedEqs.clone(),
            &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                   __a1: (
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                                + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                    ),
                ),
                metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                i32,
                i32,
                metamodelica::Ref<AvlTreePathFunction::Tree>,
            )| wrapFunctionCalls_analysis(__a0, &__a1),
            (HT, exarray, cseIndex, index, functionTree.clone()),
        )?;
        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Hastable after analysis\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            BaseHashTable::dumpHashTable(&HT)?;
            metamodelica::print(ExpandableArray::toString(
                exarray.clone(),
                &(literal!("\nExpandable Array after analysis")),
                &move |__a0: CSE_Equation| printCSEEquation(&__a0),
                true,
            )?);
        }
        if index > 0 {
            exarray = determineDependencies(exarray, HT.clone())?;
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n\nPhase 2: Dependencies\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Hashtable after dependencies\n"));
                    __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                BaseHashTable::dumpHashTable(&HT)?;
                metamodelica::print(ExpandableArray::toString(
                    exarray.clone(),
                    &(literal!("\nExpandable Array after dependencies")),
                    &move |__a0: CSE_Equation| printCSEEquation(&__a0),
                    true,
                )?);
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n\nPhase3: Substitution\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            orderedEqs_new = BackendEquation::emptyEqnsSized(
                ExpandableArray::getNumberOfElements(orderedEqs.clone())
                    + ExpandableArray::getNumberOfElements(exarray.clone()),
            );
            (HT, exarray, orderedEqs_new) = BackendEquation::traverseEquationArray(
                orderedEqs,
                &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                       __a1: (
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                        ),
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        metamodelica::Ref<DAE::Exp>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                        ),
                    ),
                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
                )| wrapFunctionCalls_substitution(__a0, &__a1),
                (HT, exarray, orderedEqs_new),
            )?;
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Hashtable after substitution\n"));
                    __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                BaseHashTable::dumpHashTable(&HT)?;
                metamodelica::print(ExpandableArray::toString(
                    exarray.clone(),
                    &(literal!("\nExpandable Array after substitution")),
                    &move |__a0: CSE_Equation| printCSEEquation(&__a0),
                    true,
                )?);
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n\nPhase 4: Create CSE-Equations\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (orderedEqs_new, orderedVars, globalKnownVars, _) = createCseEquations(
                exarray.clone(),
                orderedEqs_new,
                orderedVars,
                globalKnownVars,
                globalKnownVarHT.clone(),
                allowGlobalKnown,
            )?;
            assign_field!(
                syst.orderedEqs = orderedEqs_new.clone(),
                syst.orderedVars = orderedVars.clone()
            );
            if !(intEq(
                BackendEquation::equationArraySize(orderedEqs_new)?,
                orderedVars.numberOfVars.clone(),
            )) {
                Error::addCompilerWarning(literal!(
                    "After manipulating the system with postOptModule wrapFunctionCalls the system is unbalanced. This indicates that the original system is singular. You can use -d=dumpCSE and -d=dumpCSE_verbose for more information."
                ))?;
            }
            assign_field!(
                syst.m = None,
                syst.mT = None,
                syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING()
            );
            if Flags::isSet(Flags::DUMP_CSE.clone())? || Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n\n\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\nFinal Results\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                BackendDump::dumpVariables(
                    &syst.orderedVars,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("########### Updated Variable List ("));
                        __mm_s.push_str(&*BackendDump::printBackendDAEType2String(
                            shared.backendDAEType.clone(),
                        )?);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }),
                )?;
                BackendDump::dumpEquationArray(
                    syst.orderedEqs.clone(),
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("########### Updated Equation List ("));
                        __mm_s.push_str(&*BackendDump::printBackendDAEType2String(
                            shared.backendDAEType.clone(),
                        )?);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }),
                )?;
                BackendDump::dumpVariables(
                    &globalKnownVars,
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("########### Updated globalKnownVars ("));
                        __mm_s.push_str(&*BackendDump::printBackendDAEType2String(
                            shared.backendDAEType.clone(),
                        )?);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }),
                )?;
                metamodelica::print(ExpandableArray::toString(
                    exarray.clone(),
                    &(literal!("\n########### CSE Replacements")),
                    &move |__a0: CSE_Equation| printCSEEquation(&__a0),
                    true,
                )?);
            }
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    ArcStr::from(__mm_s)
                });
                BackendDump::dumpEqSystem(syst.clone(), &(literal!("Final EqSystem")))?;
            }
        } else {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*arcstr::literal!(BORDER));
                    __mm_s.push_str(&*literal!("\nNo function calls found. Exiting the algorithm...\n\n\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
        eqSystems = metamodelica::cons(syst, eqSystems);
    }
    assign_field!(shared.globalKnownVars = globalKnownVars);
    System::tmpTickSetIndex(cseIndex, Global::backendDAE_cseIndex.clone());
    eqSystems = metamodelica::Dangerous::listReverseInPlace(eqSystems);
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: eqSystems,
        shared: shared,
    });
    Ok(outDAE)
}

fn VarToGlobalKnownVarHT(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inGlobalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outGlobalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = inGlobalKnownVarHT.clone();
    if !(BackendVariable::isInput(&inVar))
        && !(BackendVariable::isParam(&inVar) && !(BackendVariable::varFixed(&inVar)))
        && (inVar.bindExp).is_some()
    {
        outGlobalKnownVarHT = BaseHashSet::add(BackendVariable::varCref(&inVar), &inGlobalKnownVarHT)?;
    }
    Ok((outVar, outGlobalKnownVarHT))
}

fn findCallsInGlobalKnownVars(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ) = inTuple.clone();
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    if !(BackendVariable::isInput(&inVar))
        && !(BackendVariable::isParam(&inVar) && !(BackendVariable::varFixed(&inVar)))
        && (inVar.bindExp).is_some()
    {
        let __pa0 = ::match_deref::match_deref! { match &(inVar.bindExp.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        if isCall(&exp) {
            eq = BackendEquation::generateEquation(
                metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: inVar.varName.clone(),
                    ty: inVar.varType.clone(),
                }),
                exp,
                DAE::emptyElementSource().clone(),
                BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
            )?;
            (_, outTuple) = wrapFunctionCalls_analysis(eq, &inTuple)?;
        }
    }
    Ok((outVar, outTuple))
}

fn wrapFunctionCalls_substitution(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation> = inEq.clone();
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    let mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut orderedEqs_new: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    (HT, exarray, orderedEqs_new) = inTuple.clone();
    let () = (match &*inEq {
        BackendDAE::Equation::COMPLEX_EQUATION { .. } => {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(
                    &(list![inEq.clone()]),
                    &(literal!("wrapFunctionCalls_substitution (COMPLEX_EQUATION)")),
                )?;
            }
            let (__pa0, (__pa1, __pa2, __pa3)) = BackendEquation::traverseExpsOfEquation(
                inEq,
                (std::sync::Arc::new(wrapFunctionCalls_substitution2)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    metamodelica::Ref<
                                        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                    >,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    metamodelica::Ref<
                                        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                    >,
                                ),
                            )> + 'static,
                    >),
                (HT, exarray, orderedEqs_new),
            )?;
            eq = metamodelica::Own::own(__pa0);
            HT = metamodelica::Own::own(__pa1);
            exarray = metamodelica::Own::own(__pa2);
            orderedEqs_new = metamodelica::Own::own(__pa3);
            if !(isEquationRedundant(&eq)?) {
                orderedEqs_new = BackendEquation::add(eq.clone(), orderedEqs_new)?;
                if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                    BackendDump::dumpEquationList(&(list![eq]), &(literal!("isEquationRedundant? no")))?;
                }
            } else {
                if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                    BackendDump::dumpEquationList(&(list![eq]), &(literal!("isEquationRedundant? yes")))?;
                }
            }
            ()
        }
        BackendDAE::Equation::EQUATION { .. } => {
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(
                    &(list![inEq.clone()]),
                    &(literal!("wrapFunctionCalls_substitution (EQUATION)")),
                )?;
            }
            let (__pa0, (__pa1, __pa2, __pa3)) = BackendEquation::traverseExpsOfEquation(
                inEq,
                (std::sync::Arc::new(wrapFunctionCalls_substitution2)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    metamodelica::Ref<
                                        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                    >,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    metamodelica::Ref<
                                        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                    >,
                                ),
                            )> + 'static,
                    >),
                (HT, exarray, orderedEqs_new),
            )?;
            eq = metamodelica::Own::own(__pa0);
            HT = metamodelica::Own::own(__pa1);
            exarray = metamodelica::Own::own(__pa2);
            orderedEqs_new = metamodelica::Own::own(__pa3);
            if !(isEquationRedundant(&eq)?) {
                orderedEqs_new = BackendEquation::add(eq, orderedEqs_new)?;
            }
            ()
        }
        _ => {
            orderedEqs_new = BackendEquation::add(inEq, orderedEqs_new)?;
            ()
        }
    });
    outTuple = (HT, exarray, orderedEqs_new);
    Ok((outEq, outTuple))
}

fn wrapFunctionCalls_substitution2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    (outExp, outTuple) = Expression::traverseExpBottomUp(
        inExp,
        &move |__a0: metamodelica::Ref<DAE::Exp>,
               __a1: (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
            metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        )| wrapFunctionCalls_substitution3(__a0, &__a1),
        inTuple,
    )?;
    Ok((outExp, outTuple))
}

fn wrapFunctionCalls_substitution3(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    );
    let mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut orderedEqs_new: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    >;
    let mut id: i32;
    let mut ix: i32;
    let mut cse: metamodelica::Ref<DAE::Exp>;
    let mut call: metamodelica::Ref<DAE::Exp>;
    let mut tmp: metamodelica::Ref<DAE::Exp>;
    let mut PR: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut dependencies: metamodelica::List<i32>;
    (HT, exarray, orderedEqs_new) = inTuple.clone();
    if Expression::isCall(&inExp) && BaseHashTable::hasKey(inExp.clone(), &HT)? {
        id = BaseHashTable::get(inExp, &HT)?;
        let CSE_Equation {
            cse: __pa0,
            call: __pa1,
            dependencies: __pa2,
        } = ExpandableArray::get(id, exarray.clone())?;
        cse = metamodelica::Own::own(__pa0);
        call = metamodelica::Own::own(__pa1);
        dependencies = metamodelica::Own::own(__pa2);
        (HT, exarray) = substituteDependencies(&dependencies, HT, exarray, call.clone(), cse.clone())?;
        ExpandableArray::update(
            id,
            CSE_Equation {
                cse: cse.clone(),
                call: call,
                dependencies: metamodelica::nil(),
            },
            exarray.clone(),
        )?;
        outExp = cse;
    } else if Expression::isTSUB(&inExp) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::TSUB { exp: __pa3, ix: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        tmp = metamodelica::Own::own(__pa3);
        ix = metamodelica::Own::own(__pa4);
        if Expression::isTuple(&tmp) {
            let __pa5 = ::match_deref::match_deref! { match &(tmp) {
                Deref @ DAE::Exp::TUPLE { PR: __pa5 } => __pa5.clone(),
                _ => return Err("pattern mismatch"),
            } };
            PR = metamodelica::Own::own(__pa5);
            outExp = (PR).get(ix)?;
        } else {
            outExp = inExp;
        }
    } else {
        outExp = inExp;
    }
    outTuple = (HT, exarray, orderedEqs_new);
    Ok((outExp, outTuple))
}

fn substituteDependencies(
    mut inDependencies: &metamodelica::List<i32>,
    mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
    mut inCall: metamodelica::Ref<DAE::Exp>,
    mut inCSE: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
)> {
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    ) = ht;
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>> = exarray;
    let mut cse: metamodelica::Ref<DAE::Exp>;
    let mut call: metamodelica::Ref<DAE::Exp>;
    let mut dependencies: metamodelica::List<i32>;
    let mut cse2: metamodelica::Ref<DAE::Exp>;
    let mut call2: metamodelica::Ref<DAE::Exp>;
    let mut dependencies2: metamodelica::List<i32>;
    let mut id2: i32;
    for mut id in &**inDependencies {
        let CSE_Equation {
            cse: __pa0,
            call: __pa1,
            dependencies: __pa2,
        } = ExpandableArray::get(id.clone(), exarray.clone())?;
        cse = metamodelica::Own::own(__pa0);
        call = metamodelica::Own::own(__pa1);
        dependencies = metamodelica::Own::own(__pa2);
        call = substituteExp(call, inCall.clone(), inCSE.clone())?;
        if !(BaseHashTable::hasKey(call.clone(), &ht)?) {
            ht = BaseHashTable::add((call.clone(), id.clone()), ht)?;
            ExpandableArray::update(
                id.clone(),
                CSE_Equation {
                    cse: cse,
                    call: call,
                    dependencies: dependencies,
                },
                exarray.clone(),
            )?;
        } else {
            id2 = BaseHashTable::get(call.clone(), &ht)?;
            let CSE_Equation {
                cse: __pa3,
                call: __pa4,
                dependencies: __pa5,
            } = ExpandableArray::get(id2, exarray.clone())?;
            cse2 = metamodelica::Own::own(__pa3);
            call2 = metamodelica::Own::own(__pa4);
            dependencies2 = metamodelica::Own::own(__pa5);
            cse2 = mergeCSETuples(cse.clone(), cse2)?;
            ExpandableArray::update(
                id2,
                CSE_Equation {
                    cse: cse2.clone(),
                    call: call,
                    dependencies: UnorderedSet::unique_list(
                        listAppend(dependencies, dependencies2),
                        std::sync::Arc::new(fnptr!(Util::id, _)),
                        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    )?,
                },
                exarray.clone(),
            )?;
            ExpandableArray::update(
                id.clone(),
                CSE_Equation {
                    cse: cse,
                    call: cse2,
                    dependencies: metamodelica::nil(),
                },
                exarray.clone(),
            )?;
        }
    }
    Ok((ht, exarray))
}

fn substituteExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inKey: metamodelica::Ref<DAE::Exp>,
    mut inValue: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outExp, _) = Expression::traverseExpTopDown(inExp, &substituteExp2, (inKey, inValue))?;
    Ok(outExp)
}

fn substituteExp2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTuple: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) = inTuple.clone();
    let mut key: metamodelica::Ref<DAE::Exp>;
    let mut value: metamodelica::Ref<DAE::Exp>;
    let mut tmp: metamodelica::Ref<DAE::Exp>;
    let mut expList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut ix: i32;
    (key, value) = inTuple;
    if ExpressionBasics::expEqual(&inExp, key.clone())? {
        outExp = value;
        cont = false;
    } else if Expression::isTSUB(&inExp) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::TSUB { exp: __pa0, ix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        tmp = metamodelica::Own::own(__pa0);
        ix = metamodelica::Own::own(__pa1);
        if ExpressionBasics::expEqual(&tmp, key)? {
            let __pa2 = ::match_deref::match_deref! { match &(value) {
                Deref @ DAE::Exp::TUPLE { PR: __pa2 } => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expList = metamodelica::Own::own(__pa2);
            outExp = (expList).get(ix)?;
            cont = false;
        } else {
            outExp = inExp;
            cont = true;
        }
    } else {
        outExp = inExp;
        cont = true;
    }
    Ok((outExp, cont, outTuple))
}

fn createCseEquations(
    mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
    mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut orderedVars: BackendDAE::Variables,
    mut globalKnownVars: BackendDAE::Variables,
    mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut allowGlobalKnown: bool,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    BackendDAE::Variables,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
)> {
    let mut orderedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        orderedEqs;
    let mut orderedVars: BackendDAE::Variables = orderedVars;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = globalKnownVarHT;
    let mut cse: metamodelica::Ref<DAE::Exp>;
    let mut call: metamodelica::Ref<DAE::Exp>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut var: metamodelica::Ref<BackendDAE::Var> =
        <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
    let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut delVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut isGlobalKnown: bool;
    let mut eqRedundant: bool;
    let mut add: bool;
    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("globalKnownVars:\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        BaseHashSet::dumpHashSet(&globalKnownVarHT)?;
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nTraverse expandable array\n"));
            __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut i in ({
        let __s = ExpandableArray::getNumberOfElements(exarray.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        add = true;
        let CSE_Equation {
            cse: __pa0,
            call: __pa1,
            ..
        } = ExpandableArray::get(i, exarray.clone())?;
        cse = metamodelica::Own::own(__pa0);
        call = metamodelica::Own::own(__pa1);
        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n--> cse-equation: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(cse.clone())?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(call.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        eq = BackendEquation::generateEquation(
            cse.clone(),
            call.clone(),
            DAE::emptyElementSource().clone(),
            BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
        )?;
        (
            globalKnownVarHT,
            globalKnownVars,
            orderedVars,
            eqRedundant,
            isGlobalKnown,
        ) = isEquationRedundant_flatten(&eq, globalKnownVarHT, globalKnownVars, orderedVars, allowGlobalKnown)?;
        if debug.clone() {
            metamodelica::print(literal!("\ndebug 1 - eq redundant?\n"));
        }
        if !(eqRedundant) {
            if debug.clone() {
                metamodelica::print(literal!("\ndebug 2 - no, not redundant. let's loop\n"));
            }
            varList = createVarsForExp(cse.clone(), metamodelica::nil())?;
            if (varList).is_empty() {
                orderedEqs = BackendEquation::add(eq, orderedEqs)?;
            } else {
                for mut var in &*varList {
                    let mut var = var.clone();
                    if debug.clone() {
                        metamodelica::print({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("\ndebug 3 - handle var: "));
                            __mm_s.push_str(&*BackendDump::varString(&var)?);
                            __mm_s.push_str(&*literal!(" Is it a globalKnownVar?\n"));
                            ArcStr::from(__mm_s)
                        });
                    }
                    cr = BackendVariable::varCref(&var);
                    if !(isGlobalKnown) {
                        if debug.clone() {
                            metamodelica::print(literal!(
                                "\ndebug 4 - The variable is not a globalKnownVar. Should an equation be added?\n"
                            ));
                        }
                        if add {
                            if debug.clone() {
                                metamodelica::print(literal!("\ndebug 5 - yes, definitely!\n"));
                            }
                            orderedEqs = BackendEquation::add(eq.clone(), orderedEqs)?;
                            add = false;
                        }
                        if debug.clone() {
                            metamodelica::print({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("\ndebug 6 - Is this cref a CSE cref?: "));
                                __mm_s.push_str(&*ExpressionBasics::printExpStr(Expression::crefExp(cr.clone())?)?);
                                __mm_s.push_str(&*literal!("\n"));
                                ArcStr::from(__mm_s)
                            });
                        }
                        if isCSECref(&cr) {
                            if debug.clone() {
                                metamodelica::print(literal!(
                                    "\ndebug 7 - yes it is a CSE cref. Add to orderedVars!\n"
                                ));
                            }
                            orderedVars = BackendVariable::addVar(var, orderedVars)?;
                        }
                        if debug.clone() {
                            metamodelica::print(literal!("\ndebug 8\n"));
                        }
                    } else {
                        if debug.clone() {
                            metamodelica::print(literal!("\ndebug 9 - The variable is a globalKnownVar.\n"));
                        }
                        if !(isCSECref(&cr)) {
                            if debug.clone() {
                                metamodelica::print(literal!(
                                    "\ndebug 10 - The globalKnownVar is no CSE cref, so copy attributes and delete it from orderedVars if it is in that list.\n"
                                ));
                            }
                            (delVars, orderedVars) =
                                BackendVariable::deleteVarIfExistsAndReturn(cr.clone(), orderedVars);
                            if (delVars).is_empty() {
                                (delVars, _) = BackendVariable::getVar(cr, &globalKnownVars)?;
                            }
                            var = (delVars).get(1)?;
                        }
                        var = BackendVariable::setBindExp(var, Some(call.clone()));
                        var = BackendVariable::makeParam(var);
                        var = BackendVariable::setVarFinal(var, true)?;
                        if intGt(((varList).len() as i32), 1) || Expression::isTuple(&cse) {
                            if debug.clone() {
                                metamodelica::print(literal!("\ndebug 11 - It is a tuple! Add it to tplExp\n"));
                            }
                            assign_field!(var.tplExp = Some(cse.clone()));
                        }
                        if debug.clone() {
                            metamodelica::print(literal!("\ndebug 12 - Add the variable to globalKnownVars\n"));
                        }
                        globalKnownVars = BackendVariable::addVar(var, globalKnownVars)?;
                    }
                }
            }
        }
    }
    if debug.clone() {
        metamodelica::print(literal!("\ndebug 13\n"));
    }
    Ok((orderedEqs, orderedVars, globalKnownVars, globalKnownVarHT))
}

fn determineDependencies(
    mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
    mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>> {
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>> = exarray;
    let mut callArguments: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    for mut i in 1..=ExpandableArray::getNumberOfElements(exarray.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(ExpandableArray::get(i, exarray.clone())?) {
            CSE_Equation { call: Deref @ DAE::Exp::CALL { expLst: __pa0, .. }, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        callArguments = metamodelica::Own::own(__pa0);
        let (_, (_, __pa2, _)) =
            Expression::traverseExpList(callArguments, &determineDependencies2, (HT.clone(), exarray, i))?;
        exarray = metamodelica::Own::own(__pa2);
    }
    Ok(exarray)
}

fn determineDependencies2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
    );
    let mut id: i32;
    let mut index: i32;
    let mut dependencies: metamodelica::List<i32>;
    let mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cse: metamodelica::Ref<DAE::Exp>;
    let mut call: metamodelica::Ref<DAE::Exp>;
    if Expression::isCall(&inExp) {
        (HT, exarray, index) = inTuple;
        if BaseHashTable::hasKey(inExp.clone(), &HT)? {
            id = BaseHashTable::get(inExp, &HT)?;
            let CSE_Equation {
                cse: __pa0,
                call: __pa1,
                dependencies: __pa2,
            } = ExpandableArray::get(id, exarray.clone())?;
            cse = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            dependencies = metamodelica::Own::own(__pa2);
            if !(listMember(index, dependencies.clone())) {
                dependencies = metamodelica::cons(index, dependencies);
                ExpandableArray::update(
                    id,
                    CSE_Equation {
                        cse: cse,
                        call: call,
                        dependencies: dependencies,
                    },
                    exarray.clone(),
                )?;
            }
        }
        outTuple = (HT, exarray, index);
    } else {
        outTuple = inTuple;
    }
    Ok((outExp, outTuple))
}

fn allArgsInGlobalKnownVars(
    mut callArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut globalKnownVarHT: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<bool> {
    let mut allCrefsAreGlobal: bool = true;
    let mut crefList: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (_, crefList) =
        Expression::traverseExpList(callArgs, &Expression::traversingComponentRefFinder, metamodelica::nil())?;
    for mut cr in &*crefList {
        if allCrefsAreGlobal {
            allCrefsAreGlobal = BaseHashSet::has(cr.clone(), globalKnownVarHT)?;
        } else {
            return Ok(allCrefsAreGlobal);
        }
    }
    Ok(allCrefsAreGlobal)
}

fn addConstantCseVarsToGlobalKnownVarHT(
    mut cse_crExp: &metamodelica::Ref<DAE::Exp>,
    mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
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
    ),
)> {
    let mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = globalKnownVarHT;
    let () = (::match_deref::match_deref! { match cse_crExp {
        Deref @ DAE::Exp::TUPLE { PR: expLst } => {
            for mut exp in &*expLst.clone() {
                if Expression::isNotWild(metamodelica::AsArg::as_arg(&exp)) {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(metamodelica::AsArg::as_arg(&exp), globalKnownVarHT)?;
                }
            }
            ()
        },
        Deref @ DAE::Exp::CALL { expLst, .. } => {
            for mut exp in &*expLst.clone() {
                if Expression::isNotWild(metamodelica::AsArg::as_arg(&exp)) {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(metamodelica::AsArg::as_arg(&exp), globalKnownVarHT)?;
                }
            }
            ()
        },
        Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
            for mut exp in &*expLst.clone() {
                if Expression::isNotWild(metamodelica::AsArg::as_arg(&exp)) {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(metamodelica::AsArg::as_arg(&exp), globalKnownVarHT)?;
                }
            }
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            globalKnownVarHT = BaseHashSet::add(cr.clone(), &globalKnownVarHT)?;
            crefs = ComponentReference::expandCref(cr, true)?;
            for mut cr_ in &*crefs {
                globalKnownVarHT = BaseHashSet::add(cr_.clone(), &globalKnownVarHT)?;
            }
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (Expression::isArrayType(&(Expression::r#typeof(cse_crExp.clone())?))) => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            globalKnownVarHT = BaseHashSet::add(cr.clone(), &globalKnownVarHT)?;
            crefs = ComponentReference::expandCref(cr, true)?;
            for mut cr_ in &*crefs {
                globalKnownVarHT = BaseHashSet::add(cr_.clone(), &globalKnownVarHT)?;
            }
            ()
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            globalKnownVarHT = BaseHashSet::add(cr.clone(), &globalKnownVarHT)?;
            ()
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("addConstantCseVarsToGlobalKnownVarHT failed. Reached else case that should not be reachable while handling CSE expression:\n")); __mm_s.push_str(&*ExpressionDump::dumpExpStr(cse_crExp.clone(), 0)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/CommonSubExpression.mo"))?;
            return Err("fail");
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(globalKnownVarHT)
}

fn wrapFunctionCalls_analysis(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: &(
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation> = inEq.clone();
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    );
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cseIndex: i32;
    let mut exIndex: i32;
    let mut index: i32;
    let mut ix: i32;
    let mut lhs: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut cref: metamodelica::Ref<DAE::Exp>;
    let mut call: metamodelica::Ref<DAE::Exp>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut cseEquation: CSE_Equation;
    (HT, exarray, cseIndex, index, functionTree) = inTuple.clone();
    let () = (match &*inEq {
        BackendDAE::Equation::COMPLEX_EQUATION {
            left: __esc_lhs,
            right: __esc_rhs,
            ..
        } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(
                    &(list![inEq.clone()]),
                    &(literal!("wrapFunctionCalls_analysis (COMPLEX_EQUATION)")),
                )?;
            }
            if isCallAndTuple(metamodelica::AsArg::as_arg(&lhs), metamodelica::AsArg::as_arg(&rhs)) {
                (cref, call) = getTheRightPattern(lhs.clone(), rhs.clone())?;
                if BaseHashTable::hasKey(call.clone(), &HT)? {
                    exIndex = BaseHashTable::get(call, &HT)?;
                    cseEquation = ExpandableArray::get(exIndex, exarray.clone())?;
                    cseEquation.cse = mergeCSETuples(cseEquation.cse.clone(), cref)?;
                    exarray = ExpandableArray::update(exIndex, cseEquation, exarray)?;
                } else if !(isSkipCase(call.clone(), &functionTree)?) {
                    index = index + 1;
                    HT = BaseHashTable::add((call.clone(), index), HT)?;
                    exarray = ExpandableArray::set(
                        index,
                        CSE_Equation {
                            cse: cref,
                            call: call,
                            dependencies: metamodelica::nil(),
                        },
                        exarray,
                    )?;
                }
            } else if isCallAndRecord(metamodelica::AsArg::as_arg(&lhs), metamodelica::AsArg::as_arg(&rhs)) {
                (cref, call) = getTheRightPattern(lhs.clone(), rhs.clone())?;
                if BaseHashTable::hasKey(call.clone(), &HT)? {
                    exIndex = BaseHashTable::get(call, &HT)?;
                    cseEquation = ExpandableArray::get(exIndex, exarray.clone())?;
                    cseEquation.cse = cref;
                    exarray = ExpandableArray::update(exIndex, cseEquation, exarray)?;
                } else if !(isSkipCase(call.clone(), &functionTree)?) {
                    index = index + 1;
                    HT = BaseHashTable::add((call.clone(), index), HT)?;
                    exarray = ExpandableArray::set(
                        index,
                        CSE_Equation {
                            cse: cref,
                            call: call,
                            dependencies: metamodelica::nil(),
                        },
                        exarray,
                    )?;
                }
            }
            let (_, (__pa0, __pa1, __pa2, __pa3, __pa4)) = BackendEquation::traverseExpsOfEquation(
                inEq,
                (std::sync::Arc::new(wrapFunctionCalls_analysis2)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    i32,
                                    i32,
                                    metamodelica::Ref<AvlTreePathFunction::Tree>,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    i32,
                                    i32,
                                    metamodelica::Ref<AvlTreePathFunction::Tree>,
                                ),
                            )> + 'static,
                    >),
                (HT, exarray, cseIndex, index, functionTree),
            )?;
            HT = metamodelica::Own::own(__pa0);
            exarray = metamodelica::Own::own(__pa1);
            cseIndex = metamodelica::Own::own(__pa2);
            index = metamodelica::Own::own(__pa3);
            functionTree = metamodelica::Own::own(__pa4);
            ()
        }
        BackendDAE::Equation::EQUATION {
            exp: __esc_lhs,
            scalar: __esc_rhs,
            ..
        } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                BackendDump::dumpEquationList(
                    &(list![inEq.clone()]),
                    &(literal!("wrapFunctionCalls_analysis (EQUATION)")),
                )?;
            }
            if isCallAndCref(metamodelica::AsArg::as_arg(&lhs), metamodelica::AsArg::as_arg(&rhs))
                || isConstAndCall(metamodelica::AsArg::as_arg(&lhs), metamodelica::AsArg::as_arg(&rhs))
            {
                (cref, call) = getTheRightPattern(lhs.clone(), rhs.clone())?;
                if BaseHashTable::hasKey(call.clone(), &HT)? {
                    exIndex = BaseHashTable::get(call, &HT)?;
                    cseEquation = ExpandableArray::get(exIndex, exarray.clone())?;
                    cseEquation.cse = cref;
                    exarray = ExpandableArray::update(exIndex, cseEquation, exarray)?;
                } else if !(isSkipCase(call.clone(), &functionTree)?) {
                    index = index + 1;
                    HT = BaseHashTable::add((call.clone(), index), HT)?;
                    exarray = ExpandableArray::set(
                        index,
                        CSE_Equation {
                            cse: cref,
                            call: call,
                            dependencies: metamodelica::nil(),
                        },
                        exarray,
                    )?;
                }
            } else if isTsubAndCref(metamodelica::AsArg::as_arg(&lhs), metamodelica::AsArg::as_arg(&rhs)) {
                let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(getTheRightPattern(lhs.clone(), rhs.clone())?) {
                    (__pa0, Deref @ DAE::Exp::TSUB { exp: __pa2 @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_TUPLE { types: __pa1, .. }, .. }, .. }, ix: __pa3, ty: _ }) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cref = metamodelica::Own::own(__pa0);
                types = metamodelica::Own::own(__pa1);
                call = metamodelica::Own::own(__pa2);
                ix = metamodelica::Own::own(__pa3);
                if BaseHashTable::hasKey(call.clone(), &HT)? {
                    exIndex = BaseHashTable::get(call, &HT)?;
                    cseEquation = ExpandableArray::get(exIndex, exarray.clone())?;
                    cref = createCrefForTsub(((types).len() as i32), ix, cref);
                    cseEquation.cse = mergeCSETuples(cseEquation.cse.clone(), cref)?;
                    exarray = ExpandableArray::update(exIndex, cseEquation, exarray)?;
                } else if !(isSkipCase(call.clone(), &functionTree)?) {
                    index = index + 1;
                    HT = BaseHashTable::add((call.clone(), index), HT)?;
                    cref = createCrefForTsub(((types).len() as i32), ix, cref);
                    exarray = ExpandableArray::set(
                        index,
                        CSE_Equation {
                            cse: cref,
                            call: call,
                            dependencies: metamodelica::nil(),
                        },
                        exarray,
                    )?;
                }
            }
            let (_, (__pa5, __pa6, __pa7, __pa8, __pa9)) = BackendEquation::traverseExpsOfEquation(
                inEq,
                (std::sync::Arc::new(wrapFunctionCalls_analysis2)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    i32,
                                    i32,
                                    metamodelica::Ref<AvlTreePathFunction::Tree>,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
                                    i32,
                                    i32,
                                    metamodelica::Ref<AvlTreePathFunction::Tree>,
                                ),
                            )> + 'static,
                    >),
                (HT, exarray, cseIndex, index, functionTree),
            )?;
            HT = metamodelica::Own::own(__pa5);
            exarray = metamodelica::Own::own(__pa6);
            cseIndex = metamodelica::Own::own(__pa7);
            index = metamodelica::Own::own(__pa8);
            functionTree = metamodelica::Own::own(__pa9);
            ()
        }
        _ => (),
    });
    outTuple = (HT, exarray, cseIndex, index, functionTree);
    Ok((outEq, outTuple))
}

fn createCrefForTsub(
    mut length: i32,
    mut ix: i32,
    mut cref: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outCref: metamodelica::Ref<DAE::Exp>;
    let mut expList: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    for mut i in 1..=ix - 1 {
        expList = metamodelica::cons(
            metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(),
                ty: DAE::T_UNKNOWN_DEFAULT().clone(),
            }),
            expList,
        );
    }
    expList = metamodelica::cons(cref, expList);
    for mut i in ix + 1..=length {
        expList = metamodelica::cons(
            metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(),
                ty: DAE::T_UNKNOWN_DEFAULT().clone(),
            }),
            expList,
        );
    }
    outCref = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expList.reverse() });
    outCref
}

fn wrapFunctionCalls_analysis2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    );
    (_, outTuple) = Expression::traverseExpTopDown(inExp, &wrapFunctionCalls_analysis3, inTuple)?;
    Ok((outExp, outTuple))
}

fn wrapFunctionCalls_analysis3(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut cont: bool;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>,
        i32,
        i32,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    );
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut HT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    let mut exarray: metamodelica::Ref<ExpandableArray::ExpandableArray<CSE_Equation>>;
    let mut cseIndex: i32;
    let mut index: i32;
    let mut tsub: metamodelica::Ref<DAE::Exp>;
    (HT, exarray, cseIndex, index, functionTree) = inTuple.clone();
    cont = ({
        let mut expList: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::IFEXP { expCond: __inExp_expCond, .. } => {
                (_, outTuple) = Expression::traverseExpTopDown(__inExp_expCond.clone(), &wrapFunctionCalls_analysis3, inTuple)?;
                cont = false;
                return Ok((outExp, cont, outTuple));
                return Err("fail")
            },
            _ if (isSkipCase(inExp.clone(), &functionTree)?) => {
                false
            },
            __esc_tsub @ Deref @ DAE::Exp::TSUB { exp: call @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_TUPLE { types, .. }, .. }, .. }, ix, ty } => {
                tsub = (*__esc_tsub).clone();
                let mut cse_var: metamodelica::Ref<DAE::Exp>;
                let mut cse_var2: metamodelica::Ref<DAE::Exp>;
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut id: i32;
                let mut cseEquation: CSE_Equation;
                if !(BaseHashTable::hasKey(call.clone(), &HT)?) {
                    index = index + 1;
                    HT = BaseHashTable::add((call.clone(), index), HT)?;
                    (cse_var, cseIndex) = createReturnExp(metamodelica::AsArg::as_arg(&ty), cseIndex, &(literal!("$cse")), false)?;
                    cse_var2 = createCrefForTsub(((types).len() as i32), ix.clone(), cse_var);
                    exarray = ExpandableArray::set(index, CSE_Equation { cse: cse_var2, call: call.clone(), dependencies: metamodelica::nil() }, exarray)?;
                } else {
                    id = BaseHashTable::get(call.clone(), &HT)?;
                    cseEquation = ExpandableArray::get(id, exarray.clone())?;
                    if Expression::isTuple(&cseEquation.cse) {
                        let __pa0 = ::match_deref::match_deref! { match &(cseEquation.cse.clone()) {
                            Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        expList = metamodelica::Own::own(__pa0);
                        e = (expList).get(ix.clone())?;
                        if isWildCref(&e) {
                            (cse_var, cseIndex) = createReturnExp(metamodelica::AsArg::as_arg(&ty), cseIndex, &(literal!("$cse")), false)?;
                            expList = List::set(expList, ix.clone(), cse_var)?;
                            cseEquation.cse = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expList });
                            exarray = ExpandableArray::update(id, cseEquation, exarray)?;
                        }
                    } else {
                        Error::addMessage(Error::GENERIC_ELAB_EXPRESSION.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionDump::dumpExpStr(inExp.clone(), 0)?); __mm_s.push_str(&*literal!(" This should never happen, Error in wrapFunctionCalls_analysis3. Trying to recover.")); ArcStr::from(__mm_s) }])?;
                    }
                }
                true
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RELATION { exp1: e, operator: _, exp2: e2, index: _, optionExpisASUB: _ }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                (_, outTuple) = Expression::traverseExpTopDown(e.clone(), &wrapFunctionCalls_analysis3, inTuple)?;
                (_, outTuple) = Expression::traverseExpTopDown(e2.clone(), &wrapFunctionCalls_analysis3, outTuple)?;
                cont = false;
                return Ok((outExp, cont, outTuple));
                true
            },
            Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty, .. }, .. } => {
                let mut cse_var: metamodelica::Ref<DAE::Exp>;
                if !(BaseHashTable::hasKey(inExp.clone(), &HT)?) {
                    index = index + 1;
                    HT = BaseHashTable::add((inExp.clone(), index), HT)?;
                    (cse_var, cseIndex) = createReturnExp(metamodelica::AsArg::as_arg(&ty), cseIndex, &(literal!("$cse")), false)?;
                    exarray = ExpandableArray::set(index, CSE_Equation { cse: cse_var, call: inExp.clone(), dependencies: metamodelica::nil() }, exarray)?;
                }
                true
            },
            _ => {
                true
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    outTuple = (HT, exarray, cseIndex, index, functionTree);
    Ok((outExp, cont, outTuple))
}

fn getTheRightPattern(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut outExp1: metamodelica::Ref<DAE::Exp>;
    let mut outExp2: metamodelica::Ref<DAE::Exp>;
    (outExp1, outExp2) = (::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::RCONST { .. }, Deref @ DAE::Exp::CALL { .. }) => (inExp1, inExp2),
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::RCONST { .. }) => (inExp2, inExp1),
        (Deref @ DAE::Exp::TUPLE { .. }, Deref @ DAE::Exp::CALL { .. }) => (inExp1, inExp2),
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::TUPLE { .. }) => (inExp2, inExp1),
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { .. }) => (inExp1, inExp2),
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::CREF { .. }) => (inExp2, inExp1),
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::TSUB { .. }) => (inExp1, inExp2),
        (Deref @ DAE::Exp::TSUB { .. }, Deref @ DAE::Exp::CREF { .. }) => (inExp2, inExp1),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2))
}

fn isEquationRedundant(mut inEq: &metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match inEq {
        Deref @ BackendDAE::Equation::EQUATION { exp: exp1, scalar: exp2, .. } => {
            ExpressionBasics::expEqual(exp1, exp2.clone())?
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: lhs }, scalar: Deref @ DAE::Exp::TUPLE { PR: rhs }, .. } if (((lhs).len() as i32) == ((rhs).len() as i32)) => {
            metamodelica::print(literal!("This should never appear\n"));
            isEquationRedundant2(lhs.clone(), rhs.clone())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::TUPLE { PR: lhs }, right: Deref @ DAE::Exp::TUPLE { PR: rhs }, .. } if (((lhs).len() as i32) == ((rhs).len() as i32)) => {
            isEquationRedundant2(lhs.clone(), rhs.clone())?
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp1 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, right: exp2 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, .. } => {
            ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp1), exp2.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

fn isEquationRedundant2(
    mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<bool> {
    let mut result: bool = true;
    let mut l: metamodelica::Ref<DAE::Exp>;
    let mut r: metamodelica::Ref<DAE::Exp>;
    let mut ll: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rr: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    if (lhs).is_empty() {
        return Ok(result);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lhs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    l = metamodelica::Own::own(__pa0);
    ll = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rhs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r = metamodelica::Own::own(__pa2);
    rr = metamodelica::Own::own(__pa3);
    if !(isWildCref(&l)) && !(isWildCref(&r)) {
        if !(ExpressionBasics::expEqual(&l, r)?) {
            result = false;
            return Ok(result);
        }
    }
    result = isEquationRedundant2(ll, rr)?;
    Ok(result)
}

fn isEquationRedundant_flatten(
    mut inEq: &metamodelica::Ref<BackendDAE::Equation>,
    mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut globalKnownVars: BackendDAE::Variables,
    mut orderedVars: BackendDAE::Variables,
    mut allowGlobalKnown: bool,
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    BackendDAE::Variables,
    BackendDAE::Variables,
    bool,
    bool,
)> {
    let mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = globalKnownVarHT;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut orderedVars: BackendDAE::Variables = orderedVars;
    let mut outB: bool;
    let mut isGlobalKnown: bool = false;
    outB = (::match_deref::match_deref! { match inEq {
        Deref @ BackendDAE::Equation::EQUATION { exp: exp1, scalar: exp2, .. } => {
            let mut isRedundant: bool;
            isRedundant = ExpressionBasics::expEqual(exp1, exp2.clone())?;
            if !(isRedundant) {
                isGlobalKnown = allowGlobalKnown && allArgsInGlobalKnownVars(list![exp2.clone()], &globalKnownVarHT)?;
                if isGlobalKnown {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp1, globalKnownVarHT)?;
                }
            }
            isRedundant
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::TUPLE { PR: lhs }, right: Deref @ DAE::Exp::TUPLE { PR: rhs }, .. } if (((lhs).len() as i32) == ((rhs).len() as i32)) => {
            let mut isRedundant: bool;
            (globalKnownVarHT, globalKnownVars, orderedVars, isRedundant) = isEquationRedundant_flatten2(lhs.clone(), rhs.clone(), globalKnownVarHT, globalKnownVars, orderedVars)?;
            isRedundant
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size: _, left: exp1 @ Deref @ DAE::Exp::TUPLE { PR: lhs }, right: exp2, source: _, attr: _ } => {
            let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            let mut isRedundant: bool;
            isRedundant = ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&exp1), exp2.clone())?;
            if !(isRedundant) {
                isGlobalKnown = allowGlobalKnown && allArgsInGlobalKnownVars(list![exp2.clone()], &globalKnownVarHT)?;
                if isGlobalKnown {
                    for mut expMem in &*lhs.clone() {
                        varList = createVarsForExp(expMem.clone(), metamodelica::nil())?;
                        for mut var in &*varList {
                            let mut var = var.clone();
                            var = BackendVariable::setBindExp(var, Some(exp2.clone()));
                            globalKnownVars = BackendVariable::addVar(var, globalKnownVars)?;
                            globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(metamodelica::AsArg::as_arg(&expMem), globalKnownVarHT)?;
                        }
                    }
                }
            }
            isRedundant
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: exp1, right: exp2, .. } => {
            let mut isRedundant: bool;
            isRedundant = ExpressionBasics::expEqual(exp1, exp2.clone())?;
            if !(isRedundant) {
                isGlobalKnown = allowGlobalKnown && allArgsInGlobalKnownVars(list![exp2.clone()], &globalKnownVarHT)?;
                if isGlobalKnown {
                    globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(exp1, globalKnownVarHT)?;
                }
            }
            isRedundant
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((globalKnownVarHT, globalKnownVars, orderedVars, outB, isGlobalKnown))
}

fn isEquationRedundant_flatten2(
    mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    mut globalKnownVars: BackendDAE::Variables,
    mut orderedVars: BackendDAE::Variables,
) -> Result<(
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
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
        ),
    ),
    BackendDAE::Variables,
    BackendDAE::Variables,
    bool,
)> {
    let mut globalKnownVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = globalKnownVarHT;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut orderedVars: BackendDAE::Variables = orderedVars;
    let mut result: bool = true;
    let mut l: metamodelica::Ref<DAE::Exp>;
    let mut r: metamodelica::Ref<DAE::Exp>;
    let mut ll: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rr: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    if (lhs).is_empty() {
        return Ok((globalKnownVarHT, globalKnownVars, orderedVars, result));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lhs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    l = metamodelica::Own::own(__pa0);
    ll = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rhs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    r = metamodelica::Own::own(__pa2);
    rr = metamodelica::Own::own(__pa3);
    if !(isWildCref(&l)) && !(isWildCref(&r)) {
        if !(ExpressionBasics::expEqual(&l, r.clone())?) {
            if BaseHashSet::has(Expression::expCref(&r)?, &globalKnownVarHT)? {
                let __pa4 = ::match_deref::match_deref! { match &(createVarsForExp(l.clone(), metamodelica::nil())?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } => __pa4.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                var = metamodelica::Own::own(__pa4);
                var = BackendVariable::setBindExp(var, Some(r));
                globalKnownVars = BackendVariable::addVar(var.clone(), globalKnownVars)?;
                globalKnownVarHT = addConstantCseVarsToGlobalKnownVarHT(&l, globalKnownVarHT)?;
                if !(isCSECref(&var.varName)) {
                    (_, orderedVars) = BackendVariable::deleteVarIfExistsAndReturn(var.varName.clone(), orderedVars);
                }
            } else {
                result = false;
                return Ok((globalKnownVarHT, globalKnownVars, orderedVars, result));
            }
        }
    }
    (globalKnownVarHT, globalKnownVars, orderedVars, result) =
        isEquationRedundant_flatten2(ll, rr, globalKnownVarHT, globalKnownVars, orderedVars)?;
    Ok((globalKnownVarHT, globalKnownVars, orderedVars, result))
}

fn isCall(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::CALL { .. } => true,
        _ => false,
    });
    outBoolean
}

fn isCallAndCref(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inExp2: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match (inExp, inExp2) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::CREF { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isTsubAndCref(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inExp2: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match (inExp, inExp2) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::TSUB { .. }) => true,
        (Deref @ DAE::Exp::TSUB { .. }, Deref @ DAE::Exp::CREF { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isConstAndCall(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inExp2: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match (inExp, inExp2) {
        (Deref @ DAE::Exp::RCONST { .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::RCONST { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isCallAndTuple(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inExp2: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match (inExp, inExp2) {
        (Deref @ DAE::Exp::TUPLE { .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::TUPLE { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn isCallAndRecord(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inExp2: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match (inExp, inExp2) {
        (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, Deref @ DAE::Exp::CALL { .. }) => true,
        (Deref @ DAE::Exp::CALL { .. }, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn mergeCSETuples(
    mut inCref1: metamodelica::Ref<DAE::Exp>,
    mut inCref2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outCref: metamodelica::Ref<DAE::Exp>;
    let mut expLst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expLst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expLst3: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    if Expression::isTuple(&inCref1) && Expression::isTuple(&inCref2) {
        let __pa0 = ::match_deref::match_deref! { match &(inCref1) {
            Deref @ DAE::Exp::TUPLE { PR: __pa0 } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        expLst1 = metamodelica::Own::own(__pa0);
        let __pa1 = ::match_deref::match_deref! { match &(inCref2) {
            Deref @ DAE::Exp::TUPLE { PR: __pa1 } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        expLst2 = metamodelica::Own::own(__pa1);
        expLst1 = mergeCSETuples2(&expLst1, &expLst2)?;
        outCref = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst1 });
    } else if !(Expression::isTuple(&inCref1)) && Expression::isTuple(&inCref2) {
        metamodelica::print(literal!("mergeCSETuples: This should never appear! (1)\n"));
        let __pa2 = ::match_deref::match_deref! { match &(inCref2) {
            Deref @ DAE::Exp::TUPLE { PR: __pa2 } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        expLst2 = metamodelica::Own::own(__pa2);
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(expLst2.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa3);
        expLst3 = metamodelica::Own::own(__pa4);
        if isWildCref(&e) {
            expLst2 = metamodelica::cons(inCref1, expLst3);
        }
        outCref = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst2 });
    } else if Expression::isTuple(&inCref1) && !(Expression::isTuple(&inCref2)) {
        metamodelica::print(literal!("mergeCSETuples: This should never appear! (2)\n"));
        let __pa5 = ::match_deref::match_deref! { match &(inCref1) {
            Deref @ DAE::Exp::TUPLE { PR: __pa5 } => __pa5.clone(),
            _ => return Err("pattern mismatch"),
        } };
        expLst1 = metamodelica::Own::own(__pa5);
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(expLst1.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa6);
        expLst3 = metamodelica::Own::own(__pa7);
        if isWildCref(&e) {
            expLst1 = metamodelica::cons(inCref2, expLst3);
        }
        outCref = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst1 });
    } else {
        outCref = inCref1;
    }
    Ok(outCref)
}

fn mergeCSETuples2(
    mut inExpLst1: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExpLst2: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    outExpLst = (::match_deref::match_deref! { match (inExpLst1, inExpLst2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            outExpLst
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: expLst1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: expLst2 }) => {
            outExpLst = mergeCSETuples2(expLst1, expLst2)?;
            if !(isWildCref(metamodelica::AsArg::as_arg(&e1))) && !(isWildCref(metamodelica::AsArg::as_arg(&e2))) {
                if isCSEExp(metamodelica::AsArg::as_arg(&e1)) && !(isCSEExp(metamodelica::AsArg::as_arg(&e2))) {
                    outExpLst = metamodelica::cons(e2.clone(), outExpLst);
                } else {
                    outExpLst = metamodelica::cons(e1.clone(), outExpLst);
                }
            } else if isWildCref(metamodelica::AsArg::as_arg(&e1)) && !(isWildCref(metamodelica::AsArg::as_arg(&e2))) {
                outExpLst = metamodelica::cons(e2.clone(), outExpLst);
            } else if !(isWildCref(metamodelica::AsArg::as_arg(&e1))) && isWildCref(metamodelica::AsArg::as_arg(&e2)) {
                outExpLst = metamodelica::cons(e1.clone(), outExpLst);
            } else if isWildCref(metamodelica::AsArg::as_arg(&e1)) && isWildCref(metamodelica::AsArg::as_arg(&e2)) {
                outExpLst = metamodelica::cons(e1.clone(), outExpLst);
            }
            outExpLst
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExpLst)
}

fn isWildCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isSkipCase(
    mut inCall: metamodelica::Ref<DAE::Exp>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<bool> {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(&*inCall) {
        Deref @ DAE::Exp::ASUB { .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_round" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$getPart" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cardinality" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Clock" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "firstTick" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "getInstanceName" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "interval" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "reinit" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "String" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { .. } if (Expression::isImpureCall(inCall.clone())? || isCallRecordConstructor(&inCall, functionTree)) => {
            true
        },
        Deref @ DAE::Exp::CALL { .. } if (Flags::getConfigBool(Flags::WFC_ADVANCED.clone())?) => {
            isSkipCase_advanced(&inCall)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outB)
}

fn isSkipCase_advanced(mut inCall: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match inCall {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, .. } => {
            true
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

fn isCallRecordConstructor(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut funcsIn: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> bool {
    let mut outIsCall: bool;
    outIsCall = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, .. } => {
                    let mut func: DAE::Function;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcsIn, path.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    func = metamodelica::Own::own(__pa0);
                    Ok(((DAEUtil::getFunctionElements(&func)?)).is_empty())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIsCall
}

fn createReturnExp(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inIndex: i32,
    mut inPrefix: &ArcStr,
    mut inComplex: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outIndex: i32;
    (outExp, outIndex) = (match &**inType {
        DAE::Type::T_REAL { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_REAL_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: DAE::T_REAL_DEFAULT().clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_INTEGER { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_INTEGER_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_STRING { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_STRING_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: DAE::T_STRING_DEFAULT().clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_BOOL { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_BOOL_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: DAE::T_BOOL_DEFAULT().clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_ENUMERATION { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: inType.clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: inType.clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_CLOCK { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: DAE::T_CLOCK_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: DAE::T_CLOCK_DEFAULT().clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_TUPLE { types: typeLst, .. } => {
            let mut i: i32;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            if inComplex {
                (expLst, i) = List::mapFold(
                    typeLst,
                    &({
                        let __pe_b2 = inPrefix.clone();
                        let __pe_b3 = inComplex;
                        move |__pe_a0, __pe_a1| createReturnExp(&__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone())
                    }),
                    inIndex,
                )?;
                value = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst });
            } else {
                let __pa0 = ::match_deref::match_deref! { match &(typeLst.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                ty = metamodelica::Own::own(__pa0);
                (value, i) = createReturnExp(&ty, inIndex, inPrefix, false)?;
            }
            (value, i)
        }
        DAE::Type::T_ARRAY { .. } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: inType.clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: inType.clone(),
            });
            (value, inIndex + 1)
        }
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { path: _ },
            ..
        } => {
            let mut r#str: ArcStr;
            let mut value: metamodelica::Ref<DAE::Exp>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inPrefix);
                __mm_s.push_str(&*intString(inIndex));
                ArcStr::from(__mm_s)
            };
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: r#str,
                identType: inType.clone(),
                subscriptLst: metamodelica::nil(),
            });
            value = metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: inType.clone(),
            });
            (value, inIndex + 1)
        }
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("  - createReturnExp failed for "));
                    __mm_s.push_str(&*TypesDump::printTypeStr(inType.clone()));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/CommonSubExpression.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok((outExp, outIndex))
}

fn createVarsForExp_onlyCSECrefs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inAccumVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = (::match_deref::match_deref! { match &(&*inExp) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => {
            inAccumVarLst
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } } if (isCSECref(metamodelica::AsArg::as_arg(&cr))) => {
            let mut cr_: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut arrayDim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            crefs = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
            outVarLst = inAccumVarLst;
            for mut cr_ in &*crefs {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(&cr_)?;
                outVarLst = metamodelica::cons(BackendVariable::createCSEArrayVar(cr_.clone(), ComponentReference::crefTypeFull(&cr_)?, arrayDim)?, outVarLst);
            }
            outVarLst
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (isCSECref(metamodelica::AsArg::as_arg(&cr)) && Expression::isArrayType(&(Expression::r#typeof(inExp.clone())?))) => {
            let mut cr_: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut arrayDim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            crefs = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
            outVarLst = inAccumVarLst;
            ty = DAEUtil::expTypeElementType(&(Expression::r#typeof(inExp.clone())?));
            for mut cr_ in &*crefs {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(&cr_)?;
                outVarLst = metamodelica::cons(BackendVariable::createCSEArrayVar(cr_, ty.clone(), arrayDim)?, outVarLst);
            }
            outVarLst
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (isCSECref(metamodelica::AsArg::as_arg(&cr))) => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            var = BackendVariable::createCSEVar(cr.clone(), Expression::r#typeof(inExp.clone())?)?;
            metamodelica::cons(var, inAccumVarLst)
        },
        Deref @ DAE::Exp::TUPLE { PR: expLst } => {
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp_onlyCSECrefs, inAccumVarLst)?;
            outVarLst
        },
        Deref @ DAE::Exp::ARRAY { array: expLst, .. } => {
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp_onlyCSECrefs, inAccumVarLst)?;
            outVarLst
        },
        Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
            metamodelica::print(literal!("This should never appear\n"));
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp_onlyCSECrefs, inAccumVarLst)?;
            outVarLst
        },
        _ => {
            inAccumVarLst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarLst)
}

fn createVarsForExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inAccumVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = (::match_deref::match_deref! { match &(&*inExp) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => {
            inAccumVarLst
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } } => {
            let mut cr_: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut arrayDim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            crefs = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
            outVarLst = inAccumVarLst;
            for mut cr_ in &*crefs {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(&cr_)?;
                outVarLst = metamodelica::cons(BackendVariable::createCSEArrayVar(cr_.clone(), ComponentReference::crefTypeFull(&cr_)?, arrayDim)?, outVarLst);
            }
            outVarLst
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } if (Expression::isArrayType(&(Expression::r#typeof(inExp.clone())?))) => {
            let mut cr_: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut arrayDim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            crefs = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
            outVarLst = inAccumVarLst;
            ty = DAEUtil::expTypeElementType(&(Expression::r#typeof(inExp.clone())?));
            for mut cr_ in &*crefs {
                let mut cr_ = cr_.clone();
                arrayDim = ComponentReferenceBasics::crefDims(&cr_)?;
                outVarLst = metamodelica::cons(BackendVariable::createCSEArrayVar(cr_, ty.clone(), arrayDim)?, outVarLst);
            }
            outVarLst
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            var = BackendVariable::createCSEVar(cr.clone(), Expression::r#typeof(inExp.clone())?)?;
            metamodelica::cons(var, inAccumVarLst)
        },
        Deref @ DAE::Exp::TUPLE { PR: expLst } => {
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp, inAccumVarLst)?;
            outVarLst
        },
        Deref @ DAE::Exp::ARRAY { array: expLst, .. } => {
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp, inAccumVarLst)?;
            outVarLst
        },
        Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp, inAccumVarLst)?;
            outVarLst
        },
        Deref @ DAE::Exp::CALL { expLst, .. } => {
            outVarLst = List::fold(metamodelica::AsArg::as_arg(&expLst), &createVarsForExp, inAccumVarLst)?;
            outVarLst
        },
        _ => {
            inAccumVarLst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarLst)
}

pub(crate) fn isCSECref(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut b: bool;
    b = (match &**cr {
        DAE::ComponentRef::CREF_IDENT { ident: s, .. } => StringUtil::startsWith(s.clone(), literal!("$cse")),
        DAE::ComponentRef::CREF_QUAL { ident: s, .. } => StringUtil::startsWith(s.clone(), literal!("$cse")),
        _ => false,
    });
    b
}

pub(crate) fn isCSEExp(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (match &**inExp {
        DAE::Exp::CREF {
            componentRef: __inExp_componentRef,
            ..
        } => isCSECref(metamodelica::AsArg::as_arg(&__inExp_componentRef)),
        _ => false,
    });
    b
}

pub(crate) fn cseBinary(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
        inDAE,
        &fnptr!(
            CSE1,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>,
            i32
        ),
        1,
    )?;
    Ok(outDAE)
}

fn CSE1(
    mut inSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inIndex: i32,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    i32,
) {
    let mut outSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outIndex: i32;
    (outSystem, outIndex) = ({
        let mut index: i32 = inIndex;
        'mc: {
            let __mc_input = inSystem.clone();
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    syst @ Deref @ BackendDAE::EqSystem { orderedVars, orderedEqs, .. } => {
                        let mut varList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                        let mut eqList: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut HT: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
                        let mut HT2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                        let mut HT3: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                        let mut syst = (*syst).clone();
                        let mut orderedEqs = (*orderedEqs).clone();
                        HT = HashTableExpToExp::emptyHashTableSized(49999);
                        HT2 = HashTableExpToIndex::emptyHashTableSized(49999);
                        HT3 = HashTableExpToIndex::emptyHashTableSized(49999);
                        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                            metamodelica::print(literal!("collect statistics\n========================================\n"));
                        }
                        (HT, HT2, index) = BackendEquation::traverseEquationArray(orderedEqs.clone(), &createStatistics, (HT.clone(), HT2.clone(), index))?;
                        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                            metamodelica::print(literal!("\nstart substitution\n========================================\n"));
                        }
                        let (__pa0, (__pa1, __pa2, _, __pa3, __pa4)) = BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), &substituteCSE, (HT.clone(), HT2.clone(), HT3.clone(), metamodelica::nil(), metamodelica::nil()))?;
                        orderedEqs = metamodelica::Own::own(__pa0);
                        HT = metamodelica::Own::own(__pa1);
                        HT2 = metamodelica::Own::own(__pa2);
                        eqList = metamodelica::Own::own(__pa3);
                        varList = metamodelica::Own::own(__pa4);
                        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                            metamodelica::print(literal!("\n"));
                        }
                        assign_field!(
                            syst.orderedEqs = BackendEquation::addList(&eqList, orderedEqs.clone())?,
                            syst.orderedVars = BackendVariable::addVars(&varList, orderedVars.clone())?
                        );
                        if Flags::isSet(Flags::DUMP_CSE.clone())? {
                            BackendDump::dumpVariables(&syst.orderedVars, &(literal!("########### Updated Variable List ###########")))?;
                            BackendDump::dumpEquationArray(syst.orderedEqs.clone(), &(literal!("########### Updated Equation List ###########")))?;
                        }
                        Ok((BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), index))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok((inSystem.clone(), inIndex))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        }
    });
    (outSystem, outShared, outIndex)
}

fn substituteCSE(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableExpToExp::FuncHashCref,
                HashTableExpToExp::FuncCrefEqual,
                HashTableExpToExp::FuncCrefStr,
                HashTableExpToExp::FuncExpStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    (outEq, outTuple) = (match &*inEq {
        BackendDAE::Equation::ALGORITHM { .. } => (inEq, inTuple),
        BackendDAE::Equation::WHEN_EQUATION { .. } => (inEq, inTuple),
        BackendDAE::Equation::IF_EQUATION { .. } => (inEq, inTuple),
        _ => {
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            let mut tpl: (
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                    ),
                    i32,
                    (
                        HashTableExpToExp::FuncHashCref,
                        HashTableExpToExp::FuncCrefEqual,
                        HashTableExpToExp::FuncCrefStr,
                        HashTableExpToExp::FuncExpStr,
                    ),
                ),
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    ),
                    i32,
                    (
                        HashTableExpToIndex::FuncHashCref,
                        HashTableExpToIndex::FuncCrefEqual,
                        HashTableExpToIndex::FuncCrefStr,
                        HashTableExpToIndex::FuncExpStr,
                    ),
                ),
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    ),
                    i32,
                    (
                        HashTableExpToIndex::FuncHashCref,
                        HashTableExpToIndex::FuncCrefEqual,
                        HashTableExpToIndex::FuncCrefStr,
                        HashTableExpToIndex::FuncExpStr,
                    ),
                ),
                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            );
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("traverse "));
                    __mm_s.push_str(&*BackendDump::equationString(&inEq)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(
                inEq.clone(),
                (std::sync::Arc::new(substituteCSE1)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        (
                                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<
                                                    Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>,
                                                >,
                                            ),
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::Exp>,
                                                            metamodelica::Ref<DAE::Exp>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                            ),
                                        ),
                                        (
                                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            ),
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::Exp>,
                                                            metamodelica::Ref<DAE::Exp>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                            ),
                                        ),
                                        (
                                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            ),
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::Exp>,
                                                            metamodelica::Ref<DAE::Exp>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                            ),
                                        ),
                                        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                    ),
                                    metamodelica::Ref<DAE::ElementSource>,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        (
                                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<
                                                    Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>,
                                                >,
                                            ),
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::Exp>,
                                                            metamodelica::Ref<DAE::Exp>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                            ),
                                        ),
                                        (
                                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            ),
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::Exp>,
                                                            metamodelica::Ref<DAE::Exp>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                            ),
                                        ),
                                        (
                                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            (
                                                i32,
                                                i32,
                                                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                            ),
                                            i32,
                                            (
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(
                                                            metamodelica::Ref<DAE::Exp>,
                                                            metamodelica::Ref<DAE::Exp>,
                                                        )
                                                            -> Result<bool>
                                                        + 'static,
                                                >,
                                                Arc<
                                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                        + 'static,
                                                >,
                                                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                            ),
                                        ),
                                        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                    ),
                                    metamodelica::Ref<DAE::ElementSource>,
                                ),
                            )> + 'static,
                    >),
                (inTuple, BackendEquation::equationSource(&inEq)?),
            )?;
            eq = metamodelica::Own::own(__pa0);
            tpl = metamodelica::Own::own(__pa1);
            (eq, tpl)
        }
    });
    Ok((outEq, outTuple))
}

fn substituteCSE1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        ),
        metamodelica::Ref<DAE::ElementSource>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        ),
        metamodelica::Ref<DAE::ElementSource>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTableExpToExp::FuncHashCref,
                    HashTableExpToExp::FuncCrefEqual,
                    HashTableExpToExp::FuncCrefStr,
                    HashTableExpToExp::FuncExpStr,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    HashTableExpToIndex::FuncHashCref,
                    HashTableExpToIndex::FuncCrefEqual,
                    HashTableExpToIndex::FuncCrefStr,
                    HashTableExpToIndex::FuncExpStr,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    HashTableExpToIndex::FuncHashCref,
                    HashTableExpToIndex::FuncCrefEqual,
                    HashTableExpToIndex::FuncCrefStr,
                    HashTableExpToIndex::FuncExpStr,
                ),
            ),
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        ),
        metamodelica::Ref<DAE::ElementSource>,
    );
    (outExp, outTuple) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            substituteCSE_main,
            metamodelica::Ref<DAE::Exp>,
            (
                (
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>
                        ),
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        metamodelica::Ref<DAE::Exp>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>
                        )
                    ),
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>
                        ),
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        metamodelica::Ref<DAE::Exp>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>
                        )
                    ),
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>
                        ),
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::Exp>,
                                        metamodelica::Ref<DAE::Exp>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>
                        )
                    ),
                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
                ),
                metamodelica::Ref<DAE::ElementSource>
            )
        ),
        inTuple,
    )?;
    Ok((outExp, outTuple))
}

fn substituteCSE_main(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        ),
        metamodelica::Ref<DAE::ElementSource>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                    Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                            + 'static,
                    >,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                ),
            ),
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        ),
        metamodelica::Ref<DAE::ElementSource>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTuple: (
        (
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTableExpToExp::FuncHashCref,
                    HashTableExpToExp::FuncCrefEqual,
                    HashTableExpToExp::FuncCrefStr,
                    HashTableExpToExp::FuncExpStr,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    HashTableExpToIndex::FuncHashCref,
                    HashTableExpToIndex::FuncCrefEqual,
                    HashTableExpToIndex::FuncCrefStr,
                    HashTableExpToIndex::FuncExpStr,
                ),
            ),
            (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                ),
                i32,
                (
                    HashTableExpToIndex::FuncHashCref,
                    HashTableExpToIndex::FuncCrefEqual,
                    HashTableExpToIndex::FuncCrefStr,
                    HashTableExpToIndex::FuncExpStr,
                ),
            ),
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        ),
        metamodelica::Ref<DAE::ElementSource>,
    );
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (&*inExp, &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { .. }, ((HT, HT2, HT3, eqLst, varLst), source)) => {
                    let mut value: metamodelica::Ref<DAE::Exp>;
                    let mut counter: i32;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut HT3 = (*HT3).clone();
                    let mut eqLst = (*eqLst).clone();
                    let mut varLst = (*varLst).clone();
                    value = BaseHashTable::get(inExp.clone(), &(HT.clone()))?;
                    counter = BaseHashTable::get(value.clone(), &(HT2.clone()))?;
                    let true = (intGt(counter, 1)) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  - substitute cse binary: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" (counter: ")); __mm_s.push_str(&*intString(counter)); __mm_s.push_str(&*literal!(", id: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(value.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                    }
                    if !(BaseHashTable::hasKey(value.clone(), &(HT3.clone()))?) {
                        HT3 = BaseHashTable::add((value.clone(), 1), HT3.clone())?;
                        varLst = createVarsForExp_onlyCSECrefs(value.clone(), varLst.clone())?;
                        eq = BackendEquation::generateEquation(value.clone(), inExp.clone(), source.clone(), BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone())?;
                        eqLst = metamodelica::cons(eq.clone(), eqLst.clone());
                    }
                    Ok((value.clone(), true, ((HT.clone(), HT2.clone(), HT3.clone(), eqLst.clone(), varLst.clone()), source.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTuple)
}

fn createStatistics(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        i32,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        i32,
    ),
)> {
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableExpToExp::FuncHashCref,
                HashTableExpToExp::FuncCrefEqual,
                HashTableExpToExp::FuncCrefStr,
                HashTableExpToExp::FuncExpStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        i32,
    );
    (outEq, outTuple) = (match &*inEq {
        BackendDAE::Equation::ALGORITHM { .. } => (inEq, inTuple),
        BackendDAE::Equation::WHEN_EQUATION { .. } => (inEq, inTuple),
        BackendDAE::Equation::IF_EQUATION { .. } => (inEq, inTuple),
        _ => {
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            let mut tpl: (
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
                    ),
                    i32,
                    (
                        HashTableExpToExp::FuncHashCref,
                        HashTableExpToExp::FuncCrefEqual,
                        HashTableExpToExp::FuncCrefStr,
                        HashTableExpToExp::FuncExpStr,
                    ),
                ),
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    ),
                    i32,
                    (
                        HashTableExpToIndex::FuncHashCref,
                        HashTableExpToIndex::FuncCrefEqual,
                        HashTableExpToIndex::FuncCrefStr,
                        HashTableExpToIndex::FuncExpStr,
                    ),
                ),
                i32,
            );
            if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("traverse "));
                    __mm_s.push_str(&*BackendDump::equationString(&inEq)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (eq, tpl) = BackendEquation::traverseExpsOfEquation(
                inEq,
                (std::sync::Arc::new(createStatistics1)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<
                                                Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>,
                                            >,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                        ),
                                    ),
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    i32,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<
                                                Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>,
                                            >,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                        ),
                                    ),
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    i32,
                                ),
                            )> + 'static,
                    >),
                inTuple,
            )?;
            (eq, tpl)
        }
    });
    Ok((outEq, outTuple))
}

fn createStatistics1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        i32,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        i32,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableExpToExp::FuncHashCref,
                HashTableExpToExp::FuncCrefEqual,
                HashTableExpToExp::FuncCrefStr,
                HashTableExpToExp::FuncExpStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        i32,
    );
    (outExp, outTuple) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            createStatistics_main,
            metamodelica::Ref<DAE::Exp>,
            (
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                                + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>
                    )
                ),
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                                + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>
                    )
                ),
                i32
            )
        ),
        inTuple,
    )?;
    Ok((outExp, outTuple))
}

fn createStatistics_main(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        i32,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        i32,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTuple: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
            ),
            i32,
            (
                HashTableExpToExp::FuncHashCref,
                HashTableExpToExp::FuncCrefEqual,
                HashTableExpToExp::FuncCrefStr,
                HashTableExpToExp::FuncExpStr,
            ),
        ),
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        i32,
    );
    (outExp, cont, outTuple) = 'mc: {
        let __mc_input = (&*inExp, &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1, operator: op, exp2 }, (HT, HT2, i)) => {
                    let mut value: metamodelica::Ref<DAE::Exp>;
                    let mut counter: i32;
                    let mut HT = (*HT).clone();
                    let mut HT2 = (*HT2).clone();
                    let mut i = (*i).clone();
                    if checkOp(metamodelica::AsArg::as_arg(&op)) {
                        if BaseHashTable::hasKey(inExp.clone(), &(HT.clone()))? {
                            value = BaseHashTable::get(inExp.clone(), &(HT.clone()))?;
                            counter = BaseHashTable::get(value.clone(), &(HT2.clone()))? + 1;
                            BaseHashTable::update((value.clone(), counter), &(HT2.clone()))?;
                            if isCommutative(metamodelica::AsArg::as_arg(&op)) {
                                        value = BaseHashTable::get(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: op.clone(), exp2: exp1.clone() }), &(HT.clone()))?;
                                        BaseHashTable::update((value.clone(), counter), &(HT2.clone()))?;
                            }
                        } else {
                            (value, i) = createReturnExp(&(Expression::r#typeof(inExp.clone())?), i.clone(), &(literal!("$cseb")), true)?;
                            counter = 1;
                            HT = BaseHashTable::add((inExp.clone(), value.clone()), HT.clone())?;
                            HT2 = BaseHashTable::add((value.clone(), counter), HT2.clone())?;
                            if isCommutative(metamodelica::AsArg::as_arg(&op)) {
                                        HT = BaseHashTable::add((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: op.clone(), exp2: exp1.clone() }), value.clone()), HT.clone())?;
                            }
                        }
                        if Flags::isSet(Flags::DUMP_CSE_VERBOSE.clone())? {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  - cse binary expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" (counter: ")); __mm_s.push_str(&*intString(counter)); __mm_s.push_str(&*literal!(", id: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(value.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) });
                        }
                    }
                    Ok((inExp.clone(), true, (HT.clone(), HT2.clone(), i.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTuple)
}

fn isCommutative(mut inOp: &DAE::Operator) -> bool {
    let mut outCommutative: bool;
    outCommutative = (match inOp.clone() {
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::ADD { .. } => true,
        _ => false,
    });
    outCommutative
}

fn checkOp(mut inOp: &DAE::Operator) -> bool {
    let mut outB: bool;
    outB = (match inOp.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::SUB { .. } => true,
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::DIV { .. } => true,
        DAE::Operator::POW { .. } => true,
        DAE::Operator::UMINUS { .. } => true,
        _ => false,
    });
    outB
}

// =============================================================================
// Common Sub Expressions
//
// =============================================================================
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum CommonSubExp {
    ASSIGNMENT_CSE {
        eqIdcs: metamodelica::List<i32>,
        sharedVars: metamodelica::List<i32>,
        aliasVars: metamodelica::List<i32>,
    },
    SHORTCUT_CSE {
        eqIdcs: metamodelica::List<i32>,
        sharedVar: i32,
    },
}
impl metamodelica::gc::MMTrace for CommonSubExp {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            CommonSubExp::ASSIGNMENT_CSE {
                eqIdcs,
                sharedVars,
                aliasVars,
            } => {
                metamodelica::gc::MMTrace::mm_accept(eqIdcs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sharedVars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(aliasVars, __mmv)?;
                Ok(())
            }
            CommonSubExp::SHORTCUT_CSE { eqIdcs, sharedVar } => {
                metamodelica::gc::MMTrace::mm_accept(eqIdcs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sharedVar, __mmv)?;
                Ok(())
            }
        }
    }
}
pub(crate) use self::CommonSubExp::{ASSIGNMENT_CSE, SHORTCUT_CSE};

pub(crate) fn commonSubExpressionReplacement(
    mut daeIn: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut daeOut: metamodelica::Ref<BackendDAE::BackendDAE>;
    daeOut = BackendDAEUtil::mapEqSystem(
        daeIn,
        &fnptr!(
            commonSubExpression,
            metamodelica::Ref<BackendDAE::EqSystem>,
            metamodelica::Ref<BackendDAE::Shared>
        ),
    )?;
    Ok(daeOut)
}

fn commonSubExpression(
    mut sysIn: metamodelica::Ref<BackendDAE::EqSystem>,
    mut sharedIn: metamodelica::Ref<BackendDAE::Shared>,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut sysOut: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut sharedOut: metamodelica::Ref<BackendDAE::Shared>;
    (sysOut, sharedOut) = 'mc: {
        let __mc_input = (&*sysIn, &*sharedIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. }, Deref @ BackendDAE::Shared { functionTree, .. }) => {
                    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
                    let mut m: metamodelica::Array<metamodelica::List<i32>>;
                    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
                    let mut cseLst: metamodelica::List<CommonSubExp>;
                    let mut isInitial: bool;
                    isInitial = BackendDAEUtil::isInitializationDAE(&sharedIn);
                    (_, m, mT) = BackendDAEUtil::getAdjacencyMatrix(sysIn.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), isInitial)?;
                    cseLst = commonSubExpressionFind(m.clone(), mT.clone(), vars.clone(), eqs.clone(), isInitial);
                    syst = commonSubExpressionUpdate(cseLst.clone(), m.clone(), mT.clone(), sysIn.clone())?;
                    GCExt::free(m.clone());
                    GCExt::free(mT.clone());
                    assign_field!(syst.orderedEqs = eqs.clone());
                    Ok((syst.clone(), sharedIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((sysIn.clone(), sharedIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (sysOut, sharedOut)
}

fn commonSubExpressionFind(
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut varsIn: BackendDAE::Variables,
    mut eqsIn: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut isInitial: bool,
) -> metamodelica::List<CommonSubExp> {
    let mut cseOut: metamodelica::List<CommonSubExp>;
    let mut eqIdcs: metamodelica::List<i32>;
    let mut varIdcs: metamodelica::List<i32>;
    let mut lengthLst: metamodelica::List<i32>;
    let mut range: metamodelica::List<i32>;
    let mut partitions: metamodelica::List<metamodelica::List<i32>>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut eqSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut cseLst2: metamodelica::List<CommonSubExp>;
    let mut cseLst3: metamodelica::List<CommonSubExp>;
    let mut shortenPathsCSE: metamodelica::List<CommonSubExp>;
    let mut varIdcsSet: metamodelica::Ref<AvlSetInt::Tree>;
    match '__try0: {
        range = List::intRange(metamodelica::arrayLength(mIn.clone()));
        lengthLst = unwrap_break_err!(List::mapArray(mIn.clone(), &fnptr!(listLength, _)), '__try0);
        (_, eqIdcs) =
            unwrap_break_err!(List::filter1OnTrueSync(&lengthLst, &fnptr!(intEq, i32, i32), 2, range.clone()), '__try0);
        (eqLst, eqIdcs) = unwrap_break_err!(List::filterOnTrueSync(&(unwrap_break_err!(BackendEquation::getList(eqIdcs.clone(), eqsIn.clone()), '__try0)), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendEquation::isNotAlgorithm(&__a0)) }, eqIdcs.clone()), '__try0);
        eqs = unwrap_break_err!(BackendEquation::listEquation(&eqLst), '__try0);
        varIdcs = unwrap_break_err!(UnorderedSet::unique_list(unwrap_break_err!(List::flatten(unwrap_break_err!(List::map1(eqIdcs.clone(), &Array::getIndexFirst, mIn.clone()), '__try0)), '__try0), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>)), '__try0);
        varLst = unwrap_break_err!(List::map1(varIdcs.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), varsIn.clone()), '__try0);
        vars = unwrap_break_err!(BackendVariable::listVar1(&varLst), '__try0);
        eqSys = BackendDAEUtil::createEqSystem(
            vars.clone(),
            eqs.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        (_, m, mT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrix(eqSys.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, isInitial), '__try0);
        partitions = unwrap_break_err!(ResolveLoops::partitionBipartiteGraph(m.clone(), mT.clone()), '__try0);
        partitions = unwrap_break_err!(List::filterOnFalse(partitions.clone(), &fnptr!(listEmpty, _)), '__try0);
        cseLst2 = unwrap_break_err!(List::fold(&partitions, &({ let __pe_b1 = m.clone(); let __pe_b2 = mT.clone(); let __pe_b3 = vars.clone(); let __pe_b4 = eqs.clone(); let __pe_b5 = eqIdcs.clone(); let __pe_b6 = varIdcs.clone(); move |__pe_a0, __pe_a7| Ok(getCSE2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3, __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7)) }), metamodelica::nil()), '__try0);
        shortenPathsCSE = shortenPaths(
            &partitions,
            m.clone(),
            mT.clone(),
            vars.clone(),
            eqs.clone(),
            metamodelica::arrayFromVec(eqIdcs.clone().into_iter().cloned().collect()),
            metamodelica::arrayFromVec(varIdcs.clone().into_iter().cloned().collect()),
            metamodelica::nil(),
            isInitial,
        );
        (_, eqIdcs) =
            unwrap_break_err!(List::filter1OnTrueSync(&lengthLst, &fnptr!(intEq, i32, i32), 3, range.clone()), '__try0);
        (eqLst, eqIdcs) = unwrap_break_err!(List::filterOnTrueSync(&(unwrap_break_err!(BackendEquation::getList(eqIdcs.clone(), eqsIn.clone()), '__try0)), &move |__a0: metamodelica::Ref<BackendDAE::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(BackendEquation::isNotAlgorithm(&__a0)) }, eqIdcs.clone()), '__try0);
        eqs = unwrap_break_err!(BackendEquation::listEquation(&eqLst), '__try0);
        varIdcsSet = crate::AvlSetInt::Tree::interned_EMPTY();
        for mut eq in &*eqIdcs {
            varIdcsSet = unwrap_break_err!(AvlSetInt::addList(varIdcsSet.clone(), &(unwrap_break_err!(metamodelica::arrayGet(mIn.clone(), eq.clone()), '__try0))), '__try0);
        }
        varIdcs = AvlSetInt::listKeysReverse(&varIdcsSet, metamodelica::nil());
        varLst = unwrap_break_err!(List::map1(varIdcs.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), varsIn.clone()), '__try0);
        vars = unwrap_break_err!(BackendVariable::listVar1(&varLst), '__try0);
        eqSys = BackendDAEUtil::createEqSystem(
            vars.clone(),
            eqs.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            BackendEquation::emptyEqns(),
        );
        (_, m, mT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrix(eqSys.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, isInitial), '__try0);
        partitions = unwrap_break_err!(ResolveLoops::partitionBipartiteGraph(m.clone(), mT.clone()), '__try0);
        cseLst3 = unwrap_break_err!(List::fold(&partitions, &({ let __pe_b1 = m.clone(); let __pe_b2 = mT.clone(); let __pe_b3 = vars.clone(); let __pe_b4 = eqs.clone(); let __pe_b5 = eqIdcs.clone(); let __pe_b6 = varIdcs.clone(); move |__pe_a0, __pe_a7| Ok(getCSE3(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3, __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7)) }), metamodelica::nil()), '__try0);
        cseOut = listAppend(cseLst2.clone(), listAppend(cseLst3.clone(), shortenPathsCSE.clone()));
        Ok::<_, &'static str>((cseOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            cseOut = __try0_o0;
        }
        Err(_) => {
            cseOut = metamodelica::nil();
        }
    }
    cseOut
}

fn shortenPaths(
    mut allPartitions: &metamodelica::List<metamodelica::List<i32>>,
    mut mIn: metamodelica::Array<metamodelica::List<i32>>,
    mut mTIn: metamodelica::Array<metamodelica::List<i32>>,
    mut allVars: BackendDAE::Variables,
    mut allEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut eqMap: metamodelica::Array<i32>,
    mut varMap: metamodelica::Array<i32>,
    mut cseIn: metamodelica::List<CommonSubExp>,
    mut isInitial: bool,
) -> metamodelica::List<CommonSubExp> {
    let mut cseOut: metamodelica::List<CommonSubExp>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut eqSys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut pathVars: BackendDAE::Variables;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut numVars: i32;
    let mut varIdx: i32;
    let mut pathVarIdxMap: metamodelica::Array<i32>;
    let mut partition: metamodelica::List<i32> = metamodelica::nil();
    let mut adjEqs: metamodelica::List<i32>;
    let mut pathVarIdcs: metamodelica::List<i32>;
    let mut cses: metamodelica::List<CommonSubExp>;
    match '__try0: {
        numVars = BackendVariable::varsSize(&allVars);
        (_, pathVarIdcs) = unwrap_break_err!(List::filter1OnTrueSync(&(unwrap_break_err!(List::mapArray(mTIn.clone(), &fnptr!(listLength, _)), '__try0)), &fnptr!(intEq, i32, i32), 2, List::intRange(numVars)), '__try0);
        pathVars = unwrap_break_err!(BackendVariable::listVar1(&(unwrap_break_err!(List::map1(pathVarIdcs.clone(), &move |__a0: i32, __a1: BackendDAE::Variables| BackendVariable::getVarAtIndexFirst(__a0, &__a1), allVars.clone()), '__try0))), '__try0);
        pathVarIdxMap = metamodelica::arrayFromVec(
            unwrap_break_err!(List::map1(pathVarIdcs.clone(), &Array::getIndexFirst, varMap.clone()), '__try0)
                .into_iter()
                .cloned()
                .collect(),
        );
        cses = cseIn.clone();
        if BackendVariable::varsSize(&pathVars) > 0 {
            for mut partition in &**allPartitions {
                let mut partition = partition.clone();
                eqLst = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                    for mut i in (partition.clone()).into_iter().cloned() {
                        let __x = unwrap_break_err!(BackendEquation::get(allEqs.clone(), i.clone()), '__try0);
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                eqs = unwrap_break_err!(BackendEquation::listEquation(&eqLst), '__try0);
                eqSys = BackendDAEUtil::createEqSystem(
                    pathVars.clone(),
                    eqs.clone(),
                    metamodelica::nil(),
                    openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                    BackendEquation::emptyEqns(),
                );
                (_, m, mT) = unwrap_break_err!(BackendDAEUtil::getAdjacencyMatrix(eqSys.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, None, isInitial), '__try0);
                for mut idx in 1..=metamodelica::arrayLength(mT.clone()) {
                    adjEqs = metamodelica::Dangerous::arrayGetNoBoundsChecking(mT.clone(), idx);
                    if ((adjEqs).len() as i32) == 2 {
                        adjEqs = ({
                            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                            for mut eq in (adjEqs.clone()).into_iter().cloned() {
                                let __x = unwrap_break_err!(metamodelica::arrayGet(eqMap.clone(), unwrap_break_err!((partition).get(eq.clone()), '__try0)), '__try0);
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        });
                        varIdx = unwrap_break_err!(metamodelica::arrayGet(pathVarIdxMap.clone(), idx), '__try0);
                        cses = metamodelica::cons(
                            CommonSubExp::SHORTCUT_CSE {
                                eqIdcs: adjEqs.clone(),
                                sharedVar: varIdx,
                            },
                            cses.clone(),
                        );
                    }
                }
                GCExt::free(m.clone());
                GCExt::free(mT.clone());
            }
        }
        cseOut = cses.clone();
        Ok::<_, &'static str>((cseOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            cseOut = __try0_o0;
        }
        Err(_) => {
            cseOut = cseIn.clone();
        }
    }
    cseOut
}

fn getCSE2(
    mut partition: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut vars: &BackendDAE::Variables,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut eqMap: metamodelica::List<i32>,
    mut varMap: metamodelica::List<i32>,
    mut cseIn: metamodelica::List<CommonSubExp>,
) -> metamodelica::List<CommonSubExp> {
    let mut cseOut: metamodelica::List<CommonSubExp>;
    cseOut = 'mc: {
        let __mc_input = &*partition;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: eqIdx1, tail: Deref @ metamodelica::ListNode::Cons { head: eqIdx2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut sharedVarIdx: i32;
                    let mut varIdx1: i32;
                    let mut varIdx2: i32;
                    let mut varIdcs1: metamodelica::List<i32>;
                    let mut varIdcs2: metamodelica::List<i32>;
                    let mut sharedVarIdcs: metamodelica::List<i32>;
                    let mut eqIdcs: metamodelica::List<i32>;
                    let mut eq1: metamodelica::Ref<BackendDAE::Equation>;
                    let mut eq2: metamodelica::Ref<BackendDAE::Equation>;
                    let mut var1: metamodelica::Ref<BackendDAE::Var>;
                    let mut var2: metamodelica::Ref<BackendDAE::Var>;
                    let mut varExp1: metamodelica::Ref<DAE::Exp>;
                    let mut varExp2: metamodelica::Ref<DAE::Exp>;
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs1: metamodelica::Ref<DAE::Exp>;
                    let mut rhs2: metamodelica::Ref<DAE::Exp>;
                    varIdcs1 = metamodelica::arrayGet(m.clone(), eqIdx1.clone())?;
                    varIdcs2 = metamodelica::arrayGet(m.clone(), eqIdx2.clone())?;
                    (sharedVarIdcs, varIdcs1, varIdcs2) = List::intersection1OnTrue(varIdcs1.clone(), varIdcs2.clone(), &fnptr!(intEq, i32, i32))?;
                    let __pa0 = ::match_deref::match_deref! { match &(varIdcs1.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varIdx1 = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(varIdcs2.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varIdx2 = metamodelica::Own::own(__pa2);
                    let __pa4 = ::match_deref::match_deref! { match &(sharedVarIdcs.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil } => __pa4.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    sharedVarIdx = metamodelica::Own::own(__pa4);
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(BackendEquation::getList(partition.clone(), eqs.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eq1 = metamodelica::Own::own(__pa6);
                    eq2 = metamodelica::Own::own(__pa7);
                    BackendVariable::getVarAt(vars, sharedVarIdx)?;
                    var1 = BackendVariable::getVarAt(vars, varIdx1)?;
                    var2 = BackendVariable::getVarAt(vars, varIdx2)?;
                    varExp1 = BackendVariable::varExp(&var1)?;
                    varExp2 = BackendVariable::varExp(&var2)?;
                    let (__pa9, __pa10) = ::match_deref::match_deref! { match &(eq1.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa9, scalar: __pa10, .. } => (__pa9.clone(), __pa10.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs = metamodelica::Own::own(__pa9);
                    rhs1 = metamodelica::Own::own(__pa10);
                    (rhs1, _) = ExpressionSolve::solve(lhs.clone(), rhs1.clone(), varExp1.clone(), None)?;
                    let (__pa11, __pa12) = ::match_deref::match_deref! { match &(eq2.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa11, scalar: __pa12, .. } => (__pa11.clone(), __pa12.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs = metamodelica::Own::own(__pa11);
                    rhs2 = metamodelica::Own::own(__pa12);
                    (rhs2, _) = ExpressionSolve::solve(lhs.clone(), rhs2.clone(), varExp2.clone(), None)?;
                    let true = (ExpressionBasics::expEqual(&rhs1, rhs2.clone())?) else { return Err("pattern mismatch") };
                    sharedVarIdcs = List::map1(sharedVarIdcs.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), varMap.clone())?;
                    varIdcs2 = listAppend(varIdcs1.clone(), varIdcs2.clone());
                    varIdcs2 = List::map1(varIdcs2.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), varMap.clone())?;
                    eqIdcs = List::map1(partition.clone(), &move |__a0: i32, __a1: _| List::getIndexFirst(__a0, &__a1), eqMap.clone())?;
                    Ok(metamodelica::cons(CommonSubExp::ASSIGNMENT_CSE { eqIdcs: eqIdcs.clone(), sharedVars: sharedVarIdcs.clone(), aliasVars: varIdcs2.clone() }, cseIn.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cseIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    cseOut
}

fn getCSE3(
    mut partition: metamodelica::List<i32>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut vars: &BackendDAE::Variables,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut eqMap: metamodelica::List<i32>,
    mut varMap: metamodelica::List<i32>,
    mut cseIn: metamodelica::List<CommonSubExp>,
) -> metamodelica::List<CommonSubExp> {
    let mut cseOut: metamodelica::List<CommonSubExp>;
    cseOut = 'mc: {
        let __mc_input = &*cseIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            let mut eqIdx1: i32;
                            let mut eqIdx2: i32;
                            let mut varIdx1: i32;
                            let mut varIdx2: i32;
                            let mut varIdcs1: metamodelica::List<i32>;
                            let mut varIdcs2: metamodelica::List<i32>;
                            let mut sharedVarIdcs: metamodelica::List<i32>;
                            let mut eqIdcs: metamodelica::List<i32>;
                            let mut loop1: metamodelica::List<i32> = metamodelica::nil();
                            let mut loops: metamodelica::List<metamodelica::List<i32>>;
                            let mut eq1: metamodelica::Ref<BackendDAE::Equation>;
                            let mut eq2: metamodelica::Ref<BackendDAE::Equation>;
                            let mut var1: metamodelica::Ref<BackendDAE::Var>;
                            let mut var2: metamodelica::Ref<BackendDAE::Var>;
                            let mut varExp1: metamodelica::Ref<DAE::Exp>;
                            let mut varExp2: metamodelica::Ref<DAE::Exp>;
                            let mut lhs: metamodelica::Ref<DAE::Exp>;
                            let mut rhs1: metamodelica::Ref<DAE::Exp>;
                            let mut rhs2: metamodelica::Ref<DAE::Exp>;
                            let mut varMapArr: metamodelica::Array<i32>;
                            let mut eqMapArr: metamodelica::Array<i32>;
                            let mut cseLst: metamodelica::List<CommonSubExp>;
                            (loops, _, _, _) = ResolveLoops::resolveLoops_findLoops(&(list![partition.clone()]), m.clone(), mT.clone(), false);
                            cseLst = cseIn.clone();
                            for mut loop1 in &*loops {
                                let mut loop1 = loop1.clone();
                                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(loop1.clone()) {
                                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                                    _ => return Err("pattern mismatch"),
                                } };
                                eqIdx1 = metamodelica::Own::own(__pa0);
                                eqIdx2 = metamodelica::Own::own(__pa1);
                                varIdcs1 = metamodelica::arrayGet(m.clone(), eqIdx1)?;
                                varIdcs2 = metamodelica::arrayGet(m.clone(), eqIdx2)?;
                                (sharedVarIdcs, varIdcs1, varIdcs2) = List::intersection1OnTrue(varIdcs1.clone(), varIdcs2.clone(), &fnptr!(intEq, i32, i32))?;
                                let __pa3 = ::match_deref::match_deref! { match &(varIdcs1.clone()) {
                                    Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
                                    _ => return Err("pattern mismatch"),
                                } };
                                varIdx1 = metamodelica::Own::own(__pa3);
                                let __pa5 = ::match_deref::match_deref! { match &(varIdcs2.clone()) {
                                    Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } => __pa5.clone(),
                                    _ => return Err("pattern mismatch"),
                                } };
                                varIdx2 = metamodelica::Own::own(__pa5);
                                let (__pa7, __pa8) = ::match_deref::match_deref! { match &(BackendEquation::getList(loop1.clone(), eqs.clone())?) {
                                    Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa7.clone(), __pa8.clone()),
                                    _ => return Err("pattern mismatch"),
                                } };
                                eq1 = metamodelica::Own::own(__pa7);
                                eq2 = metamodelica::Own::own(__pa8);
                                var1 = BackendVariable::getVarAt(vars, varIdx1)?;
                                var2 = BackendVariable::getVarAt(vars, varIdx2)?;
                                varExp1 = BackendVariable::varExp(&var1)?;
                                varExp2 = BackendVariable::varExp(&var2)?;
                                let (__pa10, __pa11) = ::match_deref::match_deref! { match &(eq1.clone()) {
                                    Deref @ BackendDAE::Equation::EQUATION { exp: __pa10, scalar: __pa11, .. } => (__pa10.clone(), __pa11.clone()),
                                    _ => return Err("pattern mismatch"),
                                } };
                                lhs = metamodelica::Own::own(__pa10);
                                rhs1 = metamodelica::Own::own(__pa11);
                                (rhs1, _) = ExpressionSolve::solve(lhs.clone(), rhs1.clone(), varExp1.clone(), None)?;
                                let (__pa12, __pa13) = ::match_deref::match_deref! { match &(eq2.clone()) {
                                    Deref @ BackendDAE::Equation::EQUATION { exp: __pa12, scalar: __pa13, .. } => (__pa12.clone(), __pa13.clone()),
                                    _ => return Err("pattern mismatch"),
                                } };
                                lhs = metamodelica::Own::own(__pa12);
                                rhs2 = metamodelica::Own::own(__pa13);
                                (rhs2, _) = ExpressionSolve::solve(lhs.clone(), rhs2.clone(), varExp2.clone(), None)?;
                                if ExpressionBasics::expEqual(&rhs1, rhs2.clone())? {
                                    eqMapArr = metamodelica::arrayFromVec(eqMap.clone().into_iter().cloned().collect());
                                    varMapArr = metamodelica::arrayFromVec(varMap.clone().into_iter().cloned().collect());
                                    sharedVarIdcs = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (sharedVarIdcs.clone()).into_iter().cloned() {
                            let __x = metamodelica::arrayGet(varMapArr.clone(), i.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                    varIdcs2 = listAppend(varIdcs1.clone(), varIdcs2.clone());
                                    varIdcs2 = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (varIdcs2.clone()).into_iter().cloned() {
                            let __x = metamodelica::arrayGet(varMapArr.clone(), i.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                    eqIdcs = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (loop1.clone()).into_iter().cloned() {
                            let __x = metamodelica::arrayGet(eqMapArr.clone(), i.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                    GCExt::free(eqMapArr.clone());
                                    GCExt::free(varMapArr.clone());
                                    cseLst = metamodelica::cons(CommonSubExp::ASSIGNMENT_CSE { eqIdcs: eqIdcs.clone(), sharedVars: sharedVarIdcs.clone(), aliasVars: varIdcs2.clone() }, cseLst.clone());
                                }
                            }
                            Ok(cseLst.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(cseIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    cseOut
}

fn commonSubExpressionUpdate(
    mut tplsIn: metamodelica::List<CommonSubExp>,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut sysIn: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((tplsIn, sysIn.clone())) {
            (Deref @ metamodelica::ListNode::Nil, syst @ Deref @ BackendDAE::EqSystem { .. }) => {
                return Ok(BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)))
            },
            (Deref @ metamodelica::ListNode::Cons { head: CommonSubExp::ASSIGNMENT_CSE { eqIdcs: Deref @ metamodelica::ListNode::Cons { head: eqIdx1, tail: Deref @ metamodelica::ListNode::Cons { head: eqIdx2, tail: Deref @ metamodelica::ListNode::Nil } }, aliasVars: Deref @ metamodelica::ListNode::Cons { head: varIdx1, tail: Deref @ metamodelica::ListNode::Cons { head: varIdx2, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: rest }, syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. }) => {
                let mut varIdx_remain: i32;
                let mut varIdxAlias: i32;
                let mut eqIdxDel: i32;
                let mut eqIdcs: metamodelica::List<i32>;
                let mut eqs1: metamodelica::List<i32>;
                let mut eqs2: metamodelica::List<i32>;
                let mut var1: metamodelica::Ref<BackendDAE::Var>;
                let mut var2: metamodelica::Ref<BackendDAE::Var>;
                let mut var_remain: metamodelica::Ref<BackendDAE::Var>;
                let mut var_alias: metamodelica::Ref<BackendDAE::Var>;
                let mut repl: BackendVarTransform::VariableReplacements;
                let mut varExp_remain: metamodelica::Ref<DAE::Exp>;
                let mut varExp_alias: metamodelica::Ref<DAE::Exp>;
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eqs = (*eqs).clone();
                repl = BackendVarTransform::emptyReplacements();
                eqs1 = metamodelica::arrayGet(mT.clone(), varIdx1.clone())?;
                eqs2 = metamodelica::arrayGet(mT.clone(), varIdx2.clone())?;
                var1 = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), varIdx1.clone())?;
                var2 = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), varIdx2.clone())?;
                if BackendVariable::isStateVar(&var1) {
                    varIdxAlias = varIdx2.clone();
                    varIdx_remain = varIdx1.clone();
                } else if BackendVariable::isStateVar(&var2) {
                    varIdx_remain = varIdx2.clone();
                    varIdxAlias = varIdx1.clone();
                } else {
                    if intLe(((eqs2).len() as i32), ((eqs1).len() as i32)) {
                        varIdxAlias = varIdx2.clone();
                        varIdx_remain = varIdx1.clone();
                    } else {
                        varIdxAlias = varIdx1.clone();
                        varIdx_remain = varIdx2.clone();
                    }
                }
                if intLe(((eqs2).len() as i32), ((eqs1).len() as i32)) {
                    eqIdxDel = eqIdx2.clone();
                } else {
                    eqIdxDel = eqIdx1.clone();
                }
                var_remain = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), varIdx_remain)?;
                var_alias = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), varIdxAlias)?;
                cref = BackendVariable::varCref(&var_alias);
                varExp_remain = BackendVariable::varExp(&var_remain)?;
                varExp_alias = BackendVariable::varExp(&var_alias)?;
                repl = BackendVarTransform::addReplacement(repl, cref, varExp_remain.clone(), None)?;
                eqIdcs = metamodelica::arrayGet(mT.clone(), varIdxAlias)?;
                eqLst = BackendEquation::getList(eqIdcs.clone(), eqs.clone())?;
                eqs = List::threadFold(&eqIdcs, eqLst, &BackendEquation::setAtIndexFirst, eqs.clone())?;
                BackendEquation::setAtIndex(eqs.clone(), eqIdxDel, metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: varExp_remain, scalar: varExp_alias, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }))?;
                { (tplsIn, m, mT, sysIn) = (rest.clone(), m.clone(), mT.clone(), syst.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: CommonSubExp::SHORTCUT_CSE { eqIdcs: Deref @ metamodelica::ListNode::Cons { head: eqIdx1, tail: Deref @ metamodelica::ListNode::Cons { head: eqIdx2, tail: Deref @ metamodelica::ListNode::Nil } }, sharedVar }, tail: rest }, syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. }) => {
                let mut n: i32;
                let mut var: metamodelica::Ref<BackendDAE::Var>;
                let mut eq1: metamodelica::Ref<BackendDAE::Equation>;
                let mut eq2: metamodelica::Ref<BackendDAE::Equation>;
                let mut eqNew: metamodelica::Ref<BackendDAE::Equation>;
                let mut lhs1: metamodelica::Ref<DAE::Exp>;
                let mut rhs1: metamodelica::Ref<DAE::Exp>;
                let mut lhs2: metamodelica::Ref<DAE::Exp>;
                let mut rhs2: metamodelica::Ref<DAE::Exp>;
                let mut varExp: metamodelica::Ref<DAE::Exp>;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::getList(list![eqIdx1.clone(), eqIdx2.clone()], eqs.clone())?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                eq1 = metamodelica::Own::own(__pa0);
                eq2 = metamodelica::Own::own(__pa1);
                var = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), sharedVar.clone())?;
                varExp = BackendVariable::varExp(&var)?;
                let (__pa3, __pa4) = ::match_deref::match_deref! { match &(eq1) {
                    Deref @ BackendDAE::Equation::EQUATION { exp: __pa3, scalar: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                lhs1 = metamodelica::Own::own(__pa3);
                rhs1 = metamodelica::Own::own(__pa4);
                let (__pa5, __pa6) = ::match_deref::match_deref! { match &(eq2) {
                    Deref @ BackendDAE::Equation::EQUATION { exp: __pa5, scalar: __pa6, .. } => (__pa5.clone(), __pa6.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                lhs2 = metamodelica::Own::own(__pa5);
                rhs2 = metamodelica::Own::own(__pa6);
                let true = (hasAlgebraicOperationsOnly(&lhs1)) else { return Err("pattern mismatch") };
                let true = (hasAlgebraicOperationsOnly(&rhs1)) else { return Err("pattern mismatch") };
                let true = (hasAlgebraicOperationsOnly(&lhs2)) else { return Err("pattern mismatch") };
                let true = (hasAlgebraicOperationsOnly(&rhs2)) else { return Err("pattern mismatch") };
                (rhs1, _) = ExpressionSolve::solve(lhs1, rhs1, varExp.clone(), None)?;
                (lhs1, _) = ExpressionSolve::solve(lhs2, rhs2, varExp, None)?;
                (_, lhs1, rhs1) = cancelExpressions(lhs1, rhs1)?;
                n = (((Expression::getAllCrefs(Expression::expSub(lhs1.clone(), rhs1.clone())?)?)).len() as i32);
                if n <= 2 {
                    eqNew = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: lhs1, scalar: rhs1, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    BackendEquation::setAtIndex(eqs.clone(), eqIdx1.clone(), eqNew)?;
                }
                { (tplsIn, m, mT, sysIn) = (rest.clone(), m.clone(), mT.clone(), syst.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                { (tplsIn, m, mT, sysIn) = (rest.clone(), m.clone(), mT.clone(), sysIn); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn hasAlgebraicOperationsOnly<'__b>(mut exp: &'__b metamodelica::Ref<DAE::Exp>) -> bool {
    '__tco: loop {
        match &**exp {
            DAE::Exp::RCONST { .. } => return true,
            DAE::Exp::CREF { .. } => return true,
            DAE::Exp::BINARY {
                exp1: e1,
                operator: _,
                exp2: e2,
            } => {
                let mut b: bool;
                b = hasAlgebraicOperationsOnly(e1);
                return b && hasAlgebraicOperationsOnly(e2);
            }
            DAE::Exp::UNARY { operator: _, exp: e1 } => {
                let mut b: bool;
                {
                    exp = e1;
                    continue '__tco;
                }
            }
            _ => return false,
        }
    }
}

fn cancelExpressions(
    mut e1In: metamodelica::Ref<DAE::Exp>,
    mut e2In: metamodelica::Ref<DAE::Exp>,
) -> Result<(bool, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut canceled: bool = false;
    let mut e1Out: metamodelica::Ref<DAE::Exp> = e1In.clone();
    let mut e2Out: metamodelica::Ref<DAE::Exp> = e2In.clone();
    let mut topLevelFactors1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut topLevelFactors2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    topLevelFactors1 = getTopLevelFactors(e1In.clone(), metamodelica::nil());
    topLevelFactors2 = getTopLevelFactors(e2In.clone(), metamodelica::nil());
    if !((topLevelFactors1).is_empty()) && !((topLevelFactors1).is_empty()) {
        topLevelFactors1 =
            List::intersectionOnTrue(&topLevelFactors1, &topLevelFactors2, &move |__a0: metamodelica::Ref<
                DAE::Exp,
            >,
                                                                                  __a1: metamodelica::Ref<
                DAE::Exp,
            >| {
                ExpressionBasics::expEqual(&__a0, __a1)
            })?;
        if ((topLevelFactors1).len() as i32) == 1 {
            e1Out = Expression::expDiv(e1In, (topLevelFactors1).head().cloned()?)?;
            (e1Out, _) = ExpressionSimplify::simplify(e1Out)?;
            e2Out = Expression::expDiv(e2In, (topLevelFactors2).head().cloned()?)?;
            (e2Out, _) = ExpressionSimplify::simplify(e2Out)?;
            canceled = true;
        }
    }
    Ok((canceled, e1Out, e2Out))
}

fn getTopLevelFactors(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut lstIn: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp) {
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                let mut eLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                eLst = getTopLevelFactors(e1.clone(), lstIn);
                { (exp, lstIn) = (e2.clone(), eLst); continue '__tco; }
            },
            Deref @ DAE::Exp::UNARY { operator: _, exp: e1 @ Deref @ DAE::Exp::CREF { .. } } => {
                return metamodelica::cons(e1.clone(), lstIn)
            },
            e1 @ Deref @ DAE::Exp::CREF { .. } => {
                return metamodelica::cons(e1.clone(), lstIn)
            },
            _ => {
                return lstIn
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn printCSE(mut cse: &CommonSubExp) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (match cse.clone() {
        CommonSubExp::ASSIGNMENT_CSE {
            eqIdcs: mut eqIdcs,
            sharedVars: mut sharedVars,
            aliasVars: mut aliasVars,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ASSIGN_CSE: eqs{"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(eqIdcs.clone(), &fnptr!(intString, i32))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!("   sharedVars{"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(sharedVars.clone(), &fnptr!(intString, i32))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!("   aliasVars{"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(aliasVars.clone(), &fnptr!(intString, i32))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        }
        CommonSubExp::SHORTCUT_CSE {
            eqIdcs: mut eqIdcs,
            sharedVar: mut sharedVar,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("SHORTCUT_CSE: eqs{"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(eqIdcs.clone(), &fnptr!(intString, i32))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("}"));
            __mm_s.push_str(&*literal!("   sharedVar{"));
            __mm_s.push_str(&*intString(sharedVar.clone()));
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        }
    });
    Ok(s)
}
