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

use crate::NFSCodeEnv;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub type Env = metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;

pub type Item = metamodelica::Ref<NFSCodeEnv::Item>;

pub type Extends = metamodelica::Ref<NFSCodeEnv::Extends>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub(crate) fn flattenProgram(
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnv: Env,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Element>>, Env)> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outEnv: Env;
    (outProgram, outEnv) = List::mapFold(inProgram, &flattenClass, inEnv)?;
    Ok((outProgram, outEnv))
}

pub(crate) fn flattenClass(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
) -> Result<(metamodelica::Ref<SCode::Element>, Env)> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: Env;
    (outClass, outEnv) = 'mc: {
        let __mc_input = &*inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: cdef, info, .. } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut cls_env: metamodelica::Ref<NFSCodeEnv::Frame>;
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let mut cls_ty: NFSCodeEnv::ClassType;
                    let mut cdef = (*cdef).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupInClass(name.clone(), inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, classType: __pa1, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cls_env = metamodelica::Own::own(__pa0);
                    cls_ty = metamodelica::Own::own(__pa1);
                    env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv.clone());
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(flattenClassDef(metamodelica::AsArg::as_arg(&cdef), env.clone(), metamodelica::AsArg::as_arg(&info))?) {
                        (__pa3, Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cdef = metamodelica::Own::own(__pa3);
                    cls_env = metamodelica::Own::own(__pa4);
                    env = metamodelica::Own::own(__pa5);
                    cls = SCodeUtil::setClassDef(cdef.clone(), inClass.clone())?;
                    item = NFSCodeEnv::newClassItem(cls.clone(), list![cls_env.clone()], cls_ty);
                    env = NFSCodeEnv::updateItemInEnv(&item, &env, metamodelica::AsArg::as_arg(&name))?;
                    Ok((cls.clone(), env.clone()))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenImports.flattenClass failed on ")); __mm_s.push_str(&*SCodeUtil::elementName(&inClass)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outClass, outEnv))
}

fn flattenClassDef(
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::Ref<SCode::ClassDef>, Env)> {
    let mut outClassDef: metamodelica::Ref<SCode::ClassDef>;
    let mut outEnv: Env;
    (outClassDef, outEnv) = (match &**inClassDef {
        SCode::ClassDef::PARTS {
            elementLst: el,
            normalEquationLst: neql,
            initialEquationLst: ieql,
            normalAlgorithmLst: nal,
            initialAlgorithmLst: ial,
            constraintLst: nco,
            clsattrs: clats,
            externalDecl: extdecl,
        } => {
            let mut env: Env;
            let mut el = (*el).clone();
            let mut neql = (*neql).clone();
            let mut ieql = (*ieql).clone();
            let mut nal = (*nal).clone();
            let mut ial = (*ial).clone();
            let mut nco = (*nco).clone();
            el = List::filterOnTrue(
                el.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(isNotImport(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>),
            )?;
            (el, env) = List::mapFold(metamodelica::AsArg::as_arg(&el), &flattenElement, inEnv)?;
            neql = List::map1(neql.clone(), &flattenEquation, env.clone())?;
            ieql = List::map1(ieql.clone(), &flattenEquation, env.clone())?;
            nal = List::map1(
                nal.clone(),
                &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>,
                       __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| {
                    flattenAlgorithm(&__a0, __a1)
                },
                env.clone(),
            )?;
            ial = List::map1(
                ial.clone(),
                &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>,
                       __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| {
                    flattenAlgorithm(&__a0, __a1)
                },
                env.clone(),
            )?;
            nco = List::map2(nco.clone(), &flattenConstraints, env.clone(), inInfo.clone())?;
            (
                metamodelica::Ref::new(SCode::ClassDef::PARTS {
                    elementLst: el.clone(),
                    normalEquationLst: neql.clone(),
                    initialEquationLst: ieql.clone(),
                    normalAlgorithmLst: nal.clone(),
                    initialAlgorithmLst: ial.clone(),
                    constraintLst: nco.clone(),
                    clsattrs: clats.clone(),
                    externalDecl: extdecl.clone(),
                }),
                env,
            )
        }
        SCode::ClassDef::CLASS_EXTENDS {
            modifications: mods,
            composition: cdef,
        } => {
            let mut env: Env;
            let mut mods = (*mods).clone();
            let mut cdef = (*cdef).clone();
            (cdef, env) = flattenClassDef(metamodelica::AsArg::as_arg(&cdef), inEnv, inInfo)?;
            mods = flattenModifier(mods.clone(), env.clone(), inInfo.clone())?;
            (
                metamodelica::Ref::new(SCode::ClassDef::CLASS_EXTENDS {
                    modifications: mods.clone(),
                    composition: cdef.clone(),
                }),
                env,
            )
        }
        SCode::ClassDef::DERIVED {
            typeSpec: ty,
            modifications: mods,
            attributes: attr,
        } => {
            let mut env = inEnv.clone();
            let mut ty = (*ty).clone();
            let mut mods = (*mods).clone();
            mods = flattenModifier(mods.clone(), env.clone(), inInfo.clone())?;
            env = NFSCodeEnv::removeExtendsFromLocalScope(&env)?;
            ty = flattenTypeSpec(ty.clone(), env, inInfo.clone())?;
            (
                metamodelica::Ref::new(SCode::ClassDef::DERIVED {
                    typeSpec: ty.clone(),
                    modifications: mods.clone(),
                    attributes: attr.clone(),
                }),
                inEnv,
            )
        }
        _ => (inClassDef.clone(), inEnv),
    });
    Ok((outClassDef, outEnv))
}

fn flattenDerivedClassDef(
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut outClassDef: metamodelica::Ref<SCode::ClassDef>;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut mods: metamodelica::Ref<SCode::Mod>;
    let mut attr: SCode::Attributes;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inClassDef)) {
        Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa0, modifications: __pa1, attributes: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    mods = metamodelica::Own::own(__pa1);
    attr = metamodelica::Own::own(__pa2);
    ty = flattenTypeSpec(ty, inEnv.clone(), inInfo.clone())?;
    mods = flattenModifier(mods, inEnv, inInfo)?;
    outClassDef = metamodelica::Ref::new(SCode::ClassDef::DERIVED {
        typeSpec: ty,
        modifications: mods,
        attributes: attr,
    });
    Ok(outClassDef)
}

fn isNotImport(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outB: bool;
    outB = (match &**inElement {
        SCode::Element::IMPORT { .. } => false,
        _ => true,
    });
    outB
}

fn flattenElement(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
) -> Result<(metamodelica::Ref<SCode::Element>, Env)> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    let mut outEnv: Env;
    (outElement, outEnv) = (match &*inElement.clone() {
        SCode::Element::COMPONENT { name, .. } => {
            let mut env: Env;
            let mut elem: metamodelica::Ref<SCode::Element>;
            let mut item: Item;
            elem = flattenComponent(&inElement, inEnv.clone())?;
            item = NFSCodeEnv::newVarItem(elem.clone(), true);
            env = NFSCodeEnv::updateItemInEnv(&item, &inEnv, metamodelica::AsArg::as_arg(&name))?;
            (elem, env)
        }
        SCode::Element::CLASS { .. } => {
            let mut env: Env;
            let mut elem: metamodelica::Ref<SCode::Element>;
            (elem, env) = flattenClass(inElement, inEnv)?;
            (elem, env)
        }
        SCode::Element::EXTENDS { .. } => (flattenExtends(&inElement, inEnv.clone())?, inEnv),
        _ => (inElement, inEnv),
    });
    Ok((outElement, outEnv))
}

fn flattenComponent(
    mut inComponent: &metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outComponent: metamodelica::Ref<SCode::Element>;
    let mut name: ArcStr;
    let mut prefixes: metamodelica::Ref<SCode::Prefixes>;
    let mut attr: SCode::Attributes;
    let mut type_spec: metamodelica::Ref<Absyn::TypeSpec>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut info: SourceInfo;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &((*inComponent)) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: __pa1, attributes: __pa2, typeSpec: __pa3, modifications: __pa4, comment: __pa5, condition: __pa6, info: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    prefixes = metamodelica::Own::own(__pa1);
    attr = metamodelica::Own::own(__pa2);
    type_spec = metamodelica::Own::own(__pa3);
    r#mod = metamodelica::Own::own(__pa4);
    cmt = metamodelica::Own::own(__pa5);
    cond = metamodelica::Own::own(__pa6);
    info = metamodelica::Own::own(__pa7);
    attr = flattenAttributes(attr, inEnv.clone(), info.clone())?;
    type_spec = flattenTypeSpec(type_spec, inEnv.clone(), info.clone())?;
    r#mod = flattenModifier(r#mod, inEnv.clone(), info.clone())?;
    cond = flattenOptExp(cond, inEnv, info.clone())?;
    outComponent = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: name,
        prefixes: prefixes,
        attributes: attr,
        typeSpec: type_spec,
        modifications: r#mod,
        comment: cmt,
        condition: cond,
        info: info,
    });
    Ok(outComponent)
}

fn flattenAttributes(
    mut inAttributes: SCode::Attributes,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut ct: SCode::ConnectorType;
    let mut prl: SCode::Parallelism;
    let mut var: SCode::Variability;
    let mut dir: Absyn::Direction;
    let mut isf: Absyn::IsField;
    let SCode::ATTR {
        arrayDims: __pa0,
        connectorType: __pa1,
        parallelism: __pa2,
        variability: __pa3,
        direction: __pa4,
        isField: __pa5,
    } = inAttributes;
    ad = metamodelica::Own::own(__pa0);
    ct = metamodelica::Own::own(__pa1);
    prl = metamodelica::Own::own(__pa2);
    var = metamodelica::Own::own(__pa3);
    dir = metamodelica::Own::own(__pa4);
    isf = metamodelica::Own::own(__pa5);
    ad = List::map2(ad, &flattenSubscript, inEnv, inInfo)?;
    outAttributes = SCode::Attributes {
        arrayDims: ad,
        connectorType: ct,
        parallelism: prl,
        variability: var,
        direction: dir,
        isField: isf,
    };
    Ok(outAttributes)
}

fn flattenTypeSpec(
    mut inTypeSpec: metamodelica::Ref<Absyn::TypeSpec>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut outTypeSpec: metamodelica::Ref<Absyn::TypeSpec>;
    outTypeSpec = (::match_deref::match_deref! { match &(inTypeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { path, arrayDim: ad } => {
            let mut path = (*path).clone();
            (_, path, _) = NFSCodeLookup::lookupClassName(path.clone(), inEnv, &inInfo)?;
            metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: ad.clone() })
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. } => {
            inTypeSpec
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path, typeSpecs: tys, arrayDim: ad } => {
            let mut tys = (*tys).clone();
            tys = List::map2(tys.clone(), &flattenTypeSpec, inEnv, inInfo)?;
            metamodelica::Ref::new(Absyn::TypeSpec::TCOMPLEX { path: path.clone(), typeSpecs: tys.clone(), arrayDim: ad.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTypeSpec)
}

fn flattenExtends(
    mut inExtends: &metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outExtends: metamodelica::Ref<SCode::Element>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
    let mut info: SourceInfo;
    let mut env: Env;
    let mut vis: SCode::Visibility;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*inExtends)) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: __pa1, modifications: __pa2, ann: __pa3, info: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    vis = metamodelica::Own::own(__pa1);
    r#mod = metamodelica::Own::own(__pa2);
    ann = metamodelica::Own::own(__pa3);
    info = metamodelica::Own::own(__pa4);
    env = NFSCodeEnv::removeExtendsFromLocalScope(&inEnv)?;
    (_, path, _) = NFSCodeLookup::lookupBaseClassName(path, env, &info)?;
    r#mod = flattenModifier(r#mod, inEnv, info.clone())?;
    outExtends = metamodelica::Ref::new(SCode::Element::EXTENDS {
        baseClassPath: path,
        visibility: vis,
        modifications: r#mod,
        ann: ann,
        info: info,
    });
    Ok(outExtends)
}

fn flattenEquation(
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    let mut outEquation: metamodelica::Ref<SCode::Equation>;
    (outEquation, _) = SCodeUtil::mapFoldEquations(
        inEquation,
        (std::sync::Arc::new(flattenEquationTraverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SCode::Equation>,
                        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                    ) -> Result<(
                        metamodelica::Ref<SCode::Equation>,
                        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                    )> + 'static,
            >),
        inEnv,
    )?;
    Ok(outEquation)
}

fn flattenEquationTraverser(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut env: Env,
) -> Result<(metamodelica::Ref<SCode::Equation>, Env)> {
    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let mut env: Env = env;
    (eq, env) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_FOR { index: iter_name, info, .. } => {
            env = NFSCodeEnv::extendEnvWithIterators(&(list![metamodelica::Ref::new(Absyn::ForIterator { name: iter_name.clone(), guardExp: None, range: None })]), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env)?;
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq, env)
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: crefExp @ Deref @ Absyn::Exp::CREF { componentRef: cref }, expReinit: exp, comment: cmt, info } => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), metamodelica::AsArg::as_arg(&info));
            eq = metamodelica::Ref::new(SCode::Equation::EQ_REINIT { cref: crefExp.clone(), expReinit: exp.clone(), comment: cmt.clone(), info: info.clone() });
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo))> + 'static>), (env.clone(), info.clone()))?;
            (eq, env)
        },
        _ => {
            let mut info: SourceInfo;
            info = SCodeUtil::getEquationInfo(&eq);
            (eq, _) = SCodeUtil::mapFoldEquationExps(eq, (std::sync::Arc::new(traverseExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)) -> Result<(metamodelica::Ref<Absyn::Exp>, (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo))> + 'static>), (env.clone(), info))?;
            (eq, env)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, env))
}

fn traverseExp(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo);
    (outExp, outTuple) = AbsynUtil::traverseExpBidir(
        inExp,
        (std::sync::Arc::new(flattenExpTraverserEnter)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    )> + 'static,
            >),
        (std::sync::Arc::new(fnptr!(
            flattenExpTraverserExit,
            metamodelica::Ref<Absyn::Exp>,
            (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    )> + 'static,
            >),
        inTuple,
    )?;
    Ok((outExp, outTuple))
}

fn flattenConstraints(
    mut inConstraints: SCode::ConstraintSection,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<SCode::ConstraintSection> {
    let mut outConstraints: SCode::ConstraintSection;
    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let SCode::CONSTRAINTS { constraints: __pa0 } = inConstraints;
    exps = metamodelica::Own::own(__pa0);
    exps = List::map2(exps, &flattenExp, inEnv, inInfo)?;
    outConstraints = SCode::ConstraintSection { constraints: exps };
    Ok(outConstraints)
}

fn flattenAlgorithm(
    mut inAlgorithm: &metamodelica::Ref<SCode::AlgorithmSection>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> {
    let mut outAlgorithm: metamodelica::Ref<SCode::AlgorithmSection>;
    let mut statements: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let __arc1 = &(*inAlgorithm);
    let SCode::ALGORITHM { statements: __pa0 } = &**__arc1;
    statements = metamodelica::Own::own(__pa0);
    statements = List::map1(statements, &flattenStatement, inEnv)?;
    outAlgorithm = metamodelica::Ref::new(SCode::AlgorithmSection { statements: statements });
    Ok(outAlgorithm)
}

fn flattenStatement(
    mut inStatement: metamodelica::Ref<SCode::Statement>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    let mut outStatement: metamodelica::Ref<SCode::Statement>;
    (outStatement, _) = SCodeUtil::mapFoldStatements(
        inStatement,
        (std::sync::Arc::new(flattenStatementTraverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SCode::Statement>,
                        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                    ) -> Result<(
                        metamodelica::Ref<SCode::Statement>,
                        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                    )> + 'static,
            >),
        inEnv,
    )?;
    Ok(outStatement)
}

fn flattenStatementTraverser(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut env: Env,
) -> Result<(metamodelica::Ref<SCode::Statement>, Env)> {
    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let mut env: Env = env;
    (stmt, env) = (match &*stmt.clone() {
        SCode::Statement::ALG_FOR {
            index: iter_name, info, ..
        } => {
            env = NFSCodeEnv::extendEnvWithIterators(
                &(list![metamodelica::Ref::new(Absyn::ForIterator {
                    name: iter_name.clone(),
                    guardExp: None,
                    range: None
                })]),
                System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()),
                env,
            )?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(
                stmt,
                (std::sync::Arc::new(traverseExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                            ) -> Result<(
                                metamodelica::Ref<Absyn::Exp>,
                                (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                            )> + 'static,
                    >),
                (env.clone(), info.clone()),
            )?;
            (stmt, env)
        }
        SCode::Statement::ALG_PARFOR {
            index: iter_name, info, ..
        } => {
            env = NFSCodeEnv::extendEnvWithIterators(
                &(list![metamodelica::Ref::new(Absyn::ForIterator {
                    name: iter_name.clone(),
                    guardExp: None,
                    range: None
                })]),
                System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()),
                env,
            )?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(
                stmt,
                (std::sync::Arc::new(traverseExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                            ) -> Result<(
                                metamodelica::Ref<Absyn::Exp>,
                                (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                            )> + 'static,
                    >),
                (env.clone(), info.clone()),
            )?;
            (stmt, env)
        }
        _ => {
            let mut info: SourceInfo;
            info = SCodeUtil::getStatementInfo(&stmt)?;
            (stmt, _) = SCodeUtil::mapFoldStatementExps(
                stmt,
                (std::sync::Arc::new(traverseExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Exp>,
                                (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                            ) -> Result<(
                                metamodelica::Ref<Absyn::Exp>,
                                (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                            )> + 'static,
                    >),
                (env.clone(), info),
            )?;
            (stmt, env)
        }
    });
    Ok((stmt, env))
}

fn flattenModifier(
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &*inMod {
        SCode::Mod::MOD {
            finalPrefix: fp,
            eachPrefix: ep,
            subModLst: sub_mods,
            binding: opt_exp,
            comment: cmt,
            info,
        } => {
            let mut sub_mods = (*sub_mods).clone();
            let mut opt_exp = (*opt_exp).clone();
            opt_exp = flattenModOptExp(opt_exp.clone(), inEnv.clone(), inInfo.clone())?;
            sub_mods = List::map2(
                sub_mods.clone(),
                &move |__a0: metamodelica::Ref<SCode::SubMod>,
                       __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                       __a2: SourceInfo| flattenSubMod(&__a0, __a1, __a2),
                inEnv,
                inInfo,
            )?;
            metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: fp.clone(),
                eachPrefix: ep.clone(),
                subModLst: sub_mods.clone(),
                binding: opt_exp.clone(),
                comment: cmt.clone(),
                info: info.clone(),
            })
        }
        SCode::Mod::REDECL {
            finalPrefix: fp,
            eachPrefix: ep,
            element: el,
        } => {
            let mut el = (*el).clone();
            el = flattenRedeclare(el.clone(), inEnv)?;
            metamodelica::Ref::new(SCode::Mod::REDECL {
                finalPrefix: fp.clone(),
                eachPrefix: ep.clone(),
                element: el.clone(),
            })
        }
        SCode::Mod::NOMOD { .. } => inMod,
        _ => return Err("match: no arm matched"),
    });
    Ok(outMod)
}

fn flattenModOptExp(
    mut inOptExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut outOptExp: Option<metamodelica::Ref<Absyn::Exp>>;
    outOptExp = (::match_deref::match_deref! { match &(inOptExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv, inInfo)?;
            Some(exp.clone())
        },
        _ => {
            inOptExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOptExp)
}

fn flattenSubMod(
    mut inSubMod: &metamodelica::Ref<SCode::SubMod>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut outSubMod: metamodelica::Ref<SCode::SubMod>;
    outSubMod = (match &**inSubMod {
        SCode::SubMod { ident, r#mod } => {
            let mut r#mod = (*r#mod).clone();
            r#mod = flattenModifier(r#mod.clone(), inEnv, inInfo)?;
            metamodelica::Ref::new(SCode::SubMod {
                ident: ident.clone(),
                r#mod: r#mod.clone(),
            })
        }
    });
    Ok(outSubMod)
}

fn flattenRedeclare(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef @ Deref @ SCode::ClassDef::DERIVED { .. }, cmt, info } => {
            let mut cdef2: metamodelica::Ref<SCode::ClassDef>;
            cdef2 = flattenDerivedClassDef(metamodelica::AsArg::as_arg(&cdef), inEnv, info.clone())?;
            metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef2, cmt: cmt.clone(), info: info.clone() })
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, .. } => {
            inElement
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut element: metamodelica::Ref<SCode::Element>;
            element = flattenComponent(&inElement, inEnv)?;
            element
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Unknown redeclare in NFSCodeFlattenImports.flattenRedeclare")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn flattenSubscript(
    mut inSub: metamodelica::Ref<Absyn::Subscript>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut outSub: metamodelica::Ref<Absyn::Subscript>;
    outSub = (match &*inSub {
        Absyn::Subscript::SUBSCRIPT { subscript: exp } => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv, inInfo)?;
            metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: exp.clone() })
        }
        Absyn::Subscript::NOSUB { .. } => inSub,
    });
    Ok(outSub)
}

fn flattenExp(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    (outExp, _) = AbsynUtil::traverseExpBidir(
        inExp,
        (std::sync::Arc::new(flattenExpTraverserEnter)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    )> + 'static,
            >),
        (std::sync::Arc::new(fnptr!(
            flattenExpTraverserExit,
            metamodelica::Ref<Absyn::Exp>,
            (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
                    )> + 'static,
            >),
        (inEnv, inInfo),
    )?;
    Ok(outExp)
}

fn flattenOptExp(
    mut inExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut outExp: Option<metamodelica::Ref<Absyn::Exp>>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(exp) => {
            let mut exp = (*exp).clone();
            exp = flattenExp(exp.clone(), inEnv, inInfo)?;
            Some(exp.clone())
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn flattenExpTraverserEnter(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ Absyn::Exp::CREF { componentRef: cref }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), metamodelica::AsArg::as_arg(&info));
            (metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cref.clone() }), tup.clone())
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType, iterators: iters }, .. }, (env, info)) => {
            let mut cref = (*cref).clone();
            let mut exp = (*exp).clone();
            let mut env = (*env).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), metamodelica::AsArg::as_arg(&info));
            env = NFSCodeEnv::extendEnvWithIterators(metamodelica::AsArg::as_arg(&iters), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone())?;
            exp = flattenExp(exp.clone(), env.clone(), info.clone())?;
            (metamodelica::Ref::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: exp.clone(), iterType: iterType.clone(), iterators: iters.clone() }), typeVars: var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone() }), (env.clone(), info.clone()))
        },
        (Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "SOME", .. }, .. }, _) => {
            (inExp, inTuple)
        },
        (Deref @ Absyn::Exp::CALL { function_: cref, functionArgs: args, .. }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), metamodelica::AsArg::as_arg(&info));
            (metamodelica::Ref::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: args.clone(), typeVars: var_field!((*inExp).typeVars, Absyn::Exp::CALL).clone() }), tup.clone())
        },
        (Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, functionArgs: args }, tup @ (env, info)) => {
            let mut cref = (*cref).clone();
            cref = NFSCodeLookup::lookupComponentRef(cref.clone(), env.clone(), metamodelica::AsArg::as_arg(&info));
            (metamodelica::Ref::new(Absyn::Exp::PARTEVALFUNCTION { function_: cref.clone(), functionArgs: args.clone() }), tup.clone())
        },
        (exp @ Deref @ Absyn::Exp::MATCHEXP { .. }, (env, info)) => {
            let mut env = (*env).clone();
            env = NFSCodeEnv::extendEnvWithMatch(metamodelica::AsArg::as_arg(&exp), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), env.clone())?;
            (exp.clone(), (env.clone(), info.clone()))
        },
        _ => {
            (inExp, inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn flattenExpTraverserExit(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
) -> (
    metamodelica::Ref<Absyn::Exp>,
    (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
) {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }, .. }, (Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: env }, info)) => {
            (inExp, (env.clone(), info.clone()))
        },
        (Deref @ Absyn::Exp::MATCHEXP { .. }, (Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. }, .. }, tail: env }, info)) => {
            (inExp, (env.clone(), info.clone()))
        },
        _ => {
            (inExp, inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTuple)
}

pub(crate) fn flattenComponentRefSubs(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    outCref = (match &**inCref {
        Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
            let mut subs = (*subs).clone();
            subs = List::map2(subs.clone(), &flattenSubscript, inEnv.clone(), inInfo.clone())?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: name.clone(),
                subscripts: subs.clone(),
            })
        }
        Absyn::ComponentRef::CREF_QUAL {
            name,
            subscripts: subs,
            componentRef: cref,
        } => {
            let mut subs = (*subs).clone();
            let mut cref = (*cref).clone();
            subs = List::map2(subs.clone(), &flattenSubscript, inEnv.clone(), inInfo.clone())?;
            cref = flattenComponentRefSubs(metamodelica::AsArg::as_arg(&cref), inEnv, inInfo)?;
            metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: name.clone(),
                subscripts: subs.clone(),
                componentRef: cref.clone(),
            })
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut cref = (*cref).clone();
            cref = flattenComponentRefSubs(metamodelica::AsArg::as_arg(&cref), inEnv, inInfo)?;
            AbsynUtil::crefMakeFullyQualified(cref.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}
