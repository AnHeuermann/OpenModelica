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

use crate::ConnectionGraph;
use crate::FGraph;
use crate::FNode;
use crate::InnerOuter;
use crate::Inst;
use crate::InstUtil;
use crate::Lookup;
use crate::Mod;
use crate::PrefixUtil;
use crate::UnitAbsyn;
use crate::UnitAbsynBuilder;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::InstBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

pub type InstDims = metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;

pub(crate) fn instantiateExternalObject(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut els: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut r#impl: bool,
    mut comment: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    ClassInf::State,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut dae: DAE::DAElist;
    let mut ciState: ClassInf::State;
    (outCache, outEnv, outIH, dae, ciState) = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), inIH, r#impl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, false) => {
                    let mut destr: metamodelica::Ref<SCode::Element>;
                    let mut constr: metamodelica::Ref<SCode::Element>;
                    let mut className: Ident;
                    let mut classNameFQ: metamodelica::Ref<Absyn::Path>;
                    let mut functp: metamodelica::Ref<DAE::Type>;
                    let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    className = FNode::refName(FGraph::lastScopeRef(metamodelica::AsArg::as_arg(&env))?);
                    checkExternalObjectMod(&inMod, className.clone())?;
                    destr = SCodeUtil::getExternalObjectDestructor(els)?;
                    constr = SCodeUtil::getExternalObjectConstructor(els)?;
                    env = FGraph::mkClassNode(env.clone(), destr.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inMod.clone(), false)?;
                    env = FGraph::mkClassNode(env.clone(), constr.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inMod.clone(), false)?;
                    (cache, ih) = instantiateExternalObjectDestructor(cache.clone(), env.clone(), ih.clone(), destr.clone())?;
                    (cache, ih, functp) = instantiateExternalObjectConstructor(cache.clone(), env.clone(), ih.clone(), constr.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    classNameFQ = metamodelica::Own::own(__pa0);
                    (env, r) = FGraph::stripLastScopeRef(env.clone())?;
                    env = FGraph::mkTypeNode(env.clone(), className.clone(), functp.clone())?;
                    env = FGraph::pushScopeRef(env.clone(), r.clone())?;
                    source = ElementSource::addElementSourcePartOfOpt(DAE::emptyElementSource().clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?)?;
                    source = ElementSource::addCommentToSource(source.clone(), Some(comment.clone()));
                    source = ElementSource::addElementSourceFileInfo(source.clone(), info.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::EXTOBJECTCLASS { path: classNameFQ.clone(), source: source.clone() })] }, ClassInf::State::EXTERNAL_OBJ { path: classNameFQ.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ih, true) => {
                    let mut classNameFQ: metamodelica::Ref<Absyn::Path>;
                    let __pa0 = ::match_deref::match_deref! { match &(FGraph::getScopePath(&inEnv)?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    classNameFQ = metamodelica::Own::own(__pa0);
                    Ok((cache.clone(), inEnv.clone(), ih.clone(), DAE::emptyDae().clone(), ClassInf::State::EXTERNAL_OBJ { path: classNameFQ.clone() }))
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
                    Debug::trace(literal!("- InstFunction.instantiateExternalObject failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, dae, ciState))
}

fn checkExternalObjectMod(mut inMod: &metamodelica::Ref<DAE::Mod>, mut inClassName: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match inMod {
        Deref @ DAE::Mod::NOMOD { .. } => {
            ()
        },
        Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            ()
        },
        Deref @ DAE::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SubMod { ident: id, r#mod }, tail: _ }, .. } => {
            let mut info: SourceInfo;
            info = Mod::getModInfo(metamodelica::AsArg::as_arg(&r#mod));
            Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![id.clone(), inClassName], &info)?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn instantiateExternalObjectDestructor(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut cl: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, metamodelica::List<InnerOuter::TopInstance>)> {
    let mut outCache: FCore::Cache;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    (outCache, outIH) = 'mc: {
        let __mc_input = (inCache, inIH);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih) => {
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, _, ih) = implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), ih.clone()))
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
                    Debug::trace(literal!("- InstFunction.instantiateExternalObjectDestructor failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outIH))
}

fn instantiateExternalObjectConstructor(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut cl: metamodelica::Ref<SCode::Element>,
) -> Result<(
    FCore::Cache,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outCache: FCore::Cache;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outCache, outIH, outType) = 'mc: {
        let __mc_input = (inCache, inIH);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, ih) => {
                    let mut env1: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env1, ih) = implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    (cache, ty, _) = Lookup::lookupType(cache.clone(), env1.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("constructor") }), None)?;
                    Ok((cache.clone(), ih.clone(), ty.clone()))
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
                    Debug::trace(literal!("- InstFunction.instantiateExternalObjectConstructor failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outIH, outType))
}

pub fn implicitFunctionInstantiation(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    (outCache, outEnv, outIH) = (::match_deref::match_deref! { match &(inClass) {
        c @ Deref @ SCode::Element::CLASS { name: n, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, partialPrefix: pPrefix, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut r#mod = inMod;
            let mut pre = inPrefix;
            let mut inst_dims = inInstDims;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut cenv: FCore::Graph;
            let mut fpath: metamodelica::Ref<Absyn::Path>;
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut fun: DAE::Function;
            let mut c = (*c).clone();
            (cache, c, cenv) = Lookup::lookupRecordConstructorClass(cache, env, metamodelica::Ref::new(Absyn::Path::IDENT { name: n.clone() }))?;
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(implicitFunctionInstantiation2(cache, cenv, ih, r#mod, pre, c.clone(), inst_dims, true)?) {
                (__pa0, __pa1, __pa2, Deref @ metamodelica::ListNode::Cons { head: DAE::Function::FUNCTION { path: __pa3, type_: __pa4, source: __pa5, .. }, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            env = metamodelica::Own::own(__pa1);
            ih = metamodelica::Own::own(__pa2);
            fpath = metamodelica::Own::own(__pa3);
            ty1 = metamodelica::Own::own(__pa4);
            source = metamodelica::Own::own(__pa5);
            fun = DAE::Function::RECORD_CONSTRUCTOR { path: fpath, type_: ty1, source: source };
            cache = InstUtil::addFunctionsToDAE(cache, &(list![fun]), pPrefix.clone())?;
            (cache, env, ih)
        },
        c @ Deref @ SCode::Element::CLASS { restriction: r, partialPrefix: pPrefix, .. } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut r#mod = inMod;
            let mut pre = inPrefix;
            let mut inst_dims = inInstDims;
            let mut funs: metamodelica::List<DAE::Function>;
            if '__try0: {
                let SCode::R_RECORD { isOperator: _ } = (r.clone()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            (cache, env, ih, funs) = implicitFunctionInstantiation2(cache, env, ih, r#mod, pre, c.clone(), inst_dims, false)?;
            cache = InstUtil::addFunctionsToDAE(cache, &funs, pPrefix.clone())?;
            (cache, env, ih)
        },
        Deref @ SCode::Element::CLASS { name: n, .. } => {
            let mut env = inEnv;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.implicitFunctionInstantiation failed ")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) })?;
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(&env)); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH))
}

fn implicitFunctionInstantiation2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inMod: metamodelica::Ref<DAE::Mod>,
    mut inPrefix: DAE::Prefix,
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inInstDims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
    mut instFunctionTypeOnly: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<DAE::Function>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut funcs: metamodelica::List<DAE::Function>;
    (outCache, outEnv, outIH, funcs) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inMod, inPrefix, inClass.clone(), inInstDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, ih, r#mod, pre, Deref @ SCode::Element::CLASS { classDef: cd, prefixes: Deref @ SCode::Prefixes { visibility, .. }, partialPrefix, name: n, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: funcRest }, info, .. }, inst_dims) => {
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut ty1: metamodelica::Ref<DAE::Type>;
                            let mut env_1: FCore::Graph;
                            let mut cenv: FCore::Graph;
                            let mut fpath: metamodelica::Ref<Absyn::Path>;
                            let mut c: metamodelica::Ref<SCode::Element>;
                            let mut source: metamodelica::Ref<DAE::ElementSource>;
                            let mut daeElts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut derFuncs: metamodelica::List<DAE::FunctionDefinition>;
                            let mut inlineType: DAE::InlineType;
                            let mut partialPrefixBool: bool;
                            let mut isImpure: bool;
                            let mut cmt: metamodelica::Ref<SCode::Comment>;
                            let mut cs: InstTypes::CallingScope;
                            let mut cache = (*cache).clone();
                            let mut ih = (*ih).clone();
                            let false = (SCodeUtil::isExternalFunctionRestriction(funcRest.clone())) else { return Err("pattern mismatch") };
                            isImpure = SCodeUtil::isImpureFunctionRestriction(funcRest.clone());
                            c = if (Config::acceptMetaModelicaGrammar()?) {inClass.clone()} else {SCodeUtil::setClassPartialPrefix(openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, inClass.clone())?};
                            cs = if (instFunctionTypeOnly) {openmodelica_frontend_inst::InstTypes::CallingScope::TYPE_CALL} else {openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL};
                            let (__pa0, __pa1, __pa2, _, DAE::DAE { elementLst: __pa3 }, _, __pa4, _, _, _) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), r#mod.clone(), pre.clone(), c.clone(), inst_dims.clone(), true, cs, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                            cache = metamodelica::Own::own(__pa0);
                            cenv = metamodelica::Own::own(__pa1);
                            ih = metamodelica::Own::own(__pa2);
                            daeElts = metamodelica::Own::own(__pa3);
                            ty = metamodelica::Own::own(__pa4);
                            List::map2_0(&daeElts, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: bool, __a2: SourceInfo| InstUtil::checkFunctionElement(__a0, __a1, &__a2), false, info.clone())?;
                            env_1 = env.clone();
                            (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env_1.clone(), n.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                            cmt = InstUtil::extractComment(&daeElts)?;
                            derFuncs = InstUtil::getDeriveAnnotation(metamodelica::AsArg::as_arg(&cd), &cmt, &fpath, metamodelica::AsArg::as_arg(&cache), &cenv, metamodelica::AsArg::as_arg(&ih), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info));
                            cache = instantiateDerivativeFuncs(cache.clone(), env.clone(), ih.clone(), &derFuncs, &fpath, metamodelica::AsArg::as_arg(&info))?;
                            ty1 = InstUtil::setFullyQualifiedTypename(ty.clone(), fpath.clone());
                            checkExtObjOutput(&ty1, info.clone())?;
                            if InstUtil::functionAlwaysFails(&daeElts)? {
                                ty1 = Types::setFunctionNoReturn(ty1.clone());
                            }
                            env_1 = FGraph::mkTypeNode(env_1.clone(), n.clone(), ty1.clone())?;
                            source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                            inlineType = InstBasics::commentIsInlineFunc(&cmt);
                            partialPrefixBool = SCodeUtil::partialBool(partialPrefix.clone());
                            daeElts = InstUtil::optimizeFunctionCheckForLocals(&fpath, daeElts.clone(), None, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
                            InstUtil::checkFunctionDefUse(daeElts.clone(), metamodelica::AsArg::as_arg(&info))?;
                            if false && Config::acceptMetaModelicaGrammar()? && !(instFunctionTypeOnly) {
                                InstUtil::checkFunctionInputUsed(&daeElts, None, AbsynUtil::pathString(fpath.clone(), literal!("."), true, false)?)?;
                            }
                            Ok((cache.clone(), env_1.clone(), ih.clone(), list![DAE::Function::FUNCTION { path: fpath.clone(), functions: metamodelica::cons(DAE::FunctionDefinition::FUNCTION_DEF { body: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
                for mut e in (daeElts.clone()).into_iter().cloned() {
                    if !(!(DAEUtil::isComment(&(e.clone())))) { continue; }
                    let __x = e.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }) }, derFuncs.clone()), type_: ty1.clone(), visibility: visibility.clone(), partialPrefix: partialPrefixBool, isImpure: isImpure, inlineType: inlineType, unusedInputs: metamodelica::nil(), source: source.clone(), comment: Some(cmt.clone()) }]))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, r#mod, pre, c @ Deref @ SCode::Element::CLASS { partialPrefix, prefixes: Deref @ SCode::Prefixes { visibility, .. }, name: n, restriction: restr @ SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity } }, classDef: cd @ parts @ Deref @ SCode::ClassDef::PARTS { externalDecl: Some(scExtdecl), .. }, info, encapsulatedPrefix, .. }, inst_dims) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut env_1: FCore::Graph;
                    let mut tempenv: FCore::Graph;
                    let mut cenv: FCore::Graph;
                    let mut fpath: metamodelica::Ref<Absyn::Path>;
                    let mut vis: SCode::Visibility;
                    let mut extdecl: DAE::ExternalDecl;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut daeElts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut derFuncs: metamodelica::List<DAE::FunctionDefinition>;
                    let mut partialPrefixBool: bool;
                    let mut isImpure: bool;
                    let mut cmt: metamodelica::Ref<SCode::Comment>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let (__pa0, __pa1, __pa2, _, DAE::DAE { elementLst: __pa3 }, _, __pa4, _, _, _) = Inst::instClass(cache.clone(), env.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), r#mod.clone(), pre.clone(), c.clone(), inst_dims.clone(), true, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    cache = metamodelica::Own::own(__pa0);
                    cenv = metamodelica::Own::own(__pa1);
                    ih = metamodelica::Own::own(__pa2);
                    daeElts = metamodelica::Own::own(__pa3);
                    ty = metamodelica::Own::own(__pa4);
                    List::map2_0(&daeElts, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: bool, __a2: SourceInfo| InstUtil::checkFunctionElement(__a0, __a1, &__a2), true, info.clone())?;
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), n.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    cmt = InstUtil::extractComment(&daeElts)?;
                    derFuncs = InstUtil::getDeriveAnnotation(metamodelica::AsArg::as_arg(&cd), &cmt, &fpath, metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&info));
                    cache = instantiateDerivativeFuncs(cache.clone(), env.clone(), ih.clone(), &derFuncs, &fpath, metamodelica::AsArg::as_arg(&info))?;
                    ty1 = InstUtil::setFullyQualifiedTypename(ty.clone(), fpath.clone());
                    checkExtObjOutput(&ty1, info.clone())?;
                    env_1 = FGraph::mkTypeNode(cenv.clone(), n.clone(), ty1.clone())?;
                    vis = openmodelica_frontend_types::SCode::Visibility::PUBLIC;
                    isImpure = AbsynUtil::isImpure(purity.clone(), false);
                    (cache, tempenv, ih, _, _, _, _, _, _, _, _, _) = Inst::instClassdef(cache.clone(), env_1.clone(), ih.clone(), UnitAbsyn::noStore().clone(), r#mod.clone(), pre.clone(), ClassInf::State::FUNCTION { path: fpath.clone(), isImpure: isImpure }, metamodelica::AsArg::as_arg(&n), metamodelica::AsArg::as_arg(&parts), restr.clone(), vis, partialPrefix.clone(), encapsulatedPrefix.clone(), inst_dims.clone(), true, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), Connect::emptySet().clone(), None, &cmt, metamodelica::AsArg::as_arg(&info))?;
                    (cache, ih, extdecl) = instExtDecl(cache.clone(), tempenv.clone(), ih.clone(), n.clone(), scExtdecl.clone(), &daeElts, ty1.clone(), true, pre.clone(), info.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    partialPrefixBool = SCodeUtil::partialBool(partialPrefix.clone());
                    InstUtil::checkExternalFunction(&daeElts, extdecl.clone(), AbsynUtil::pathString(fpath.clone(), literal!("."), true, false)?)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone(), list![DAE::Function::FUNCTION { path: fpath.clone(), functions: metamodelica::cons(DAE::FunctionDefinition::FUNCTION_EXT { body: daeElts.clone(), externalDecl: extdecl.clone() }, derFuncs.clone()), type_: ty1.clone(), visibility: visibility.clone(), partialPrefix: partialPrefixBool, isImpure: isImpure, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, unusedInputs: metamodelica::nil(), source: source.clone(), comment: Some(cmt.clone()) }]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, pre, Deref @ SCode::Element::CLASS { name: n, prefixes: Deref @ SCode::Prefixes { visibility, .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity } }, classDef: Deref @ SCode::ClassDef::OVERLOAD { pathLst: funcnames }, cmt, .. }, _) => {
                    let mut fpath: metamodelica::Ref<Absyn::Path>;
                    let mut resfns: metamodelica::List<DAE::Function>;
                    let mut isImpure: bool;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    (cache, env, ih, resfns) = instOverloadedFunctions(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&pre), metamodelica::AsArg::as_arg(&funcnames), var_field!((*inClass).info, SCode::Element::CLASS))?;
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env.clone(), n.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    isImpure = AbsynUtil::isImpure(purity.clone(), false);
                    resfns = metamodelica::cons(DAE::Function::FUNCTION { path: fpath.clone(), functions: list![DAE::FunctionDefinition::FUNCTION_DEF { body: metamodelica::nil() }], type_: DAE::T_UNKNOWN_DEFAULT().clone(), visibility: visibility.clone(), partialPrefix: true, isImpure: isImpure, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, unusedInputs: metamodelica::nil(), source: DAE::emptyElementSource().clone(), comment: Some(cmt.clone()) }, resfns.clone());
                    Ok((cache.clone(), env.clone(), ih.clone(), resfns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, Deref @ SCode::Element::CLASS { name: n, .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.implicitFunctionInstantiation2 failed ")); __mm_s.push_str(&*n); ArcStr::from(__mm_s) })?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  Scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, funcs))
}

fn instantiateDerivativeFuncs(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut ih: metamodelica::List<InnerOuter::TopInstance>,
    mut funcs: &metamodelica::List<DAE::FunctionDefinition>,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache;
    outCache = instantiateDerivativeFuncs2(cache, env, ih, &(DAEUtil::getDerivativePaths(funcs)), path, info)?;
    Ok(outCache)
}

fn instantiateDerivativeFuncs2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPaths: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache;
    outCache = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), inIH, &**inPaths);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(cache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: p, tail: paths }) => {
                            let mut funcs: metamodelica::List<DAE::Function> = metamodelica::nil();
                            let mut cenv: FCore::Graph;
                            let mut cdef: metamodelica::Ref<SCode::Element>;
                            let mut cache = (*cache).clone();
                            let mut ih = (*ih).clone();
                            let mut p = (*p).clone();
                            (cache, cdef, cenv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&p), Some(info.clone()))?;
                            (cache, p) = Inst::makeFullyQualified(cache.clone(), cenv.clone(), p.clone())?;
                            let () = 'mc: {
                let __mc_input = ();
                if let Ok(__v) = (|| -> Result<_> {
                            let () = __mc_input.clone() else { return Err("nomatch") };
                            FCore::checkCachedInstFuncGuard(metamodelica::AsArg::as_arg(&cache), p.clone())?;
                            Ok(())
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            let _ = __mc_input.clone() else { return Err("nomatch") };
                            let mut cache: FCore::Cache = cache.clone();
                            let mut funcs: metamodelica::List<DAE::Function>;
                            let mut ih: metamodelica::List<InnerOuter::TopInstance> = ih.clone();
                            cache = FCore::addCachedInstFuncGuard(cache.clone(), p.clone())?;
                            (cache, _, ih, funcs) = implicitFunctionInstantiation2(cache.clone(), cenv.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cdef.clone(), metamodelica::nil(), false)?;
                            funcs = InstUtil::addNameToDerivativeMapping(funcs.clone(), path.clone());
                            cache = FCore::addDaeFunction(cache.clone(), &funcs)?;
                            Ok(())
                })() { break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            Ok(instantiateDerivativeFuncs2(cache.clone(), env.clone(), ih.clone(), metamodelica::AsArg::as_arg(&paths), path, info)?)
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    let mut fun: ArcStr;
                    let mut scope: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &((*inPaths)) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    p = metamodelica::Own::own(__pa0);
                    fun = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    scope = FGraph::printGraphPathStr(&inEnv);
                    Error::addSourceMessage(&(Error::LOOKUP_FUNCTION_ERROR.clone()), list![fun.clone(), scope.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCache)
}

pub(crate) fn implicitFunctionTypeInstantiation(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inClass: metamodelica::Ref<SCode::Element>,
) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    (outCache, outEnv, outIH) = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), inIH, &*inClass);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ } }, classDef: Deref @ SCode::ClassDef::PARTS { .. }, .. }) => {
                    let mut env_1: FCore::Graph;
                    let mut funs: metamodelica::List<DAE::Function>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    (cache, env_1, ih, funs) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inClass.clone(), metamodelica::nil(), true)?;
                    cache = FCore::addDaeExtFunction(cache.clone(), &funs)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { name: id, prefixes, encapsulatedPrefix: e, partialPrefix: p, restriction: r, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, externalDecl: extDecl, .. }, cmt, info }) => {
                    let mut stripped_class: metamodelica::Ref<SCode::Element>;
                    let mut env_1: FCore::Graph;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut elts = (*elts).clone();
                    elts = List::select(elts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isElementImportantForFunction(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>))?;
                    stripped_class = metamodelica::Ref::new(SCode::Element::CLASS { name: id.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: e.clone(), partialPrefix: p.clone(), restriction: r.clone(), classDef: metamodelica::Ref::new(SCode::ClassDef::PARTS { elementLst: elts.clone(), normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: extDecl.clone() }), cmt: cmt.clone(), info: info.clone() });
                    (cache, env_1, ih, _) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, stripped_class.clone(), metamodelica::nil(), true)?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { name: id, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: cn, .. }, modifications: mod1, .. }, info, .. }) => {
                    let mut env_1: FCore::Graph;
                    let mut fpath: metamodelica::Ref<Absyn::Path>;
                    let mut mod2: metamodelica::Ref<DAE::Mod>;
                    let mut cenv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cn), None)?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    cenv = metamodelica::Own::own(__pa2);
                    (cache, mod2) = Mod::elabMod(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, mod1.clone(), false, Mod::ModScope::DERIVED { path: cn.clone() }, info.clone())?;
                    (cache, _, ih, _, _, _, ty, _, _, _) = Inst::instClass(cache.clone(), cenv.clone(), ih.clone(), UnitAbsynBuilder::emptyInstStore(), mod2.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), true, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    env_1 = env.clone();
                    (cache, fpath) = Inst::makeFullyQualifiedIdent(cache.clone(), env_1.clone(), id.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }))?;
                    ty1 = InstUtil::setFullyQualifiedTypename(ty.clone(), fpath.clone());
                    env_1 = FGraph::mkTypeNode(env_1.clone(), id.clone(), ty1.clone())?;
                    Ok((cache.clone(), env_1.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::OVERLOAD { .. }, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    (cache, env, ih, _) = implicitFunctionInstantiation2(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, inClass.clone(), metamodelica::nil(), true)?;
                    Ok((cache.clone(), env.clone(), ih.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ SCode::Element::CLASS { name: id, .. }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.implicitFunctionTypeInstantiation failed ")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("\nenv: ")); __mm_s.push_str(&*FGraph::getGraphNameStr(&inEnv)); __mm_s.push_str(&*literal!("\nelelement: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(inClass.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH))
}

fn instOverloadedFunctions(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut pre: &DAE::Prefix,
    mut inAbsynPathLst: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inInfo: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::List<DAE::Function>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outFns: metamodelica::List<DAE::Function>;
    (outCache, outEnv, outIH, outFns) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inIH.clone(), &**inAbsynPathLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, ih, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((cache.clone(), inEnv.clone(), ih.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, Deref @ metamodelica::ListNode::Cons { head: r#fn, tail: fns }) => {
                    let mut cenv: FCore::Graph;
                    let mut c: metamodelica::Ref<SCode::Element>;
                    let mut resfns1: metamodelica::List<DAE::Function>;
                    let mut resfns2: metamodelica::List<DAE::Function>;
                    let mut rest: SCode::Restriction;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&r#fn), Some(inInfo.clone()))?) {
                        (__pa0, __pa2 @ Deref @ SCode::Element::CLASS { restriction: __pa1, .. }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rest = metamodelica::Own::own(__pa1);
                    c = metamodelica::Own::own(__pa2);
                    cenv = metamodelica::Own::own(__pa3);
                    let true = (SCodeUtil::isFunctionRestriction(&rest)) else { return Err("pattern mismatch") };
                    (cache, env, ih, resfns1) = implicitFunctionInstantiation2(inCache.clone(), cenv.clone(), inIH.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), pre.clone(), c.clone(), metamodelica::nil(), false)?;
                    (cache, env, ih, resfns2) = instOverloadedFunctions(cache.clone(), env.clone(), ih.clone(), pre, metamodelica::AsArg::as_arg(&fns), inInfo)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), listAppend(resfns1.clone(), resfns2.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, Deref @ metamodelica::ListNode::Cons { head: r#fn, tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Inst.instOverloaded_functions failed ")); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outFns))
}

fn instExtDecl(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut iH: metamodelica::List<InnerOuter::TopInstance>,
    mut name: ArcStr,
    mut inScExtDecl: metamodelica::Ref<SCode::ExternalDecl>,
    mut inElements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut funcType: metamodelica::Ref<DAE::Type>,
    mut r#impl: bool,
    mut pre: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::ExternalDecl,
)> {
    let mut cache: FCore::Cache = cache;
    let mut iH: metamodelica::List<InnerOuter::TopInstance> = iH;
    let mut daeextdecl: DAE::ExternalDecl;
    let mut fname: ArcStr;
    let mut lang: ArcStr;
    let mut fargs: metamodelica::List<DAE::ExtArg>;
    let mut rettype: DAE::ExtArg;
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
    let mut extdecl: metamodelica::Ref<SCode::ExternalDecl> = inScExtDecl;
    ann = InstUtil::instExtGetAnnotation(&extdecl);
    lang = InstUtil::instExtGetLang(&extdecl)?;
    fname = InstUtil::instExtGetFname(&extdecl, name)?;
    if !(InstUtil::isExtExplicitCall(&extdecl)) {
        (fargs, rettype) = instExtMakeDefaultExternalCall(inElements, funcType, lang.clone(), info)?;
    } else {
        (cache, fargs) = InstUtil::instExtGetFargs(cache, env.clone(), &extdecl, r#impl, pre.clone(), &info)?;
        (cache, rettype) = InstUtil::instExtGetRettype(cache, env, &extdecl, r#impl, pre, info)?;
    }
    daeextdecl = DAE::ExternalDecl {
        name: fname,
        args: fargs,
        returnArg: rettype,
        language: lang,
        ann: ann,
    };
    Ok((cache, iH, daeextdecl))
}

fn instExtMakeDefaultExternalCall(
    mut elements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut funcType: metamodelica::Ref<DAE::Type>,
    mut lang: ArcStr,
    mut info: SourceInfo,
) -> Result<(metamodelica::List<DAE::ExtArg>, DAE::ExtArg)> {
    let mut fargs: metamodelica::List<DAE::ExtArg>;
    let mut rettype: DAE::ExtArg;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut singleOutput: bool;
    fargs = metamodelica::nil();
    if metamodelica::stringEq(&lang, &(literal!("builtin"))) {
        rettype = openmodelica_frontend_types::DAE::ExtArg::NOEXTARG;
        return Ok((fargs, rettype));
    }
    (rettype, singleOutput) = (::match_deref::match_deref! { match &(funcType.clone()) {
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            if !metamodelica::stringEq(&lang, &(literal!("builtin"))) {
                Error::addSourceMessage(&(Error::EXT_FN_SINGLE_RETURN_ARRAY.clone()), list![lang], &info)?;
            }
            (openmodelica_frontend_types::DAE::ExtArg::NOEXTARG, false)
        },
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { .. }, .. } => (openmodelica_frontend_types::DAE::ExtArg::NOEXTARG, false),
        Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_NORETCALL { .. }, .. } => (openmodelica_frontend_types::DAE::ExtArg::NOEXTARG, false),
        Deref @ DAE::Type::T_FUNCTION { funcResultType: __esc_ty, .. } => {
            ty = (*__esc_ty).clone();
            (DAE::ExtArg::EXTARG { componentRef: DAEUtil::varCref(&(List::find(elements, &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0)) })?))?, direction: openmodelica_ast::Absyn::Direction::OUTPUT, type_: ty.clone() }, true)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("instExtMakeDefaultExternalCall failed for ")); __mm_s.push_str(&*TypesDump::unparseType(funcType)?); ArcStr::from(__mm_s) }, info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    for mut elt in &**elements {
        fargs = (match &*elt.clone() {
            DAE::Element::VAR {
                direction: DAE::VarDirection::OUTPUT { .. },
                componentRef: __elt_componentRef,
                dims: __elt_dims,
                ..
            } if (!(singleOutput)) => addExtVarToCall(
                __elt_componentRef.clone(),
                openmodelica_ast::Absyn::Direction::OUTPUT,
                metamodelica::AsArg::as_arg(&__elt_dims),
                fargs,
            )?,
            DAE::Element::VAR {
                direction: DAE::VarDirection::INPUT { .. },
                componentRef: __elt_componentRef,
                dims: __elt_dims,
                ..
            } => addExtVarToCall(
                __elt_componentRef.clone(),
                openmodelica_ast::Absyn::Direction::INPUT,
                metamodelica::AsArg::as_arg(&__elt_dims),
                fargs,
            )?,
            DAE::Element::VAR {
                direction: DAE::VarDirection::BIDIR { .. },
                componentRef: __elt_componentRef,
                dims: __elt_dims,
                ..
            } => addExtVarToCall(
                __elt_componentRef.clone(),
                openmodelica_ast::Absyn::Direction::OUTPUT,
                metamodelica::AsArg::as_arg(&__elt_dims),
                fargs,
            )?,
            _ => fargs,
        });
    }
    fargs = fargs.reverse();
    Ok((fargs, rettype))
}

fn addExtVarToCall(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut dir: Absyn::Direction,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut fargs: metamodelica::List<DAE::ExtArg>,
) -> Result<metamodelica::List<DAE::ExtArg>> {
    let mut fargs: metamodelica::List<DAE::ExtArg> = fargs;
    fargs = metamodelica::cons(
        DAE::ExtArg::EXTARG {
            componentRef: cr.clone(),
            direction: dir,
            type_: ComponentReference::crefTypeFull(&cr)?,
        },
        fargs,
    );
    for mut dim in 1..=((dims).len() as i32) {
        fargs = metamodelica::cons(
            DAE::ExtArg::EXTARGSIZE {
                componentRef: cr.clone(),
                type_: ComponentReference::crefTypeFull(&cr)?,
                exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: dim }),
            },
            fargs,
        );
    }
    Ok(fargs)
}

pub(crate) fn getRecordConstructorFunction(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, DAE::Function)> {
    let mut outCache: FCore::Cache;
    let mut outFunc: DAE::Function;
    (outCache, outFunc) = 'mc: {
        let __mc_input = &*inPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut func: DAE::Function;
                    path = AbsynUtil::makeFullyQualified(inPath.clone());
                    func = FCore::getCachedInstFunc(&inCache, path.clone())?;
                    Ok((inCache.clone(), func.clone()))
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
                    let mut recordCl: metamodelica::Ref<SCode::Element>;
                    let mut recordEnv: FCore::Graph;
                    let mut func: DAE::Function;
                    let mut cache: FCore::Cache;
                    let mut recType: metamodelica::Ref<DAE::Type>;
                    let mut fixedTy: metamodelica::Ref<DAE::Type>;
                    let mut funcTy: metamodelica::Ref<DAE::Type>;
                    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut inputs: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut locals: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                    let mut eqCo: Option<(metamodelica::Ref<Absyn::Path>, i32, DAE::InlineType)>;
                    let mut name: ArcStr;
                    let mut newName: ArcStr;
                    let mut extConvert: bool;
                    (_, recordCl, recordEnv) = Lookup::lookupClass(&inCache, inEnv, &inPath, None)?;
                    let true = (SCodeUtil::isRecord(&recordCl)) else { return Err("pattern mismatch") };
                    name = SCodeUtil::getElementName(&recordCl)?;
                    newName = FGraph::getInstanceOriginalName(&recordEnv, name.clone());
                    recordCl = SCodeUtil::setClassName(newName.clone(), recordCl.clone())?;
                    (cache, _, _, _, _, _, recType, _, _, _) = Inst::instClass(inCache.clone(), recordEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsynBuilder::emptyInstStore(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, recordCl.clone(), metamodelica::nil(), true, openmodelica_frontend_inst::InstTypes::CallingScope::INNER_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(recType.clone()) {
                        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, varLst: __pa1, equalityConstraint: __pa2, usedExternally: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path = metamodelica::Own::own(__pa0);
                    vars = metamodelica::Own::own(__pa1);
                    eqCo = metamodelica::Own::own(__pa2);
                    extConvert = metamodelica::Own::own(__pa3);
                    vars = Types::filterRecordComponents(vars.clone(), &(SCodeUtil::elementInfo(&recordCl)))?;
                    (inputs, locals) = List::extractOnTrue(&vars, &move |__a0: metamodelica::Ref<DAE::Var>| Types::isModifiableTypesVar(&__a0))?;
                    inputs = List::map(inputs.clone(), &fnptr!(Types::setVarDefaultInput, metamodelica::Ref<DAE::Var>))?;
                    locals = List::map(locals.clone(), &fnptr!(Types::setVarProtected, metamodelica::Ref<DAE::Var>))?;
                    vars = listAppend(inputs.clone(), locals.clone());
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    fixedTy = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: vars.clone(), equalityConstraint: eqCo.clone(), usedExternally: extConvert });
                    fargs = Types::makeFargsList(inputs.clone())?;
                    funcTy = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: fixedTy.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: path.clone() });
                    func = DAE::Function::RECORD_CONSTRUCTOR { path: path.clone(), type_: funcTy.clone(), source: DAE::emptyElementSource().clone() };
                    cache = InstUtil::addFunctionsToDAE(cache.clone(), &(list![func.clone()]), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    path = AbsynUtil::pathSetLastIdent(&path, &name);
                    fixedTy = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: vars.clone(), equalityConstraint: eqCo.clone(), usedExternally: extConvert });
                    fargs = Types::makeFargsList(inputs.clone())?;
                    funcTy = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: fixedTy.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: path.clone() });
                    func = DAE::Function::RECORD_CONSTRUCTOR { path: path.clone(), type_: funcTy.clone(), source: DAE::emptyElementSource().clone() };
                    cache = InstUtil::addFunctionsToDAE(cache.clone(), &(list![func.clone()]), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    Ok((cache.clone(), func.clone()))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("InstFunction.getRecordConstructorFunction failed for ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outFunc))
}

pub(crate) fn addRecordConstructorFunction(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> FCore::Cache {
    let mut outCache: FCore::Cache;
    outCache = 'mc: {
        let __mc_input = (inCache.clone(), &**inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, .. }) => {
                    let mut cache = (*cache).clone();
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    (cache, _) = getRecordConstructorFunction(cache.clone(), inEnv, path.clone())?;
                    Ok(cache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vars, equalityConstraint: eqCo, usedExternally: extConvert }) => {
                    let mut inputs: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut locals: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut fixedTy: metamodelica::Ref<DAE::Type>;
                    let mut funcTy: metamodelica::Ref<DAE::Type>;
                    let mut func: DAE::Function;
                    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                    let mut cache = (*cache).clone();
                    let mut path = (*path).clone();
                    let mut vars = (*vars).clone();
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    vars = Types::filterRecordComponents(vars.clone(), inInfo)?;
                    (inputs, locals) = List::extractOnTrue(metamodelica::AsArg::as_arg(&vars), &move |__a0: metamodelica::Ref<DAE::Var>| Types::isModifiableTypesVar(&__a0))?;
                    inputs = List::map(inputs.clone(), &fnptr!(Types::setVarDefaultInput, metamodelica::Ref<DAE::Var>))?;
                    locals = List::map(locals.clone(), &fnptr!(Types::setVarProtected, metamodelica::Ref<DAE::Var>))?;
                    vars = listAppend(inputs.clone(), locals.clone());
                    fixedTy = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: vars.clone(), equalityConstraint: eqCo.clone(), usedExternally: extConvert.clone() });
                    fargs = Types::makeFargsList(inputs.clone())?;
                    funcTy = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: fargs.clone(), funcResultType: fixedTy.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_DEFAULT.clone(), path: path.clone() });
                    func = DAE::Function::RECORD_CONSTRUCTOR { path: path.clone(), type_: funcTy.clone(), source: DAE::emptyElementSource().clone() };
                    cache = InstUtil::addFunctionsToDAE(cache.clone(), &(list![func.clone()]), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    Ok(cache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCache
}

fn isElementImportantForFunction(mut elt: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match elt {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED { .. }, .. }, attributes: SCode::Attributes { direction: Absyn::Direction::BIDIR { .. }, variability: SCode::Variability::VAR { .. }, .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn checkExtObjOutput(mut inType: &metamodelica::Ref<DAE::Type>, mut info: SourceInfo) -> Result<()> {
    let () = (match &**inType {
        DAE::Type::T_FUNCTION {
            funcResultType: ty,
            path,
            ..
        } => {
            ::match_deref::match_deref! { match &(Types::traverseType(ty.clone(), (path.clone(), info, true), &checkExtObjOutputWork)?) {
                (_, (_, _, true)) => (),
                _ => return Err("pattern mismatch"),
            } };
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn checkExtObjOutputWork(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut inTpl: (metamodelica::Ref<Absyn::Path>, SourceInfo, bool),
) -> Result<(
    metamodelica::Ref<DAE::Type>,
    (metamodelica::Ref<Absyn::Path>, SourceInfo, bool),
)> {
    let mut oty: metamodelica::Ref<DAE::Type> = ty.clone();
    let mut outTpl: (metamodelica::Ref<Absyn::Path>, SourceInfo, bool);
    outTpl = (::match_deref::match_deref! { match &((ty, inTpl.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: path1 }, .. }, (path2, info, true)) => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut b: bool;
            let mut path1 = (*path1).clone();
            path1 = AbsynUtil::joinPaths(path1.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("constructor") }))?;
            str1 = AbsynUtil::pathStringNoQual(path2.clone(), literal!("."), false, false)?;
            str2 = AbsynUtil::pathStringNoQual(path1.clone(), literal!("."), false, false)?;
            b = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2));
            Error::assertionOrAddSourceMessage(b, &(Error::FUNCTION_RETURN_EXT_OBJ.clone()), list![str1, str2], metamodelica::AsArg::as_arg(&info))?;
            outTpl = if (b) {inTpl} else {(path2.clone(), info.clone(), false)};
            outTpl
        },
        _ => {
            inTpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oty, outTpl))
}
