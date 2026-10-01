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
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Sorting;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub(crate) const BORDER: &'static str =
    "********************************************************************************";

pub(crate) const UNDERLINE: &'static str =
    "================================================================================";

type selectParameterFunc =
    std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

pub(crate) fn evaluateParameters(
    mut DAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut DAE: metamodelica::Ref<BackendDAE::BackendDAE> = DAE;
    let mut selectParameterfunc: selectParameterFunc;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut aliasVars: BackendDAE::Variables;
    let mut initialEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut cache: FCore::Cache;
    let mut graph: FCore::Graph;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut comps: metamodelica::List<metamodelica::List<i32>>;
    let mut ass2: metamodelica::Array<i32>;
    let mut markarr: metamodelica::Array<i32>;
    let mut size: i32;
    let mut mark: i32;
    let mut nselect: i32;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mt: metamodelica::Array<metamodelica::List<i32>>;
    let mut selectedParameters: metamodelica::List<i32>;
    let mut ht: metamodelica::Ref<AvlSetCR::Tree>;
    let mut isInitial: bool;
    isInitial = BackendDAEUtil::isInitializationDAE(&DAE.shared);
    if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nBEGINNING of preOptModule 'evaluateParameters'\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
        BackendDump::dumpBackendDAE(&DAE, &(literal!("DAE before evaluating parameters")))?;
    }
    if !(Flags::isSet(Flags::EVAL_PARAM.clone())?) {
        selectParameterfunc = (match (
            Flags::getConfigBool(Flags::EVALUATE_FINAL_PARAMS.clone())?,
            Flags::getConfigBool(Flags::EVALUATE_PROTECTED_PARAMS.clone())?,
        ) {
            (false, false) => {
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    metamodelica::print(literal!(
                        "\nStructural parameters and parameters with annotation(Evaluate=true) will be evaluated.\n"
                    ));
                }
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::hasVarEvaluateAnnotationTrue(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)
            }
            (true, false) => {
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    metamodelica::print(literal!(
                        "\nStructural parameters, final parameters and parameters with annotation(Evaluate=true) will be evaluated.\n"
                    ));
                }
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::hasVarEvaluateAnnotationTrueOrFinal(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)
            }
            (false, true) => {
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    metamodelica::print(literal!(
                        "\nStructural parameters, protected parameters and parameters with annotation(Evaluate=true) will be evaluated.\n"
                    ));
                }
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::hasVarEvaluateAnnotationTrueOrProtected(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)
            }
            (true, true) => {
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    metamodelica::print(literal!(
                        "\nStructural parameters, final parameters, protected parameters and parameters with annotation(Evaluate=true) will be evaluated.\n"
                    ));
                }
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BackendVariable::hasVarEvaluateAnnotationTrueOrFinalOrProtected(
                            &__a0,
                        ))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)
            }
            _ => return Err("match: no arm matched"),
        });
        let __arc8 = DAE;
        let BackendDAE::DAE {
            eqs: __pa0,
            shared: __t7,
        } = &*__arc8;
        let __arc9 = __t7.clone();
        let __pa6 = (__arc9).clone();
        let BackendDAE::SHARED {
            globalKnownVars: __pa1,
            aliasVars: __pa2,
            initialEqs: __pa3,
            cache: __pa4,
            graph: __pa5,
            ..
        } = &*__arc9;
        systs = metamodelica::Own::own(__pa0);
        globalKnownVars = metamodelica::Own::own(__pa1);
        aliasVars = metamodelica::Own::own(__pa2);
        initialEqs = metamodelica::Own::own(__pa3);
        cache = metamodelica::Own::own(__pa4);
        graph = metamodelica::Own::own(__pa5);
        shared = metamodelica::Own::own(__pa6);
        size = BackendVariable::varsSize(&globalKnownVars);
        m = arrayCreate(size, metamodelica::nil());
        mt = arrayCreate(size, metamodelica::nil());
        ass2 = Array::createIntRange(size);
        ht = FCore::getEvaluatedParams(cache.clone())?;
        (_, _, _, selectedParameters, m, mt, _, _) = BackendVariable::traverseBackendDAEVars(
            globalKnownVars.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>,
                      __a1: (
                    BackendDAE::Variables,
                    i32,
                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                    metamodelica::List<i32>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Ref<AvlSetCR::Tree>,
                    bool,
                )| getParameterAdjacencyMatrix(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                BackendDAE::Variables,
                                i32,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                                metamodelica::List<i32>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Ref<AvlSetCR::Tree>,
                                bool,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                BackendDAE::Variables,
                                i32,
                                Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
                                metamodelica::List<i32>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Ref<AvlSetCR::Tree>,
                                bool,
                            ),
                        )> + 'static,
                >),
            (
                globalKnownVars.clone(),
                1,
                selectParameterfunc.clone(),
                metamodelica::nil(),
                m.clone(),
                mt.clone(),
                ht,
                isInitial,
            ),
        )?;
        nselect = ((selectedParameters).len() as i32);
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nSTART evaluating parameters:\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Number of parameters: "));
                __mm_s.push_str(&*intString(size));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Number of parameters selected for evaluation: "));
                __mm_s.push_str(&*intString(nselect));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Selected parameters for evaluation:\n"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(selectedParameters.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            BackendDump::dumpAdjacencyMatrix(m.clone())?;
            BackendDump::dumpAdjacencyMatrixT(mt.clone())?;
        }
        markarr = arrayCreate(size, -1);
        size = intMax(
            BaseHashTable::defaultBucketSize.clone(),
            (((intReal(size)) * (metamodelica::OrderedFloat(0.7_f64))).0.floor() as i32),
        );
        nselect = intMax(BaseHashTable::defaultBucketSize.clone(), nselect * 2);
        repl = BackendVarTransform::emptyReplacementsSized(size);
        oRepl = BackendVarTransform::emptyReplacementsSized(nselect);
        (globalKnownVars, cache, repl, oRepl, mark) = evaluateSelectedParameters(
            &selectedParameters,
            globalKnownVars,
            m.clone(),
            initialEqs.clone(),
            cache,
            graph.clone(),
            markarr.clone(),
            isInitial,
            repl,
            oRepl,
            1,
        )?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\nAfter evaluating the selected parameters:\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("\nAll replacements:"));
            BackendVarTransform::dumpReplacements(&repl)?;
            metamodelica::print(literal!("\nReplacements that will be replaced in the DAE:"));
            BackendVarTransform::dumpReplacements(&oRepl)?;
            BackendDump::dumpVariables(&globalKnownVars, &(literal!("globalKnownVars")))?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nmark: "));
                __mm_s.push_str(&*intString(mark));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("markarr: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::mapArray(markarr.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        comps = Sorting::TarjanTransposed(mt.clone(), ass2.clone())?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\nAfter sorting parameters:\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\nOrder:\n"));
                ArcStr::from(__mm_s)
            });
            for mut comp in &*comps {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(comp.clone(), &fnptr!(intString, i32))?,
                        literal!(","),
                    ));
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
        (globalKnownVars, repl, oRepl, cache, mark) = traverseParameterSorted(
            &comps,
            globalKnownVars,
            m.clone(),
            initialEqs.clone(),
            cache,
            graph.clone(),
            mark,
            markarr.clone(),
            repl,
            oRepl,
            isInitial,
        )?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "\n\nAfter replacing the evaluated parameters in parameter bindings:\n"
                ));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("\nAll replacements:"));
            BackendVarTransform::dumpReplacements(&repl)?;
            metamodelica::print(literal!("\nReplacements that will be replaced in the DAE:"));
            BackendVarTransform::dumpReplacements(&oRepl)?;
            BackendDump::dumpVariables(&globalKnownVars, &(literal!("globalKnownVars")))?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nmark: "));
                __mm_s.push_str(&*intString(mark));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("markarr: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::mapArray(markarr.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        let (__pa10, (__pa11, __pa12, __pa13, __pa14, __pa15, __pa16, __pa17, _, __pa18, __pa19)) = List::mapFold(
            &systs,
            &replaceEvaluatedParametersSystem,
            (
                globalKnownVars,
                m.clone(),
                initialEqs,
                cache,
                graph,
                mark,
                markarr.clone(),
                isInitial,
                repl,
                oRepl,
            ),
        )?;
        systs = metamodelica::Own::own(__pa10);
        globalKnownVars = metamodelica::Own::own(__pa11);
        m = metamodelica::Own::own(__pa12);
        initialEqs = metamodelica::Own::own(__pa13);
        cache = metamodelica::Own::own(__pa14);
        graph = metamodelica::Own::own(__pa15);
        mark = metamodelica::Own::own(__pa16);
        markarr = metamodelica::Own::own(__pa17);
        repl = metamodelica::Own::own(__pa18);
        oRepl = metamodelica::Own::own(__pa19);
        (aliasVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
            aliasVars,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>,
                      __a1: (
                    BackendDAE::Variables,
                    metamodelica::Array<metamodelica::List<i32>>,
                    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
                    FCore::Cache,
                    FCore::Graph,
                    i32,
                    metamodelica::Array<i32>,
                    bool,
                    BackendVarTransform::VariableReplacements,
                    BackendVarTransform::VariableReplacements,
                )| replaceEvaluatedParameterTraverser(__a0, &__a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                BackendDAE::Variables,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Ref<
                                    ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                >,
                                FCore::Cache,
                                FCore::Graph,
                                i32,
                                metamodelica::Array<i32>,
                                bool,
                                BackendVarTransform::VariableReplacements,
                                BackendVarTransform::VariableReplacements,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<BackendDAE::Var>,
                            (
                                BackendDAE::Variables,
                                metamodelica::Array<metamodelica::List<i32>>,
                                metamodelica::Ref<
                                    ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                                >,
                                FCore::Cache,
                                FCore::Graph,
                                i32,
                                metamodelica::Array<i32>,
                                bool,
                                BackendVarTransform::VariableReplacements,
                                BackendVarTransform::VariableReplacements,
                            ),
                        )> + 'static,
                >),
            (
                globalKnownVars.clone(),
                m.clone(),
                initialEqs.clone(),
                cache.clone(),
                graph.clone(),
                mark,
                markarr.clone(),
                isInitial,
                repl.clone(),
                oRepl.clone(),
            ),
        )?;
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "\n\nAfter replacing the evaluated parameters in variable bindings and start attributes:\n"
                ));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(literal!("\nAll replacements:"));
            BackendVarTransform::dumpReplacements(&repl)?;
            metamodelica::print(literal!("\nReplacements that will be replaced in the DAE:"));
            BackendVarTransform::dumpReplacements(&oRepl)?;
            BackendDump::dumpVariables(&globalKnownVars, &(literal!("globalKnownVars")))?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\nmark: "));
                __mm_s.push_str(&*intString(mark));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("markarr: "));
                __mm_s.push_str(&*stringDelimitList(
                    List::mapArray(markarr.clone(), &fnptr!(intString, i32))?,
                    literal!(","),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
        if Flags::getConfigBool(Flags::REPLACE_EVALUATED_PARAMS.clone())? {
            assign_field!(
                shared.externalObjects = BackendVariable::listVar1(
                    &(List::map1(
                        BackendVariable::varList(&shared.externalObjects)?,
                        &move |__a0: metamodelica::Ref<BackendDAE::Var>,
                               __a1: BackendVarTransform::VariableReplacements| {
                            BackendVarTransform::replaceBindingExp(__a0, &__a1)
                        },
                        oRepl.clone()
                    )?)
                )?
            );
        }
        assign_field!(
            shared.globalKnownVars = globalKnownVars,
            shared.aliasVars = aliasVars,
            shared.initialEqs = initialEqs,
            shared.graph = graph,
            shared.cache = cache
        );
        DAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: systs,
            shared: shared,
        });
        if Flags::getConfigBool(Flags::REPLACE_EVALUATED_PARAMS.clone())? {
            if !(BackendVarTransform::isReplacementEmpty(&oRepl)?) {
                DAE = replaceEvaluatedParametersEqns(&DAE, oRepl)?;
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    BackendDump::dumpBackendDAE(&DAE, &(literal!("DAE after replacing the evaluated parameters")))?;
                }
            } else {
                if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                    metamodelica::print(literal!("\nThere is no evaluated parameter.\n"));
                }
            }
        } else {
            if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                Error::addCompilerNotification(literal!(
                    "Evaluated parameters are not replaced in the DAE. Use --replaceEvaluatedParameters=true to replace them in the DAE."
                ))?;
            }
        }
    } else {
        if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!(
                    "\nThere is nothing to do. All parameters are already evaluated.\n"
                ));
                __mm_s.push_str(&*arcstr::literal!(UNDERLINE));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nEND of preOptModule 'evaluateParameters'\n"));
            __mm_s.push_str(&*arcstr::literal!(BORDER));
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(DAE)
}

fn getParameterAdjacencyMatrix(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(
        BackendDAE::Variables,
        i32,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<AvlSetCR::Tree>,
        bool,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        i32,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<AvlSetCR::Tree>,
        bool,
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendDAE::Variables,
        i32,
        selectParameterFunc,
        metamodelica::List<i32>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<AvlSetCR::Tree>,
        bool,
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(e), .. }, (globalKnownVars, index, selectParameter, selectedParameters, m, mt, ht, isInitial)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut select: bool;
                    let mut selectedParameters = (*selectedParameters).clone();
                    let mut m = (*m).clone();
                    let mut mt = (*mt).clone();
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(e.clone(), &fnptr!(BackendDAEUtil::traversingadjacencyRowExpFinder, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::List<i32>, bool)), (globalKnownVars.clone(), metamodelica::nil(), isInitial.clone()))?;
                    ilst = metamodelica::Own::own(__pa0);
                    ilst = BackendDAEUtil::uniqueRow(ilst.clone())?;
                    cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                    select = selectParameter(v.clone())? || AvlSetCR::hasKey(ht.clone(), cref.clone())?;
                    selectedParameters = List::consOnTrue(select, index.clone(), selectedParameters.clone());
                    m = metamodelica::arrayUpdate(m.clone(), index.clone(), ilst.clone())?;
                    mt = List::fold1(&(metamodelica::cons(index.clone(), ilst.clone())), &Array::consToElement, index.clone(), mt.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), index.clone() + 1, selectParameter.clone(), selectedParameters.clone(), m.clone(), mt.clone(), ht.clone(), isInitial.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, values: attr, .. }, (globalKnownVars, index, selectParameter, selectedParameters, m, mt, ht, isInitial)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut select: bool;
                    let mut selectedParameters = (*selectedParameters).clone();
                    let mut m = (*m).clone();
                    let mut mt = (*mt).clone();
                    e = DAEUtil::getStartAttrFail(attr.clone())?;
                    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(e.clone(), &fnptr!(BackendDAEUtil::traversingadjacencyRowExpFinder, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::List<i32>, bool)), (globalKnownVars.clone(), metamodelica::nil(), isInitial.clone()))?;
                    ilst = metamodelica::Own::own(__pa0);
                    ilst = BackendDAEUtil::uniqueRow(ilst.clone())?;
                    cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                    select = selectParameter(v.clone())? || AvlSetCR::hasKey(ht.clone(), cref.clone())?;
                    selectedParameters = List::consOnTrue(select, index.clone(), selectedParameters.clone());
                    m = metamodelica::arrayUpdate(m.clone(), index.clone(), ilst.clone())?;
                    mt = List::fold1(&(metamodelica::cons(index.clone(), ilst.clone())), &Array::consToElement, index.clone(), mt.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), index.clone() + 1, selectParameter.clone(), selectedParameters.clone(), m.clone(), mt.clone(), ht.clone(), isInitial.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (globalKnownVars, index, selectParameter, selectedParameters, m, mt, ht, isInitial)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ilst: metamodelica::List<i32>;
                    let mut select: bool;
                    let mut selectedParameters = (*selectedParameters).clone();
                    let mut mt = (*mt).clone();
                    cref = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
                    select = selectParameter(v.clone())? || AvlSetCR::hasKey(ht.clone(), cref.clone())?;
                    selectedParameters = List::consOnTrue(select, index.clone(), selectedParameters.clone());
                    ilst = list![index.clone()];
                    mt = metamodelica::arrayUpdate(mt.clone(), index.clone(), ilst.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), index.clone() + 1, selectParameter.clone(), selectedParameters.clone(), m.clone(), mt.clone(), ht.clone(), isInitial.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, outTpl))
}

fn evaluateSelectedParameters(
    mut iSelected: &metamodelica::List<i32>,
    mut globalKnownVars: BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut cache: FCore::Cache,
    mut graph: FCore::Graph,
    mut markarr: metamodelica::Array<i32>,
    mut isInitial: bool,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
    mut mark: i32,
) -> Result<(
    BackendDAE::Variables,
    FCore::Cache,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
    i32,
)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut mark: i32 = mark;
    for mut i in &**iSelected {
        (globalKnownVars, cache, repl, replEvaluate, mark) = evaluateSelectedParameters0(
            i.clone(),
            globalKnownVars,
            m.clone(),
            inIEqns.clone(),
            cache,
            graph.clone(),
            markarr.clone(),
            isInitial,
            repl,
            replEvaluate,
            mark,
        )?;
    }
    Ok((globalKnownVars, cache, repl, replEvaluate, mark))
}

fn evaluateSelectedParameters0(
    mut i: i32,
    mut globalKnownVars: BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut cache: FCore::Cache,
    mut graph: FCore::Graph,
    mut markarr: metamodelica::Array<i32>,
    mut isInitial: bool,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
    mut mark: i32,
) -> Result<(
    BackendDAE::Variables,
    FCore::Cache,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
    i32,
)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut mark: i32 = mark;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    match '__try0: {
        let false = (intGt(
            ({
                let __elt = (*unwrap_break_err!(metamodelica::index_checked(&markarr.borrow(), i), '__try0)).clone();
                __elt
            }),
            0,
        )) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        unwrap_break_err!(metamodelica::arrayUpdate(markarr.clone(), i, mark), '__try0);
        (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(
            &({
                let __elt = (*unwrap_break_err!(metamodelica::index_checked(&m.borrow(), i), '__try0)).clone();
                __elt
            }),
            globalKnownVars.clone(),
            m.clone(),
            inIEqns.clone(),
            cache.clone(),
            &graph,
            mark,
            markarr.clone(),
            isInitial,
            repl.clone(),
            replEvaluate.clone(),
        );
        v = unwrap_break_err!(BackendVariable::getVarAt(&globalKnownVars, i), '__try0);
        (v, globalKnownVars, cache, mark, repl) = unwrap_break_err!(evaluateFixedAttribute(v.clone(), true, globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), &graph, mark, markarr.clone(), isInitial, repl.clone()), '__try0);
        (globalKnownVars, repl, replEvaluate, cache) = unwrap_break_err!(evaluateSelectedParameter(v.clone(), i, globalKnownVars.clone(), inIEqns.clone(), repl.clone(), replEvaluate.clone(), cache.clone(), graph.clone()), '__try0);
        Ok::<_, &'static str>((
            cache.clone(),
            globalKnownVars.clone(),
            repl.clone(),
            replEvaluate.clone(),
            v.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            cache = __try0_o0;
            globalKnownVars = __try0_o1;
            repl = __try0_o2;
            replEvaluate = __try0_o3;
            v = __try0_o4;
        }
        Err(_) => {
            v = BackendVariable::getVarAt(&globalKnownVars, i)?;
            (globalKnownVars, repl, replEvaluate, cache) = evaluateSelectedParameter(
                v.clone(),
                i,
                globalKnownVars.clone(),
                inIEqns.clone(),
                repl.clone(),
                replEvaluate.clone(),
                cache.clone(),
                graph.clone(),
            )?;
        }
    }
    Ok((globalKnownVars, cache, repl, replEvaluate, mark))
}

fn evaluateSelectedParameters1(
    mut iUsed: &metamodelica::List<i32>,
    mut globalKnownVars: BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut cache: FCore::Cache,
    mut graph: &FCore::Graph,
    mut mark: i32,
    mut markarr: metamodelica::Array<i32>,
    mut isInitial: bool,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
) -> (
    BackendDAE::Variables,
    FCore::Cache,
    i32,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
) {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut mark: i32 = mark;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    (globalKnownVars, cache, mark, repl, replEvaluate) = 'mc: {
        let __mc_input = &**iUsed;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((globalKnownVars.clone(), cache.clone(), mark, repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: i, tail: rest } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut mark: i32 = mark.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let false = (intGt(({let __elt = (*metamodelica::index_checked(&markarr.borrow(), i.clone())?).clone(); __elt}), 0)) else { return Err("pattern mismatch") };
                    metamodelica::arrayUpdate(markarr.clone(), i.clone(), mark)?;
                    (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(&(({let __elt = (*metamodelica::index_checked(&m.borrow(), i.clone())?).clone(); __elt})), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph, mark, markarr.clone(), isInitial, repl.clone(), replEvaluate.clone());
                    v = BackendVariable::getVarAt(&globalKnownVars, i.clone())?;
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), true, globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph, mark, markarr.clone(), isInitial, repl.clone())?;
                    (globalKnownVars, cache, repl, replEvaluate) = evaluateParameter(v.clone(), i.clone(), globalKnownVars.clone(), inIEqns.clone(), cache.clone(), graph.clone(), repl.clone(), replEvaluate.clone())?;
                    (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(metamodelica::AsArg::as_arg(&rest), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph, mark, markarr.clone(), isInitial, repl.clone(), replEvaluate.clone());
                    Ok(((globalKnownVars.clone(), cache.clone(), mark, repl.clone(), replEvaluate.clone()), cache.clone(), globalKnownVars.clone(), mark.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            globalKnownVars = __wb1;
            mark = __wb2;
            repl = __wb3;
            replEvaluate = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut mark: i32 = mark.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    (globalKnownVars, cache, mark, repl, replEvaluate) = evaluateSelectedParameters1(metamodelica::AsArg::as_arg(&rest), globalKnownVars.clone(), m.clone(), inIEqns.clone(), cache.clone(), graph, mark, markarr.clone(), isInitial, repl.clone(), replEvaluate.clone());
                    Ok(((globalKnownVars.clone(), cache.clone(), mark, repl.clone(), replEvaluate.clone()), cache.clone(), globalKnownVars.clone(), mark.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            globalKnownVars = __wb1;
            mark = __wb2;
            repl = __wb3;
            replEvaluate = __wb4;
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (globalKnownVars, cache, mark, repl, replEvaluate)
}

fn evaluateSelectedParameter(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut index: i32,
    mut globalKnownVars: BackendDAE::Variables,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
    mut cache: FCore::Cache,
    mut graph: FCore::Graph,
) -> Result<(
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
    FCore::Cache,
)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut cache: FCore::Cache = cache;
    let () = 'mc: {
        let __mc_input = &*var;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::CONST { .. }, bindExp: Some(e), .. } => {
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let true = (Expression::isConst(e.clone())?) else { return Err("pattern mismatch") };
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e.clone(), None)?;
                    Ok(((), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            repl = __wb0;
            replEvaluate = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::CONST { .. }, bindExp: Some(e), .. } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    (e1, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), &repl, None);
                    (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    v = BackendVariable::setBindExp(var.clone(), Some(e1.clone()));
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
                    Ok(((), cache.clone(), globalKnownVars.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            globalKnownVars = __wb1;
            repl = __wb2;
            replEvaluate = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(e), .. } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let true = (Expression::isConst(e.clone())?) else { return Err("pattern mismatch") };
                    v = BackendVariable::setVarFinal(var.clone(), true)?;
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                    if BackendVariable::varFixed(&v) {
                        repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e.clone(), None)?;
                        replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e.clone(), None)?;
                    }
                    Ok(((), globalKnownVars.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            globalKnownVars = __wb0;
            repl = __wb1;
            replEvaluate = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(e), .. } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    (e1, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), &repl, None);
                    (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    v = BackendVariable::setBindExp(var.clone(), Some(e1.clone()));
                    v = BackendVariable::setVarFinal(v.clone(), true)?;
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
                    Ok(((), cache.clone(), globalKnownVars.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            globalKnownVars = __wb1;
            repl = __wb2;
            replEvaluate = __wb3;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, values: attr, .. } => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut value: metamodelica::Ref<Values::Value>;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                    let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                    let true = (BackendVariable::varFixed(&var)) else { return Err("pattern mismatch") };
                    let false = (BackendVariable::varHasBindExp(&var)) else { return Err("pattern mismatch") };
                    e = DAEUtil::getStartAttrFail(attr.clone())?;
                    (e1, _) = BackendVarTransform::replaceExp(&e, &repl, None);
                    (cache, value) = Ceval::ceval(cache.clone(), graph.clone(), e1.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    e1 = ValuesUtil::valueExp(value.clone(), None)?;
                    v = BackendVariable::setVarStartValue(var.clone(), e1.clone())?;
                    v = BackendVariable::setVarFinal(v.clone(), true)?;
                    globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                    repl = BackendVarTransform::addReplacement(repl.clone(), cr.clone(), e1.clone(), None)?;
                    replEvaluate = BackendVarTransform::addReplacement(replEvaluate.clone(), cr.clone(), e1.clone(), None)?;
                    Ok(((), cache.clone(), globalKnownVars.clone(), repl.clone(), replEvaluate.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            globalKnownVars = __wb1;
            repl = __wb2;
            replEvaluate = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut info: SourceInfo;
                    if Flags::isSet(Flags::EVAL_PARAM_DUMP.clone())? {
                        info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(&var));
                        Error::addSourceMessage(&(Error::COMPILER_WARNING.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot evaluate Variable \"")); __mm_s.push_str(&*BackendDump::varString(&var)?); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }], &info)?;
                    }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((globalKnownVars, repl, replEvaluate, cache))
}

fn evaluateParameter(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut index: i32,
    mut globalKnownVars: BackendDAE::Variables,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut cache: FCore::Cache,
    mut graph: FCore::Graph,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
) -> Result<(
    BackendDAE::Variables,
    FCore::Cache,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
)> {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let () = (::match_deref::match_deref! { match &(&*var) {
        Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(e), .. } => {
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut value: metamodelica::Ref<Values::Value>;
            let mut e = (*e).clone();
            (e, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), &repl, None);
            (cache, value) = Ceval::ceval(cache, graph, e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
            e1 = ValuesUtil::valueExp(value, None)?;
            v = BackendVariable::setVarFinal(var.clone(), true)?;
            globalKnownVars = BackendVariable::setVarAt(globalKnownVars, index, v)?;
            repl = BackendVarTransform::addReplacement(repl, cr.clone(), e1.clone(), None)?;
            replEvaluate = BackendVarTransform::addReplacement(replEvaluate, cr.clone(), e1, None)?;
            ()
        },
        Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, values: attr, .. } if (BackendVariable::varFixed(&var)) => {
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut value: metamodelica::Ref<Values::Value>;
            e = DAEUtil::getStartAttrFail(attr.clone())?;
            (e, _) = BackendVarTransform::replaceExp(&e, &repl, None);
            (cache, value) = Ceval::ceval(cache, graph, e, false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
            e1 = ValuesUtil::valueExp(value, None)?;
            v = BackendVariable::setVarFinal(var.clone(), true)?;
            globalKnownVars = BackendVariable::setVarAt(globalKnownVars, index, v)?;
            repl = BackendVarTransform::addReplacement(repl, cr.clone(), e1.clone(), None)?;
            replEvaluate = BackendVarTransform::addReplacement(replEvaluate, cr.clone(), e1, None)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((globalKnownVars, cache, repl, replEvaluate))
}

fn evaluateFixedAttribute(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut addVar: bool,
    mut globalKnownVars: BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut cache: FCore::Cache,
    mut graph: &FCore::Graph,
    mut mark: i32,
    mut markarr: metamodelica::Array<i32>,
    mut isInitial: bool,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    BackendDAE::Variables,
    FCore::Cache,
    i32,
    BackendVarTransform::VariableReplacements,
)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut mark: i32 = mark;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    (var, globalKnownVars, cache, mark, repl) = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ BackendDAE::Var { values: None, .. } => {
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(Deref @ DAE::Exp::BCONST { bool: _ }), .. }), .. } => {
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { varName: cr, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: Some(e), .. }), source, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(metamodelica::AsArg::as_arg(&cr), e.clone(), attr.clone(), source.clone(), var, addVar, globalKnownVars, m.clone(), inIEqns, cache, graph, mark, markarr.clone(), isInitial, repl)?;
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { varName: cr, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: Some(e), .. }), source, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(metamodelica::AsArg::as_arg(&cr), e.clone(), attr.clone(), source.clone(), var, addVar, globalKnownVars, m.clone(), inIEqns, cache, graph, mark, markarr.clone(), isInitial, repl)?;
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { varName: cr, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(e), .. }), source, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(metamodelica::AsArg::as_arg(&cr), e.clone(), attr.clone(), source.clone(), var, addVar, globalKnownVars, m.clone(), inIEqns, cache, graph, mark, markarr.clone(), isInitial, repl)?;
            (var, globalKnownVars, cache, mark, repl)
        },
        Deref @ BackendDAE::Var { varName: cr, values: attr @ Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(e), .. }), source, .. } => {
            (var, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute1(metamodelica::AsArg::as_arg(&cr), e.clone(), attr.clone(), source.clone(), var, addVar, globalKnownVars, m.clone(), inIEqns, cache, graph, mark, markarr.clone(), isInitial, repl)?;
            (var, globalKnownVars, cache, mark, repl)
        },
        _ => {
            (var, globalKnownVars, cache, mark, repl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((var, globalKnownVars, cache, mark, repl))
}

fn evaluateFixedAttribute1(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut addVar: bool,
    mut globalKnownVars: BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut cache: FCore::Cache,
    mut graph: &FCore::Graph,
    mut mark: i32,
    mut markarr: metamodelica::Array<i32>,
    mut isInitial: bool,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    BackendDAE::Variables,
    FCore::Cache,
    i32,
    BackendVarTransform::VariableReplacements,
)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut cache: FCore::Cache = cache;
    let mut mark: i32 = mark;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut b: bool;
    let mut ilst: metamodelica::List<i32>;
    let mut attr1: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    (e1, _) = BackendVarTransform::replaceExp(&e, &repl, None);
    let (_, (_, __pa0, _)) = Expression::traverseExpTopDown(
        e1.clone(),
        &fnptr!(
            BackendDAEUtil::traversingadjacencyRowExpFinder,
            metamodelica::Ref<DAE::Exp>,
            (BackendDAE::Variables, metamodelica::List<i32>, bool)
        ),
        (globalKnownVars.clone(), metamodelica::nil(), isInitial),
    )?;
    ilst = metamodelica::Own::own(__pa0);
    (globalKnownVars, cache, mark, repl, _) = evaluateSelectedParameters1(
        &(BackendDAEUtil::uniqueRow(ilst)?),
        globalKnownVars,
        m.clone(),
        inIEqns,
        cache,
        graph,
        mark,
        markarr.clone(),
        isInitial,
        repl,
        BackendVarTransform::emptyReplacements(),
    );
    (e1, _) = BackendVarTransform::replaceExp(&e1, &repl, None);
    (e1, _) = ExpressionSimplify::simplify(e1)?;
    b = Expression::isConst(e1.clone())?;
    e1 = evaluateFixedAttributeReportWarning(b, cr, e, e1, source, globalKnownVars.clone())?;
    attr1 = DAEUtil::setFixedAttr(attr, Some(e1))?;
    var = BackendVariable::setVarAttributes(var, attr1);
    globalKnownVars = if (addVar) {
        BackendVariable::addVar(var.clone(), globalKnownVars)?
    } else {
        globalKnownVars
    };
    Ok((var, globalKnownVars, cache, mark, repl))
}

fn evaluateFixedAttributeReportWarning(
    mut b: bool,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut msg: ArcStr;
    let mut info: SourceInfo;
    if b {
        outExp = e1;
    } else {
        info = ElementSource::getElementSourceFileInfo(source);
        (outExp, _) = Expression::traverseExpBottomUp(
            e1,
            &fnptr!(
                replaceCrefWithBindStartExp,
                metamodelica::Ref<DAE::Exp>,
                (
                    BackendDAE::Variables,
                    bool,
                    (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
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
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>
                        )
                    )
                )
            ),
            (globalKnownVars, false, HashSet::emptyHashSet()),
        )?;
        msg = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr)?);
            __mm_s.push_str(&*literal!(" has unevaluateable fixed attribute value \""));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(e)?);
            __mm_s.push_str(&*literal!("\" use values from start attribute(s) \""));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?);
            __mm_s.push_str(&*literal!("\""));
            ArcStr::from(__mm_s)
        };
        Error::addSourceMessage(&(Error::COMPILER_WARNING.clone()), list![msg], &info)?;
    }
    Ok(outExp)
}

fn replaceCrefWithBindStartExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        BackendDAE::Variables,
        bool,
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
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        bool,
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
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        BackendDAE::Variables,
        bool,
        (
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
    );
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, b, hs)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut b = (*b).clone();
                    let mut hs = (*hs).clone();
                    let false = (BaseHashSet::has(cr.clone(), &(hs.clone()))?) else { return Err("pattern mismatch") };
                    (v, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?;
                    e = BackendVariable::varStartValueType(&v)?;
                    hs = BaseHashSet::add(cr.clone(), &(hs.clone()))?;
                    let (__pa0, (_, __pa1, __pa2)) = Expression::traverseExpBottomUp(e.clone(), &fnptr!(replaceCrefWithBindStartExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), (vars.clone(), b.clone(), hs.clone()))?;
                    e = metamodelica::Own::own(__pa0);
                    b = metamodelica::Own::own(__pa1);
                    hs = metamodelica::Own::own(__pa2);
                    Ok((e.clone(), (vars.clone(), b.clone(), hs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { .. }, (vars, _, hs)) => {
                    Ok((e.clone(), (vars.clone(), true, hs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn traverseParameterSorted(
    mut inComps: &metamodelica::List<metamodelica::List<i32>>,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut inIEqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iCache: FCore::Cache,
    mut graph: FCore::Graph,
    mut iMark: i32,
    mut markarr: metamodelica::Array<i32>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
    mut isInitial: bool,
) -> Result<(
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
    FCore::Cache,
    i32,
)> {
    let mut oKnVars: BackendDAE::Variables = inGlobalKnownVars;
    let mut oRepl: BackendVarTransform::VariableReplacements = repl;
    let mut oReplEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let mut oCache: FCore::Cache = iCache;
    let mut oMark: i32 = iMark;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    for mut ilst in &**inComps {
        for mut i in &*ilst.clone() {
            v = BackendVariable::getVarAt(&oKnVars, i.clone())?;
            (v, oKnVars, oCache, oMark, oRepl) = evaluateFixedAttribute(
                v,
                true,
                oKnVars,
                m.clone(),
                inIEqns.clone(),
                oCache,
                &graph,
                oMark,
                markarr.clone(),
                isInitial,
                oRepl,
            )?;
            (oKnVars, oRepl, oReplEvaluate) = evaluateParameterBindings(
                v,
                i.clone(),
                oKnVars,
                oCache.clone(),
                graph.clone(),
                oRepl,
                oReplEvaluate,
            );
        }
    }
    Ok((oKnVars, oRepl, oReplEvaluate, oCache, oMark))
}

fn evaluateParameterBindings(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut index: i32,
    mut globalKnownVars: BackendDAE::Variables,
    mut cache: FCore::Cache,
    mut graph: FCore::Graph,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
) -> (
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
) {
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    let () = 'mc: {
        let __mc_input = var.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        v @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(e), hideResult: hideResultOpt, .. } => {
                            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                            let mut value: metamodelica::Ref<Values::Value>;
                            let mut hideResultExp: metamodelica::Ref<DAE::Exp>;
                            let mut b: bool;
                            let mut v = (*v).clone();
                            let mut e = (*e).clone();
                            let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                            let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                            let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                            if Expression::isConst(e.clone())? && BackendVariable::isFinalVar(metamodelica::AsArg::as_arg(&v)) && BackendVariable::varFixed(metamodelica::AsArg::as_arg(&v)) {
                                (repl, replEvaluate) = addConstExpReplacement(e.clone(), cr.clone(), repl.clone(), replEvaluate.clone())?;
                            } else {
                                (e, b) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), &replEvaluate, None);
                                if b {
                                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                                    e = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CALL { expLst: exps, .. } if (Expression::isConstWorkList(exps.clone())?) => {
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                            e1 = ValuesUtil::valueExp(value.clone(), None)?;
                            e1.clone()
                },
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { expLst: exps, .. }, sub: _ } if (Expression::isConstWorkList(exps.clone())?) => {
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                            e1 = ValuesUtil::valueExp(value.clone(), None)?;
                            e1.clone()
                },
                _ => {
                            e.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                                    v = BackendVariable::setBindExp(v.clone(), Some(e.clone()));
                                    if !(BackendVariable::hasVarEvaluateAnnotationFalse(metamodelica::AsArg::as_arg(&v))) {
                                                (repl, replEvaluate) = addConstExpReplacement(e.clone(), cr.clone(), repl.clone(), replEvaluate.clone())?;
                                                v = if (Expression::isConst(e.clone())?) {BackendVariable::setVarFinal(v.clone(), true)?} else {v.clone()};
                                    }
                                }
                            }
                            let (__pa0, (__pa1, _)) = BackendDAEUtil::traverseBackendDAEVarAttr(v.values.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)), (replEvaluate.clone(), false))?;
                            attr = metamodelica::Own::own(__pa0);
                            replEvaluate = metamodelica::Own::own(__pa1);
                            v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                            assign_field!(v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
                Some(__esc_hideResultExp) => {
                            hideResultExp = (*__esc_hideResultExp).clone();
                            (hideResultExp, b) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&hideResultExp), &replEvaluate, None);
                            if b {
                                (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                            }
                            Some(hideResultExp.clone())
                },
                _ => v.hideResult.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }));
                            globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                            Ok(((), globalKnownVars.clone(), repl.clone(), replEvaluate.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            globalKnownVars = __wb0;
            repl = __wb1;
            replEvaluate = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        v @ Deref @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, values: attr, hideResult: hideResultOpt, .. } => {
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut value: metamodelica::Ref<Values::Value>;
                            let mut hideResultExp: metamodelica::Ref<DAE::Exp>;
                            let mut b: bool;
                            let mut v = (*v).clone();
                            let mut attr = (*attr).clone();
                            let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                            let mut repl: BackendVarTransform::VariableReplacements = repl.clone();
                            let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                            let true = (BackendVariable::varFixed(&var)) else { return Err("pattern mismatch") };
                            e = DAEUtil::getStartAttrFail(attr.clone())?;
                            (e, b) = BackendVarTransform::replaceExp(&e, &replEvaluate, None);
                            if b {
                                (e, _) = ExpressionSimplify::simplify(e.clone())?;
                                e = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CALL { expLst: exps, .. } if (Expression::isConstWorkList(exps.clone())?) => {
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                            e1 = ValuesUtil::valueExp(value.clone(), None)?;
                            e1.clone()
                },
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { expLst: exps, .. }, sub: _ } if (Expression::isConstWorkList(exps.clone())?) => {
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                            e1 = ValuesUtil::valueExp(value.clone(), None)?;
                            e1.clone()
                },
                _ => {
                            e.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                                v = BackendVariable::setVarStartValue(var.clone(), e.clone())?;
                                (repl, replEvaluate) = addConstExpReplacement(e.clone(), cr.clone(), repl.clone(), replEvaluate.clone())?;
                                v = if (Expression::isConst(e.clone())?) {BackendVariable::setVarFinal(v.clone(), true)?} else {v.clone()};
                            }
                            let (__pa0, (__pa1, _)) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)), (replEvaluate.clone(), false))?;
                            attr = metamodelica::Own::own(__pa0);
                            replEvaluate = metamodelica::Own::own(__pa1);
                            v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                            assign_field!(v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
                Some(__esc_hideResultExp) => {
                            hideResultExp = (*__esc_hideResultExp).clone();
                            (hideResultExp, b) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&hideResultExp), &replEvaluate, None);
                            if b {
                                (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                            }
                            Some(hideResultExp.clone())
                },
                _ => v.hideResult.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }));
                            globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                            Ok(((), globalKnownVars.clone(), repl.clone(), replEvaluate.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            globalKnownVars = __wb0;
            repl = __wb1;
            replEvaluate = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        v @ Deref @ BackendDAE::Var { bindExp: Some(e), hideResult: hideResultOpt, .. } => {
                            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
                            let mut value: metamodelica::Ref<Values::Value>;
                            let mut hideResultExp: metamodelica::Ref<DAE::Exp>;
                            let mut b: bool;
                            let mut v = (*v).clone();
                            let mut e = (*e).clone();
                            let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                            let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                            (e, b) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), &replEvaluate, None);
                            if b {
                                (e, _) = ExpressionSimplify::simplify(e.clone())?;
                                e = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CALL { expLst: exps, .. } if (Expression::isConstWorkList(exps.clone())?) => {
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                            e1 = ValuesUtil::valueExp(value.clone(), None)?;
                            e1.clone()
                },
                Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CALL { expLst: exps, .. }, sub: _ } if (Expression::isConstWorkList(exps.clone())?) => {
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            (_, value) = Ceval::ceval(cache.clone(), graph.clone(), e.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                            e1 = ValuesUtil::valueExp(value.clone(), None)?;
                            e1.clone()
                },
                _ => {
                            e.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                                v = BackendVariable::setBindExp(var.clone(), Some(e.clone()));
                            }
                            let (__pa0, (__pa1, _)) = BackendDAEUtil::traverseBackendDAEVarAttr(v.values.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)), (replEvaluate.clone(), false))?;
                            attr = metamodelica::Own::own(__pa0);
                            replEvaluate = metamodelica::Own::own(__pa1);
                            v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                            assign_field!(v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
                Some(__esc_hideResultExp) => {
                            hideResultExp = (*__esc_hideResultExp).clone();
                            (hideResultExp, b) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&hideResultExp), &replEvaluate, None);
                            if b {
                                (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                            }
                            Some(hideResultExp.clone())
                },
                _ => v.hideResult.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }));
                            globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                            Ok(((), globalKnownVars.clone(), replEvaluate.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            globalKnownVars = __wb0;
            replEvaluate = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ BackendDAE::Var { values: attr, hideResult: hideResultOpt, .. } => {
                            let mut v: metamodelica::Ref<BackendDAE::Var>;
                            let mut hideResultExp: metamodelica::Ref<DAE::Exp>;
                            let mut b: bool;
                            let mut attr = (*attr).clone();
                            let mut globalKnownVars: BackendDAE::Variables = globalKnownVars.clone();
                            let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate.clone();
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)), (replEvaluate.clone(), false))?) {
                                (__pa0, (__pa1, true)) => (__pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            attr = metamodelica::Own::own(__pa0);
                            replEvaluate = metamodelica::Own::own(__pa1);
                            v = BackendVariable::setVarAttributes(var.clone(), attr.clone());
                            assign_field!(v.hideResult = (::match_deref::match_deref! { match &(hideResultOpt.clone()) {
                Some(__esc_hideResultExp) => {
                            hideResultExp = (*__esc_hideResultExp).clone();
                            (hideResultExp, b) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&hideResultExp), &replEvaluate, None);
                            if b {
                                (hideResultExp, _) = ExpressionSimplify::simplify(hideResultExp.clone())?;
                            }
                            Some(hideResultExp.clone())
                },
                _ => v.hideResult.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }));
                            globalKnownVars = BackendVariable::setVarAt(globalKnownVars.clone(), index, v.clone())?;
                            Ok(((), globalKnownVars.clone(), replEvaluate.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            globalKnownVars = __wb0;
            replEvaluate = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (globalKnownVars, repl, replEvaluate)
}

fn addConstExpReplacement(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut repl: BackendVarTransform::VariableReplacements,
    mut replEvaluate: BackendVarTransform::VariableReplacements,
) -> Result<(
    BackendVarTransform::VariableReplacements,
    BackendVarTransform::VariableReplacements,
)> {
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut replEvaluate: BackendVarTransform::VariableReplacements = replEvaluate;
    if Expression::isConst(inExp.clone())? {
        repl = BackendVarTransform::addReplacement(repl, cr.clone(), inExp.clone(), None)?;
        replEvaluate = BackendVarTransform::addReplacement(replEvaluate, cr, inExp, None)?;
    }
    Ok((repl, replEvaluate))
}

fn traverseExpVisitorWrapper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (BackendVarTransform::VariableReplacements, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (BackendVarTransform::VariableReplacements, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (BackendVarTransform::VariableReplacements, bool);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (exp @ Deref @ DAE::Exp::CREF { .. }, (repl, b)) => {
            let mut b1: bool;
            let mut exp = (*exp).clone();
            (exp, b1) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&repl), None);
            (exp.clone(), (repl.clone(), b.clone() || b1))
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTpl)
}

fn replaceEvaluatedParametersSystem(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inTypeA: (
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        FCore::Cache,
        FCore::Graph,
        i32,
        metamodelica::Array<i32>,
        bool,
        BackendVarTransform::VariableReplacements,
        BackendVarTransform::VariableReplacements,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    (
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        FCore::Cache,
        FCore::Graph,
        i32,
        metamodelica::Array<i32>,
        bool,
        BackendVarTransform::VariableReplacements,
        BackendVarTransform::VariableReplacements,
    ),
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outTypeA: (
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        FCore::Cache,
        FCore::Graph,
        i32,
        metamodelica::Array<i32>,
        bool,
        BackendVarTransform::VariableReplacements,
        BackendVarTransform::VariableReplacements,
    );
    let mut vars: BackendDAE::Variables;
    let __arc1 = isyst.clone();
    let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &*__arc1;
    vars = metamodelica::Own::own(__pa0);
    (vars, outTypeA) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        vars,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>,
                  __a1: (
                BackendDAE::Variables,
                metamodelica::Array<metamodelica::List<i32>>,
                metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
                FCore::Cache,
                FCore::Graph,
                i32,
                metamodelica::Array<i32>,
                bool,
                BackendVarTransform::VariableReplacements,
                BackendVarTransform::VariableReplacements,
            )| replaceEvaluatedParameterTraverser(__a0, &__a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            metamodelica::Array<metamodelica::List<i32>>,
                            metamodelica::Ref<
                                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                            >,
                            FCore::Cache,
                            FCore::Graph,
                            i32,
                            metamodelica::Array<i32>,
                            bool,
                            BackendVarTransform::VariableReplacements,
                            BackendVarTransform::VariableReplacements,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            metamodelica::Array<metamodelica::List<i32>>,
                            metamodelica::Ref<
                                ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
                            >,
                            FCore::Cache,
                            FCore::Graph,
                            i32,
                            metamodelica::Array<i32>,
                            bool,
                            BackendVarTransform::VariableReplacements,
                            BackendVarTransform::VariableReplacements,
                        ),
                    )> + 'static,
            >),
        inTypeA,
    )?;
    osyst = BackendDAEUtil::setEqSystVars(isyst, vars);
    Ok((osyst, outTypeA))
}

fn replaceEvaluatedParameterTraverser(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTpl: &(
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        FCore::Cache,
        FCore::Graph,
        i32,
        metamodelica::Array<i32>,
        bool,
        BackendVarTransform::VariableReplacements,
        BackendVarTransform::VariableReplacements,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        FCore::Cache,
        FCore::Graph,
        i32,
        metamodelica::Array<i32>,
        bool,
        BackendVarTransform::VariableReplacements,
        BackendVarTransform::VariableReplacements,
    ),
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
        metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
        FCore::Cache,
        FCore::Graph,
        i32,
        metamodelica::Array<i32>,
        bool,
        BackendVarTransform::VariableReplacements,
        BackendVarTransform::VariableReplacements,
    );
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { bindExp: Some(e), values: attr, .. }, (globalKnownVars, m, ieqns, cache, graph, mark, markarr, isInitial, repl, replEvaluate)) => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut v = (*v).clone();
                    let mut attr = (*attr).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut cache = (*cache).clone();
                    let mut mark = (*mark).clone();
                    let mut repl = (*repl).clone();
                    let mut replEvaluate = (*replEvaluate).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&replEvaluate), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    v = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    let (__pa1, (__pa2, __pa3)) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)), (replEvaluate.clone(), false))?;
                    attr = metamodelica::Own::own(__pa1);
                    replEvaluate = metamodelica::Own::own(__pa2);
                    b = metamodelica::Own::own(__pa3);
                    v = if (b) {BackendVariable::setVarAttributes(v.clone(), attr.clone())} else {v.clone()};
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), false, globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), metamodelica::AsArg::as_arg(&graph), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { values: attr, .. }, (globalKnownVars, m, ieqns, cache, graph, mark, markarr, isInitial, repl, replEvaluate)) => {
                    let mut v = (*v).clone();
                    let mut attr = (*attr).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut cache = (*cache).clone();
                    let mut mark = (*mark).clone();
                    let mut repl = (*repl).clone();
                    let mut replEvaluate = (*replEvaluate).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), &fnptr!(traverseExpVisitorWrapper, metamodelica::Ref<DAE::Exp>, (BackendVarTransform::VariableReplacements, bool)), (replEvaluate.clone(), false))?) {
                        (__pa0, (__pa1, true)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    attr = metamodelica::Own::own(__pa0);
                    replEvaluate = metamodelica::Own::own(__pa1);
                    v = BackendVariable::setVarAttributes(v.clone(), attr.clone());
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), false, globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), metamodelica::AsArg::as_arg(&graph), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (globalKnownVars, m, ieqns, cache, graph, mark, markarr, isInitial, repl, replEvaluate)) => {
                    let mut v = (*v).clone();
                    let mut globalKnownVars = (*globalKnownVars).clone();
                    let mut cache = (*cache).clone();
                    let mut mark = (*mark).clone();
                    let mut repl = (*repl).clone();
                    (v, globalKnownVars, cache, mark, repl) = evaluateFixedAttribute(v.clone(), false, globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), metamodelica::AsArg::as_arg(&graph), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone())?;
                    Ok((v.clone(), (globalKnownVars.clone(), m.clone(), ieqns.clone(), cache.clone(), graph.clone(), mark.clone(), markarr.clone(), isInitial.clone(), repl.clone(), replEvaluate.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, outTpl))
}

fn replaceEvaluatedParametersEqns(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut lsteqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut b: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systs = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    lsteqns = BackendEquation::equationList(shared.initialEqs.clone())?;
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, &inRepl, None)?;
    if b {
        assign_field!(shared.initialEqs = BackendEquation::listEquation(&lsteqns)?);
    }
    lsteqns = BackendEquation::equationList(shared.removedEqs.clone())?;
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, &inRepl, None)?;
    if b {
        assign_field!(shared.removedEqs = BackendEquation::listEquation(&lsteqns)?);
    }
    systs = List::map1(
        systs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>, __a1: BackendVarTransform::VariableReplacements| {
            replaceEvaluatedParametersSystemEqns(__a0, &__a1)
        },
        inRepl,
    )?;
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systs,
        shared: shared,
    });
    Ok(outDAE)
}

fn replaceEvaluatedParametersSystemEqns(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inRepl: &BackendVarTransform::VariableReplacements,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst;
    let mut lsteqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut b: bool;
    lsteqns = BackendEquation::equationList(osyst.orderedEqs.clone())?;
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, inRepl, None)?;
    if b {
        assign_field!(osyst.orderedEqs = BackendEquation::listEquation(&lsteqns)?);
        osyst = BackendDAEUtil::clearEqSyst(&osyst);
    }
    lsteqns = BackendEquation::equationList(osyst.removedEqs.clone())?;
    (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, inRepl, None)?;
    if b {
        assign_field!(osyst.removedEqs = BackendEquation::listEquation(&lsteqns)?);
    }
    Ok(osyst)
}
