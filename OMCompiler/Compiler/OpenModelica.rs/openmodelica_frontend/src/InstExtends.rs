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

use crate::FGraph;
use crate::InnerOuter;
use crate::Inst;
use crate::InstUtil;
use crate::Lookup;
use crate::Mod;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::SCodeInstUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::AvlSetString;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

fn instExtendsList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inLocalElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inElementsFromExtendsScope: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inState: &ClassInf::State,
    mut inClassName: ArcStr,
    mut inImpl: bool,
    mut inPartialInst: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH;
    let mut outMod: metamodelica::Ref<DAE::Mod> = inMod;
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> =
        metamodelica::nil();
    let mut outNormalEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
    let mut outInitialEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
    let mut outNormalAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
    let mut outInitialAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = metamodelica::nil();
    let mut duplicates: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut duplicateUnparseStrings: metamodelica::List<ArcStr> = metamodelica::nil();
    duplicates = List::sortedDuplicates(
        List::sort(
            inElementsFromExtendsScope.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<SCode::Element>,
                      __a1: metamodelica::Ref<SCode::Element>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(SCodeUtil::elementEqual(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<SCode::Element>,
                            metamodelica::Ref<SCode::Element>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?,
        &move |__a0: metamodelica::Ref<SCode::Element>,
               __a1: metamodelica::Ref<SCode::Element>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(SCodeUtil::elementEqual(&__a0, &__a1)) },
    )?;
    if Config::acceptMetaModelicaGrammar()? {
        duplicates = List::filterOnFalse(
            duplicates,
            &move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(SCodeUtil::isTypeVar(&__a0))
            },
        )?;
    }
    if !((duplicates).is_empty()) {
        duplicateUnparseStrings = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut i in (duplicates.clone()).into_iter().cloned() {
                let __x = SCodeDump::unparseElementStr(i.clone(), SCodeDump::defaultOptions.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if ((duplicates).len() as i32) > 1 {
            Error::addMultiSourceMessage(
                &(Error::DUPLICATE_VARIABLE_ERROR.clone()),
                &duplicateUnparseStrings,
                &(List::map(
                    duplicates,
                    &move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(SCodeUtil::elementInfo(&__a0))
                    },
                )?),
            )?;
        } else {
            Error::addSourceMessage(
                &(Error::DUPLICATE_VARIABLE_ERROR.clone()),
                duplicateUnparseStrings,
                &(SCodeUtil::elementInfo(&((duplicates).head().cloned()?))),
            )?;
        }
        return Err("fail");
    }
    for mut el in &*inLocalElements.reverse() {
        let () = 'mc: {
            let __mc_input = el.clone();
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::EXTENDS { .. } => {
                        let mut cn: ArcStr;
                        let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::makeNotFullyQualified(var_field!((**el).baseClassPath, SCode::Element::EXTENDS).clone())) {
                            Deref @ Absyn::Path::IDENT { name: __pa0 } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        cn = metamodelica::Own::own(__pa0);
                        let true = (InstUtil::isBuiltInClass(&cn)?) else { return Err("pattern mismatch") };
                        Ok(())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            Deref @ SCode::Element::EXTENDS { .. } => {
                                let mut cn: ArcStr;
                                let mut bc_str: ArcStr;
                                let mut scope_str: ArcStr;
                                let mut base_first_id: ArcStr;
                                let mut emod: metamodelica::Ref<SCode::Mod>;
                                let mut eq_name: bool;
                                let mut ocls: Option<metamodelica::Ref<SCode::Element>>;
                                let mut cls: metamodelica::Ref<SCode::Element>;
                                let mut cenv: FCore::Graph;
                                let mut encf: SCode::Encapsulated;
                                let mut els1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                                let mut rest_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                                let mut import_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                                let mut cdef_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                                let mut clsext_els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                                let mut els2: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>;
                                let mut eq1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                                let mut ieq1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                                let mut eq2: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                                let mut ieq2: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                                let mut alg1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                                let mut ialg1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                                let mut alg2: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                                let mut ialg2: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                                let mut comments1: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                                let mut comments2: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
                                let mut cmt: metamodelica::Ref<SCode::Comment>;
                                let mut r#mod: metamodelica::Ref<DAE::Mod>;
                                let mut tree: metamodelica::Ref<AvlSetString::Tree>;
                                let mut cacheArr: metamodelica::Array<FCore::Cache>;
                                let mut htHasEntries: bool;
                                let mut outCache: FCore::Cache = outCache.clone();
                                let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = outComments.clone();
                                let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> = outElements.clone();
                                let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                                let mut outInitialAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = outInitialAlgs.clone();
                                let mut outInitialEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>> = outInitialEqs.clone();
                                let mut outMod: metamodelica::Ref<DAE::Mod> = outMod.clone();
                                let mut outNormalAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = outNormalAlgs.clone();
                                let mut outNormalEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>> = outNormalEqs.clone();
                                emod = InstUtil::chainRedeclares(&outMod, var_field!((**el).modifications, SCode::Element::EXTENDS).clone());
                                base_first_id = AbsynUtil::pathFirstIdent(var_field!((**el).baseClassPath, SCode::Element::EXTENDS));
                                eq_name = stringEq(&inClassName, &base_first_id) && AbsynUtil::pathEqual(&(ClassInfUtil::getStateName(inState)), &(AbsynUtil::joinPaths(FGraph::getGraphName(&outEnv)?, AbsynUtil::makeIdentPathFromString(base_first_id.clone()))?));
                                (outCache, ocls, cenv) = lookupBaseClass(var_field!((**el).baseClassPath, SCode::Element::EXTENDS), eq_name, inClassName.clone(), outEnv.clone(), outCache.clone())?;
                                if (ocls).is_some() {
                                    let __pa0 = ::match_deref::match_deref! { match &(ocls.clone()) {
                                        Some(__pa0) => __pa0.clone(),
                                        _ => return Err("pattern mismatch"),
                                    } };
                                    cls = metamodelica::Own::own(__pa0);
                                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(cls.clone()) {
                                        Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, cmt: __pa3, .. } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                                        _ => return Err("pattern mismatch"),
                                    } };
                                    cn = metamodelica::Own::own(__pa1);
                                    encf = metamodelica::Own::own(__pa2);
                                    cmt = metamodelica::Own::own(__pa3);
                                } else {
                                    if Flags::getConfigBool(Flags::PERMISSIVE.clone())? {
                                        bc_str = AbsynUtil::pathString(var_field!((**el).baseClassPath, SCode::Element::EXTENDS).clone(), literal!("."), true, false)?;
                                        scope_str = FGraph::printGraphPathStr(&inEnv);
                                        Error::addSourceMessage(&(Error::LOOKUP_BASECLASS_ERROR.clone()), list![bc_str.clone(), scope_str.clone()], var_field!((**el).info, SCode::Element::EXTENDS))?;
                                    }
                                    return Err("fail");
                                }
                                (outCache, cenv, outIH, els1, eq1, ieq1, alg1, ialg1, r#mod, comments1) = instDerivedClasses(outCache.clone(), cenv.clone(), outIH.clone(), outMod.clone(), inPrefix.clone(), &cls, inImpl, var_field!((**el).info, SCode::Element::EXTENDS).clone())?;
                                els1 = updateElementListVisibility(els1.clone(), var_field!((**el).visibility, SCode::Element::EXTENDS).clone());
                                tree = AvlSetString::new();
                                tree = getLocalIdentList(&(InstUtil::constantAndParameterEls(inElementsFromExtendsScope.clone())), tree.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::Ref<AvlSetString::Tree>| getLocalIdentElement(&__a0, __a1))?;
                                tree = getLocalIdentList(&(InstUtil::constantAndParameterEls(els1.clone())), tree.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::Ref<AvlSetString::Tree>| getLocalIdentElement(&__a0, __a1))?;
                                cacheArr = arrayCreate(1, outCache.clone());
                                emod = fixModifications(cacheArr.clone(), inEnv.clone(), emod.clone(), tree.clone())?;
                                cenv = FGraph::openScope(cenv.clone(), encf, cn.clone(), FGraph::classInfToScopeType(inState))?;
                                (import_els, cdef_els, clsext_els, rest_els) = InstUtil::splitEltsNoComponents(&els1)?;
                                (outCache, cenv, outIH) = InstUtil::addClassdefsToEnv(outCache.clone(), cenv.clone(), outIH.clone(), inPrefix.clone(), &import_els, inImpl, None, false)?;
                                (outCache, cenv, outIH) = InstUtil::addClassdefsToEnv(outCache.clone(), cenv.clone(), outIH.clone(), inPrefix.clone(), &cdef_els, inImpl, Some(r#mod.clone()), false)?;
                                rest_els = SCodeInstUtil::addRedeclareAsElementsToExtends(&(rest_els.clone()), ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                    for mut e in (rest_els.clone()).into_iter().cloned() {
                                if !(SCodeUtil::isRedeclareElement(&(e.clone()))) { continue; }
                                let __x = e.clone();
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }))?;
                                outMod = Mod::elabUntypedMod(emod.clone(), Mod::ModScope::EXTENDS { path: var_field!((**el).baseClassPath, SCode::Element::EXTENDS).clone() })?;
                                outMod = Mod::merge(r#mod.clone(), outMod.clone(), literal!(""), false)?;
                                (outCache, _, outIH, _, els2, eq2, ieq2, alg2, ialg2, comments2) = instExtendsAndClassExtendsList2(outCache.clone(), cenv.clone(), outIH.clone(), outMod.clone(), inPrefix.clone(), rest_els.clone(), &clsext_els, els1.clone(), inState, inClassName.clone(), inImpl, inPartialInst)?;
                                tree = AvlSetString::new();
                                tree = getLocalIdentList(&els2, tree.clone(), &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool), __a1: metamodelica::Ref<AvlSetString::Tree>| getLocalIdentElementTpl(&__a0, __a1))?;
                                tree = getLocalIdentList(&cdef_els, tree.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::Ref<AvlSetString::Tree>| getLocalIdentElement(&__a0, __a1))?;
                                tree = getLocalIdentList(&import_els, tree.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::Ref<AvlSetString::Tree>| getLocalIdentElement(&__a0, __a1))?;
                                htHasEntries = !(AvlSetString::isEmpty(&tree));
                                metamodelica::arrayUpdate(cacheArr.clone(), 1, outCache.clone())?;
                                if htHasEntries {
                                    els2 = fixList(cacheArr.clone(), &cenv, els2.clone(), &tree, (std::sync::Arc::new(fixLocalIdent) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool), metamodelica::Ref<AvlSetString::Tree>) -> Result<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> + 'static>))?;
                                }
                                outElements = listAppend(els2.clone(), outElements.clone());
                                outNormalEqs = List::unionAppendListOnTrue(&(eq2.clone().reverse()), outNormalEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                outInitialEqs = List::unionAppendListOnTrue(&(ieq2.clone().reverse()), outInitialEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                outNormalAlgs = List::unionAppendListOnTrue(&(alg2.clone().reverse()), outNormalAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                outInitialAlgs = List::unionAppendListOnTrue(&(ialg2.clone().reverse()), outInitialAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                outComments = listAppend(comments1.clone(), listAppend(comments2.clone(), metamodelica::cons(cmt.clone(), outComments.clone())));
                                if !(inPartialInst) {
                                    if htHasEntries {
                                        eq1 = fixList(cacheArr.clone(), &cenv, eq1.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::Equation>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Equation>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static>))?;
                                        ieq1 = fixList(cacheArr.clone(), &cenv, ieq1.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::Equation>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Equation>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static>))?;
                                        alg1 = fixList(cacheArr.clone(), &cenv, alg1.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::AlgorithmSection>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixAlgorithm(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::AlgorithmSection>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> + 'static>))?;
                                        ialg1 = fixList(cacheArr.clone(), &cenv, ialg1.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::AlgorithmSection>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixAlgorithm(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::AlgorithmSection>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> + 'static>))?;
                                    }
                                    outNormalEqs = List::unionAppendListOnTrue(&(eq1.clone().reverse()), outNormalEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                    outInitialEqs = List::unionAppendListOnTrue(&(ieq1.clone().reverse()), outInitialEqs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                    outNormalAlgs = List::unionAppendListOnTrue(&(alg1.clone().reverse()), outNormalAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                    outInitialAlgs = List::unionAppendListOnTrue(&(ialg1.clone().reverse()), outInitialAlgs.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)))?;
                                }
                                outCache = metamodelica::arrayGet(cacheArr.clone(), 1)?;
                                Ok(((), outCache.clone(), outComments.clone(), outElements.clone(), outIH.clone(), outInitialAlgs.clone(), outInitialEqs.clone(), outMod.clone(), outNormalAlgs.clone(), outNormalEqs.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                outCache = __wb0;
                outComments = __wb1;
                outElements = __wb2;
                outIH = __wb3;
                outInitialAlgs = __wb4;
                outInitialEqs = __wb5;
                outMod = __wb6;
                outNormalAlgs = __wb7;
                outNormalEqs = __wb8;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::EXTENDS { .. } => {
                        if !((Flags::getConfigBool(Flags::PERMISSIVE.clone())?)) { return Err("guard") }
                        Ok(())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::COMPONENT { .. } => {
                        let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> = outElements.clone();
                        if SCodeUtil::isConstant(SCodeUtil::attrVariability(var_field!((**el).attributes, SCode::Element::COMPONENT))) || !(inPartialInst) {
                            outElements = metamodelica::cons((el.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), false), outElements.clone());
                        }
                        Ok(((), outElements.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outElements = __wb0;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::CLASS { .. } => {
                        let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = outComments.clone();
                        let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> = outElements.clone();
                        outElements = metamodelica::cons((el.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), false), outElements.clone());
                        outComments = list![var_field!((**el).cmt, SCode::Element::CLASS).clone()];
                        Ok(((), outComments.clone(), outElements.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outComments = __wb0;
                outElements = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ SCode::Element::IMPORT { .. } => {
                        let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> = outElements.clone();
                        outElements = metamodelica::cons((el.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), false), outElements.clone());
                        Ok(((), outElements.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outElements = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instExtendsList failed on:\n\t")); __mm_s.push_str(&*literal!("className: ")); __mm_s.push_str(&*inClassName); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("env:       ")); __mm_s.push_str(&*FGraph::printGraphPathStr(&outEnv)); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("mods:      ")); __mm_s.push_str(&*Mod::printModStr(&outMod)?); __mm_s.push_str(&*literal!("\n\t")); __mm_s.push_str(&*literal!("elem:      ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
    }
    (outElements, outMod) = updateComponentsAndClassdefs(&outElements, outMod, inEnv)?;
    Ok((
        outCache,
        outEnv,
        outIH,
        outMod,
        outElements,
        outNormalEqs,
        outInitialEqs,
        outNormalAlgs,
        outInitialAlgs,
        outComments,
    ))
}

fn lookupBaseClass(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inSelfReference: bool,
    mut inClassName: ArcStr,
    mut inEnv: FCore::Graph,
    mut inCache: FCore::Cache,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<SCode::Element>>, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outElement: Option<metamodelica::Ref<SCode::Element>>;
    let mut outEnv: FCore::Graph;
    (outCache, outElement, outEnv) = (::match_deref::match_deref! { match &((inPath.clone(), inSelfReference)) {
        (Deref @ Absyn::Path::IDENT { name }, true) => {
            let mut elem: metamodelica::Ref<SCode::Element>;
            let mut env: FCore::Graph;
            (elem, env) = Lookup::lookupClassLocal(inEnv, name.clone())?;
            (inCache, Some(elem), env)
        },
        (_, _) => {
            let mut elem: metamodelica::Ref<SCode::Element>;
            let mut env: FCore::Graph;
            let mut cache: FCore::Cache;
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = AbsynUtil::removePartialPrefix(&(metamodelica::Ref::new(Absyn::Path::IDENT { name: inClassName })), inPath);
            (cache, elem, env) = Lookup::lookupClass(&inCache, &inEnv, &path, None)?;
            (cache, Some(elem), env)
        },
        _ => {
            (inCache, None, inEnv)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outElement, outEnv))
}

fn updateElementListVisibility(
    mut inElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inVisibility: SCode::Visibility,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outElements = (match inVisibility {
        SCode::Visibility::PUBLIC { .. } => inElements,
        _ => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut e in (inElements).into_iter().cloned() {
                    let __x = SCodeUtil::makeElementProtected(e.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
    });
    outElements
}

pub(crate) fn instExtendsAndClassExtendsList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inExtendsElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inClassExtendsElementLst: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inElementsFromExtendsScope: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inState: &ClassInf::State,
    mut inClassName: ArcStr,
    mut inImpl: bool,
    mut isPartialInst: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>)>;
    let mut outNormalEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outInitialEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outNormalAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outInitialAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
    let mut elts: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>;
    let mut cdefelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut tmpelts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut extendselts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    extendselts = List::map(inExtendsElementLst, &SCodeInstUtil::expandEnumerationClass)?;
    (
        outCache,
        outEnv,
        outIH,
        outMod,
        elts,
        outNormalEqs,
        outInitialEqs,
        outNormalAlgs,
        outInitialAlgs,
        outComments,
    ) = instExtendsAndClassExtendsList2(
        inCache,
        inEnv,
        inIH,
        inMod,
        inPrefix.clone(),
        extendselts,
        inClassExtendsElementLst,
        inElementsFromExtendsScope,
        inState,
        inClassName,
        inImpl,
        isPartialInst,
    )?;
    outElements = List::map(elts, &fnptr!(Util::tuple312, _))?;
    tmpelts = List::map(outElements.clone(), &fnptr!(Util::tuple21, _))?;
    (_, cdefelts, _, _) = InstUtil::splitEltsNoComponents(&tmpelts)?;
    (outCache, outEnv, outIH) = InstUtil::addClassdefsToEnv(
        outCache,
        outEnv,
        outIH,
        inPrefix,
        &cdefelts,
        inImpl,
        Some(outMod.clone()),
        false,
    )?;
    Ok((
        outCache,
        outEnv,
        outIH,
        outMod,
        outElements,
        outNormalEqs,
        outInitialEqs,
        outNormalAlgs,
        outInitialAlgs,
        outComments,
    ))
}

fn instExtendsAndClassExtendsList2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inExtendsElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inClassExtendsElementLst: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inElementsFromExtendsScope: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inState: &ClassInf::State,
    mut inClassName: ArcStr,
    mut inImpl: bool,
    mut isPartialInst: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>;
    let mut outNormalEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outInitialEqs: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outNormalAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outInitialAlgs: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut comments: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
    (
        outCache,
        outEnv,
        outIH,
        outMod,
        outElements,
        outNormalEqs,
        outInitialEqs,
        outNormalAlgs,
        outInitialAlgs,
        comments,
    ) = instExtendsList(
        inCache,
        inEnv.clone(),
        inIH,
        inMod,
        inPrefix,
        inExtendsElementLst,
        inElementsFromExtendsScope,
        inState,
        inClassName,
        inImpl,
        isPartialInst,
    )?;
    (outMod, outElements) = instClassExtendsList(&inEnv, outMod, inClassExtendsElementLst, outElements)?;
    Ok((
        outCache,
        outEnv,
        outIH,
        outMod,
        outElements,
        outNormalEqs,
        outInitialEqs,
        outNormalAlgs,
        outInitialAlgs,
        comments,
    ))
}

fn instClassExtendsList(
    mut inEnv: &FCore::Graph,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inClassExtendsList: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
) -> Result<(
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
)> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>;
    (outMod, outElements) = 'mc: {
        let __mc_input = (inMod, &**inClassExtendsList, inElements);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, Deref @ metamodelica::ListNode::Nil, compelts) => {
                    Ok((emod.clone(), compelts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, Deref @ metamodelica::ListNode::Cons { head: first @ Deref @ SCode::Element::CLASS { name, .. }, tail: rest }, compelts) => {
                    let mut emod = (*emod).clone();
                    let mut compelts = (*compelts).clone();
                    (emod, compelts) = instClassExtendsList2(inEnv, emod.clone(), name.clone(), first.clone(), metamodelica::AsArg::as_arg(&compelts))?;
                    (emod, compelts) = instClassExtendsList(inEnv, emod.clone(), metamodelica::AsArg::as_arg(&rest), compelts.clone())?;
                    Ok((emod.clone(), compelts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { name, .. }, tail: _ }, compelts) => {
                    let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut names: metamodelica::List<ArcStr>;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instClassExtendsList failed ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) })?;
                    Debug::traceln(literal!("  Candidate classes: "))?;
                    els = List::map(compelts.clone(), &fnptr!(Util::tuple31, _))?;
                    names = List::map(els.clone(), &move |__a0: metamodelica::Ref<SCode::Element>| SCodeUtil::elementName(&__a0))?;
                    Debug::traceln(stringDelimitList(names.clone(), literal!(",")))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMod, outElements))
}

fn buildClassExtendsName(mut inEnvPath: &ArcStr, mut inClassName: &ArcStr) -> ArcStr {
    let mut outClassName: ArcStr;
    outClassName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$parent."));
        __mm_s.push_str(&*inClassName);
        __mm_s.push_str(&*literal!(".$env."));
        __mm_s.push_str(&*inEnvPath);
        ArcStr::from(__mm_s)
    };
    outClassName
}

fn instClassExtendsList2(
    mut inEnv: &FCore::Graph,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inName: ArcStr,
    mut inClassExtendsElt: metamodelica::Ref<SCode::Element>,
    mut inElements: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
) -> Result<(
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
)> {
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outElements: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>;
    (outMod, outElements) = 'mc: {
        let __mc_input = (inMod, inName, inClassExtendsElt, &**inElements);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, name1, classExtendsElt, Deref @ metamodelica::ListNode::Cons { head: (cl @ Deref @ SCode::Element::CLASS { name: name2, classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. }, mod1, b), tail: rest }) => {
                    let mut elt: metamodelica::Ref<SCode::Element>;
                    let mut compelt: metamodelica::Ref<SCode::Element>;
                    let mut classDef: metamodelica::Ref<SCode::ClassDef>;
                    let mut classExtendsCdef: metamodelica::Ref<SCode::ClassDef>;
                    let mut partialPrefix1: SCode::Partial;
                    let mut partialPrefix2: SCode::Partial;
                    let mut encapsulatedPrefix1: SCode::Encapsulated;
                    let mut encapsulatedPrefix2: SCode::Encapsulated;
                    let mut restriction1: SCode::Restriction;
                    let mut restriction2: SCode::Restriction;
                    let mut prefixes1: metamodelica::Ref<SCode::Prefixes>;
                    let mut prefixes2: metamodelica::Ref<SCode::Prefixes>;
                    let mut vis2: SCode::Visibility;
                    let mut env_path: ArcStr;
                    let mut externalDecl1: Option<metamodelica::Ref<SCode::ExternalDecl>>;
                    let mut externalDecl2: Option<metamodelica::Ref<SCode::ExternalDecl>>;
                    let mut comment1: metamodelica::Ref<SCode::Comment>;
                    let mut comment2: metamodelica::Ref<SCode::Comment>;
                    let mut els1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut els2: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut nEqn1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut nEqn2: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut inEqn1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut inEqn2: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut nAlg1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut nAlg2: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut inAlg1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut inAlg2: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut inCons1: metamodelica::List<SCode::ConstraintSection>;
                    let mut inCons2: metamodelica::List<SCode::ConstraintSection>;
                    let mut clats: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
                    let mut mods: metamodelica::Ref<SCode::Mod>;
                    let mut info1: SourceInfo;
                    let mut info2: SourceInfo;
                    let mut emod = (*emod).clone();
                    let mut name2 = (*name2).clone();
                    let true = (metamodelica::stringEq(&name1, &name2)) else { return Err("pattern mismatch") };
                    env_path = AbsynUtil::pathString(FGraph::getGraphName(inEnv)?, literal!("."), true, false)?;
                    name2 = buildClassExtendsName(&env_path, metamodelica::AsArg::as_arg(&name2));
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(cl.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa0, encapsulatedPrefix: __pa1, partialPrefix: __pa2, restriction: __pa3, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa4, normalEquationLst: __pa5, initialEquationLst: __pa6, normalAlgorithmLst: __pa7, initialAlgorithmLst: __pa8, constraintLst: __pa9, clsattrs: __pa10, externalDecl: __pa11 }, cmt: __pa12, info: __pa13 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    prefixes2 = metamodelica::Own::own(__pa0);
                    encapsulatedPrefix2 = metamodelica::Own::own(__pa1);
                    partialPrefix2 = metamodelica::Own::own(__pa2);
                    restriction2 = metamodelica::Own::own(__pa3);
                    els2 = metamodelica::Own::own(__pa4);
                    nEqn2 = metamodelica::Own::own(__pa5);
                    inEqn2 = metamodelica::Own::own(__pa6);
                    nAlg2 = metamodelica::Own::own(__pa7);
                    inAlg2 = metamodelica::Own::own(__pa8);
                    inCons2 = metamodelica::Own::own(__pa9);
                    clats = metamodelica::Own::own(__pa10);
                    externalDecl2 = metamodelica::Own::own(__pa11);
                    comment2 = metamodelica::Own::own(__pa12);
                    info2 = metamodelica::Own::own(__pa13);
                    let (__pa15, __pa16, __pa17, __pa18, __pa19, __pa20, __pa21) = ::match_deref::match_deref! { match &(classExtendsElt.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa15, encapsulatedPrefix: __pa16, partialPrefix: __pa17, restriction: __pa18, classDef: __pa19, cmt: __pa20, info: __pa21 } => (__pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    prefixes1 = metamodelica::Own::own(__pa15);
                    encapsulatedPrefix1 = metamodelica::Own::own(__pa16);
                    partialPrefix1 = metamodelica::Own::own(__pa17);
                    restriction1 = metamodelica::Own::own(__pa18);
                    classExtendsCdef = metamodelica::Own::own(__pa19);
                    comment1 = metamodelica::Own::own(__pa20);
                    info1 = metamodelica::Own::own(__pa21);
                    let (__pa22, __pa23, __pa24, __pa25, __pa26, __pa27, __pa28, __pa29) = ::match_deref::match_deref! { match &(classExtendsCdef.clone()) {
                        Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: __pa22, composition: Deref @ SCode::ClassDef::PARTS { elementLst: __pa23, normalEquationLst: __pa24, initialEquationLst: __pa25, normalAlgorithmLst: __pa26, initialAlgorithmLst: __pa27, constraintLst: __pa28, clsattrs: _, externalDecl: __pa29 } } => (__pa22.clone(), __pa23.clone(), __pa24.clone(), __pa25.clone(), __pa26.clone(), __pa27.clone(), __pa28.clone(), __pa29.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    mods = metamodelica::Own::own(__pa22);
                    els1 = metamodelica::Own::own(__pa23);
                    nEqn1 = metamodelica::Own::own(__pa24);
                    inEqn1 = metamodelica::Own::own(__pa25);
                    nAlg1 = metamodelica::Own::own(__pa26);
                    inAlg1 = metamodelica::Own::own(__pa27);
                    inCons1 = metamodelica::Own::own(__pa28);
                    externalDecl1 = metamodelica::Own::own(__pa29);
                    classDef = metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: els2.clone(), normalEquationLst: nEqn2.clone(), initialEquationLst: inEqn2.clone(), normalAlgorithmLst: nAlg2.clone(), initialAlgorithmLst: inAlg2.clone(), constraintLst: inCons2.clone(), clsattrs: clats.clone(), externalDecl: externalDecl2.clone() });
                    compelt = metamodelica::Ref::new(SCode::Element::CLASS { name: name2.clone(), prefixes: prefixes2.clone(), encapsulatedPrefix: encapsulatedPrefix2, partialPrefix: partialPrefix2, restriction: restriction2.clone(), classDef: classDef.clone(), cmt: comment2.clone(), info: info2.clone() });
                    vis2 = SCodeUtil::prefixesVisibility(&prefixes2);
                    elt = metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: metamodelica::Ref::new(Absyn::Path::IDENT { name: name2.clone() }), visibility: vis2, modifications: mods.clone(), ann: None, info: info1.clone() });
                    classDef = metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: metamodelica::cons(elt.clone(), els1.clone()), normalEquationLst: nEqn1.clone(), initialEquationLst: inEqn1.clone(), normalAlgorithmLst: nAlg1.clone(), initialAlgorithmLst: inAlg1.clone(), constraintLst: inCons1.clone(), clsattrs: clats.clone(), externalDecl: externalDecl1.clone() });
                    elt = metamodelica::Ref::new(SCode::Element::CLASS { name: name1.clone(), prefixes: prefixes1.clone(), encapsulatedPrefix: encapsulatedPrefix1, partialPrefix: partialPrefix1, restriction: restriction1.clone(), classDef: classDef.clone(), cmt: comment1.clone(), info: info1.clone() });
                    emod = Mod::renameTopLevelNamedSubMod(emod.clone(), metamodelica::AsArg::as_arg(&name1), name2.clone());
                    Ok((emod.clone(), metamodelica::cons((compelt.clone(), mod1.clone(), b.clone()), metamodelica::cons((elt.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), true), rest.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, name1, classExtendsElt, Deref @ metamodelica::ListNode::Cons { head: (cl @ Deref @ SCode::Element::CLASS { name: name2, classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. }, mod1, b), tail: rest }) => {
                    let mut elt: metamodelica::Ref<SCode::Element>;
                    let mut compelt: metamodelica::Ref<SCode::Element>;
                    let mut classDef: metamodelica::Ref<SCode::ClassDef>;
                    let mut classExtendsCdef: metamodelica::Ref<SCode::ClassDef>;
                    let mut partialPrefix1: SCode::Partial;
                    let mut partialPrefix2: SCode::Partial;
                    let mut encapsulatedPrefix1: SCode::Encapsulated;
                    let mut encapsulatedPrefix2: SCode::Encapsulated;
                    let mut restriction1: SCode::Restriction;
                    let mut restriction2: SCode::Restriction;
                    let mut prefixes1: metamodelica::Ref<SCode::Prefixes>;
                    let mut prefixes2: metamodelica::Ref<SCode::Prefixes>;
                    let mut vis2: SCode::Visibility;
                    let mut env_path: ArcStr;
                    let mut externalDecl1: Option<metamodelica::Ref<SCode::ExternalDecl>>;
                    let mut comment1: metamodelica::Ref<SCode::Comment>;
                    let mut comment2: metamodelica::Ref<SCode::Comment>;
                    let mut els1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut nEqn1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut inEqn1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut nAlg1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut inAlg1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut inCons1: metamodelica::List<SCode::ConstraintSection>;
                    let mut mods: metamodelica::Ref<SCode::Mod>;
                    let mut derivedMod: metamodelica::Ref<SCode::Mod>;
                    let mut info1: SourceInfo;
                    let mut info2: SourceInfo;
                    let mut attrs: SCode::Attributes;
                    let mut derivedTySpec: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut emod = (*emod).clone();
                    let mut name2 = (*name2).clone();
                    let true = (metamodelica::stringEq(&name1, &name2)) else { return Err("pattern mismatch") };
                    env_path = AbsynUtil::pathString(FGraph::getGraphName(inEnv)?, literal!("."), true, false)?;
                    name2 = buildClassExtendsName(&env_path, metamodelica::AsArg::as_arg(&name2));
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(cl.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa0, encapsulatedPrefix: __pa1, partialPrefix: __pa2, restriction: __pa3, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa4, modifications: __pa5, attributes: __pa6 }, cmt: __pa7, info: __pa8 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    prefixes2 = metamodelica::Own::own(__pa0);
                    encapsulatedPrefix2 = metamodelica::Own::own(__pa1);
                    partialPrefix2 = metamodelica::Own::own(__pa2);
                    restriction2 = metamodelica::Own::own(__pa3);
                    derivedTySpec = metamodelica::Own::own(__pa4);
                    derivedMod = metamodelica::Own::own(__pa5);
                    attrs = metamodelica::Own::own(__pa6);
                    comment2 = metamodelica::Own::own(__pa7);
                    info2 = metamodelica::Own::own(__pa8);
                    let (__pa10, __pa11, __pa12, __pa13, __pa14, __pa15, __pa16) = ::match_deref::match_deref! { match &(classExtendsElt.clone()) {
                        Deref @ SCode::Element::CLASS { name: _, prefixes: __pa10, encapsulatedPrefix: __pa11, partialPrefix: __pa12, restriction: __pa13, classDef: __pa14, cmt: __pa15, info: __pa16 } => (__pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    prefixes1 = metamodelica::Own::own(__pa10);
                    encapsulatedPrefix1 = metamodelica::Own::own(__pa11);
                    partialPrefix1 = metamodelica::Own::own(__pa12);
                    restriction1 = metamodelica::Own::own(__pa13);
                    classExtendsCdef = metamodelica::Own::own(__pa14);
                    comment1 = metamodelica::Own::own(__pa15);
                    info1 = metamodelica::Own::own(__pa16);
                    let (__pa17, __pa18, __pa19, __pa20, __pa21, __pa22, __pa23, __pa24) = ::match_deref::match_deref! { match &(classExtendsCdef.clone()) {
                        Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: __pa17, composition: Deref @ SCode::ClassDef::PARTS { elementLst: __pa18, normalEquationLst: __pa19, initialEquationLst: __pa20, normalAlgorithmLst: __pa21, initialAlgorithmLst: __pa22, constraintLst: __pa23, clsattrs: _, externalDecl: __pa24 } } => (__pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone(), __pa23.clone(), __pa24.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    mods = metamodelica::Own::own(__pa17);
                    els1 = metamodelica::Own::own(__pa18);
                    nEqn1 = metamodelica::Own::own(__pa19);
                    inEqn1 = metamodelica::Own::own(__pa20);
                    nAlg1 = metamodelica::Own::own(__pa21);
                    inAlg1 = metamodelica::Own::own(__pa22);
                    inCons1 = metamodelica::Own::own(__pa23);
                    externalDecl1 = metamodelica::Own::own(__pa24);
                    classDef = metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: derivedTySpec.clone(), modifications: derivedMod.clone(), attributes: attrs.clone() });
                    compelt = metamodelica::Ref::new(SCode::Element::CLASS { name: name2.clone(), prefixes: prefixes2.clone(), encapsulatedPrefix: encapsulatedPrefix2, partialPrefix: partialPrefix2, restriction: restriction2.clone(), classDef: classDef.clone(), cmt: comment2.clone(), info: info2.clone() });
                    vis2 = SCodeUtil::prefixesVisibility(&prefixes2);
                    elt = metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: metamodelica::Ref::new(Absyn::Path::IDENT { name: name2.clone() }), visibility: vis2, modifications: mods.clone(), ann: None, info: info1.clone() });
                    classDef = metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: metamodelica::cons(elt.clone(), els1.clone()), normalEquationLst: nEqn1.clone(), initialEquationLst: inEqn1.clone(), normalAlgorithmLst: nAlg1.clone(), initialAlgorithmLst: inAlg1.clone(), constraintLst: inCons1.clone(), clsattrs: metamodelica::nil(), externalDecl: externalDecl1.clone() });
                    elt = metamodelica::Ref::new(SCode::Element::CLASS { name: name1.clone(), prefixes: prefixes1.clone(), encapsulatedPrefix: encapsulatedPrefix1, partialPrefix: partialPrefix1, restriction: restriction1.clone(), classDef: classDef.clone(), cmt: comment1.clone(), info: info1.clone() });
                    emod = Mod::renameTopLevelNamedSubMod(emod.clone(), metamodelica::AsArg::as_arg(&name1), name2.clone());
                    Ok((emod.clone(), metamodelica::cons((compelt.clone(), mod1.clone(), b.clone()), metamodelica::cons((elt.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), true), rest.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (emod, name1, classExtendsElt, Deref @ metamodelica::ListNode::Cons { head: first, tail: rest }) => {
                    let mut emod = (*emod).clone();
                    let mut rest = (*rest).clone();
                    (emod, rest) = instClassExtendsList2(inEnv, emod.clone(), name1.clone(), classExtendsElt.clone(), metamodelica::AsArg::as_arg(&rest))?;
                    Ok((emod.clone(), metamodelica::cons(first.clone(), rest.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::ListNode::Nil) => {
                    Debug::traceln(literal!("TODO: Make a proper Error message here - Inst.instClassExtendsList2 couldn't find the class to extend"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outMod, outElements))
}

pub(crate) fn instDerivedClasses(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inBoolean: bool,
    mut inInfo: SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv1: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSCodeElementLst2: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outSCodeEquationLst3: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outSCodeEquationLst4: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outSCodeAlgorithmLst5: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outSCodeAlgorithmLst6: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
    (
        outCache,
        outEnv1,
        outIH,
        outSCodeElementLst2,
        outSCodeEquationLst3,
        outSCodeEquationLst4,
        outSCodeAlgorithmLst5,
        outSCodeAlgorithmLst6,
        outMod,
        outComments,
    ) = instDerivedClassesWork(
        inCache, inEnv, inIH, inMod, inPrefix, inClass, inBoolean, inInfo, false, 0,
    )?;
    Ok((
        outCache,
        outEnv1,
        outIH,
        outSCodeElementLst2,
        outSCodeEquationLst3,
        outSCodeEquationLst4,
        outSCodeAlgorithmLst5,
        outSCodeAlgorithmLst6,
        outMod,
        outComments,
    ))
}

fn instDerivedClassesWork(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inBoolean: bool,
    mut inInfo: SourceInfo,
    mut overflow: bool,
    mut numIter: i32,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    metamodelica::Ref<DAE::Mod>,
    metamodelica::List<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv1: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSCodeElementLst2: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outSCodeEquationLst3: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outSCodeEquationLst4: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outSCodeAlgorithmLst5: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outSCodeAlgorithmLst6: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
    let mut outMod: metamodelica::Ref<DAE::Mod>;
    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = metamodelica::nil();
    (
        outCache,
        outEnv1,
        outIH,
        outSCodeElementLst2,
        outSCodeEquationLst3,
        outSCodeEquationLst4,
        outSCodeAlgorithmLst5,
        outSCodeAlgorithmLst6,
        outMod,
        outComments,
    ) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv.clone(),
            inIH,
            inMod.clone(),
            inPrefix,
            &**inClass,
            inBoolean,
            inInfo.clone(),
            overflow,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, Deref @ SCode::Element::CLASS { name, .. }, _, _, _) => {
                    let true = (InstUtil::isBuiltInClass(metamodelica::AsArg::as_arg(&name))?) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), env.clone(), ih.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inMod.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, _, Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elt, normalEquationLst: eq, initialEquationLst: ieq, normalAlgorithmLst: alg, initialAlgorithmLst: ialg, externalDecl: extdecl, .. }, .. }, _, info, _) => {
                    Error::assertionOrAddSourceMessage((extdecl).is_none(), &(Error::EXTENDS_EXTERNAL.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), elt.clone(), eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), inMod.clone(), list![var_field!((**inClass).cmt, SCode::Element::CLASS).clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, Deref @ SCode::Element::CLASS { info, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, arrayDim: _ }, modifications: dmod, .. }, .. }, r#impl, _, false) => {
                    let mut elt: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut cenv: FCore::Graph;
                    let mut daeDMOD: metamodelica::Ref<DAE::Mod>;
                    let mut eq: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut ieq: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut alg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut ialg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut dmod = (*dmod).clone();
                    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = outComments.clone();
                    (cache, c, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&tp), Some(info.clone()))?;
                    dmod = InstUtil::chainRedeclares(metamodelica::AsArg::as_arg(&r#mod), dmod.clone());
                    (cache, daeDMOD) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), pre.clone(), dmod.clone(), r#impl.clone(), Mod::ModScope::DERIVED { path: tp.clone() }, info.clone())?;
                    r#mod = Mod::merge(r#mod.clone(), daeDMOD.clone(), literal!(""), true)?;
                    (cache, env, ih, elt, eq, ieq, alg, ialg, r#mod, outComments) = instDerivedClassesWork(cache.clone(), cenv.clone(), ih.clone(), r#mod.clone(), pre.clone(), &c, r#impl.clone(), info.clone(), numIter >= Global::recursionDepthLimit.clone(), numIter + 1)?;
                    Ok(((cache.clone(), env.clone(), ih.clone(), elt.clone(), eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), r#mod.clone(), metamodelica::cons(var_field!((**inClass).cmt, SCode::Element::CLASS).clone(), outComments.clone())), outComments.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComments = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, Deref @ SCode::Element::CLASS { name: n, prefixes, classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst }, cmt, info, .. }, r#impl, _, false) => {
                    let mut elt: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut eq: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut ieq: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut alg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut ialg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut r#mod = (*r#mod).clone();
                    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>> = outComments.clone();
                    c = SCodeInstUtil::expandEnumeration(n.clone(), enumLst.clone(), prefixes.clone(), cmt.clone(), info.clone())?;
                    (cache, env, ih, elt, eq, ieq, alg, ialg, r#mod, outComments) = instDerivedClassesWork(cache.clone(), env.clone(), ih.clone(), r#mod.clone(), pre.clone(), &c, r#impl.clone(), info.clone(), numIter >= Global::recursionDepthLimit.clone(), numIter + 1)?;
                    Ok(((cache.clone(), env.clone(), ih.clone(), elt.clone(), eq.clone(), ieq.clone(), alg.clone(), ialg.clone(), r#mod.clone(), outComments.clone()), outComments.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComments = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, true) => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    str1 = SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?;
                    str2 = FGraph::printGraphPathStr(&inEnv);
                    Error::addSourceMessage(&(Error::RECURSION_DEPTH_DERIVED.clone()), list![str1.clone(), str2.clone()], &inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Inst.instDerivedClasses failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((
        outCache,
        outEnv1,
        outIH,
        outSCodeElementLst2,
        outSCodeEquationLst3,
        outSCodeEquationLst4,
        outSCodeAlgorithmLst5,
        outSCodeAlgorithmLst6,
        outMod,
        outComments,
    ))
}

fn noImportElements(
    mut inElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outElements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (inElements).into_iter().cloned() {
            if !(!(SCodeUtil::elementIsImport(&(e.clone())))) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElements
}

fn updateComponentsAndClassdefs(
    mut inComponents: &metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inEnv: FCore::Graph,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>,
    metamodelica::Ref<DAE::Mod>,
)> {
    let mut outComponents: metamodelica::List<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)>;
    let mut outRestMod: metamodelica::Ref<DAE::Mod>;
    (outComponents, outRestMod) = List::map1Fold(
        inComponents,
        &move |__a0: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool),
               __a1: FCore::Graph,
               __a2: metamodelica::Ref<DAE::Mod>| updateComponentsAndClassdefs2(__a0, &__a1, __a2),
        inEnv,
        inMod,
    )?;
    Ok((outComponents, outRestMod))
}

fn updateComponentsAndClassdefs2(
    mut inComponent: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool),
    mut inEnv: &FCore::Graph,
    mut inMod: metamodelica::Ref<DAE::Mod>,
) -> Result<(
    (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool),
    metamodelica::Ref<DAE::Mod>,
)> {
    let mut outComponent: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool) = (
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default(),
        metamodelica::Ref::new(DAE::Mod::NOMOD),
        false,
    );
    let mut outRestMod: metamodelica::Ref<DAE::Mod>;
    let mut el: metamodelica::Ref<SCode::Element>;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut b: bool;
    (el, r#mod, b) = inComponent.clone();
    (outComponent, outRestMod) = 'mc: {
        let __mc_input = &*el;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::COMPONENT { .. } => {
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    let mut mod_rest: metamodelica::Ref<DAE::Mod>;
                    cmod = Mod::lookupCompModificationFromEqu(&inMod, var_field!((*el).name, SCode::Element::COMPONENT).clone())?;
                    cmod = Mod::merge(cmod.clone(), r#mod.clone(), var_field!((*el).name, SCode::Element::COMPONENT).clone(), false)?;
                    mod_rest = inMod.clone();
                    Ok(((el.clone(), cmod.clone(), b), mod_rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::EXTENDS { .. } => {
                    Ok((inComponent.clone(), inMod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::IMPORT { .. } => {
                    Ok(((el.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), b), inMod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. } => {
                    let mut comp: metamodelica::Ref<SCode::Element>;
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    let mut mod_rest: metamodelica::Ref<DAE::Mod>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Mod::lookupCompModification(&inMod, var_field!((*el).name, SCode::Element::CLASS).clone())?) {
                        Deref @ DAE::Mod::REDECL { element: __pa0, r#mod: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    comp = metamodelica::Own::own(__pa0);
                    cmod = metamodelica::Own::own(__pa1);
                    mod_rest = inMod.clone();
                    cmod = Mod::merge(cmod.clone(), r#mod.clone(), var_field!((*el).name, SCode::Element::CLASS).clone(), false)?;
                    comp = SCodeUtil::mergeWithOriginal(comp.clone(), &el);
                    Ok(((comp.clone(), cmod.clone(), b), mod_rest.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let mut cmod: metamodelica::Ref<DAE::Mod>;
                    let mut outComponent: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool) = outComponent.clone();
                    cmod = Mod::lookupCompModification(&inMod, var_field!((*el).name, SCode::Element::CLASS).clone())?;
                    outComponent = if (cmod.clone() == openmodelica_frontend_types::DAE::Mod::interned_NOMOD()) {inComponent.clone()} else {(el.clone(), cmod.clone(), b)};
                    Ok(((outComponent.clone(), inMod.clone()), outComponent.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outComponent = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstExtends.updateComponentsAndClassdefs2 failed on:\n")); __mm_s.push_str(&*literal!("env = ")); __mm_s.push_str(&*FGraph::printGraphPathStr(inEnv)); __mm_s.push_str(&*literal!("\nmod = ")); __mm_s.push_str(&*Mod::printModStr(&inMod)?); __mm_s.push_str(&*literal!("\ncmod = ")); __mm_s.push_str(&*Mod::printModStr(&r#mod)?); __mm_s.push_str(&*literal!("\nbool = ")); __mm_s.push_str(&*boolString(b)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outComponent, outRestMod))
}

fn getLocalIdentList<Type_A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ielts: &metamodelica::List<Type_A>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
    mut getIdent: &dyn ::std::ops::Fn(Type_A, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<AvlSetString::Tree>>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    pub type getIdentFn<Type_A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<metamodelica::Ref<AvlSetString::Tree>>
            + 'static,
    >;

    let mut tree: metamodelica::Ref<AvlSetString::Tree> = tree;
    for mut elt in &**ielts {
        tree = getIdent(elt.clone(), tree)?;
    }
    Ok(tree)
}

fn getLocalIdentElementTpl(
    mut eltTpl: &(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool),
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut tree: metamodelica::Ref<AvlSetString::Tree> = tree;
    let mut elt: metamodelica::Ref<SCode::Element>;
    (elt, _, _) = eltTpl.clone();
    tree = getLocalIdentElement(&elt, tree)?;
    Ok(tree)
}

fn getLocalIdentElement(
    mut elt: &metamodelica::Ref<SCode::Element>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<AvlSetString::Tree>> {
    let mut tree: metamodelica::Ref<AvlSetString::Tree> = tree;
    tree = (match &**elt {
        SCode::Element::COMPONENT { name: id, .. } => AvlSetString::add(tree, id)?,
        SCode::Element::CLASS { name: id, .. } => AvlSetString::add(tree, id)?,
        _ => tree,
    });
    Ok(tree)
}

fn fixLocalIdent(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut elt: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool),
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<(metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool)> {
    let mut elt: (metamodelica::Ref<SCode::Element>, metamodelica::Ref<DAE::Mod>, bool) = elt;
    let mut elt1: metamodelica::Ref<SCode::Element>;
    let mut elt2: metamodelica::Ref<SCode::Element>;
    let mut r#mod: metamodelica::Ref<DAE::Mod>;
    let mut b: bool;
    (elt1, r#mod, b) = elt.clone();
    elt2 = fixElement(inCache.clone(), inEnv, elt1.clone(), tree)?;
    if !(referenceEq(&*(elt1), &*(&*elt2))) || !(b) {
        elt = (elt2, r#mod, true);
    }
    Ok(elt)
}

fn fixElement(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inElt: metamodelica::Ref<SCode::Element>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElts: metamodelica::Ref<SCode::Element>;
    outElts = 'mc: {
        let __mc_input = (inEnv, inElt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, elt @ Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, .. }) => {
                    let mut name: ArcStr;
                    let mut prefixes: metamodelica::Ref<SCode::Prefixes>;
                    let mut typeSpec1: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut typeSpec2: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut modifications1: metamodelica::Ref<SCode::Mod>;
                    let mut modifications2: metamodelica::Ref<SCode::Mod>;
                    let mut comment: metamodelica::Ref<SCode::Comment>;
                    let mut condition: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut info: SourceInfo;
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut elt2: metamodelica::Ref<SCode::Element>;
                    let mut attr: SCode::Attributes;
                    let mut env = (*env).clone();
                    let (__pa8, __pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa9) = ::match_deref::match_deref! { match &(Lookup::lookupIdentLocal(metamodelica::arrayGet(inCache.clone(), 1)?, metamodelica::AsArg::as_arg(&env), var_field!((**elt).name, SCode::Element::COMPONENT).clone())?) {
                        (_, _, __pa8 @ Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: __pa1, attributes: __pa2 @ SCode::Attributes { .. }, typeSpec: __pa3, modifications: __pa4, comment: __pa5, condition: __pa6, info: __pa7 }, _, _, __pa9) => (__pa8.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa9.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    prefixes = metamodelica::Own::own(__pa1);
                    attr = metamodelica::Own::own(__pa2);
                    typeSpec1 = metamodelica::Own::own(__pa3);
                    modifications1 = metamodelica::Own::own(__pa4);
                    comment = metamodelica::Own::own(__pa5);
                    condition = metamodelica::Own::own(__pa6);
                    info = metamodelica::Own::own(__pa7);
                    elt2 = metamodelica::Own::own(__pa8);
                    env = metamodelica::Own::own(__pa9);
                    modifications2 = fixModifications(inCache.clone(), env.clone(), modifications1.clone(), tree.clone())?;
                    typeSpec2 = fixTypeSpec(inCache.clone(), env.clone(), typeSpec1.clone(), tree.clone())?;
                    ad = fixArrayDim(inCache.clone(), metamodelica::AsArg::as_arg(&env), attr.arrayDims.clone(), &tree)?;
                    if !(metamodelica::ReferenceEq::reference_eq(&(ad), &(attr.arrayDims.clone()))) {
                        attr.arrayDims = ad.clone();
                    }
                    if !(metamodelica::ReferenceEq::reference_eq(&(ad), &(attr.arrayDims.clone())) && referenceEq(&*(&*typeSpec1),&*(&*typeSpec2)) && referenceEq(&*(&*modifications1),&*(&*modifications2))) {
                        elt2 = metamodelica::Ref::new(SCode::Element::COMPONENT { name: name.clone(), prefixes: prefixes.clone(), attributes: attr.clone(), typeSpec: typeSpec2.clone(), modifications: modifications2.clone(), comment: comment.clone(), condition: condition.clone(), info: info.clone() });
                    }
                    Ok(elt2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, elt @ Deref @ SCode::Element::COMPONENT { attributes: attr, .. }) => {
                    let mut typeSpec2: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut modifications2: metamodelica::Ref<SCode::Mod>;
                    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut elt = (*elt).clone();
                    let mut attr = (*attr).clone();
                    modifications2 = fixModifications(inCache.clone(), env.clone(), var_field!((*elt).modifications, SCode::Element::COMPONENT).clone(), tree.clone())?;
                    typeSpec2 = fixTypeSpec(inCache.clone(), env.clone(), var_field!((*elt).typeSpec, SCode::Element::COMPONENT).clone(), tree.clone())?;
                    ad = fixArrayDim(inCache.clone(), metamodelica::AsArg::as_arg(&env), attr.arrayDims.clone(), &tree)?;
                    if !(metamodelica::ReferenceEq::reference_eq(&(ad), &(attr.arrayDims.clone()))) {
                        attr.arrayDims = ad.clone();
                    }
                    if !(metamodelica::ReferenceEq::reference_eq(&(ad), &(attr.arrayDims.clone())) && referenceEq(&*(var_field!((*elt).typeSpec, SCode::Element::COMPONENT).clone()),&*(&*typeSpec2)) && referenceEq(&*(var_field!((*elt).modifications, SCode::Element::COMPONENT).clone()),&*(&*modifications2))) {
                        elt = metamodelica::Ref::new(SCode::Element::COMPONENT { name: var_field!((*elt).name, SCode::Element::COMPONENT).clone(), prefixes: var_field!((*elt).prefixes, SCode::Element::COMPONENT).clone(), attributes: attr.clone(), typeSpec: typeSpec2.clone(), modifications: modifications2.clone(), comment: var_field!((*elt).comment, SCode::Element::COMPONENT).clone(), condition: var_field!((*elt).condition, SCode::Element::COMPONENT).clone(), info: var_field!((*elt).info, SCode::Element::COMPONENT).clone() });
                    }
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes: prefixes @ Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, partialPrefix, restriction, classDef: _, cmt: comment, info }) => {
                    let mut classDef1: metamodelica::Ref<SCode::ClassDef>;
                    let mut classDef2: metamodelica::Ref<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    let mut prefixes = (*prefixes).clone();
                    let mut partialPrefix = (*partialPrefix).clone();
                    let mut restriction = (*restriction).clone();
                    let mut comment = (*comment).clone();
                    let mut info = (*info).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupClassLocal(env.clone(), name.clone())?) {
                        (Deref @ SCode::Element::CLASS { prefixes: __pa0, partialPrefix: __pa1, restriction: __pa2, cmt: __pa3, info: __pa4, classDef: __pa5, .. }, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    prefixes = metamodelica::Own::own(__pa0);
                    partialPrefix = metamodelica::Own::own(__pa1);
                    restriction = metamodelica::Own::own(__pa2);
                    comment = metamodelica::Own::own(__pa3);
                    info = metamodelica::Own::own(__pa4);
                    classDef1 = metamodelica::Own::own(__pa5);
                    env = metamodelica::Own::own(__pa6);
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, name.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restriction)))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(&*classDef1),&*(&*classDef2))) {inElt.clone()} else {metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. }, partialPrefix, restriction, classDef: classDef1, cmt: comment, info }) => {
                    let mut classDef2: metamodelica::Ref<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, name.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restriction)))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(classDef1.clone()),&*(&*classDef2))) {inElt.clone()} else {metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes: prefixes @ Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED { .. }, partialPrefix, restriction, classDef: _, cmt: comment, info }) => {
                    let mut classDef1: metamodelica::Ref<SCode::ClassDef>;
                    let mut classDef2: metamodelica::Ref<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    let mut prefixes = (*prefixes).clone();
                    let mut partialPrefix = (*partialPrefix).clone();
                    let mut restriction = (*restriction).clone();
                    let mut comment = (*comment).clone();
                    let mut info = (*info).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Lookup::lookupClassLocal(env.clone(), name.clone())?) {
                        (Deref @ SCode::Element::CLASS { prefixes: __pa0, partialPrefix: __pa1, restriction: __pa2, cmt: __pa3, info: __pa4, classDef: __pa5, .. }, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    prefixes = metamodelica::Own::own(__pa0);
                    partialPrefix = metamodelica::Own::own(__pa1);
                    restriction = metamodelica::Own::own(__pa2);
                    comment = metamodelica::Own::own(__pa3);
                    info = metamodelica::Own::own(__pa4);
                    classDef1 = metamodelica::Own::own(__pa5);
                    env = metamodelica::Own::own(__pa6);
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, name.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restriction)))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(&*classDef1),&*(&*classDef2))) {inElt.clone()} else {metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED { .. }, partialPrefix, restriction, classDef: classDef1, cmt: comment, info }) => {
                    let mut classDef2: metamodelica::Ref<SCode::ClassDef>;
                    let mut env = (*env).clone();
                    env = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, name.clone(), FGraph::restrictionToScopeType(metamodelica::AsArg::as_arg(&restriction)))?;
                    classDef2 = fixClassdef(inCache.clone(), env.clone(), classDef1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(classDef1.clone()),&*(&*classDef2))) {inElt.clone()} else {metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: partialPrefix.clone(), restriction: restriction.clone(), classDef: classDef2.clone(), cmt: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::Element::EXTENDS { baseClassPath: extendsPath1, visibility: vis, modifications: modifications1, ann: optAnnotation, info }) => {
                    let mut modifications2: metamodelica::Ref<SCode::Mod>;
                    let mut extendsPath2: metamodelica::Ref<Absyn::Path>;
                    extendsPath2 = fixPath(inCache.clone(), env.clone(), extendsPath1.clone(), tree.clone());
                    modifications2 = fixModifications(inCache.clone(), env.clone(), modifications1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(extendsPath1.clone()),&*(&*extendsPath2)) && referenceEq(&*(modifications1.clone()),&*(&*modifications2))) {inElt.clone()} else {metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: extendsPath2.clone(), visibility: vis.clone(), modifications: modifications2.clone(), ann: optAnnotation.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ SCode::Element::IMPORT { .. }) => {
                    Ok(inElt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, elt) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixElement failed: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(elt.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElts)
}

fn fixClassdef(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inCd: metamodelica::Ref<SCode::ClassDef>,
    mut inTree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut outCd: metamodelica::Ref<SCode::ClassDef>;
    let mut tree: metamodelica::Ref<AvlSetString::Tree> = inTree.clone();
    outCd = 'mc: {
        let __mc_input = (inEnv, inCd.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::ClassDef::PARTS { elementLst: elts, normalEquationLst: ne, initialEquationLst: ie, normalAlgorithmLst: na, initialAlgorithmLst: ia, constraintLst: nc, clsattrs: clats, externalDecl: ed }) => {
                    let mut elts_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut ne_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut ie_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut na_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut ia_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut nc_1: metamodelica::List<SCode::ConstraintSection>;
                    let mut tree: metamodelica::Ref<AvlSetString::Tree> = tree.clone();
                    tree = getLocalIdentList(metamodelica::AsArg::as_arg(&elts), tree.clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::Ref<AvlSetString::Tree>| getLocalIdentElement(&__a0, __a1))?;
                    elts_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), elts.clone(), &tree, (std::sync::Arc::new(fixElement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Element>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Element>> + 'static>))?;
                    ne_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), ne.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::Equation>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Equation>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static>))?;
                    ie_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), ie.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::Equation>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Equation>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static>))?;
                    na_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), na.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::AlgorithmSection>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixAlgorithm(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::AlgorithmSection>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> + 'static>))?;
                    ia_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), ia.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::AlgorithmSection>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixAlgorithm(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::AlgorithmSection>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> + 'static>))?;
                    nc_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), nc.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: SCode::ConstraintSection, __a3: metamodelica::Ref<AvlSetString::Tree>| fixConstraint(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, SCode::ConstraintSection, metamodelica::Ref<AvlSetString::Tree>) -> Result<SCode::ConstraintSection> + 'static>))?;
                    Ok((if (metamodelica::ReferenceEq::reference_eq(&(elts.clone()), &(elts_1)) && metamodelica::ReferenceEq::reference_eq(&(ne.clone()), &(ne_1)) && metamodelica::ReferenceEq::reference_eq(&(ie.clone()), &(ie_1)) && metamodelica::ReferenceEq::reference_eq(&(na.clone()), &(na_1)) && metamodelica::ReferenceEq::reference_eq(&(ia.clone()), &(ia_1)) && metamodelica::ReferenceEq::reference_eq(&(nc.clone()), &(nc_1))) {inCd.clone()} else {metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: elts_1.clone(), normalEquationLst: ne_1.clone(), initialEquationLst: ie_1.clone(), normalAlgorithmLst: na_1.clone(), initialAlgorithmLst: ia_1.clone(), constraintLst: nc_1.clone(), clsattrs: clats.clone(), externalDecl: ed.clone() })}, tree.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            tree = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod, composition: cd @ Deref @ SCode::ClassDef::PARTS { elementLst: elts, normalEquationLst: ne, initialEquationLst: ie, normalAlgorithmLst: na, initialAlgorithmLst: ia, constraintLst: nc, clsattrs: clats, externalDecl: ed } }) => {
                    let mut elts_1: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut ne_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut ie_1: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut na_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut ia_1: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
                    let mut nc_1: metamodelica::List<SCode::ConstraintSection>;
                    let mut mod_1: metamodelica::Ref<SCode::Mod>;
                    let mut cd_1: metamodelica::Ref<SCode::ClassDef>;
                    mod_1 = fixModifications(cache.clone(), env.clone(), r#mod.clone(), inTree.clone())?;
                    elts_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), elts.clone(), &tree, (std::sync::Arc::new(fixElement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Element>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Element>> + 'static>))?;
                    ne_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), ne.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::Equation>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Equation>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static>))?;
                    ie_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), ie.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::Equation>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Equation>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static>))?;
                    na_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), na.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::AlgorithmSection>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixAlgorithm(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::AlgorithmSection>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> + 'static>))?;
                    ia_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), ia.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::Ref<SCode::AlgorithmSection>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixAlgorithm(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::AlgorithmSection>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> + 'static>))?;
                    nc_1 = fixList(cache.clone(), metamodelica::AsArg::as_arg(&env), nc.clone(), &tree, (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: SCode::ConstraintSection, __a3: metamodelica::Ref<AvlSetString::Tree>| fixConstraint(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, SCode::ConstraintSection, metamodelica::Ref<AvlSetString::Tree>) -> Result<SCode::ConstraintSection> + 'static>))?;
                    cd_1 = if (metamodelica::ReferenceEq::reference_eq(&(elts.clone()), &(elts_1)) && metamodelica::ReferenceEq::reference_eq(&(ne.clone()), &(ne_1)) && metamodelica::ReferenceEq::reference_eq(&(ie.clone()), &(ie_1)) && metamodelica::ReferenceEq::reference_eq(&(na.clone()), &(na_1)) && metamodelica::ReferenceEq::reference_eq(&(ia.clone()), &(ia_1)) && metamodelica::ReferenceEq::reference_eq(&(nc.clone()), &(nc_1))) {cd.clone()} else {metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: elts_1.clone(), normalEquationLst: ne_1.clone(), initialEquationLst: ie_1.clone(), normalAlgorithmLst: na_1.clone(), initialAlgorithmLst: ia_1.clone(), constraintLst: nc_1.clone(), clsattrs: clats.clone(), externalDecl: ed.clone() })};
                    Ok(if (referenceEq(&*(cd.clone()),&*(&*cd_1)) && referenceEq(&*(r#mod.clone()),&*(&*mod_1))) {inCd.clone()} else {metamodelica::Ref::new(SCode::ClassDef::CLASS_EXTENDS { modifications: mod_1.clone(), composition: cd_1.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ SCode::ClassDef::DERIVED { typeSpec: ts, modifications: r#mod, attributes: attr }) => {
                    let mut ts_1: metamodelica::Ref<Absyn::TypeSpec>;
                    let mut mod_1: metamodelica::Ref<SCode::Mod>;
                    ts_1 = fixTypeSpec(cache.clone(), env.clone(), ts.clone(), tree.clone())?;
                    mod_1 = fixModifications(cache.clone(), env.clone(), r#mod.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(ts.clone()),&*(&*ts_1)) && referenceEq(&*(r#mod.clone()),&*(&*mod_1))) {inCd.clone()} else {metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: ts_1.clone(), modifications: mod_1.clone(), attributes: attr.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd @ Deref @ SCode::ClassDef::ENUMERATION { .. }) => {
                    Ok(cd.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd @ Deref @ SCode::ClassDef::OVERLOAD { .. }) => {
                    Ok(cd.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd @ Deref @ SCode::ClassDef::PDER { .. }) => {
                    Ok(cd.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, cd) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixClassDef failed: ")); __mm_s.push_str(&*SCodeDump::classDefStr(cd.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCd)
}

fn fixEquation(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inEeq: &metamodelica::Ref<SCode::Equation>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut outEeq: metamodelica::Ref<SCode::Equation>;
    outEeq = (match &**inEeq {
        SCode::Equation::EQ_IF {
            condition: expl,
            thenBranch: eqll,
            elseBranch: eql,
            comment,
            info,
        } => {
            let mut expl = (*expl).clone();
            let mut eqll = (*eqll).clone();
            let mut eql = (*eql).clone();
            expl = fixList(
                cache.clone(),
                &inEnv,
                expl.clone(),
                &tree,
                (std::sync::Arc::new(fixExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<Absyn::Exp>>
                            + 'static,
                    >),
            )?;
            eqll = fixListList(
                cache.clone(),
                &inEnv,
                eqll.clone(),
                &tree,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Array<FCore::Cache>,
                          __a1: FCore::Graph,
                          __a2: metamodelica::Ref<SCode::Equation>,
                          __a3: metamodelica::Ref<AvlSetString::Tree>| {
                        fixEquation(__a0, __a1, &__a2, __a3)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<SCode::Equation>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<SCode::Equation>>
                            + 'static,
                    >),
            )?;
            eql = fixList(
                cache.clone(),
                &inEnv,
                eql.clone(),
                &tree,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Array<FCore::Cache>,
                          __a1: FCore::Graph,
                          __a2: metamodelica::Ref<SCode::Equation>,
                          __a3: metamodelica::Ref<AvlSetString::Tree>| {
                        fixEquation(__a0, __a1, &__a2, __a3)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<SCode::Equation>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<SCode::Equation>>
                            + 'static,
                    >),
            )?;
            metamodelica::Ref::new(SCode::Equation::EQ_IF {
                condition: expl.clone(),
                thenBranch: eqll.clone(),
                elseBranch: eql.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_EQUALS {
            expLeft: exp1,
            expRight: exp2,
            comment,
            info,
        } => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp2 = fixExp(cache.clone(), inEnv, exp2.clone(), tree)?;
            metamodelica::Ref::new(SCode::Equation::EQ_EQUALS {
                expLeft: exp1.clone(),
                expRight: exp2.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_PDE {
            expLeft: exp1,
            expRight: exp2,
            domain: cref,
            comment,
            info,
        } => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            let mut cref = (*cref).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp2 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
            cref = fixCref(cache.clone(), inEnv, cref.clone(), &tree);
            metamodelica::Ref::new(SCode::Equation::EQ_PDE {
                expLeft: exp1.clone(),
                expRight: exp2.clone(),
                domain: cref.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_CONNECT {
            crefLeft: cref1,
            crefRight: cref2,
            comment,
            info,
        } => {
            let mut cref1 = (*cref1).clone();
            let mut cref2 = (*cref2).clone();
            cref1 = fixCref(cache.clone(), inEnv.clone(), cref1.clone(), &tree);
            cref2 = fixCref(cache.clone(), inEnv, cref2.clone(), &tree);
            metamodelica::Ref::new(SCode::Equation::EQ_CONNECT {
                crefLeft: cref1.clone(),
                crefRight: cref2.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_FOR {
            index: id,
            range: optExp,
            eEquationLst: eql,
            comment,
            info,
        } => {
            let mut optExp = (*optExp).clone();
            let mut eql = (*eql).clone();
            optExp = fixOption(cache.clone(), inEnv.clone(), optExp.clone(), tree.clone(), &fixExp)?;
            eql = fixList(
                cache.clone(),
                &inEnv,
                eql.clone(),
                &tree,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Array<FCore::Cache>,
                          __a1: FCore::Graph,
                          __a2: metamodelica::Ref<SCode::Equation>,
                          __a3: metamodelica::Ref<AvlSetString::Tree>| {
                        fixEquation(__a0, __a1, &__a2, __a3)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<SCode::Equation>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<SCode::Equation>>
                            + 'static,
                    >),
            )?;
            metamodelica::Ref::new(SCode::Equation::EQ_FOR {
                index: id.clone(),
                range: optExp.clone(),
                eEquationLst: eql.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_WHEN {
            condition: exp,
            eEquationLst: eql,
            elseBranches: whenlst,
            comment,
            info,
        } => {
            let mut exp = (*exp).clone();
            let mut eql = (*eql).clone();
            let mut whenlst = (*whenlst).clone();
            exp = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
            eql = fixList(
                cache.clone(),
                &inEnv,
                eql.clone(),
                &tree,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Array<FCore::Cache>,
                          __a1: FCore::Graph,
                          __a2: metamodelica::Ref<SCode::Equation>,
                          __a3: metamodelica::Ref<AvlSetString::Tree>| {
                        fixEquation(__a0, __a1, &__a2, __a3)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<SCode::Equation>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<SCode::Equation>>
                            + 'static,
                    >),
            )?;
            whenlst = fixListTuple2(
                cache.clone(),
                &inEnv,
                whenlst.clone(),
                &tree,
                (std::sync::Arc::new(fixExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<Absyn::Exp>>
                            + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Array<FCore::Cache>,
                          __a1: FCore::Graph,
                          __a2: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
                          __a3: metamodelica::Ref<AvlSetString::Tree>| {
                        fixListEquation(__a0, &__a1, __a2, &__a3)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::List<metamodelica::Ref<SCode::Equation>>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            )
                                -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>>
                            + 'static,
                    >),
            )?;
            metamodelica::Ref::new(SCode::Equation::EQ_WHEN {
                condition: exp.clone(),
                eEquationLst: eql.clone(),
                elseBranches: whenlst.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_ASSERT {
            condition: exp1,
            message: exp2,
            level: exp3,
            comment,
            info,
        } => {
            let mut exp1 = (*exp1).clone();
            let mut exp2 = (*exp2).clone();
            let mut exp3 = (*exp3).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp2 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
            exp3 = fixExp(cache.clone(), inEnv, exp3.clone(), tree)?;
            metamodelica::Ref::new(SCode::Equation::EQ_ASSERT {
                condition: exp1.clone(),
                message: exp2.clone(),
                level: exp3.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_TERMINATE {
            message: exp,
            comment,
            info,
        } => {
            let mut exp = (*exp).clone();
            exp = fixExp(cache.clone(), inEnv, exp.clone(), tree)?;
            metamodelica::Ref::new(SCode::Equation::EQ_TERMINATE {
                message: exp.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_REINIT {
            cref: exp1,
            expReinit: exp,
            comment,
            info,
        } => {
            let mut exp1 = (*exp1).clone();
            let mut exp = (*exp).clone();
            exp1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
            exp = fixExp(cache.clone(), inEnv, exp.clone(), tree)?;
            metamodelica::Ref::new(SCode::Equation::EQ_REINIT {
                cref: exp1.clone(),
                expReinit: exp.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
        SCode::Equation::EQ_NORETCALL { exp, comment, info } => {
            let mut exp = (*exp).clone();
            exp = fixExp(cache.clone(), inEnv, exp.clone(), tree)?;
            metamodelica::Ref::new(SCode::Equation::EQ_NORETCALL {
                exp: exp.clone(),
                comment: comment.clone(),
                info: info.clone(),
            })
        }
    });
    Ok(outEeq)
}

fn fixListEquation(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut env: &FCore::Graph,
    mut eeq: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    let mut outEeq: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    outEeq = fixList(
        cache.clone(),
        env,
        eeq,
        tree,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Array<FCore::Cache>,
                  __a1: FCore::Graph,
                  __a2: metamodelica::Ref<SCode::Equation>,
                  __a3: metamodelica::Ref<AvlSetString::Tree>| fixEquation(__a0, __a1, &__a2, __a3),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        metamodelica::Ref<SCode::Equation>,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<metamodelica::Ref<SCode::Equation>>
                    + 'static,
            >),
    )?;
    Ok(outEeq)
}

fn fixAlgorithm(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: &FCore::Graph,
    mut inAlg: metamodelica::Ref<SCode::AlgorithmSection>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> {
    let mut outAlg: metamodelica::Ref<SCode::AlgorithmSection>;
    let mut stmts1: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let mut stmts2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let __arc1 = inAlg.clone();
    let SCode::ALGORITHM { statements: __pa0 } = &*__arc1;
    stmts1 = metamodelica::Own::own(__pa0);
    stmts2 = fixList(
        inCache.clone(),
        inEnv,
        stmts1.clone(),
        tree,
        (std::sync::Arc::new(fixStatement)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        metamodelica::Ref<SCode::Statement>,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<metamodelica::Ref<SCode::Statement>>
                    + 'static,
            >),
    )?;
    outAlg = if (metamodelica::ReferenceEq::reference_eq(&(stmts1), &(stmts2))) {
        inAlg
    } else {
        metamodelica::Ref::new(SCode::AlgorithmSection { statements: stmts2 })
    };
    Ok(outAlg)
}

fn fixConstraint(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: &FCore::Graph,
    mut inConstrs: SCode::ConstraintSection,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<SCode::ConstraintSection> {
    let mut outConstrs: SCode::ConstraintSection;
    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let SCode::CONSTRAINTS { constraints: __pa0 } = inConstrs;
    exps = metamodelica::Own::own(__pa0);
    exps = fixList(
        inCache.clone(),
        inEnv,
        exps,
        tree,
        (std::sync::Arc::new(fixExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<metamodelica::Ref<Absyn::Exp>>
                    + 'static,
            >),
    )?;
    outConstrs = SCode::ConstraintSection { constraints: exps };
    Ok(outConstrs)
}

fn fixListAlgorithmItem(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut env: &FCore::Graph,
    mut alg: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Statement>>> {
    let mut outAlg: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    outAlg = fixList(
        cache.clone(),
        env,
        alg,
        tree,
        (std::sync::Arc::new(fixStatement)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        metamodelica::Ref<SCode::Statement>,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<metamodelica::Ref<SCode::Statement>>
                    + 'static,
            >),
    )?;
    Ok(outAlg)
}

fn fixStatement(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inStmt: metamodelica::Ref<SCode::Statement>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    let mut outStmt: metamodelica::Ref<SCode::Statement>;
    outStmt = 'mc: {
        let __mc_input = &*inStmt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: exp1, value: exp2, comment, info } => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    exp2_1 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(exp1.clone()),&*(&*exp1_1)) && referenceEq(&*(exp2.clone()),&*(&*exp2_1))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_ASSIGN { assignComponent: exp1_1.clone(), value: exp2_1.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_IF { boolExpr: exp1, trueBranch: truebranch1, elseIfBranch: elseifbranch1, elseBranch: elsebranch1, comment, info } => {
                    let mut exp2: metamodelica::Ref<Absyn::Exp>;
                    let mut elseifbranch2: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)>;
                    let mut truebranch2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    let mut elsebranch2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    truebranch2 = fixList(cache.clone(), &inEnv, truebranch1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    elseifbranch2 = fixListTuple2(cache.clone(), &inEnv, elseifbranch1.clone(), &tree, (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<SCode::Statement>>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixListAlgorithmItem(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::List<metamodelica::Ref<SCode::Statement>>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::List<metamodelica::Ref<SCode::Statement>>> + 'static>))?;
                    elsebranch2 = fixList(cache.clone(), &inEnv, elsebranch1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    Ok(if (referenceEq(&*(exp1.clone()),&*(&*exp2)) && metamodelica::ReferenceEq::reference_eq(&(truebranch1.clone()), &(truebranch2)) && metamodelica::ReferenceEq::reference_eq(&(elseifbranch1.clone()), &(elseifbranch2)) && metamodelica::ReferenceEq::reference_eq(&(elsebranch1.clone()), &(elsebranch2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_IF { boolExpr: exp2.clone(), trueBranch: truebranch2.clone(), elseIfBranch: elseifbranch2.clone(), elseBranch: elsebranch2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_FOR { index: iter, range: optExp1, forBody: body1, comment, info } => {
                    let mut optExp2: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut body2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    optExp2 = fixOption(cache.clone(), inEnv.clone(), optExp1.clone(), tree.clone(), &fixExp)?;
                    body2 = fixList(cache.clone(), &inEnv, body1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    Ok(if ((match (&(optExp1), &(optExp2)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&(body1.clone()), &(body2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_FOR { index: iter.clone(), range: optExp2.clone(), forBody: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_PARFOR { index: iter, range: optExp1, parforBody: body1, comment, info } => {
                    let mut optExp2: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut body2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    optExp2 = fixOption(cache.clone(), inEnv.clone(), optExp1.clone(), tree.clone(), &fixExp)?;
                    body2 = fixList(cache.clone(), &inEnv, body1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    Ok(if ((match (&(optExp1), &(optExp2)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&(body1.clone()), &(body2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_PARFOR { index: iter.clone(), range: optExp2.clone(), parforBody: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_WHILE { boolExpr: exp1, whileBody: body1, comment, info } => {
                    let mut exp2: metamodelica::Ref<Absyn::Exp>;
                    let mut body2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    body2 = fixList(cache.clone(), &inEnv, body1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    Ok(if (referenceEq(&*(exp1.clone()),&*(&*exp2)) && metamodelica::ReferenceEq::reference_eq(&(body1.clone()), &(body2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_WHILE { boolExpr: exp2.clone(), whileBody: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_WHEN_A { branches: whenlst, comment, info } => {
                    let mut whenlst = (*whenlst).clone();
                    whenlst = fixListTuple2(cache.clone(), &inEnv, whenlst.clone(), &tree, (std::sync::Arc::new(fixExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Array<FCore::Cache>, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<SCode::Statement>>, __a3: metamodelica::Ref<AvlSetString::Tree>| fixListAlgorithmItem(__a0, &__a1, __a2, &__a3)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::List<metamodelica::Ref<SCode::Statement>>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::List<metamodelica::Ref<SCode::Statement>>> + 'static>))?;
                    Ok(metamodelica::Ref::new(SCode::Statement::ALG_WHEN_A { branches: whenlst.clone(), comment: comment.clone(), info: info.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_ASSERT { condition: exp, message: exp1, level: exp2, comment, info } => {
                    let mut exp_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp_1 = fixExp(cache.clone(), inEnv.clone(), exp.clone(), tree.clone())?;
                    exp1_1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    exp2_1 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(exp.clone()),&*(&*exp_1)) && referenceEq(&*(exp1.clone()),&*(&*exp1_1)) && referenceEq(&*(exp2.clone()),&*(&*exp2_1))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_ASSERT { condition: exp_1.clone(), message: exp1_1.clone(), level: exp2_1.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_TERMINATE { message: exp1, comment, info } => {
                    let mut exp2: metamodelica::Ref<Absyn::Exp>;
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(exp1.clone()),&*(&*exp2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_TERMINATE { message: exp2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_REINIT { cref: exp1, newValue: exp2, comment, info } => {
                    let mut exp1_1: metamodelica::Ref<Absyn::Exp>;
                    let mut exp2_1: metamodelica::Ref<Absyn::Exp>;
                    exp1_1 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    exp2_1 = fixExp(cache.clone(), inEnv.clone(), exp2.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(exp1.clone()),&*(&*exp1_1)) && referenceEq(&*(exp2.clone()),&*(&*exp2_1))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_REINIT { cref: exp1_1.clone(), newValue: exp2_1.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_NORETCALL { exp: exp1, comment, info } => {
                    let mut exp2: metamodelica::Ref<Absyn::Exp>;
                    exp2 = fixExp(cache.clone(), inEnv.clone(), exp1.clone(), tree.clone())?;
                    Ok(if (referenceEq(&*(exp1.clone()),&*(&*exp2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_NORETCALL { exp: exp2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_RETURN { .. } => {
                    Ok(inStmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_BREAK { .. } => {
                    Ok(inStmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_FAILURE { stmts: body1, comment, info } => {
                    let mut body2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    body2 = fixList(cache.clone(), &inEnv, body1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    Ok(if (metamodelica::ReferenceEq::reference_eq(&(body1.clone()), &(body2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_FAILURE { stmts: body2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_TRY { body: truebranch1, elseBody: elsebranch1, comment, info } => {
                    let mut truebranch2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    let mut elsebranch2: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                    truebranch2 = fixList(cache.clone(), &inEnv, truebranch1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    elsebranch2 = fixList(cache.clone(), &inEnv, elsebranch1.clone(), &tree, (std::sync::Arc::new(fixStatement) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::Statement>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::Statement>> + 'static>))?;
                    Ok(if (metamodelica::ReferenceEq::reference_eq(&(truebranch1.clone()), &(truebranch2)) && metamodelica::ReferenceEq::reference_eq(&(elsebranch1.clone()), &(elsebranch2))) {inStmt.clone()} else {metamodelica::Ref::new(SCode::Statement::ALG_TRY { body: truebranch2.clone(), elseBody: elsebranch2.clone(), comment: comment.clone(), info: info.clone() })})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_CONTINUE { .. } => {
                    Ok(inStmt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixStatement")); __mm_s.push_str(&*literal!(" failed: ")); __mm_s.push_str(&*Dump::unparseAlgorithmStr(SCodeUtil::statementToAlgorithmItem(&inStmt)?)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/InstExtends.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStmt)
}

fn fixArrayDim(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: &FCore::Graph,
    mut ads: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut ads: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = ads;
    ads = fixList(
        inCache.clone(),
        inEnv,
        ads,
        tree,
        (std::sync::Arc::new(fixSubscript)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        metamodelica::Ref<Absyn::Subscript>,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<metamodelica::Ref<Absyn::Subscript>>
                    + 'static,
            >),
    )?;
    Ok(ads)
}

fn fixSubscript(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inSub: metamodelica::Ref<Absyn::Subscript>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut outSub: metamodelica::Ref<Absyn::Subscript>;
    outSub = (match &*inSub {
        Absyn::Subscript::NOSUB { .. } => inSub,
        Absyn::Subscript::SUBSCRIPT { subscript: exp1 } => {
            let mut exp2: metamodelica::Ref<Absyn::Exp>;
            exp2 = fixExp(cache.clone(), inEnv, exp1.clone(), tree)?;
            if (referenceEq(&*(exp1.clone()), &*(&*exp2))) {
                inSub
            } else {
                metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: exp2 })
            }
        }
    });
    Ok(outSub)
}

fn fixTypeSpec(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inTs: metamodelica::Ref<Absyn::TypeSpec>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut outTs: metamodelica::Ref<Absyn::TypeSpec>;
    outTs = (match &*inTs {
        Absyn::TypeSpec::TPATH {
            path: path1,
            arrayDim: arrayDim1,
        } => {
            let mut path2: metamodelica::Ref<Absyn::Path>;
            let mut arrayDim2: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>;
            arrayDim2 = fixOption(
                cache.clone(),
                inEnv.clone(),
                arrayDim1.clone(),
                tree.clone(),
                &move |__a0: metamodelica::Array<FCore::Cache>,
                       __a1: FCore::Graph,
                       __a2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
                       __a3: metamodelica::Ref<AvlSetString::Tree>| {
                    fixArrayDim(__a0, &__a1, __a2, &__a3)
                },
            )?;
            path2 = fixPath(cache.clone(), inEnv, path1.clone(), tree);
            if ((match (&(arrayDim2), &(arrayDim1)) {
                (None, None) => true,
                (Some(__refeq_l), Some(__refeq_r)) => {
                    metamodelica::ReferenceEq::reference_eq(&(*__refeq_l), &(*__refeq_r))
                }
                _ => false,
            }) && referenceEq(&*(path1.clone()), &*(&*path2)))
            {
                inTs
            } else {
                metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                    path: path2,
                    arrayDim: arrayDim2,
                })
            }
        }
        Absyn::TypeSpec::TCOMPLEX {
            path: path1,
            typeSpecs: typeSpecs1,
            arrayDim: arrayDim1,
        } => {
            let mut path2: metamodelica::Ref<Absyn::Path>;
            let mut arrayDim2: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>;
            let mut typeSpecs2: metamodelica::List<metamodelica::Ref<Absyn::TypeSpec>>;
            arrayDim2 = fixOption(
                cache.clone(),
                inEnv.clone(),
                arrayDim1.clone(),
                tree.clone(),
                &move |__a0: metamodelica::Array<FCore::Cache>,
                       __a1: FCore::Graph,
                       __a2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
                       __a3: metamodelica::Ref<AvlSetString::Tree>| {
                    fixArrayDim(__a0, &__a1, __a2, &__a3)
                },
            )?;
            path2 = fixPath(cache.clone(), inEnv.clone(), path1.clone(), tree.clone());
            typeSpecs2 = fixList(
                cache.clone(),
                &inEnv,
                typeSpecs1.clone(),
                &tree,
                (std::sync::Arc::new(fixTypeSpec)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Array<FCore::Cache>,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::TypeSpec>,
                                metamodelica::Ref<AvlSetString::Tree>,
                            ) -> Result<metamodelica::Ref<Absyn::TypeSpec>>
                            + 'static,
                    >),
            )?;
            if ((match (&(arrayDim2), &(arrayDim1)) {
                (None, None) => true,
                (Some(__refeq_l), Some(__refeq_r)) => {
                    metamodelica::ReferenceEq::reference_eq(&(*__refeq_l), &(*__refeq_r))
                }
                _ => false,
            }) && referenceEq(&*(path1.clone()), &*(&*path2))
                && metamodelica::ReferenceEq::reference_eq(&(typeSpecs1.clone()), &(typeSpecs2)))
            {
                inTs
            } else {
                metamodelica::Ref::new(Absyn::TypeSpec::TCOMPLEX {
                    path: path2,
                    typeSpecs: typeSpecs2,
                    arrayDim: arrayDim2,
                })
            }
        }
    });
    Ok(outTs)
}

fn fixPath(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> metamodelica::Ref<Absyn::Path> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = &*inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
                    Ok(inPath.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut id: ArcStr;
                    let mut path2: metamodelica::Ref<Absyn::Path>;
                    id = AbsynUtil::pathFirstIdent(&inPath);
                    let true = (AvlSetString::hasKey(tree.clone(), id.clone())?) else { return Err("pattern mismatch") };
                    path2 = FGraph::pathStripGraphScopePrefix(inPath.clone(), &inEnv, false);
                    Ok(path2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    Lookup::lookupClassLocal(inEnv.clone(), AbsynUtil::pathFirstIdent(&inPath))?;
                    path = FGraph::pathStripGraphScopePrefix(inPath.clone(), &inEnv, false);
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut cache: FCore::Cache;
                    (cache, path) = Inst::makeFullyQualified(metamodelica::arrayGet(inCache.clone(), 1)?, inEnv.clone(), inPath.clone())?;
                    path = FGraph::pathStripGraphScopePrefix(path.clone(), &inEnv, false);
                    metamodelica::arrayUpdate(inCache.clone(), 1, cache.clone())?;
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    path = FGraph::pathStripGraphScopePrefix(inPath.clone(), &inEnv, false);
                    Ok(path.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outPath
}

fn lookupVarNoErrorMessage(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ident: ArcStr,
) -> Result<(FCore::Graph, ArcStr)> {
    let mut outEnv: FCore::Graph;
    let mut id: ArcStr;
    match '__try0: {
        ErrorExt::setCheckpoint(literal!("InstExtends.lookupVarNoErrorMessage"));
        (_, _, _, _, _, _, outEnv, _, id) = unwrap_break_err!(Lookup::lookupVarIdent(inCache.clone(), inEnv.clone(), ident.clone(), metamodelica::nil()), '__try0);
        ErrorExt::rollBack(literal!("InstExtends.lookupVarNoErrorMessage"));
        Ok::<_, &'static str>((id.clone(), outEnv.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            id = __try0_o0;
            outEnv = __try0_o1;
        }
        Err(__try0_err) => {
            ErrorExt::rollBack(literal!("InstExtends.lookupVarNoErrorMessage"));
            return Err(__try0_err);
        }
    }
    Ok((outEnv, id))
}

fn fixCref(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (inEnv.clone(), inCref.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. }) => {
                    let mut env = (*env).clone();
                    env = FGraph::topScope(&inEnv)?;
                    Ok(fixCref(cache.clone(), env.clone(), var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), tree))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, cref) => {
                    let mut id: ArcStr;
                    let mut cref = (*cref).clone();
                    id = AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&cref))?;
                    let true = (AvlSetString::hasKey(tree.clone(), id.clone())?) else { return Err("pattern mismatch") };
                    cref = FGraph::crefStripGraphScopePrefix(cref.clone(), metamodelica::AsArg::as_arg(&env), false);
                    cref = if (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cref), &inCref)?) {inCref.clone()} else {cref.clone()};
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, cref) => {
                    let mut id: ArcStr;
                    let mut denv: FCore::Graph;
                    let mut cref = (*cref).clone();
                    id = AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&cref))?;
                    (denv, id) = lookupVarNoErrorMessage(metamodelica::arrayGet(cache.clone(), 1)?, env.clone(), id.clone())?;
                    denv = FGraph::openScope(denv.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, id.clone(), None)?;
                    cref = AbsynUtil::crefReplaceFirstIdent(metamodelica::AsArg::as_arg(&cref), &(FGraph::getGraphName(&denv)?))?;
                    cref = FGraph::crefStripGraphScopePrefix(cref.clone(), metamodelica::AsArg::as_arg(&env), false);
                    cref = if (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cref), &inCref)?) {inCref.clone()} else {cref.clone()};
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, cref) => {
                    let mut id: ArcStr;
                    let mut denv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut cref = (*cref).clone();
                    id = AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&cref))?;
                    (_, c, denv) = Lookup::lookupClassIdent(metamodelica::arrayGet(cache.clone(), 1)?, env.clone(), &id, None)?;
                    id = SCodeUtil::getElementName(&c)?;
                    denv = FGraph::openScope(denv.clone(), openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED, id.clone(), None)?;
                    cref = AbsynUtil::crefReplaceFirstIdent(metamodelica::AsArg::as_arg(&cref), &(FGraph::getGraphName(&denv)?))?;
                    cref = FGraph::crefStripGraphScopePrefix(cref.clone(), metamodelica::AsArg::as_arg(&env), false);
                    cref = if (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cref), &inCref)?) {inCref.clone()} else {cref.clone()};
                    Ok(cref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCref.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCref
}

fn fixModifications(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod> = inMod.clone();
    outMod = 'mc: {
        let __mc_input = outMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::NOMOD { .. } => {
                    Ok(inMod.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::MOD { .. } => {
                    let mut subModLst: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    let mut exp: Option<metamodelica::Ref<Absyn::Exp>>;
                    let mut outMod: metamodelica::Ref<SCode::Mod> = outMod.clone();
                    subModLst = fixList(inCache.clone(), &inEnv, var_field!((*outMod).subModLst, SCode::Mod::MOD).clone(), &tree, (std::sync::Arc::new(fixSubMod) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<FCore::Cache>, FCore::Graph, metamodelica::Ref<SCode::SubMod>, metamodelica::Ref<AvlSetString::Tree>) -> Result<metamodelica::Ref<SCode::SubMod>> + 'static>))?;
                    if !(metamodelica::ReferenceEq::reference_eq(&(var_field!((*outMod).subModLst, SCode::Mod::MOD).clone()), &(subModLst))) {
                        assign_variant_field!(outMod => SCode::Mod::MOD; subModLst = subModLst.clone());
                    }
                    exp = fixOption(inCache.clone(), inEnv.clone(), var_field!((*outMod).binding, SCode::Mod::MOD).clone(), tree.clone(), &fixExp)?;
                    if !((match (&(exp), &(var_field!((*outMod).binding, SCode::Mod::MOD))) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {
                        assign_variant_field!(outMod => SCode::Mod::MOD; binding = exp.clone());
                    }
                    Ok((outMod.clone(), outMod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outMod = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { element: Deref @ SCode::Element::COMPONENT { .. }, .. } => {
                    let mut e: metamodelica::Ref<SCode::Element>;
                    let mut outMod: metamodelica::Ref<SCode::Mod> = outMod.clone();
                    e = fixElement(inCache.clone(), inEnv.clone(), var_field!((*outMod).element, SCode::Mod::REDECL).clone(), tree.clone())?;
                    if !(referenceEq(&*(&*e),&*(var_field!((*outMod).element, SCode::Mod::REDECL).clone()))) {
                        assign_variant_field!(outMod => SCode::Mod::REDECL; element = e.clone());
                    }
                    Ok((outMod.clone(), outMod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outMod = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Mod::REDECL { element: e @ Deref @ SCode::Element::CLASS { classDef: cdef, .. }, .. } => {
                    let mut e = (*e).clone();
                    let mut cdef = (*cdef).clone();
                    let mut outMod: metamodelica::Ref<SCode::Mod> = outMod.clone();
                    cdef = fixClassdef(inCache.clone(), inEnv.clone(), cdef.clone(), tree.clone())?;
                    if !(referenceEq(&*(cdef.clone()),&*(var_field!((*e).classDef, SCode::Element::CLASS).clone()))) {
                        assign_variant_field!(e => SCode::Element::CLASS; classDef = cdef.clone());
                        assign_variant_field!(outMod => SCode::Mod::REDECL; element = e.clone());
                    }
                    Ok((outMod.clone(), outMod.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outMod = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstExtends.fixModifications failed: ")); __mm_s.push_str(&*SCodeDump::printModStr(inMod.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outMod)
}

fn fixSubMod(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut subMod: metamodelica::Ref<SCode::SubMod>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut subMod: metamodelica::Ref<SCode::SubMod> = subMod;
    let mut ident: ArcStr;
    let mut mod1: metamodelica::Ref<SCode::Mod>;
    let mut mod2: metamodelica::Ref<SCode::Mod>;
    let __arc2 = subMod.clone();
    let SCode::NAMEMOD {
        ident: __pa0,
        r#mod: __pa1,
    } = &*__arc2;
    ident = metamodelica::Own::own(__pa0);
    mod1 = metamodelica::Own::own(__pa1);
    mod2 = fixModifications(inCache.clone(), inEnv, mod1.clone(), tree)?;
    if !(referenceEq(&*(mod1), &*(&*mod2))) {
        subMod = metamodelica::Ref::new(SCode::SubMod {
            ident: ident,
            r#mod: mod2,
        });
    }
    Ok(subMod)
}

fn fixExp(
    mut cache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    (outExp, _) = AbsynUtil::traverseExp(
        inExp,
        (std::sync::Arc::new(fnptr!(
            fixExpTraverse,
            metamodelica::Ref<Absyn::Exp>,
            (
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                metamodelica::Ref<AvlSetString::Tree>
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (
                            metamodelica::Array<FCore::Cache>,
                            FCore::Graph,
                            metamodelica::Ref<AvlSetString::Tree>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (
                            metamodelica::Array<FCore::Cache>,
                            FCore::Graph,
                            metamodelica::Ref<AvlSetString::Tree>,
                        ),
                    )> + 'static,
            >),
        (cache.clone(), inEnv, tree),
    )?;
    Ok(outExp)
}

fn fixExpTraverse(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut tpl: (
        metamodelica::Array<FCore::Cache>,
        FCore::Graph,
        metamodelica::Ref<AvlSetString::Tree>,
    ),
) -> (
    metamodelica::Ref<Absyn::Exp>,
    (
        metamodelica::Array<FCore::Cache>,
        FCore::Graph,
        metamodelica::Ref<AvlSetString::Tree>,
    ),
) {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut tpl: (
        metamodelica::Array<FCore::Cache>,
        FCore::Graph,
        metamodelica::Ref<AvlSetString::Tree>,
    ) = tpl;
    exp = (::match_deref::match_deref! { match &((exp.clone(), tpl.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cref }, (cache, env, tree)) => {
            let mut cref1: metamodelica::Ref<Absyn::ComponentRef>;
            cref1 = fixCref(cache.clone(), env.clone(), cref.clone(), metamodelica::AsArg::as_arg(&tree));
            if (referenceEq(&*(cref.clone()),&*(&*cref1))) {exp} else {metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cref1 })}
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, .. }, (cache, env, tree)) => {
            let mut cref1: metamodelica::Ref<Absyn::ComponentRef>;
            cref1 = fixCref(cache.clone(), env.clone(), cref.clone(), metamodelica::AsArg::as_arg(&tree));
            if (referenceEq(&*(cref.clone()),&*(&*cref1))) {exp} else {metamodelica::Ref::new(Absyn::Exp::CALL { function_: cref1, functionArgs: var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), typeVars: var_field!((*exp).typeVars, Absyn::Exp::CALL).clone() })}
        },
        (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, functionArgs: fargs }, (cache, env, tree)) => {
            let mut cref1: metamodelica::Ref<Absyn::ComponentRef>;
            cref1 = fixCref(cache.clone(), env.clone(), cref.clone(), metamodelica::AsArg::as_arg(&tree));
            if (referenceEq(&*(cref.clone()),&*(&*cref1))) {exp} else {metamodelica::Ref::new(Absyn::Exp::PARTEVALFUNCTION { function_: cref1, functionArgs: fargs.clone() })}
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, tpl)
}

fn fixOption<Type_A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut inA: Option<Type_A>,
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
    mut fixA: &dyn ::std::ops::Fn(
        metamodelica::Array<FCore::Cache>,
        FCore::Graph,
        Type_A,
        metamodelica::Ref<AvlSetString::Tree>,
    ) -> Result<Type_A>,
) -> Result<Option<Type_A>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >;

    let mut outA: Option<Type_A>;
    outA = (match inA.clone() {
        None => inA,
        Some(mut A1) => {
            let mut A2: Type_A;
            A2 = fixA(inCache.clone(), inEnv, A1.clone(), tree)?;
            if (metamodelica::ReferenceEq::reference_eq(&(A1), &(A2.clone()))) {
                inA
            } else {
                Some(A2)
            }
        }
    });
    Ok(outA)
}

fn fixList<Type_A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: &FCore::Graph,
    mut inA: metamodelica::List<Type_A>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
    mut fixA: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >,
) -> Result<metamodelica::List<Type_A>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >;

    let mut outA: metamodelica::List<Type_A>;
    if (inA).is_empty() {
        outA = inA;
        return Ok(outA);
    }
    outA = List::mapCheckReferenceEq(
        inA,
        &({
            let __pe_b0 = inCache.clone();
            let __pe_b1 = inEnv.clone();
            let __pe_b3 = tree.clone();
            move |__pe_a2| fixA(__pe_b0.clone(), __pe_b1.clone(), __pe_a2, __pe_b3.clone())
        }),
    )?;
    Ok(outA)
}

fn fixListList<Type_A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: &FCore::Graph,
    mut inA: metamodelica::List<metamodelica::List<Type_A>>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
    mut fixA: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >,
) -> Result<metamodelica::List<metamodelica::List<Type_A>>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >;

    let mut outA: metamodelica::List<metamodelica::List<Type_A>> = metamodelica::nil();
    if (inA).is_empty() {
        outA = metamodelica::nil();
        return Ok(outA);
    }
    outA = List::mapCheckReferenceEq(
        inA,
        &({
            let __pe_b0 = inCache.clone();
            let __pe_b1 = inEnv.clone();
            let __pe_b3 = tree.clone();
            let __pe_b4: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        _,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<_>
                    + 'static,
            > = fixA.clone();
            move |__pe_a2| fixList(__pe_b0.clone(), &__pe_b1, __pe_a2, &__pe_b3, __pe_b4.clone())
        }),
    )?;
    Ok(outA)
}

fn fixListTuple2<
    Type_A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
    Type_B: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
>(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: &FCore::Graph,
    mut inRest: metamodelica::List<(Type_A, Type_B)>,
    mut tree: &metamodelica::Ref<AvlSetString::Tree>,
    mut fixA: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >,
    mut fixB: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_B,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_B>
            + 'static,
    >,
) -> Result<metamodelica::List<(Type_A, Type_B)>> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >;

    pub type FixBFn<Type_B: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_B,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_B>
            + 'static,
    >;

    let mut outA: metamodelica::List<(Type_A, Type_B)>;
    outA = fixList(
        inCache.clone(),
        inEnv,
        inRest,
        tree,
        (std::sync::Arc::new({
            let __pe_b4: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        _,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<_>
                    + 'static,
            > = fixA.clone();
            let __pe_b5: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        _,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<_>
                    + 'static,
            > = fixB.clone();
            move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| {
                fixTuple2(__pe_a0, __pe_a1, __pe_a2, __pe_a3, &*__pe_b4, &*__pe_b5)
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Array<FCore::Cache>,
                        FCore::Graph,
                        _,
                        metamodelica::Ref<AvlSetString::Tree>,
                    ) -> Result<_>
                    + 'static,
            >),
    )?;
    Ok(outA)
}

fn fixTuple2<
    Type_A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
    Type_B: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
>(
    mut inCache: metamodelica::Array<FCore::Cache>,
    mut inEnv: FCore::Graph,
    mut tpl: (Type_A, Type_B),
    mut tree: metamodelica::Ref<AvlSetString::Tree>,
    mut fixA: &dyn ::std::ops::Fn(
        metamodelica::Array<FCore::Cache>,
        FCore::Graph,
        Type_A,
        metamodelica::Ref<AvlSetString::Tree>,
    ) -> Result<Type_A>,
    mut fixB: &dyn ::std::ops::Fn(
        metamodelica::Array<FCore::Cache>,
        FCore::Graph,
        Type_B,
        metamodelica::Ref<AvlSetString::Tree>,
    ) -> Result<Type_B>,
) -> Result<(Type_A, Type_B)> {
    pub type FixAFn<Type_A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_A,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_A>
            + 'static,
    >;

    pub type FixBFn<Type_B: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Array<FCore::Cache>,
                FCore::Graph,
                Type_B,
                metamodelica::Ref<AvlSetString::Tree>,
            ) -> Result<Type_B>
            + 'static,
    >;

    let mut tpl: (Type_A, Type_B) = tpl;
    let mut a1: Type_A;
    let mut a2: Type_A;
    let mut b1: Type_B;
    let mut b2: Type_B;
    (a1, b1) = tpl.clone();
    a2 = fixA(inCache.clone(), inEnv.clone(), a1.clone(), tree.clone())?;
    b2 = fixB(inCache.clone(), inEnv, b1.clone(), tree)?;
    if !(metamodelica::ReferenceEq::reference_eq(&(a1), &(a2.clone()))
        && metamodelica::ReferenceEq::reference_eq(&(b1), &(b2.clone())))
    {
        tpl = (a2, b2);
    }
    Ok(tpl)
}
