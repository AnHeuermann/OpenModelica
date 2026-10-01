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

use crate::NFInstDump;
use crate::NFInstTypes;
use crate::NFSCodeEnv;
use crate::NFSCodeEnv::EnvTree;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub(crate) fn checkRecursiveShortDefinition(
    mut inTypeSpec: metamodelica::Ref<Absyn::TypeSpec>,
    mut inTypeName: ArcStr,
    mut inTypeEnv: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inTypeEnv;
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
                Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                    let mut ts_path: metamodelica::Ref<Absyn::Path>;
                    let mut ty_path: metamodelica::Ref<Absyn::Path>;
                    ts_path = AbsynUtil::typeSpecPath(&inTypeSpec);
                    ty_path = NFSCodeEnv::getEnvPath(inTypeEnv)?;
                    let false = (isSelfReference(inTypeName.clone(), ty_path.clone(), ts_path.clone())?) else { return Err("pattern mismatch") };
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
                    let mut ty: ArcStr;
                    ty = Dump::unparseTypeSpec(inTypeSpec.clone())?;
                    Error::addSourceMessage(&(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone()), list![inTypeName.clone(), ty.clone()], inInfo)?;
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

fn isSelfReference(
    mut inTypeName: ArcStr,
    mut inTypePath: metamodelica::Ref<Absyn::Path>,
    mut inReferencedName: metamodelica::Ref<Absyn::Path>,
) -> Result<bool> {
    let mut selfRef: bool;
    selfRef = (::match_deref::match_deref! { match &(inReferencedName) {
        Deref @ Absyn::Path::FULLYQUALIFIED { path: p2 } => {
            let mut p1 = inTypePath.clone();
            AbsynUtil::pathEqual(&(AbsynUtil::joinPaths(p1, metamodelica::Ref::new(Absyn::Path::IDENT { name: inTypeName }))?), metamodelica::AsArg::as_arg(&p2))
        },
        p2 => {
            stringEqual(&(AbsynUtil::pathLastIdent(&inTypePath)), &(AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&p2))))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(selfRef)
}

pub(crate) fn checkClassExtendsReplaceability(
    mut inBaseClass: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inOriginInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inBaseClass {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { .. }, .. }, .. }, .. } => (),
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn checkRedeclareModifier(
    mut inModifier: &metamodelica::Ref<NFSCodeEnv::Redeclaration>,
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
    mut inEnv: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inModifier {
        Deref @ NFSCodeEnv::Redeclaration::RAW_MODIFIER { modifier: e @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } } => {
            checkRedeclareModifier2(metamodelica::AsArg::as_arg(&e), inBaseClass, inEnv)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkRedeclareModifier2(
    mut inModifier: &metamodelica::Ref<SCode::Element>,
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
    mut inEnv: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inModifier;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, .. }, .. } => {
                    let mut ty_path: metamodelica::Ref<Absyn::Path>;
                    ty_path = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&ty));
                    let false = (isSelfReference(name.clone(), inBaseClass.clone(), ty_path.clone())?) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: ty, .. }, info, .. } => {
                    let mut ty_str: ArcStr;
                    ty_str = Dump::unparseTypeSpec(ty.clone())?;
                    Error::addSourceMessage(&(Error::RECURSIVE_SHORT_CLASS_DEFINITION.clone()), list![name.clone(), ty_str.clone()], metamodelica::AsArg::as_arg(&info))?;
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

pub(crate) fn checkModifierIfRedeclare(
    mut inItem: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inModifier: &metamodelica::Ref<SCode::Mod>,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (match &**inModifier {
        SCode::Mod::REDECL { element: el, .. } => {
            checkRedeclaredElementPrefix(inItem, el, inInfo)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn checkRedeclaredElementPrefix(
    mut inItem: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inReplacement: &metamodelica::Ref<SCode::Element>,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (inItem, inReplacement) {
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, prefixes: Deref @ SCode::Prefixes { finalPrefix: fin, replaceablePrefix: repl, .. }, attributes: SCode::Attributes { variability: var, .. }, typeSpec: ty1, info, .. }, .. }, Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { .. }, typeSpec: ty2, .. }) => {
            let mut ty: ArcStr;
            let mut ok: bool;
            ty = literal!("component");
            ok = checkCompRedeclarationReplaceable(name.clone(), metamodelica::AsArg::as_arg(&repl), metamodelica::AsArg::as_arg(&ty1), ty2, inInfo.clone(), info.clone())?;
            ok = checkRedeclarationFinal(name.clone(), ty.clone(), fin.clone(), inInfo.clone(), info.clone())? && ok;
            ok = checkRedeclarationVariability(name.clone(), ty, var.clone(), inInfo, info.clone())? && ok;
            let true = (ok) else { return Err("pattern mismatch") };
            ()
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, prefixes: Deref @ SCode::Prefixes { finalPrefix: fin, replaceablePrefix: repl, .. }, restriction: res, info, .. }, .. }, Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { .. }, .. }) => {
            let mut ty: ArcStr;
            let mut ok: bool;
            ty = SCodeDump::restrictionStringPP(res.clone())?;
            ok = checkClassRedeclarationReplaceable(name.clone(), metamodelica::AsArg::as_arg(&repl), inInfo.clone(), info.clone())?;
            ok = checkRedeclarationFinal(name.clone(), ty, fin.clone(), inInfo, info.clone())? && ok;
            let true = (ok) else { return Err("pattern mismatch") };
            ()
        },
        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name, info, .. }, .. }, Deref @ SCode::Element::CLASS { restriction: res, .. }) => {
            let mut ty: ArcStr;
            ty = SCodeDump::restrictionStringPP(res.clone())?;
            ty = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("a ")); __mm_s.push_str(&*ty); ArcStr::from(__mm_s) };
            Error::addMultiSourceMessage(&(Error::INVALID_REDECLARE_AS.clone()), &(list![literal!("component"), name.clone(), ty]), &(list![inInfo, info.clone()]))?;
            return Err("fail")
        },
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, info, .. }, .. }, Deref @ SCode::Element::COMPONENT { name, .. }) => {
            let mut ty: ArcStr;
            ty = SCodeDump::restrictionStringPP(res.clone())?;
            Error::addMultiSourceMessage(&(Error::INVALID_REDECLARE_AS.clone()), &(list![ty, name.clone(), literal!("a component")]), &(list![inInfo, info.clone()]))?;
            return Err("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkClassRedeclarationReplaceable(
    mut inName: ArcStr,
    mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>,
    mut inOriginInfo: SourceInfo,
    mut inInfo: SourceInfo,
) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match &**inReplaceable {
        SCode::Replaceable::NOT_REPLACEABLE { .. } if (!(Flags::getConfigBool(Flags::IGNORE_REPLACEABLE.clone())?)) => {
            Error::addMultiSourceMessage(
                &(Error::REDECLARE_NON_REPLACEABLE.clone()),
                &(list![inName]),
                &(list![inOriginInfo, inInfo]),
            )?;
            false
        }
        _ => true,
    });
    Ok(isValid)
}

fn checkCompRedeclarationReplaceable(
    mut inName: ArcStr,
    mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>,
    mut inType1: &metamodelica::Ref<Absyn::TypeSpec>,
    mut inType2: &metamodelica::Ref<Absyn::TypeSpec>,
    mut inOriginInfo: SourceInfo,
    mut inInfo: SourceInfo,
) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match &**inReplaceable {
        SCode::Replaceable::NOT_REPLACEABLE { .. }
            if (AbsynUtil::pathEqual(&(AbsynUtil::typeSpecPath(inType1)), &(AbsynUtil::typeSpecPath(inType2)))) =>
        {
            true
        }
        SCode::Replaceable::NOT_REPLACEABLE { .. } if (!(Flags::getConfigBool(Flags::IGNORE_REPLACEABLE.clone())?)) => {
            Error::addMultiSourceMessage(
                &(Error::REDECLARE_NON_REPLACEABLE.clone()),
                &(list![inName]),
                &(list![inOriginInfo, inInfo]),
            )?;
            return Err("fail");
        }
        _ => true,
    });
    Ok(isValid)
}

fn checkRedeclarationFinal(
    mut inName: ArcStr,
    mut inType: ArcStr,
    mut inFinal: SCode::Final,
    mut inOriginInfo: SourceInfo,
    mut inInfo: SourceInfo,
) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match inFinal {
        SCode::Final::NOT_FINAL { .. } => true,
        SCode::Final::FINAL { .. } => {
            Error::addMultiSourceMessage(
                &(Error::INVALID_REDECLARE.clone()),
                &(list![literal!("final"), inType, inName]),
                &(list![inOriginInfo, inInfo]),
            )?;
            false
        }
    });
    Ok(isValid)
}

fn checkRedeclarationVariability(
    mut inName: ArcStr,
    mut inType: ArcStr,
    mut inVariability: SCode::Variability,
    mut inOriginInfo: SourceInfo,
    mut inInfo: SourceInfo,
) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match inVariability {
        SCode::Variability::CONST { .. } => {
            Error::addMultiSourceMessage(
                &(Error::INVALID_REDECLARE.clone()),
                &(list![literal!("constant"), inType, inName]),
                &(list![inOriginInfo, inInfo]),
            )?;
            false
        }
        _ => true,
    });
    Ok(isValid)
}

fn checkRedeclarationVisibility(
    mut inName: ArcStr,
    mut inType: &ArcStr,
    mut inOriginalVisibility: SCode::Visibility,
    mut inNewVisibility: SCode::Visibility,
    mut inOriginInfo: SourceInfo,
    mut inNewInfo: SourceInfo,
) -> Result<bool> {
    let mut isValid: bool;
    isValid = (match (inOriginalVisibility, inNewVisibility) {
        (SCode::Visibility::PUBLIC { .. }, SCode::Visibility::PROTECTED { .. }) => {
            Error::addMultiSourceMessage(
                &(Error::INVALID_REDECLARE_AS.clone()),
                &(list![literal!("public element"), inName, literal!("protected")]),
                &(list![inNewInfo, inOriginInfo]),
            )?;
            false
        }
        (SCode::Visibility::PROTECTED { .. }, SCode::Visibility::PUBLIC { .. }) => {
            Error::addMultiSourceMessage(
                &(Error::INVALID_REDECLARE_AS.clone()),
                &(list![literal!("protected element"), inName, literal!("public")]),
                &(list![inNewInfo, inOriginInfo]),
            )?;
            false
        }
        _ => true,
    });
    Ok(isValid)
}

pub(crate) fn checkDuplicateRedeclarations(
    mut inRedeclare: metamodelica::Ref<NFSCodeEnv::Redeclaration>,
    mut inRedeclarations: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
) -> Result<()> {
    let mut el_name: ArcStr;
    let mut el_info: SourceInfo;
    (el_name, el_info) = NFSCodeEnv::getRedeclarationNameInfo(inRedeclare)?;
    let false = (checkDuplicateRedeclarations2(&el_name, &el_info, inRedeclarations)) else {
        return Err("pattern mismatch");
    };
    Ok(())
}

fn checkDuplicateRedeclarations2(
    mut inRedeclareName: &ArcStr,
    mut inRedeclareInfo: &SourceInfo,
    mut inRedeclarations: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Redeclaration>>,
) -> bool {
    let mut outIsDuplicate: bool;
    outIsDuplicate = 'mc: {
        let __mc_input = &**inRedeclarations;
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
                Deref @ metamodelica::ListNode::Cons { head: redecl, tail: _ } => {
                    let mut el_name: ArcStr;
                    let mut el_info: SourceInfo;
                    (el_name, el_info) = NFSCodeEnv::getRedeclarationNameInfo(redecl.clone())?;
                    let true = (stringEqual(&inRedeclareName, &el_name)) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), &el_info)?;
                    Error::addSourceMessage(&(Error::DUPLICATE_REDECLARATION.clone()), list![inRedeclareName.clone()], inRedeclareInfo)?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_redecls } => {
                    Ok(checkDuplicateRedeclarations2(inRedeclareName, inRedeclareInfo, metamodelica::AsArg::as_arg(&rest_redecls)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIsDuplicate
}

pub(crate) fn checkRecursiveComponentDeclaration(
    mut inComponentName: ArcStr,
    mut inComponentInfo: &SourceInfo,
    mut inTypeEnv: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
    mut inTypeItem: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inComponentEnv: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&*inTypeEnv, &*inComponentEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let false = (NFSCodeEnv::envPrefixOf(inTypeEnv.clone(), inComponentEnv.clone())) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(cls_name), .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ } }) => {
                    let mut el: metamodelica::Ref<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(metamodelica::AsArg::as_arg(&tree), cls_name.clone())?) {
                        Deref @ NFSCodeEnv::Item::CLASS { cls: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    el = metamodelica::Own::own(__pa0);
                    let true = (SCodeUtil::isFunction(&el)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(cls_name), .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: tree, .. }, tail: _ } }) => {
                    let mut el: metamodelica::Ref<SCode::Element>;
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::EnvTree::get(metamodelica::AsArg::as_arg(&tree), cls_name.clone())?) {
                        Deref @ NFSCodeEnv::Item::CLASS { cls: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    el = metamodelica::Own::own(__pa0);
                    let true = (SCodeUtil::isUniontype(&el)) else { return Err("pattern mismatch") };
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
                    let mut ty_name: ArcStr;
                    ty_name = NFSCodeEnv::getItemName(inTypeItem)?;
                    Error::addSourceMessage(&(Error::RECURSIVE_DEFINITION.clone()), list![inComponentName.clone(), ty_name.clone()], inComponentInfo)?;
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

pub(crate) fn checkIdentNotEqTypeName(
    mut inIdent: ArcStr,
    mut inTypeName: &metamodelica::Ref<Absyn::TypeSpec>,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut outIsNotEq: bool;
    outIsNotEq = 'mc: {
        let __mc_input = (inIdent, &**inTypeName);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id, Deref @ Absyn::TypeSpec::TPATH { path: Deref @ Absyn::Path::IDENT { name: ty }, .. }) => {
                    let true = (stringEq(&id, &ty)) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::LOOKUP_TYPE_FOUND_COMP.clone()), list![id.clone()], inInfo)?;
                    Ok(false)
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
    outIsNotEq
}

pub(crate) fn checkComponentsEqual(
    mut inComponent1: &metamodelica::Ref<NFInstTypes::Component>,
    mut inComponent2: &metamodelica::Ref<NFInstTypes::Component>,
) -> () {
    let () = (match &**inComponent2 {
        _ => {
            metamodelica::print(literal!("Found duplicate component\n"));
            ()
        }
    });
    ()
}

pub(crate) fn checkInstanceRestriction(
    mut inItem: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inPrefix: metamodelica::Ref<NFInstPrefix::Prefix>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inItem;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, .. }, .. } => {
                    let true = (SCodeUtil::isInstantiableClassRestriction(metamodelica::AsArg::as_arg(&res))) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { restriction: res, .. }, .. } => {
                    let mut pre_str: ArcStr;
                    let mut res_str: ArcStr;
                    res_str = SCodeDump::restrictionStringPP(res.clone())?;
                    pre_str = NFInstDump::prefixStr(inPrefix.clone())?;
                    Error::addSourceMessage(&(Error::INVALID_CLASS_RESTRICTION.clone()), list![res_str.clone(), pre_str.clone()], inInfo)?;
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
                    Debug::traceln(literal!("- NFSCodeCheck.checkInstanceRestriction failed on unknown item."))?;
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

pub(crate) fn checkPartialInstance(
    mut inItem: &metamodelica::Ref<NFSCodeEnv::Item>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inItem {
        Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name, partialPrefix: SCode::Partial::PARTIAL { .. }, .. }, .. } => {
            Error::addSourceMessage(&(Error::INST_PARTIAL_CLASS.clone()), list![name.clone()], inInfo)?;
            return Err("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
