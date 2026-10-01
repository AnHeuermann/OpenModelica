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

use crate::NFInstTypes;
use crate::NFSCodeCheck;
use crate::NFSCodeEnv;
use crate::NFSCodeEnv::EnvTree;
use crate::NFSCodeLookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Env = metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;

pub type Item = metamodelica::Ref<NFSCodeEnv::Item>;

pub type Extends = metamodelica::Ref<NFSCodeEnv::Extends>;

pub type Prefix = metamodelica::Ref<NFInstPrefix::Prefix>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Replacement {
    /// an item got replaced
    REPLACED {
        name: ArcStr,
        old: Item,
        new: Item,
        env: Env,
    },
    /// the redeclares got pushed into the extends of the base classes
    PUSHED {
        name: ArcStr,
        redeclaredItem: Item,
        baseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
        old: metamodelica::Ref<NFSCodeEnv::ExtendsTable>,
        new: metamodelica::Ref<NFSCodeEnv::ExtendsTable>,
        env: Env,
    },
}
impl metamodelica::gc::MMTrace for Replacement {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Replacement::REPLACED { name, old, new, env } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(old, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(new, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                Ok(())
            }
            Replacement::PUSHED {
                name,
                redeclaredItem,
                baseClasses,
                old,
                new,
                env,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(redeclaredItem, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseClasses, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(old, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(new, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(env, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::Replacement::{PUSHED, REPLACED};

pub type Replacements = metamodelica::List<Replacement>;

pub(crate) static emptyReplacements: std::sync::LazyLock<metamodelica::List<Replacement>> =
    std::sync::LazyLock::new(|| metamodelica::nil());

pub(crate) fn addElementRedeclarationsToEnv(
    mut inRedeclares: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inEnv: Env,
) -> Result<Env> {
    let mut outEnv: Env;
    outEnv = List::fold(
        inRedeclares,
        &move |__a0: metamodelica::Ref<SCode::Element>,
               __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| {
            addElementRedeclarationsToEnv2(&__a0, __a1)
        },
        inEnv,
    )?;
    Ok(outEnv)
}

fn addElementRedeclarationsToEnv2(mut inRedeclare: &metamodelica::Ref<SCode::Element>, mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env;
    outEnv = 'mc: {
        let __mc_input = &*inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut name: ArcStr;
                    let mut info: SourceInfo;
                    let mut env_path: metamodelica::Ref<Absyn::Path>;
                    let mut ext_pathl: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut env: Env;
                    let mut item: Item;
                    name = SCodeUtil::elementName(inRedeclare)?;
                    info = SCodeUtil::elementInfo(inRedeclare);
                    ext_pathl = lookupElementRedeclaration(name.clone(), inEnv.clone(), &info)?;
                    env_path = NFSCodeEnv::getEnvPath(&inEnv)?;
                    item = metamodelica::Ref::new(NFSCodeEnv::Item::ALIAS { name: name.clone(), path: Some(env_path.clone()), info: info.clone() });
                    env = addRedeclareToEnvExtendsTable(&item, &ext_pathl, &inEnv, &info)?;
                    Ok(env.clone())
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenRedeclare.addElementRedeclarationsToEnv failed for ")); __mm_s.push_str(&*SCodeUtil::elementName(inRedeclare)?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inEnv)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEnv)
}

fn lookupElementRedeclaration(
    mut inName: ArcStr,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    outPaths = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            paths = NFSCodeLookup::lookupBaseClasses(inName.clone(), inEnv.clone())?;
            Ok(paths.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addSourceMessage(
                &(Error::REDECLARE_NONEXISTING_ELEMENT.clone()),
                list![inName.clone()],
                inInfo,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outPaths)
}

fn addRedeclareToEnvExtendsTable(
    mut inRedeclaredElement: &Item,
    mut inBaseClasses: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<Env> {
    let mut outEnv: Env;
    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let mut re: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cei: Option<metamodelica::Ref<SCode::Element>>;
    let __arc3 = NFSCodeEnv::getEnvExtendsTable(inEnv)?;
    let NFSCodeEnv::EXTENDS_TABLE {
        baseClasses: __pa0,
        redeclaredElements: __pa1,
        classExtendsInfo: __pa2,
    } = &*__arc3;
    bcl = metamodelica::Own::own(__pa0);
    re = metamodelica::Own::own(__pa1);
    cei = metamodelica::Own::own(__pa2);
    bcl = addRedeclareToEnvExtendsTable2(inRedeclaredElement, inBaseClasses, &bcl)?;
    outEnv = NFSCodeEnv::setEnvExtendsTable(
        metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
            baseClasses: bcl,
            redeclaredElements: re,
            classExtendsInfo: cei,
        }),
        inEnv,
    )?;
    Ok(outEnv)
}

fn addRedeclareToEnvExtendsTable2(
    mut inRedeclaredElement: &Item,
    mut inBaseClasses: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inExtends: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    outExtends = 'mc: {
        let __mc_input = (&**inBaseClasses, &**inExtends);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bc1, tail: rest_bc }, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Extends { baseClass: bc2, redeclareModifiers: el, index, info }, tail: exl }) => {
                    let mut ex: Extends;
                    let mut redecl: metamodelica::Ref<NFSCodeEnv::Redeclaration>;
                    let mut exl = (*exl).clone();
                    let true = (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&bc1), metamodelica::AsArg::as_arg(&bc2))) else { return Err("pattern mismatch") };
                    redecl = metamodelica::Ref::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: inRedeclaredElement.clone() });
                    NFSCodeCheck::checkDuplicateRedeclarations(redecl.clone(), metamodelica::AsArg::as_arg(&el))?;
                    ex = metamodelica::Ref::new(NFSCodeEnv::Extends { baseClass: bc2.clone(), redeclareModifiers: metamodelica::cons(redecl.clone(), el.clone()), index: index.clone(), info: info.clone() });
                    exl = addRedeclareToEnvExtendsTable2(inRedeclaredElement, metamodelica::AsArg::as_arg(&rest_bc), metamodelica::AsArg::as_arg(&exl))?;
                    Ok(metamodelica::cons(ex.clone(), exl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inExtends.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: ex, tail: exl }) => {
                    let mut exl = (*exl).clone();
                    exl = addRedeclareToEnvExtendsTable2(inRedeclaredElement, inBaseClasses, metamodelica::AsArg::as_arg(&exl))?;
                    Ok(metamodelica::cons(ex.clone(), exl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExtends)
}

pub(crate) fn processRedeclare(
    mut inRedeclare: metamodelica::Ref<NFSCodeEnv::Redeclaration>,
    mut inEnv: Env,
    mut inPrefix: &metamodelica::Ref<NFInstPrefix::Prefix>,
) -> Result<metamodelica::Ref<NFSCodeEnv::Redeclaration>> {
    let mut outRedeclare: metamodelica::Ref<NFSCodeEnv::Redeclaration>;
    outRedeclare = 'mc: {
        let __mc_input = &*inRedeclare;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: el @ Deref @ SCode::Element::CLASS { .. } } => {
                    let mut el_item: Item;
                    let mut redecl_item: Item;
                    let mut cls_env: Env;
                    cls_env = NFSCodeEnv::makeClassEnvironment(metamodelica::AsArg::as_arg(&el), true)?;
                    el_item = NFSCodeEnv::newClassItem(el.clone(), cls_env.clone(), crate::NFSCodeEnv::ClassType::USERDEFINED);
                    redecl_item = metamodelica::Ref::new(NFSCodeEnv::Item::REDECLARED_ITEM { item: el_item.clone(), declaredEnv: inEnv.clone() });
                    Ok(metamodelica::Ref::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: redecl_item.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: el @ Deref @ SCode::Element::COMPONENT { .. } } => {
                    let mut el_item: Item;
                    let mut redecl_item: Item;
                    el_item = NFSCodeEnv::newVarItem(el.clone(), true);
                    redecl_item = metamodelica::Ref::new(NFSCodeEnv::Item::REDECLARED_ITEM { item: el_item.clone(), declaredEnv: inEnv.clone() });
                    Ok(metamodelica::Ref::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: redecl_item.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { .. } => {
                    Ok(inRedeclare.clone())
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFSCodeFlattenRedeclare.processRedeclare failed on ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(NFSCodeEnv::getRedeclarationElement(inRedeclare.clone())?, SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*AbsynUtil::pathString(NFSCodeEnv::getEnvPath(&inEnv)?, literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRedeclare)
}

pub(crate) fn replaceRedeclares(
    mut inRedeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
    mut inClassItem: Item,
    mut inClassEnv: Env,
    mut inElementEnv: Env,
    mut inReplaceRedeclares: NFSCodeLookup::RedeclareReplaceStrategy,
) -> (
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
) {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outEnv) = 'mc: {
        let __mc_input = inReplaceRedeclares;
        if let Ok(__v) = (|| -> Result<_> {
            let NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((Some(inClassItem.clone()), Some(inClassEnv.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let NFSCodeLookup::RedeclareReplaceStrategy::INSERT_REDECLARES { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut item: Item;
            let mut env: Env;
            (item, env, _) = replaceRedeclaredElementsInEnv(
                inRedeclares.clone(),
                inClassItem.clone(),
                inClassEnv.clone(),
                inElementEnv.clone(),
                NFInstPrefix::emptyPrefix().clone(),
            )?;
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

pub(crate) fn replaceRedeclaredElementsInEnv(
    mut inRedeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
    mut inItem: Item,
    mut inTypeEnv: Env,
    mut inElementEnv: Env,
    mut inPrefix: metamodelica::Ref<NFInstPrefix::Prefix>,
) -> Result<(Item, Env, Replacements)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    let mut outReplacements: Replacements;
    (outItem, outEnv, outReplacements) = 'mc: {
        let __mc_input = (&*inRedeclares, &*inItem);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((inItem.clone(), inTypeEnv.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ NFSCodeEnv::Item::CLASS { cls, env: Deref @ metamodelica::ListNode::Cons { head: item_env, tail: Deref @ metamodelica::ListNode::Nil }, classType: cls_ty }) => {
                    let mut env: Env;
                    let mut redecls: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
                    let mut repl: Replacements;
                    let mut item_env = (*item_env).clone();
                    env = NFSCodeEnv::enterFrame(item_env.clone(), inTypeEnv.clone());
                    redecls = List::map2(inRedeclares.clone(), &move |__a0: metamodelica::Ref<NFSCodeEnv::Redeclaration>, __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, __a2: metamodelica::Ref<NFInstPrefix::Prefix>| processRedeclare(__a0, __a1, &__a2), inElementEnv.clone(), inPrefix.clone())?;
                    (env, repl) = List::fold(&redecls, &move |__a0: metamodelica::Ref<NFSCodeEnv::Redeclaration>, __a1: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, metamodelica::List<Replacement>)| replaceRedeclaredElementInEnv(&__a0, __a1), (env.clone(), emptyReplacements.clone()))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(env.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    item_env = metamodelica::Own::own(__pa0);
                    env = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: list![item_env.clone()], classType: cls_ty.clone() }), env.clone(), repl.clone()))
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
                    Debug::trace(literal!("- NFSCodeFlattenRedeclare.replaceRedeclaredElementsInEnv failed for:\n\t"))?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("redeclares: ")); __mm_s.push_str(&*stringDelimitList(List::map(inRedeclares.clone(), &NFSCodeEnv::printRedeclarationStr)?, literal!("\n---------\n"))); __mm_s.push_str(&*literal!("\n\titem: ")); __mm_s.push_str(&*NFSCodeEnv::itemStr(&inItem)); __mm_s.push_str(&*literal!("\n\tin scope:")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&inElementEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outEnv, outReplacements))
}

pub(crate) fn extractRedeclaresFromModifier(
    mut inMod: &metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>> {
    let mut outRedeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
    outRedeclares = (match &**inMod {
        SCode::Mod::MOD {
            subModLst: sub_mods, ..
        } => {
            let mut redeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
            redeclares = List::fold(
                sub_mods,
                &move |__a0: metamodelica::Ref<SCode::SubMod>,
                       __a1: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>| {
                    extractRedeclareFromSubMod(&__a0, __a1)
                },
                metamodelica::nil(),
            )?;
            redeclares
        }
        _ => metamodelica::nil(),
    });
    Ok(outRedeclares)
}

fn extractRedeclareFromSubMod(
    mut inMod: &metamodelica::Ref<SCode::SubMod>,
    mut inRedeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>> {
    let mut outRedeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>;
    outRedeclares = (::match_deref::match_deref! { match inMod {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::REDECL { element: el, .. }, .. } => {
            let mut redecl: metamodelica::Ref<NFSCodeEnv::Redeclaration>;
            redecl = metamodelica::Ref::new(NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: el.clone() });
            NFSCodeCheck::checkDuplicateRedeclarations(redecl.clone(), &inRedeclares)?;
            metamodelica::cons(redecl, inRedeclares)
        },
        _ => {
            inRedeclares
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outRedeclares)
}

fn replaceRedeclaredElementInEnv(
    mut inRedeclare: &metamodelica::Ref<NFSCodeEnv::Redeclaration>,
    mut inEnv: (
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    metamodelica::List<Replacement>,
)> {
    let mut outEnv: (
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    );
    outEnv = 'mc: {
        let __mc_input = &**inRedeclare;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item } => {
                    let mut name: ArcStr;
                    let mut envRpl: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, metamodelica::List<Replacement>);
                    name = NFSCodeEnv::getItemName(metamodelica::AsArg::as_arg(&item))?;
                    envRpl = pushRedeclareIntoExtendsNoFail(name.clone(), item.clone(), inEnv.clone());
                    Ok(replaceElementInScope(name.clone(), item.clone(), &envRpl)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item } => {
                    let mut name: ArcStr;
                    let mut bcl: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    name = NFSCodeEnv::getItemName(metamodelica::AsArg::as_arg(&item))?;
                    bcl = NFSCodeLookup::lookupBaseClasses(name.clone(), Util::tuple21(inEnv.clone()))?;
                    Ok(pushRedeclareIntoExtends(name.clone(), item.clone(), bcl.clone(), &inEnv)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item } => {
                    let mut name: ArcStr;
                    let mut scope_name: ArcStr;
                    let mut info: SourceInfo;
                    scope_name = NFSCodeEnv::getScopeName(&(Util::tuple21(inEnv.clone())))?;
                    name = NFSCodeEnv::getItemName(metamodelica::AsArg::as_arg(&item))?;
                    info = NFSCodeEnv::getItemInfo(metamodelica::AsArg::as_arg(&item))?;
                    Error::addSourceMessage(&(Error::MISSING_MODIFIED_ELEMENT.clone()), list![name.clone(), scope_name.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEnv)
}

fn pushRedeclareIntoExtendsNoFail(
    mut inName: ArcStr,
    mut inRedeclare: Item,
    mut inEnv: (
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    ),
) -> (
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    metamodelica::List<Replacement>,
) {
    let mut outEnv: (
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    );
    outEnv = 'mc: {
        let __mc_input = &inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut bcl: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut envRpl: (metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>, metamodelica::List<Replacement>);
                    bcl = NFSCodeLookup::lookupBaseClasses(inName.clone(), Util::tuple21(inEnv.clone()))?;
                    envRpl = pushRedeclareIntoExtends(inName.clone(), inRedeclare.clone(), bcl.clone(), &inEnv)?;
                    Ok(envRpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inEnv.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outEnv
}

fn pushRedeclareIntoExtends(
    mut inName: ArcStr,
    mut inRedeclare: Item,
    mut inBaseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inEnv: &(
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    metamodelica::List<Replacement>,
)> {
    let mut outEnv: (
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    );
    let mut exts: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let mut re: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cei: Option<metamodelica::Ref<SCode::Element>>;
    let mut etNew: metamodelica::Ref<NFSCodeEnv::ExtendsTable>;
    let mut etOld: metamodelica::Ref<NFSCodeEnv::ExtendsTable>;
    let mut env: Env;
    let mut repl: Replacements;
    (env, repl) = inEnv.clone();
    let (__pa3, __pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(env.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: __pa3 @ Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: __pa2 }, .. }, tail: _ } => (__pa3.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exts = metamodelica::Own::own(__pa0);
    re = metamodelica::Own::own(__pa1);
    cei = metamodelica::Own::own(__pa2);
    etOld = metamodelica::Own::own(__pa3);
    exts = pushRedeclareIntoExtends2(&inName, &inRedeclare, inBaseClasses.clone(), &exts)?;
    etNew = metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
        baseClasses: exts,
        redeclaredElements: re,
        classExtendsInfo: cei,
    });
    env = NFSCodeEnv::setEnvExtendsTable(etNew.clone(), &env)?;
    repl = metamodelica::cons(
        Replacement::PUSHED {
            name: inName,
            redeclaredItem: inRedeclare,
            baseClasses: inBaseClasses,
            old: etOld,
            new: etNew,
            env: env.clone(),
        },
        repl,
    );
    outEnv = (env, repl);
    Ok(outEnv)
}

fn pushRedeclareIntoExtends2(
    mut inName: &ArcStr,
    mut inRedeclare: &Item,
    mut inBaseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inExtends: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    outExtends = (::match_deref::match_deref! { match &((inBaseClasses.clone(), inExtends.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: bc1, tail: rest_bc }, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Extends { baseClass: bc2, redeclareModifiers: redecls, index, info }, tail: rest_exts }) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&bc1), metamodelica::AsArg::as_arg(&bc2))) => {
            let mut redecls = (*redecls).clone();
            let mut rest_exts = (*rest_exts).clone();
            redecls = pushRedeclareIntoExtends3(inRedeclare, inName, metamodelica::AsArg::as_arg(&redecls), metamodelica::nil())?;
            rest_exts = pushRedeclareIntoExtends2(inName, inRedeclare, rest_bc.clone(), metamodelica::AsArg::as_arg(&rest_exts))?;
            metamodelica::cons(metamodelica::Ref::new(NFSCodeEnv::Extends { baseClass: bc2.clone(), redeclareModifiers: redecls.clone(), index: index.clone(), info: info.clone() }), rest_exts.clone())
        },
        (rest_bc, Deref @ metamodelica::ListNode::Cons { head: ext, tail: rest_exts }) => {
            let mut rest_exts = (*rest_exts).clone();
            rest_exts = pushRedeclareIntoExtends2(inName, inRedeclare, rest_bc.clone(), metamodelica::AsArg::as_arg(&rest_exts))?;
            metamodelica::cons(ext.clone(), rest_exts.clone())
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            inExtends.clone()
        },
        (_, Deref @ metamodelica::ListNode::Nil) => {
            let mut bc_strl: metamodelica::List<ArcStr>;
            let mut bcl_str: ArcStr;
            let mut err_msg: ArcStr;
            bc_strl = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut p in (inBaseClasses).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            bcl_str = stringDelimitList(bc_strl, literal!(", "));
            err_msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSCodeFlattenRedeclare.pushRedeclareIntoExtends2 couldn't find the base classes {")); __mm_s.push_str(&*bcl_str); __mm_s.push_str(&*literal!("} for ")); __mm_s.push_str(&*inName); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![err_msg])?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExtends)
}

fn pushRedeclareIntoExtends3<'__b>(
    mut inRedeclare: &'__b Item,
    mut inName: &'__b ArcStr,
    mut inRedeclares: &'__b metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
    mut inOutRedeclares: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inRedeclares {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: item }, tail: rest_redecls } if (stringEqual(&(NFSCodeEnv::getItemName(metamodelica::AsArg::as_arg(&item))?), &inName)) => {
                return Ok(List::append_reverse(&inOutRedeclares, metamodelica::cons(metamodelica::Ref::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: inRedeclare.clone() }), rest_redecls.clone())))
            },
            Deref @ metamodelica::ListNode::Cons { head: redecl, tail: rest_redecls } => {
                { (inRedeclare, inName, inRedeclares, inOutRedeclares) = (inRedeclare, inName, rest_redecls, metamodelica::cons(redecl.clone(), inOutRedeclares)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::cons(metamodelica::Ref::new(NFSCodeEnv::Redeclaration::PROCESSED_MODIFIER { modifier: inRedeclare.clone() }), inOutRedeclares).reverse())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn replaceElementInScope(
    mut inElementName: ArcStr,
    mut inElement: Item,
    mut inEnv: &(
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    metamodelica::List<Replacement>,
)> {
    let mut outEnv: (
        metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
        metamodelica::List<Replacement>,
    );
    outEnv = (::match_deref::match_deref! { match &(inEnv) {
        (env @ Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ }, repl) => {
            let mut old_item: Item;
            let mut new_item: Item;
            let mut env = (*env).clone();
            let mut tree = (*tree).clone();
            let mut repl = (*repl).clone();
            old_item = NFSCodeEnv::EnvTree::get(metamodelica::AsArg::as_arg(&tree), inElementName.clone())?;
            new_item = propagateItemPrefixes(old_item.clone(), inElement)?;
            new_item = NFSCodeEnv::linkItemUsage(&old_item, &new_item);
            tree = NFSCodeEnv::EnvTree::add(tree.clone(), &inElementName, &new_item, &fnptr!(NFSCodeEnv::EnvTree::addConflictReplace, metamodelica::Ref<NFSCodeEnv::Item>, metamodelica::Ref<NFSCodeEnv::Item>, ArcStr))?;
            env = NFSCodeEnv::setEnvClsAndVars(tree.clone(), metamodelica::AsArg::as_arg(&env))?;
            repl = metamodelica::cons(Replacement::REPLACED { name: inElementName, old: old_item, new: new_item, env: env.clone() }, repl.clone());
            (env.clone(), repl.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outEnv)
}

fn propagateItemPrefixes(mut inOriginalItem: Item, mut inNewItem: Item) -> Result<Item> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inOriginalItem.clone(), inNewItem.clone())) {
            (Deref @ NFSCodeEnv::Item::VAR { var: el1, .. }, Deref @ NFSCodeEnv::Item::VAR { var: el2, isUsed: iu2 }) => {
                let mut el2 = (*el2).clone();
                el2 = propagateAttributesVar(metamodelica::AsArg::as_arg(&el1), metamodelica::AsArg::as_arg(&el2))?;
                return Ok(metamodelica::Ref::new(NFSCodeEnv::Item::VAR { var: el2.clone(), isUsed: iu2.clone() }))
            },
            (Deref @ NFSCodeEnv::Item::CLASS { cls: el1, .. }, Deref @ NFSCodeEnv::Item::CLASS { cls: el2, env: env2, classType: ty2 }) => {
                let mut el2 = (*el2).clone();
                el2 = propagateAttributesClass(metamodelica::AsArg::as_arg(&el1), metamodelica::AsArg::as_arg(&el2))?;
                return Ok(metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: el2.clone(), env: env2.clone(), classType: ty2.clone() }))
            },
            (Deref @ NFSCodeEnv::Item::ALIAS { .. }, _) => {
                return Ok(inNewItem)
            },
            (_, Deref @ NFSCodeEnv::Item::ALIAS { .. }) => {
                return Ok(inNewItem)
            },
            (Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, .. }, _) => {
                { (inOriginalItem, inNewItem) = (item.clone(), inNewItem); continue '__tco; }
            },
            (_, Deref @ NFSCodeEnv::Item::REDECLARED_ITEM { item, declaredEnv: env1 }) => {
                let mut item = (*item).clone();
                item = propagateItemPrefixes(inOriginalItem, item.clone())?;
                return Ok(metamodelica::Ref::new(NFSCodeEnv::Item::REDECLARED_ITEM { item: item.clone(), declaredEnv: env1.clone() }))
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("NFSCodeFlattenRedeclare.propagateAttributes failed on unknown item.")])?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn propagateAttributesVar(
    mut inOriginalVar: &metamodelica::Ref<SCode::Element>,
    mut inNewVar: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outNewVar: metamodelica::Ref<SCode::Element>;
    let mut name: ArcStr;
    let mut pref1: metamodelica::Ref<SCode::Prefixes>;
    let mut pref2: metamodelica::Ref<SCode::Prefixes>;
    let mut attr1: SCode::Attributes;
    let mut attr2: SCode::Attributes;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut cond: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut info: SourceInfo;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inOriginalVar)) {
        Deref @ SCode::Element::COMPONENT { prefixes: __pa0, attributes: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    pref1 = metamodelica::Own::own(__pa0);
    attr1 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &((*inNewVar)) {
        Deref @ SCode::Element::COMPONENT { name: __pa2, prefixes: __pa3, attributes: __pa4, typeSpec: __pa5, modifications: __pa6, comment: __pa7, condition: __pa8, info: __pa9 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa2);
    pref2 = metamodelica::Own::own(__pa3);
    attr2 = metamodelica::Own::own(__pa4);
    ty = metamodelica::Own::own(__pa5);
    r#mod = metamodelica::Own::own(__pa6);
    cmt = metamodelica::Own::own(__pa7);
    cond = metamodelica::Own::own(__pa8);
    info = metamodelica::Own::own(__pa9);
    pref2 = propagatePrefixes(&pref1, &pref2);
    attr2 = propagateAttributes(attr1, attr2);
    outNewVar = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: name,
        prefixes: pref2,
        attributes: attr2,
        typeSpec: ty,
        modifications: r#mod,
        comment: cmt,
        condition: cond,
        info: info,
    });
    Ok(outNewVar)
}

pub(crate) fn propagateAttributesClass(
    mut inOriginalClass: &metamodelica::Ref<SCode::Element>,
    mut inNewClass: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outNewClass: metamodelica::Ref<SCode::Element>;
    let mut name: ArcStr;
    let mut pref1: metamodelica::Ref<SCode::Prefixes>;
    let mut pref2: metamodelica::Ref<SCode::Prefixes>;
    let mut ep: SCode::Encapsulated;
    let mut pp: SCode::Partial;
    let mut res: SCode::Restriction;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let mut info: SourceInfo;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let __pa0 = ::match_deref::match_deref! { match &((*inOriginalClass)) {
        Deref @ SCode::Element::CLASS { prefixes: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    pref1 = metamodelica::Own::own(__pa0);
    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &((*inNewClass)) {
        Deref @ SCode::Element::CLASS { name: __pa1, prefixes: __pa2, encapsulatedPrefix: __pa3, partialPrefix: __pa4, restriction: __pa5, classDef: __pa6, cmt: __pa7, info: __pa8 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa1);
    pref2 = metamodelica::Own::own(__pa2);
    ep = metamodelica::Own::own(__pa3);
    pp = metamodelica::Own::own(__pa4);
    res = metamodelica::Own::own(__pa5);
    cdef = metamodelica::Own::own(__pa6);
    cmt = metamodelica::Own::own(__pa7);
    info = metamodelica::Own::own(__pa8);
    pref2 = propagatePrefixes(&pref1, &pref2);
    outNewClass = metamodelica::Ref::new(SCode::Element::CLASS {
        name: name,
        prefixes: pref2,
        encapsulatedPrefix: ep,
        partialPrefix: pp,
        restriction: res,
        classDef: cdef,
        cmt: cmt,
        info: info,
    });
    Ok(outNewClass)
}

fn propagatePrefixes(
    mut inOriginalPrefixes: &metamodelica::Ref<SCode::Prefixes>,
    mut inNewPrefixes: &metamodelica::Ref<SCode::Prefixes>,
) -> metamodelica::Ref<SCode::Prefixes> {
    let mut outNewPrefixes: metamodelica::Ref<SCode::Prefixes>;
    let mut vis1: SCode::Visibility;
    let mut vis2: SCode::Visibility;
    let mut io1: Absyn::InnerOuter;
    let mut io2: Absyn::InnerOuter;
    let mut rdp: SCode::Redeclare;
    let mut fp: SCode::Final;
    let mut rpp: metamodelica::Ref<SCode::Replaceable>;
    let __arc2 = &(*inOriginalPrefixes);
    let SCode::PREFIXES {
        visibility: __pa0,
        innerOuter: __pa1,
        ..
    } = &**__arc2;
    vis1 = metamodelica::Own::own(__pa0);
    io1 = metamodelica::Own::own(__pa1);
    let __arc8 = &(*inNewPrefixes);
    let SCode::PREFIXES {
        visibility: __pa3,
        redeclarePrefix: __pa4,
        finalPrefix: __pa5,
        innerOuter: __pa6,
        replaceablePrefix: __pa7,
    } = &**__arc8;
    vis2 = metamodelica::Own::own(__pa3);
    rdp = metamodelica::Own::own(__pa4);
    fp = metamodelica::Own::own(__pa5);
    io2 = metamodelica::Own::own(__pa6);
    rpp = metamodelica::Own::own(__pa7);
    io2 = propagatePrefixInnerOuter(io1, io2);
    outNewPrefixes = metamodelica::Ref::new(SCode::Prefixes {
        visibility: vis2,
        redeclarePrefix: rdp,
        finalPrefix: fp,
        innerOuter: io2,
        replaceablePrefix: rpp,
    });
    outNewPrefixes
}

fn propagatePrefixInnerOuter(mut inOriginalIO: Absyn::InnerOuter, mut inIO: Absyn::InnerOuter) -> Absyn::InnerOuter {
    let mut outIO: Absyn::InnerOuter;
    outIO = (match inIO {
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => inOriginalIO,
        _ => inIO,
    });
    outIO
}

fn propagateAttributes(
    mut inOriginalAttributes: SCode::Attributes,
    mut inNewAttributes: SCode::Attributes,
) -> SCode::Attributes {
    let mut outNewAttributes: SCode::Attributes;
    let mut dims1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut ct1: SCode::ConnectorType;
    let mut ct2: SCode::ConnectorType;
    let mut prl1: SCode::Parallelism;
    let mut prl2: SCode::Parallelism;
    let mut var1: SCode::Variability;
    let mut var2: SCode::Variability;
    let mut dir1: Absyn::Direction;
    let mut dir2: Absyn::Direction;
    let mut isf1: Absyn::IsField;
    let mut isf2: Absyn::IsField;
    let SCode::ATTR {
        arrayDims: __pa0,
        connectorType: __pa1,
        parallelism: __pa2,
        variability: __pa3,
        direction: __pa4,
        isField: __pa5,
    } = inOriginalAttributes;
    dims1 = metamodelica::Own::own(__pa0);
    ct1 = metamodelica::Own::own(__pa1);
    prl1 = metamodelica::Own::own(__pa2);
    var1 = metamodelica::Own::own(__pa3);
    dir1 = metamodelica::Own::own(__pa4);
    isf1 = metamodelica::Own::own(__pa5);
    let SCode::ATTR {
        arrayDims: __pa6,
        connectorType: __pa7,
        parallelism: __pa8,
        variability: __pa9,
        direction: __pa10,
        isField: __pa11,
    } = inNewAttributes;
    dims2 = metamodelica::Own::own(__pa6);
    ct2 = metamodelica::Own::own(__pa7);
    prl2 = metamodelica::Own::own(__pa8);
    var2 = metamodelica::Own::own(__pa9);
    dir2 = metamodelica::Own::own(__pa10);
    isf2 = metamodelica::Own::own(__pa11);
    dims2 = propagateArrayDimensions(dims1, dims2);
    ct2 = propagateConnectorType(ct1, ct2);
    prl2 = propagateParallelism(prl1, prl2);
    var2 = propagateVariability(var1, var2);
    dir2 = propagateDirection(dir1, dir2);
    isf2 = propagateIsField(isf1, isf2);
    outNewAttributes = SCode::Attributes {
        arrayDims: dims2,
        connectorType: ct2,
        parallelism: prl2,
        variability: var2,
        direction: dir2,
        isField: isf2,
    };
    outNewAttributes
}

fn propagateArrayDimensions(
    mut inOriginalDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inNewDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outNewDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outNewDims = (::match_deref::match_deref! { match &(inNewDims.clone()) {
        Deref @ metamodelica::ListNode::Nil => inOriginalDims,
        _ => inNewDims,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNewDims
}

fn propagateConnectorType(
    mut inOriginalConnectorType: SCode::ConnectorType,
    mut inNewConnectorType: SCode::ConnectorType,
) -> SCode::ConnectorType {
    let mut outNewConnectorType: SCode::ConnectorType;
    outNewConnectorType = (match inNewConnectorType {
        SCode::ConnectorType::POTENTIAL { .. } => inOriginalConnectorType,
        _ => inNewConnectorType,
    });
    outNewConnectorType
}

fn propagateParallelism(
    mut inOriginalParallelism: SCode::Parallelism,
    mut inNewParallelism: SCode::Parallelism,
) -> SCode::Parallelism {
    let mut outNewParallelism: SCode::Parallelism;
    outNewParallelism = (match inNewParallelism {
        SCode::Parallelism::NON_PARALLEL { .. } => inOriginalParallelism,
        _ => inNewParallelism,
    });
    outNewParallelism
}

fn propagateVariability(
    mut inOriginalVariability: SCode::Variability,
    mut inNewVariability: SCode::Variability,
) -> SCode::Variability {
    let mut outNewVariability: SCode::Variability;
    outNewVariability = (match inNewVariability {
        SCode::Variability::VAR { .. } => inOriginalVariability,
        _ => inNewVariability,
    });
    outNewVariability
}

fn propagateDirection(
    mut inOriginalDirection: Absyn::Direction,
    mut inNewDirection: Absyn::Direction,
) -> Absyn::Direction {
    let mut outNewDirection: Absyn::Direction;
    outNewDirection = (match inNewDirection {
        Absyn::Direction::BIDIR { .. } => inOriginalDirection,
        _ => inNewDirection,
    });
    outNewDirection
}

fn propagateIsField(mut inOriginalIsField: Absyn::IsField, mut inNewIsField: Absyn::IsField) -> Absyn::IsField {
    let mut outNewIsField: Absyn::IsField;
    outNewIsField = (match inNewIsField {
        Absyn::IsField::NONFIELD { .. } => inOriginalIsField,
        _ => inNewIsField,
    });
    outNewIsField
}

fn traceReplaceElementInScope(
    mut inElementName: &ArcStr,
    mut inOldItem: &Item,
    mut inNewItem: &Item,
    mut inEnv: &Env,
) -> () {
    let () = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("replacing element: ")); __mm_s.push_str(&*inElementName); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv)); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Old Element:")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inOldItem)); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&(NFSCodeEnv::getItemEnvNoFail(inOldItem)?))); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("New Element:")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inNewItem)); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(&(NFSCodeEnv::getItemEnvNoFail(inNewItem)?))); __mm_s.push_str(&*literal!("\n===============\n")); ArcStr::from(__mm_s) });
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
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("traceReplaceElementInScope failed on element: ")); __mm_s.push_str(&*inElementName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
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

fn tracePushRedeclareIntoExtends(
    mut inName: &ArcStr,
    mut inRedeclare: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inBaseClasses: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inEnv: &Env,
    mut inEtNew: &metamodelica::Ref<NFSCodeEnv::ExtendsTable>,
    mut inEtOld: &metamodelica::Ref<NFSCodeEnv::ExtendsTable>,
) -> () {
    let () = 'mc: {
        let __mc_input = &**inEtOld;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("pushing: ")); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!(" redeclare: ")); __mm_s.push_str(&*NFSCodeEnv::itemStr(inRedeclare)); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) });
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("into baseclases: ")); __mm_s.push_str(&*stringDelimitList(({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut p in (inBaseClasses.clone()).into_iter().cloned() {
                            let __x = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }), literal!(", "))); __mm_s.push_str(&*literal!("\n\t")); ArcStr::from(__mm_s) });
                            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("called from env: ")); __mm_s.push_str(&*NFSCodeEnv::getEnvName(inEnv)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            metamodelica::print(literal!("-----------------\n"));
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
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tracePushRedeclareIntoExtends failed on element: ")); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
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
