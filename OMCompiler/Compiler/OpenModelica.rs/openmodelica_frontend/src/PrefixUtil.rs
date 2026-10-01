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
use crate::Lookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

//import Util;
pub(crate) fn printComponentPrefixStr(mut pre: &metamodelica::Ref<DAE::ComponentPrefix>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match pre {
        Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. } => literal!("<Prefix.NOCOMPPRE()>"),
        Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, subscripts: Deref @ metamodelica::ListNode::Nil, prefix: __pre_prefix, .. } => __pre_prefix.clone(),
        Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, prefix: __pre_prefix, subscripts: __pre_subscripts, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*__pre_prefix); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionDump::printSubscriptLstStr(__pre_subscripts.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        Deref @ DAE::ComponentPrefix::PRE { subscripts: Deref @ metamodelica::ListNode::Nil, next: __pre_next, prefix: __pre_prefix, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printComponentPrefixStr(metamodelica::AsArg::as_arg(&__pre_next))?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*__pre_prefix); ArcStr::from(__mm_s) },
        Deref @ DAE::ComponentPrefix::PRE { next: __pre_next, prefix: __pre_prefix, subscripts: __pre_subscripts, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printComponentPrefixStr(metamodelica::AsArg::as_arg(&__pre_next))?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*__pre_prefix); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionDump::printSubscriptLstStr(__pre_subscripts.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn printPrefixStr(mut inPrefix: &DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = inPrefix;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::NOPRE { .. } => {
                    Ok(literal!("<Prefix.NOPRE()>"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ } => {
                    Ok(literal!("<Prefix.PREFIX(DAE.NOCOMPPRE())>"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: Deref @ metamodelica::ListNode::Nil, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, ci_state: _, info: _ }, classPre: _ } => {
                    Ok(r#str.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: ss, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, ci_state: _, info: _ }, classPre: _ } => {
                    let mut s: ArcStr;
                    s = stringAppend(r#str.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(ss.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionDump::subscriptString(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) });
                    Ok(s.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: Deref @ metamodelica::ListNode::Nil, next: rest, ci_state: _, info: _ }, classPre: cp } => {
                    let mut s: ArcStr;
                    let mut rest_1: ArcStr;
                    let mut s_1: ArcStr;
                    rest_1 = printPrefixStr(&(DAE::Prefix::PREFIX { compPre: rest.clone(), classPre: cp.clone() }))?;
                    s = stringAppend(rest_1.clone(), literal!("."));
                    s_1 = stringAppend(s.clone(), r#str.clone());
                    Ok(s_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: r#str, dimensions: _, subscripts: ss, next: rest, ci_state: _, info: _ }, classPre: cp } => {
                    let mut s: ArcStr;
                    let mut rest_1: ArcStr;
                    let mut s_1: ArcStr;
                    let mut s_2: ArcStr;
                    rest_1 = printPrefixStr(&(DAE::Prefix::PREFIX { compPre: rest.clone(), classPre: cp.clone() }))?;
                    s = stringAppend(rest_1.clone(), literal!("."));
                    s_1 = stringAppend(s.clone(), r#str.clone());
                    s_2 = stringAppend(s_1.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(List::map(ss.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionDump::subscriptString(&__a0))?, literal!(", "))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) });
                    Ok(s_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outString)
}

pub(crate) fn printPrefixStr2(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inPrefix) {
        DAE::Prefix::NOPRE { .. } => {
            literal!("")
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ } => {
            literal!("")
        },
        p => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*printPrefixStr(metamodelica::AsArg::as_arg(&p))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn printPrefixStr3(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inPrefix) {
        DAE::Prefix::NOPRE { .. } => {
            literal!("<NO COMPONENT>")
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ } => {
            literal!("<NO COMPONENT>")
        },
        p => {
            printPrefixStr(metamodelica::AsArg::as_arg(&p))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn printPrefixStrIgnoreNoPre(mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inPrefix) {
        DAE::Prefix::NOPRE { .. } => {
            literal!("")
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ } => {
            literal!("")
        },
        p => {
            printPrefixStr(metamodelica::AsArg::as_arg(&p))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub(crate) fn printPrefix(mut p: &DAE::Prefix) -> Result<()> {
    let mut s: ArcStr;
    s = printPrefixStr(p)?;
    Print::printBuf(s)?;
    Ok(())
}

pub(crate) fn prefixAdd(
    mut inIdent: ArcStr,
    mut inType: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inIntegerLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inPrefix: &DAE::Prefix,
    mut vt: SCode::Variability,
    mut ci_state: ClassInf::State,
    mut inInfo: SourceInfo,
) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix;
    outPrefix = (match inPrefix.clone() {
        DAE::Prefix::PREFIX {
            compPre: ref p,
            classPre: _,
        } => {
            let mut i = inIdent;
            let mut s = inIntegerLst;
            DAE::Prefix::PREFIX {
                compPre: metamodelica::Ref::new(DAE::ComponentPrefix::PRE {
                    prefix: i,
                    dimensions: inType,
                    subscripts: s,
                    next: p.clone(),
                    ci_state: ci_state,
                    info: inInfo,
                }),
                classPre: DAE::ClassPrefix { variability: vt },
            }
        }
        DAE::Prefix::NOPRE { .. } => {
            let mut i = inIdent;
            let mut s = inIntegerLst;
            DAE::Prefix::PREFIX {
                compPre: metamodelica::Ref::new(DAE::ComponentPrefix::PRE {
                    prefix: i,
                    dimensions: inType,
                    subscripts: s,
                    next: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
                    ci_state: ci_state,
                    info: inInfo,
                }),
                classPre: DAE::ClassPrefix { variability: vt },
            }
        }
    });
    Ok(outPrefix)
}

pub(crate) fn prefixFirst(mut inPrefix: &DAE::Prefix) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix;
    outPrefix = (::match_deref::match_deref! { match &(inPrefix) {
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: a, dimensions: pdims, subscripts: b, ci_state, info, .. }, classPre: cp } => {
            DAE::Prefix::PREFIX { compPre: metamodelica::Ref::new(DAE::ComponentPrefix::PRE { prefix: a.clone(), dimensions: pdims.clone(), subscripts: b.clone(), next: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(), ci_state: ci_state.clone(), info: info.clone() }), classPre: cp.clone() }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outPrefix)
}

pub(crate) fn prefixFirstCref(mut inPrefix: DAE::Prefix) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut name: ArcStr;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inPrefix) {
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: __pa0, subscripts: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    subs = metamodelica::Own::own(__pa1);
    outCref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: name,
        identType: DAE::T_UNKNOWN_DEFAULT().clone(),
        subscriptLst: subs,
    });
    Ok(outCref)
}

pub(crate) fn prefixLast(mut inPrefix: DAE::Prefix) -> Result<DAE::Prefix> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPrefix) {
            res @ DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. }, classPre: _ } => {
                return Ok(res.clone())
            },
            DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { next: p, .. }, classPre: cp } => {
                let mut res: DAE::Prefix;
                { inPrefix = DAE::Prefix::PREFIX { compPre: p.clone(), classPre: cp.clone() }; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn prefixStripLast(mut inPrefix: &DAE::Prefix) -> Result<DAE::Prefix> {
    let mut outPrefix: DAE::Prefix;
    outPrefix = (match inPrefix.clone() {
        DAE::Prefix::NOPRE { .. } => openmodelica_frontend_types::DAE::Prefix::NOPRE,
        DAE::Prefix::PREFIX {
            compPre: mut compPre,
            classPre: mut cp,
        } => {
            let mut compPre = compPre.clone();
            compPre = compPreStripLast(metamodelica::AsArg::as_arg(&compPre))?;
            DAE::Prefix::PREFIX {
                compPre: compPre.clone(),
                classPre: cp.clone(),
            }
        }
    });
    Ok(outPrefix)
}

fn compPreStripLast(
    mut inCompPrefix: &metamodelica::Ref<DAE::ComponentPrefix>,
) -> Result<metamodelica::Ref<DAE::ComponentPrefix>> {
    let mut outCompPrefix: metamodelica::Ref<DAE::ComponentPrefix>;
    outCompPrefix = (match &**inCompPrefix {
        DAE::ComponentPrefix::NOCOMPPRE { .. } => {
            openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE()
        }
        DAE::ComponentPrefix::PRE { next, .. } => next.clone(),
    });
    Ok(outCompPrefix)
}

pub(crate) fn prefixPath(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inPrefix: DAE::Prefix,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPath, inPrefix)) {
            (p, DAE::Prefix::NOPRE { .. }) => {
                return Ok(p.clone())
            },
            (p, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: s, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. }, classPre: _ }) => {
                let mut p_1: metamodelica::Ref<Absyn::Path>;
                return Ok(metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: s.clone(), path: p.clone() }))
            },
            (p, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: s, next: ss, .. }, classPre: cp }) => {
                let mut p_1: metamodelica::Ref<Absyn::Path>;
                { (inPath, inPrefix) = (metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: s.clone(), path: p.clone() }), DAE::Prefix::PREFIX { compPre: ss.clone(), classPre: cp.clone() }); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn prefixToPath(mut inPrefix: &DAE::Prefix) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (match inPrefix.clone() {
        DAE::Prefix::PREFIX {
            compPre: ref ss,
            classPre: _,
        } => componentPrefixToPath(metamodelica::AsArg::as_arg(&ss))?,
        _ => return Err("match: no arm matched"),
    });
    Ok(outPath)
}

pub(crate) fn identAndPrefixToPath(mut ident: ArcStr, mut inPrefix: DAE::Prefix) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = AbsynUtil::pathString(
        prefixPath(metamodelica::Ref::new(Absyn::Path::IDENT { name: ident }), inPrefix)?,
        literal!("."),
        true,
        false,
    )?;
    Ok(r#str)
}

pub(crate) fn componentPrefixToPath(
    mut pre: &metamodelica::Ref<DAE::ComponentPrefix>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = (::match_deref::match_deref! { match pre {
        Deref @ DAE::ComponentPrefix::PRE { prefix: s, next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. } => {
            metamodelica::Ref::new(Absyn::Path::IDENT { name: s.clone() })
        },
        Deref @ DAE::ComponentPrefix::PRE { prefix: s, next: ss, .. } => {
            metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: s.clone(), path: componentPrefixToPath(ss)? })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(path)
}

pub(crate) fn prefixCref(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut pre: DAE::Prefix,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache;
    let mut cref_1: metamodelica::Ref<DAE::ComponentRef>;
    (outCache, cref_1) = prefixToCref2(cache, env, inIH, pre, Some(cref))?;
    Ok((outCache, cref_1))
}

pub(crate) fn prefixCrefNoContext(
    mut inPre: DAE::Prefix,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    (_, outCref) = prefixToCref2(
        FCore::noCache(),
        FGraph::empty(),
        &(InnerOuter::emptyInstHierarchy().clone()),
        inPre,
        Some(inCref),
    )?;
    Ok(outCref)
}

pub(crate) fn prefixToCref(mut pre: DAE::Prefix) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref_1: metamodelica::Ref<DAE::ComponentRef>;
    (_, cref_1) = prefixToCref2(
        FCore::noCache(),
        FGraph::empty(),
        &(InnerOuter::emptyInstHierarchy().clone()),
        pre,
        None,
    )?;
    Ok(cref_1)
}

fn prefixToCref2<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &'__b InstanceHierarchy,
    mut inPrefix: DAE::Prefix,
    mut inExpComponentRefOption: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, inPrefix.clone(), inExpComponentRefOption)) {
            (_, _, DAE::Prefix::NOPRE { .. }, None) => {
                return Ok(return Err("fail"))
            },
            (_, _, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ }, None) => {
                return Ok(return Err("fail"))
            },
            (cache, _, DAE::Prefix::NOPRE { .. }, Some(cref)) => {
                return Ok((cache.clone(), cref.clone()))
            },
            (cache, _, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ }, Some(cref)) => {
                return Ok((cache.clone(), cref.clone()))
            },
            (cache, env, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: i, dimensions: ds, subscripts: s, next: xs, ci_state, .. }, classPre: cp }, None) => {
                let mut cref_1: metamodelica::Ref<DAE::ComponentRef>;
                let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                let mut ident_ty: metamodelica::Ref<DAE::Type>;
                let mut cache = (*cache).clone();
                ident_ty = Expression::liftArrayLeftList(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ci_state.clone(), varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), ds.clone());
                cref_ = ComponentReferenceBasics::makeCrefIdent(i.clone(), ident_ty, s.clone());
                { (inCache, inEnv, inIH, inPrefix, inExpComponentRefOption) = (cache.clone(), env.clone(), inIH, DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_)); continue '__tco; }
            },
            (cache, env, DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: i, dimensions: ds, subscripts: s, next: xs, ci_state, .. }, classPre: cp }, Some(cref)) => {
                let mut cref_1: metamodelica::Ref<DAE::ComponentRef>;
                let mut cref_2: metamodelica::Ref<DAE::ComponentRef>;
                let mut ident_ty: metamodelica::Ref<DAE::Type>;
                let mut cache = (*cache).clone();
                let mut cref = (*cref).clone();
                (cache, cref) = prefixSubscriptsInCref(cache.clone(), env.clone(), inIH, &inPrefix, cref.clone())?;
                ident_ty = Expression::liftArrayLeftList(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ci_state.clone(), varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), ds.clone());
                cref_2 = ComponentReferenceBasics::makeCrefQual(i.clone(), ident_ty, s.clone(), cref.clone());
                { (inCache, inEnv, inIH, inPrefix, inExpComponentRefOption) = (cache.clone(), env.clone(), inIH, DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_2)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn prefixToCrefOpt(mut pre: DAE::Prefix) -> Result<Option<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut cref_1: Option<metamodelica::Ref<DAE::ComponentRef>>;
    cref_1 = prefixToCrefOpt2(pre, None)?;
    Ok(cref_1)
}

pub(crate) fn prefixToCrefOpt2(
    mut inPrefix: DAE::Prefix,
    mut inExpComponentRefOption: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<Option<metamodelica::Ref<DAE::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inPrefix, inExpComponentRefOption)) {
            (DAE::Prefix::NOPRE { .. }, None) => {
                return Ok(None)
            },
            (DAE::Prefix::NOPRE { .. }, Some(cref)) => {
                return Ok(Some(cref.clone()))
            },
            (DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ }, Some(cref)) => {
                return Ok(Some(cref.clone()))
            },
            (DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: i, subscripts: s, next: xs, .. }, classPre: cp }, None) => {
                let mut cref_1: Option<metamodelica::Ref<DAE::ComponentRef>>;
                let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                cref_ = ComponentReferenceBasics::makeCrefIdent(i.clone(), metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), s.clone());
                { (inPrefix, inExpComponentRefOption) = (DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_)); continue '__tco; }
            },
            (DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { prefix: i, subscripts: s, next: xs, .. }, classPre: cp }, Some(cref)) => {
                let mut cref_1: Option<metamodelica::Ref<DAE::ComponentRef>>;
                let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                cref_ = ComponentReferenceBasics::makeCrefQual(i.clone(), metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, varLst: metamodelica::nil(), equalityConstraint: None, usedExternally: false }), s.clone(), cref.clone());
                { (inPrefix, inExpComponentRefOption) = (DAE::Prefix::PREFIX { compPre: xs.clone(), classPre: cp.clone() }, Some(cref_)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn makeCrefFromPrefixNoFail(mut pre: DAE::Prefix) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    cref = (::match_deref::match_deref! { match &(pre.clone()) {
        DAE::Prefix::NOPRE { .. } => {
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            c = ComponentReferenceBasics::makeCrefIdent(literal!(""), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            c
        },
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, classPre: _ } => {
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            c = ComponentReferenceBasics::makeCrefIdent(literal!(""), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
            c
        },
        _ => {
            let mut c: metamodelica::Ref<DAE::ComponentRef>;
            c = prefixToCref(pre)?;
            c
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn prefixSubscriptsInCref(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &InstanceHierarchy,
    mut pre: &DAE::Prefix,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache;
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    (outCache, outCr) = prefixSubscriptsInCrefWork(inCache, inEnv, inIH, pre, inCr, metamodelica::nil())?;
    Ok((outCache, outCr))
}

fn prefixSubscriptsInCrefWork<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &'__b InstanceHierarchy,
    mut pre: &'__b DAE::Prefix,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, inCr)) {
            (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: tp, subscriptLst: subs }) => {
                let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                let mut cache = (*cache).clone();
                let mut subs = (*subs).clone();
                (cache, subs) = prefixSubscripts(cache.clone(), env.clone(), inIH, pre, metamodelica::AsArg::as_arg(&subs))?;
                cr = ComponentReferenceBasics::makeCrefIdent(id.clone(), tp.clone(), subs.clone());
                return Ok((cache.clone(), ComponentReference::implode_reverse(&(metamodelica::cons(cr, acc)))?))
            },
            (cache, env, Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: tp, subscriptLst: subs, componentRef: cr }) => {
                let mut crid: metamodelica::Ref<DAE::ComponentRef>;
                let mut cache = (*cache).clone();
                let mut subs = (*subs).clone();
                let mut cr = (*cr).clone();
                (cache, subs) = prefixSubscripts(cache.clone(), env.clone(), inIH, pre, metamodelica::AsArg::as_arg(&subs))?;
                crid = ComponentReferenceBasics::makeCrefIdent(id.clone(), tp.clone(), subs.clone());
                { (inCache, inEnv, inIH, pre, inCr, acc) = (cache.clone(), env.clone(), inIH, pre, cr.clone(), metamodelica::cons(crid, acc)); continue '__tco; }
            },
            (cache, _, Deref @ DAE::ComponentRef::WILD { .. }) => {
                return Ok((cache.clone(), openmodelica_frontend_types::DAE::ComponentRef::interned_WILD()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn prefixSubscripts(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &InstanceHierarchy,
    mut pre: &DAE::Prefix,
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Subscript>>)> {
    let mut outCache: FCore::Cache;
    let mut outSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    (outCache, outSubs) = (::match_deref::match_deref! { match inSubs {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut sub = (*sub).clone();
            let mut subs = (*subs).clone();
            (cache, sub) = prefixSubscript(cache, env.clone(), inIH, pre, metamodelica::AsArg::as_arg(&sub))?;
            (cache, subs) = prefixSubscripts(cache, env, inIH, pre, metamodelica::AsArg::as_arg(&subs))?;
            (cache, metamodelica::cons(sub.clone(), subs.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outSubs))
}

fn prefixSubscript(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &InstanceHierarchy,
    mut pre: &DAE::Prefix,
    mut sub: &metamodelica::Ref<DAE::Subscript>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Subscript>)> {
    let mut outCache: FCore::Cache;
    let mut outSub: metamodelica::Ref<DAE::Subscript>;
    (outCache, outSub) = (match &**sub {
        DAE::Subscript::WHOLEDIM { .. } => {
            let mut cache = inCache;
            (cache, openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM())
        }
        DAE::Subscript::SLICE { exp } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut exp = (*exp).clone();
            (cache, exp) = prefixExpWork(cache, &env, inIH, exp.clone(), pre)?;
            (
                cache,
                metamodelica::Ref::new(DAE::Subscript::SLICE { exp: exp.clone() }),
            )
        }
        DAE::Subscript::WHOLE_NONEXP { exp } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut exp = (*exp).clone();
            (cache, exp) = prefixExpWork(cache, &env, inIH, exp.clone(), pre)?;
            (
                cache,
                metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: exp.clone() }),
            )
        }
        DAE::Subscript::INDEX { exp } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut exp = (*exp).clone();
            (cache, exp) = prefixExpWork(cache, &env, inIH, exp.clone(), pre)?;
            (
                cache,
                metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp.clone() }),
            )
        }
    });
    Ok((outCache, outSub))
}

pub(crate) fn prefixCrefInnerOuter(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inPrefix: DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache;
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    (outCache, outCref) = (::match_deref::match_deref! { match &((inCache, inIH, inCref, inPrefix)) {
        (cache, ih, cref, pre) => {
            let mut newCref: metamodelica::Ref<DAE::ComponentRef>;
            newCref = InnerOuter::prefixOuterCrefWithTheInnerPrefix(metamodelica::AsArg::as_arg(&ih), cref.clone(), pre.clone())?;
            (cache.clone(), newCref)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outCref))
}

pub(crate) fn prefixExp(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut ih: &metamodelica::List<InnerOuter::TopInstance>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut pre: &DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>)> {
    let mut cache: FCore::Cache = cache;
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    if let Ok((__pa0, __pa1)) = prefixExpWork(cache.clone(), env, ih, exp.clone(), pre) {
        cache = metamodelica::Own::own(__pa0);
        exp = metamodelica::Own::own(__pa1);
    } else {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("PrefixUtil.prefixExp"));
                __mm_s.push_str(&*literal!(" failed on exp: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?);
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*makePrefixString(pre)?);
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("FrontEnd/PrefixUtil.mo"),
        )?;
        return Err("fail");
    }
    Ok((cache, exp))
}

fn prefixExpWork(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut ih: &metamodelica::List<InnerOuter::TopInstance>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut pre: &DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>)> {
    let mut cache: FCore::Cache = cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (cache, outExp) = (::match_deref::match_deref! { match &((inExp.clone(), pre.clone())) {
        (e, DAE::Prefix::NOPRE { .. }) if (!(System::getHasInnerOuterDefinitions())) => {
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::ICONST { .. }, _) => {
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::RCONST { .. }, _) => {
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::SCONST { .. }, _) => {
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::BCONST { .. }, _) => {
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::ENUM_LITERAL { .. }, _) => {
            (cache, e.clone())
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, ty: t }, _) => {
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut t = (*t).clone();
            if System::getHasInnerOuterDefinitions() && !((ih).is_empty()) {
                if '__try0: {
                    cr_1 = unwrap_break_err!(InnerOuter::prefixOuterCrefWithTheInnerPrefix(ih, cr.clone(), pre.clone()), '__try0);
                    (cache, t) = unwrap_break_err!(prefixExpressionsInType(cache.clone(), env.clone(), ih.clone(), pre.clone(), t.clone()), '__try0);
                    outExp = unwrap_break_err!(Expression::makeCrefExp(cr_1.clone(), t.clone()), '__try0);
                    return Ok((cache, outExp));
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            }
            if openmodelica_frontend_types::DAE::Prefix::NOPRE == pre.clone() {
                crefExp = inExp;
            } else {
                (cache, crefExp) = prefixExpCref(cache, env.clone(), ih.clone(), inExp, pre.clone())?;
            }
            (cache, crefExp)
        },
        (Deref @ DAE::Exp::CLKCONST { clk }, _) => {
            let mut clk = (*clk).clone();
            (cache, clk) = prefixClockKind(cache, env.clone(), ih.clone(), clk.clone(), pre.clone())?;
            (cache, metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: clk.clone() }))
        },
        (Deref @ DAE::Exp::ASUB { exp: e1, sub: subs }, _) => {
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e1 = (*e1).clone();
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (cache, es_1) = prefixExpList(cache, env, ih, &expl, pre)?;
            (cache, e1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            e2 = Expression::makeASUB(e1.clone(), es_1)?;
            (cache, e2)
        },
        (Deref @ DAE::Exp::TSUB { exp: e1, ix: index_, ty: t }, _) => {
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            e2 = metamodelica::Ref::new(DAE::Exp::TSUB { exp: e1.clone(), ix: index_.clone(), ty: t.clone() });
            (cache, e2)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: o, exp2: e2 }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, e2_1) = prefixExpWork(cache, env, ih, e2.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1, operator: o.clone(), exp2: e2_1 }))
        },
        (Deref @ DAE::Exp::UNARY { operator: o, exp: e1 }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::UNARY { operator: o.clone(), exp: e1_1 }))
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e1, operator: o, exp2: e2 }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, e2_1) = prefixExpWork(cache, env, ih, e2.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1, operator: o.clone(), exp2: e2_1 }))
        },
        (Deref @ DAE::Exp::LUNARY { operator: o, exp: e1 }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::LUNARY { operator: o.clone(), exp: e1_1 }))
        },
        (Deref @ DAE::Exp::RELATION { exp1: e1, operator: o, exp2: e2, index: index_, optionExpisASUB: isExpisASUB }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, e2_1) = prefixExpWork(cache, env, ih, e2.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1_1, operator: o.clone(), exp2: e2_1, index: index_.clone(), optionExpisASUB: isExpisASUB.clone() }))
        },
        (Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, _) => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e3_1: metamodelica::Ref<DAE::Exp>;
            (cache, e1_1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, e2_1) = prefixExpWork(cache, env, ih, e2.clone(), pre)?;
            (cache, e3_1) = prefixExpWork(cache, env, ih, e3.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1_1, expThen: e2_1, expElse: e3_1 }))
        },
        (Deref @ DAE::Exp::SIZE { exp: cref, sz: Some(dim) }, _) => {
            let mut cref_1: metamodelica::Ref<DAE::Exp>;
            let mut dim_1: metamodelica::Ref<DAE::Exp>;
            (cache, cref_1) = prefixExpWork(cache, env, ih, cref.clone(), pre)?;
            (cache, dim_1) = prefixExpWork(cache, env, ih, dim.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::SIZE { exp: cref_1, sz: Some(dim_1) }))
        },
        (Deref @ DAE::Exp::SIZE { exp: cref, sz: None }, _) => {
            let mut cref_1: metamodelica::Ref<DAE::Exp>;
            (cache, cref_1) = prefixExpWork(cache, env, ih, cref.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::SIZE { exp: cref_1, sz: None }))
        },
        (Deref @ DAE::Exp::CALL { path: f, expLst: es, attr }, _) => {
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (cache, es_1) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&es), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::CALL { path: f.clone(), expLst: es_1, attr: attr.clone() }))
        },
        (e @ Deref @ DAE::Exp::PARTEVALFUNCTION { .. }, _) => {
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e = (*e).clone();
            (cache, es_1) = prefixExpList(cache, env, ih, var_field!((*e).expList, DAE::Exp::PARTEVALFUNCTION), pre)?;
            assign_variant_field!(e => DAE::Exp::PARTEVALFUNCTION; expList = es_1);
            (cache, e.clone())
        },
        (Deref @ DAE::Exp::RECORD { path: f, exps: es, comp: fieldNames, ty: t }, _) => {
            (cache, _) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&es), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::RECORD { path: f.clone(), exps: es.clone(), comp: fieldNames.clone(), ty: t.clone() }))
        },
        (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, _) => {
            (cache, inExp)
        },
        (Deref @ DAE::Exp::ARRAY { ty: t, scalar: sc, array: es }, _) => {
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (cache, es_1) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&es), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: sc.clone(), array: es_1 }))
        },
        (Deref @ DAE::Exp::TUPLE { PR: es }, _) => {
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (cache, es_1) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&es), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::TUPLE { PR: es_1 }))
        },
        (Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Nil, .. }, _) => {
            (cache, inExp)
        },
        (Deref @ DAE::Exp::MATRIX { ty: t, integer: a, matrix: Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } }, _) => {
            let mut x_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut xs_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut t = (*t).clone();
            (cache, x_1) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&x), pre)?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(prefixExpWork(cache, env, ih, metamodelica::Ref::new(DAE::Exp::MATRIX { ty: t.clone(), integer: a.clone(), matrix: xs.clone() }), pre)?) {
                (__pa0, Deref @ DAE::Exp::MATRIX { ty: __pa1, integer: _, matrix: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            t = metamodelica::Own::own(__pa1);
            xs_1 = metamodelica::Own::own(__pa2);
            (cache, metamodelica::Ref::new(DAE::Exp::MATRIX { ty: t.clone(), integer: a.clone(), matrix: metamodelica::cons(x_1, xs_1) }))
        },
        (Deref @ DAE::Exp::RANGE { ty: t, start, step: None, stop }, _) => {
            let mut start_1: metamodelica::Ref<DAE::Exp>;
            let mut stop_1: metamodelica::Ref<DAE::Exp>;
            (cache, start_1) = prefixExpWork(cache, env, ih, start.clone(), pre)?;
            (cache, stop_1) = prefixExpWork(cache, env, ih, stop.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::RANGE { ty: t.clone(), start: start_1, step: None, stop: stop_1 }))
        },
        (Deref @ DAE::Exp::RANGE { ty: t, start, step: Some(step), stop }, _) => {
            let mut start_1: metamodelica::Ref<DAE::Exp>;
            let mut stop_1: metamodelica::Ref<DAE::Exp>;
            let mut step_1: metamodelica::Ref<DAE::Exp>;
            (cache, start_1) = prefixExpWork(cache, env, ih, start.clone(), pre)?;
            (cache, step_1) = prefixExpWork(cache, env, ih, step.clone(), pre)?;
            (cache, stop_1) = prefixExpWork(cache, env, ih, stop.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::RANGE { ty: t.clone(), start: start_1, step: Some(step_1), stop: stop_1 }))
        },
        (Deref @ DAE::Exp::CAST { ty: tp, exp: e }, _) => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            (cache, e_1) = prefixExpWork(cache, env, ih, e.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: e_1 }))
        },
        (Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: exp, iterators: riters }, _) => {
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut riters = (*riters).clone();
            (cache, exp_1) = prefixExpWork(cache, env, ih, exp.clone(), pre)?;
            (cache, riters) = prefixIterators(cache, env.clone(), ih, metamodelica::AsArg::as_arg(&riters), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: exp_1, iterators: riters.clone() }))
        },
        (Deref @ DAE::Exp::LIST { valList: es }, _) => {
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (cache, es_1) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&es), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::LIST { valList: es_1 }))
        },
        (Deref @ DAE::Exp::CONS { car: e1, cdr: e2 }, _) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (cache, e1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, e2) = prefixExpWork(cache, env, ih, e2.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::CONS { car: e1.clone(), cdr: e2.clone() }))
        },
        (Deref @ DAE::Exp::META_TUPLE { listExp: es }, _) => {
            let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (cache, es_1) = prefixExpList(cache, env, ih, metamodelica::AsArg::as_arg(&es), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: es_1 }))
        },
        (Deref @ DAE::Exp::META_OPTION { exp: Some(e1) }, _) => {
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: Some(e1.clone()) }))
        },
        (Deref @ DAE::Exp::META_OPTION { exp: None }, _) => {
            (cache, metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: None }))
        },
        (Deref @ DAE::Exp::METARECORDCALL { .. }, _) => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (cache, expl) = prefixExpList(cache, env, ih, var_field!((*inExp).args, DAE::Exp::METARECORDCALL), pre)?;
            (cache, metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: var_field!((*inExp).path, DAE::Exp::METARECORDCALL).clone(), args: expl, fieldNames: var_field!((*inExp).fieldNames, DAE::Exp::METARECORDCALL).clone(), index: var_field!((*inExp).index, DAE::Exp::METARECORDCALL).clone(), typeVars: var_field!((*inExp).typeVars, DAE::Exp::METARECORDCALL).clone() }))
        },
        (e @ Deref @ DAE::Exp::UNBOX { exp: e1, .. }, _) => {
            let mut e = (*e).clone();
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            assign_variant_field!(e => DAE::Exp::UNBOX; exp = e1.clone());
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::BOX { exp: e1 }, _) => {
            let mut e = (*e).clone();
            let mut e1 = (*e1).clone();
            (cache, e1) = prefixExpWork(cache, env, ih, e1.clone(), pre)?;
            assign_variant_field!(e => DAE::Exp::BOX; exp = e1.clone());
            (cache, e.clone())
        },
        (e, DAE::Prefix::NOPRE { .. }) => {
            (cache, e.clone())
        },
        (e @ Deref @ DAE::Exp::EMPTY { .. }, _) => {
            (cache, e.clone())
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.prefixExpWork")); __mm_s.push_str(&*literal!(" failed on exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*makePrefixString(pre)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/PrefixUtil.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((cache, outExp))
}

fn prefixExpCref(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: InstanceHierarchy,
    mut inCref: metamodelica::Ref<DAE::Exp>,
    mut inPrefix: DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>)> {
    let mut outCache: FCore::Cache;
    let mut outCref: metamodelica::Ref<DAE::Exp>;
    let mut is_iter: Option<bool>;
    let mut cache: FCore::Cache;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    (is_iter, cache) = Lookup::isIterator(inCache, &inEnv, &cr);
    (outCache, outCref) = prefixExpCref2(cache, inEnv, inIH, is_iter, inCref, inPrefix)?;
    Ok((outCache, outCref))
}

fn prefixExpCref2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: InstanceHierarchy,
    mut inIsIter: Option<bool>,
    mut inCref: metamodelica::Ref<DAE::Exp>,
    mut inPrefix: DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>)> {
    let mut outCache: FCore::Cache;
    let mut outCref: metamodelica::Ref<DAE::Exp>;
    (outCache, outCref) = (::match_deref::match_deref! { match &((inIsIter, inCref.clone())) {
        (Some(false), Deref @ DAE::Exp::CREF { componentRef: cr, ty }) => {
            let mut cache = inCache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr = (*cr).clone();
            let mut ty = (*ty).clone();
            (cache, cr) = prefixCref(cache, inEnv.clone(), &inIH, inPrefix.clone(), cr.clone())?;
            (cache, ty) = prefixExpressionsInType(cache, inEnv, inIH, inPrefix, ty.clone())?;
            exp = Expression::makeCrefExp(cr.clone(), ty.clone())?;
            (cache, exp)
        },
        (Some(true), _) => {
            (inCache, inCref)
        },
        (None, Deref @ DAE::Exp::CREF { componentRef: cr, ty }) => {
            let mut cache = inCache.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cr = (*cr).clone();
            let mut ty = (*ty).clone();
            (cache, cr) = prefixSubscriptsInCref(cache, inEnv.clone(), &inIH, &inPrefix, cr.clone())?;
            (cache, ty) = prefixExpressionsInType(cache, inEnv, inIH, inPrefix, ty.clone())?;
            exp = Expression::makeCrefExp(cr.clone(), ty.clone())?;
            (cache, exp)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outCref))
}

fn prefixIterators(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ih: &InstanceHierarchy,
    mut inIters: &metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut pre: &DAE::Prefix,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outIters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
    (outCache, outIters) = (::match_deref::match_deref! { match inIters {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id, exp, guardExp: Some(gexp), ty }, tail: iters } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut iter: metamodelica::Ref<DAE::ReductionIterator>;
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            let mut iters = (*iters).clone();
            (cache, exp) = prefixExpWork(cache, &env, ih, exp.clone(), pre)?;
            (cache, gexp) = prefixExpWork(cache, &env, ih, gexp.clone(), pre)?;
            iter = metamodelica::Ref::new(DAE::ReductionIterator { id: id.clone(), exp: exp.clone(), guardExp: Some(gexp.clone()), ty: ty.clone() });
            (cache, iters) = prefixIterators(cache, env, ih, metamodelica::AsArg::as_arg(&iters), pre)?;
            (cache, metamodelica::cons(iter, iters.clone()))
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id, exp, guardExp: None, ty }, tail: iters } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut iter: metamodelica::Ref<DAE::ReductionIterator>;
            let mut exp = (*exp).clone();
            let mut iters = (*iters).clone();
            (cache, exp) = prefixExpWork(cache, &env, ih, exp.clone(), pre)?;
            iter = metamodelica::Ref::new(DAE::ReductionIterator { id: id.clone(), exp: exp.clone(), guardExp: None, ty: ty.clone() });
            (cache, iters) = prefixIterators(cache, env, ih, metamodelica::AsArg::as_arg(&iters), pre)?;
            (cache, metamodelica::cons(iter, iters.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outIters))
}

pub(crate) fn prefixExpList(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inPrefix: &DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Exp>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut e_1: metamodelica::Ref<DAE::Exp>;
    for mut e in &**inExpExpLst {
        (outCache, e_1) = prefixExpWork(outCache, inEnv, inIH, e.clone(), inPrefix)?;
        outExpExpLst = metamodelica::cons(e_1, outExpExpLst);
    }
    outExpExpLst = Dangerous::listReverseInPlace(outExpExpLst);
    Ok((outCache, outExpExpLst))
}

//--------------------------------------------
//   PART OF THE WORKAROUND FOR VALUEBLOCKS. KS
fn prefixStatements(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut inIH: &InstanceHierarchy,
    mut stmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut p: &DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache = cache;
    let mut outStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    for mut st in &**stmts {
        let () = (match &*st.clone() {
            DAE::Statement::STMT_ASSIGN {
                type_: t,
                exp1: e1,
                exp: e,
                source,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut e1 = (*e1).clone();
                let mut e = (*e).clone();
                (outCache, e1) = prefixExpWork(outCache, env, inIH, e1.clone(), p)?;
                (outCache, e) = prefixExpWork(outCache, env, inIH, e.clone(), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                    type_: t.clone(),
                    exp1: e1.clone(),
                    exp: e.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_TUPLE_ASSIGN {
                type_: t,
                expExpLst: eLst,
                exp: e,
                source,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut eLst = (*eLst).clone();
                let mut e = (*e).clone();
                (outCache, e) = prefixExpWork(outCache, env, inIH, e.clone(), p)?;
                (outCache, eLst) = prefixExpList(outCache, env, inIH, metamodelica::AsArg::as_arg(&eLst), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN {
                    type_: t.clone(),
                    expExpLst: eLst.clone(),
                    exp: e.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_ASSIGN_ARR {
                type_: t,
                lhs: e1,
                exp: e,
                source,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut e1 = (*e1).clone();
                let mut e = (*e).clone();
                (outCache, e1) = prefixExpWork(outCache, env, inIH, e1.clone(), p)?;
                (outCache, e) = prefixExpWork(outCache, env, inIH, e.clone(), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR {
                    type_: t.clone(),
                    lhs: e1.clone(),
                    exp: e.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_FOR {
                type_: t,
                iterIsArray: bool,
                iter: id,
                range: e,
                statementLst: sList,
                source,
                sub_iters,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut e = (*e).clone();
                let mut sList = (*sList).clone();
                (outCache, e) = prefixExpWork(outCache, env, inIH, e.clone(), p)?;
                (outCache, sList) = prefixStatements(outCache, env, inIH, metamodelica::AsArg::as_arg(&sList), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_FOR {
                    type_: t.clone(),
                    iterIsArray: bool.clone(),
                    iter: id.clone(),
                    range: e.clone(),
                    statementLst: sList.clone(),
                    source: source.clone(),
                    sub_iters: sub_iters.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_IF {
                exp: e1,
                statementLst: sList,
                else_: elseBranch,
                source,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut e1 = (*e1).clone();
                let mut sList = (*sList).clone();
                let mut elseBranch = (*elseBranch).clone();
                (outCache, e1) = prefixExpWork(outCache, env, inIH, e1.clone(), p)?;
                (outCache, sList) = prefixStatements(outCache, env, inIH, metamodelica::AsArg::as_arg(&sList), p)?;
                (outCache, elseBranch) = prefixElse(
                    outCache,
                    env.clone(),
                    inIH.clone(),
                    metamodelica::AsArg::as_arg(&elseBranch),
                    p.clone(),
                )?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_IF {
                    exp: e1.clone(),
                    statementLst: sList.clone(),
                    else_: elseBranch.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_WHILE {
                exp: e1,
                statementLst: sList,
                source,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut e1 = (*e1).clone();
                let mut sList = (*sList).clone();
                (outCache, e1) = prefixExpWork(outCache, env, inIH, e1.clone(), p)?;
                (outCache, sList) = prefixStatements(outCache, env, inIH, metamodelica::AsArg::as_arg(&sList), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_WHILE {
                    exp: e1.clone(),
                    statementLst: sList.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_ASSERT {
                cond: e1,
                msg: e2,
                level: e3,
                source,
            } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut e1 = (*e1).clone();
                let mut e2 = (*e2).clone();
                let mut e3 = (*e3).clone();
                (outCache, e1) = prefixExpWork(outCache, env, inIH, e1.clone(), p)?;
                (outCache, e2) = prefixExpWork(outCache, env, inIH, e2.clone(), p)?;
                (outCache, e3) = prefixExpWork(outCache, env, inIH, e3.clone(), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_ASSERT {
                    cond: e1.clone(),
                    msg: e2.clone(),
                    level: e3.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_FAILURE { body: b, source } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                let mut b = (*b).clone();
                (outCache, b) = prefixStatements(outCache, env, inIH, metamodelica::AsArg::as_arg(&b), p)?;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_FAILURE {
                    body: b.clone(),
                    source: source.clone(),
                });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_RETURN { source } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_RETURN { source: source.clone() });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            DAE::Statement::STMT_BREAK { source } => {
                let mut elem: metamodelica::Ref<DAE::Statement>;
                elem = metamodelica::Ref::new(DAE::Statement::STMT_BREAK { source: source.clone() });
                outStmts = metamodelica::cons(elem, outStmts);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
    }
    outStmts = Dangerous::listReverseInPlace(outStmts);
    Ok((outCache, outStmts))
}

fn prefixElse(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inIH: InstanceHierarchy,
    mut elseBranch: &metamodelica::Ref<DAE::Else>,
    mut p: DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Else>)> {
    let mut outCache: FCore::Cache;
    let mut outElse: metamodelica::Ref<DAE::Else>;
    (outCache, outElse) = (match &**elseBranch {
        DAE::Else::NOELSE { .. } => {
            let mut localCache = cache;
            (localCache, openmodelica_frontend_types::DAE::Else::interned_NOELSE())
        }
        DAE::Else::ELSEIF {
            exp: e,
            statementLst: lStmt,
            else_: el,
        } => {
            let mut localCache = cache;
            let mut localEnv = env;
            let mut ih = inIH;
            let mut pre = p;
            let mut stmt: metamodelica::Ref<DAE::Else>;
            let mut e = (*e).clone();
            let mut lStmt = (*lStmt).clone();
            let mut el = (*el).clone();
            (localCache, e) = prefixExpWork(localCache, &localEnv, &ih, e.clone(), &pre)?;
            (localCache, el) = prefixElse(
                localCache,
                localEnv.clone(),
                ih.clone(),
                metamodelica::AsArg::as_arg(&el),
                pre.clone(),
            )?;
            (localCache, lStmt) =
                prefixStatements(localCache, &localEnv, &ih, metamodelica::AsArg::as_arg(&lStmt), &pre)?;
            stmt = metamodelica::Ref::new(DAE::Else::ELSEIF {
                exp: e.clone(),
                statementLst: lStmt.clone(),
                else_: el.clone(),
            });
            (localCache, stmt)
        }
        DAE::Else::ELSE { statementLst: lStmt } => {
            let mut localCache = cache;
            let mut localEnv = env;
            let mut ih = inIH;
            let mut pre = p;
            let mut stmt: metamodelica::Ref<DAE::Else>;
            let mut lStmt = (*lStmt).clone();
            (localCache, lStmt) =
                prefixStatements(localCache, &localEnv, &ih, metamodelica::AsArg::as_arg(&lStmt), &pre)?;
            stmt = metamodelica::Ref::new(DAE::Else::ELSE {
                statementLst: lStmt.clone(),
            });
            (localCache, stmt)
        }
    });
    Ok((outCache, outElse))
}

pub(crate) fn makePrefixString(mut pre: &DAE::Prefix) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match pre.clone() {
        DAE::Prefix::NOPRE { .. } => literal!("from top scope"),
        _ => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("from calling scope: "));
                __mm_s.push_str(&*printPrefixStr(pre)?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(r#str)
}

pub(crate) fn prefixExpressionsInType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPre: DAE::Prefix,
    mut inTy: metamodelica::Ref<DAE::Type>,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outTy: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    (outCache, outTy) = 'mc: {
        let __mc_input = &*inTy;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), inTy.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outTy: metamodelica::Ref<DAE::Type> = outTy.clone();
                    let (__pa0, (__pa1, _, _, _)) = Types::traverseType(inTy.clone(), (inCache.clone(), inEnv.clone(), inIH.clone(), inPre.clone()), &fnptr!(prefixArrayDimensions, metamodelica::Ref<DAE::Type>, (FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::Prefix)))?;
                    outTy = metamodelica::Own::own(__pa0);
                    outCache = metamodelica::Own::own(__pa1);
                    Ok(((outCache.clone(), outTy.clone()), outCache.clone(), outTy.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outTy = __wb1;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outTy))
}

fn prefixArrayDimensions(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut tpl: (
        FCore::Cache,
        FCore::Graph,
        metamodelica::List<InnerOuter::TopInstance>,
        DAE::Prefix,
    ),
) -> (
    metamodelica::Ref<DAE::Type>,
    (
        FCore::Cache,
        FCore::Graph,
        metamodelica::List<InnerOuter::TopInstance>,
        DAE::Prefix,
    ),
) {
    let mut oty: metamodelica::Ref<DAE::Type> = ty;
    let mut otpl: (
        FCore::Cache,
        FCore::Graph,
        metamodelica::List<InnerOuter::TopInstance>,
        DAE::Prefix,
    );
    (oty, otpl) = (::match_deref::match_deref! { match &((oty.clone(), tpl.clone())) {
        (Deref @ DAE::Type::T_ARRAY { .. }, (cache, env, ih, pre)) => {
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut cache = (*cache).clone();
            (cache, dims) = prefixDimensions(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), metamodelica::AsArg::as_arg(&pre), var_field!((*oty).dims, DAE::Type::T_ARRAY));
            assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = dims);
            (oty, (cache.clone(), env.clone(), ih.clone(), pre.clone()))
        },
        _ => {
            (oty, tpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oty, otpl)
}

pub(crate) fn prefixDimensions(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPre: &DAE::Prefix,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> (FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>) {
    let mut outCache: FCore::Cache;
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDims) = 'mc: {
        let __mc_input = &**inDims;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inCache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: e }, tail: rest } => {
                    let mut new: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache: FCore::Cache;
                    let mut e = (*e).clone();
                    (cache, e) = prefixExpWork(inCache.clone(), inEnv, inIH, e.clone(), inPre)?;
                    (cache, new) = prefixDimensions(&cache, inEnv, inIH, inPre, metamodelica::AsArg::as_arg(&rest));
                    Ok((cache.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: e.clone() }), new.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: d, tail: rest } => {
                    let mut new: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache: FCore::Cache;
                    (cache, new) = prefixDimensions(inCache, inEnv, inIH, inPre, metamodelica::AsArg::as_arg(&rest));
                    Ok((cache.clone(), metamodelica::cons(d.clone(), new.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outDims)
}

pub(crate) fn isPrefix(mut prefix: &DAE::Prefix) -> bool {
    let mut isPrefix: bool;
    isPrefix = (match prefix.clone() {
        DAE::Prefix::PREFIX { .. } => true,
        _ => false,
    });
    isPrefix
}

pub(crate) fn isNoPrefix(mut inPrefix: &DAE::Prefix) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (match inPrefix.clone() {
        DAE::Prefix::NOPRE { .. } => true,
        _ => false,
    });
    outIsEmpty
}

pub(crate) fn prefixClockKind(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inClkKind: metamodelica::Ref<DAE::ClockKind>,
    mut inPrefix: DAE::Prefix,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ClockKind>)> {
    let mut outCache: FCore::Cache;
    let mut outClkKind: metamodelica::Ref<DAE::ClockKind>;
    (outCache, outClkKind) = (match &*inClkKind {
        DAE::ClockKind::INFERRED_CLOCK { .. } => {
            let mut cache = inCache;
            (cache, inClkKind)
        }
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: e,
            resolution,
        } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut p = inPrefix;
            let mut clkKind: metamodelica::Ref<DAE::ClockKind>;
            let mut e = (*e).clone();
            let mut resolution = (*resolution).clone();
            (cache, e) = prefixExpWork(cache, &env, &ih, e.clone(), &p)?;
            (cache, resolution) = prefixExpWork(cache, &env, &ih, resolution.clone(), &p)?;
            clkKind = metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK {
                intervalCounter: e.clone(),
                resolution: resolution.clone(),
            });
            (cache, clkKind)
        }
        DAE::ClockKind::REAL_CLOCK { interval: e } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut p = inPrefix;
            let mut clkKind: metamodelica::Ref<DAE::ClockKind>;
            let mut e = (*e).clone();
            (cache, e) = prefixExpWork(cache, &env, &ih, e.clone(), &p)?;
            clkKind = metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() });
            (cache, clkKind)
        }
        DAE::ClockKind::EVENT_CLOCK {
            condition: e,
            startInterval: interval,
        } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut p = inPrefix;
            let mut clkKind: metamodelica::Ref<DAE::ClockKind>;
            let mut e = (*e).clone();
            let mut interval = (*interval).clone();
            (cache, e) = prefixExpWork(cache, &env, &ih, e.clone(), &p)?;
            (cache, interval) = prefixExpWork(cache, &env, &ih, interval.clone(), &p)?;
            clkKind = metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK {
                condition: e.clone(),
                startInterval: interval.clone(),
            });
            (cache, clkKind)
        }
        DAE::ClockKind::SOLVER_CLOCK {
            c: e,
            solverMethod: method,
        } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut p = inPrefix;
            let mut clkKind: metamodelica::Ref<DAE::ClockKind>;
            let mut e = (*e).clone();
            let mut method = (*method).clone();
            (cache, e) = prefixExpWork(cache, &env, &ih, e.clone(), &p)?;
            (cache, method) = prefixExpWork(cache, &env, &ih, method.clone(), &p)?;
            clkKind = metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK {
                c: e.clone(),
                solverMethod: method.clone(),
            });
            (cache, clkKind)
        }
    });
    Ok((outCache, outClkKind))
}

pub(crate) fn getPrefixInfo(mut inPrefix: &DAE::Prefix) -> SourceInfo {
    let mut outInfo: SourceInfo;
    outInfo = (::match_deref::match_deref! { match &(inPrefix) {
        DAE::Prefix::PREFIX { compPre: Deref @ DAE::ComponentPrefix::PRE { info: __esc_outInfo, .. }, .. } => {
            outInfo = (*__esc_outInfo).clone();
            outInfo.clone()
        },
        _ => Absyn::dummyInfo.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInfo
}

pub(crate) fn prefixHashWork<'__b>(mut inPrefix: &'__b metamodelica::Ref<DAE::ComponentPrefix>, mut hash: i32) -> i32 {
    '__tco: loop {
        match &**inPrefix {
            DAE::ComponentPrefix::PRE { .. } => {
                (inPrefix, hash) = (
                    var_field!((**inPrefix).next, DAE::ComponentPrefix::PRE),
                    31 * hash + stringHashDjb2(&var_field!((**inPrefix).prefix, DAE::ComponentPrefix::PRE)),
                );
                continue '__tco;
            }
            _ => return hash,
        }
    }
}

pub(crate) fn componentPrefixPathEqual<'__b>(
    mut pre1: &'__b metamodelica::Ref<DAE::ComponentPrefix>,
    mut pre2: &'__b metamodelica::Ref<DAE::ComponentPrefix>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match (pre1, pre2) {
            (Deref @ DAE::ComponentPrefix::PRE { .. }, Deref @ DAE::ComponentPrefix::PRE { .. }) => if (metamodelica::stringEq(&var_field!((**pre1).prefix, DAE::ComponentPrefix::PRE), &var_field!((**pre2).prefix, DAE::ComponentPrefix::PRE))) {{ (pre1, pre2) = (var_field!((**pre1).next, DAE::ComponentPrefix::PRE), var_field!((**pre2).next, DAE::ComponentPrefix::PRE)); continue '__tco; }} else {return false},
            (Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }) => return true,
            _ => return false,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn componentPrefix(mut inPrefix: &DAE::Prefix) -> metamodelica::Ref<DAE::ComponentPrefix> {
    let mut outPrefix: metamodelica::Ref<DAE::ComponentPrefix>;
    outPrefix = (match inPrefix.clone() {
        DAE::Prefix::PREFIX { .. } => var_field!(inPrefix.compPre, DAE::Prefix::PREFIX).clone(),
        _ => openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
    });
    outPrefix
}

pub fn writeComponentPrefix(
    mut file: File::File,
    mut pre: &metamodelica::Ref<DAE::ComponentPrefix>,
    mut escape: File::Escape,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match pre {
        Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, prefix: __pre_prefix, subscripts: __pre_subscripts, .. } => {
            File::writeEscape(file.clone(), __pre_prefix.clone(), escape);
            ComponentReference::writeSubscripts(file, metamodelica::AsArg::as_arg(&__pre_subscripts), escape)?;
            ()
        },
        Deref @ DAE::ComponentPrefix::PRE { next: __pre_next, prefix: __pre_prefix, subscripts: __pre_subscripts, .. } => {
            writeComponentPrefix(file.clone(), metamodelica::AsArg::as_arg(&__pre_next), File::Escape::None.clone())?;
            File::writeEscape(file.clone(), __pre_prefix.clone(), escape);
            ComponentReference::writeSubscripts(file, metamodelica::AsArg::as_arg(&__pre_subscripts), escape)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn hasSubs<'__b>(mut pre: &'__b metamodelica::Ref<DAE::ComponentPrefix>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match pre {
            Deref @ DAE::ComponentPrefix::PRE { subscripts: Deref @ metamodelica::ListNode::Nil, .. } => { pre = var_field!((**pre).next, DAE::ComponentPrefix::PRE); continue '__tco; },
            Deref @ DAE::ComponentPrefix::PRE { .. } => return true,
            _ => return false,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn removeCompPrefixFromExps(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCompPref: &metamodelica::Ref<DAE::ComponentPrefix>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outExp, _) = Expression::traverseExpBottomUp(
        inExp,
        &({
            let __pe_b2 = inCompPref.clone();
            move |__pe_a0, __pe_a1| removeCompPrefixFromCrefExp(__pe_a0, __pe_a1, __pe_b2.clone())
        }),
        false,
    )?;
    Ok(outExp)
}

fn removeCompPrefixFromCrefExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inB: bool,
    mut inCompPref: metamodelica::Ref<DAE::ComponentPrefix>,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut b: bool;
    (outExp, b) = (::match_deref::match_deref! { match &(inExp.clone()) {
        exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { .. }, .. } => {
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut exp = (*exp).clone();
            cref = removePrefixFromCref(var_field!((*exp).componentRef, DAE::Exp::CREF).clone(), inCompPref)?;
            assign_variant_field!(exp => DAE::Exp::CREF; componentRef = cref);
            (exp.clone(), true)
        },
        _ => {
            (inExp, inB)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, b))
}

fn removePrefixFromCref(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inCompPref: metamodelica::Ref<DAE::ComponentPrefix>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCref.clone(), inCompPref)) {
            (_, Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }) => {
                return Ok(inCref)
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, _) => {
                return Ok(inCref)
            },
            (cref @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }, pref @ Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. }) => {
                if stringEqual(&var_field!((**cref).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**pref).prefix, DAE::ComponentPrefix::PRE)) {
                }
                return Ok(var_field!((**cref).componentRef, DAE::ComponentRef::CREF_QUAL).clone())
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }, pref @ Deref @ DAE::ComponentPrefix::PRE { next: Deref @ DAE::ComponentPrefix::PRE { prefix: _, .. }, .. }) => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                let mut pref = (*pref).clone();
                cref = removePrefixFromCref(inCref, var_field!((*pref).next, DAE::ComponentPrefix::PRE).clone())?;
                assign_variant_field!(pref => DAE::ComponentPrefix::PRE; next = openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE());
                { (inCref, inCompPref) = (cref, pref.clone()); continue '__tco; }
            },
            (_, Deref @ DAE::ComponentPrefix::PRE { .. }) => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.removePrefixFromCref")); __mm_s.push_str(&*literal!(" :Cref is not qualified but we have prefix to remove: ")); __mm_s.push_str(&*ComponentReference::crefStr(&inCref)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/PrefixUtil.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PrefixUtil.removePrefixFromCref")); __mm_s.push_str(&*literal!(" :failed on cref: ")); __mm_s.push_str(&*ComponentReference::crefStr(&inCref)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/PrefixUtil.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}
