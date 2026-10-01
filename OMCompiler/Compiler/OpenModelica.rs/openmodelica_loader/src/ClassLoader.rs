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

use crate::Parser;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTableStringToProgram;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_script_util::PackageManagement;
use openmodelica_util::Autoconf;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type HashTable = (
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (i32, i32, metamodelica::Array<Option<(ArcStr, Absyn::Program)>>),
    i32,
    (
        HashTableStringToProgram::FuncHashCref,
        HashTableStringToProgram::FuncCrefEqual,
        HashTableStringToProgram::FuncCrefStr,
        HashTableStringToProgram::FuncExpStr,
    ),
);

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum PackageOrder {
    CLASSPART {
        cp: metamodelica::Ref<Absyn::ClassPart>,
    },
    ELEMENT {
        element: metamodelica::Ref<Absyn::ElementItem>,
        /// public
        r#pub: bool,
    },
    CLASSLOAD {
        cl: ArcStr,
    },
}
impl metamodelica::gc::MMTrace for PackageOrder {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            PackageOrder::CLASSPART { cp } => {
                metamodelica::gc::MMTrace::mm_accept(cp, __mmv)?;
                Ok(())
            }
            PackageOrder::ELEMENT { element, r#pub } => {
                metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#pub, __mmv)?;
                Ok(())
            }
            PackageOrder::CLASSLOAD { cl } => {
                metamodelica::gc::MMTrace::mm_accept(cl, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for PackageOrder {
    fn default() -> Self {
        Self::CLASSPART { cp: Default::default() }
    }
}
pub(crate) use self::PackageOrder::{CLASSLOAD, CLASSPART, ELEMENT};

#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub(crate) enum LoadFileStrategy {
    STRATEGY_HASHTABLE { ht: HashTable },
    STRATEGY_ON_DEMAND { encoding: ArcStr },
}
impl metamodelica::gc::MMTrace for LoadFileStrategy {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            LoadFileStrategy::STRATEGY_HASHTABLE { ht } => {
                metamodelica::gc::MMTrace::mm_accept(ht, __mmv)?;
                Ok(())
            }
            LoadFileStrategy::STRATEGY_ON_DEMAND { encoding } => {
                metamodelica::gc::MMTrace::mm_accept(encoding, __mmv)?;
                Ok(())
            }
        }
    }
}
impl PartialEq for LoadFileStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::STRATEGY_HASHTABLE { ht: __l_ht }, Self::STRATEGY_HASHTABLE { ht: __r_ht }) => {
                (match (__l_ht, __r_ht) {
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
                })
            }
            (
                Self::STRATEGY_ON_DEMAND { encoding: __l_encoding },
                Self::STRATEGY_ON_DEMAND { encoding: __r_encoding },
            ) => __l_encoding == __r_encoding,
            _ => false,
        }
    }
}
impl Eq for LoadFileStrategy {}
impl PartialOrd for LoadFileStrategy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for LoadFileStrategy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn __variant_idx(__v: &LoadFileStrategy) -> u32 {
            match __v {
                LoadFileStrategy::STRATEGY_HASHTABLE { .. } => 0,
                LoadFileStrategy::STRATEGY_ON_DEMAND { .. } => 1,
            }
        }
        match __variant_idx(self).cmp(&__variant_idx(other)) {
            std::cmp::Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match (self, other) {
            (Self::STRATEGY_HASHTABLE { ht: __l_ht }, Self::STRATEGY_HASHTABLE { ht: __r_ht }) => {
                (match (__l_ht, __r_ht) {
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
            }
            (
                Self::STRATEGY_ON_DEMAND { encoding: __l_encoding },
                Self::STRATEGY_ON_DEMAND { encoding: __r_encoding },
            ) => __l_encoding.cmp(__r_encoding),
            _ => unreachable!("variant-index equality already implies same variant"),
        }
    }
}
impl std::fmt::Debug for LoadFileStrategy {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STRATEGY_HASHTABLE { ht: __d_ht } => {
                let mut __ds = __f.debug_struct("STRATEGY_HASHTABLE");
                __ds.field("ht", &format_args!("<dyn-fn-container@{:p}>", __d_ht as *const _));
                __ds.finish()
            }
            Self::STRATEGY_ON_DEMAND { encoding: __d_encoding } => {
                let mut __ds = __f.debug_struct("STRATEGY_ON_DEMAND");
                __ds.field("encoding", __d_encoding);
                __ds.finish()
            }
        }
    }
}

pub(crate) use self::LoadFileStrategy::{STRATEGY_HASHTABLE, STRATEGY_ON_DEMAND};

pub fn loadClass(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut priorityList: metamodelica::List<ArcStr>,
    mut modelicaPath: ArcStr,
    mut encoding: Option<ArcStr>,
    mut requireExactVersion: bool,
    mut encrypted: bool,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    outProgram = 'mc: {
        let __mc_input = (&**inPath, modelicaPath);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: classname }, mp) => {
                    let mut gd: ArcStr;
                    let mut mps: metamodelica::List<ArcStr>;
                    let mut p: Absyn::Program;
                    gd = arcstr::literal!(Autoconf::groupDelimiter);
                    mps = System::strtok(mp.clone(), gd.clone());
                    p = loadClassFromMps(classname.clone(), priorityList.clone(), mps.clone(), encoding.clone(), requireExactVersion, encrypted)?;
                    checkOnLoadMessage(p.clone())?;
                    Ok(p.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { name: pack, .. }, mp) => {
                    let mut gd: ArcStr;
                    let mut mps: metamodelica::List<ArcStr>;
                    let mut p: Absyn::Program;
                    gd = arcstr::literal!(Autoconf::groupDelimiter);
                    mps = System::strtok(mp.clone(), gd.clone());
                    p = loadClassFromMps(pack.clone(), priorityList.clone(), mps.clone(), encoding.clone(), requireExactVersion, encrypted)?;
                    checkOnLoadMessage(p.clone())?;
                    Ok(p.clone())
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
                    Debug::trace(literal!("ClassLoader.loadClass failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outProgram)
}

fn loadClassFromMps(
    mut id: ArcStr,
    mut prios: metamodelica::List<ArcStr>,
    mut mps: metamodelica::List<ArcStr>,
    mut encoding: Option<ArcStr>,
    mut requireExactVersion: bool,
    mut encrypted: bool,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut mp: ArcStr;
    let mut name: ArcStr;
    let mut version: ArcStr;
    let mut isDir: bool;
    let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
    let mut versionsThatProvideTheWanted: metamodelica::List<ArcStr>;
    let mut commands: metamodelica::List<ArcStr>;
    let mut versions: metamodelica::List<ArcStr>;
    if !(requireExactVersion) {
        if (prios).is_empty() {
            versions = PackageManagement::versionsThatProvideTheWanted(id.clone(), literal!("default"), false);
        } else {
            versions = metamodelica::nil();
            for mut v in &*prios.clone().reverse() {
                versionsThatProvideTheWanted =
                    PackageManagement::versionsThatProvideTheWanted(id.clone(), v.clone(), false);
                if (versionsThatProvideTheWanted).is_empty() {
                    versions = metamodelica::cons(v.clone(), versions);
                } else {
                    versions = listAppend(versionsThatProvideTheWanted.clone(), versions);
                }
            }
        }
    } else {
        versions = prios.clone();
    }
    if let Ok((__pa0, __pa1, __pa2)) =
        System::getLoadModelPath(id.clone(), versions.clone(), mps.clone(), requireExactVersion)
    {
        mp = metamodelica::Own::own(__pa0);
        name = metamodelica::Own::own(__pa1);
        isDir = metamodelica::Own::own(__pa2);
    } else {
        version = (::match_deref::match_deref! { match &(&*prios) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_version, tail: _ } => {
                version = (*__esc_version).clone();
                version.clone()
            },
            _ => literal!("default"),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        versionsThatProvideTheWanted =
            PackageManagement::versionsThatProvideTheWanted(id.clone(), version.clone(), false);
        if !((versionsThatProvideTheWanted).is_empty()) {
            if metamodelica::stringEq(&version, &(literal!("default")))
                || metamodelica::stringEq(&version, &(literal!("")))
            {
                commands = list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("  installPackage("));
                    __mm_s.push_str(&*id);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }];
            } else {
                commands = metamodelica::nil();
                if listMember(version.clone(), versionsThatProvideTheWanted.clone()) {
                    commands = metamodelica::cons(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("  installPackage("));
                            __mm_s.push_str(&*id);
                            __mm_s.push_str(&*literal!(", \""));
                            __mm_s.push_str(&*version);
                            __mm_s.push_str(&*literal!("\", exactMatch=true)"));
                            ArcStr::from(__mm_s)
                        },
                        commands.clone(),
                    );
                }
                commands = metamodelica::cons(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("  installPackage("));
                        __mm_s.push_str(&*id);
                        __mm_s.push_str(&*literal!(", \""));
                        __mm_s.push_str(&*version);
                        __mm_s.push_str(&*literal!("\", exactMatch=false)"));
                        ArcStr::from(__mm_s)
                    },
                    commands.clone(),
                );
            }
            if !metamodelica::stringEq(&((versionsThatProvideTheWanted).head().cloned()?), &version) {
                commands = metamodelica::cons(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("  installPackage("));
                        __mm_s.push_str(&*id);
                        __mm_s.push_str(&*literal!(", \""));
                        __mm_s.push_str(&*(versionsThatProvideTheWanted).head().cloned()?);
                        __mm_s.push_str(&*literal!("\", exactMatch=true)"));
                        ArcStr::from(__mm_s)
                    },
                    commands.clone(),
                );
            }
            Error::addMessage(
                Error::NOTIFY_PKG_FOUND.clone(),
                list![stringDelimitList(commands.clone(), literal!("\n"))],
            )?;
        }
        return Err("fail");
    }
    Config::setLanguageStandardFromMSL(name.clone(), false)?;
    cl = loadClassFromMp(id, mp, name, isDir, encoding, encrypted)?;
    if (cl).is_some() {
        outProgram = Absyn::Program {
            classes: list![cl.ok_or("pattern mismatch")?],
            within_: openmodelica_ast::Absyn::Within::TOP,
        };
    } else {
        outProgram = Absyn::Program {
            classes: metamodelica::nil(),
            within_: openmodelica_ast::Absyn::Within::TOP,
        };
    }
    Ok(outProgram)
}

pub fn loadClassFromMp(
    mut id: ArcStr,
    mut path: ArcStr,
    mut name: ArcStr,
    mut isDir: bool,
    mut optEncoding: Option<ArcStr>,
    mut encrypted: bool,
) -> Result<Option<metamodelica::Ref<Absyn::Class>>> {
    let mut outClass: Option<metamodelica::Ref<Absyn::Class>>;
    outClass = ({
        let mut lveStarted: bool = false;
        (match isDir {
            false => {
                let mut pd: ArcStr;
                let mut encoding: ArcStr;
                let mut encodingfile: ArcStr;
                let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
                let mut strategy: LoadFileStrategy;
                pd = arcstr::literal!(Autoconf::pathDelimiter);
                encodingfile = stringAppendList(list![path.clone(), pd.clone(), literal!("package.encoding")]);
                encoding = System::trimChar(
                    System::trimChar(
                        if (System::regularFileExists(encodingfile.clone())) {
                            System::readFile(encodingfile)?
                        } else {
                            optEncoding.unwrap_or(literal!("UTF-8"))
                        },
                        literal!("\n"),
                    )?,
                    literal!(" "),
                )?;
                strategy = LoadFileStrategy::STRATEGY_ON_DEMAND { encoding: encoding };
                cl = parsePackageFile(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*path);
                        __mm_s.push_str(&*pd);
                        __mm_s.push_str(&*name);
                        ArcStr::from(__mm_s)
                    },
                    &strategy,
                    false,
                    &(openmodelica_ast::Absyn::Within::TOP),
                    id,
                    encrypted,
                )?;
                cl
            }
            true => {
                let mut pd: ArcStr;
                let mut encoding: ArcStr;
                let mut encodingfile: ArcStr;
                let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
                let mut filenames: metamodelica::List<ArcStr>;
                let mut strategy: LoadFileStrategy;
                let mut lveInstance: Option<i32>;
                pd = arcstr::literal!(Autoconf::pathDelimiter);
                encodingfile = stringAppendList(list![
                    path.clone(),
                    pd.clone(),
                    name.clone(),
                    pd.clone(),
                    literal!("package.encoding")
                ]);
                encoding = System::trimChar(
                    System::trimChar(
                        if (System::regularFileExists(encodingfile.clone())) {
                            System::readFile(encodingfile)?
                        } else {
                            optEncoding.unwrap_or(literal!("UTF-8"))
                        },
                        literal!("\n"),
                    )?,
                    literal!(" "),
                )?;
                lveInstance = None;
                if encrypted {
                    (lveStarted, lveInstance) = Parser::startLibraryVendorExecutable({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*path);
                        __mm_s.push_str(&*pd);
                        __mm_s.push_str(&*name);
                        ArcStr::from(__mm_s)
                    });
                    if !(lveStarted) {
                        Error::addMessage(
                            Error::INTERNAL_ERROR.clone(),
                            list![literal!("Unable to start library vendor executable.")],
                        )?;
                        return Err("fail");
                    }
                }
                if (Testsuite::isRunning()? || Config::noProc()? == 1) && !(encrypted) {
                    strategy = LoadFileStrategy::STRATEGY_ON_DEMAND { encoding: encoding };
                } else {
                    filenames = getAllFilesFromDirectory(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*path);
                            __mm_s.push_str(&*pd);
                            __mm_s.push_str(&*name);
                            ArcStr::from(__mm_s)
                        },
                        encrypted,
                        metamodelica::nil(),
                    )?;
                    strategy = LoadFileStrategy::STRATEGY_HASHTABLE {
                        ht: Parser::parallelParseFiles(
                            filenames,
                            encoding,
                            Config::noProc()?,
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*path);
                                __mm_s.push_str(&*pd);
                                __mm_s.push_str(&*name);
                                ArcStr::from(__mm_s)
                            },
                            lveInstance.clone(),
                        )?,
                    };
                }
                cl = loadCompletePackageFromMp(
                    id,
                    name,
                    path,
                    strategy,
                    openmodelica_ast::Absyn::Within::TOP,
                    Error::getNumErrorMessages(),
                    encrypted,
                )?;
                if encrypted && lveStarted {
                    Parser::stopLibraryVendorExecutable(lveInstance);
                }
                cl
            }
        })
    });
    Ok(outClass)
}

fn getAllFilesFromDirectory(
    mut dir: ArcStr,
    mut encrypted: bool,
    mut acc: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut files: metamodelica::List<ArcStr>;
    let mut subdirs: metamodelica::List<ArcStr>;
    let mut pd: ArcStr = arcstr::literal!(Autoconf::pathDelimiter);
    if encrypted {
        files = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*literal!("package.moc"));
                ArcStr::from(__mm_s)
            },
            listAppend(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut f in (System::mocFiles(dir.clone())).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*dir);
                            __mm_s.push_str(&*pd);
                            __mm_s.push_str(&*f);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                acc,
            ),
        );
    } else {
        files = metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*literal!("package.mo"));
                ArcStr::from(__mm_s)
            },
            listAppend(
                ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut f in (System::moFiles(dir.clone())).into_iter().cloned() {
                        let __x = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*dir);
                            __mm_s.push_str(&*pd);
                            __mm_s.push_str(&*f);
                            ArcStr::from(__mm_s)
                        };
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                acc,
            ),
        );
    }
    subdirs = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut d in (List::filter2OnTrue(
            System::subDirectories(dir.clone()),
            (std::sync::Arc::new(
                move |__a0: ArcStr, __a1: ArcStr, __a2: bool| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(existPackage(&__a0, &__a1, __a2))
                },
            ) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, bool) -> Result<bool> + 'static>),
            dir.clone(),
            encrypted,
        )?)
        .into_iter()
        .cloned()
        {
            let __x = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*d);
                ArcStr::from(__mm_s)
            };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    files = List::fold1(&subdirs, &getAllFilesFromDirectory, encrypted, files)?;
    Ok(files)
}

fn loadCompletePackageFromMp(
    mut id: ArcStr,
    mut inIdent: ArcStr,
    mut inString: ArcStr,
    mut strategy: LoadFileStrategy,
    mut inWithin: Absyn::Within,
    mut numError: i32,
    mut encrypted: bool,
) -> Result<Option<metamodelica::Ref<Absyn::Class>>> {
    let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
    cl = 'mc: {
        let __mc_input = (inIdent, inString, inWithin);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut pack, mut mp, mut within_) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut pd: ArcStr;
            let mut mp_1: ArcStr;
            let mut packagefile: ArcStr;
            let mut orderfile: ArcStr;
            let mut tv: metamodelica::List<ArcStr>;
            let mut ca: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
            let mut cp: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut cmt: Option<ArcStr>;
            let mut opt_cl: Option<metamodelica::Ref<Absyn::Class>>;
            let mut class_: metamodelica::Ref<Absyn::Class>;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut w2: Absyn::Within;
            let mut reverseOrder: metamodelica::List<PackageOrder>;
            let mut ann: metamodelica::List<metamodelica::Ref<Absyn::Annotation>>;
            pd = arcstr::literal!(Autoconf::pathDelimiter);
            mp_1 = stringAppendList(list![mp.clone(), pd.clone(), pack.clone()]);
            packagefile = stringAppendList(list![
                mp_1.clone(),
                pd.clone(),
                if (encrypted) {
                    literal!("package.moc")
                } else {
                    literal!("package.mo")
                }
            ]);
            orderfile = stringAppendList(list![mp_1.clone(), pd.clone(), literal!("package.order")]);
            if !(System::regularFileExists(packagefile.clone())) {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Expected file "));
                        __mm_s.push_str(&*packagefile);
                        __mm_s.push_str(&*literal!(" to exist"));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("FrontEnd/ClassLoader.mo"),
                )?;
                return Err("fail");
            }
            opt_cl = parsePackageFile(
                packagefile.clone(),
                &strategy,
                true,
                &(within_.clone()),
                id.clone(),
                encrypted,
            )?;
            if (opt_cl).is_some() {
                let (__pa5, __pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(opt_cl.clone().ok_or("pattern mismatch")?) {
                    __pa5 @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars: __pa0, classAttrs: __pa1, classParts: __pa2, ann: __pa3, comment: __pa4 }, .. } => (__pa5.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                tv = metamodelica::Own::own(__pa0);
                ca = metamodelica::Own::own(__pa1);
                cp = metamodelica::Own::own(__pa2);
                ann = metamodelica::Own::own(__pa3);
                cmt = metamodelica::Own::own(__pa4);
                class_ = metamodelica::Own::own(__pa5);
                reverseOrder = getPackageContentNames(
                    &class_,
                    orderfile.clone(),
                    mp_1.clone(),
                    Error::getNumErrorMessages(),
                    encrypted,
                )?;
                path = AbsynUtil::joinWithinPath(
                    &(within_.clone()),
                    metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }),
                )?;
                w2 = Absyn::Within::WITHIN { path: path.clone() };
                cp = List::fold4(
                    &reverseOrder,
                    &move |__a0: PackageOrder,
                           __a1: ArcStr,
                           __a2: LoadFileStrategy,
                           __a3: Absyn::Within,
                           __a4: bool,
                           __a5: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>| {
                        loadCompletePackageFromMp2(&__a0, __a1, __a2, __a3, __a4, __a5)
                    },
                    mp_1.clone(),
                    strategy.clone(),
                    w2.clone(),
                    encrypted,
                    metamodelica::nil(),
                )?;
                assign_field!(
                    class_.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS {
                        typeVars: tv.clone(),
                        classAttrs: ca.clone(),
                        classParts: cp.clone(),
                        ann: ann.clone(),
                        comment: cmt.clone()
                    })
                );
                opt_cl = Some(class_.clone());
            }
            Ok(opt_cl.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut pack, mut mp, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (numError == Error::getNumErrorMessages()) else {
                return Err("pattern mismatch");
            };
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("loadCompletePackageFromMp failed for unknown reason: mp="));
                    __mm_s.push_str(&*mp);
                    __mm_s.push_str(&*literal!(" pack="));
                    __mm_s.push_str(&*pack);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/ClassLoader.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(cl)
}

fn mergeBefore(
    mut cp: metamodelica::Ref<Absyn::ClassPart>,
    mut cps: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> {
    let mut ocp: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    ocp = (::match_deref::match_deref! { match &((cp.clone(), cps.clone())) {
        (Deref @ Absyn::ClassPart::PUBLIC { contents: ei1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: ei2 }, tail: rest }) => {
            let mut ei: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            ei = listAppend(ei1.clone(), ei2.clone());
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: ei }), rest.clone())
        },
        (Deref @ Absyn::ClassPart::PROTECTED { contents: ei1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: ei2 }, tail: rest }) => {
            let mut ei: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            ei = listAppend(ei1.clone(), ei2.clone());
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: ei }), rest.clone())
        },
        _ => {
            metamodelica::cons(cp, cps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocp
}

fn loadCompletePackageFromMp2(
    mut po: &PackageOrder,
    mut mp: ArcStr,
    mut strategy: LoadFileStrategy,
    mut w1: Absyn::Within,
    mut encrypted: bool,
    mut acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut cps: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    cps = (match po.clone() {
        PackageOrder::CLASSPART { cp: mut cp } => {
            cps = mergeBefore(cp.clone(), acc);
            cps
        }
        PackageOrder::ELEMENT {
            element: ref ei,
            r#pub: true,
        } => {
            cps = mergeBefore(
                metamodelica::Ref::new(Absyn::ClassPart::PUBLIC {
                    contents: list![ei.clone()],
                }),
                acc,
            );
            cps
        }
        PackageOrder::ELEMENT {
            element: ref ei,
            r#pub: false,
        } => {
            cps = mergeBefore(
                metamodelica::Ref::new(Absyn::ClassPart::PROTECTED {
                    contents: list![ei.clone()],
                }),
                acc,
            );
            cps
        }
        PackageOrder::CLASSLOAD { cl: mut id } => {
            let mut ei: metamodelica::Ref<Absyn::ElementItem>;
            let mut pd: ArcStr;
            let mut file: ArcStr;
            let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
            let mut bDirectoryAndFileExists: bool;
            pd = arcstr::literal!(Autoconf::pathDelimiter);
            file = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*mp);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*id);
                __mm_s.push_str(&*if (encrypted) {
                    literal!("/package.moc")
                } else {
                    literal!("/package.mo")
                });
                ArcStr::from(__mm_s)
            };
            bDirectoryAndFileExists = System::directoryExists({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*mp);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*id);
                ArcStr::from(__mm_s)
            }) && System::regularFileExists(file);
            if bDirectoryAndFileExists {
                cl = loadCompletePackageFromMp(
                    id.clone(),
                    id.clone(),
                    mp,
                    strategy,
                    w1,
                    Error::getNumErrorMessages(),
                    encrypted,
                )?;
                if (cl).is_some() {
                    ei = AbsynUtil::makeClassElement(cl.ok_or("pattern mismatch")?);
                    cps = mergeBefore(
                        metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![ei] }),
                        acc,
                    );
                } else {
                    cps = acc;
                }
            } else {
                file = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*mp);
                    __mm_s.push_str(&*pd);
                    __mm_s.push_str(&*id);
                    __mm_s.push_str(&*if (encrypted) { literal!(".moc") } else { literal!(".mo") });
                    ArcStr::from(__mm_s)
                };
                if !(System::regularFileExists(file.clone())) {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Expected file "));
                            __mm_s.push_str(&*file);
                            __mm_s.push_str(&*literal!(" to exist"));
                            ArcStr::from(__mm_s)
                        },
                        metamodelica::sourceInfo!("FrontEnd/ClassLoader.mo"),
                    )?;
                    return Err("fail");
                }
                cl = parsePackageFile(file, &strategy, false, &w1, id.clone(), encrypted)?;
                if (cl).is_some() {
                    ei = AbsynUtil::makeClassElement(cl.ok_or("pattern mismatch")?);
                    cps = mergeBefore(
                        metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![ei] }),
                        acc,
                    );
                } else {
                    cps = acc;
                }
            }
            cps
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cps)
}

pub(crate) fn parsePackageFile(
    mut name: ArcStr,
    mut strategy: &LoadFileStrategy,
    mut expectPackage: bool,
    mut w1: &Absyn::Within,
    mut pack: ArcStr,
    mut encrypted: bool,
) -> Result<Option<metamodelica::Ref<Absyn::Class>>> {
    let mut cl: Option<metamodelica::Ref<Absyn::Class>>;
    let mut class_: metamodelica::Ref<Absyn::Class>;
    let mut cs: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut w2: Absyn::Within;
    let mut classNames: metamodelica::List<ArcStr>;
    let mut info: SourceInfo;
    let mut r#str: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut cname: ArcStr;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let Absyn::PROGRAM {
        classes: __pa0,
        within_: __pa1,
    } = getProgramFromStrategy(name.clone(), strategy)?;
    cs = metamodelica::Own::own(__pa0);
    w2 = metamodelica::Own::own(__pa1);
    classNames = List::map(
        cs.clone(),
        &move |__a0: metamodelica::Ref<Absyn::Class>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::getClassName(&__a0))
        },
    )?;
    r#str = stringDelimitList(classNames, literal!(", "));
    if !(((cs).len() as i32) == 1) {
        if encrypted {
            cl = None;
            return Ok(cl);
        } else {
            Error::addSourceMessage(
                &(Error::LIBRARY_ONE_PACKAGE_PER_FILE.clone()),
                list![r#str],
                &(SourceInfo {
                    fileName: name,
                    isReadOnly: true,
                    lineNumberStart: 0,
                    columnNumberStart: 0,
                    lineNumberEnd: 0,
                    columnNumberEnd: 0,
                    lastModification: metamodelica::OrderedFloat(0.0_f64),
                }),
            )?;
            return Err("fail");
        }
    }
    let (__pa5, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(cs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa5 @ Deref @ Absyn::Class { name: __pa2, body: __pa3, info: __pa4, .. }, tail: Deref @ metamodelica::ListNode::Nil } => (__pa5.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cname = metamodelica::Own::own(__pa2);
    body = metamodelica::Own::own(__pa3);
    info = metamodelica::Own::own(__pa4);
    class_ = metamodelica::Own::own(__pa5);
    cl = Some(class_);
    if !(stringEqual(&cname, &pack)) {
        if stringEqual(&(System::tolower(cname.clone())), &(System::tolower(pack.clone()))) {
            Error::addSourceMessage(
                &(Error::LIBRARY_UNEXPECTED_NAME_CASE_SENSITIVE.clone()),
                list![pack.clone(), cname],
                &info,
            )?;
        } else {
            Error::addSourceMessage(
                &(Error::LIBRARY_UNEXPECTED_NAME.clone()),
                list![pack.clone(), cname],
                &info,
            )?;
            return Err("fail");
        }
    }
    if expectPackage && !(AbsynUtil::isParts(&body)) {
        Error::addSourceMessage(&(Error::LIBRARY_EXPECTED_PARTS.clone()), list![pack], &info)?;
        return Err("fail");
    } else if !(AbsynUtil::withinEqual(w1, &w2)
        || Config::languageStandardAtMost(Config::LanguageStandard::_2_x.clone())?)
    {
        s1 = AbsynUtil::withinString(w1)?;
        s2 = AbsynUtil::withinString(&w2)?;
        if AbsynUtil::withinEqualCaseInsensitive(w1, &w2) {
            Error::addSourceMessage(&(Error::LIBRARY_WITHIN_WRONG_CASE.clone()), list![s1, s2], &info)?;
        } else {
            Error::addSourceMessage(&(Error::LIBRARY_UNEXPECTED_WITHIN.clone()), list![s1, s2], &info)?;
            return Err("fail");
        }
    }
    Ok(cl)
}

fn getBothPackageAndFilename(mut r#str: &ArcStr, mut mp: &ArcStr) -> Result<ArcStr> {
    let mut out: ArcStr;
    out = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Testsuite::friendly(System::realpath({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*mp);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(".mo"));
            ArcStr::from(__mm_s)
        })?)?);
        __mm_s.push_str(&*literal!(", "));
        __mm_s.push_str(&*Testsuite::friendly(System::realpath({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*mp);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("/package.mo"));
            ArcStr::from(__mm_s)
        })?)?);
        ArcStr::from(__mm_s)
    };
    Ok(out)
}

fn getPackageContentNames(
    mut cl: &metamodelica::Ref<Absyn::Class>,
    mut filename: ArcStr,
    mut mp: ArcStr,
    mut numError: i32,
    mut encrypted: bool,
) -> Result<metamodelica::List<PackageOrder>> {
    let mut po: metamodelica::List<PackageOrder> = metamodelica::nil();
    po = 'mc: {
        let __mc_input = &**cl;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: cp, .. }, info, .. } => {
                    let mut contents: ArcStr;
                    let mut duplicatesStr: ArcStr;
                    let mut differencesStr: ArcStr;
                    let mut duplicates: metamodelica::List<ArcStr>;
                    let mut namesToFind: metamodelica::List<ArcStr>;
                    let mut mofiles: metamodelica::List<ArcStr>;
                    let mut subdirs: metamodelica::List<ArcStr>;
                    let mut differences: metamodelica::List<ArcStr>;
                    let mut intersection: metamodelica::List<ArcStr>;
                    let mut po1: metamodelica::List<PackageOrder>;
                    let mut po2: metamodelica::List<PackageOrder>;
                    let mut po: metamodelica::List<PackageOrder> = po.clone();
                    match '__try0: {
                        let true = (System::regularFileExists(filename.clone())) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        contents = unwrap_break_err!(System::readFile(filename.clone()), '__try0);
                        namesToFind = System::strtok(contents.clone(), literal!("\n"));
                        namesToFind = unwrap_break_err!(List::removeOnTrue(literal!(""), &fnptr!(stringEqual, ArcStr, ArcStr), unwrap_break_err!(List::map(namesToFind.clone(), &fnptr!(System::trimWhitespace, ArcStr)), '__try0)), '__try0);
                        duplicates = unwrap_break_err!(List::sortedDuplicates(unwrap_break_err!(List::sort(namesToFind.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0), &fnptr!(stringEq, ArcStr, ArcStr)), '__try0);
                        duplicatesStr = stringDelimitList(duplicates.clone(), literal!(", "));
                        unwrap_break_err!(Error::assertionOrAddSourceMessage((duplicates).is_empty(), &(Error::PACKAGE_ORDER_DUPLICATES.clone()), list![duplicatesStr.clone()], &(SourceInfo { fileName: filename.clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) })), '__try0);
                        if encrypted {
                            mofiles = unwrap_break_err!(List::map(System::mocFiles(mp.clone()), &Util::removeLast4Char), '__try0);
                        } else {
                            mofiles = unwrap_break_err!(List::map(System::moFiles(mp.clone()), &Util::removeLast3Char), '__try0);
                        }
                        subdirs = System::subDirectories(mp.clone());
                        subdirs = unwrap_break_err!(List::filter2OnTrue(subdirs.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr, __a2: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(existPackage(&__a0, &__a1, __a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, bool) -> Result<bool> + 'static>), mp.clone(), encrypted), '__try0);
                        intersection = unwrap_break_err!(List::intersectionOnTrue(&subdirs, &mofiles, &fnptr!(stringEq, ArcStr, ArcStr)), '__try0);
                        differencesStr = stringDelimitList(unwrap_break_err!(List::map1(intersection.clone(), &move |__a0: ArcStr, __a1: ArcStr| getBothPackageAndFilename(&__a0, &__a1), mp.clone()), '__try0), literal!(", "));
                        unwrap_break_err!(Error::assertionOrAddSourceMessage((intersection).is_empty(), &(Error::PACKAGE_DUPLICATE_CHILDREN.clone()), list![differencesStr.clone()], &(SourceInfo { fileName: filename.clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) })), '__try0);
                        mofiles = listAppend(subdirs.clone(), mofiles.clone());
                        differences = unwrap_break_err!(List::setDifference(mofiles.clone(), &namesToFind), '__try0);
                        po1 = unwrap_break_err!(getPackageContentNamesinParts(namesToFind.clone(), cp.clone(), metamodelica::nil()), '__try0);
                        (po1, differences) = unwrap_break_err!(List::map3Fold(&po1, &move |__a0: PackageOrder, __a1: ArcStr, __a2: SourceInfo, __a3: bool, __a4: metamodelica::List<ArcStr>| checkPackageOrderFilesExist(__a0, __a1, &__a2, __a3, __a4), mp.clone(), info.clone(), encrypted, differences.clone()), '__try0);
                        differencesStr = stringDelimitList(differences.clone(), literal!("\n\t"));
                        unwrap_break_err!(Error::assertionOrAddSourceMessage((differences).is_empty(), &(Error::PACKAGE_ORDER_FILE_NOT_COMPLETE.clone()), list![differencesStr.clone()], &(SourceInfo { fileName: filename.clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) })), '__try0);
                        po2 = unwrap_break_err!(List::map(differences.clone(), &fnptr!(makeClassLoad, ArcStr)), '__try0);
                        po = listAppend(po2.clone(), po1.clone());
                        Ok::<_, &'static str>((differencesStr.clone(), intersection.clone(), mofiles.clone(), po.clone(), subdirs.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
                            differencesStr = __try0_o0;
                            intersection = __try0_o1;
                            mofiles = __try0_o2;
                            po = __try0_o3;
                            subdirs = __try0_o4;
                        }
                        Err(_) => {
                            mofiles = List::map(System::moFiles(mp.clone()), &Util::removeLast3Char)?;
                            subdirs = System::subDirectories(mp.clone());
                            subdirs = List::filter2OnTrue(subdirs.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr, __a2: bool| -> metamodelica::Result<_> { ::std::result::Result::Ok(existPackage(&__a0, &__a1, __a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, bool) -> Result<bool> + 'static>), mp.clone(), encrypted)?;
                            mofiles = List::sort(listAppend(subdirs.clone(), mofiles.clone()), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
                            intersection = List::sortedDuplicates(mofiles.clone(), &fnptr!(stringEq, ArcStr, ArcStr))?;
                            differencesStr = stringDelimitList(List::map1(intersection.clone(), &move |__a0: ArcStr, __a1: ArcStr| getBothPackageAndFilename(&__a0, &__a1), mp.clone())?, literal!(", "));
                            Error::assertionOrAddSourceMessage((intersection).is_empty(), &(Error::PACKAGE_DUPLICATE_CHILDREN.clone()), list![differencesStr.clone()], metamodelica::AsArg::as_arg(&info))?;
                            po = listAppend(List::map(cp.clone(), &fnptr!(makeClassPart, metamodelica::Ref<Absyn::ClassPart>))?, List::map(mofiles.clone(), &fnptr!(makeClassLoad, ArcStr))?);
                        }
                    }
                    Ok((po.clone(), po.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            po = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { info, .. } => {
                    let true = (numError == Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("getPackageContentNames failed for unknown reason")], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(po)
}

fn makeClassPart(mut part: metamodelica::Ref<Absyn::ClassPart>) -> PackageOrder {
    let mut po: PackageOrder;
    po = PackageOrder::CLASSPART { cp: part };
    po
}

fn makeElement(mut el: metamodelica::Ref<Absyn::ElementItem>, mut r#pub: bool) -> PackageOrder {
    let mut po: PackageOrder;
    po = PackageOrder::ELEMENT {
        element: el,
        r#pub: r#pub,
    };
    po
}

fn makeClassLoad(mut r#str: ArcStr) -> PackageOrder {
    let mut po: PackageOrder;
    po = PackageOrder::CLASSLOAD { cl: r#str };
    po
}

fn checkPackageOrderFilesExist(
    mut po: PackageOrder,
    mut mp: ArcStr,
    mut info: &SourceInfo,
    mut encrypted: bool,
    mut differences: metamodelica::List<ArcStr>,
) -> Result<(PackageOrder, metamodelica::List<ArcStr>)> {
    let mut po: PackageOrder = po;
    let mut differences: metamodelica::List<ArcStr> = differences;
    let () = (match po.clone() {
        PackageOrder::CLASSLOAD { cl: mut r#str } => {
            let mut pd: ArcStr;
            let mut str2: ArcStr;
            let mut str3: ArcStr;
            let mut str4: ArcStr;
            pd = arcstr::literal!(Autoconf::pathDelimiter);
            str2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*if (encrypted) { literal!(".moc") } else { literal!(".mo") });
                ArcStr::from(__mm_s)
            };
            if !(System::directoryExists({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*mp);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            }) || System::regularFileExists({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*mp);
                __mm_s.push_str(&*pd);
                __mm_s.push_str(&*str2);
                ArcStr::from(__mm_s)
            })) {
                if let Ok(__iflet0) = List::find(
                    &(System::moFiles(mp.clone())),
                    &({
                        let __pe_b1 = System::tolower(str2.clone());
                        move |__pe_a0| Ok(Util::stringEqCaseInsensitive(__pe_a0, __pe_b1.clone()))
                    }),
                ) {
                    str3 = __iflet0;
                } else {
                    Error::addSourceMessage(
                        &(Error::PACKAGE_ORDER_FILE_NOT_FOUND.clone()),
                        list![r#str.clone()],
                        info,
                    )?;
                    return Err("fail");
                }
                Error::addSourceMessage(
                    &(Error::PACKAGE_ORDER_CASE_SENSITIVE.clone()),
                    list![r#str.clone(), str2, str3.clone()],
                    info,
                )?;
                str4 = Util::removeLastNChar(str3, if (encrypted) { 4 } else { 3 })?;
                differences = List::removeOnTrue(str4.clone(), &fnptr!(stringEq, ArcStr, ArcStr), differences)?;
                po = PackageOrder::CLASSLOAD { cl: str4 };
            }
            ()
        }
        _ => (),
    });
    Ok((po, differences))
}

fn existPackage(mut name: &ArcStr, mut mp: &ArcStr, mut encrypted: bool) -> bool {
    let mut b: bool;
    let mut pd: ArcStr;
    pd = arcstr::literal!(Autoconf::pathDelimiter);
    b = System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*mp);
        __mm_s.push_str(&*pd);
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*pd);
        __mm_s.push_str(&*if (encrypted) {
            literal!("package.moc")
        } else {
            literal!("package.mo")
        });
        ArcStr::from(__mm_s)
    });
    b
}

fn getPackageContentNamesinParts(
    mut inNamesToSort: metamodelica::List<ArcStr>,
    mut cps: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut acc: metamodelica::List<PackageOrder>,
) -> Result<metamodelica::List<PackageOrder>> {
    let mut outOrder: metamodelica::List<PackageOrder>;
    outOrder = (::match_deref::match_deref! { match &(cps) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut namesToSort = inNamesToSort;
            outOrder = listAppend(List::mapReverse(namesToSort, &fnptr!(makeClassLoad, ArcStr))?, acc);
            outOrder
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rcp } => {
            let mut namesToSort = inNamesToSort;
            (outOrder, namesToSort) = getPackageContentNamesinElts(namesToSort, elts.clone(), acc, true)?;
            outOrder = getPackageContentNamesinParts(namesToSort, rcp.clone(), outOrder)?;
            outOrder
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rcp } => {
            let mut namesToSort = inNamesToSort;
            (outOrder, namesToSort) = getPackageContentNamesinElts(namesToSort, elts.clone(), acc, false)?;
            outOrder = getPackageContentNamesinParts(namesToSort, rcp.clone(), outOrder)?;
            outOrder
        },
        Deref @ metamodelica::ListNode::Cons { head: cp, tail: rcp } => {
            let mut namesToSort = inNamesToSort;
            outOrder = getPackageContentNamesinParts(namesToSort, rcp.clone(), metamodelica::cons(PackageOrder::CLASSPART { cp: cp.clone() }, acc))?;
            outOrder
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOrder)
}

fn getPackageContentNamesinElts(
    mut inNamesToSort: metamodelica::List<ArcStr>,
    mut inElts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut po: metamodelica::List<PackageOrder>,
    mut r#pub: bool,
) -> Result<(metamodelica::List<PackageOrder>, metamodelica::List<ArcStr>)> {
    let mut outOrder: metamodelica::List<PackageOrder>;
    let mut outNames: metamodelica::List<ArcStr>;
    (outOrder, outNames) = (::match_deref::match_deref! { match &((inNamesToSort.clone(), inElts.clone())) {
        (namesToSort, Deref @ metamodelica::ListNode::Nil) => {
            (po, namesToSort.clone())
        },
        (Deref @ metamodelica::ListNode::Cons { head: name1, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: ei @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, .. }, info, .. } }, tail: elts }) => {
            let mut names: metamodelica::List<ArcStr>;
            let mut compNames: metamodelica::List<ArcStr>;
            let mut b: bool;
            let mut orderElt: PackageOrder;
            compNames = List::map(comps.clone(), &move |__a0: metamodelica::Ref<Absyn::ComponentItem>| AbsynUtil::componentName(&__a0))?;
            (names, b) = matchCompNames(&inNamesToSort, &compNames, metamodelica::AsArg::as_arg(&info))?;
            orderElt = if (b) {makeElement(ei.clone(), r#pub)} else {makeClassLoad(name1.clone())};
            (outOrder, names) = getPackageContentNamesinElts(names, if (b) {elts.clone()} else {inElts}, metamodelica::cons(orderElt, po), r#pub)?;
            (outOrder, names)
        },
        (Deref @ metamodelica::ListNode::Cons { head: name1, tail: namesToSort }, Deref @ metamodelica::ListNode::Cons { head: ei @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: name2, info, .. }, .. }, .. } }, tail: elts }) => {
            let mut names: metamodelica::List<ArcStr>;
            let mut b: bool;
            let mut orderElt: PackageOrder;
            let mut load: PackageOrder;
            load = makeClassLoad(name1.clone());
            b = metamodelica::stringEq(&name1, &name2);
            Error::assertionOrAddSourceMessage(if (b) {!(listMember(load.clone(), po.clone()))} else {true}, &(Error::PACKAGE_MO_NOT_IN_ORDER.clone()), list![name2.clone()], metamodelica::AsArg::as_arg(&info))?;
            orderElt = if (b) {makeElement(ei.clone(), r#pub)} else {load};
            (outOrder, names) = getPackageContentNamesinElts(namesToSort.clone(), if (b) {elts.clone()} else {inElts}, metamodelica::cons(orderElt, po), r#pub)?;
            (outOrder, names)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: name2, info, .. }, .. }, .. } }, tail: _ }) => {
            let mut names: metamodelica::List<ArcStr>;
            let mut load: PackageOrder;
            load = makeClassLoad(name2.clone());
            Error::assertionOrAddSourceMessage(!(listMember(load, po.clone())), &(Error::PACKAGE_MO_NOT_IN_ORDER.clone()), list![name2.clone()], metamodelica::AsArg::as_arg(&info))?;
            Error::addSourceMessage(&(Error::FOUND_ELEMENT_NOT_IN_ORDER_FILE.clone()), list![name2.clone()], metamodelica::AsArg::as_arg(&info))?;
            (outOrder, names) = getPackageContentNamesinElts(metamodelica::cons(name2.clone(), inNamesToSort), inElts, po, r#pub)?;
            (outOrder, names)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. }, tail: _ }, .. }, info, .. } }, tail: _ }) => {
            let mut names: metamodelica::List<ArcStr>;
            let mut load: PackageOrder;
            load = makeClassLoad(name2.clone());
            Error::assertionOrAddSourceMessage(!(listMember(load, po.clone())), &(Error::PACKAGE_MO_NOT_IN_ORDER.clone()), list![name2.clone()], metamodelica::AsArg::as_arg(&info))?;
            Error::addSourceMessage(&(Error::FOUND_ELEMENT_NOT_IN_ORDER_FILE.clone()), list![name2.clone()], metamodelica::AsArg::as_arg(&info))?;
            (outOrder, names) = getPackageContentNamesinElts(metamodelica::cons(name2.clone(), inNamesToSort), inElts, po, r#pub)?;
            (outOrder, names)
        },
        (namesToSort, Deref @ metamodelica::ListNode::Cons { head: ei, tail: elts }) => {
            let mut names: metamodelica::List<ArcStr>;
            (outOrder, names) = getPackageContentNamesinElts(namesToSort.clone(), elts.clone(), metamodelica::cons(PackageOrder::ELEMENT { element: ei.clone(), r#pub: r#pub }, po), r#pub)?;
            (outOrder, names)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outOrder, outNames))
}

fn matchCompNames(
    mut names: &metamodelica::List<ArcStr>,
    mut comps: &metamodelica::List<ArcStr>,
    mut info: &SourceInfo,
) -> Result<(metamodelica::List<ArcStr>, bool)> {
    let mut outNames: metamodelica::List<ArcStr>;
    let mut matchedNames: bool;
    (outNames, matchedNames) = (::match_deref::match_deref! { match (names, comps) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            (names.clone(), true)
        },
        (Deref @ metamodelica::ListNode::Cons { head: n1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: n2, tail: rest2 }) => {
            let mut b: bool;
            let mut b1: bool;
            let mut rest1 = (*rest1).clone();
            if metamodelica::stringEq(&n1, &n2) {
                (rest1, b) = matchCompNames(metamodelica::AsArg::as_arg(&rest1), rest2, info)?;
                Error::assertionOrAddSourceMessage(b, &(Error::ORDER_FILE_COMPONENTS.clone()), metamodelica::nil(), info)?;
                b1 = true;
            } else {
                b1 = false;
            }
            (rest1.clone(), b1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outNames, matchedNames))
}

fn packageOrderName(mut ord: &PackageOrder) -> ArcStr {
    let mut name: ArcStr;
    name = (match ord.clone() {
        PackageOrder::CLASSLOAD { cl: mut __esc_name } => {
            name = __esc_name.clone();
            name
        }
        _ => literal!("#"),
    });
    name
}

pub fn checkOnLoadMessage(mut p1: Absyn::Program) -> Result<()> {
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let Absyn::PROGRAM { classes: __pa0, .. } = p1;
    classes = metamodelica::Own::own(__pa0);
    List::map2(
        classes,
        &move |__a0: metamodelica::Ref<Absyn::Class>,
               __a1: metamodelica::Ref<Absyn::Path>,
               __a2: _|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::getNamedAnnotationInClass(
                &__a0,
                &__a1,
                metamodelica::arc_ref(&__a2),
            ))
        },
        metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("__OpenModelica_messageOnLoad"),
        }),
        (std::sync::Arc::new(checkOnLoadMessageWork)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Option<metamodelica::Ref<Absyn::Modification>>) -> Result<i32> + 'static,
            >),
    )?;
    Ok(())
}

fn checkOnLoadMessageWork(mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>) -> Result<i32> {
    let mut dummy: i32;
    dummy = (::match_deref::match_deref! { match &(r#mod) {
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { info, exp: Deref @ Absyn::Exp::STRING { value: r#str } }, .. }) => {
            Error::addSourceMessage(&(Error::COMPILER_NOTIFICATION_SCRIPTING.clone()), list![r#str.clone()], metamodelica::AsArg::as_arg(&info))?;
            1
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(dummy)
}

fn getProgramFromStrategy(mut filename: ArcStr, mut strategy: &LoadFileStrategy) -> Result<Absyn::Program> {
    let mut program: Absyn::Program;
    let mut f: ArcStr = filename.clone();
    program = (match strategy.clone() {
        LoadFileStrategy::STRATEGY_HASHTABLE { .. } => {
            if !(BaseHashTable::hasKey(
                filename.clone(),
                &(var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone()),
            )?) {
                if let Ok(__iflet0) = List::getMemberOnTrue(
                    filename.clone(),
                    &(BaseHashTable::hashTableKeyList(
                        &(var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone()),
                    )?),
                    &fnptr!(Util::stringEqCaseInsensitive, ArcStr, ArcStr),
                ) {
                    f = __iflet0;
                } else {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("HashTable missing file: "));
                            __mm_s.push_str(&*filename);
                            __mm_s.push_str(&*literal!(" - all entries include:\n"));
                            __mm_s.push_str(&*stringDelimitList(
                                BaseHashTable::hashTableKeyList(
                                    &(var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone()),
                                )?,
                                literal!("\n"),
                            ));
                            ArcStr::from(__mm_s)
                        },
                        metamodelica::sourceInfo!("FrontEnd/ClassLoader.mo"),
                    )?;
                    return Err("fail");
                }
            }
            BaseHashTable::get(
                f,
                &(var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone()),
            )?
        }
        LoadFileStrategy::STRATEGY_ON_DEMAND { .. } => Parser::parse(
            filename,
            var_field!(strategy.encoding, LoadFileStrategy::STRATEGY_ON_DEMAND).clone(),
            literal!(""),
            None,
            Config::acceptedGrammar()?,
            Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
            Flags::getConfigBool(Flags::STRICT.clone())?,
        )?,
    });
    Ok(program)
}
