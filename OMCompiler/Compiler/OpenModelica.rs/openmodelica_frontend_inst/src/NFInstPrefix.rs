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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Prefix {
    EMPTY_PREFIX {
        /// The path of the class the prefix originates from.
        classPath: Option<metamodelica::Ref<Absyn::Path>>,
    },
    PREFIX {
        name: ArcStr,
        dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
        restPrefix: metamodelica::Ref<Prefix>,
    },
}
impl metamodelica::gc::MMTrace for Prefix {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Prefix::EMPTY_PREFIX { classPath } => {
                metamodelica::gc::MMTrace::mm_accept(classPath, __mmv)?;
                Ok(())
            }
            Prefix::PREFIX { name, dims, restPrefix } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restPrefix, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Prefix {
    fn default() -> Self {
        Self::EMPTY_PREFIX {
            classPath: Default::default(),
        }
    }
}
pub use self::Prefix::{EMPTY_PREFIX, PREFIX};

thread_local! { static __emptyPrefix_TLS: metamodelica::Ref<Prefix> = metamodelica::Ref::new(Prefix::EMPTY_PREFIX { classPath: None }); }
pub fn emptyPrefix() -> metamodelica::Ref<Prefix> {
    __emptyPrefix_TLS.with(|__t| __t.clone())
}

thread_local! { static __functionPrefix_TLS: metamodelica::Ref<Prefix> = metamodelica::Ref::new(Prefix::EMPTY_PREFIX { classPath: None }); }
pub(crate) fn functionPrefix() -> metamodelica::Ref<Prefix> {
    __functionPrefix_TLS.with(|__t| __t.clone())
}

pub(crate) fn makePrefix(
    mut inName: ArcStr,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = metamodelica::Ref::new(Prefix::PREFIX {
        name: inName,
        dims: inDims,
        restPrefix: emptyPrefix().clone(),
    });
    outPrefix
}

pub(crate) fn makeEmptyPrefix(mut inClassPath: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = metamodelica::Ref::new(Prefix::EMPTY_PREFIX {
        classPath: Some(inClassPath),
    });
    outPrefix
}

pub(crate) fn add(
    mut inName: ArcStr,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inPrefix: metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = metamodelica::Ref::new(Prefix::PREFIX {
        name: inName,
        dims: inDimensions,
        restPrefix: inPrefix,
    });
    outPrefix
}

pub(crate) fn addPath(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inPrefix: metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = fromPath2(inPath, inPrefix);
    outPrefix
}

pub(crate) fn addOptPath(
    mut inOptPath: Option<metamodelica::Ref<Absyn::Path>>,
    mut inPrefix: metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = (::match_deref::match_deref! { match &(inOptPath) {
        None => {
            inPrefix
        },
        Some(p) => {
            addPath(metamodelica::AsArg::as_arg(&p), inPrefix)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPrefix
}

pub(crate) fn addString(mut inName: ArcStr, mut inPrefix: metamodelica::Ref<Prefix>) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = metamodelica::Ref::new(Prefix::PREFIX {
        name: inName,
        dims: metamodelica::nil(),
        restPrefix: inPrefix,
    });
    outPrefix
}

pub(crate) fn addStringList(
    mut inStrings: &metamodelica::List<ArcStr>,
    mut inPrefix: metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = fromStringList2(inStrings, inPrefix);
    outPrefix
}

pub(crate) fn restPrefix(mut inPrefix: &metamodelica::Ref<Prefix>) -> Result<metamodelica::Ref<Prefix>> {
    let mut outRestPrefix: metamodelica::Ref<Prefix>;
    let __pa0 = ::match_deref::match_deref! { match &((*inPrefix)) {
        Deref @ Prefix::PREFIX { restPrefix: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outRestPrefix = metamodelica::Own::own(__pa0);
    Ok(outRestPrefix)
}

pub(crate) fn firstName(mut inPrefix: &metamodelica::Ref<Prefix>) -> ArcStr {
    let mut outStr: ArcStr;
    outStr = (match &**inPrefix {
        Prefix::EMPTY_PREFIX { .. } => {
            literal!("")
        }
        Prefix::PREFIX { name, .. } => name.clone(),
    });
    outStr
}

pub(crate) fn prefixCref<'__b>(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inPrefix: &'__b metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    '__tco: loop {
        match &**inPrefix {
            Prefix::EMPTY_PREFIX { .. } => return inCref,
            Prefix::PREFIX {
                name,
                restPrefix: rest_prefix,
                ..
            } => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                cref = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: name.clone(),
                    identType: DAE::T_UNKNOWN_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil(),
                    componentRef: inCref,
                });
                {
                    (inCref, inPrefix) = (cref, rest_prefix);
                    continue '__tco;
                }
            }
        }
    }
}

pub(crate) fn prefixPath<'__b>(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inPrefix: &'__b metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Absyn::Path> {
    '__tco: loop {
        match &**inPrefix {
            Prefix::EMPTY_PREFIX { .. } => return inPath,
            Prefix::PREFIX {
                name,
                restPrefix: rest_prefix,
                ..
            } => {
                let mut path: metamodelica::Ref<Absyn::Path>;
                path = metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: name.clone(),
                    path: inPath,
                });
                {
                    (inPath, inPrefix) = (path, rest_prefix);
                    continue '__tco;
                }
            }
        }
    }
}

pub(crate) fn prefixStr(mut inString: ArcStr, mut inPrefix: &metamodelica::Ref<Prefix>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match &**inPrefix {
        Prefix::EMPTY_PREFIX { .. } => inString,
        _ => {
            let mut r#str: ArcStr;
            r#str = toStr(inPrefix);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*inString);
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    outString
}

pub(crate) fn toCref(mut inPrefix: &metamodelica::Ref<Prefix>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match inPrefix {
        Deref @ Prefix::PREFIX { name, restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, .. } => {
            metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() })
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            cref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: DAE::T_UNKNOWN_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            prefixCref(cref, rest_prefix)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outCref)
}

pub(crate) fn toPath(mut inPrefix: &metamodelica::Ref<Prefix>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match inPrefix {
        Deref @ Prefix::PREFIX { name, restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, .. } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() })
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut path: metamodelica::Ref<Absyn::Path>;
            path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() });
            prefixPath(path, rest_prefix)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPath)
}

pub(crate) fn fromPath(mut inPath: &metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = fromPath2(inPath, emptyPrefix().clone());
    outPrefix
}

fn fromPath2<'__b>(
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
    mut inPrefix: metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Prefix> {
    '__tco: loop {
        match &**inPath {
            Absyn::Path::QUALIFIED { name, path } => {
                (inPath, inPrefix) = (
                    path,
                    metamodelica::Ref::new(Prefix::PREFIX {
                        name: name.clone(),
                        dims: metamodelica::nil(),
                        restPrefix: inPrefix,
                    }),
                );
                continue '__tco;
            }
            Absyn::Path::IDENT { name } => {
                return metamodelica::Ref::new(Prefix::PREFIX {
                    name: name.clone(),
                    dims: metamodelica::nil(),
                    restPrefix: inPrefix,
                });
            }
            Absyn::Path::FULLYQUALIFIED { path } => {
                (inPath, inPrefix) = (path, inPrefix);
                continue '__tco;
            }
        }
    }
}

pub(crate) fn fromStringList(mut inStrings: &metamodelica::List<ArcStr>) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = fromStringList2(inStrings, emptyPrefix().clone());
    outPrefix
}

fn fromStringList2<'__b>(
    mut inStrings: &'__b metamodelica::List<ArcStr>,
    mut inPrefix: metamodelica::Ref<Prefix>,
) -> metamodelica::Ref<Prefix> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStrings {
            Deref @ metamodelica::ListNode::Cons { head: r#str, tail: strl } => {
                { (inStrings, inPrefix) = (strl, metamodelica::Ref::new(Prefix::PREFIX { name: r#str.clone(), dims: metamodelica::nil(), restPrefix: inPrefix })); continue '__tco; }
            },
            _ => {
                return inPrefix
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn toStr(mut inPrefix: &metamodelica::Ref<Prefix>) -> ArcStr {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match inPrefix {
        Deref @ Prefix::EMPTY_PREFIX { .. } => {
            literal!("")
        },
        Deref @ Prefix::PREFIX { name, restPrefix: Deref @ Prefix::EMPTY_PREFIX { .. }, .. } => {
            name.clone()
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*toStr(rest_prefix)); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStr
}

pub(crate) fn toStrWithEmpty(mut inPrefix: &metamodelica::Ref<Prefix>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (::match_deref::match_deref! { match inPrefix {
        Deref @ Prefix::EMPTY_PREFIX { classPath: None } => {
            literal!("E()")
        },
        Deref @ Prefix::EMPTY_PREFIX { classPath: Some(path) } => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E(")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        Deref @ Prefix::PREFIX { name, restPrefix: rest_prefix, .. } => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*toStrWithEmpty(rest_prefix)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) };
            r#str
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStr)
}

pub(crate) fn isPackagePrefix<'__b>(mut inPrefix: &'__b metamodelica::Ref<Prefix>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inPrefix {
            Deref @ Prefix::PREFIX { restPrefix: prefix, .. } => {
                { inPrefix = prefix; continue '__tco; }
            },
            Deref @ Prefix::EMPTY_PREFIX { classPath: None } => {
                return true
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn toPackagePrefix(mut inPrefix: &metamodelica::Ref<Prefix>) -> metamodelica::Ref<Prefix> {
    let mut outPrefix: metamodelica::Ref<Prefix>;
    outPrefix = (match &**inPrefix {
        Prefix::PREFIX {
            name,
            dims,
            restPrefix: rest_prefix,
        } => {
            let mut rest_prefix = (*rest_prefix).clone();
            rest_prefix = toPackagePrefix(metamodelica::AsArg::as_arg(&rest_prefix));
            metamodelica::Ref::new(Prefix::PREFIX {
                name: name.clone(),
                dims: dims.clone(),
                restPrefix: rest_prefix.clone(),
            })
        }
        Prefix::EMPTY_PREFIX { .. } => metamodelica::Ref::new(Prefix::EMPTY_PREFIX { classPath: None }),
    });
    outPrefix
}
