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

use crate::NFSCodeCheck;
use crate::NFSCodeEnv;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeFlattenRedeclare;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Env = metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;

pub type Item = metamodelica::Ref<NFSCodeEnv::Item>;

pub type Extends = metamodelica::Ref<NFSCodeEnv::Extends>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub(crate) fn analyse(
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Element>>, Env)> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outEnv: Env;
    analyseClass(inClassName.clone(), inEnv.clone(), &(Absyn::dummyInfo.clone()))?;
    analyseClassExtends(inEnv.clone())?;
    (outEnv, outProgram) = collectUsedProgram(&inEnv, inProgram, &inClassName)?;
    Ok((outProgram, outEnv))
}

fn analyseClass(
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut item: Item;
            let mut env: Env;
            (item, env) = lookupClass(
                inClassName.clone(),
                inEnv.clone(),
                true,
                inInfo,
                Some(Error::LOOKUP_ERROR.clone()),
            )?;
            checkItemIsClass(&item)?;
            analyseItem(&item, env.clone())?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseClass failed for "));
                __mm_s.push_str(&*AbsynUtil::pathString(
                    inClassName.clone(),
                    literal!("."),
                    true,
                    false,
                )?);
                __mm_s.push_str(&*literal!(" in "));
                __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inEnv));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn lookupClass(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inBuiltinPossible: bool,
    mut inInfo: &SourceInfo,
    mut inErrorType: Option<ErrorTypes::Message>,
) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    (outItem, outEnv) = 'mc: {
        let __mc_input = inErrorType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut item: Item;
            let mut env: Env;
            (item, env) = lookupClass2(
                inPath.clone(),
                inEnv.clone(),
                inBuiltinPossible,
                inInfo,
                inErrorType.clone(),
            )?;
            (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item.clone(), env.clone());
            Ok((item.clone(), env.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let Some(mut error_id) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut name_str: ArcStr;
            let mut env_str: ArcStr;
            name_str = AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?;
            env_str = NFSCodeEnv::getEnvName(&inEnv);
            Error::addSourceMessage(&(error_id.clone()), list![name_str.clone(), env_str.clone()], inInfo)?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outEnv))
}

fn lookupClass2<'__b>(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inBuiltinPossible: bool,
    mut inInfo: &'__b SourceInfo,
    mut inErrorType: Option<ErrorTypes::Message>,
) -> Result<(Item, Env)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPath.clone(), inEnv.clone(), inBuiltinPossible)) {
            (Deref @ Absyn::Path::IDENT { .. }, _, true) => {
                let mut item: Item;
                let mut env: Env;
                (item, _, env) = NFSCodeLookup::lookupNameSilent(inPath, inEnv, inInfo)?;
                return Ok((item, env))
            },
            (Deref @ Absyn::Path::IDENT { .. }, _, false) => {
                let mut item: Item;
                let mut env: Env;
                (item, _, env) = NFSCodeLookup::lookupNameSilentNoBuiltin(inPath, inEnv, inInfo)?;
                return Ok((item, env))
            },
            (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$ce", path: Deref @ Absyn::Path::IDENT { name: id } }, Deref @ metamodelica::ListNode::Cons { head: _, tail: env }, _) => {
                let mut item: Item;
                let mut env = (*env).clone();
                return Ok(NFSCodeLookup::lookupInheritedName(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&env))?)
            },
            (Deref @ Absyn::Path::QUALIFIED { name: id, path: rest_path }, _, _) => {
                let mut item: Item;
                let mut env: Env;
                (item, _, env) = NFSCodeLookup::lookupNameSilent(metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }), inEnv, inInfo)?;
                (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item, env);
                analyseItem(&item, env.clone())?;
                return Ok(lookupNameInItem(metamodelica::AsArg::as_arg(&rest_path), item, env, inErrorType)?)
            },
            (Deref @ Absyn::Path::FULLYQUALIFIED { path: rest_path }, _, _) => {
                let mut item: Item;
                let mut env: Env;
                env = NFSCodeEnv::getEnvTopScope(inEnv)?;
                { (inPath, inEnv, inBuiltinPossible, inInfo, inErrorType) = (rest_path.clone(), env, false, inInfo, inErrorType); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lookupNameInItem<'__b>(
    mut inName: &'__b metamodelica::Ref<Absyn::Path>,
    mut inItem: Item,
    mut inEnv: Env,
    mut inErrorType: Option<ErrorTypes::Message>,
) -> Result<(Item, Env)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inItem.clone(), inEnv.clone())) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inItem, inEnv))
            },
            (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: type_path, .. }, modifications: mods, info, .. }, .. }, _) => {
                let mut env: Env;
                let mut type_env: Env;
                let mut redeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
                let mut item: Item;
                (item, type_env) = lookupClass(type_path.clone(), inEnv.clone(), true, metamodelica::AsArg::as_arg(&info), inErrorType.clone())?;
                let true = (NFSCodeEnv::isClassItem(&item)) else { return Err("pattern mismatch") };
                redeclares = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(metamodelica::AsArg::as_arg(&mods))?;
                (item, type_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redeclares, item, type_env, inEnv, NFInstPrefix::emptyPrefix().clone())?;
                { (inName, inItem, inEnv, inErrorType) = (inName, item, type_env, inErrorType); continue '__tco; }
            },
            (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, .. }, env: Deref @ metamodelica::ListNode::Cons { head: class_env, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                let mut env: Env;
                let mut item: Item;
                env = NFSCodeEnv::enterFrame(class_env.clone(), inEnv);
                return Ok(lookupClass(inName.clone(), env, false, metamodelica::AsArg::as_arg(&info), inErrorType)?)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn checkItemIsClass(mut inItem: &Item) -> Result<()> {
    let () = (::match_deref::match_deref! { match inItem {
        Deref @ NFSCodeEnv::Item::CLASS { .. } => {
            ()
        },
        Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, info, .. }, .. } => {
            Error::addSourceMessage(&(Error::LOOKUP_TYPE_FOUND_COMP.clone()), list![name.clone()], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn analyseItem(mut inItem: &Item, mut inEnv: Env) -> Result<()> {
    if NFSCodeEnv::isItemUsed(inItem) {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match inItem {
        Deref @ NFSCodeEnv::Item::VAR { .. } => {
            let mut env = inEnv.clone();
            markItemAsUsed(inItem, &env)?;
            ()
        },
        Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BASIC_TYPE { .. }, .. } => {
            ()
        },
        Deref @ NFSCodeEnv::Item::CLASS { cls: cls @ Deref @ SCode::Element::CLASS { classDef: cdef, restriction: res, info, cmt, .. }, env: Deref @ metamodelica::ListNode::Cons { head: cls_env, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut env = inEnv.clone();
            markItemAsUsed(inItem, &env)?;
            env = NFSCodeEnv::enterFrame(cls_env.clone(), env);
            if if (metamodelica::stringEq(&var_field!((**cls).name, SCode::Element::CLASS), &(literal!("cardinality")))) {(::match_deref::match_deref! { match &(inEnv) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: None, .. }, tail: Deref @ metamodelica::ListNode::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })} else {false} {
                System::setUsesCardinality(true);
            }
            analyseClassDef(metamodelica::AsArg::as_arg(&cdef), metamodelica::AsArg::as_arg(&res), env.clone(), false, info.clone())?;
            analyseMetaType(metamodelica::AsArg::as_arg(&res), env.clone(), metamodelica::AsArg::as_arg(&info))?;
            analyseComment(metamodelica::AsArg::as_arg(&cmt), env.clone(), info.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(env) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            env = metamodelica::Own::own(__pa0);
            analyseRedeclaredClass(cls.clone(), &env)?;
            ()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseItem failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inEnv)); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseItemIfRedeclares(
    mut inRepls: &metamodelica::List<NFSCodeFlattenRedeclare::Replacement>,
    mut inItem: &Item,
    mut inEnv: &Env,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inRepls {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => {
            let mut env: Env;
            let __pa0 = ::match_deref::match_deref! { match &((*inEnv)) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            env = metamodelica::Own::own(__pa0);
            analyseItemNoStopOnUsed(inItem, env)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseItemNoStopOnUsed(mut inItem: &Item, mut inEnv: Env) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**inItem, inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::VAR { .. }, env) => {
                    markItemAsUsed(inItem, metamodelica::AsArg::as_arg(&env))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { classType: NFSCodeEnv::ClassType::BASIC_TYPE { .. }, .. }, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls: cls @ Deref @ SCode::Element::CLASS { classDef: cdef, restriction: res, info, cmt, .. }, env: Deref @ metamodelica::ListNode::Cons { head: cls_env, tail: Deref @ metamodelica::ListNode::Nil }, .. }, env) => {
                    let mut env = (*env).clone();
                    markItemAsUsed(inItem, metamodelica::AsArg::as_arg(&env))?;
                    env = NFSCodeEnv::enterFrame(cls_env.clone(), env.clone());
                    analyseClassDef(metamodelica::AsArg::as_arg(&cdef), metamodelica::AsArg::as_arg(&res), env.clone(), false, info.clone())?;
                    analyseMetaType(metamodelica::AsArg::as_arg(&res), env.clone(), metamodelica::AsArg::as_arg(&info))?;
                    analyseComment(metamodelica::AsArg::as_arg(&cmt), env.clone(), info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    env = metamodelica::Own::own(__pa0);
                    analyseRedeclaredClass(cls.clone(), metamodelica::AsArg::as_arg(&env))?;
                    Ok(())
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseItemNoStopOnUsed failed on ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
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

fn markItemAsUsed(mut inItem: &Item, mut inEnv: &Env) -> Result<()> {
    let () = (::match_deref::match_deref! { match inItem {
        Deref @ NFSCodeEnv::Item::VAR { isUsed: Some(is_used), .. } => {
            Mutable::update(is_used.clone(), true);
            markEnvAsUsed(inEnv);
            ()
        },
        Deref @ NFSCodeEnv::Item::VAR { isUsed: None, .. } => {
            ()
        },
        Deref @ NFSCodeEnv::Item::CLASS { env: Deref @ metamodelica::ListNode::Cons { head: cls_env, tail: Deref @ metamodelica::ListNode::Nil }, cls: Deref @ SCode::Element::CLASS { .. }, .. } => {
            markFrameAsUsed(metamodelica::AsArg::as_arg(&cls_env));
            markEnvAsUsed(inEnv);
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn markFrameAsUsed(mut inFrame: &metamodelica::Ref<NFSCodeEnv::Frame>) -> () {
    let () = (match &**inFrame {
        NFSCodeEnv::Frame {
            isUsed: Some(is_used), ..
        } => {
            Mutable::update(is_used.clone(), true);
            ()
        }
        _ => (),
    });
    ()
}

fn markEnvAsUsed(mut inEnv: &Env) -> () {
    let () = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: f @ Deref @ NFSCodeEnv::Frame { isUsed: Some(is_used), .. }, tail: rest_env } => {
                    let false = (Mutable::access(is_used.clone())) else { return Err("pattern mismatch") };
                    markEnvAsUsed2(metamodelica::AsArg::as_arg(&f), rest_env.clone())?;
                    Mutable::update(is_used.clone(), true);
                    markEnvAsUsed(metamodelica::AsArg::as_arg(&rest_env));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    ()
}

fn markEnvAsUsed2(
    mut inFrame: &metamodelica::Ref<NFSCodeEnv::Frame>,
    mut inEnv: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
) -> Result<()> {
    let () = (match &**inFrame {
        NFSCodeEnv::Frame {
            frameType: NFSCodeEnv::FrameType::IMPLICIT_SCOPE { .. },
            ..
        } => (),
        NFSCodeEnv::Frame { name: Some(name), .. } => {
            analyseClass(
                metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
                inEnv,
                &(Absyn::dummyInfo.clone()),
            )?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn analyseClassDef(
    mut inClassDef: &metamodelica::Ref<SCode::ClassDef>,
    mut inRestriction: &SCode::Restriction,
    mut inEnv: Env,
    mut inInModifierScope: bool,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**inClassDef, &*inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: el, normalEquationLst: nel, initialEquationLst: iel, normalAlgorithmLst: nal, initialAlgorithmLst: ial, externalDecl: ext_decl, .. }, _) => {
                    analyseElements(metamodelica::AsArg::as_arg(&el), &inEnv, inRestriction)?;
                    List::map1_0(metamodelica::AsArg::as_arg(&nel), &analyseEquation, inEnv.clone())?;
                    List::map1_0(metamodelica::AsArg::as_arg(&iel), &analyseEquation, inEnv.clone())?;
                    List::map1_0(metamodelica::AsArg::as_arg(&nal), &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| analyseAlgorithm(&__a0, __a1), inEnv.clone())?;
                    List::map1_0(metamodelica::AsArg::as_arg(&ial), &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| analyseAlgorithm(&__a0, __a1), inEnv.clone())?;
                    analyseExternalDecl(ext_decl.clone(), inEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }, _) => {
                    isExternalObject(el.clone(), &inEnv, &inInfo)?;
                    analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("constructor") }), inEnv.clone(), &inInfo)?;
                    analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("destructor") }), inEnv.clone(), &inInfo)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, _) => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("NFSCodeDependency.analyseClassDef failed on CLASS_EXTENDS")], &inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, modifications: mods, .. }, Deref @ metamodelica::ListNode::Cons { head: _, tail: env }) => {
                    let mut ty_env: Env;
                    let mut nore_env: Env;
                    let mut ty_item: Item;
                    let mut redecls: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
                    let mut repls: metamodelica::List<NFSCodeFlattenRedeclare::Replacement>;
                    let mut env = (*env).clone();
                    env = if (inInModifierScope) {inEnv.clone()} else {env.clone()};
                    nore_env = NFSCodeEnv::removeRedeclaresFromLocalScope(metamodelica::AsArg::as_arg(&env))?;
                    analyseTypeSpec(metamodelica::AsArg::as_arg(&ty), nore_env.clone(), inInfo.clone())?;
                    (ty_item, _, ty_env) = NFSCodeLookup::lookupTypeSpec(ty.clone(), env.clone(), &inInfo)?;
                    (ty_item, ty_env, _) = NFSCodeEnv::resolveRedeclaredItem(ty_item.clone(), ty_env.clone());
                    ty_env = NFSCodeEnv::mergeItemEnv(&ty_item, &ty_env);
                    redecls = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(metamodelica::AsArg::as_arg(&mods))?;
                    (ty_item, ty_env, repls) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redecls.clone(), ty_item.clone(), ty_env.clone(), inEnv.clone(), NFInstPrefix::emptyPrefix().clone())?;
                    analyseItemIfRedeclares(&repls, &ty_item, &ty_env)?;
                    analyseModifier(metamodelica::AsArg::as_arg(&mods), inEnv.clone(), ty_env.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::ENUMERATION { .. }, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::OVERLOAD { pathLst: paths }, _) => {
                    if !(Config::synchronousFeaturesAllowed()?) && metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(&((paths).head().cloned()?))), &(literal!("OMC_NO_CLOCK"))) {
                        List::map2_0(&(list![(paths).head().cloned()?]), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, __a2: SourceInfo| analyseClass(__a0, __a1, &__a2), inEnv.clone(), inInfo.clone())?;
                    } else {
                        List::map2_0(metamodelica::AsArg::as_arg(&paths), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, __a2: SourceInfo| analyseClass(__a0, __a1, &__a2), inEnv.clone(), inInfo.clone())?;
                    }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::ClassDef::PDER { .. }, _) => {
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

fn isExternalObject(
    mut inElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut el: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut el_names: metamodelica::List<ArcStr>;
    el = List::filterOnTrue(
        inElements.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isNotExternalObject(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>),
    )?;
    let false = (((el).len() as i32) == ((inElements).len() as i32)) else {
        return Err("pattern mismatch");
    };
    el_names = List::filterMap(&el, &move |__a0: metamodelica::Ref<SCode::Element>| elementName(&__a0));
    checkExternalObject(el_names, inEnv, inInfo)?;
    Ok(())
}

fn elementName(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inElement {
        SCode::Element::COMPONENT { name, .. } => name.clone(),
        SCode::Element::CLASS { name, .. } => name.clone(),
        SCode::Element::DEFINEUNIT { name, .. } => name.clone(),
        SCode::Element::EXTENDS { baseClassPath: bc, .. } => {
            let mut name: ArcStr;
            name = AbsynUtil::pathString(bc.clone(), literal!("."), true, false)?;
            name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("extends "));
                __mm_s.push_str(&*name);
                ArcStr::from(__mm_s)
            };
            name
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

fn isNotExternalObject(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: Deref @ "ExternalObject" }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn checkExternalObject(
    mut inElements: metamodelica::List<ArcStr>,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inElements.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "constructor", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "destructor", tail: Deref @ metamodelica::ListNode::Nil } } => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "destructor", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "constructor", tail: Deref @ metamodelica::ListNode::Nil } } => {
            ()
        },
        _ => {
            let mut env_str: ArcStr;
            let mut has_con: bool;
            let mut has_des: bool;
            has_con = List::isMemberOnTrue(literal!("constructor"), &inElements, &fnptr!(stringEqual, ArcStr, ArcStr))?;
            has_des = List::isMemberOnTrue(literal!("destructor"), &inElements, &fnptr!(stringEqual, ArcStr, ArcStr))?;
            env_str = NFSCodeEnv::getEnvName(inEnv);
            checkExternalObject2(inElements, has_con, has_des, env_str, inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkExternalObject2(
    mut inElements: metamodelica::List<ArcStr>,
    mut inHasConstructor: bool,
    mut inHasDestructor: bool,
    mut inObjectName: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (match (inHasConstructor, inHasDestructor) {
        (true, true) => {
            let mut el = inElements;
            let mut el_str: ArcStr;
            (el, _) = List::deleteMemberOnTrue(literal!("constructor"), el, &fnptr!(stringEqual, ArcStr, ArcStr))?;
            (el, _) = List::deleteMemberOnTrue(literal!("destructor"), el, &fnptr!(stringEqual, ArcStr, ArcStr))?;
            el_str = stringDelimitList(el, literal!(", "));
            el_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("contains invalid elements: "));
                __mm_s.push_str(&*el_str);
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(
                &(Error::INVALID_EXTERNAL_OBJECT.clone()),
                list![inObjectName, el_str],
                inInfo,
            )?;
            ()
        }
        (false, true) => {
            Error::addSourceMessage(
                &(Error::INVALID_EXTERNAL_OBJECT.clone()),
                list![inObjectName, literal!("missing constructor")],
                inInfo,
            )?;
            ()
        }
        (true, false) => {
            Error::addSourceMessage(
                &(Error::INVALID_EXTERNAL_OBJECT.clone()),
                list![inObjectName, literal!("missing destructor")],
                inInfo,
            )?;
            ()
        }
        (false, false) => {
            Error::addSourceMessage(
                &(Error::INVALID_EXTERNAL_OBJECT.clone()),
                list![inObjectName, literal!("missing both constructor and destructor")],
                inInfo,
            )?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn analyseMetaType(mut inRestriction: &SCode::Restriction, mut inEnv: Env, mut inInfo: &SourceInfo) -> Result<()> {
    let () = (match inRestriction.clone() {
        SCode::Restriction::R_METARECORD {
            name: ref union_name, ..
        } => {
            analyseClass(union_name.clone(), inEnv, inInfo)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

fn analyseRedeclaredClass(mut inClass: metamodelica::Ref<SCode::Element>, mut inEnv: &Env) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &*inClass;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let false = (SCodeUtil::isElementRedeclare(&inClass)?) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { .. } => {
                    let mut item: Item;
                    item = metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: inClass.clone(), env: NFSCodeEnv::emptyEnv.clone(), classType: crate::NFSCodeEnv::ClassType::USERDEFINED });
                    analyseRedeclaredClass2(&item, inEnv)?;
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

fn analyseRedeclaredClass2(mut inItem: &Item, mut inEnv: &Env) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inItem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { info, .. }, .. } => {
                    let mut item: Item;
                    let mut env: Env;
                    (item, env) = NFSCodeLookup::lookupRedeclaredClassByItem(inItem, inEnv, metamodelica::AsArg::as_arg(&info))?;
                    analyseItem(&item, env.clone())?;
                    Ok(())
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeDependency.analyseRedeclaredClass2 failed for ")); __mm_s.push_str(&*NFSCodeEnv::getItemName(inItem)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
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

fn analyseElements(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnv: &Env,
    mut inClassRestriction: &SCode::Restriction,
) -> Result<()> {
    let mut exts: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    exts = NFSCodeEnv::getEnvExtendsFromTable(inEnv)?;
    analyseElements2(inElements, inEnv, &exts, inClassRestriction)?;
    Ok(())
}

fn analyseElements2(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnv: &Env,
    mut inExtends: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inClassRestriction: &SCode::Restriction,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inElements {
        Deref @ metamodelica::ListNode::Cons { head: el, tail: rest_el } => {
            let mut exts: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
            exts = analyseElement(metamodelica::AsArg::as_arg(&el), inEnv.clone(), inExtends.clone(), inClassRestriction)?;
            analyseElements2(rest_el, inEnv, &exts, inClassRestriction)?;
            ()
        },
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseElement(
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
    mut inExtends: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inClassRestriction: &SCode::Restriction,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    outExtends = (::match_deref::match_deref! { match &((inElement.clone(), inExtends.clone(), inClassRestriction.clone())) {
        (Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: Deref @ "ExternalObject" }, .. }, _, _) => {
            return Err("fail")
        },
        (Deref @ SCode::Element::EXTENDS { modifications: mods, info, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Extends { baseClass: bc, .. }, tail: exts }, _) => {
            let mut ty_item: Item;
            let mut ty_env: Env;
            (ty_item, _, ty_env) = NFSCodeLookup::lookupBaseClassName(bc.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&info))?;
            analyseExtends(bc.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&info))?;
            ty_env = NFSCodeEnv::mergeItemEnv(&ty_item, &ty_env);
            analyseModifier(metamodelica::AsArg::as_arg(&mods), inEnv, ty_env, info.clone())?;
            exts.clone()
        },
        (Deref @ SCode::Element::COMPONENT { name, attributes: attr, typeSpec: ty, modifications: mods, condition: cond_exp, prefixes, info, .. }, _, _) => {
            let mut ty_item: Item;
            let mut ty_env: Env;
            let mut redecls: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
            markAsUsedOnRestriction(name.clone(), inClassRestriction, &inEnv, metamodelica::AsArg::as_arg(&info));
            analyseAttributes(attr.clone(), inEnv.clone(), info.clone())?;
            analyseTypeSpec(metamodelica::AsArg::as_arg(&ty), inEnv.clone(), info.clone())?;
            (ty_item, _, ty_env) = NFSCodeLookup::lookupTypeSpec(ty.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&info))?;
            (ty_item, ty_env, _) = NFSCodeEnv::resolveRedeclaredItem(ty_item, ty_env);
            ty_env = NFSCodeEnv::mergeItemEnv(&ty_item, &ty_env);
            NFSCodeCheck::checkRecursiveComponentDeclaration(name.clone(), metamodelica::AsArg::as_arg(&info), ty_env.clone(), &ty_item, inEnv.clone())?;
            redecls = NFSCodeFlattenRedeclare::extractRedeclaresFromModifier(metamodelica::AsArg::as_arg(&mods))?;
            (ty_item, ty_env, _) = NFSCodeFlattenRedeclare::replaceRedeclaredElementsInEnv(redecls, ty_item, ty_env, inEnv.clone(), NFInstPrefix::emptyPrefix().clone())?;
            analyseModifier(metamodelica::AsArg::as_arg(&mods), inEnv.clone(), ty_env, info.clone())?;
            analyseOptExp(cond_exp.clone(), inEnv.clone(), info.clone())?;
            analyseConstrainClass(SCodeUtil::replaceableOptConstraint(&(SCodeUtil::prefixesReplaceable(metamodelica::AsArg::as_arg(&prefixes)))), inEnv, info.clone())?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, restriction: SCode::Restriction::R_OPERATOR { .. }, info, .. }, _, SCode::Restriction::R_RECORD { isOperator: true }) => {
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, restriction: SCode::Restriction::R_OPERATOR { .. }, info, .. }, _, _) => {
            let mut r#str: ArcStr;
            r#str = SCodeDump::restrString(inClassRestriction)?;
            Error::addSourceMessage(&(Error::OPERATOR_FUNCTION_NOT_EXPECTED.clone()), list![name.clone(), r#str], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        (Deref @ SCode::Element::CLASS { name, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, info, .. }, _, SCode::Restriction::R_RECORD { isOperator: true }) => {
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. } }, info, .. }, _, _) => {
            let mut r#str: ArcStr;
            r#str = SCodeDump::restrString(inClassRestriction)?;
            Error::addSourceMessage(&(Error::OPERATOR_FUNCTION_NOT_EXPECTED.clone()), list![name.clone(), r#str], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        (Deref @ SCode::Element::CLASS { name, restriction: res, info, .. }, _, SCode::Restriction::R_OPERATOR { .. }) => {
            let true = (SCodeUtil::isFunctionOrExtFunctionRestriction(metamodelica::AsArg::as_arg(&res))) else { return Err("pattern mismatch") };
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, restriction: res, info, .. }, _, SCode::Restriction::R_OPERATOR { .. }) => {
            let mut r#str: ArcStr;
            let false = (SCodeUtil::isFunctionOrExtFunctionRestriction(metamodelica::AsArg::as_arg(&res))) else { return Err("pattern mismatch") };
            r#str = SCodeDump::restrString(metamodelica::AsArg::as_arg(&res))?;
            Error::addSourceMessage(&(Error::OPERATOR_FUNCTION_EXPECTED.clone()), list![name.clone(), r#str], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        (Deref @ SCode::Element::CLASS { name: name @ Deref @ "equalityConstraint", info, .. }, _, _) => {
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, info, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, _, _) => {
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER { .. }, .. }, info, .. }, _, _) => {
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        (Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { innerOuter: Absyn::InnerOuter::INNER_OUTER { .. }, .. }, info, .. }, _, _) => {
            analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), inEnv, metamodelica::AsArg::as_arg(&info))?;
            inExtends
        },
        _ => {
            inExtends
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExtends)
}

fn markAsUsedOnConstant(
    mut inName: ArcStr,
    mut inAttr: &SCode::Attributes,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> () {
    let () = 'mc: {
        let __mc_input = (inAttr, &**inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Attributes { variability: var, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: cls_and_vars, .. }, tail: _ }) => {
                    let mut is_used: Mutable::Mutable<bool>;
                    let true = (SCodeUtil::isParameterOrConst(var.clone())) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(metamodelica::AsArg::as_arg(&cls_and_vars), inName.clone())?) {
                        Deref @ NFSCodeEnv::Item::VAR { isUsed: Some(__pa0), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    is_used = metamodelica::Own::own(__pa0);
                    Mutable::update(is_used.clone(), true);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    ()
}

fn markAsUsedOnRestriction(
    mut inName: ArcStr,
    mut inRestriction: &SCode::Restriction,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> () {
    let () = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: cls_and_vars, .. }, tail: _ } => {
                    let mut is_used: Mutable::Mutable<bool>;
                    let true = (markAsUsedOnRestriction2(inRestriction)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(metamodelica::AsArg::as_arg(&cls_and_vars), inName.clone())?) {
                        Deref @ NFSCodeEnv::Item::VAR { isUsed: Some(__pa0), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    is_used = metamodelica::Own::own(__pa0);
                    Mutable::update(is_used.clone(), true);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    ()
}

fn markAsUsedOnRestriction2(mut inRestriction: &SCode::Restriction) -> bool {
    let mut isRestricted: bool;
    isRestricted = (match inRestriction.clone() {
        SCode::Restriction::R_CONNECTOR { .. } => true,
        SCode::Restriction::R_RECORD { isOperator: _ } => true,
        _ => false,
    });
    isRestricted
}

fn analyseExtends(
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut item: Item;
    let mut env: Env;
    (item, env) = lookupClass(inClassName, inEnv, true, inInfo, None)?;
    analyseItem(&item, env)?;
    Ok(())
}

fn analyseAttributes(mut inAttributes: SCode::Attributes, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let SCode::ATTR { arrayDims: __pa0, .. } = inAttributes;
    ad = metamodelica::Own::own(__pa0);
    List::map2_0(
        &ad,
        &move |__a0: metamodelica::Ref<Absyn::Subscript>,
               __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
               __a2: SourceInfo| analyseSubscript(&__a0, __a1, __a2),
        inEnv,
        inInfo,
    )?;
    Ok(())
}

fn analyseModifier(
    mut inModifier: &metamodelica::Ref<SCode::Mod>,
    mut inEnv: Env,
    mut inTypeEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (match &**inModifier {
        SCode::Mod::NOMOD { .. } => (),
        SCode::Mod::MOD {
            subModLst: sub_mods,
            binding: bind_exp,
            ..
        } => {
            List::map2_0(
                sub_mods,
                &move |__a0: metamodelica::Ref<SCode::SubMod>,
                       __a1: (
                    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
                ),
                       __a2: SourceInfo| analyseSubMod(&__a0, &__a1, __a2),
                (inEnv.clone(), inTypeEnv),
                inInfo.clone(),
            )?;
            analyseModBinding(bind_exp.clone(), inEnv, inInfo)?;
            ()
        }
        SCode::Mod::REDECL { element: el, .. } => {
            analyseRedeclareModifier(el, inEnv, &inTypeEnv)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn analyseRedeclareModifier(
    mut inElement: &metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
    mut inTypeEnv: &Env,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { prefixes, classDef: cdef, restriction: restr, info, .. } => {
                    analyseClassDef(metamodelica::AsArg::as_arg(&cdef), metamodelica::AsArg::as_arg(&restr), inEnv.clone(), true, info.clone())?;
                    analyseConstrainClass(SCodeUtil::replaceableOptConstraint(&(SCodeUtil::prefixesReplaceable(metamodelica::AsArg::as_arg(&prefixes)))), inEnv.clone(), info.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    analyseElement(inElement, inEnv.clone(), metamodelica::nil(), &(openmodelica_frontend_types::SCode::Restriction::R_CLASS))?;
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

fn analyseConstrainClass(
    mut inCC: Option<metamodelica::Ref<SCode::ConstrainClass>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inCC) {
        Some(Deref @ SCode::ConstrainClass { constrainingClass: path, modifier: r#mod, .. }) => {
            let mut env: Env;
            analyseClass(path.clone(), inEnv.clone(), &inInfo)?;
            (_, env) = lookupClass(path.clone(), inEnv.clone(), true, &inInfo, Some(Error::LOOKUP_ERROR.clone()))?;
            analyseModifier(metamodelica::AsArg::as_arg(&r#mod), inEnv, env, inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseSubMod(
    mut inSubMod: &metamodelica::Ref<SCode::SubMod>,
    mut inEnv: &(
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    ),
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((&**inSubMod, inEnv)) {
        (Deref @ SCode::SubMod { ident, r#mod: m }, (env, ty_env)) => {
            analyseNameMod(ident.clone(), env.clone(), ty_env.clone(), metamodelica::AsArg::as_arg(&m), inInfo)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseNameMod(
    mut inIdent: ArcStr,
    mut inEnv: Env,
    mut inTypeEnv: Env,
    mut inMod: &metamodelica::Ref<SCode::Mod>,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let mut item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (item, env) = lookupNameMod(
        metamodelica::Ref::new(Absyn::Path::IDENT { name: inIdent.clone() }),
        inTypeEnv.clone(),
        &inInfo,
    );
    analyseNameMod2(&inIdent, item, env, inEnv, inTypeEnv, inMod, inInfo)?;
    Ok(())
}

fn analyseNameMod2(
    mut inIdent: &ArcStr,
    mut inItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    mut inItemEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
    mut inEnv: Env,
    mut inTypeEnv: Env,
    mut inModifier: &metamodelica::Ref<SCode::Mod>,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inItem, inItemEnv)) {
        (Some(item), Some(env)) => {
            let mut env = (*env).clone();
            NFSCodeCheck::checkModifierIfRedeclare(metamodelica::AsArg::as_arg(&item), inModifier, inInfo.clone())?;
            analyseItem(metamodelica::AsArg::as_arg(&item), env.clone())?;
            env = NFSCodeEnv::mergeItemEnv(metamodelica::AsArg::as_arg(&item), metamodelica::AsArg::as_arg(&env));
            analyseModifier(inModifier, inEnv, env.clone(), inInfo)?;
            ()
        },
        _ => {
            analyseModifier(inModifier, inEnv, inTypeEnv, inInfo)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn lookupNameMod(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> (
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
) {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outEnv) = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut item: Item;
            let mut env: Env;
            (item, _, env) = NFSCodeLookup::lookupNameSilent(inPath.clone(), inEnv.clone(), inInfo)?;
            (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item.clone(), env.clone());
            Ok((Some(item.clone()), Some(env.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((None, None))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outItem, outEnv)
}

fn analyseSubscript(
    mut inSubscript: &metamodelica::Ref<Absyn::Subscript>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (match &**inSubscript {
        Absyn::Subscript::NOSUB { .. } => (),
        Absyn::Subscript::SUBSCRIPT { subscript: sub_exp } => {
            analyseExp(sub_exp.clone(), inEnv, inInfo)?;
            ()
        }
    });
    Ok(())
}

fn analyseModBinding(
    mut inBinding: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inBinding) {
        None => {
            ()
        },
        Some(bind_exp) => {
            analyseExp(bind_exp.clone(), inEnv, inInfo)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseTypeSpec(
    mut inTypeSpec: &metamodelica::Ref<Absyn::TypeSpec>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inTypeSpec {
        Deref @ Absyn::TypeSpec::TPATH { path: type_path, arrayDim: ad } => {
            analyseClass(type_path.clone(), inEnv.clone(), &inInfo)?;
            analyseTypeSpecDims(ad.clone(), inEnv, inInfo)?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. } => {
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { typeSpecs: tys, .. } => {
            List::map2_0(tys, &move |__a0: metamodelica::Ref<Absyn::TypeSpec>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, __a2: SourceInfo| analyseTypeSpec(&__a0, __a1, __a2), inEnv, inInfo)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseTypeSpecDims(
    mut inDims: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inDims) {
        Some(dims) => {
            List::map2_0(metamodelica::AsArg::as_arg(&dims), &move |__a0: metamodelica::Ref<Absyn::Subscript>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, __a2: SourceInfo| analyseTypeSpecDim(&__a0, __a1, __a2), inEnv, inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseTypeSpecDim(
    mut inDim: &metamodelica::Ref<Absyn::Subscript>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (match &**inDim {
        Absyn::Subscript::NOSUB { .. } => (),
        Absyn::Subscript::SUBSCRIPT { subscript: dim } => {
            analyseExp(dim.clone(), inEnv, inInfo)?;
            ()
        }
    });
    Ok(())
}

fn analyseExternalDecl(
    mut inExtDecl: Option<metamodelica::Ref<SCode::ExternalDecl>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inExtDecl) {
        Some(Deref @ SCode::ExternalDecl { args, annotation_: None, .. }) => {
            List::map2_0(metamodelica::AsArg::as_arg(&args), &analyseExp, inEnv, inInfo)?;
            ()
        },
        Some(Deref @ SCode::ExternalDecl { args, annotation_: Some(ann), .. }) => {
            List::map2_0(metamodelica::AsArg::as_arg(&args), &analyseExp, inEnv.clone(), inInfo.clone())?;
            analyseAnnotation(metamodelica::AsArg::as_arg(&ann), inEnv, inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseComment(
    mut inComment: &metamodelica::Ref<SCode::Comment>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inComment {
        Deref @ SCode::Comment { annotation_: Some(ann), .. } => {
            analyseAnnotation(metamodelica::AsArg::as_arg(&ann), inEnv, inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseAnnotation(
    mut inAnnotation: &metamodelica::Ref<SCode::Annotation>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inAnnotation {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: sub_mods, .. } } => {
            List::map2_0(metamodelica::AsArg::as_arg(&sub_mods), &move |__a0: metamodelica::Ref<SCode::SubMod>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, __a2: SourceInfo| -> metamodelica::Result<_> { ::std::result::Result::Ok(analyseAnnotationMod(&__a0, __a1, __a2)) }, inEnv, inInfo)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn analyseAnnotationMod(mut inMod: &metamodelica::Ref<SCode::SubMod>, mut inEnv: Env, mut inInfo: SourceInfo) -> () {
    let () = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::SubMod { ident: Deref @ "derivative", r#mod: mods } => {
                    analyseModifier(metamodelica::AsArg::as_arg(&mods), inEnv.clone(), NFSCodeEnv::emptyEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::SubMod { ident: Deref @ "inverse", r#mod: mods } => {
                    analyseModifier(metamodelica::AsArg::as_arg(&mods), inEnv.clone(), NFSCodeEnv::emptyEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::SubMod { ident: id, r#mod: mods } => {
                    analyseAnnotationName(id.clone(), inEnv.clone(), &inInfo)?;
                    analyseModifier(metamodelica::AsArg::as_arg(&mods), inEnv.clone(), NFSCodeEnv::emptyEnv.clone(), inInfo.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    ()
}

fn analyseAnnotationName(mut inName: ArcStr, mut inEnv: Env, mut inInfo: &SourceInfo) -> Result<()> {
    let mut item: Item;
    let mut env: Env;
    (item, _, env) = NFSCodeLookup::lookupNameSilent(
        metamodelica::Ref::new(Absyn::Path::IDENT { name: inName }),
        inEnv,
        inInfo,
    )?;
    (item, env, _) = NFSCodeEnv::resolveRedeclaredItem(item, env);
    analyseItem(&item, env)?;
    Ok(())
}

fn analyseExp(mut inExp: metamodelica::Ref<Absyn::Exp>, mut inEnv: Env, mut inInfo: SourceInfo) -> Result<()> {
    AbsynUtil::traverseExpBidir(
        inExp,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Absyn::Exp>,
                  __a1: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)| {
                analyseExpTraverserEnter(__a0, &__a1)
            },
        )
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
            analyseExpTraverserExit,
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
    Ok(())
}

fn analyseOptExp(
    mut inExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inEnv: Env,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inExp) {
        Some(exp) => {
            analyseExp(exp.clone(), inEnv, inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn analyseExpTraverserEnter(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: &(metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo),
)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outTuple: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo);
    let mut env: Env;
    let mut info: SourceInfo;
    (env, info) = inTuple.clone();
    env = analyseExp2(&inExp, env, &info)?;
    outExp = inExp;
    outTuple = (env, info);
    Ok((outExp, outTuple))
}

fn analyseExp2(mut inExp: &metamodelica::Ref<Absyn::Exp>, mut inEnv: Env, mut inInfo: &SourceInfo) -> Result<Env> {
    let mut outEnv: Env;
    outEnv = (::match_deref::match_deref! { match inExp {
        Deref @ Absyn::Exp::CREF { componentRef: cref } => {
            analyseCref(cref, inEnv.clone(), inInfo);
            inEnv
        },
        Deref @ Absyn::Exp::CALL { function_: cref, functionArgs: Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators: iters, .. }, .. } => {
            let mut env: Env;
            analyseCref(cref, inEnv.clone(), inInfo);
            env = NFSCodeEnv::extendEnvWithIterators(metamodelica::AsArg::as_arg(&iters), System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), inEnv)?;
            env
        },
        Deref @ Absyn::Exp::CALL { function_: cref, .. } => {
            analyseCref(cref, inEnv.clone(), inInfo);
            inEnv
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: cref, .. } => {
            analyseCref(cref, inEnv.clone(), inInfo);
            inEnv
        },
        Deref @ Absyn::Exp::MATCHEXP { .. } => {
            let mut env: Env;
            env = NFSCodeEnv::extendEnvWithMatch(inExp, System::tmpTickIndex(NFSCodeEnv::tmpTickIndex.clone()), inEnv)?;
            env
        },
        _ => {
            inEnv
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn analyseCref(mut inCref: &metamodelica::Ref<Absyn::ComponentRef>, mut inEnv: Env, mut inInfo: &SourceInfo) -> () {
    let () = 'mc: {
        let __mc_input = &**inCref;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::WILD { .. } => {
                    Ok(())
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
                    let mut item: Item;
                    let mut env: Env;
                    path = AbsynUtil::crefToPathIgnoreSubs(inCref)?;
                    (item, env) = lookupClass(path.clone(), inEnv.clone(), true, inInfo, None)?;
                    analyseItem(&item, env.clone())?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    ()
}

fn analyseExpTraverserExit(
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

fn analyseEquation(mut inEquation: metamodelica::Ref<SCode::Equation>, mut inEnv: Env) -> Result<()> {
    SCodeUtil::mapFoldEquations(
        inEquation,
        (std::sync::Arc::new(analyseEquationTraverser)
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
    Ok(())
}

fn analyseEquationTraverser(
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
        Deref @ SCode::Equation::EQ_REINIT { cref: Deref @ Absyn::Exp::CREF { componentRef: cref1 }, info, .. } => {
            analyseCref(metamodelica::AsArg::as_arg(&cref1), env.clone(), metamodelica::AsArg::as_arg(&info));
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
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Absyn::Exp>,
                  __a1: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, SourceInfo)| {
                analyseExpTraverserEnter(__a0, &__a1)
            },
        )
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
            analyseExpTraverserExit,
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

fn analyseAlgorithm(mut inAlgorithm: &metamodelica::Ref<SCode::AlgorithmSection>, mut inEnv: Env) -> Result<()> {
    let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let __arc1 = &(*inAlgorithm);
    let SCode::ALGORITHM { statements: __pa0 } = &**__arc1;
    stmts = metamodelica::Own::own(__pa0);
    List::map1_0(&stmts, &analyseStatement, inEnv)?;
    Ok(())
}

fn analyseStatement(mut inStatement: metamodelica::Ref<SCode::Statement>, mut inEnv: Env) -> Result<()> {
    SCodeUtil::mapFoldStatements(
        inStatement,
        (std::sync::Arc::new(analyseStatementTraverser)
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
    Ok(())
}

fn analyseStatementTraverser(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut env: Env,
) -> Result<(metamodelica::Ref<SCode::Statement>, Env)> {
    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let mut env: Env = env;
    (stmt, env) = (match &*stmt {
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
            SCodeUtil::mapFoldStatementExps(
                stmt.clone(),
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
            SCodeUtil::mapFoldStatementExps(
                stmt.clone(),
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
            SCodeUtil::mapFoldStatementExps(
                stmt.clone(),
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

fn analyseClassExtends(mut inEnv: Env) -> Result<()> {
    let mut tree: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    tree = metamodelica::Own::own(__pa0);
    NFSCodeEnv::EnvTree::foldCond(
        &tree,
        &move |__a0: ArcStr,
               __a1: metamodelica::Ref<NFSCodeEnv::Item>,
               __a2: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(analyseAvlValue(&__a0, &__a1, __a2)) },
        inEnv,
    )?;
    Ok(())
}

fn analyseAvlValue(mut key: &ArcStr, mut value: &Item, mut env: Env) -> (Env, bool) {
    let mut env: Env = env;
    let mut cont: bool;
    cont = 'mc: {
        let __mc_input = (&**value, &*env);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(_), isUsed: Some(is_used), .. }, tail: _ }) => {
                    let false = (Mutable::access(is_used.clone())) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ NFSCodeEnv::Item::CLASS { cls, env: Deref @ metamodelica::ListNode::Cons { head: cls_env, tail: Deref @ metamodelica::ListNode::Nil }, classType: cls_ty }, _) => {
                    let mut env2: Env;
                    env2 = NFSCodeEnv::enterFrame(cls_env.clone(), env.clone());
                    analyseClassExtendsDef(cls.clone(), cls_ty.clone(), env2.clone());
                    analyseClassExtends(env2.clone())?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (env, cont)
}

fn analyseClassExtendsDef(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inClassType: NFSCodeEnv::ClassType,
    mut inEnv: Env,
) -> () {
    let () = 'mc: {
        let __mc_input = (&*inClass, inClassType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { name: cls_name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: bc, .. }, tail: _ }, .. }, info, .. }, NFSCodeEnv::ClassType::CLASS_EXTENDS { .. }) => {
                    let mut item: Item;
                    let mut env: Env;
                    (item, _, env) = NFSCodeLookup::lookupBaseClassName(bc.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&info))?;
                    let true = (NFSCodeEnv::isItemUsed(&item)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    env = metamodelica::Own::own(__pa0);
                    analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: cls_name.clone() }), env.clone(), metamodelica::AsArg::as_arg(&info))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { name: cls_name, info, .. }, NFSCodeEnv::ClassType::USERDEFINED { .. }) => {
                    let mut item: Item;
                    let mut env: Env;
                    let true = (SCodeUtil::isElementRedeclare(&inClass)?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(inEnv.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    env = metamodelica::Own::own(__pa0);
                    item = metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: inClass.clone(), env: NFSCodeEnv::emptyEnv.clone(), classType: inClassType.clone() });
                    (item, _) = NFSCodeLookup::lookupRedeclaredClassByItem(&item, &env, metamodelica::AsArg::as_arg(&info))?;
                    let true = (NFSCodeEnv::isItemUsed(&item)) else { return Err("pattern mismatch") };
                    analyseClass(metamodelica::Ref::new(Absyn::Path::IDENT { name: cls_name.clone() }), env.clone(), metamodelica::AsArg::as_arg(&info))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    ()
}

fn collectUsedProgram(
    mut inEnv: &Env,
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
) -> Result<(Env, metamodelica::List<metamodelica::Ref<SCode::Element>>)> {
    let mut outEnv: Env;
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut env: Env;
    let mut cls_and_vars: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    env = NFSCodeEnv::buildInitialEnv()?;
    let __pa0 = ::match_deref::match_deref! { match &((*inEnv)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cls_and_vars = metamodelica::Own::own(__pa0);
    (outProgram, outEnv) = collectUsedProgram2(&cls_and_vars, inEnv, inProgram, inClassName, env)?;
    Ok((outEnv, outProgram))
}

fn collectUsedProgram2(
    mut clsAndVars: &metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>,
    mut inEnv: &Env,
    mut inProgram: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inAccumEnv: Env,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Element>>, Env)> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outAccumEnv: Env;
    (outProgram, outAccumEnv) = 'mc: {
        let __mc_input = (&**inProgram, inAccumEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((inProgram.clone(), inAccumEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cls @ Deref @ SCode::Element::CLASS { name, .. }, tail: rest_prog }, env) => {
                    let mut cls_el: metamodelica::Ref<SCode::Element>;
                    let mut rest_prog = (*rest_prog).clone();
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(collectUsedClass(metamodelica::AsArg::as_arg(&cls), inEnv.clone(), clsAndVars, inClassName, metamodelica::AsArg::as_arg(&env), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() })))?) {
                        (__pa0 @ Deref @ SCode::Element::CLASS { .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cls_el = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    (rest_prog, env) = collectUsedProgram2(clsAndVars, inEnv, metamodelica::AsArg::as_arg(&rest_prog), inClassName, env.clone())?;
                    Ok((metamodelica::cons(cls_el.clone(), rest_prog.clone()), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { .. }, tail: rest_prog }, env) => {
                    let mut rest_prog = (*rest_prog).clone();
                    let mut env = (*env).clone();
                    (rest_prog, env) = collectUsedProgram2(clsAndVars, inEnv, metamodelica::AsArg::as_arg(&rest_prog), inClassName, env.clone())?;
                    Ok((rest_prog.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outProgram, outAccumEnv))
}

fn collectUsedClass(
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
    mut inClsAndVars: &metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inAccumEnv: &Env,
    mut inAccumPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<(metamodelica::Ref<SCode::Element>, Env)> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outAccumEnv: Env;
    (outClass, outAccumEnv) = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { name, prefixes: prefixes @ Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: _ }, .. }, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef, cmt, info } => {
            let mut basename: ArcStr;
            let mut item: Item;
            let mut resolved_item: Item;
            let mut class_frame: metamodelica::Ref<NFSCodeEnv::Frame>;
            let mut class_env: Env;
            let mut env: Env;
            let mut enclosing_env: Env;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut cdef = (*cdef).clone();
            item = NFSCodeEnv::EnvTree::get(inClsAndVars, name.clone())?;
            (resolved_item, _) = NFSCodeLookup::resolveAlias(item.clone(), inEnv.clone())?;
            let true = (checkClassUsed(&resolved_item, metamodelica::AsArg::as_arg(&cdef))) else { return Err("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemEnv(&resolved_item)?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            class_frame = metamodelica::Own::own(__pa0);
            enclosing_env = NFSCodeEnv::enterScope(inEnv, name.clone())?;
            (cdef, class_env) = collectUsedClassDef(cdef.clone(), enclosing_env, &class_frame, inClassName, inAccumPath)?;
            cls = metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
            resolved_item = updateItemEnv(&resolved_item, cls.clone(), class_env)?;
            basename = { let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*arcstr::literal!(NFSCodeEnv::BASE_CLASS_SUFFIX)); ArcStr::from(__mm_s) };
            env = NFSCodeEnv::extendEnvWithItem(&resolved_item, inAccumEnv, &basename)?;
            env = NFSCodeEnv::extendEnvWithItem(&item, &env, name)?;
            (cls, env)
        },
        Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: cdef, cmt, info } => {
            let mut item: Item;
            let mut class_frame: metamodelica::Ref<NFSCodeEnv::Frame>;
            let mut class_env: Env;
            let mut env: Env;
            let mut enclosing_env: Env;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut cdef = (*cdef).clone();
            SCodeUtil::replaceableOptConstraint(&(SCodeUtil::prefixesReplaceable(prefixes)));
            item = NFSCodeEnv::EnvTree::get(inClsAndVars, name.clone())?;
            let true = (checkClassUsed(&item, metamodelica::AsArg::as_arg(&cdef))) else { return Err("pattern mismatch") };
            let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::getItemEnv(&item)?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            class_frame = metamodelica::Own::own(__pa0);
            enclosing_env = NFSCodeEnv::enterScope(inEnv, name.clone())?;
            (cdef, class_env) = collectUsedClassDef(cdef.clone(), enclosing_env, &class_frame, inClassName, inAccumPath)?;
            cls = metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
            item = updateItemEnv(&item, cls.clone(), class_env)?;
            env = NFSCodeEnv::extendEnvWithItem(&item, inAccumEnv, name)?;
            (cls, env)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outClass, outAccumEnv))
}

fn checkClassUsed(mut inItem: &Item, mut inClassDef: &metamodelica::Ref<SCode::ClassDef>) -> bool {
    let mut isUsed: bool;
    isUsed = (::match_deref::match_deref! { match inItem {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name: Deref @ "GraphicalAnnotationsProgram____", .. }, .. } => true,
        _ => NFSCodeEnv::isItemUsed(inItem),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUsed
}

fn updateItemEnv(mut inItem: &Item, mut inClass: metamodelica::Ref<SCode::Element>, mut inEnv: Env) -> Result<Item> {
    let mut outItem: Item;
    outItem = (match &**inItem {
        NFSCodeEnv::Item::CLASS { classType: cls_ty, .. } => metamodelica::Ref::new(NFSCodeEnv::Item::CLASS {
            cls: inClass,
            env: inEnv,
            classType: cls_ty.clone(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outItem)
}

fn collectUsedClassDef(
    mut classDef: metamodelica::Ref<SCode::ClassDef>,
    mut env: Env,
    mut inClassEnv: &metamodelica::Ref<NFSCodeEnv::Frame>,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inAccumPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<(metamodelica::Ref<SCode::ClassDef>, Env)> {
    let mut classDef: metamodelica::Ref<SCode::ClassDef> = classDef;
    let mut env: Env = env;
    let () = (match &*classDef {
        SCode::ClassDef::PARTS { elementLst: el, .. } => {
            let mut el = (*el).clone();
            (el, env) = collectUsedElements(
                metamodelica::AsArg::as_arg(&el),
                env,
                inClassEnv,
                inClassName,
                inAccumPath.clone(),
            )?;
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = el.clone());
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS { composition: cdef, .. } => {
            let mut cdef = (*cdef).clone();
            (cdef, env) = collectUsedClassDef(cdef.clone(), env, inClassEnv, inClassName, inAccumPath)?;
            assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; composition = cdef.clone());
            ()
        }
        _ => {
            env = list![inClassEnv.clone()];
            ()
        }
    });
    Ok((classDef, env))
}

fn collectUsedElements(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnv: Env,
    mut inClassEnv: &metamodelica::Ref<NFSCodeEnv::Frame>,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inAccumPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Element>>, Env)> {
    let mut outUsedElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outNewEnv: Env;
    let mut empty_class_env: metamodelica::Ref<NFSCodeEnv::Frame>;
    let mut cls_and_vars: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    let mut collect_constants: bool;
    (empty_class_env, cls_and_vars) = NFSCodeEnv::removeClsAndVarsFromFrame(inClassEnv);
    collect_constants = AbsynUtil::pathEqual(inClassName, &inAccumPath);
    (outUsedElements, outNewEnv) = collectUsedElements2(
        inElements,
        inEnv.clone(),
        &cls_and_vars,
        &(metamodelica::nil()),
        list![empty_class_env],
        inClassName,
        inAccumPath,
        collect_constants,
    );
    outNewEnv = removeUnusedRedeclares(&outNewEnv, &inEnv)?;
    Ok((outUsedElements, outNewEnv))
}

fn collectUsedElements2(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnclosingEnv: Env,
    mut inClsAndVars: &metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>,
    mut inAccumElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inAccumEnv: Env,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inAccumPath: metamodelica::Ref<Absyn::Path>,
    mut inCollectConstants: bool,
) -> (metamodelica::List<metamodelica::Ref<SCode::Element>>, Env) {
    let mut outAccumElements: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    let mut accum_env: Env = inAccumEnv;
    let mut accum_el: metamodelica::Ref<SCode::Element>;
    for mut el in &**inElements {
        if '__try0: {
            (accum_el, accum_env) = unwrap_break_err!(collectUsedElement(el.clone(), inEnclosingEnv.clone(), inClsAndVars, accum_env.clone(), inClassName, inAccumPath.clone(), inCollectConstants), '__try0);
            outAccumElements = metamodelica::cons(accum_el.clone(), outAccumElements.clone());
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    outAccumElements = outAccumElements.reverse();
    (outAccumElements, accum_env)
}

fn collectUsedElement(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inEnclosingEnv: Env,
    mut inClsAndVars: &metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>,
    mut inAccumEnv: Env,
    mut inClassName: &metamodelica::Ref<Absyn::Path>,
    mut inAccumPath: metamodelica::Ref<Absyn::Path>,
    mut inCollectConstants: bool,
) -> Result<(metamodelica::Ref<SCode::Element>, Env)> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    let mut outAccumEnv: Env;
    (outElement, outAccumEnv) = (match &*inElement {
        SCode::Element::CLASS { name, .. } => {
            let mut env = inAccumEnv.clone();
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut cls_path: metamodelica::Ref<Absyn::Path>;
            cls_path = AbsynUtil::joinPaths(
                inAccumPath,
                metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }),
            )?;
            (cls, env) = collectUsedClass(&inElement, inEnclosingEnv, inClsAndVars, inClassName, &env, &cls_path)?;
            (cls, env)
        }
        SCode::Element::COMPONENT {
            name,
            attributes:
                SCode::Attributes {
                    variability: SCode::Variability::CONST { .. },
                    ..
                },
            ..
        } => {
            let mut env: Env;
            let mut item: Item;
            item = NFSCodeEnv::EnvTree::get(inClsAndVars, name.clone())?;
            let true = (inCollectConstants || NFSCodeEnv::isItemUsed(&item)) else {
                return Err("pattern mismatch");
            };
            env = NFSCodeEnv::extendEnvWithItem(&item, &inAccumEnv, metamodelica::AsArg::as_arg(&name))?;
            (inElement, env)
        }
        SCode::Element::COMPONENT { name, .. } => {
            let mut env: Env;
            let mut item: Item;
            item = NFSCodeEnv::newVarItem(inElement.clone(), true);
            env = NFSCodeEnv::extendEnvWithItem(&item, &inAccumEnv, metamodelica::AsArg::as_arg(&name))?;
            (inElement, env)
        }
        _ => (inElement, inAccumEnv),
    });
    Ok((outElement, outAccumEnv))
}

fn removeUnusedRedeclares(mut inEnv: &Env, mut inTotalEnv: &Env) -> Result<Env> {
    let mut outEnv: Env;
    let mut name: Option<ArcStr>;
    let mut ty: NFSCodeEnv::FrameType;
    let mut cls_and_vars: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let mut re: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cei: Option<metamodelica::Ref<SCode::Element>>;
    let mut imps: NFSCodeEnv::ImportTable;
    let mut is_used: Option<Mutable::Mutable<bool>>;
    let mut env: Env;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &((*inEnv)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa3, redeclaredElements: __pa4, classExtendsInfo: __pa5 }, importTable: __pa6, isUsed: __pa7 }, tail: Deref @ metamodelica::ListNode::Nil } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    cls_and_vars = metamodelica::Own::own(__pa2);
    bcl = metamodelica::Own::own(__pa3);
    re = metamodelica::Own::own(__pa4);
    cei = metamodelica::Own::own(__pa5);
    imps = metamodelica::Own::own(__pa6);
    is_used = metamodelica::Own::own(__pa7);
    env = NFSCodeEnv::removeRedeclaresFromLocalScope(inTotalEnv)?;
    bcl = List::map1(
        bcl,
        &move |__a0: metamodelica::Ref<NFSCodeEnv::Extends>,
               __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(removeUnusedRedeclares2(&__a0, __a1)) },
        env,
    )?;
    outEnv = list![metamodelica::Ref::new(NFSCodeEnv::Frame {
        name: name,
        frameType: ty,
        clsAndVars: cls_and_vars,
        extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
            baseClasses: bcl,
            redeclaredElements: re,
            classExtendsInfo: cei
        }),
        importTable: imps,
        isUsed: is_used
    })];
    Ok(outEnv)
}

fn removeUnusedRedeclares2(
    mut inExtends: &metamodelica::Ref<NFSCodeEnv::Extends>,
    mut inEnv: Env,
) -> metamodelica::Ref<NFSCodeEnv::Extends> {
    let mut outExtends: metamodelica::Ref<NFSCodeEnv::Extends>;
    let mut bc: metamodelica::Ref<Absyn::Path>;
    let mut redeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
    let mut index: i32;
    let mut info: SourceInfo;
    let __arc4 = &(*inExtends);
    let NFSCodeEnv::EXTENDS {
        baseClass: __pa0,
        redeclareModifiers: __pa1,
        index: __pa2,
        info: __pa3,
    } = &**__arc4;
    bc = metamodelica::Own::own(__pa0);
    redeclares = metamodelica::Own::own(__pa1);
    index = metamodelica::Own::own(__pa2);
    info = metamodelica::Own::own(__pa3);
    redeclares = List::filter1(
        &redeclares,
        &move |__a0: metamodelica::Ref<NFSCodeEnv::Redeclaration>,
               __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| {
            removeUnusedRedeclares3(__a0, &__a1)
        },
        inEnv,
    );
    outExtends = metamodelica::Ref::new(NFSCodeEnv::Extends {
        baseClass: bc,
        redeclareModifiers: redeclares,
        index: index,
        info: info,
    });
    outExtends
}

fn removeUnusedRedeclares3(
    mut inRedeclare: metamodelica::Ref<NFSCodeEnv::Redeclaration>,
    mut inEnv: &Env,
) -> Result<()> {
    let mut name: ArcStr;
    let mut item: Item;
    (name, _) = NFSCodeEnv::getRedeclarationNameInfo(inRedeclare)?;
    (item, _, _) = NFSCodeLookup::lookupSimpleName(&name, inEnv)?;
    let true = (NFSCodeEnv::isItemUsed(&item)) else {
        return Err("pattern mismatch");
    };
    Ok(())
}
