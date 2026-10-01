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
use crate::FGraphBuild;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub type Name = ArcStr;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = metamodelica::Ref<FCore::Node>;

pub type Data = metamodelica::Ref<FCore::Data>;

pub type Kind = FCore::Kind;

pub type Ref = Mutable::Mutable<metamodelica::Ref<FCore::Node>>;

pub type Refs = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type Children = metamodelica::Ref<FCore::RefTree::Tree>;

pub type Parents = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Msg = Option<SourceInfo>;

pub(crate) static dummyLookupOption: Option<SourceInfo> = None;

// SOME(Absyn.dummyInfo);
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Options {
    pub ignoreImports: bool,
    pub ignoreExtends: bool,
    pub ignoreParents: bool,
}

impl metamodelica::gc::MMTrace for Options {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ignoreImports, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ignoreExtends, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.ignoreParents, __mmv)?;
        Ok(())
    }
}
pub type OPTIONS = Options;

pub(crate) static ignoreNothing: Options = Options {
    ignoreImports: false,
    ignoreExtends: false,
    ignoreParents: false,
};

pub(crate) static ignoreParents: Options = Options {
    ignoreImports: false,
    ignoreExtends: false,
    ignoreParents: true,
};

pub(crate) static ignoreParentsAndImports: Options = Options {
    ignoreImports: true,
    ignoreExtends: false,
    ignoreParents: true,
};

pub(crate) static ignoreAll: Options = Options {
    ignoreImports: true,
    ignoreExtends: true,
    ignoreParents: true,
};

pub(crate) fn id(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inName: &Name,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph, inOptions, inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            r = FNode::child(inRef.clone(), arcstr::literal!(FNode::forNodeName))?;
            r = FNode::child(r.clone(), inName.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut g,
                Options {
                    ignoreImports: _,
                    ignoreExtends: _,
                    ignoreParents: false,
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let true = (FNode::isRefImplicitScope(inRef.clone())) else {
                return Err("pattern mismatch");
            };
            r = FNode::refOriginalParent(inRef.clone())?;
            (g, r) = id(g.clone(), r.clone(), inName, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut g, _, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())) else {
                return Err("pattern mismatch");
            };
            r = FNode::child(inRef.clone(), inName.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut g,
                Options {
                    ignoreImports: false,
                    ignoreExtends: _,
                    ignoreParents: _,
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())) else {
                return Err("pattern mismatch");
            };
            (g, r) = imp(g.clone(), inRef.clone(), inName, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut g,
                Options {
                    ignoreImports: _,
                    ignoreExtends: false,
                    ignoreParents: _,
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())) else {
                return Err("pattern mismatch");
            };
            (g, r) = ext(g.clone(), inRef.clone(), inName, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut g,
                Options {
                    ignoreImports: _,
                    ignoreExtends: _,
                    ignoreParents: false,
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isEncapsulated(&(FNode::fromRef(inRef.clone())))?) else {
                return Err("pattern mismatch");
            };
            r = FNode::top(inRef.clone())?;
            (g, r) = id(g.clone(), r.clone(), inName, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut g,
                Options {
                    ignoreImports: _,
                    ignoreExtends: _,
                    ignoreParents: false,
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let false = (FNode::isRefImplicitScope(inRef.clone())) else {
                return Err("pattern mismatch");
            };
            let false = (FNode::isEncapsulated(&(FNode::fromRef(inRef.clone())))?) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::hasParents(&(FNode::fromRef(inRef.clone())))) else {
                return Err("pattern mismatch");
            };
            r = FNode::refOriginalParent(inRef.clone())?;
            (g, r) = search(g.clone(), &(list![r.clone()]), inName, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                _,
                Options {
                    ignoreImports: _,
                    ignoreExtends: _,
                    ignoreParents: false,
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let false = (FNode::hasParents(&(FNode::fromRef(inRef.clone())))) else {
                return Err("pattern mismatch");
            };
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _, Some(_)) = __mc_input.clone() else {
                return Err("nomatch");
            };
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FLookup.id failed for: "));
                __mm_s.push_str(&*inName);
                __mm_s.push_str(&*literal!(" in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(inRef.clone())))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

pub(crate) fn search(
    mut inGraph: Graph,
    mut inRefs: &Refs,
    mut inName: &Name,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph, &**inRefs, &inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::ListNode::Cons { head: r, tail: _ }, _) => {
                    let mut g = (*g).clone();
                    let mut r = (*r).clone();
                    (g, r) = id(g.clone(), r.clone(), inName, inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = search(g.clone(), metamodelica::AsArg::as_arg(&rest), inName, inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(_)) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.search failed for: ")); __mm_s.push_str(&*inName); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef((inRefs).head().cloned()?)))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

pub(crate) fn name(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph, &**inPath, &inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::Path::IDENT { name: i }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::Path::QUALIFIED { name: i, path: rest }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    (g, r) = name(g.clone(), r.clone(), metamodelica::AsArg::as_arg(&rest), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::Path::QUALIFIED { name: i, path: rest }, _) => {
                    let mut r: Ref;
                    let mut s: ArcStr;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    if '__try0: {
                        unwrap_break_err!(name(g.clone(), r.clone(), metamodelica::AsArg::as_arg(&rest), inOptions, inMsg.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("missing: ")); __mm_s.push_str(&*AbsynUtil::pathString(rest.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" in scope: ")); __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?); ArcStr::from(__mm_s) };
                    (g, r) = FGraphBuild::mkAssertNode(AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&rest)), s.clone(), r.clone(), g.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::Path::FULLYQUALIFIED { path: rest }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = FNode::top(inRef.clone())?;
                    (g, r) = name(g.clone(), r.clone(), metamodelica::AsArg::as_arg(&rest), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(_)) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.name failed for: ")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(inRef.clone())))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

pub(crate) fn ext(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inName: &Name,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let true = (FNode::isClassExtends(&(FNode::fromRef(inRef.clone())))) else {
                return Err("pattern mismatch");
            };
            r = FNode::child(inRef.clone(), arcstr::literal!(FNode::refNodeName))?;
            r = FNode::target(&(FNode::fromRef(r.clone())))?;
            (g, r) = id(g.clone(), r.clone(), inName, ignoreParents.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let true = (FNode::isClassExtends(&(FNode::fromRef(inRef.clone())))) else {
                return Err("pattern mismatch");
            };
            r = FNode::refOriginalParent(inRef.clone())?;
            (g, r) = id(g.clone(), r.clone(), inName, ignoreNothing.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let mut refs: Refs;
            refs = FNode::extendsRefs(inRef.clone())?;
            let false = ((refs).is_empty()) else {
                return Err("pattern mismatch");
            };
            refs = List::mapMap(
                refs.clone(),
                &fnptr!(FNode::fromRef, Mutable::Mutable<metamodelica::Ref<FCore::Node>>),
                &move |__a0: metamodelica::Ref<FCore::Node>| FNode::target(&__a0),
            )?;
            (g, r) = search(g.clone(), &refs, inName, ignoreParentsAndImports.clone(), inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

pub(crate) fn imp(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inName: &Name,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = inGraph;
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let mut qi: metamodelica::List<Absyn::Import>;
            let true = (FNode::hasImports(FNode::fromRef(inRef.clone()))?) else {
                return Err("pattern mismatch");
            };
            (qi, _) = FNode::imports(FNode::fromRef(inRef.clone()))?;
            (g, r) = imp_qual(g.clone(), inRef.clone(), inName, &qi, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut g = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut r: Ref;
            let mut uqi: metamodelica::List<Absyn::Import>;
            let true = (FNode::hasImports(FNode::fromRef(inRef.clone()))?) else {
                return Err("pattern mismatch");
            };
            (_, uqi) = FNode::imports(FNode::fromRef(inRef.clone()))?;
            (g, r) = imp_unqual(g.clone(), inRef.clone(), inName, &uqi, inOptions, inMsg.clone())?;
            Ok((g.clone(), r.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

fn imp_qual(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inName: &Name,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph, &**inImports);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: rest_imps }) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let false = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    (g, r) = imp_qual(g.clone(), inRef.clone(), inName, metamodelica::AsArg::as_arg(&rest_imps), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, path }, tail: _ }) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    let true = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    (g, r) = fq(g.clone(), metamodelica::AsArg::as_arg(&path), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::NAMED_IMPORT { name, .. }, tail: _ }) => {
                    let true = (stringEqual(&inName, &name)) else { return Err("pattern mismatch") };
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

pub(crate) fn imp_unqual(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inName: &Name,
    mut inImports: &metamodelica::List<Absyn::Import>,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph, &**inImports);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::ListNode::Cons { head: Absyn::Import::UNQUAL_IMPORT { path }, tail: _ }) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = fq(g.clone(), metamodelica::AsArg::as_arg(&path), inOptions, inMsg.clone())?;
                    (g, r) = id(g.clone(), r.clone(), inName, ignoreParents.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_imps }) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = imp_unqual(g.clone(), inRef.clone(), inName, metamodelica::AsArg::as_arg(&rest_imps), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}

pub(crate) fn fq(
    mut inGraph: Graph,
    mut inName: &metamodelica::Ref<Absyn::Path>,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = name(inGraph.clone(), FGraph::top(&inGraph)?, inName, inOptions, inMsg)?;
    Ok((outGraph, outRef))
}

pub(crate) fn cr(
    mut inGraph: Graph,
    mut inRef: Ref,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inOptions: Options,
    mut inMsg: Msg,
) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = 'mc: {
        let __mc_input = (inGraph, &**inCref, &inMsg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::ComponentRef::CREF_IDENT { name: i, subscripts: _ }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: _, componentRef: rest }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    let true = (FNode::isRefComponent(r.clone())) else { return Err("pattern mismatch") };
                    r = FNode::child(r.clone(), arcstr::literal!(FNode::refNodeName))?;
                    r = FNode::target(&(FNode::fromRef(r.clone())))?;
                    (g, r) = cr(g.clone(), r.clone(), metamodelica::AsArg::as_arg(&rest), ignoreParents.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: _, componentRef: rest }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    let true = (FNode::isRefClass(r.clone())) else { return Err("pattern mismatch") };
                    (g, r) = cr(g.clone(), r.clone(), metamodelica::AsArg::as_arg(&rest), ignoreParents.clone(), inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: _, componentRef: rest }, _) => {
                    let mut r: Ref;
                    let mut s: ArcStr;
                    let mut g = (*g).clone();
                    (g, r) = id(g.clone(), inRef.clone(), metamodelica::AsArg::as_arg(&i), inOptions, inMsg.clone())?;
                    let true = (FNode::isRefClass(r.clone()) || FNode::isRefComponent(r.clone())) else { return Err("pattern mismatch") };
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("missing: ")); __mm_s.push_str(&*AbsynUtil::crefString(metamodelica::AsArg::as_arg(&rest))?); __mm_s.push_str(&*literal!(" in scope: ")); __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?); ArcStr::from(__mm_s) };
                    (g, r) = FGraphBuild::mkAssertNode(AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&rest))?, s.clone(), r.clone(), g.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (g, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: rest }, _) => {
                    let mut r: Ref;
                    let mut g = (*g).clone();
                    r = FGraph::top(metamodelica::AsArg::as_arg(&g))?;
                    (g, r) = cr(g.clone(), r.clone(), metamodelica::AsArg::as_arg(&rest), inOptions, inMsg.clone())?;
                    Ok((g.clone(), r.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(_)) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FLookup.cr failed for: ")); __mm_s.push_str(&*AbsynUtil::crefString(inCref)?); __mm_s.push_str(&*literal!(" in: ")); __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(inRef.clone())))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outGraph, outRef))
}
