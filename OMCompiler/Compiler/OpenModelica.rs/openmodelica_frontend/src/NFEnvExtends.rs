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
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Env = metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>;

pub type ClassType = NFSCodeEnv::ClassType;

pub type Extends = metamodelica::Ref<NFSCodeEnv::Extends>;

pub type Frame = metamodelica::Ref<NFSCodeEnv::Frame>;

pub type FrameType = NFSCodeEnv::FrameType;

pub type Import = Absyn::Import;

pub type Item = metamodelica::Ref<NFSCodeEnv::Item>;

pub type ExtendsTableArray = metamodelica::Array<ExtendsWrapper>;

pub(crate) const BASECLASS_NOT_FOUND_ERROR: &'static str = "$1";

pub(crate) const BASECLASS_INHERITED_ERROR: &'static str = "$2";

pub(crate) const BASECLASS_REPLACEABLE_ERROR: &'static str = "$3";

pub(crate) const BASECLASS_IS_VAR_ERROR: &'static str = "$4";

pub(crate) const BASECLASS_UNKNOWN_ERROR: &'static str = "$5";

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ExtendsWrapper {
    UNQUALIFIED_EXTENDS { ext: Extends },
    QUALIFIED_EXTENDS { ext: Extends },
    NO_EXTENDS,
}
impl metamodelica::gc::MMTrace for ExtendsWrapper {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ExtendsWrapper::UNQUALIFIED_EXTENDS { ext } => {
                metamodelica::gc::MMTrace::mm_accept(ext, __mmv)?;
                Ok(())
            }
            ExtendsWrapper::QUALIFIED_EXTENDS { ext } => {
                metamodelica::gc::MMTrace::mm_accept(ext, __mmv)?;
                Ok(())
            }
            ExtendsWrapper::NO_EXTENDS => Ok(()),
        }
    }
}
impl Default for ExtendsWrapper {
    fn default() -> Self {
        Self::NO_EXTENDS
    }
}
pub use self::ExtendsWrapper::{NO_EXTENDS, QUALIFIED_EXTENDS, UNQUALIFIED_EXTENDS};

pub(crate) fn update(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env;
    let mut env: Env;
    env = qualify(inEnv)?;
    outEnv = update2(&env)?;
    Ok(outEnv)
}

pub(crate) fn qualify(mut inEnv: Env) -> Result<Env> {
    let mut outEnv: Env;
    outEnv = 'mc: {
        let __mc_input = &*inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ext_count: i32;
                    let mut ext_table: ExtendsTableArray;
                    ext_count = System::tmpTickIndex(NFSCodeEnv::extendsTickIndex.clone());
                    ext_table = createExtendsTable(ext_count);
                    Ok(qualify2(inEnv.clone(), crate::NFSCodeEnv::ClassType::USERDEFINED, ext_table.clone())?)
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
                    Debug::traceln(literal!("- NFEnvExtends.qualify failed."))?;
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

fn qualify2(mut inEnv: Env, mut inClassType: ClassType, mut inExtendsTable: ExtendsTableArray) -> Result<Env> {
    let mut outEnv: Env;
    let mut env: Env;
    let mut tree: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    env = qualifyLocalScope(inEnv, inClassType, inExtendsTable.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(env.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { clsAndVars: __pa0, .. }, tail: _ } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    tree = metamodelica::Own::own(__pa0);
    tree = NFSCodeEnv::EnvTree::map(
        tree,
        &({
            let __pe_b2 = env.clone();
            let __pe_b3 = inExtendsTable.clone();
            move |__pe_a0, __pe_a1| qualify3(&__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    outEnv = NFSCodeEnv::setEnvClsAndVars(tree, &env)?;
    Ok(outEnv)
}

fn qualify3(mut name: &ArcStr, mut item: Item, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<Item> {
    let mut item: Item = item;
    item = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { cls, env: Deref @ metamodelica::ListNode::Cons { head: cls_env, tail: Deref @ metamodelica::ListNode::Nil }, classType: cls_ty } => {
            let mut env: Env;
            let mut cls_env = (*cls_env).clone();
            env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv);
            let __pa0 = ::match_deref::match_deref! { match &(qualify2(env, cls_ty.clone(), inExtendsTable.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cls_env = metamodelica::Own::own(__pa0);
            metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: list![cls_env.clone()], classType: cls_ty.clone() })
        },
        _ => {
            item
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn qualifyLocalScope(mut inEnv: Env, mut inClassType: ClassType, mut inExtendsTable: ExtendsTableArray) -> Result<Env> {
    let mut outEnv: Env;
    let mut exts: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let mut re: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cei: Option<metamodelica::Ref<SCode::Element>>;
    let __arc3 = NFSCodeEnv::getEnvExtendsTable(&inEnv)?;
    let NFSCodeEnv::EXTENDS_TABLE {
        baseClasses: __pa0,
        redeclaredElements: __pa1,
        classExtendsInfo: __pa2,
    } = &*__arc3;
    exts = metamodelica::Own::own(__pa0);
    re = metamodelica::Own::own(__pa1);
    cei = metamodelica::Own::own(__pa2);
    exts = qualifyExtendsList(exts, inClassType, inEnv.clone(), inExtendsTable.clone())?;
    outEnv = NFSCodeEnv::setEnvExtendsTable(
        metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
            baseClasses: exts,
            redeclaredElements: re,
            classExtendsInfo: cei,
        }),
        &inEnv,
    )?;
    Ok(outEnv)
}

fn qualifyExtendsList(
    mut inExtends: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inClassType: ClassType,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    outExtends = (::match_deref::match_deref! { match &((inExtends.clone(), inClassType)) {
        (Deref @ metamodelica::ListNode::Cons { head: ext, tail: extl }, NFSCodeEnv::ClassType::CLASS_EXTENDS { .. }) => {
            let mut extl = (*extl).clone();
            extl = List::map2Reverse(extl.clone(), &qualifyExtends, inEnv, inExtendsTable.clone())?;
            metamodelica::cons(ext.clone(), extl.clone())
        },
        _ => {
            let mut extl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
            extl = List::map2Reverse(inExtends, &qualifyExtends, inEnv, inExtendsTable.clone())?;
            extl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExtends)
}

fn qualifyExtends(mut inExtends: Extends, mut inEnv: Env, mut inExtendsTable: ExtendsTableArray) -> Result<Extends> {
    let mut outExtends: Extends;
    outExtends = 'mc: {
        let __mc_input = &*inExtends;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Extends { baseClass: Deref @ Absyn::Path::IDENT { name: id }, .. } => {
                    NFSCodeLookup::lookupBuiltinType(metamodelica::AsArg::as_arg(&id))?;
                    Ok(inExtends.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ext: Extends;
                    let __pa0 = ::match_deref::match_deref! { match &(qualifyExtends2(inExtends.clone(), inEnv.clone(), inExtendsTable.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ext = metamodelica::Own::own(__pa0);
                    Ok(ext.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Extends { baseClass: bc, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFEnvExtends.qualifyExtends failed on ")); __mm_s.push_str(&*AbsynUtil::pathString(bc.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
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

fn qualifyExtends2(
    mut inExtends: Extends,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<Option<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: Option<metamodelica::Ref<NFSCodeEnv::Extends>>;
    outExtends = 'mc: {
        let __mc_input = &*inExtends;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Extends { index, .. } => {
                    Ok(lookupQualifiedExtends(index.clone(), inExtendsTable.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ NFSCodeEnv::Extends { baseClass: bc, redeclareModifiers: rl, index, info } => {
                    let mut ext: Extends;
                    let mut env: Env;
                    let mut bc = (*bc).clone();
                    addUnqualifiedToTable(inExtends.clone(), index.clone(), inExtendsTable.clone())?;
                    env = NFSCodeEnv::removeExtendFromLocalScope(bc.clone(), &inEnv)?;
                    bc = qualifyExtends3(bc.clone(), env.clone(), inExtendsTable.clone(), true, bc.clone(), metamodelica::AsArg::as_arg(&info), None)?;
                    List::map2_0(metamodelica::AsArg::as_arg(&rl), &move |__a0: metamodelica::Ref<NFSCodeEnv::Redeclaration>, __a1: metamodelica::Ref<Absyn::Path>, __a2: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>| NFSCodeCheck::checkRedeclareModifier(&__a0, __a1, &__a2), bc.clone(), inEnv.clone())?;
                    ext = metamodelica::Ref::new(NFSCodeEnv::Extends { baseClass: bc.clone(), redeclareModifiers: rl.clone(), index: index.clone(), info: info.clone() });
                    updateQualifiedInTable(ext.clone(), index.clone(), inExtendsTable.clone())?;
                    Ok(Some(ext.clone()))
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

fn qualifyExtends3<'__b>(
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
    mut inIsFirst: bool,
    mut inFullPath: metamodelica::Ref<Absyn::Path>,
    mut inInfo: &'__b SourceInfo,
    mut inErrorPath: Option<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inBaseClass, inErrorPath)) {
            (_, Some(bc)) => {
                return Ok(bc.clone())
            },
            (Deref @ Absyn::Path::IDENT { name }, _) => {
                let mut env: Env;
                let mut ep: Option<metamodelica::Ref<Absyn::Path>>;
                let mut opath: Option<metamodelica::Ref<Absyn::Path>>;
                (opath, env, ep) = qualifyExtendsPart(name.clone(), &inEnv, inExtendsTable.clone(), inIsFirst, inFullPath, inInfo)?;
                return Ok(makeExtendsPath(opath, None, &env, ep, inIsFirst)?)
            },
            (Deref @ Absyn::Path::QUALIFIED { name, path: rest_path }, _) => {
                let mut env: Env;
                let mut ep: Option<metamodelica::Ref<Absyn::Path>>;
                let mut opath: Option<metamodelica::Ref<Absyn::Path>>;
                let mut rest_path = (*rest_path).clone();
                (opath, env, ep) = qualifyExtendsPart(name.clone(), &inEnv, inExtendsTable.clone(), inIsFirst, inFullPath.clone(), inInfo)?;
                rest_path = qualifyExtends3(rest_path.clone(), env.clone(), inExtendsTable.clone(), false, inFullPath, inInfo, ep.clone())?;
                return Ok(makeExtendsPath(opath, Some(rest_path.clone()), &env, ep, inIsFirst)?)
            },
            (Deref @ Absyn::Path::FULLYQUALIFIED { path: rest_path }, _) => {
                let mut env: Env;
                env = NFSCodeEnv::getEnvTopScope(inEnv)?;
                { (inBaseClass, inEnv, inExtendsTable, inIsFirst, inFullPath, inInfo, inErrorPath) = (rest_path.clone(), env, inExtendsTable.clone(), inIsFirst, rest_path.clone(), inInfo, None); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn makeExtendsPath(
    mut inFirstPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inRestPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inEnv: &Env,
    mut inErrorPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inIsFirst: bool,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inFirstPath, inRestPath.clone(), inErrorPath, inIsFirst)) {
        (_, _, Some(path), _) => {
            path.clone()
        },
        (_, Some(path @ Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", .. }), _, _) => {
            path.clone()
        },
        (_, Some(path @ Deref @ Absyn::Path::FULLYQUALIFIED { .. }), _, _) => {
            path.clone()
        },
        (_, _, _, true) => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = NFSCodeEnv::getEnvPath(inEnv)?;
            path = AbsynUtil::joinPathsOptSuffix(path, inRestPath)?;
            path = AbsynUtil::makeFullyQualified(path);
            path
        },
        (Some(path), _, _, _) => {
            AbsynUtil::joinPathsOptSuffix(path.clone(), inRestPath)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

fn qualifyExtendsPart(
    mut inName: ArcStr,
    mut inEnv: &Env,
    mut inExtendsTable: ExtendsTableArray,
    mut inIsFirst: bool,
    mut inFullPath: metamodelica::Ref<Absyn::Path>,
    mut inInfo: &SourceInfo,
) -> Result<(
    Option<metamodelica::Ref<Absyn::Path>>,
    Env,
    Option<metamodelica::Ref<Absyn::Path>>,
)> {
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Env;
    let mut outErrorPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut oitem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut oenv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    let mut fe: bool;
    (oitem, outPath, oenv, fe) = lookupSimpleName(&inName, inEnv, inExtendsTable.clone());
    (outEnv, outErrorPath) = qualifyExtendsPart2(
        metamodelica::Ref::new(Absyn::Path::IDENT { name: inName }),
        oitem,
        oenv,
        inEnv,
        inIsFirst,
        fe,
        inFullPath,
    )?;
    Ok((outPath, outEnv, outErrorPath))
}

fn qualifyExtendsPart2(
    mut inPartName: metamodelica::Ref<Absyn::Path>,
    mut inItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    mut inFoundEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
    mut inOriginEnv: &Env,
    mut inIsFirst: bool,
    mut inFromExtends: bool,
    mut inFullPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(Env, Option<metamodelica::Ref<Absyn::Path>>)> {
    let mut outEnv: Env;
    let mut outErrorPath: Option<metamodelica::Ref<Absyn::Path>>;
    (outEnv, outErrorPath) = (::match_deref::match_deref! { match &((inItem, inFoundEnv)) {
        (Some(item), Some(env)) => {
            let mut ep: Option<metamodelica::Ref<Absyn::Path>>;
            let mut env = (*env).clone();
            ep = checkExtendsPart(inIsFirst, inFromExtends, inPartName, metamodelica::AsArg::as_arg(&item), inFullPath, metamodelica::AsArg::as_arg(&env), inOriginEnv)?;
            env = NFSCodeEnv::mergeItemEnv(metamodelica::AsArg::as_arg(&item), metamodelica::AsArg::as_arg(&env));
            (env.clone(), ep)
        },
        _ => {
            (NFSCodeEnv::emptyEnv.clone(), makeExtendsError(inFullPath, inPartName, arcstr::literal!(BASECLASS_NOT_FOUND_ERROR))?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEnv, outErrorPath))
}

fn makeExtendsError(
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
    mut inPart: metamodelica::Ref<Absyn::Path>,
    mut inError: ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Path>>> {
    let mut outError: Option<metamodelica::Ref<Absyn::Path>>;
    outError = (match inError.clone() {
        _ => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = AbsynUtil::joinPaths(
                inPart,
                metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("$bc"),
                    path: inBaseClass,
                }),
            )?;
            path = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("$E"),
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: inError,
                    path: path,
                }),
            });
            Some(path)
        }
    });
    Ok(outError)
}

fn checkExtendsPart(
    mut inIsFirst: bool,
    mut inFromExtends: bool,
    mut inPartName: metamodelica::Ref<Absyn::Path>,
    mut inItem: &Item,
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
    mut inFoundEnv: &Env,
    mut inOriginEnv: &Env,
) -> Result<Option<metamodelica::Ref<Absyn::Path>>> {
    let mut outErrorPath: Option<metamodelica::Ref<Absyn::Path>>;
    outErrorPath = 'mc: {
        let __mc_input = (inIsFirst, inFromExtends, &**inItem);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, true, _) => {
                    Ok(makeExtendsError(inBaseClass.clone(), inPartName.clone(), arcstr::literal!(BASECLASS_INHERITED_ERROR))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ NFSCodeEnv::Item::CLASS { .. }) => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ NFSCodeEnv::Item::VAR { .. }) => {
                    let mut part: metamodelica::Ref<Absyn::Path>;
                    part = NFSCodeEnv::mergePathWithEnvPath(inPartName.clone(), inFoundEnv);
                    Ok(makeExtendsError(inBaseClass.clone(), part.clone(), arcstr::literal!(BASECLASS_IS_VAR_ERROR))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(makeExtendsError(inBaseClass.clone(), inPartName.clone(), arcstr::literal!(BASECLASS_UNKNOWN_ERROR))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outErrorPath)
}

fn splitExtendsErrorPath(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> Result<(metamodelica::Ref<Absyn::Path>, metamodelica::Ref<Absyn::Path>)> {
    let mut outBaseClass: metamodelica::Ref<Absyn::Path>;
    let mut outPartPath: metamodelica::Ref<Absyn::Path>;
    (outBaseClass, outPartPath) = (::match_deref::match_deref! { match inPath {
        Deref @ Absyn::Path::QUALIFIED { name: part_str, path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$bc", path: bc } } => {
            (bc.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: part_str.clone() }))
        },
        Deref @ Absyn::Path::QUALIFIED { name: part_str, path: part } => {
            let mut bc: metamodelica::Ref<Absyn::Path>;
            let mut part = (*part).clone();
            (bc, part) = splitExtendsErrorPath(metamodelica::AsArg::as_arg(&part))?;
            (bc, metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: part_str.clone(), path: part.clone() }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outBaseClass, outPartPath))
}

pub(crate) fn printExtendsError(
    mut inErrorPath: metamodelica::Ref<Absyn::Path>,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &*inErrorPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { name: Deref @ "$E", path: Deref @ Absyn::Path::QUALIFIED { name: err_str, path: bc } } => {
                    let mut part: metamodelica::Ref<Absyn::Path>;
                    let mut env: Env;
                    let mut bc = (*bc).clone();
                    (bc, part) = splitExtendsErrorPath(metamodelica::AsArg::as_arg(&bc))?;
                    env = NFSCodeEnv::removeExtendFromLocalScope(inErrorPath.clone(), inEnv)?;
                    printExtendsError2(metamodelica::AsArg::as_arg(&err_str), bc.clone(), &part, env.clone(), inInfo)?;
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFEnvExtends.printExtendsError failed to print error ")); __mm_s.push_str(&*AbsynUtil::pathString(inErrorPath.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
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

pub(crate) fn printExtendsError2(
    mut inError: &ArcStr,
    mut inBaseClass: metamodelica::Ref<Absyn::Path>,
    mut inPartPath: &metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inPartPath;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut bc_str: ArcStr;
                    let mut env_str: ArcStr;
                    let true = (stringEq(&inError, &arcstr::literal!(BASECLASS_NOT_FOUND_ERROR))) else { return Err("pattern mismatch") };
                    bc_str = AbsynUtil::pathString(inBaseClass.clone(), literal!("."), true, false)?;
                    env_str = NFSCodeEnv::getEnvName(&inEnv);
                    Error::addSourceMessage(&(Error::LOOKUP_BASECLASS_ERROR.clone()), list![bc_str.clone(), env_str.clone()], inInfo)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: part } => {
                    let mut bc_str: ArcStr;
                    let mut exts: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
                    let true = (stringEq(&inError, &arcstr::literal!(BASECLASS_INHERITED_ERROR))) else { return Err("pattern mismatch") };
                    bc_str = AbsynUtil::pathString(inBaseClass.clone(), literal!("."), true, false)?;
                    Error::addSourceMessage(&(Error::INHERITED_EXTENDS.clone()), list![bc_str.clone()], inInfo)?;
                    exts = NFSCodeEnv::getEnvExtendsFromTable(&inEnv)?;
                    printInheritedExtendsError(metamodelica::AsArg::as_arg(&part), &exts, &inEnv);
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
                    let mut bc_str: ArcStr;
                    let mut part: ArcStr;
                    let mut info: SourceInfo;
                    let true = (stringEq(&inError, &arcstr::literal!(BASECLASS_REPLACEABLE_ERROR))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupFullyQualified(inPartPath, inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { name: __pa0, info: __pa1, .. }, .. }, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    part = metamodelica::Own::own(__pa0);
                    info = metamodelica::Own::own(__pa1);
                    bc_str = AbsynUtil::pathString(inBaseClass.clone(), literal!("."), true, false)?;
                    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), inInfo)?;
                    Error::addSourceMessage(&(Error::REPLACEABLE_BASE_CLASS.clone()), list![part.clone(), bc_str.clone()], &info)?;
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
                    let mut bc_str: ArcStr;
                    let mut part: ArcStr;
                    let mut info: SourceInfo;
                    let true = (stringEq(&inError, &arcstr::literal!(BASECLASS_IS_VAR_ERROR))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupFullyQualified(inPartPath, inEnv.clone())?) {
                        (Deref @ NFSCodeEnv::Item::VAR { var: Deref @ SCode::Element::COMPONENT { name: __pa0, info: __pa1, .. }, .. }, _, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    part = metamodelica::Own::own(__pa0);
                    info = metamodelica::Own::own(__pa1);
                    bc_str = AbsynUtil::pathString(inBaseClass.clone(), literal!("."), true, false)?;
                    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), &info)?;
                    Error::addSourceMessage(&(Error::EXTEND_THROUGH_COMPONENT.clone()), list![part.clone(), bc_str.clone()], inInfo)?;
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

fn printInheritedExtendsError(
    mut inName: &ArcStr,
    mut inExtends: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inEnv: &Env,
) -> () {
    let () = 'mc: {
        let __mc_input = &**inExtends;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: ext @ Deref @ NFSCodeEnv::Extends { baseClass: bc, info: info2, .. }, tail: rest_ext } => {
                    let mut item: Item;
                    let mut info1: SourceInfo;
                    let mut bc_str: ArcStr;
                    let mut bc = (*bc).clone();
                    let mut info2 = (*info2).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupInBaseClasses3(inName.clone(), metamodelica::AsArg::as_arg(&ext), inEnv.clone(), inEnv.clone(), crate::NFSCodeLookup::RedeclareReplaceStrategy::IGNORE_REDECLARES, metamodelica::nil())?) {
                        (Some(__pa0), _, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    item = metamodelica::Own::own(__pa0);
                    info1 = NFSCodeEnv::getItemInfo(&item)?;
                    let __arc3 = ext.clone();
                    let NFSCodeEnv::EXTENDS { baseClass: __pa1, info: __pa2, .. } = &*__arc3;
                    bc = metamodelica::Own::own(__pa1);
                    info2 = metamodelica::Own::own(__pa2);
                    bc = AbsynUtil::makeNotFullyQualified(bc.clone());
                    bc_str = AbsynUtil::pathString(bc.clone(), literal!("."), true, false)?;
                    Error::addSourceMessage(&(Error::ERROR_FROM_HERE.clone()), metamodelica::nil(), &info1)?;
                    Error::addSourceMessage(&(Error::EXTENDS_INHERITED_FROM_LOCAL_EXTENDS.clone()), list![inName.clone(), bc_str.clone()], metamodelica::AsArg::as_arg(&info2))?;
                    printInheritedExtendsError(inName, metamodelica::AsArg::as_arg(&rest_ext), inEnv);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_ext } => {
                    printInheritedExtendsError(inName, metamodelica::AsArg::as_arg(&rest_ext), inEnv);
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
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
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn lookupSimpleName(
    mut inName: &ArcStr,
    mut inEnv: &Env,
    mut inExtendsTable: ExtendsTableArray,
) -> (
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
    bool,
) {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    let mut outFromExtends: bool;
    (outItem, outPath, outEnv, outFromExtends) = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut fe: bool;
                    (opt_item, opt_path, opt_env, fe) = lookupInLocalScope(inName.clone(), inEnv.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone(), fe))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { frameType: frame_type, .. }, tail: env } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    NFSCodeLookup::frameNotEncapsulated(frame_type.clone())?;
                    (opt_item, opt_path, opt_env, _) = lookupSimpleName(inName, metamodelica::AsArg::as_arg(&env), inExtendsTable.clone());
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((None, None, None, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outItem, outPath, outEnv, outFromExtends)
}

fn lookupInLocalScope(
    mut inName: ArcStr,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
    bool,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    let mut outFromExtends: bool;
    (outItem, outPath, outEnv, outFromExtends) = 'mc: {
        let __mc_input = &*inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut item: Item;
                    let mut env: Env;
                    (item, env) = NFSCodeLookup::lookupInClass(inName.clone(), inEnv.clone())?;
                    Ok((Some(item.clone()), Some(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() })), Some(env.clone()), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: bcl @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. }, tail: _ } => {
                    let mut oitem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut oenv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (oitem, oenv) = lookupInBaseClasses(&inName, metamodelica::AsArg::as_arg(&bcl), &inEnv, inExtendsTable.clone())?;
                    Ok((oitem.clone(), Some(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() })), oenv.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { hidden: false, qualifiedImports: imps, .. }, .. }, tail: _ } => {
                    let mut oitem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opath: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut oenv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (oitem, opath, oenv) = lookupInQualifiedImports(&inName, metamodelica::AsArg::as_arg(&imps), &inEnv, inExtendsTable.clone())?;
                    Ok((oitem.clone(), opath.clone(), oenv.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { importTable: NFSCodeEnv::ImportTable { hidden: false, unqualifiedImports: imps, .. }, .. }, tail: _ } => {
                    let mut oitem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opath: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut oenv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (oitem, opath, oenv) = lookupInUnqualifiedImports(&inName, metamodelica::AsArg::as_arg(&imps), &inEnv, inExtendsTable.clone())?;
                    Ok((oitem.clone(), opath.clone(), oenv.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv, outFromExtends))
}

fn lookupInBaseClasses(
    mut inName: &ArcStr,
    mut inExtends: &metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inEnv: &Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outEnv) = 'mc: {
        let __mc_input = &**inExtends;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: ext, tail: _ } => {
                    let mut opt_ext: Option<metamodelica::Ref<NFSCodeEnv::Extends>>;
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    let mut env: Env;
                    env = NFSCodeEnv::setImportTableHidden(inEnv, false)?;
                    opt_ext = qualifyExtends2(ext.clone(), env.clone(), inExtendsTable.clone())?;
                    (opt_item, opt_env) = lookupInBaseClasses2(inName.clone(), opt_ext.clone(), env.clone(), inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_ext } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (opt_item, opt_env) = lookupInBaseClasses(inName, metamodelica::AsArg::as_arg(&rest_ext), inEnv, inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outEnv))
}

fn lookupInBaseClasses2(
    mut inName: ArcStr,
    mut inExtends: Option<metamodelica::Ref<NFSCodeEnv::Extends>>,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outEnv) = (::match_deref::match_deref! { match &(inExtends) {
        Some(Deref @ NFSCodeEnv::Extends { baseClass: Deref @ Absyn::Path::FULLYQUALIFIED { path: bc }, .. }) => {
            let mut item: Item;
            let mut env: Env;
            let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
            let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
            (item, env) = lookupFullyQualified(metamodelica::AsArg::as_arg(&bc), inEnv, inExtendsTable.clone())?;
            env = NFSCodeEnv::mergeItemEnv(&item, &env);
            env = NFSCodeEnv::setImportTableHidden(&env, true)?;
            (opt_item, _, opt_env, _) = lookupInLocalScope(inName, env, inExtendsTable.clone())?;
            (opt_item, opt_env)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outItem, outEnv))
}

fn lookupInQualifiedImports(
    mut inName: &ArcStr,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: &Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    let false = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    (opt_item, opt_path, opt_env) = lookupInQualifiedImports(inName, metamodelica::AsArg::as_arg(&rest_imps), inEnv, inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, path }, tail: _ } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    let true = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    (item, env) = lookupFullyQualified(metamodelica::AsArg::as_arg(&path), inEnv.clone(), inExtendsTable.clone())?;
                    path = NFSCodeEnv::prefixIdentWithEnv(inName.clone(), &env)?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ } => {
                    let true = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    Ok((None, None, None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

fn lookupInUnqualifiedImports(
    mut inName: &ArcStr,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inEnv: &Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(
    Option<metamodelica::Ref<NFSCodeEnv::Item>>,
    Option<metamodelica::Ref<Absyn::Path>>,
    Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>,
)> {
    let mut outItem: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
    let mut outPath: Option<metamodelica::Ref<Absyn::Path>>;
    let mut outEnv: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
    (outItem, outPath, outEnv) = 'mc: {
        let __mc_input = &**inImports;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ } => {
                    let mut item: Item;
                    let mut env: Env;
                    let mut path = (*path).clone();
                    (item, env) = lookupFullyQualified(metamodelica::AsArg::as_arg(&path), inEnv.clone(), inExtendsTable.clone())?;
                    env = NFSCodeEnv::mergeItemEnv(&item, &env);
                    (item, env) = lookupFullyQualified2(&(metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() })), env.clone(), inExtendsTable.clone())?;
                    path = NFSCodeEnv::prefixIdentWithEnv(inName.clone(), &env)?;
                    path = AbsynUtil::makeFullyQualified(path.clone());
                    Ok((Some(item.clone()), Some(path.clone()), Some(env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_imps } => {
                    let mut opt_item: Option<metamodelica::Ref<NFSCodeEnv::Item>>;
                    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
                    let mut opt_env: Option<metamodelica::List<metamodelica::Ref<NFSCodeEnv::Frame>>>;
                    (opt_item, opt_path, opt_env) = lookupInUnqualifiedImports(inName, metamodelica::AsArg::as_arg(&rest_imps), inEnv, inExtendsTable.clone())?;
                    Ok((opt_item.clone(), opt_path.clone(), opt_env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outItem, outPath, outEnv))
}

fn lookupFullyQualified(
    mut inName: &metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(Item, Env)> {
    let mut outItem: Item;
    let mut outEnv: Env;
    let mut env: Env;
    env = NFSCodeEnv::getEnvTopScope(inEnv)?;
    (outItem, outEnv) = lookupFullyQualified2(inName, env, inExtendsTable.clone())?;
    Ok((outItem, outEnv))
}

fn lookupFullyQualified2<'__b>(
    mut inName: &'__b metamodelica::Ref<Absyn::Path>,
    mut inEnv: Env,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<(Item, Env)> {
    '__tco: loop {
        match &**inName {
            Absyn::Path::IDENT { name } => {
                let mut item: Item;
                let mut env: Env;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope(name.clone(), inEnv, inExtendsTable.clone())?) {
                    (Some(__pa0), _, Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                item = metamodelica::Own::own(__pa0);
                env = metamodelica::Own::own(__pa1);
                return Ok((item, env));
            }
            Absyn::Path::QUALIFIED { name, path: rest_path } => {
                let mut item: Item;
                let mut env: Env;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupInLocalScope(name.clone(), inEnv, inExtendsTable.clone())?) {
                    (Some(__pa0), _, Some(__pa1), _) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                item = metamodelica::Own::own(__pa0);
                env = metamodelica::Own::own(__pa1);
                env = NFSCodeEnv::mergeItemEnv(&item, &env);
                {
                    (inName, inEnv, inExtendsTable) = (rest_path, env, inExtendsTable.clone());
                    continue '__tco;
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

fn createExtendsTable(mut inSize: i32) -> ExtendsTableArray {
    let mut outTable: ExtendsTableArray;
    outTable = arrayCreate(inSize, crate::NFEnvExtends::ExtendsWrapper::NO_EXTENDS);
    outTable
}

fn lookupQualifiedExtends(
    mut inIndex: i32,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<Option<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: Option<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let mut ext: ExtendsWrapper;
    ext = metamodelica::arrayGet(inExtendsTable.clone(), inIndex)?;
    outExtends = lookupQualifiedExtends2(&ext, inExtendsTable.clone())?;
    Ok(outExtends)
}

fn lookupQualifiedExtends2(
    mut inExtends: &ExtendsWrapper,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<Option<metamodelica::Ref<NFSCodeEnv::Extends>>> {
    let mut outExtends: Option<metamodelica::Ref<NFSCodeEnv::Extends>>;
    outExtends = (::match_deref::match_deref! { match &(inExtends) {
        ExtendsWrapper::QUALIFIED_EXTENDS { ext } => {
            Some(ext.clone())
        },
        ExtendsWrapper::UNQUALIFIED_EXTENDS { ext: Deref @ NFSCodeEnv::Extends { .. } } => {
            None
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExtends)
}

fn addUnqualifiedToTable(
    mut inExtends: Extends,
    mut inIndex: i32,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<()> {
    metamodelica::arrayUpdate(
        inExtendsTable.clone(),
        inIndex,
        ExtendsWrapper::UNQUALIFIED_EXTENDS { ext: inExtends },
    )?;
    Ok(())
}

fn updateQualifiedInTable(
    mut inExtends: Extends,
    mut inIndex: i32,
    mut inExtendsTable: ExtendsTableArray,
) -> Result<()> {
    metamodelica::arrayUpdate(
        inExtendsTable.clone(),
        inIndex,
        ExtendsWrapper::QUALIFIED_EXTENDS { ext: inExtends },
    )?;
    Ok(())
}

fn update2(mut inEnv: &Env) -> Result<Env> {
    let mut outEnv: Env;
    let mut env: Env;
    let mut rest_env: Env;
    let mut name: Option<ArcStr>;
    let mut ty: FrameType;
    let mut tree: metamodelica::Ref<NFSCodeEnv::EnvTree::Tree>;
    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
    let mut re: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut imps: NFSCodeEnv::ImportTable;
    let mut iu: Option<Mutable::Mutable<bool>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &((*inEnv)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: __pa0, frameType: __pa1, clsAndVars: __pa2, extendsTable: Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa3, redeclaredElements: __pa4, classExtendsInfo: _ }, importTable: __pa5, isUsed: __pa6 }, tail: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    tree = metamodelica::Own::own(__pa2);
    bcl = metamodelica::Own::own(__pa3);
    re = metamodelica::Own::own(__pa4);
    imps = metamodelica::Own::own(__pa5);
    iu = metamodelica::Own::own(__pa6);
    rest_env = metamodelica::Own::own(__pa7);
    tree = NFSCodeEnv::EnvTree::map(
        tree,
        &({
            let __pe_b2 = inEnv.clone();
            move |__pe_a0, __pe_a1| update3(&__pe_a0, __pe_a1, __pe_b2.clone())
        }),
    )?;
    env = metamodelica::cons(
        metamodelica::Ref::new(NFSCodeEnv::Frame {
            name: name,
            frameType: ty,
            clsAndVars: tree,
            extendsTable: metamodelica::Ref::new(NFSCodeEnv::ExtendsTable {
                baseClasses: bcl,
                redeclaredElements: metamodelica::nil(),
                classExtendsInfo: None,
            }),
            importTable: imps,
            isUsed: iu,
        }),
        rest_env,
    );
    outEnv = NFSCodeFlattenRedeclare::addElementRedeclarationsToEnv(&re, env)?;
    Ok(outEnv)
}

fn update3(mut name: &ArcStr, mut item: Item, mut inEnv: Env) -> Result<Item> {
    let mut item: Item = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ NFSCodeEnv::Item::CLASS { cls, env: Deref @ metamodelica::ListNode::Cons { head: cls_env, tail: Deref @ metamodelica::ListNode::Nil }, classType: cls_ty } => {
            let mut env: Env;
            let mut cls = (*cls).clone();
            let mut cls_env = (*cls_env).clone();
            env = NFSCodeEnv::enterFrame(cls_env.clone(), inEnv);
            (cls, env) = updateClassExtends(cls.clone(), env, cls_ty.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(update2(&env)?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cls_env = metamodelica::Own::own(__pa0);
            item = metamodelica::Ref::new(NFSCodeEnv::Item::CLASS { cls: cls.clone(), env: list![cls_env.clone()], classType: cls_ty.clone() });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn updateClassExtends(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inEnv: Env,
    mut inClassType: ClassType,
) -> Result<(metamodelica::Ref<SCode::Element>, Env)> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: Env;
    (outClass, outEnv) = (::match_deref::match_deref! { match &((inEnv.clone(), inClassType)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ NFSCodeEnv::Frame { name: Some(name), extendsTable: Deref @ NFSCodeEnv::ExtendsTable { classExtendsInfo: Some(ext), .. }, .. }, tail: _ }, NFSCodeEnv::ClassType::CLASS_EXTENDS { .. }) => {
            let mut env: Env;
            let mut mods: metamodelica::Ref<SCode::Mod>;
            let mut info: SourceInfo;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ext.clone()) {
                Deref @ SCode::Element::EXTENDS { modifications: __pa0, info: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            mods = metamodelica::Own::own(__pa0);
            info = metamodelica::Own::own(__pa1);
            (cls, env) = updateClassExtends2(inClass, name.clone(), mods, info, inEnv);
            (cls, env)
        },
        _ => {
            (inClass, inEnv)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClass, outEnv))
}

fn updateClassExtends2(
    mut inClass: metamodelica::Ref<SCode::Element>,
    mut inName: ArcStr,
    mut inMods: metamodelica::Ref<SCode::Mod>,
    mut inInfo: SourceInfo,
    mut inEnv: Env,
) -> (metamodelica::Ref<SCode::Element>, Env) {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outEnv: Env;
    (outClass, outEnv) = 'mc: {
        let __mc_input = &*inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: cls_frame, tail: env } => {
                    let mut ext: metamodelica::Ref<SCode::Element>;
                    let mut cls: metamodelica::Ref<SCode::Element>;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut cls_frame = (*cls_frame).clone();
                    (path, _) = lookupClassExtendsBaseClass(inName.clone(), metamodelica::AsArg::as_arg(&env), &inInfo)?;
                    ext = metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: path.clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, modifications: inMods.clone(), ann: None, info: inInfo.clone() });
                    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeEnv::extendEnvWithExtends(&ext, &(list![cls_frame.clone()]))?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cls_frame = metamodelica::Own::own(__pa0);
                    cls = SCodeUtil::addElementToClass(ext.clone(), inClass.clone())?;
                    Ok((cls.clone(), metamodelica::cons(cls_frame.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inClass.clone(), inEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outClass, outEnv)
}

fn lookupClassExtendsBaseClass(
    mut inName: ArcStr,
    mut inEnv: &Env,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::Ref<Absyn::Path>, Item)> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut outItem: Item;
    (outPath, outItem) = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut item: Item;
            let mut basename: ArcStr;
            basename = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inName);
                __mm_s.push_str(&*arcstr::literal!(NFSCodeEnv::BASE_CLASS_SUFFIX));
                ArcStr::from(__mm_s)
            };
            (item, _) = NFSCodeLookup::lookupInheritedName(&basename, inEnv)?;
            path = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("$ce"),
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: basename.clone() }),
            });
            Ok((path.clone(), item.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut item: Item;
            (item, _) = NFSCodeLookup::lookupInheritedName(&inName, inEnv)?;
            path = metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() });
            Ok((path.clone(), item.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addSourceMessage(
                &(Error::INVALID_REDECLARATION_OF_CLASS.clone()),
                list![inName.clone()],
                inInfo,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outPath, outItem))
}

pub(crate) fn extendEnvWithClassExtends(
    mut inClassExtends: metamodelica::Ref<SCode::Element>,
    mut inEnv: &Env,
) -> Result<Env> {
    let mut outEnv: Env;
    outEnv = (::match_deref::match_deref! { match &(inClassExtends.clone()) {
        Deref @ SCode::Element::CLASS { name, prefixes, encapsulatedPrefix: ep, partialPrefix: pp, restriction: res, classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: mods, composition: cdef }, cmt, info } => {
            let mut env: Env;
            let mut cls_env: Env;
            let mut cls: metamodelica::Ref<SCode::Element>;
            let mut ext: metamodelica::Ref<SCode::Element>;
            cls = metamodelica::Ref::new(SCode::Element::CLASS { name: name.clone(), prefixes: prefixes.clone(), encapsulatedPrefix: ep.clone(), partialPrefix: pp.clone(), restriction: res.clone(), classDef: cdef.clone(), cmt: cmt.clone(), info: info.clone() });
            cls_env = NFSCodeEnv::makeClassEnvironment(&cls, false)?;
            ext = metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, modifications: mods.clone(), ann: None, info: info.clone() });
            cls_env = addClassExtendsInfoToEnv(ext, &cls_env)?;
            env = NFSCodeEnv::extendEnvWithItem(&(NFSCodeEnv::newClassItem(cls, cls_env, crate::NFSCodeEnv::ClassType::CLASS_EXTENDS)), inEnv, metamodelica::AsArg::as_arg(&name))?;
            env
        },
        _ => {
            let mut info: SourceInfo;
            let mut el_str: ArcStr;
            let mut env_str: ArcStr;
            let mut err_msg: ArcStr;
            info = SCodeUtil::elementInfo(&inClassExtends);
            el_str = SCodeDump::unparseElementStr(inClassExtends, SCodeDump::defaultOptions.clone())?;
            env_str = NFSCodeEnv::getEnvName(inEnv);
            err_msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSCodeFlattenRedeclare.extendEnvWithClassExtends failed on unknown element ")); __mm_s.push_str(&*el_str); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*env_str); ArcStr::from(__mm_s) };
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![err_msg], &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn addClassExtendsInfoToEnv(mut inClassExtends: metamodelica::Ref<SCode::Element>, mut inEnv: &Env) -> Result<Env> {
    let mut outEnv: Env;
    outEnv = 'mc: {
        let __mc_input = &**inEnv;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut bcl: metamodelica::List<metamodelica::Ref<NFSCodeEnv::Extends>>;
                    let mut re: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut ext: metamodelica::Ref<NFSCodeEnv::ExtendsTable>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(NFSCodeEnv::getEnvExtendsTable(inEnv)?) {
                        Deref @ NFSCodeEnv::ExtendsTable { baseClasses: __pa0, redeclaredElements: __pa1, classExtendsInfo: None } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    bcl = metamodelica::Own::own(__pa0);
                    re = metamodelica::Own::own(__pa1);
                    ext = metamodelica::Ref::new(NFSCodeEnv::ExtendsTable { baseClasses: bcl.clone(), redeclaredElements: re.clone(), classExtendsInfo: Some(inClassExtends.clone()) });
                    Ok(NFSCodeEnv::setEnvExtendsTable(ext.clone(), inEnv)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut estr: ArcStr;
                    estr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- NFEnvExtends.addClassExtendsInfoToEnv: Trying to overwrite ")); __mm_s.push_str(&*literal!("existing class extends information, this should not happen!.")); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![estr.clone()])?;
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
